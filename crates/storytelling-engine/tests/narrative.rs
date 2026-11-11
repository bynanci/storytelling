// SPDX-License-Identifier: MIT

use serde_json::{Value, json};
use storytelling_core::narrative::NarrativeProjectedVariantV1;
use storytelling_engine::{MAX_INPUT_BYTES, render_narrative};

const VALID: &[u8] = include_bytes!("../../../tests/fixtures/narrative-v1/valid-v1.json");
const N03: &[u8] = include_bytes!("../../../tests/fixtures/narrative-v1/n03-authoring-v1.json");

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/narrative-v1")
            .join(name),
    )
    .unwrap()
}

#[test]
fn accepted_fixtures_project_variants_and_keep_source_provenance() {
    for (bytes, expected_id) in [(VALID, "door-repair"), (N03, "door-repair")] {
        let projection = render_narrative(bytes).unwrap();
        assert_eq!(projection.document_id, expected_id);
        assert!(projection.variants.len() >= 2);
        match &projection.variants[0] {
            NarrativeProjectedVariantV1::SpokenOutline { beats, .. } => {
                assert!(!beats.is_empty());
                assert_eq!(beats[0].source.unit_id, "S01");
                assert_eq!(beats[0].source.revision, 1);
                assert_eq!(beats[0].source.origin.as_str(), "supplied_observation");
            }
            _ => panic!("first declared variant must remain the oral outline"),
        }
        match &projection.variants[1] {
            NarrativeProjectedVariantV1::SingleIllustration { sources, .. } => {
                assert!(sources.iter().any(|source| source.unit_id == "C01"));
                assert!(sources.iter().all(|source| source.revision == 1));
            }
            _ => panic!("second declared variant must remain the illustration"),
        }
    }
}

#[test]
fn paired_contract_fixtures_return_their_declared_diagnostics() {
    let cases = [
        ("invalid-order-cycle.json", "narrative.order_conflict"),
        (
            "invalid-multiple-order-cycles.json",
            "narrative.order_conflict",
        ),
        (
            "invalid-dangling-reference.json",
            "narrative.dangling_reference",
        ),
        ("invalid-stale-revision.json", "narrative.stale_revision"),
        ("invalid-unknown-field.json", "narrative.unknown_field"),
        (
            "invalid-unknown-version.json",
            "narrative.unsupported_version",
        ),
        ("invalid-duplicate-field.json", "narrative.duplicate_field"),
    ];
    for (name, expected_code) in cases {
        let result = render_narrative(&fixture(name));
        assert_eq!(result.unwrap_err().code, expected_code, "{name}");
    }

    let cycle = render_narrative(&fixture("invalid-multiple-order-cycles.json")).unwrap_err();
    assert_eq!(cycle.relation_ids, ["R03", "R04", "R05"]);
}

#[test]
fn provenance_reference_relation_scope_and_explicit_order_rules_reject() {
    fn changed(change: impl FnOnce(&mut Value)) -> Vec<u8> {
        let mut document: Value = serde_json::from_slice(VALID).unwrap();
        change(&mut document);
        serde_json::to_vec(&document).unwrap()
    }

    let cases: Vec<(Vec<u8>, &str)> = vec![
        (
            changed(|document| {
                document["units"][0]["based_on"] = json!([{"unit_id":"S02","revision":1}]);
            }),
            "narrative.observation_has_basis",
        ),
        (
            changed(|document| document["units"][4]["based_on"] = json!([])),
            "narrative.missing_basis",
        ),
        (
            changed(|document| {
                document["units"][0]["origin"] = json!("author_interpretation");
                document["units"][0]["based_on"] = json!([{"unit_id":"I01","revision":1}]);
                document["units"][4]["based_on"][0]["unit_id"] = json!("S01");
            }),
            "narrative.derivation_cycle",
        ),
        (
            changed(|document| {
                document["variants"][0]["beats"]
                    .as_array_mut()
                    .unwrap()
                    .swap(0, 1);
            }),
            "narrative.order_violation",
        ),
        (
            changed(|document| {
                document["relations"][2]["variant_id"] = json!("illustration-wide");
            }),
            "narrative.invalid_relation_scope",
        ),
        (
            changed(|document| {
                document["variants"][0]["beats"][1]["unit_ref"]["unit_id"] = json!("S01");
            }),
            "narrative.duplicate_reference",
        ),
    ];
    for (bytes, expected_code) in cases {
        assert_eq!(render_narrative(&bytes).unwrap_err().code, expected_code);
    }
}

#[test]
fn diagnostic_precedence_checks_all_ids_and_references_before_basis_and_scope() {
    fn changed(change: impl FnOnce(&mut Value)) -> Vec<u8> {
        let mut document: Value = serde_json::from_slice(VALID).unwrap();
        change(&mut document);
        serde_json::to_vec(&document).unwrap()
    }

    let bad_reference_before_missing_basis = changed(|document| {
        document["units"][4]["based_on"] = json!([]);
        document["variants"][0]["beats"][0]["unit_ref"]["unit_id"] = json!("MISSING");
    });
    assert_eq!(
        render_narrative(&bad_reference_before_missing_basis)
            .unwrap_err()
            .code,
        "narrative.dangling_reference"
    );

    let later_duplicate_relation_id_before_earlier_dangling_reference = changed(|document| {
        document["relations"][0]["from"]["unit_id"] = json!("MISSING");
        document["relations"][1]["id"] = json!("R01");
    });
    assert_eq!(
        render_narrative(&later_duplicate_relation_id_before_earlier_dangling_reference)
            .unwrap_err()
            .code,
        "narrative.duplicate_id"
    );
}

#[test]
fn raw_byte_and_container_depth_boundaries_are_inclusive() {
    let mut exact_bytes = VALID.to_vec();
    exact_bytes.extend(std::iter::repeat_n(
        b' ',
        MAX_INPUT_BYTES - exact_bytes.len(),
    ));
    assert_eq!(exact_bytes.len(), MAX_INPUT_BYTES);
    assert!(render_narrative(&exact_bytes).is_ok());
    exact_bytes.push(b' ');
    assert_eq!(
        render_narrative(&exact_bytes).unwrap_err().code,
        "input.too_large"
    );

    // The root object has depth one. Thirty-one nested arrays make 32 containers.
    let at_limit = format!(
        "{{\"schema_version\":1,\"extra\":{}0{}}}",
        "[".repeat(31),
        "]".repeat(31)
    );
    assert_eq!(
        render_narrative(at_limit.as_bytes()).unwrap_err().code,
        "narrative.unknown_field"
    );
    let over_limit = format!(
        "{{\"schema_version\":1,\"extra\":{}0{}}}",
        "[".repeat(32),
        "]".repeat(32)
    );
    assert_eq!(
        render_narrative(over_limit.as_bytes()).unwrap_err().code,
        "narrative.depth_exceeded"
    );
}
