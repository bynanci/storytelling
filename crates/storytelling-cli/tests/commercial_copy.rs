use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/commercial-copy-v1")
        .join(name)
}

fn ledger() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/narrative-v1/n04-revision-history-v1.json")
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn run(copy: &str) -> Output {
    cli()
        .arg("audit-copy-claims")
        .arg(fixture(copy))
        .arg(ledger())
        .output()
        .unwrap()
}

fn report(output: &Output) -> Value {
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout.iter().filter(|byte| **byte == b'\n').count(), 1);
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn outlines_audience_claims_wording_and_call_to_action_with_reference_only_status() {
    let output = run("n05-copy-v1.json");
    let result = report(&output);
    assert_eq!(result["ok"], true);
    assert_eq!(result["commercial_copy"]["audience"], "社區電子報讀者");
    assert_eq!(result["commercial_copy"]["claims"][0]["id"], "gate-test-result");
    assert_eq!(
        result["commercial_copy"]["claims"][0]["evidence_status"],
        "current"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][1]["evidence_status"],
        "missing"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][2]["kind"],
        "creative_proposal"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][2]["evidence_status"],
        "not_applicable"
    );
    assert_eq!(
        result["commercial_copy"]["call_to_action"]["kind"],
        "creative_proposal"
    );
    assert_eq!(
        result["commercial_copy"]["evidence_scope"],
        "reference-identity-and-revision-only"
    );
}

#[test]
fn labels_a_claim_against_an_older_source_revision_as_revised() {
    let result = report(&run("n05-copy-revised-ref-v1.json"));
    let claim = &result["commercial_copy"]["claims"][0];
    assert_eq!(claim["evidence_status"], "revised");
    assert_eq!(claim["evidence_refs"][0]["status"], "revised");
    assert_eq!(claim["evidence_refs"][0]["current_revision"], 2);
}

#[test]
fn labels_a_missing_source_id_as_dangling_without_claiming_support() {
    let result = report(&run("n05-copy-dangling-ref-v1.json"));
    let claim = &result["commercial_copy"]["claims"][0];
    assert_eq!(claim["evidence_status"], "dangling");
    assert_eq!(claim["evidence_refs"][0]["status"], "dangling");
    assert_eq!(claim["evidence_refs"][0]["current_revision"], Value::Null);
}

#[test]
fn repeated_file_run_is_identical_and_does_not_modify_input_files() {
    let copy_path = fixture("n05-copy-v1.json");
    let ledger_path = ledger();
    let copy_before = std::fs::read(&copy_path).unwrap();
    let ledger_before = std::fs::read(&ledger_path).unwrap();
    let first = run("n05-copy-v1.json");
    let second = run("n05-copy-v1.json");
    assert_eq!(first.status.code(), Some(0));
    assert_eq!(first.stdout, second.stdout);
    assert_eq!(std::fs::read(copy_path).unwrap(), copy_before);
    assert_eq!(std::fs::read(ledger_path).unwrap(), ledger_before);
}
