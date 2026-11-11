// SPDX-License-Identifier: MIT

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
            "storytelling-narrative-{}-{}-{}",
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

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/narrative-v1/n03-authoring-v1.json")
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn render_file(input: &Path, directory: &Path) -> Output {
    cli()
        .arg("render-narrative")
        .arg(input)
        .current_dir(directory)
        .output()
        .unwrap()
}

fn render_stdin(bytes: &[u8], directory: &Path) -> Output {
    let mut child = cli()
        .args(["render-narrative", "-"])
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

fn directory_snapshot(root: &Path) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeMap<PathBuf, Option<Vec<u8>>>) {
        for entry in std::fs::read_dir(current).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let relative = path.strip_prefix(root).unwrap().to_path_buf();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                files.insert(relative, None);
                visit(root, &path, files);
            } else {
                assert!(kind.is_file(), "unexpected filesystem entry");
                files.insert(relative, Some(std::fs::read(path).unwrap()));
            }
        }
    }

    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

fn json_error(output: &Output, expected_exit: i32) -> Value {
    assert_eq!(output.status.code(), Some(expected_exit));
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(output.stdout.iter().filter(|b| **b == b'\n').count(), 1);
    serde_json::from_slice(&output.stdout).unwrap()
}

fn assert_no_video_input_fields(value: &Value) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                assert!(
                    !matches!(
                        key.as_str(),
                        "assets"
                            | "asset_id"
                            | "media_path"
                            | "file_path"
                            | "duration"
                            | "source_range"
                            | "frame_rate"
                            | "fps"
                            | "cut_in"
                            | "cut_out"
                    ),
                    "unexpected video-only input field: {key}"
                );
                assert_no_video_input_fields(value);
            }
        }
        Value::Array(values) => values.iter().for_each(assert_no_video_input_fields),
        _ => {}
    }
}

#[test]
fn renders_three_declared_outputs_reproducibly_without_media_or_writes() {
    let directory = TestDirectory::new();
    let input_path = directory.join("authoring.json");
    let sentinel_path = directory.join("sentinel.txt");
    let original = std::fs::read(fixture()).unwrap();
    let document: Value = serde_json::from_slice(&original).unwrap();
    assert_no_video_input_fields(&document);
    assert_eq!(document["variants"].as_array().unwrap().len(), 3);
    std::fs::write(&input_path, &original).unwrap();
    std::fs::write(&sentinel_path, b"leave this file unchanged").unwrap();
    let before = directory_snapshot(&directory.0);

    let first = render_file(&input_path, &directory.0);
    let second = render_file(&input_path, &directory.0);
    let stdin = render_stdin(&original, &directory.0);
    for output in [&first, &second, &stdin] {
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        assert_eq!(output.stdout.last(), Some(&b'\n'));
    }
    assert_eq!(
        first.stdout, second.stdout,
        "same input must render identically"
    );
    assert_eq!(
        first.stdout, stdin.stdout,
        "file and stdin must render identically"
    );
    assert_eq!(std::fs::read(&input_path).unwrap(), original);
    assert_eq!(directory_snapshot(&directory.0), before);

    let rendered = String::from_utf8(first.stdout).unwrap();
    let oral_start = rendered.find("## Variant oral ").unwrap();
    let wide_start = rendered.find("## Variant illustration-wide ").unwrap();
    let close_start = rendered.find("## Variant illustration-close ").unwrap();
    assert!(oral_start < wide_start && wide_start < close_start);
    let oral = &rendered[oral_start..wide_start];
    let wide = &rendered[wide_start..close_start];
    let close = &rendered[close_start..];
    assert!(
        oral.find("[S01@1 | supplied_observation]").unwrap()
            < oral.find("[S02@1 | supplied_observation]").unwrap()
    );
    assert!(
        oral.find("[S02@1 | supplied_observation]").unwrap()
            < oral.find("[S03@1 | supplied_observation]").unwrap()
    );
    assert!(oral.contains("[I01@1 | author_interpretation]"));
    assert!(!oral.contains("C01") && !oral.contains("紅傘"));
    assert!(wide.contains("寬景") && wide.contains("後方") && wide.contains("工具箱"));
    assert!(wide.contains("[C01@1 | creative_proposal]"));
    assert!(wide.contains("紅傘"));
    assert!(close.contains("鉸鏈") && close.contains("前景") && close.contains("背景"));
    assert!(!close.contains("C01") && !close.contains("紅傘"));
}

#[test]
fn dangling_reference_returns_the_contract_diagnostic() {
    let directory = TestDirectory::new();
    let input_path = directory.join("broken-reference.json");
    let mut document: Value = serde_json::from_slice(&std::fs::read(fixture()).unwrap()).unwrap();
    document["variants"][0]["beats"][0]["unit_ref"]["unit_id"] = json!("MISSING");
    let bytes = serde_json::to_vec(&document).unwrap();
    std::fs::write(&input_path, &bytes).unwrap();
    let before = directory_snapshot(&directory.0);

    let output = render_file(&input_path, &directory.0);
    let error = json_error(&output, 1);
    assert_eq!(
        error,
        json!({
            "ok": false,
            "error": {
                "code": "narrative.dangling_reference",
                "path": "$.variants[0].beats[0].unit_ref.unit_id"
            }
        })
    );
    assert_eq!(std::fs::read(&input_path).unwrap(), bytes);
    assert_eq!(directory_snapshot(&directory.0), before);
}

#[test]
fn revision_tracking_queries_shared_and_variant_local_impacts() {
    let directory = TestDirectory::new();
    let input_path = directory.join("revision-history.json");
    let original = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/narrative-v1/n04-revision-history-v1.json"),
    )
    .unwrap();
    std::fs::write(&input_path, &original).unwrap();
    let before = directory_snapshot(&directory.0);

    let shared = cli()
        .args(["query-narrative-impact"])
        .arg(&input_path)
        .arg("shared-observation-change")
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert_eq!(shared.status.code(), Some(0));
    let shared = String::from_utf8(shared.stdout).unwrap();
    assert!(shared.contains("S02@1 -> S02@2"));
    assert!(shared.contains("Affected variants: oral, illustration-wide, illustration-close"));

    let local = cli()
        .args(["query-narrative-impact"])
        .arg(&input_path)
        .arg("b-only-detail-change")
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert_eq!(local.status.code(), Some(0));
    let local = String::from_utf8(local.stdout).unwrap();
    assert!(local.contains("C01@1 -> C01@2"));
    assert!(local.contains("Affected variants: illustration-close"));
    assert!(!local.contains("Affected variants: oral"));

    let history = cli()
        .arg("track-narrative-revisions")
        .arg(&input_path)
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert_eq!(history.status.code(), Some(0));
    let history = String::from_utf8(history.stdout).unwrap();
    assert!(history.contains("Choice decline-red-umbrella-in-a: illustration-wide C01@1 declined"));
    assert!(history.contains("Choice accept-red-umbrella-in-b: illustration-close C01@1 accepted"));
    assert!(history.contains("Suggestion suggest-red-umbrella-for-b: illustration-close C01@1 pending"));
    assert!(history.contains("Source S02@1"));
    assert!(history.contains("Source S02@2"));
    assert!(history.contains("Source C01@1"));
    assert!(history.contains("Source C01@2"));
    assert!(history.contains("Current illustration-close@3: needs_review"));
    assert!(history.contains("Current oral@2: current"));
    assert!(history.contains("Current illustration-wide@3: current"));
    let oral = history.split("Current oral@2:").nth(1).unwrap();
    let oral = oral.split("Current illustration-wide@").next().unwrap();
    assert!(!oral.contains("紅傘") && !oral.contains("C01"));
    assert_eq!(std::fs::read(&input_path).unwrap(), original);
    assert_eq!(directory_snapshot(&directory.0), before);
}

#[test]
fn untrusted_suggestion_cannot_claim_author_acceptance_from_the_cli() {
    let directory = TestDirectory::new();
    let input_path = directory.join("self-confirmed-suggestion.json");
    let original = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/narrative-v1/n04-invalid-self-confirm-v1.json"),
    )
    .unwrap();
    std::fs::write(&input_path, &original).unwrap();
    let before = directory_snapshot(&directory.0);

    let output = cli()
        .arg("track-narrative-revisions")
        .arg(&input_path)
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert_eq!(
        json_error(&output, 1),
        json!({
            "ok": false,
            "error": {
                "code": "revision.untrusted_decision",
                "path": "$.events[0].decision"
            }
        })
    );
    assert_eq!(std::fs::read(&input_path).unwrap(), original);
    assert_eq!(directory_snapshot(&directory.0), before);
}
