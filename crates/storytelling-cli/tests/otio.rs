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
            "storytelling-otio-{}-{}-{}",
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
        .join("../../tests/fixtures/openfilm/finite-v1")
        .join(name)
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn stdin_output(bytes: &[u8], flags: &[&str]) -> Output {
    let mut child = cli()
        .args(["export-otio", "-"])
        .args(flags)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Err(error) = child.stdin.take().unwrap().write_all(bytes) {
        // A bounded consumer can close stdin before all oversized bytes are sent.
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    child.wait_with_output().unwrap()
}

fn json_line(output: &Output, exit: i32) -> Value {
    assert_eq!(
        output.status.code(),
        Some(exit),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
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

fn timeline(output: &Output) -> Value {
    let document = json_line(output, 0);
    assert_eq!(document["OTIO_SCHEMA"], "Timeline.1");
    assert!(
        document.get("ok").is_none(),
        "success must be the OTIO document"
    );
    assert!(document.get("document").is_none());
    assert_eq!(document["tracks"]["OTIO_SCHEMA"], "Stack.1");
    assert_eq!(document["tracks"]["children"].as_array().unwrap().len(), 1);
    let track = &document["tracks"]["children"][0];
    assert_eq!(track["OTIO_SCHEMA"], "Track.1");
    assert_eq!(track["kind"], "Video");
    assert_eq!(track["children"][0]["OTIO_SCHEMA"], "Clip.1");
    assert_eq!(
        track["children"][0]["media_reference"]["OTIO_SCHEMA"],
        "ExternalReference.1"
    );
    let report = &document["metadata"]["storytelling"]["report"];
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["profile"], "storytelling.otio-visual-ticks.v1");
    assert_eq!(report["scope"], "declared-otio-visual-interchange");
    for field in ["executable", "source_verified", "host_verified"] {
        assert_eq!(report[field], false, "{field} must remain false");
    }
    assert_eq!(report["host_name"], Value::Null);
    assert_eq!(report["host_version"], Value::Null);
    assert!(document["metadata"]["storytelling"]["source_snapshot_json"].is_string());
    document
}

fn compile_error(output: &Output, code: &str, path: &str, losses: Value) {
    assert_eq!(
        json_line(output, 1),
        json!({"ok": false, "error": {"code": code, "path": path, "losses": losses}}),
        "compilation failure must contain no OTIO document or partial result"
    );
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
fn exports_a_real_timeline_identically_from_file_and_stdin_without_writing() {
    let directory = TestDirectory::new();
    let input = directory.join("snapshot.json");
    let original = std::fs::read(fixture("still.json")).unwrap();
    std::fs::write(&input, &original).unwrap();
    std::fs::create_dir(directory.join("untouched")).unwrap();
    std::fs::write(
        directory.join("untouched").join("sentinel.txt"),
        b"synthetic sentinel",
    )
    .unwrap();
    let before = directory_snapshot(&directory.0);

    let file_output = cli()
        .arg("export-otio")
        .arg(&input)
        .arg("--metadata-only-locks")
        .current_dir(&directory.0)
        .output()
        .unwrap();
    let document = timeline(&file_output);
    let stdin_output = stdin_output(&original, &["--metadata-only-locks"]);
    timeline(&stdin_output);
    assert_eq!(stdin_output.stdout, file_output.stdout);
    assert_eq!(document["name"], "finite-still");
    assert_eq!(document["global_start_time"], Value::Null);
    let clip = &document["tracks"]["children"][0]["children"][0];
    assert_eq!(
        clip["metadata"]["storytelling"]["occurrence_id"],
        "still-occurrence"
    );
    assert_eq!(
        clip["media_reference"]["target_url"],
        "synthetic://finite/photo.jpg"
    );
    assert_eq!(clip["source_range"]["duration"]["value"], 5);
    assert_eq!(clip["source_range"]["duration"]["rate"], 1);
    let report = &document["metadata"]["storytelling"]["report"];
    assert_eq!(
        report["choices"],
        json!({"picture_only": false, "metadata_only_locks": true})
    );
    assert_eq!(
        report["import_limitations"],
        json!([
            {"code":"media.not_verified","path":"$.assets","disposition":"not-verified"},
            {"code":"image.hold_window","path":"$.assets[0]","disposition":"intentional-delta"}
        ])
    );
    let source = document["metadata"]["storytelling"]["source_snapshot_json"]
        .as_str()
        .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(source).unwrap(),
        serde_json::from_slice::<Value>(&original).unwrap()
    );
    assert_eq!(std::fs::read(&input).unwrap(), original);
    assert_eq!(directory_snapshot(&directory.0), before);
}

#[test]
fn locked_still_requires_the_explicit_metadata_only_locks_choice() {
    let input = fixture("still.json");
    let expected = json!([{
        "feature":"editing.locks_metadata_only",
        "path":"$",
        "disposition":"metadata-only",
        "accepted":false
    }]);
    for flags in [vec![], vec!["--picture-only"]] {
        let output = cli()
            .arg("export-otio")
            .arg(&input)
            .args(flags)
            .output()
            .unwrap();
        compile_error(
            &output,
            "otio.loss_choice_required",
            "$.choices",
            expected.clone(),
        );
    }
}

#[test]
fn video_requires_picture_only_and_keeps_audio_and_media_limitations() {
    let input = fixture("video-trim.json");
    let expected = json!([{
        "feature":"audio.picture_only",
        "path":"$.assets",
        "disposition":"metadata-only",
        "accepted":false
    }]);
    for flags in [vec![], vec!["--metadata-only-locks"]] {
        let output = cli()
            .arg("export-otio")
            .arg(&input)
            .args(flags)
            .output()
            .unwrap();
        compile_error(
            &output,
            "otio.loss_choice_required",
            "$.choices",
            expected.clone(),
        );
    }
    let output = cli()
        .arg("export-otio")
        .arg(input)
        .arg("--picture-only")
        .output()
        .unwrap();
    let document = timeline(&output);
    let clip = &document["tracks"]["children"][0]["children"][0];
    assert_eq!(clip["source_range"]["start_time"]["value"], 1);
    assert_eq!(clip["source_range"]["start_time"]["rate"], 10);
    assert_eq!(clip["source_range"]["duration"]["value"], 25);
    assert_eq!(clip["source_range"]["duration"]["rate"], 10);
    let report = &document["metadata"]["storytelling"]["report"];
    assert_eq!(report["choices"]["picture_only"], true);
    assert_eq!(
        report["import_limitations"],
        json!([
            {"code":"media.not_verified","path":"$.assets","disposition":"not-verified"},
            {"code":"audio.not_mapped","path":"$.assets[0]","disposition":"blocks-host-handoff"}
        ])
    );
}

#[test]
fn accepts_both_flags_in_either_order_after_the_input() {
    let directory = TestDirectory::new();
    let input = directory.join("locked-video.json");
    let mut source: Value =
        serde_json::from_slice(&std::fs::read(fixture("video-trim.json")).unwrap()).unwrap();
    source["composition"]["tracks"][0]["locked"] = json!(true);
    std::fs::write(&input, serde_json::to_vec(&source).unwrap()).unwrap();
    let mut previous = None;
    for flags in [
        ["--picture-only", "--metadata-only-locks"],
        ["--metadata-only-locks", "--picture-only"],
    ] {
        let output = cli()
            .arg("export-otio")
            .arg(&input)
            .args(flags)
            .output()
            .unwrap();
        let document = timeline(&output);
        assert_eq!(
            document["metadata"]["storytelling"]["report"]["choices"],
            json!({"picture_only":true,"metadata_only_locks":true})
        );
        if let Some(bytes) = previous {
            assert_eq!(output.stdout, bytes);
        }
        previous = Some(output.stdout);
    }
}

#[test]
fn rejects_missing_extra_unknown_duplicate_and_misplaced_arguments_with_usage_two() {
    for args in [
        vec!["export-otio"],
        vec!["export-otio", "--picture-only"],
        vec!["export-otio", "--picture-only", "synthetic.json"],
        vec!["export-otio", "-", "extra"],
        vec!["export-otio", "synthetic.json", "extra"],
        vec!["export-otio", "-", "--unknown"],
        vec!["export-otio", "-", "--picture-only", "--picture-only"],
        vec![
            "export-otio",
            "-",
            "--metadata-only-locks",
            "--metadata-only-locks",
        ],
        vec![
            "export-otio",
            "-",
            "--picture-only",
            "--metadata-only-locks",
            "extra",
        ],
    ] {
        assert_eq!(
            json_line(&cli().args(&args).output().unwrap(), 2),
            json!({"ok":false,"error":{"code":"cli.usage","path":"$"}}),
            "arguments: {args:?}"
        );
    }
}

#[test]
fn redacts_a_missing_input_file_and_uses_io_exit_three() {
    let directory = TestDirectory::new();
    let input = directory.join("synthetic-missing-input.json");
    let output = cli().arg("export-otio").arg(&input).output().unwrap();
    assert_eq!(
        json_line(&output, 3),
        json!({"ok":false,"error":{"code":"input.io","path":"$"}})
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("synthetic-missing-input"));
    assert!(!stdout.contains(directory.0.file_name().unwrap().to_str().unwrap()));
    assert!(directory_snapshot(&directory.0).is_empty());
}

#[test]
fn rejects_more_than_one_mebibyte_from_file_and_stdin_with_validation_exit_one() {
    let directory = TestDirectory::new();
    let input = directory.join("oversized.json");
    let mut bytes = std::fs::read(fixture("still.json")).unwrap();
    bytes.resize(MAX_INPUT_BYTES + 1, b' ');
    std::fs::write(&input, &bytes).unwrap();
    let file_output = cli()
        .arg("export-otio")
        .arg(&input)
        .arg("--metadata-only-locks")
        .output()
        .unwrap();
    for output in [
        file_output,
        stdin_output(&bytes, &["--metadata-only-locks"]),
    ] {
        compile_error(&output, "input.too_large", "$", json!([]));
    }
    assert_eq!(std::fs::read(input).unwrap(), bytes);
}

#[test]
fn unsupported_source_effects_cannot_be_bypassed_with_loss_choices() {
    for flags in [vec![], vec!["--picture-only", "--metadata-only-locks"]] {
        let output = cli()
            .arg("export-otio")
            .arg(fixture("unknown-effect.json"))
            .args(flags)
            .output()
            .unwrap();
        compile_error(
            &output,
            "import.unsupported_feature",
            "$.composition.tracks[0].clips[0].effects",
            json!([{
                "feature":"import.unsupported_feature",
                "path":"$.composition.tracks[0].clips[0].effects",
                "disposition":"unsupported",
                "accepted":false
            }]),
        );
    }
}

#[test]
fn preserves_numeric_lexemes_inside_the_source_snapshot_json_string() {
    let original = std::fs::read_to_string(fixture("still.json")).unwrap();
    let source = original.replacen(
        "\"metadata\": {",
        "\"metadata\": {\"synthetic.integer\":9007199254740993,\"synthetic.decimal\":0.1000,\"synthetic.exponent\":1e-1,",
        1,
    );
    assert_ne!(source, original);
    let output = stdin_output(source.as_bytes(), &["--metadata-only-locks"]);
    let document = timeline(&output);
    // Parse only the outer OTIO JSON: parsing the preserved snapshot as Value
    // would discard precisely the decimal/exponent spellings being checked.
    let snapshot = document["metadata"]["storytelling"]["source_snapshot_json"]
        .as_str()
        .unwrap();
    for token in [
        "\"synthetic.integer\":9007199254740993",
        "\"synthetic.decimal\":0.1000",
        "\"synthetic.exponent\":1e-1",
    ] {
        assert!(snapshot.contains(token), "source snapshot lost {token}");
    }
    assert!(snapshot.contains("\"locked\":true"));
}

#[test]
fn help_lists_export_otio_and_both_loss_choice_flags() {
    let output = cli().arg("--help").output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("export-otio <FILE|->"));
    assert!(help.contains("--picture-only"));
    assert!(help.contains("--metadata-only-locks"));
}
