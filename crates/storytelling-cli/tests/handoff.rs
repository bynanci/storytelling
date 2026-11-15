// SPDX-License-Identifier: MIT

use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
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
            "storytelling-handoff-{}-{}-{}",
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
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/narrative-handoff-v1")
        .join(name)
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn render(name: &str) -> Output {
    cli()
        .arg("render-handoff")
        .arg(fixture(name))
        .output()
        .unwrap()
}

fn render_stdin(bytes: &[u8], directory: &Path) -> Output {
    let mut child = cli()
        .args(["render-handoff", "-"])
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

fn json_error(output: Output, code: &str) -> Value {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["ok"], false);
    assert_eq!(value["error"]["code"], code);
    value
}

fn source_snapshot() -> BTreeMap<PathBuf, Vec<u8>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    [
        "skills/storytelling/assets/umbrella-workshop/bench.svg",
        "skills/storytelling/assets/umbrella-workshop/taps.wav",
        "tests/fixtures/narrative-handoff-v1/notice-before.txt",
        "tests/fixtures/narrative-handoff-v1/notice-after.txt",
    ]
    .into_iter()
    .map(|relative| {
        let path = root.join(relative);
        let bytes = std::fs::read(&path).unwrap();
        (path, bytes)
    })
    .collect()
}

fn component_set(rendered: &str, action: &str) -> BTreeSet<String> {
    rendered
        .lines()
        .filter_map(|line| line.strip_prefix(&format!("{action} ")))
        .map(|line| line.split_whitespace().next().unwrap().to_owned())
        .collect()
}

fn generated_reference_handoff() -> Value {
    let units: Vec<_> = (0..256)
        .map(|index| {
            json!({"id":format!("u{index:03}"),"revision":1,
                "origin":"supplied_observation","text":"synthetic source","based_on":[]})
        })
        .collect();
    let beats: Vec<_> = (0..256)
        .map(|index| {
            json!({"id":format!("b{index:03}"),
                "unit_ref":{"unit_id":format!("u{index:03}"),"revision":1},
                "text":"synthetic beat"})
        })
        .collect();
    let sources: Vec<_> = (0..256)
        .map(|index| {
            json!({"unit_id":format!("u{index:03}"),"revision":if index==0 {2} else {1},
                "representation":{"kind":"description","media_kind":"text",
                    "text":"synthetic description"}})
        })
        .collect();
    let components: Vec<_> = (0..5)
        .map(|index| {
            let start = if index < 4 { index * 64 } else { 192 };
            let ids: Vec<_> = (start..start + 64)
                .map(|unit| format!("u{unit:03}"))
                .collect();
            json!({"id":format!("c{index}"),"unit_ids":ids})
        })
        .collect();
    json!({"schema_version":1,"manifest_id":"generated-reference-test",
        "ledger":{"schema_version":1,"base_document":{
            "schema_version":1,"document_id":"generated-reference-test",
            "document_revision":1,"purpose":"synthetic regression","audience":"machine",
            "units":units,"relations":[],"variants":[{"id":"v","kind":"spoken_outline",
                "name":"generated","beats":beats}]},
            "events":[{"id":"e","kind":"source_revision","unit_id":"u000",
                "expected_revision":1,"text":"revised synthetic source"}]},
        "change_id":"e","sources":sources,
        "artifacts":[{"id":"a0","name":"generated","medium":"text",
            "variant_id":"v","components":components}]})
}

fn component_lines(rendered: &str) -> Vec<String> {
    rendered
        .lines()
        .filter(|line| line.starts_with("UPDATE ") || line.starts_with("REUSE "))
        .map(str::to_owned)
        .collect()
}

#[test]
fn preserves_bound_revisions_and_repeated_component_references() {
    let dir = TestDirectory::new();
    let output = render_stdin(
        &serde_json::to_vec(&generated_reference_handoff()).unwrap(),
        &dir.0,
    );
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let rendered = String::from_utf8(output.stdout).unwrap();
    assert!(rendered.contains("Artifact a0 · variant=v@1 · status=needs_review"));
    let expected: Vec<_> = (0..5)
        .map(|component| {
            let start = if component < 4 { component * 64 } else { 192 };
            let references: Vec<_> = (start..start + 64)
                .map(|unit| {
                    let current = if unit == 0 { 2 } else { 1 };
                    format!("u{unit:03}@1 -> u{unit:03}@{current}")
                })
                .collect();
            let action = if component == 0 { "UPDATE" } else { "REUSE" };
            format!(
                "{action} a0/c{component} · draft -> current source: {}",
                references.join(", ")
            )
        })
        .collect();
    assert_eq!(component_lines(&rendered), expected);
}

#[test]
fn renders_components_and_their_references_in_declared_order() {
    let dir = TestDirectory::new();
    let mut value = generated_reference_handoff();
    let original = render_stdin(&serde_json::to_vec(&value).unwrap(), &dir.0);
    assert_eq!(original.status.code(), Some(0));
    assert!(original.stderr.is_empty());
    let original = String::from_utf8(original.stdout).unwrap();
    let components = value["artifacts"][0]["components"].as_array_mut().unwrap();
    components.reverse();
    for component in components {
        component["unit_ids"].as_array_mut().unwrap().reverse();
    }
    let reordered = render_stdin(&serde_json::to_vec(&value).unwrap(), &dir.0);
    assert_eq!(reordered.status.code(), Some(0));
    assert!(reordered.stderr.is_empty());
    let reordered = String::from_utf8(reordered.stdout).unwrap();
    let expected: Vec<_> = component_lines(&original)
        .into_iter()
        .rev()
        .map(|line| {
            let (prefix, references) = line.split_once(" · draft -> current source: ").unwrap();
            let mut references: Vec<_> = references.split(", ").collect();
            references.reverse();
            format!(
                "{prefix} · draft -> current source: {}",
                references.join(", ")
            )
        })
        .collect();
    assert_eq!(component_lines(&reordered), expected);
    let without_components = |text: &str| {
        text.lines()
            .filter(|line| !line.starts_with("UPDATE ") && !line.starts_with("REUSE "))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        without_components(&original),
        without_components(&reordered)
    );
}

#[test]
fn rejects_inventory_sources_outside_the_bound_variant_before_later_errors() {
    let dir = TestDirectory::new();
    let mut value = generated_reference_handoff();
    value["ledger"]["base_document"]["variants"][0]["beats"]
        .as_array_mut()
        .unwrap()
        .pop();
    value["artifacts"][0]["components"][4]["unit_ids"] = json!(["unknown"]);
    let error = json_error(
        render_stdin(&serde_json::to_vec(&value).unwrap(), &dir.0),
        "handoff.dangling_reference",
    );
    assert_eq!(
        error["error"]["path"],
        "$.artifacts[0].components[3].unit_ids"
    );
}

#[test]
fn computes_the_predeclared_exact_date_impact_without_false_results() {
    let expected: Value =
        serde_json::from_slice(&std::fs::read(fixture("expected-impact.json")).unwrap()).unwrap();
    let output = render("before-v1.json");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let rendered = String::from_utf8(output.stdout).unwrap();
    for (key, action) in [("update", "UPDATE"), ("reuse", "REUSE")] {
        let expected_set: BTreeSet<String> = expected[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect();
        let actual = component_set(&rendered, action);
        let missing: Vec<_> = expected_set.difference(&actual).collect();
        let unexpected: Vec<_> = actual.difference(&expected_set).collect();
        assert!(missing.is_empty(), "false negatives: {missing:?}");
        assert!(unexpected.is_empty(), "false positives: {unexpected:?}");
    }
    assert!(rendered.contains("S1@1"));
    assert!(rendered.contains("S1@2"));
    assert!(rendered.contains("Artifact invite · variant=invite@1 · status=needs_review"));
    assert!(rendered.contains("週六"));
    assert!(
        rendered.contains("Scope: declarations only; no source files are opened or rewritten.")
    );
}

#[test]
fn replays_file_and_stdin_byte_identically_and_preserves_inputs_and_sources() {
    let dir = TestDirectory::new();
    let input = dir.0.join("manifest.json");
    let bytes = std::fs::read(fixture("before-v1.json")).unwrap();
    std::fs::write(&input, &bytes).unwrap();
    std::fs::write(dir.0.join("sentinel.txt"), b"leave unchanged").unwrap();
    let sources_before = source_snapshot();
    let run = || {
        cli()
            .arg("render-handoff")
            .arg(&input)
            .current_dir(&dir.0)
            .output()
            .unwrap()
    };
    let first = run();
    let second = run();
    let stdin = render_stdin(&bytes, &dir.0);
    for output in [&first, &second, &stdin] {
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
    }
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(first.stdout, stdin.stdout);
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
    assert_eq!(
        std::fs::read(dir.0.join("sentinel.txt")).unwrap(),
        b"leave unchanged"
    );
    assert_eq!(std::fs::read_dir(&dir.0).unwrap().count(), 2);
    assert_eq!(source_snapshot(), sources_before);
}

#[test]
fn reviewed_sunday_drafts_retain_the_same_change_impact_and_become_current() {
    let before = render("before-v1.json");
    let after = render("after-v1.json");
    assert_eq!(before.status.code(), Some(0));
    assert_eq!(after.status.code(), Some(0));
    let old = String::from_utf8(before.stdout).unwrap();
    let new = String::from_utf8(after.stdout).unwrap();
    assert_eq!(component_set(&old, "UPDATE"), component_set(&new, "UPDATE"));
    assert_eq!(component_set(&old, "REUSE"), component_set(&new, "REUSE"));
    assert!(old.contains("Artifact poster · variant=poster@1 · status=needs_review"));
    assert!(new.contains("Artifact poster · variant=poster@2 · status=current"));
    assert!(new.contains("Artifact invite · variant=invite@2 · status=current"));
    assert!(new.contains("週日修傘"));
    assert!(new.contains("REUSE poster/layout"));
    assert!(new.contains("REUSE original-wav/audio"));
}

#[test]
fn descriptions_only_keep_usable_briefs_without_claiming_original_files() {
    let output = render("descriptions-only-v1.json");
    assert_eq!(output.status.code(), Some(0));
    let rendered = String::from_utf8(output.stdout).unwrap();
    assert!(rendered.contains("Source S2@1 [description]"));
    assert!(rendered.contains("Source S3@1 [description]"));
    assert!(rendered.contains("左側保留折起的藍傘"));
    assert!(rendered.contains("原始素材檔未交付"));
    assert!(!rendered.contains("skills/storytelling/assets/"));
}

#[test]
fn supplied_transcript_and_missing_material_remain_separate_from_files() {
    let dir = TestDirectory::new();
    let mut value: Value =
        serde_json::from_slice(&std::fs::read(fixture("descriptions-only-v1.json")).unwrap())
            .unwrap();
    value["sources"][1]["representation"] = json!({
        "kind":"missing", "media_kind":"image", "reason":"原始圖檔待補；仍有作者圖說可供構圖。"
    });
    value["sources"][2]["representation"] = json!({
        "kind":"transcript", "text":"作者提供的非語音標註：敲擊、停頓、敲擊。原始錄音未提供。"
    });
    let output = render_stdin(&serde_json::to_vec(&value).unwrap(), &dir.0);
    assert_eq!(output.status.code(), Some(0));
    let rendered = String::from_utf8(output.stdout).unwrap();
    assert!(rendered.contains("Source S2@1 [missing]"));
    assert!(rendered.contains("Source S3@1 [transcript]"));
    assert!(rendered.contains("原始錄音未提供"));
    assert!(rendered.contains("左側保留折起的藍傘"));
}

#[test]
fn rejects_a_component_source_that_does_not_exist() {
    json_error(
        render("missing-source-v1.json"),
        "handoff.dangling_reference",
    );
}

#[test]
fn rejects_a_current_descriptor_with_an_old_revision() {
    json_error(render("stale-revision-v1.json"), "handoff.stale_revision");
}

#[test]
fn rejects_poster_scoped_creative_material_in_another_variant() {
    json_error(render("proposal-leak-v1.json"), "handoff.variant_scope");
}

#[test]
fn rejects_an_omitted_known_variant_source_instead_of_hiding_impact() {
    let dir = TestDirectory::new();
    let mut value: Value =
        serde_json::from_slice(&std::fs::read(fixture("before-v1.json")).unwrap()).unwrap();
    value["artifacts"][1]["components"]
        .as_array_mut()
        .unwrap()
        .remove(0);
    json_error(
        render_stdin(&serde_json::to_vec(&value).unwrap(), &dir.0),
        "handoff.incomplete_mapping",
    );
}

#[test]
fn rejects_scoped_proposal_even_when_the_component_mapping_omits_it() {
    let dir = TestDirectory::new();
    let mut value: Value =
        serde_json::from_slice(&std::fs::read(fixture("proposal-leak-v1.json")).unwrap()).unwrap();
    let components = value["artifacts"][2]["components"].as_array_mut().unwrap();
    components.retain(|component| {
        !component["unit_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|id| id == "C1")
    });
    json_error(
        render_stdin(&serde_json::to_vec(&value).unwrap(), &dir.0),
        "handoff.variant_scope",
    );
}

#[test]
fn rejects_unknown_versions_before_parsing_the_shape() {
    let dir = TestDirectory::new();
    json_error(
        render_stdin(br#"{"schema_version":2}"#, &dir.0),
        "handoff.unsupported_version",
    );
}

#[test]
fn rejects_unknown_fields_and_duplicate_raw_keys() {
    let dir = TestDirectory::new();
    let bytes = std::fs::read(fixture("before-v1.json")).unwrap();
    let mut value: Value = serde_json::from_slice(&bytes).unwrap();
    value["untrusted_extra"] = json!(true);
    json_error(
        render_stdin(&serde_json::to_vec(&value).unwrap(), &dir.0),
        "handoff.unknown_field",
    );
    let text = String::from_utf8(bytes).unwrap();
    let duplicate = text.replacen('{', "{\"schema_version\":1,", 1);
    json_error(
        render_stdin(duplicate.as_bytes(), &dir.0),
        "handoff.duplicate_field",
    );
}
