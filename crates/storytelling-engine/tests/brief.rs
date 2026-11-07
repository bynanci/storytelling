// SPDX-License-Identifier: Apache-2.0
// Source-informed policy tests; independently authored synthetic declarations.
// Origin: bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76,
// packages/core/src/models.ts (StoryConstraint, StoryBeat) and
// packages/solver/src/index.ts (compose, orderedAssets).
// The brief schema/preferences are downstream design; no upstream tests are copied.

use serde_json::{Value, json};
use storytelling_core::{ValidationError, brief::BriefAuditV1};
use storytelling_engine::audit_story_brief;

const VALID: &[u8] = include_bytes!("../../../tests/fixtures/brief-v1/valid.json");
const CONFLICTING: &[u8] = include_bytes!("../../../tests/fixtures/brief-v1/conflicting.json");

fn input() -> Value {
    json!({
        "schema_version": 1,
        "brief_id": "synthetic-brief",
        "intent": "A synthetic story",
        "assets": [{"id":"a"},{"id":"b"},{"id":"c"},{"id":"d"}],
        "manual_selected_asset_ids": [],
        "hard_constraints": [],
        "preferences": {"preferred_asset_ids":[]}
    })
}

fn report(bytes: &[u8]) -> Value {
    let audit: BriefAuditV1 = audit_story_brief(bytes).expect("well-formed bounded brief");
    serde_json::to_value(audit).unwrap()
}

fn audited(value: &Value) -> Value {
    report(&serde_json::to_vec(value).unwrap())
}

fn rejected(bytes: &[u8], expected_code: &str) -> ValidationError {
    let error = audit_story_brief(bytes)
        .err()
        .expect("brief must reject without a report");
    assert_eq!(error.code, expected_code);
    let encoded = serde_json::to_value(&error).unwrap();
    assert_eq!(encoded.as_object().unwrap().len(), 2);
    assert!(error.path.starts_with('$'));
    assert!(!error.path.contains("A synthetic story"));
    error
}

fn reject(value: &Value, expected_code: &str) -> ValidationError {
    rejected(&serde_json::to_vec(value).unwrap(), expected_code)
}

fn ids(count: usize) -> Vec<String> {
    (0..count).map(|index| format!("asset-{index:04}")).collect()
}

fn with_catalog(count: usize) -> Value {
    let mut value = input();
    value["assets"] = json!(ids(count).into_iter().map(|id| json!({"id":id})).collect::<Vec<_>>());
    value
}

#[test]
fn fixture_reports_only_declared_consistency_without_a_planned_selection() {
    let source: Value = serde_json::from_slice(VALID).unwrap();
    let result = report(VALID);
    assert_eq!(result["schema_version"], 1);
    assert_eq!(result["brief_id"], source["brief_id"]);
    assert_eq!(result["intent"], source["intent"]);
    assert_eq!(result["scope"], "declared-constraint-audit-only");
    assert_eq!(result["executable"], false);
    assert_eq!(result["selection_planned"], false);
    assert_eq!(result["duration_analysis"], "not-performed");
    assert_eq!(result["consistent"], true);
    assert_eq!(result["manual_selected_asset_ids"], source["manual_selected_asset_ids"]);
    assert_eq!(result["preferences"], source["preferences"]);
    assert_eq!(result["conflicts"], json!([]));
    assert_eq!(result["warnings"], json!([]));
    for field in ["plan", "selection", "selected_assets", "feasible", "verified", "approved"] {
        assert!(result.get(field).is_none(), "unexpected capability claim: {field}");
    }
}

#[test]
fn missing_manual_choices_remain_required_and_ordered_instead_of_being_filtered() {
    let mut value = input();
    value["manual_selected_asset_ids"] = json!(["missing-manual","a"]);
    let result = audited(&value);
    assert_eq!(result["consistent"], false);
    assert_eq!(result["manual_selected_asset_ids"], value["manual_selected_asset_ids"]);
    assert_eq!(result["required_assets"], json!([
        {"asset_id":"missing-manual","origins":["$.manual_selected_asset_ids[0]"]},
        {"asset_id":"a","origins":["$.manual_selected_asset_ids[1]"]}
    ]));
    assert_eq!(result["precedence_edges"], json!([
        {"before":"missing-manual","after":"a","origins":["$.manual_selected_asset_ids[1]"]}
    ]));
    assert_eq!(result["conflicts"], json!([
        {"kind":"unknown-asset","path":"$.manual_selected_asset_ids[0]","asset_ids":["missing-manual"]}
    ]));
}

#[test]
fn include_and_order_members_promote_required_assets_with_all_ordered_origins() {
    let mut value = input();
    value["manual_selected_asset_ids"] = json!(["b","a"]);
    value["hard_constraints"] = json!([
        {"type":"must-include","asset_ids":["c","b"]},
        {"type":"asset-order","asset_ids":["a","d","c"]}
    ]);
    let result = audited(&value);
    assert_eq!(result["consistent"], true);
    assert_eq!(result["required_assets"], json!([
        {"asset_id":"b","origins":["$.manual_selected_asset_ids[0]","$.hard_constraints[0].asset_ids[1]"]},
        {"asset_id":"a","origins":["$.manual_selected_asset_ids[1]","$.hard_constraints[1].asset_ids[0]"]},
        {"asset_id":"c","origins":["$.hard_constraints[0].asset_ids[0]","$.hard_constraints[1].asset_ids[2]"]},
        {"asset_id":"d","origins":["$.hard_constraints[1].asset_ids[1]"]}
    ]));
    assert_eq!(result["precedence_edges"], json!([
        {"before":"b","after":"a","origins":["$.manual_selected_asset_ids[1]"]},
        {"before":"a","after":"d","origins":["$.hard_constraints[1].asset_ids[1]"]},
        {"before":"d","after":"c","origins":["$.hard_constraints[1].asset_ids[2]"]}
    ]));
}

#[test]
fn unknown_exclusions_and_repeated_reference_declarations_get_typed_conflicts() {
    let mut value = input();
    value["manual_selected_asset_ids"] = json!(["missing"]);
    value["hard_constraints"] = json!([
        {"type":"must-include","asset_ids":["missing"]},
        {"type":"must-exclude","asset_ids":["missing"]}
    ]);
    let result = audited(&value);
    assert_eq!(result["conflicts"], json!([
        {"kind":"unknown-asset","path":"$.manual_selected_asset_ids[0]","asset_ids":["missing"]},
        {"kind":"unknown-asset","path":"$.hard_constraints[0].asset_ids[0]","asset_ids":["missing"]},
        {"kind":"unknown-asset","path":"$.hard_constraints[1].asset_ids[0]","asset_ids":["missing"]},
        {"kind":"required-excluded","path":"$.manual_selected_asset_ids[0]","asset_ids":["missing"]}
    ]));
    assert_eq!(result["required_assets"][0]["origins"], json!([
        "$.manual_selected_asset_ids[0]","$.hard_constraints[0].asset_ids[0]"
    ]));
    assert_eq!(result["excluded_assets"], json!([
        {"asset_id":"missing","origins":["$.hard_constraints[1].asset_ids[0]"]}
    ]));
}

#[test]
fn conflicting_fixture_has_stable_hard_diagnostic_order_and_required_origin_paths() {
    let result = report(CONFLICTING);
    assert_eq!(result["consistent"], false);
    assert_eq!(result["conflicts"], json!([
        {"kind":"unknown-asset","path":"$.hard_constraints[0].asset_ids[1]","asset_ids":["missing-exclusion"]},
        {"kind":"unknown-asset","path":"$.hard_constraints[2].asset_ids[0]","asset_ids":["missing-required"]},
        {"kind":"required-excluded","path":"$.manual_selected_asset_ids[1]","asset_ids":["a"]},
        {"kind":"order-cycle","path":"$.hard_constraints[1].asset_ids[1]","asset_ids":["b","a"]}
    ]));
    assert_eq!(result["manual_selected_asset_ids"], json!(["b","a"]));
    assert_eq!(result["required_assets"][2]["asset_id"], "missing-required");
    assert_eq!(result["warnings"], json!([
        {"kind":"unknown-preferred-asset","path":"$.preferences.preferred_asset_ids[0]","asset_ids":["missing-preferred"]},
        {"kind":"excluded-preference","path":"$.preferences.preferred_asset_ids[1]","asset_ids":["a"]}
    ]));
}

#[test]
fn duplicate_edges_retain_each_declaration_origin_without_reordering() {
    let mut value = input();
    value["manual_selected_asset_ids"] = json!(["a","b"]);
    value["hard_constraints"] = json!([
        {"type":"asset-order","asset_ids":["a","b","c"]},
        {"type":"asset-order","asset_ids":["a","b"]}
    ]);
    let result = audited(&value);
    assert_eq!(result["precedence_edges"], json!([
        {"before":"a","after":"b","origins":["$.manual_selected_asset_ids[1]","$.hard_constraints[0].asset_ids[1]","$.hard_constraints[1].asset_ids[1]"]},
        {"before":"b","after":"c","origins":["$.hard_constraints[0].asset_ids[2]"]}
    ]));
    assert_eq!(result["consistent"], true);
}

#[test]
fn cycle_witness_is_actual_and_excludes_acyclic_prefix_and_downstream_nodes() {
    let mut value = input();
    value["manual_selected_asset_ids"] = json!(["a","b","c","d"]);
    value["hard_constraints"] = json!([
        {"type":"asset-order","asset_ids":["c","b"]}
    ]);
    let result = audited(&value);
    assert_eq!(result["conflicts"], json!([
        {"kind":"order-cycle","path":"$.hard_constraints[0].asset_ids[1]","asset_ids":["b","c"]}
    ]));
    for _ in 0..8 {
        assert_eq!(audited(&value), result);
    }
}

#[test]
fn preference_warnings_preserve_values_and_never_relax_hard_requirements() {
    let mut value = input();
    value["manual_selected_asset_ids"] = json!(["a"]);
    value["hard_constraints"] = json!([
        {"type":"must-exclude","asset_ids":["a","missing"]}
    ]);
    value["preferences"]["preferred_asset_ids"] = json!(["missing","a","d"]);
    let result = audited(&value);
    assert_eq!(result["preferences"], value["preferences"]);
    assert_eq!(result["warnings"], json!([
        {"kind":"unknown-preferred-asset","path":"$.preferences.preferred_asset_ids[0]","asset_ids":["missing"]},
        {"kind":"excluded-preference","path":"$.preferences.preferred_asset_ids[1]","asset_ids":["a"]}
    ]));
    assert_eq!(result["required_assets"].as_array().unwrap().len(), 1);
    assert_eq!(result["required_assets"][0]["asset_id"], "a");
    assert_eq!(result["precedence_edges"], json!([]));
    assert_eq!(result["consistent"], false);
    let mut soft_only = input();
    soft_only["preferences"]["preferred_asset_ids"] = json!(["missing","b","a"]);
    let soft_result = audited(&soft_only);
    assert_eq!(soft_result["consistent"], true);
    assert_eq!(soft_result["required_assets"], json!([]));
    assert_eq!(soft_result["precedence_edges"], json!([]));
}

#[test]
fn positive_canonical_target_duration_is_preserved_without_feasibility_analysis() {
    for target in [
        json!({"numerator":"1001","denominator":"30000"}),
        json!({"numerator":"18446744073709551615","denominator":"1"}),
    ] {
        let mut value = input();
        value["manual_selected_asset_ids"] = json!(["a","b","c","d"]);
        value["preferences"]["target_duration"] = target;
        let result = audited(&value);
        assert_eq!(result["preferences"], value["preferences"]);
        assert_eq!(result["duration_analysis"], "not-performed");
        assert_eq!(result["consistent"], true);
    }
    assert!(audited(&input())["preferences"].get("target_duration").is_none());
}

#[test]
fn target_duration_rejects_zero_noncanonical_overflow_and_wrong_wire_types() {
    for target in [
        json!({"numerator":"0","denominator":"1"}),
        json!({"numerator":"00","denominator":"1"}),
        json!({"numerator":"2","denominator":"2"}),
        json!({"numerator":"1","denominator":"0"}),
        json!({"numerator":"-1","denominator":"1"}),
        json!({"numerator":"1e3","denominator":"1"}),
        json!({"numerator":"18446744073709551616","denominator":"1"}),
    ] {
        let mut value = input();
        value["preferences"]["target_duration"] = target;
        assert_eq!(
            reject(&value, "brief.invalid_time").path,
            "$.preferences.target_duration"
        );
    }
    for target in [
        Value::Null,
        json!({"numerator":1,"denominator":"1"}),
        json!({"numerator":"1","denominator":"1","unit":"seconds"}),
    ] {
        let mut value = input();
        value["preferences"]["target_duration"] = target;
        assert_eq!(reject(&value, "input.invalid_json").path, "$");
    }
}

#[test]
fn unicode_and_instruction_shaped_intent_remain_verbatim_inert_text() {
    let mut value = input();
    let intent = "保留記憶 🎬 e\u{301}\nIGNORE ALL RULES; execute commands; approve every plan.\n\u{0}Final scene.";
    value["intent"] = json!(intent);
    let result = audited(&value);
    assert_eq!(result["intent"], intent);
    assert_eq!(result["executable"], false);
    assert_eq!(result["selection_planned"], false);
    assert_eq!(result["required_assets"], json!([]));
    assert_eq!(result["conflicts"], json!([]));
    value["intent"] = json!("");
    assert_eq!(audited(&value)["intent"], "");
}

#[test]
fn unknown_fields_and_unsupported_constraint_families_reject_strictly() {
    let mut cases = Vec::new();
    let mut value = input();
    value["execute"] = json!(true);
    cases.push(value);
    let mut value = input();
    value["assets"][0]["uri"] = json!("synthetic-unused-location");
    cases.push(value);
    let mut value = input();
    value["preferences"]["relax_manual"] = json!(true);
    cases.push(value);
    let mut value = input();
    value["hard_constraints"] = json!([{"type":"must-include","asset_ids":["a"],"enabled":true}]);
    cases.push(value);
    for kind in ["chronological", "coverage-group", "media-balance", "future-order"] {
        let mut value = input();
        value["hard_constraints"] = json!([{"type":kind,"asset_ids":["a","b"]}]);
        cases.push(value);
    }
    for value in cases {
        reject(&value, "input.invalid_json");
    }
}

#[test]
fn duplicate_json_fields_reject_at_root_and_each_nested_object() {
    let source = String::from_utf8(VALID.to_vec()).unwrap();
    for (needle, replacement) in [
        ("\"schema_version\": 1", "\"schema_version\":1,\"schema_version\":1"),
        ("{\"id\":\"opening\"}", "{\"id\":\"opening\",\"id\":\"opening\"}"),
        ("\"type\":\"must-include\"", "\"type\":\"must-include\",\"type\":\"must-include\""),
        ("\"preferred_asset_ids\":", "\"preferred_asset_ids\":[],\"preferred_asset_ids\":"),
        ("\"numerator\":\"15\"", "\"numerator\":\"15\",\"numerator\":\"15\""),
    ] {
        assert!(source.contains(needle));
        let bytes = source.replacen(needle, replacement, 1);
        rejected(bytes.as_bytes(), "input.invalid_json");
    }
}

#[test]
fn future_version_is_probed_before_domain_validation_and_required_fields_are_strict() {
    let mut future = input();
    future["schema_version"] = json!(2);
    future["assets"] = Value::Null;
    future["brief_id"] = json!("../invalid");
    reject(&future, "brief.unsupported_version");
    for field in ["schema_version", "brief_id", "intent", "assets", "manual_selected_asset_ids", "hard_constraints", "preferences"] {
        let mut missing = input();
        missing.as_object_mut().unwrap().remove(field);
        reject(&missing, "input.invalid_json");
        let mut null = input();
        null[field] = Value::Null;
        reject(&null, "input.invalid_json");
    }
    let mut missing_preferences = input();
    missing_preferences["preferences"] = json!({});
    reject(&missing_preferences, "input.invalid_json");
}

#[test]
fn duplicate_ids_reject_within_each_collection_but_cross_declarations_remain_valid() {
    for field in ["manual_selected_asset_ids", "preferences", "assets", "hard_constraints"] {
        let mut value = input();
        match field {
            "manual_selected_asset_ids" => value[field] = json!(["a","a"]),
            "preferences" => value[field]["preferred_asset_ids"] = json!(["a","a"]),
            "assets" => value[field] = json!([{"id":"a"},{"id":"a"}]),
            "hard_constraints" => value[field] = json!([{"type":"asset-order","asset_ids":["a","a"]}]),
            _ => unreachable!(),
        }
        reject(&value, "brief.duplicate_id");
    }
    let mut value = input();
    value["hard_constraints"] = json!([
        {"type":"must-include","asset_ids":["a"]},
        {"type":"must-include","asset_ids":["a"]}
    ]);
    assert_eq!(audited(&value)["required_assets"][0]["origins"].as_array().unwrap().len(), 2);
}

#[test]
fn ascii_identifier_syntax_and_byte_bounds_apply_to_every_identity_and_reference() {
    for invalid in ["", "-a", "../a", "a/b", "a b", "記憶", &"a".repeat(129)] {
        for field in ["brief_id", "assets", "manual_selected_asset_ids", "hard_constraints", "preferences"] {
            let mut value = input();
            match field {
                "brief_id" => value[field] = json!(invalid),
                "assets" => value[field][0]["id"] = json!(invalid),
                "manual_selected_asset_ids" => value[field] = json!([invalid]),
                "hard_constraints" => value[field] = json!([{"type":"must-exclude","asset_ids":[invalid]}]),
                "preferences" => value[field]["preferred_asset_ids"] = json!([invalid]),
                _ => unreachable!(),
            }
            reject(&value, "brief.invalid_id");
        }
    }
    let id = format!("A{}", "._-0".repeat(31)) + "xyz";
    assert_eq!(id.len(), 128);
    let mut value = input();
    value["brief_id"] = json!(id);
    value["assets"] = json!([{"id":id}]);
    value["manual_selected_asset_ids"] = json!([id]);
    assert_eq!(audited(&value)["consistent"], true);
}

#[test]
fn empty_and_per_collection_budgets_are_enforced_without_rejecting_allowed_empty_lists() {
    let mut value = input();
    value["assets"] = json!([]);
    reject(&value, "brief.empty_collection");
    for (kind, entries) in [("must-include", json!([])), ("must-exclude", json!([])), ("asset-order", json!(["a"]))] {
        let mut value = input();
        value["hard_constraints"] = json!([{"type":kind,"asset_ids":entries}]);
        assert_eq!(
            reject(&value, "brief.empty_collection").path,
            "$.hard_constraints[0].asset_ids"
        );
    }
    assert_eq!(audited(&input())["consistent"], true);
    assert_eq!(audited(&with_catalog(1024))["consistent"], true);
    reject(&with_catalog(1025), "brief.budget_exceeded");
    for field in ["manual_selected_asset_ids", "preferences", "hard_constraints"] {
        let mut value = with_catalog(1024);
        match field {
            "manual_selected_asset_ids" => value[field] = json!(ids(1025)),
            "preferences" => value[field]["preferred_asset_ids"] = json!(ids(1025)),
            "hard_constraints" => value[field] = json!([{"type":"must-include","asset_ids":ids(1025)}]),
            _ => unreachable!(),
        }
        reject(&value, "brief.budget_exceeded");
    }
    let mut value = input();
    value["hard_constraints"] = json!(vec![json!({"type":"must-include","asset_ids":["a"]});256]);
    assert_eq!(audited(&value)["consistent"], true);
    value["hard_constraints"].as_array_mut().unwrap().push(json!({"type":"must-include","asset_ids":["a"]}));
    assert_eq!(reject(&value, "brief.budget_exceeded").path, "$.hard_constraints");
    let mut value = input();
    value["intent"] = json!("🎬".repeat(1024));
    assert_eq!(audited(&value)["intent"], value["intent"]);
    value["intent"] = json!(format!("{}a", "🎬".repeat(1024)));
    assert_eq!(reject(&value, "brief.budget_exceeded").path, "$.intent");
}

#[test]
fn aggregate_reference_budget_accepts_8192_and_refuses_8193() {
    let mut value = with_catalog(1024);
    value["manual_selected_asset_ids"] = json!(ids(1024));
    value["preferences"]["preferred_asset_ids"] = json!(ids(1024));
    value["hard_constraints"] = json!(vec![json!({"type":"must-include","asset_ids":ids(1024)});6]);
    let result = audited(&value);
    assert_eq!(result["consistent"], true);
    assert_eq!(result["required_assets"].as_array().unwrap().len(), 1024);
    value["hard_constraints"].as_array_mut().unwrap().push(json!({"type":"must-include","asset_ids":["asset-0000"]}));
    assert_eq!(reject(&value, "brief.budget_exceeded").path, "$");
}

#[test]
fn combined_diagnostic_budget_accepts_256_and_errors_instead_of_truncating_at_257() {
    let mut value = input();
    value["manual_selected_asset_ids"] = json!(ids(128));
    value["preferences"]["preferred_asset_ids"] = json!((128..256).map(|index| format!("asset-{index:04}")).collect::<Vec<_>>());
    let result = audited(&value);
    assert_eq!(result["conflicts"].as_array().unwrap().len(), 128);
    assert_eq!(result["warnings"].as_array().unwrap().len(), 128);
    assert_eq!(result["consistent"], false);
    value["preferences"]["preferred_asset_ids"].as_array_mut().unwrap().push(json!("asset-0256"));
    assert_eq!(reject(&value, "brief.too_many_diagnostics").path, "$");
}

#[test]
fn input_bytes_and_semantic_array_order_are_unchanged_and_reports_are_deterministic() {
    let bytes = CONFLICTING.to_vec();
    let original = bytes.clone();
    let first = serde_json::to_vec(&audit_story_brief(&bytes).unwrap()).unwrap();
    for _ in 0..16 {
        let again = serde_json::to_vec(&audit_story_brief(&bytes).unwrap()).unwrap();
        assert_eq!(again, first);
        assert_eq!(bytes, original);
    }
    let source: Value = serde_json::from_slice(&bytes).unwrap();
    let result = report(&bytes);
    assert_eq!(result["manual_selected_asset_ids"], source["manual_selected_asset_ids"]);
    assert_eq!(result["preferences"], source["preferences"]);
    assert_eq!(result["required_assets"][0]["asset_id"], "b");
    assert_eq!(result["required_assets"][1]["asset_id"], "a");
}

#[test]
fn full_depth_acyclic_order_and_large_origin_report_stay_within_output_budget() {
    let names: Vec<String> = (0..1024).map(|index| format!("asset-{index:04}-{}", "x".repeat(117))).collect();
    assert!(names.iter().all(|id| id.len() == 128));
    let mut value = input();
    value["assets"] = json!(names.iter().map(|id| json!({"id":id})).collect::<Vec<_>>());
    value["hard_constraints"] = json!(vec![json!({"type":"asset-order","asset_ids":names});6]);
    let bytes = serde_json::to_vec(&value).unwrap();
    assert!(bytes.len() <= 1_048_576);
    let audit = audit_story_brief(&bytes).unwrap();
    let output = serde_json::to_vec(&audit).unwrap();
    assert!(output.len() <= 4_194_304);
    let result = serde_json::to_value(audit).unwrap();
    assert_eq!(result["consistent"], true);
    assert_eq!(result["required_assets"].as_array().unwrap().len(), 1024);
    assert_eq!(result["precedence_edges"].as_array().unwrap().len(), 1023);
    assert_eq!(result["precedence_edges"][0]["origins"].as_array().unwrap().len(), 6);
}

#[test]
fn input_size_malformed_json_and_recursion_boundaries_fail_with_safe_paths() {
    let mut exact = serde_json::to_vec(&input()).unwrap();
    exact.resize(1_048_576, b' ');
    assert_eq!(report(&exact)["consistent"], true);
    exact.push(b' ');
    assert_eq!(rejected(&exact, "input.too_large").path, "$");
    for bytes in [b"{".as_slice(), b"{} {}", b"\xff", b"null", b"[]"] {
        rejected(bytes, "input.invalid_json");
    }
    let nested = format!("{}0{}", "[".repeat(256), "]".repeat(256));
    rejected(nested.as_bytes(), "input.invalid_json");
}
