// SPDX-License-Identifier: Apache-2.0
// Uses existing synthetic finite-v1 fixtures; no upstream source/test is copied.
use serde_json::{Value, json};
use storytelling_core::review::{ReviewContextV1, ReviewHostContextV1, ReviewLossChoiceV1};
use storytelling_engine::{
    MAX_REVIEW_CONTEXT_BYTES, MAX_REVIEW_RECORD_BYTES, check_review, parse_review_context,
    prepare_review,
};

const REPEATED: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/repeat-source.json");
const REQUIRED_CAPABILITIES: &[&str] = &[
    "arbitrary-content-tick-rate",
    "external-reference",
    "single-video-track",
    "still-hold",
];

fn context() -> ReviewContextV1 {
    ReviewContextV1 {
        schema_version: 1,
        project_id: "project-1".to_owned(),
        target_revision: "new-isolated-project".to_owned(),
        host: ReviewHostContextV1 {
            id: "filmcraft".to_owned(),
            version: "0.18.0".to_owned(),
            profile_id: "isolated25fps".to_owned(),
            binary_sha256: "a".repeat(64),
            declared_capabilities: REQUIRED_CAPABILITIES
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
        },
        loss_choices: vec![
            ReviewLossChoiceV1::MetadataOnlyLocks,
            ReviewLossChoiceV1::PictureOnly,
        ],
    }
}

fn context_bytes(value: &ReviewContextV1) -> Vec<u8> {
    serde_json::to_vec(value).unwrap()
}

fn record_bytes(snapshot: &[u8], declared: &ReviewContextV1) -> Vec<u8> {
    let bound = prepare_review(snapshot, declared).expect("valid finite review binding");
    serde_json::to_vec(bound.record()).unwrap()
}

#[test]
fn same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact() {
    let declared = context();
    let first = prepare_review(REPEATED, &declared).unwrap();
    let second = prepare_review(REPEATED, &declared).unwrap();
    assert_eq!(
        first.record().binding_sha256,
        second.record().binding_sha256
    );
    assert_eq!(first.compiled_otio(), second.compiled_otio());
    assert_eq!(first.record().scope, "declared-review-binding-only");
    assert!(!first.record().executable);
    assert!(!first.record().approval_granted);
    assert!(!first.record().source_verified);
    assert!(!first.record().host_verified);
    let portable_record = serde_json::to_vec(first.record()).unwrap();
    let portable_record = String::from_utf8(portable_record).unwrap();
    assert!(!portable_record.contains("project-1"));
    assert!(!portable_record.contains("filmcraft"));
    assert!(!portable_record.contains("0.18.0"));

    let report = check_review(
        portable_record.as_bytes(),
        REPEATED,
        &context_bytes(&declared),
    )
    .unwrap();
    assert!(report.current);
    assert!(report.stale_fields.is_empty());
    assert!(!report.approval_granted);
    assert!(!report.executable);
}

#[test]
fn changed_composition_and_each_declared_context_field_are_reported_stale() {
    let declared = context();
    let record = record_bytes(REPEATED, &declared);

    let mut whitespace_changed = REPEATED.to_vec();
    whitespace_changed.push(b'\n');
    let report = check_review(&record, &whitespace_changed, &context_bytes(&declared)).unwrap();
    assert!(!report.current);
    assert!(report.stale_fields.contains(&"composition".to_owned()));
    // The trailing newline is outside RawValue's source range, so the exporter
    // emits the same document even though exact input-byte identity is stale.
    assert!(!report.stale_fields.contains(&"artifact".to_owned()));

    let mut changed_context = declared.clone();
    changed_context.project_id = "project-2".to_owned();
    let report = check_review(&record, REPEATED, &context_bytes(&changed_context)).unwrap();
    assert!(!report.current);
    assert_eq!(report.stale_fields, vec!["context.project_id"]);
}

#[test]
fn every_declared_context_field_has_a_stable_stale_identifier() {
    let declared = context();
    let record = record_bytes(REPEATED, &declared);
    let cases = [
        ("context.target_revision", 0),
        ("context.host.id", 1),
        ("context.host.version", 2),
        ("context.host.profile_id", 3),
        ("context.host.binary_sha256", 4),
        ("context.host.declared_capabilities", 5),
        ("context.loss_choices", 6),
    ];
    for (expected, case) in cases {
        let mut changed = declared.clone();
        match case {
            0 => changed.target_revision = "revision-2".to_owned(),
            1 => changed.host.id = "filmcraft-next".to_owned(),
            2 => changed.host.version = "0.18.1".to_owned(),
            3 => changed.host.profile_id = "isolated24fps".to_owned(),
            4 => changed.host.binary_sha256 = "b".repeat(64),
            5 => changed
                .host
                .declared_capabilities
                .push("z-extra".to_owned()),
            6 => changed
                .loss_choices
                .retain(|choice| *choice != ReviewLossChoiceV1::PictureOnly),
            _ => unreachable!(),
        }
        changed.host.declared_capabilities.sort();
        changed.loss_choices.sort();
        let report = check_review(&record, REPEATED, &context_bytes(&changed)).unwrap();
        assert!(!report.current, "{expected}");
        assert!(
            report.stale_fields.contains(&expected.to_owned()),
            "{expected}"
        );
    }
}

#[test]
fn semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding() {
    let declared = context();
    let record = record_bytes(REPEATED, &declared);
    let mut snapshot: Value = serde_json::from_slice(REPEATED).unwrap();
    let clips = snapshot["composition"]["tracks"][0]["clips"]
        .as_array_mut()
        .unwrap();
    clips.reverse();
    clips[0]["timelineStart"] = json!(0);
    clips[1]["timelineStart"] = json!(3);
    let reordered = serde_json::to_vec(&snapshot).unwrap();

    let report = check_review(&record, &reordered, &context_bytes(&declared)).unwrap();
    assert!(!report.current);
    assert!(report.stale_fields.contains(&"composition".to_owned()));
    assert!(report.stale_fields.contains(&"plan".to_owned()));
    assert!(report.stale_fields.contains(&"artifact".to_owned()));
}

#[test]
fn context_parser_rejects_unknown_duplicate_unsorted_and_over_budget_values() {
    let valid = context_bytes(&context());
    let parsed = parse_review_context(&valid).unwrap();
    assert_eq!(parsed, context());

    let mut unknown: Value = serde_json::from_slice(&valid).unwrap();
    unknown["unrecognized"] = json!(true);
    let error = parse_review_context(&serde_json::to_vec(&unknown).unwrap()).unwrap_err();
    assert_eq!(error.code, "review.invalid_context_json");

    let mut null_required: Value = serde_json::from_slice(&valid).unwrap();
    null_required["host"]["version"] = Value::Null;
    assert_eq!(
        parse_review_context(&serde_json::to_vec(&null_required).unwrap())
            .unwrap_err()
            .code,
        "review.invalid_context_json"
    );

    let valid_text = std::str::from_utf8(&valid).unwrap();
    let duplicate = valid_text
        .replacen(
            "\"schema_version\":1,",
            "\"schema_version\":1,\"schema_version\":1,",
            1,
        )
        .into_bytes();
    assert_eq!(
        parse_review_context(&duplicate).unwrap_err().code,
        "review.invalid_context_json"
    );

    let mut unsorted = context();
    unsorted.host.declared_capabilities.reverse();
    assert_eq!(
        parse_review_context(&context_bytes(&unsorted))
            .unwrap_err()
            .code,
        "review.noncanonical_order"
    );

    assert_eq!(
        parse_review_context(&vec![b' '; MAX_REVIEW_CONTEXT_BYTES + 1])
            .unwrap_err()
            .code,
        "review.context_too_large"
    );
}

#[test]
fn json_depth_and_record_byte_budgets_are_enforced_before_deserialization() {
    let depth = 65;
    let too_deep = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
    assert_eq!(
        parse_review_context(too_deep.as_bytes()).unwrap_err().code,
        "review.json_too_deep"
    );

    let oversized_record = vec![b' '; MAX_REVIEW_RECORD_BYTES + 1];
    let error = check_review(&oversized_record, REPEATED, &context_bytes(&context())).unwrap_err();
    assert_eq!(error.code, "review.record_too_large");
}

#[test]
fn exact_context_and_record_byte_limits_are_accepted() {
    let declared = context();
    let mut context_json = context_bytes(&declared);
    context_json.resize(MAX_REVIEW_CONTEXT_BYTES, b' ');
    assert_eq!(parse_review_context(&context_json).unwrap(), declared);

    let record = record_bytes(REPEATED, &declared);
    let mut padded_record = record;
    padded_record.resize(MAX_REVIEW_RECORD_BYTES, b' ');
    let report = check_review(&padded_record, REPEATED, &context_bytes(&declared)).unwrap();
    assert!(report.current);
}

#[test]
fn capability_collection_accepts_exact_limit_and_rejects_limit_plus_one() {
    let mut declared = context();
    declared
        .host
        .declared_capabilities
        .extend((0..60).map(|index| format!("z-capability-{index:02}")));
    declared.host.declared_capabilities.sort();
    assert_eq!(declared.host.declared_capabilities.len(), 64);
    assert!(parse_review_context(&context_bytes(&declared)).is_ok());

    declared
        .host
        .declared_capabilities
        .push("zz-extra".to_owned());
    declared.host.declared_capabilities.sort();
    assert_eq!(
        parse_review_context(&context_bytes(&declared))
            .unwrap_err()
            .code,
        "review.too_many"
    );
}

#[test]
fn context_must_declare_compiler_required_capabilities_and_record_digest_is_checked() {
    let mut missing_capabilities = context();
    missing_capabilities.host.declared_capabilities.clear();
    let error = prepare_review(REPEATED, &missing_capabilities).unwrap_err();
    assert_eq!(error.code, "review.context_missing_capability");

    let declared = context();
    let mut record: Value = serde_json::from_slice(&record_bytes(REPEATED, &declared)).unwrap();
    record["binding_sha256"] = json!("0".repeat(64));
    let error = check_review(
        &serde_json::to_vec(&record).unwrap(),
        REPEATED,
        &context_bytes(&declared),
    )
    .unwrap_err();
    assert_eq!(error.code, "review.record_digest_mismatch");
}

#[test]
fn review_record_rejects_unknown_duplicate_forged_claims_and_invalid_digests() {
    let declared = context();
    let valid = record_bytes(REPEATED, &declared);
    let mut unknown: Value = serde_json::from_slice(&valid).unwrap();
    unknown["extra"] = json!(true);
    assert_eq!(
        check_review(
            &serde_json::to_vec(&unknown).unwrap(),
            REPEATED,
            &context_bytes(&declared)
        )
        .unwrap_err()
        .code,
        "review.invalid_record_json"
    );

    let valid_text = std::str::from_utf8(&valid).unwrap();
    let duplicate = valid_text
        .replacen(
            "\"schema_version\":1,",
            "\"schema_version\":1,\"schema_version\":1,",
            1,
        )
        .into_bytes();
    assert_eq!(
        check_review(&duplicate, REPEATED, &context_bytes(&declared))
            .unwrap_err()
            .code,
        "review.invalid_record_json"
    );

    let mut forged_claim: Value = serde_json::from_slice(&valid).unwrap();
    forged_claim["approval_granted"] = json!(true);
    assert_eq!(
        check_review(
            &serde_json::to_vec(&forged_claim).unwrap(),
            REPEATED,
            &context_bytes(&declared)
        )
        .unwrap_err()
        .code,
        "review.invalid_claims"
    );

    let mut invalid_digest: Value = serde_json::from_slice(&valid).unwrap();
    invalid_digest["binding_sha256"] = json!("A".repeat(64));
    assert_eq!(
        check_review(
            &serde_json::to_vec(&invalid_digest).unwrap(),
            REPEATED,
            &context_bytes(&declared)
        )
        .unwrap_err()
        .code,
        "review.invalid_digest"
    );
}
