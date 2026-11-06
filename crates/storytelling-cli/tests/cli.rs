use std::process::Command;

#[test]
fn accepts_valid_file_with_machine_readable_output() {
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/minimal-plan.json");
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling")).arg("validate").arg(input).output().unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["ok"], true);
    assert_eq!(json["validation"]["scope"], "declared-metadata-only");
}

#[test]
fn rejects_bad_source_range_with_exit_one() {
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/invalid-source-range.json");
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling")).arg("validate").arg(input).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["error"]["code"], "plan.source_out_of_bounds");
}
