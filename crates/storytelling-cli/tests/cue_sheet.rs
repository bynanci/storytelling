// SPDX-License-Identifier: MIT

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/narrative-v1")
        .join(name)
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn render(sheet: &str) -> Output {
    cli()
        .arg("render-cue-sheet")
        .arg(fixture("n09-umbrella-source-v1.json"))
        .arg(fixture(sheet))
        .output()
        .unwrap()
}

fn render_stdin(bytes: &[u8]) -> Output {
    let mut child = cli()
        .arg("render-cue-sheet")
        .arg(fixture("n09-umbrella-source-v1.json"))
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    if let Err(error) = std::io::Write::write_all(&mut child.stdin.take().unwrap(), bytes) {
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    child.wait_with_output().unwrap()
}

fn json_error(output: &Output, code: &str) -> Value {
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

#[test]
fn renders_source_linked_cues_in_order_and_replays_byte_for_byte() {
    let input = fixture("n09-cue-sheet-v1.json");
    let original = std::fs::read(&input).unwrap();
    let first = render("n09-cue-sheet-v1.json");
    let second = render("n09-cue-sheet-v1.json");
    assert_eq!(first.status.code(), Some(0));
    assert_eq!(first.stdout, second.stdout);
    assert!(first.stderr.is_empty());
    assert_eq!(std::fs::read(input).unwrap(), original);

    let rendered = String::from_utf8(first.stdout).unwrap();
    assert!(rendered.contains("Time unit: milliseconds"));
    assert!(rendered.find("voice-01").unwrap() < rendered.find("pause-01").unwrap());
    assert!(rendered.find("pause-01").unwrap() < rendered.find("taps-01").unwrap());
    assert!(rendered.contains("週六修傘工作坊，請帶一把傘。"));
    assert!(rendered.contains("S1@1"));
    assert!(rendered.contains("S3@1"));
    assert!(rendered.contains("待錄"));
    assert!(rendered.contains("UNKNOWN"));
    assert!(rendered.contains("ESTIMATED duration 500 ms"));
    assert!(rendered.contains("KNOWN duration 1080 ms"));
    assert!(rendered.contains("taps.wav"));
}

#[test]
fn permits_overlap_on_distinct_exclusive_tracks() {
    let output = render("n09-cue-sheet-cross-track-overlap-v1.json");
    assert_eq!(output.status.code(), Some(0));
    let rendered = String::from_utf8(output.stdout).unwrap();
    assert!(rendered.contains("resolved intervals: 2"));
    assert!(rendered.contains("cross-track overlap allowed"));
}

#[test]
fn permits_half_open_intervals_that_touch_on_one_exclusive_track() {
    let output = render("n09-cue-sheet-adjacent-v1.json");
    assert_eq!(output.status.code(), Some(0));
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("resolved intervals: 2")
    );
}

#[test]
fn reports_unresolved_voice_timing_without_inventing_a_duration() {
    let output = render("n09-cue-sheet-v1.json");
    assert_eq!(output.status.code(), Some(0));
    let rendered = String::from_utf8(output.stdout).unwrap();
    assert!(rendered.contains("unresolved intervals: 3"));
    assert!(rendered.contains("完成錄音後"));
}

#[test]
fn escapes_untrusted_multiline_cue_fields_in_human_readable_output() {
    let mut sheet: Value =
        serde_json::from_slice(&std::fs::read(fixture("n09-cue-sheet-v1.json")).unwrap()).unwrap();
    sheet["cues"][0]["role"] = serde_json::json!("旁白\n99. forged cue");
    sheet["cues"][0]["timing"]["reason"] = serde_json::json!("未定\n99. forged timing");
    sheet["cues"][1]["source"]["description"] = serde_json::json!("停頓\n99. forged source");
    let output = render_stdin(&serde_json::to_vec(&sheet).unwrap());
    assert_eq!(output.status.code(), Some(0));
    let rendered = String::from_utf8(output.stdout).unwrap();
    assert!(rendered.contains(&format!(
        "role={}",
        serde_json::to_string("旁白\n99. forged cue").unwrap()
    )));
    assert!(rendered.contains(&format!(
        "Source: {}",
        serde_json::to_string("Editorial choice · 停頓\n99. forged source").unwrap()
    )));
    assert!(rendered.contains(&format!(
        "Time: {}",
        serde_json::to_string("UNKNOWN — 未定\n99. forged timing").unwrap()
    )));
    assert!(!rendered.contains("\n99. forged"));
}

#[test]
fn rejects_control_characters_in_asset_paths() {
    let mut sheet: Value =
        serde_json::from_slice(&std::fs::read(fixture("n09-cue-sheet-v1.json")).unwrap()).unwrap();
    sheet["assets"][0]["relative_path"] =
        serde_json::json!("skills/storytelling/assets/taps.wav\n99. forged source");
    json_error(
        &render_stdin(&serde_json::to_vec(&sheet).unwrap()),
        "narrative.cue.invalid_asset_path",
    );
}

#[test]
fn rejects_a_reversed_interval() {
    json_error(
        &render("n09-cue-sheet-reversed-v1.json"),
        "narrative.cue.invalid_range",
    );
}

#[test]
fn rejects_duplicate_cue_ids() {
    json_error(
        &render("n09-cue-sheet-duplicate-id-v1.json"),
        "narrative.cue.duplicate_id",
    );
}

#[test]
fn rejects_overlap_on_the_same_exclusive_track() {
    let error = json_error(
        &render("n09-cue-sheet-conflict-v1.json"),
        "narrative.cue.track_conflict",
    );
    assert_eq!(
        error["error"]["cue_ids"],
        serde_json::json!(["pause-01", "voice-01"])
    );
}

#[test]
fn refuses_an_unknown_cue_sheet_version() {
    let error = json_error(
        &render_stdin(br#"{"schema_version":2}"#),
        "narrative.cue.unsupported_version",
    );
    assert_eq!(error["error"]["path"], "$.schema_version");
}

#[test]
fn refuses_a_stale_document_revision() {
    let mut sheet: Value =
        serde_json::from_slice(&std::fs::read(fixture("n09-cue-sheet-v1.json")).unwrap()).unwrap();
    sheet["document_revision"] = serde_json::json!(2);
    let bytes = serde_json::to_vec(&sheet).unwrap();
    let error = json_error(&render_stdin(&bytes), "narrative.cue.stale_document");
    assert_eq!(error["error"]["path"], "$.document_revision");
}

#[test]
fn rejects_a_known_effect_duration_that_disagrees_with_its_asset_declaration() {
    let mut sheet: Value =
        serde_json::from_slice(&std::fs::read(fixture("n09-cue-sheet-v1.json")).unwrap()).unwrap();
    sheet["assets"][0]["duration_ms"] = serde_json::json!(1081);
    let bytes = serde_json::to_vec(&sheet).unwrap();
    let error = json_error(
        &render_stdin(&bytes),
        "narrative.cue.asset_duration_mismatch",
    );
    assert_eq!(error["error"]["path"], "$.cues[2].timing");
}

#[test]
fn accepts_the_cue_sheet_from_stdin_and_preserves_source_input() {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let directory = std::env::temp_dir().join(format!(
        "storytelling-cue-sheet-{}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&directory).unwrap();
    let document = fixture("n09-umbrella-source-v1.json");
    let bytes = std::fs::read(fixture("n09-cue-sheet-v1.json")).unwrap();
    let mut child = cli()
        .arg("render-cue-sheet")
        .arg(document)
        .arg("-")
        .current_dir(&directory)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    if let Err(error) = std::io::Write::write_all(&mut child.stdin.take().unwrap(), &bytes) {
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("taps-01")
    );
    assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 0);
    std::fs::remove_dir(directory).unwrap();
}
