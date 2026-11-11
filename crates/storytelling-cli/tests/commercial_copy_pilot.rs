// SPDX-License-Identifier: MIT
//! Synthetic N-R05 pilot: citation validity remains separate from wording support.
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

const CASES: [(&str, &str, &str); 4] = [
    (
        "n05-pilot-copy-v1.json",
        "n05-pilot-ledger-v1.json",
        "normal",
    ),
    (
        "n05-pilot-copy-no-reference-v1.json",
        "n05-pilot-ledger-v1.json",
        "missing",
    ),
    (
        "n05-pilot-copy-dangling-ref-v1.json",
        "n05-pilot-ledger-v1.json",
        "dangling",
    ),
    (
        "n05-pilot-copy-v1.json",
        "n05-pilot-ledger-revised-v1.json",
        "revised",
    ),
];

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/commercial-copy-v1")
        .join(name)
}

fn run(copy: &str, ledger: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("audit-copy-claims")
        .arg(fixture(copy))
        .arg(fixture(ledger))
        .arg("--report-v2")
        .output()
        .unwrap()
}

fn report(output: &Output) -> Value {
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn expected(case: &str) -> Value {
    serde_json::from_slice(
        &std::fs::read(fixture(&format!("expected/n05-pilot-{case}.json"))).unwrap(),
    )
    .unwrap()
}

fn check_case(index: usize) -> Value {
    let (copy, ledger, case) = CASES[index];
    let actual = report(&run(copy, ledger));
    assert_eq!(actual, expected(case), "handwritten {case} expectation");
    actual["commercial_copy"].clone()
}

#[test]
fn current_citation_does_not_certify_scoped_or_overbroad_product_wording() {
    let copy = check_case(0);
    assert_eq!(copy["reference_attention_claim_ids"], json!([]));
    for claim in &copy["claims"].as_array().unwrap()[..2] {
        assert_eq!(claim["reference_status"], "current");
        assert_eq!(claim["support_status"], "human_review_required");
        assert_eq!(claim["evidence_refs"][0]["unit_id"], "PILOT_LOG");
        assert_eq!(claim["evidence_refs"][0]["revision"], 1);
    }
    assert_eq!(
        copy["claims"][0]["wording"],
        "試行組平均整理時間由 10 分鐘降到 8 分鐘"
    );
    assert_eq!(copy["claims"][1]["wording"], "所有團隊效率都提升 20%");
    assert_eq!(copy["claims"][3]["kind"], "creative_proposal");
    assert_eq!(copy["claims"][3]["support_status"], "not_applicable");
    assert_eq!(
        copy["claims"][3]["evidence_refs"][0]["cited_source_origin"],
        "creative_proposal"
    );
    assert_eq!(copy["call_to_action"]["kind"], "creative_proposal");
}

#[test]
fn missing_citation_identifies_the_claim_without_rewriting_its_wording() {
    let copy = check_case(1);
    assert_eq!(
        copy["reference_attention_claim_ids"],
        json!(["pilot-average-time"])
    );
    assert_eq!(copy["claims"][0]["reference_status"], "missing");
    assert_eq!(copy["claims"][1]["reference_status"], "current");
}

#[test]
fn dangling_citation_identifies_the_claim_and_keeps_the_declared_reference() {
    let copy = check_case(2);
    assert_eq!(
        copy["reference_attention_claim_ids"],
        json!(["pilot-average-time"])
    );
    assert_eq!(copy["claims"][0]["reference_status"], "dangling");
    assert_eq!(
        copy["claims"][0]["evidence_refs"][0]["unit_id"],
        "MISSING_LOG"
    );
    assert_eq!(
        copy["claims"][0]["evidence_refs"][0]["current_source_text"],
        Value::Null
    );
}

#[test]
fn source_revision_flags_both_pilot_claims_and_preserves_other_copy() {
    let copy = check_case(3);
    let before = expected("normal")["commercial_copy"].clone();
    assert_eq!(
        copy["reference_attention_claim_ids"],
        json!(["pilot-average-time", "all-teams-efficiency"])
    );
    for index in 0..2 {
        assert_eq!(
            copy["claims"][index]["wording"],
            before["claims"][index]["wording"]
        );
        let cited = &copy["claims"][index]["evidence_refs"][0];
        assert_eq!(cited["revision"], 1);
        assert_eq!(cited["current_revision"], 2);
        assert_eq!(
            cited["cited_source_text"],
            before["claims"][index]["evidence_refs"][0]["cited_source_text"]
        );
        assert_ne!(cited["cited_source_text"], cited["current_source_text"]);
    }
    assert_eq!(copy["claims"][2], before["claims"][2]);
    assert_eq!(copy["claims"][3], before["claims"][3]);
    assert_eq!(copy["call_to_action"], before["call_to_action"]);
}

#[test]
fn all_four_cases_replay_identically_and_leave_every_input_unchanged() {
    let names = [
        "n05-pilot-copy-v1.json",
        "n05-pilot-copy-no-reference-v1.json",
        "n05-pilot-copy-dangling-ref-v1.json",
        "n05-pilot-ledger-v1.json",
        "n05-pilot-ledger-revised-v1.json",
    ];
    let before: Vec<_> = names
        .iter()
        .map(|name| std::fs::read(fixture(name)).unwrap())
        .collect();
    for (copy, ledger, case) in CASES {
        let first = run(copy, ledger);
        let second = run(copy, ledger);
        assert_eq!(report(&first), expected(case));
        assert_eq!(report(&second), expected(case));
        assert_eq!(first.stdout, second.stdout, "{case} replay bytes");
    }
    for (name, original) in names.iter().zip(before) {
        assert_eq!(std::fs::read(fixture(name)).unwrap(), original, "{name}");
    }
}

#[test]
fn default_report_v1_preserves_the_reviewed_cli_receipt_bytes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let copy_path = fixture("n05-copy-reviewed-v1.json");
    let ledger_path = root.join("tests/fixtures/narrative-v1/n04-revision-history-v1.json");
    let copy_before = std::fs::read(&copy_path).unwrap();
    let ledger_before = std::fs::read(&ledger_path).unwrap();
    let historical =
        std::fs::read(root.join("docs/validation/n05-commercial-copy-reviewed.stdout")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("audit-copy-claims")
        .arg(&copy_path)
        .arg(&ledger_path)
        .output()
        .unwrap();
    let actual = report(&output);
    assert_eq!(output.stdout, historical, "accepted ReportV1 receipt bytes");
    let copy = &actual["commercial_copy"];
    assert_eq!(copy["schema_version"], 1);
    assert!(copy.get("reference_attention_claim_ids").is_none());
    for claim in copy["claims"]
        .as_array()
        .unwrap()
        .iter()
        .chain(std::iter::once(&copy["call_to_action"]))
    {
        assert!(claim.get("support_status").is_none());
        for reference in claim["evidence_refs"].as_array().unwrap() {
            assert!(reference.get("current_source_text").is_none());
        }
    }
    assert_eq!(std::fs::read(copy_path).unwrap(), copy_before);
    assert_eq!(std::fs::read(ledger_path).unwrap(), ledger_before);
}

struct TemporaryLedger(PathBuf);

impl TemporaryLedger {
    fn new(bytes: &[u8]) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "storytelling-pilot-ledger-{}-{}-{}.json",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        file.write_all(bytes).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryLedger {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn run_stdin(copy: &[u8], ledger: &Path, report_v2: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_storytelling"));
    command.arg("audit-copy-claims").arg("-").arg(ledger);
    if report_v2 {
        command.arg("--report-v2");
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(copy).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn report_v2_output_limit_does_not_shrink_accepted_report_v1_capacity() {
    let mut ledger: Value =
        serde_json::from_slice(&std::fs::read(fixture("n05-pilot-ledger-v1.json")).unwrap())
            .unwrap();
    ledger["base_document"]["units"][0]["text"] = "x".repeat(8000).into();
    let ledger_bytes = serde_json::to_vec(&ledger).unwrap();
    let temporary = TemporaryLedger::new(&ledger_bytes);
    let mut copy: Value =
        serde_json::from_slice(&std::fs::read(fixture("n05-pilot-copy-v1.json")).unwrap()).unwrap();
    copy["claims"] = Value::Array(
        (0..80)
            .map(|index| {
                json!({
                    "id": format!("capacity-{index}"),
                    "kind": "factual_claim",
                    "wording": "Synthetic capacity claim awaiting human review.",
                    "evidence_refs": [{"unit_id": "PILOT_LOG", "revision": 1}]
                })
            })
            .collect(),
    );
    let copy_bytes = serde_json::to_vec(&copy).unwrap();

    let v1 = report(&run_stdin(&copy_bytes, &temporary.0, false));
    assert_eq!(v1["commercial_copy"]["schema_version"], 1);
    let v1_bytes = serde_json::to_vec(&v1["commercial_copy"]).unwrap();
    assert!(v1_bytes.len() > 600_000);
    assert!(v1_bytes.len() <= 1_048_576);

    let v2 = run_stdin(&copy_bytes, &temporary.0, true);
    assert_eq!(v2.status.code(), Some(1));
    assert!(v2.stderr.is_empty());
    let error: Value = serde_json::from_slice(&v2.stdout).unwrap();
    assert_eq!(error["error"]["code"], "copy.output_too_large");
    assert_eq!(std::fs::read(&temporary.0).unwrap(), ledger_bytes);
}

#[test]
fn report_selection_rejects_bad_flags_and_does_not_upgrade_input_schema() {
    let copy_path = fixture("n05-pilot-copy-v1.json");
    let ledger_path = fixture("n05-pilot-ledger-v1.json");
    let flag = std::ffi::OsString::from("--report-v2");
    let malformed = [
        vec![
            flag.clone(),
            copy_path.clone().into(),
            ledger_path.clone().into(),
        ],
        vec![
            copy_path.clone().into(),
            flag.clone(),
            ledger_path.clone().into(),
        ],
        vec![
            copy_path.clone().into(),
            ledger_path.clone().into(),
            flag.clone(),
            flag,
        ],
        vec![
            copy_path.clone().into(),
            ledger_path.clone().into(),
            "--report-v2=true".into(),
        ],
        vec![
            copy_path.clone().into(),
            ledger_path.clone().into(),
            "--report-v3".into(),
        ],
    ];
    for args in malformed {
        let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .arg("audit-copy-claims")
            .args(&args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{args:?}: {output:?}");
        assert!(output.stderr.is_empty());
        let error: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(error["error"]["code"], "cli.usage");
    }

    let mut copy: Value = serde_json::from_slice(&std::fs::read(copy_path).unwrap()).unwrap();
    copy["schema_version"] = 2.into();
    let output = run_stdin(&serde_json::to_vec(&copy).unwrap(), &ledger_path, true);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let error: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(error["error"]["code"], "copy.unsupported_version");
}
