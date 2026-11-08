// SPDX-License-Identifier: Apache-2.0
// Synthetic CLI coverage for the finite declared-review-binding commands.
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP_FILE: AtomicU64 = AtomicU64::new(0);

fn fixture(path: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

struct TempFiles(Vec<std::path::PathBuf>);

impl TempFiles {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn write(&mut self, label: &str, bytes: &[u8]) -> std::path::PathBuf {
        let id = NEXT_TEMP_FILE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "storytelling-review-{}-{id}-{label}.json",
            std::process::id()
        ));
        std::fs::write(&path, bytes).unwrap();
        self.0.push(path.clone());
        path
    }
}

impl Drop for TempFiles {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = std::fs::remove_file(path);
        }
    }
}

fn context() -> Value {
    json!({
        "schema_version": 1,
        "project_id": "project-1",
        "target_revision": "new-isolated-project",
        "host": {
            "id": "filmcraft",
            "version": "0.18.0",
            "profile_id": "isolated25fps",
            "binary_sha256": "a".repeat(64),
            "declared_capabilities": [
                "arbitrary-content-tick-rate",
                "external-reference",
                "single-video-track",
                "still-hold"
            ]
        },
        "loss_choices": ["metadata_only_locks", "picture_only"]
    })
}

#[test]
fn prepare_and_check_are_read_only_and_stale_is_a_successful_report() {
    let mut temp = TempFiles::new();
    let context_path = temp.write("context", &serde_json::to_vec(&context()).unwrap());
    let composition_path = fixture("tests/fixtures/openfilm/finite-v1/repeat-source.json");
    let composition_before = std::fs::read(&composition_path).unwrap();

    let prepared = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("prepare-review")
        .arg(&composition_path)
        .arg(&context_path)
        .output()
        .unwrap();
    assert_eq!(prepared.status.code(), Some(0));
    let prepared_json: Value = serde_json::from_slice(&prepared.stdout).unwrap();
    let record = prepared_json["review"].clone();
    assert_eq!(record["scope"], "declared-review-binding-only");
    assert_eq!(record["executable"], false);
    assert_eq!(record["approval_granted"], false);
    assert_eq!(record["source_verified"], false);
    assert_eq!(record["host_verified"], false);
    let record_path = temp.write("record", &serde_json::to_vec(&record).unwrap());
    let record_before = std::fs::read(&record_path).unwrap();
    let context_before = std::fs::read(&context_path).unwrap();

    let current = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("check-review")
        .arg(&record_path)
        .arg(&composition_path)
        .arg(&context_path)
        .output()
        .unwrap();
    assert_eq!(current.status.code(), Some(0));
    let current_json: Value = serde_json::from_slice(&current.stdout).unwrap();
    assert_eq!(current_json["freshness"]["current"], true);
    assert_eq!(current_json["freshness"]["stale_fields"], json!([]));

    let mut changed_bytes = composition_before.clone();
    changed_bytes.push(b'\n');
    let changed_path = temp.write("changed-composition", &changed_bytes);
    let stale = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("check-review")
        .arg(&record_path)
        .arg(&changed_path)
        .arg(&context_path)
        .output()
        .unwrap();
    assert_eq!(stale.status.code(), Some(0));
    let stale_json: Value = serde_json::from_slice(&stale.stdout).unwrap();
    assert_eq!(stale_json["freshness"]["current"], false);
    assert_eq!(
        stale_json["freshness"]["stale_fields"],
        json!(["composition"])
    );

    assert_eq!(
        std::fs::read(&composition_path).unwrap(),
        composition_before
    );
    assert_eq!(std::fs::read(&record_path).unwrap(), record_before);
    assert_eq!(std::fs::read(&context_path).unwrap(), context_before);
}

#[test]
fn prepare_review_reads_composition_from_stdin_but_context_remains_an_explicit_file() {
    let mut temp = TempFiles::new();
    let context_path = temp.write("context", &serde_json::to_vec(&context()).unwrap());
    let mut child = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("prepare-review")
        .arg("-")
        .arg(context_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(include_bytes!(
            "../../../tests/fixtures/openfilm/finite-v1/repeat-source.json"
        ))
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["ok"], true);
}

#[test]
fn review_commands_keep_usage_and_io_errors_distinct_and_path_free() {
    for args in [
        vec!["prepare-review"],
        vec!["prepare-review", "composition.json"],
        vec!["check-review", "record.json", "composition.json"],
        vec!["check-review", "-", "composition.json", "context.json"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["error"]["code"], "cli.usage");
    }

    let mut temp = TempFiles::new();
    let missing_path = temp.write("removed-context", b"{}");
    std::fs::remove_file(&missing_path).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("prepare-review")
        .arg(fixture(
            "tests/fixtures/openfilm/finite-v1/repeat-source.json",
        ))
        .arg(&missing_path)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("removed-context"));
}
