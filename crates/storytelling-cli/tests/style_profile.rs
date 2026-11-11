use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/narrative-v1")
        .join(name)
}

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_storytelling"))
}

fn check(profile: &str) -> Output {
    cli()
        .arg("check-narrative-style")
        .arg(fixture("n08-umbrella-openings-v1.json"))
        .arg(fixture(profile))
        .output()
        .unwrap()
}

fn json_error(output: &Output) -> Value {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn renders_same_facts_with_two_declared_opening_orders_and_provenance() {
    let document: Value =
        serde_json::from_slice(&std::fs::read(fixture("n08-umbrella-openings-v1.json")).unwrap())
            .unwrap();
    assert_eq!(
        document["audience"],
        "附近居民；第一次聽到修傘工作坊邀請的人。"
    );
    let output = cli()
        .arg("render-narrative")
        .arg(fixture("n08-umbrella-openings-v1.json"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let rendered = String::from_utf8(output.stdout).unwrap();
    let documentary_start = rendered.find("## Variant documentary-opening").unwrap();
    let imagery_start = rendered.find("## Variant imagery-opening").unwrap();
    assert!(documentary_start < imagery_start);
    let documentary = &rendered[documentary_start..imagery_start];
    let imagery = &rendered[imagery_start..];
    assert!(documentary.contains("週六有修傘工作坊，請帶一把傘。"));
    assert!(imagery.contains("週六有修傘工作坊，請帶一把傘。"));
    assert!(
        documentary.find("[S1@1 | supplied_observation]").unwrap()
            < documentary.find("[S3@1 | supplied_observation]").unwrap()
    );
    assert!(
        imagery.find("[S3@1 | supplied_observation]").unwrap()
            < imagery.find("[P1@1 | creative_proposal]").unwrap()
    );
    assert!(
        imagery.find("[P1@1 | creative_proposal]").unwrap()
            < imagery.find("[S1@1 | supplied_observation]").unwrap()
    );
}

#[test]
fn checks_both_opening_orders_and_replays_identically() {
    let first = check("n08-style-profile-v1.json");
    let second = check("n08-style-profile-v1.json");
    assert_eq!(first.status.code(), Some(0));
    assert!(first.stderr.is_empty());
    assert_eq!(first.stdout, second.stdout);
    let result: Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(result["ok"], true);
    assert_eq!(
        result["style_check"]["profile_id"],
        "n08-workshop-opening-order"
    );
    assert_eq!(result["style_check"]["checked_rules"], 2);
}

#[test]
fn rejects_contradictory_rules_inside_one_variant_with_rule_ids() {
    let result = json_error(&check("n08-style-profile-conflicting-v1.json"));
    assert_eq!(result["error"]["code"], "narrative.style.order_conflict");
    assert_eq!(result["error"]["path"], "$.rules");
    assert_eq!(
        result["error"]["rule_ids"],
        serde_json::json!(["imagery-taps-before-event", "imagery-event-before-taps"])
    );
}

#[test]
fn refuses_an_unknown_style_profile_version_before_rules() {
    let result = json_error(&check("n08-style-profile-unknown-version.json"));
    assert_eq!(
        result["error"]["code"],
        "narrative.style.unsupported_version"
    );
    assert_eq!(result["error"]["path"], "$.schema_version");
}

#[test]
fn reports_a_valid_rule_that_disagrees_with_declared_beat_order() {
    let result = json_error(&check("n08-style-profile-order-violation-v1.json"));
    assert_eq!(result["error"]["code"], "narrative.style.order_violation");
    assert_eq!(result["error"]["path"], "$.rules[0]");
    assert_eq!(
        result["error"]["rule_ids"],
        serde_json::json!(["documentary-taps-before-event"])
    );
}

#[test]
fn refuses_a_style_profile_bound_to_a_stale_document_revision() {
    let result = json_error(&check("n08-style-profile-stale-document-v1.json"));
    assert_eq!(result["error"]["code"], "narrative.style.stale_document");
    assert_eq!(result["error"]["path"], "$.document_revision");
}

#[test]
fn rejects_unknown_fields_in_the_closed_style_profile_shape() {
    let result = json_error(&check("n08-style-profile-unknown-field.json"));
    assert_eq!(result["error"]["code"], "narrative.style.unknown_field");
}
