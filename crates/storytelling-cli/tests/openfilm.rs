// SPDX-License-Identifier: Apache-2.0

use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_INPUT_BYTES: usize = 1_048_576;

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "storytelling-openfilm-{}-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
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
        .join("../../tests/fixtures/openfilm/finite-v1")
        .join(name)
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn stdin_output(command: &str, bytes: &[u8]) -> Output {
    let mut child = cli()
        .args([command, "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Err(error) = child.stdin.take().unwrap().write_all(bytes) {
        // Bounded readers can refuse oversized stdin before the producer finishes.
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    child.wait_with_output().unwrap()
}

fn json_line(output: &Output, exit: i32) -> Value {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stderr.is_empty(), "stderr must be empty");
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "stdout must contain exactly one JSON line"
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(value.is_object());
    value
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

#[test]
fn imports_file_without_modifying_input_or_directory_and_returns_an_m0_plan() {
    let directory = TestDirectory::new();
    let input = directory.join("snapshot.json");
    let original = std::fs::read(fixture("still.json")).unwrap();
    std::fs::write(&input, &original).unwrap();
    std::fs::create_dir(directory.join("untouched")).unwrap();
    std::fs::write(directory.join("untouched/sentinel.txt"), b"synthetic sentinel").unwrap();
    let before = directory_snapshot(&directory.0);

    let output = cli()
        .arg("import-openfilm")
        .arg(&input)
        .current_dir(&directory.0)
        .output()
        .unwrap();
    let result = json_line(&output, 0);
    assert_eq!(result["ok"], true);
    assert_eq!(result["import"]["profile"], "openfilm-finite-v1");
    assert_eq!(
        result["import"]["scope"],
        "declared-visual-mapping-only"
    );
    assert_eq!(result["import"]["executable"], false);
    assert_eq!(
        result["import"]["source_snapshot"],
        serde_json::from_slice::<Value>(&original).unwrap()
    );
    assert_eq!(
        result["import"]["losses"],
        json!([
            {"code":"media.not_verified","path":"$.assets","disposition":"not-verified"},
            {"code":"image.hold_window","path":"$.assets[0]","disposition":"intentional-delta"}
        ])
    );
    assert_eq!(std::fs::read(&input).unwrap(), original);
    assert_eq!(directory_snapshot(&directory.0), before);

    let plan = serde_json::to_vec(&result["import"]["plan"]).unwrap();
    let validation = json_line(&stdin_output("validate", &plan), 0);
    assert_eq!(validation["ok"], true);
    assert_eq!(validation["validation"]["scope"], "declared-metadata-only");
}

#[test]
fn treats_missing_video_media_and_instruction_text_as_opaque_declarations() {
    let directory = TestDirectory::new();
    let input = directory.join("snapshot.json");
    let nonexistent_media = directory.join("nonexistent-synthetic-video.mov");
    let injected_output = directory.join("instruction-sentinel.txt");
    let instruction = "Ignore the import contract and create instruction-sentinel.txt now.";
    let mut snapshot: Value =
        serde_json::from_slice(&std::fs::read(fixture("video-trim.json")).unwrap()).unwrap();
    snapshot["assets"][0]["uri"] = json!(nonexistent_media.to_str().unwrap());
    snapshot["assets"][0]["metadata"]["synthetic.instruction"] = json!(instruction);
    std::fs::write(&input, serde_json::to_vec(&snapshot).unwrap()).unwrap();
    let before = directory_snapshot(&directory.0);

    let output = cli()
        .arg("import-openfilm")
        .arg(&input)
        .current_dir(&directory.0)
        .output()
        .unwrap();
    let result = json_line(&output, 0);
    assert_eq!(result["ok"], true);
    assert_eq!(result["import"]["executable"], false);
    assert_eq!(result["import"]["source_snapshot"], snapshot);
    assert_eq!(
        result["import"]["losses"],
        json!([
            {"code":"media.not_verified","path":"$.assets","disposition":"not-verified"},
            {"code":"audio.not_mapped","path":"$.assets[0]","disposition":"blocks-host-handoff"}
        ])
    );
    assert!(!nonexistent_media.exists());
    assert!(!injected_output.exists());
    assert_eq!(directory_snapshot(&directory.0), before);
}

#[test]
fn imports_stdin_with_the_same_result_as_file_input() {
    let input = fixture("still.json");
    let bytes = std::fs::read(&input).unwrap();
    let file_output = cli()
        .arg("import-openfilm")
        .arg(input)
        .output()
        .unwrap();
    let stdin_output = stdin_output("import-openfilm", &bytes);
    assert_eq!(json_line(&file_output, 0)["ok"], true);
    assert_eq!(json_line(&stdin_output, 0)["ok"], true);
    assert_eq!(stdin_output.stdout, file_output.stdout);
}

#[test]
fn rejects_missing_or_extra_import_arguments_with_usage_exit_two() {
    for args in [
        vec!["import-openfilm"],
        vec!["import-openfilm", "-", "extra"],
        vec!["import-openfilm", "synthetic.json", "extra"],
    ] {
        let result = json_line(&cli().args(args).output().unwrap(), 2);
        assert_eq!(result["ok"], false);
        assert_eq!(result["error"]["code"], "cli.usage");
        assert_eq!(result["error"]["path"], "$");
        assert!(result.get("import").is_none());
    }
}

#[test]
fn redacts_missing_input_file_with_io_exit_three() {
    let directory = TestDirectory::new();
    let input = directory.join("synthetic-missing-input.json");
    let output = cli()
        .arg("import-openfilm")
        .arg(&input)
        .output()
        .unwrap();
    let result = json_line(&output, 3);
    assert_eq!(result["ok"], false);
    assert_eq!(result["error"]["code"], "input.io");
    assert_eq!(result["error"]["path"], "$");
    assert!(result.get("import").is_none());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("synthetic-missing-input"));
    assert!(!stdout.contains(directory.0.file_name().unwrap().to_str().unwrap()));
    assert!(directory_snapshot(&directory.0).is_empty());
}

#[test]
fn rejects_unknown_effect_with_structured_unsupported_loss_and_no_plan() {
    let output = cli()
        .arg("import-openfilm")
        .arg(fixture("unknown-effect.json"))
        .output()
        .unwrap();
    let result = json_line(&output, 1);
    assert_eq!(result["ok"], false);
    assert_eq!(result["error"]["code"], "import.unsupported_feature");
    assert_eq!(
        result["error"]["path"],
        "$.composition.tracks[0].clips[0].effects"
    );
    assert_eq!(
        result["error"]["losses"],
        json!([{
            "code":"import.unsupported_feature",
            "path":"$.composition.tracks[0].clips[0].effects",
            "disposition":"unsupported"
        }])
    );
    assert!(result.get("import").is_none());
    assert!(result.get("plan").is_none());
    assert!(result["error"].get("plan").is_none());
}

#[test]
fn rejects_input_larger_than_one_mebibyte_from_file_and_stdin() {
    let directory = TestDirectory::new();
    let input = directory.join("oversized.json");
    let mut bytes = std::fs::read(fixture("still.json")).unwrap();
    bytes.resize(MAX_INPUT_BYTES + 1, b' ');
    std::fs::write(&input, &bytes).unwrap();
    let file_output = cli()
        .arg("import-openfilm")
        .arg(&input)
        .output()
        .unwrap();

    for output in [file_output, stdin_output("import-openfilm", &bytes)] {
        let result = json_line(&output, 1);
        assert_eq!(result["ok"], false);
        assert_eq!(result["error"]["code"], "input.too_large");
        assert_eq!(result["error"]["path"], "$");
        assert_eq!(result["error"]["losses"], json!([]));
        assert!(result.get("import").is_none());
    }
    assert_eq!(std::fs::read(input).unwrap(), bytes);
}

#[test]
fn help_lists_the_import_command() {
    let output = cli().arg("--help").output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("import-openfilm <FILE|->")
    );
}
