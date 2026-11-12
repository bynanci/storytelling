// SPDX-License-Identifier: MIT

use serde_json::{Value, json};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "storytelling-video-coverage-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn join(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/narrative-video-coverage-v1")
        .join(name)
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn run_file(name: &str) -> Output {
    cli()
        .args(["render-video-coverage"])
        .arg(fixture(name))
        .output()
        .unwrap()
}

fn run_stdin(bytes: &[u8], directory: &Path) -> Output {
    let mut child = cli()
        .args(["render-video-coverage", "-"])
        .current_dir(directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    child.wait_with_output().unwrap()
}

fn run_stdin_in_temp(bytes: &[u8]) -> Output {
    let directory = TestDirectory::new();
    run_stdin(bytes, &directory.0)
}

fn json_line(output: &Output, exit: i32) -> Value {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stderr.is_empty(), "unexpected stderr");
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(output.stdout.iter().filter(|b| **b == b'\n').count(), 1);
    serde_json::from_slice(&output.stdout).unwrap()
}

fn canonical_json_line_ending(bytes: &[u8]) -> Vec<u8> {
    String::from_utf8(bytes.to_vec())
        .unwrap()
        .replace("\r\n", "\n")
        .into_bytes()
}

fn assert_error(output: Output, code: &str, path: &str) {
    let value = json_line(&output, 1);
    assert_eq!(value, json!({"error":{"code":code,"path":path}}));
}

#[test]
fn accepts_complete_video_coverage_without_media_access() {
    let output = run_file("valid-complete.json");
    let value = json_line(&output, 0);
    assert_eq!(value["profile"], "storytelling.narrative-video-coverage.v1");
    assert_eq!(value["coverage_id"], "two-unit-cut");
    assert_eq!(value["ready_for_planning"], true);
    assert_eq!(value["diagnostics"], json!([]));
    assert_eq!(value["mappings"].as_array().unwrap().len(), 2);
    assert_eq!(value["mappings"][0]["unit_ref"]["unit_id"], "S02");
    assert_eq!(value["mappings"][1]["unit_ref"]["unit_id"], "S03");
}

#[test]
fn reports_unresolved_video_unit_and_blocks_planning() {
    let output = run_file("valid-unresolved.json");
    let expected = canonical_json_line_ending(
        &std::fs::read(fixture("expected-valid-unresolved.stdout.json")).unwrap(),
    );
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert_eq!(canonical_json_line_ending(&output.stdout), expected);
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["ready_for_planning"], false);
    assert_eq!(value["mappings"][1]["status"], "unresolved");
    assert_eq!(
        value["diagnostics"][0]["code"],
        "video_coverage.unresolved_required"
    );
}

#[test]
fn rejects_contract_failures_with_stable_codes_and_safe_paths() {
    for (name, code, path) in [
        (
            "invalid-missing-selected.json",
            "video_coverage.missing_selected_unit",
            "$.selected_unit_refs[1]",
        ),
        (
            "invalid-stale-unit.json",
            "video_coverage.stale_unit_revision",
            "$.selected_unit_refs[0].revision",
        ),
        (
            "invalid-dangling-asset.json",
            "video_coverage.dangling_asset",
            "$.mappings[1].asset_ref.asset_id",
        ),
        (
            "invalid-range.json",
            "video_coverage.invalid_range",
            "$.mappings[0].source_range.end",
        ),
        (
            "invalid-version.json",
            "video_coverage.unsupported_version",
            "$.schema_version",
        ),
        (
            "invalid-duplicate-field.raw",
            "video_coverage.duplicate_field",
            "$",
        ),
    ] {
        assert_error(run_file(name), code, path);
    }
}

#[test]
fn compares_file_stdin_replay_and_preserves_input_bytes() {
    let directory = TestDirectory::new();
    let bytes = std::fs::read(fixture("invalid-range.json")).unwrap();
    let mut stale_and_range: Value = serde_json::from_slice(&bytes).unwrap();
    stale_and_range["selected_unit_refs"][0]["revision"] = json!(2);
    stale_and_range["mappings"][0]["unit_ref"]["revision"] = json!(2);
    let stale_and_range = serde_json::to_vec(&stale_and_range).unwrap();
    assert_error(
        run_stdin(&stale_and_range, &directory.0),
        "video_coverage.stale_unit_revision",
        "$.selected_unit_refs[0].revision",
    );

    let input_path = directory.join("coverage.json");
    let input = std::fs::read(fixture("valid-unresolved.json")).unwrap();
    std::fs::write(&input_path, &input).unwrap();
    std::fs::write(directory.join("sentinel.txt"), b"do not change").unwrap();
    let before_names: Vec<_> = std::fs::read_dir(&directory.0)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    let run = || {
        cli()
            .args(["render-video-coverage"])
            .arg(&input_path)
            .current_dir(&directory.0)
            .output()
            .unwrap()
    };
    let first = run();
    let second = run();
    let stdin = run_stdin(&input, &directory.0);
    for output in [&first, &second, &stdin] {
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
    }
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(first.stdout, stdin.stdout);
    assert_eq!(
        canonical_json_line_ending(&first.stdout),
        canonical_json_line_ending(
            &std::fs::read(fixture("expected-valid-unresolved.stdout.json")).unwrap()
        )
    );
    assert_eq!(std::fs::read(&input_path).unwrap(), input);
    assert_eq!(
        std::fs::read(directory.join("sentinel.txt")).unwrap(),
        b"do not change"
    );
    let after_names: Vec<_> = std::fs::read_dir(&directory.0)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(before_names, after_names);
}

#[test]
fn accepts_illustration_variant_sources() {
    let mut value: Value =
        serde_json::from_slice(&std::fs::read(fixture("valid-complete.json")).unwrap()).unwrap();
    value["narrative_document"]["variants"][0] = json!({
        "id": "rough-cut",
        "name": "Two selected illustration sources",
        "kind": "single_illustration",
        "unit_refs": value["selected_unit_refs"],
        "brief": "A synthetic illustration brief using both units."
    });
    let output = run_stdin_in_temp(&serde_json::to_vec(&value).unwrap());
    let result = json_line(&output, 0);
    assert_eq!(result["ready_for_planning"], true);
    assert_eq!(result["selected_unit_refs"], value["selected_unit_refs"]);
}

#[test]
fn rejects_shape_order_and_json_limit_violations() {
    let mut value: Value =
        serde_json::from_slice(&std::fs::read(fixture("valid-complete.json")).unwrap()).unwrap();
    value["unexpected"] = json!(true);
    assert_error(
        run_stdin_in_temp(&serde_json::to_vec(&value).unwrap()),
        "video_coverage.unknown_field",
        "$",
    );

    let mut value: Value =
        serde_json::from_slice(&std::fs::read(fixture("valid-complete.json")).unwrap()).unwrap();
    value["selected_unit_refs"] = json!([
        {"unit_id":"S03","revision":1},
        {"unit_id":"S02","revision":1}
    ]);
    assert_error(
        run_stdin_in_temp(&serde_json::to_vec(&value).unwrap()),
        "video_coverage.selection_order_mismatch",
        "$.selected_unit_refs[1].unit_id",
    );

    let mut value: Value =
        serde_json::from_slice(&std::fs::read(fixture("valid-complete.json")).unwrap()).unwrap();
    value["mappings"][0]["asset_ref"]["revision"] = json!(2);
    assert_error(
        run_stdin_in_temp(&serde_json::to_vec(&value).unwrap()),
        "video_coverage.stale_asset_revision",
        "$.mappings[0].asset_ref.revision",
    );

    assert_eq!(
        json_line(&run_stdin_in_temp(b"{"), 1),
        json!({"error":{"code":"input.invalid_json","path":null}}),
    );

    let too_deep = format!(
        "{{\"schema_version\":1,\"extra\":{}0{}}}",
        "[".repeat(33),
        "]".repeat(33)
    );
    assert_error(
        run_stdin_in_temp(too_deep.as_bytes()),
        "video_coverage.depth_exceeded",
        "$",
    );

    let at_limit = format!(
        "{{\"schema_version\":1,\"extra\":{}0{}}}",
        "[".repeat(32),
        "]".repeat(32)
    );
    assert_error(
        run_stdin_in_temp(at_limit.as_bytes()),
        "video_coverage.unknown_field",
        "$",
    );

    let mut exact_limit = std::fs::read(fixture("valid-complete.json")).unwrap();
    exact_limit.resize(1_048_576, b' ');
    assert_eq!(
        json_line(&run_stdin_in_temp(&exact_limit), 0)["ready_for_planning"],
        true
    );

    let oversized = vec![b' '; 1_048_577];
    assert_eq!(
        json_line(&run_stdin_in_temp(&oversized), 1),
        json!({"error":{"code":"input.too_large","path":null}}),
    );
}

#[test]
fn rejects_cross_reference_duplicate_and_bound_errors_by_contract_code() {
    let base = || -> Value {
        serde_json::from_slice(&std::fs::read(fixture("valid-complete.json")).unwrap()).unwrap()
    };
    let check = |value: Value, code: &str, path: &str| {
        assert_error(
            run_stdin_in_temp(&serde_json::to_vec(&value).unwrap()),
            code,
            path,
        );
    };

    let mut value = base();
    value["coverage_id"] = json!(7);
    check(value, "video_coverage.invalid_shape", "$");

    let mut value = base();
    value["variant_id"] = json!("absent-variant");
    check(value, "video_coverage.dangling_variant", "$.variant_id");

    let mut value = base();
    value["selected_unit_refs"][0]["unit_id"] = json!("NO-SUCH-UNIT");
    check(
        value,
        "video_coverage.dangling_unit",
        "$.selected_unit_refs[0].unit_id",
    );

    let mut value = base();
    value["selected_unit_refs"][1] = value["selected_unit_refs"][0].clone();
    check(
        value,
        "video_coverage.duplicate_selection",
        "$.selected_unit_refs[1].unit_id",
    );

    let mut value = base();
    value["narrative_document"]["units"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "id":"S04","revision":1,"origin":"supplied_observation",
            "text":"An unselected unit.","based_on":[]
        }));
    value["selected_unit_refs"][1] = json!({"unit_id":"S04","revision":1});
    value["mappings"][1]["unit_ref"] = json!({"unit_id":"S04","revision":1});
    check(
        value,
        "video_coverage.unit_outside_variant",
        "$.selected_unit_refs[1].unit_id",
    );

    let mut value = base();
    value["narrative_document"]["units"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "id":"S04","revision":1,"origin":"supplied_observation",
            "text":"An unselected unit.","based_on":[]
        }));
    value["mappings"][1]["unit_ref"] = json!({"unit_id":"S04","revision":1});
    check(
        value,
        "video_coverage.mapping_outside_selection",
        "$.mappings[1].unit_ref.unit_id",
    );

    let mut value = base();
    value["mappings"][1] = value["mappings"][0].clone();
    check(
        value,
        "video_coverage.duplicate_mapping",
        "$.mappings[1].unit_ref.unit_id",
    );

    let mut value = base();
    value["mappings"].as_array_mut().unwrap().reverse();
    check(
        value,
        "video_coverage.mapping_order_mismatch",
        "$.mappings[1].unit_ref",
    );

    let mut value = base();
    let duplicate = value["asset_catalog"]["assets"][0].clone();
    value["asset_catalog"]["assets"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    check(
        value,
        "video_coverage.duplicate_asset",
        "$.asset_catalog.assets[1].asset_id",
    );

    let mut value = base();
    value["selected_unit_refs"] = json!(vec![json!({"unit_id":"S02","revision":1}); 513]);
    check(
        value,
        "video_coverage.too_many_items",
        "$.selected_unit_refs",
    );

    let mut value = base();
    value["mappings"][0]["source_range"]["start"] = json!({
        "numerator":"2","denominator":"4"
    });
    check(
        value,
        "video_coverage.invalid_range",
        "$.mappings[0].source_range.start",
    );
}
