// SPDX-License-Identifier: Apache-2.0
// Source-informed fixed-window consumer regressions; see provenance-m2-02.json.
use serde_json::{Value, json};
use std::collections::BTreeMap;
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
            "storytelling-planner-{}-{}-{}",
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
        .join("../../tests/fixtures/planner-v1")
        .join(name)
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn stdin_command(command: &str, bytes: &[u8], directory: &Path) -> Output {
    let mut child = cli()
        .args([command, "-"])
        .current_dir(directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Err(error) = child.stdin.take().unwrap().write_all(bytes) {
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    child.wait_with_output().unwrap()
}

fn json_line(output: &Output, exit: i32) -> Value {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stderr.is_empty(), "unexpected stderr");
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(output.stdout.iter().filter(|b| **b == b'\n').count(), 1);
    serde_json::from_slice(&output.stdout).unwrap()
}

fn snapshot(root: &Path) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
    fn visit(root: &Path, current: &Path, result: &mut BTreeMap<PathBuf, Option<Vec<u8>>>) {
        for entry in std::fs::read_dir(current).unwrap() {
            let path = entry.unwrap().path();
            let relative = path.strip_prefix(root).unwrap().to_path_buf();
            if path.is_dir() {
                result.insert(relative, None);
                visit(root, &path, result);
            } else {
                result.insert(relative, Some(std::fs::read(path).unwrap()));
            }
        }
    }
    let mut result = BTreeMap::new();
    visit(root, root, &mut result);
    result
}

fn assert_declared_report(result: &Value) {
    assert_eq!(result["ok"], true);
    assert_eq!(result.as_object().unwrap().len(), 2);
    let report = &result["planning"];
    assert_eq!(report["profile"], "storytelling.fixed-window-planner.v1");
    assert_eq!(report["scope"], "declared-fixed-window-plan-only");
    assert_eq!(report["executable"], false);
    assert_eq!(report["source_verified"], false);
    assert_eq!(report["host_verified"], false);
    assert_eq!(report["selection_planned"], true);
    assert_eq!(report["duration_analysis"], "exact-fixed-windows");
    assert_eq!(report["content_clock_rate"], "10");
    assert_eq!(
        report["total_duration"],
        json!({"numerator":"3","denominator":"10"})
    );
    assert_eq!(report["target_status"], "met");
    assert_eq!(report["plan"]["plan_id"], "synthetic-fixed-plan");
    assert_eq!(report["plan"]["shots"][0]["id"], "shot-000001");
    assert_eq!(report["plan"]["shots"][0]["asset_id"], "detail");
    assert_eq!(report["plan"]["shots"][1]["id"], "shot-000002");
    assert_eq!(report["plan"]["shots"][1]["asset_id"], "opening");
    assert_eq!(
        report["selections"][1]["timeline_start"],
        json!({"numerator":"1","denominator":"5"})
    );
    assert!(result.get("error").is_none());
}

#[test]
fn plans_file_and_stdin_identically_without_mutating_input_or_directory() {
    let directory = TestDirectory::new();
    let original = std::fs::read(fixture("valid.json")).unwrap();
    let input = directory.join("candidates.json");
    std::fs::write(&input, &original).unwrap();
    std::fs::create_dir(directory.join("untouched")).unwrap();
    std::fs::write(
        directory.join("untouched/sentinel.txt"),
        b"synthetic sentinel",
    )
    .unwrap();
    let before = snapshot(&directory.0);
    let file = cli()
        .arg("plan-story")
        .arg(&input)
        .current_dir(&directory.0)
        .output()
        .unwrap();
    let stdin = stdin_command("plan-story", &original, &directory.0);
    assert_declared_report(&json_line(&file, 0));
    assert_declared_report(&json_line(&stdin, 0));
    assert_eq!(file.stdout, stdin.stdout);
    assert_eq!(std::fs::read(&input).unwrap(), original);
    assert_eq!(snapshot(&directory.0), before);
}

#[test]
fn emitted_nested_plan_is_accepted_by_the_actual_m0_cli() {
    let directory = TestDirectory::new();
    let output = cli()
        .arg("plan-story")
        .arg(fixture("valid.json"))
        .output()
        .unwrap();
    let result = json_line(&output, 0);
    let plan = &result["planning"]["plan"];
    assert_eq!(plan["assets"][0]["id"], "opening");
    assert_eq!(plan["assets"][1]["id"], "detail");
    assert_eq!(
        plan["shots"][0]["source_range"],
        json!({
            "start":{"numerator":"1","denominator":"10"},
            "end":{"numerator":"3","denominator":"10"}
        })
    );
    let validated = stdin_command("validate", &serde_json::to_vec(plan).unwrap(), &directory.0);
    let report = json_line(&validated, 0);
    assert_eq!(report["ok"], true);
    assert_eq!(report["validation"]["scope"], "declared-metadata-only");
    assert_eq!(report["validation"]["shots"], 2);
}

#[test]
fn hard_conflicts_emit_typed_global_paths_without_a_partial_plan() {
    let output = cli()
        .arg("plan-story")
        .arg(fixture("conflicting.json"))
        .output()
        .unwrap();
    let result = json_line(&output, 1);
    assert_eq!(
        result,
        json!({"ok":false,"error":{
            "code":"planner.brief_conflict", "path":"$.brief",
            "conflicts":[{"kind":"required-excluded", "path":"$.brief.manual_selected_asset_ids[1]", "asset_ids":["opening"]}]
        }})
    );
    assert!(result.get("planning").is_none());
    assert!(result.get("plan").is_none());
}

#[test]
fn malformed_future_unknown_and_oversized_inputs_return_domain_exit_one() {
    let directory = TestDirectory::new();
    let valid: Value =
        serde_json::from_slice(&std::fs::read(fixture("valid.json")).unwrap()).unwrap();
    let mut future = valid.clone();
    future["schema_version"] = json!(2);
    let mut unknown = valid;
    unknown["candidates"][0]["future_score"] = json!(1);
    let cases = [
        (b"{".to_vec(), "input.invalid_json"),
        (
            serde_json::to_vec(&future).unwrap(),
            "planner.unsupported_version",
        ),
        (serde_json::to_vec(&unknown).unwrap(), "input.invalid_json"),
        (vec![b' '; 1_048_577], "input.too_large"),
    ];
    for (index, (bytes, code)) in cases.iter().enumerate() {
        let path = directory.join(&format!("input-{index}.json"));
        std::fs::write(&path, bytes).unwrap();
        let file = cli().arg("plan-story").arg(&path).output().unwrap();
        let stdin = stdin_command("plan-story", bytes, &directory.0);
        for output in [&file, &stdin] {
            let result = json_line(output, 1);
            assert_eq!(result["ok"], false);
            assert_eq!(result["error"]["code"], *code);
            assert_eq!(result["error"]["conflicts"], json!([]));
            assert!(result.get("planning").is_none());
        }
        assert_eq!(file.stdout, stdin.stdout);
        assert_eq!(std::fs::read(&path).unwrap(), *bytes);
    }
}

#[test]
fn rejects_missing_extra_unknown_and_option_looking_arguments_with_usage_two() {
    for args in [
        vec!["plan-story"],
        vec!["plan-story", "-", "extra"],
        vec!["plan-story", "-", "--picture-only"],
        vec!["plan-story", "--unknown"],
        vec!["plan-story", "--picture-only"],
        vec!["plan-story", "--help"],
    ] {
        let output = cli().args(args).output().unwrap();
        let result = json_line(&output, 2);
        assert_eq!(result["ok"], false);
        assert_eq!(result["error"]["code"], "cli.usage");
        assert!(result.get("planning").is_none());
    }
}

#[test]
fn distinguishes_missing_input_io_failure_and_redacts_the_path() {
    let directory = TestDirectory::new();
    let missing = directory.join("private-planner-input.json");
    let output = cli().arg("plan-story").arg(&missing).output().unwrap();
    let result = json_line(&output, 3);
    assert_eq!(
        result,
        json!({"ok":false,"error":{"code":"input.io","path":"$"}})
    );
    assert!(!String::from_utf8_lossy(&output.stdout).contains("private-planner-input"));
    let directory_output = cli().arg("plan-story").arg(&directory.0).output().unwrap();
    assert_eq!(json_line(&directory_output, 3), result);
}

#[test]
fn native_planner_succeeds_with_an_empty_external_tool_path() {
    let directory = TestDirectory::new();
    let output = cli()
        .arg("plan-story")
        .arg(fixture("valid.json"))
        .env("PATH", &directory.0)
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert_declared_report(&json_line(&output, 0));
    assert!(snapshot(&directory.0).is_empty());
}

#[test]
fn explicit_relative_prefix_accepts_an_option_looking_filename() {
    let directory = TestDirectory::new();
    let bytes = std::fs::read(fixture("valid.json")).unwrap();
    std::fs::write(directory.join("--candidate.json"), &bytes).unwrap();
    let output = cli()
        .args(["plan-story", "./--candidate.json"])
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert_declared_report(&json_line(&output, 0));
    assert_eq!(
        std::fs::read(directory.join("--candidate.json")).unwrap(),
        bytes
    );
}

#[test]
fn help_lists_planning_alongside_all_existing_commands() {
    let output = cli().arg("--help").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let help = String::from_utf8(output.stdout).unwrap();
    for command in [
        "validate",
        "import-openfilm",
        "export-otio",
        "audit-brief",
        "verify-source",
        "plan-story",
    ] {
        assert!(help.contains(command), "missing command {command}");
    }
}
