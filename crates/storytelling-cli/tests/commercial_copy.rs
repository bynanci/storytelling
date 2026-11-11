use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

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
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn run_copy_bytes(copy_bytes: &[u8]) -> Output {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "storytelling-commercial-copy-{}-{}-{}.json",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&path, copy_bytes).unwrap();
    let output = cli()
        .arg("audit-copy-claims")
        .arg(&path)
        .arg(ledger())
        .output()
        .unwrap();
    std::fs::remove_file(path).unwrap();
    output
}

#[test]
fn outlines_audience_claims_wording_and_call_to_action_with_reference_only_status() {
    let output = run("n05-copy-v1.json");
    let result = report(&output);
    assert_eq!(result["ok"], true);
    assert_eq!(
        result["commercial_copy"]["audience"],
        "Residents who read the neighborhood bulletin."
    );
    assert_eq!(
        result["commercial_copy"]["claims"][0]["id"],
        "gate-test-result"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][0]["reference_status"],
        "current"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][0]["evidence_refs"][0]["cited_source_text"],
        "兩人試關一次；這一次門沒有磨到地面。"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][1]["reference_status"],
        "missing"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][2]["kind"],
        "creative_proposal"
    );
    assert_eq!(
        result["commercial_copy"]["claims"][2]["reference_status"],
        "not_applicable"
    );
    assert_eq!(
        result["commercial_copy"]["call_to_action"]["kind"],
        "creative_proposal"
    );
    assert_eq!(
        result["commercial_copy"]["reference_scope"],
        "reference-identity-and-revision-only"
    );
}

#[test]
fn labels_a_claim_against_an_older_source_revision_as_revised() {
    let result = report(&run("n05-copy-revised-ref-v1.json"));
    let claim = &result["commercial_copy"]["claims"][0];
    assert_eq!(claim["reference_status"], "revised");
    assert_eq!(claim["evidence_refs"][0]["reference_status"], "revised");
    assert_eq!(claim["evidence_refs"][0]["current_revision"], 2);
    assert_eq!(
        claim["evidence_refs"][0]["cited_source_text"],
        "阿梅扶住門；阿安收緊上方鉸鏈的一顆螺絲。"
    );
}

#[test]
fn labels_a_missing_source_id_as_dangling_without_claiming_support() {
    let result = report(&run("n05-copy-dangling-ref-v1.json"));
    let claim = &result["commercial_copy"]["claims"][0];
    assert_eq!(claim["reference_status"], "dangling");
    assert_eq!(claim["evidence_refs"][0]["reference_status"], "dangling");
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

#[test]
fn rejects_unknown_versions_fields_duplicate_fields_and_mismatched_ledgers() {
    let base = std::fs::read(fixture("n05-copy-v1.json")).unwrap();
    let mut unknown_version: Value = serde_json::from_slice(&base).unwrap();
    unknown_version["schema_version"] = 2.into();
    let output = run_copy_bytes(&serde_json::to_vec(&unknown_version).unwrap());
    assert_eq!(
        report_error(&output)["error"]["code"],
        "copy.unsupported_version"
    );

    let mut unknown_field: Value = serde_json::from_slice(&base).unwrap();
    unknown_field["unexpected"] = true.into();
    let output = run_copy_bytes(&serde_json::to_vec(&unknown_field).unwrap());
    assert_eq!(report_error(&output)["error"]["code"], "copy.unknown_field");

    let duplicated = String::from_utf8(base.clone()).unwrap().replacen(
        "\"copy_id\": \"gate-repair-outreach\",",
        "\"copy_id\": \"first\", \"copy_id\": \"second\",",
        1,
    );
    let output = run_copy_bytes(duplicated.as_bytes());
    assert_eq!(
        report_error(&output)["error"]["code"],
        "copy.duplicate_field"
    );

    let mut wrong_ledger: Value = serde_json::from_slice(&base).unwrap();
    wrong_ledger["evidence_document_id"] = "another-document".into();
    let output = run_copy_bytes(&serde_json::to_vec(&wrong_ledger).unwrap());
    assert_eq!(
        report_error(&output)["error"]["code"],
        "copy.evidence_document_mismatch"
    );
}

fn report_error(output: &Output) -> Value {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}
