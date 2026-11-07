use std::io::Write;
use std::process::{Command, Stdio};

fn fixture(path: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

#[test]
fn accepts_valid_file_with_machine_readable_output() {
    let input = fixture("examples/minimal-plan.json");
    let before = std::fs::read(&input).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("validate")
        .arg(&input)
        .output()
        .unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["ok"], true);
    assert_eq!(json["validation"]["scope"], "declared-metadata-only");
    assert!(output.stderr.is_empty());
    assert_eq!(std::fs::read(&input).unwrap(), before);
}

#[test]
fn rejects_bad_source_range_with_exit_one() {
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("validate")
        .arg(fixture("tests/fixtures/invalid-source-range.json"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["error"]["code"], "plan.source_out_of_bounds");
}

#[test]
fn accepts_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .args(["validate", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(include_bytes!("../../../examples/minimal-plan.json"))
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["ok"], true);
}

#[test]
fn distinguishes_usage_and_io_failures() {
    for args in [vec![], vec!["validate"], vec!["apply", "anything"]] {
        let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(json["error"]["code"], "cli.usage");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("validate")
        .arg(fixture("tests/fixtures/does-not-exist.json"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("does-not-exist"));
}

#[test]
fn reports_help_and_version_without_loading_a_project() {
    for arg in ["--help", "--version"] {
        let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .arg(arg)
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
    }
}
