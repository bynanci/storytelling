// SPDX-License-Identifier: Apache-2.0
// Independently authored finite planner regressions; see provenance-m2-02.json.
use serde_json::{Value, json};
use storytelling_engine::{plan_story, validate_plan_json};

fn time(numerator: impl ToString, denominator: impl ToString) -> Value {
    json!({"numerator": numerator.to_string(), "denominator": denominator.to_string()})
}

fn candidate(id: &str, duration: u64, start: u64, end: u64, score: i32) -> Value {
    json!({
        "asset_id": id, "content_hash": "a".repeat(64),
        "duration": time(duration, 1),
        "source_range": {"start": time(start, 1), "end": time(end, 1)},
        "score": score
    })
}

fn input(ids: &[&str], manual: &[&str], candidates: Vec<Value>, max: u64, shots: u32) -> Value {
    json!({
        "schema_version": 1,
        "brief": {
            "schema_version": 1, "brief_id": "finite-story", "intent": "Synthetic story",
            "assets": ids.iter().map(|id| json!({"id": id})).collect::<Vec<_>>(),
            "manual_selected_asset_ids": manual,
            "hard_constraints": [], "preferences": {"preferred_asset_ids": []}
        },
        "candidates": candidates, "max_duration": time(max, 1), "max_shots": shots
    })
}

fn basic() -> Value {
    input(&["a"], &["a"], vec![candidate("a", 4, 0, 4, 0)], 8, 4)
}

#[test]
fn newly_unblocked_required_asset_keeps_admission_priority_in_the_full_report() {
    let mut document = input(
        &["omega", "middle", "alpha", "zeta"],
        &[],
        ["omega", "middle", "alpha", "zeta"]
            .into_iter()
            .map(|id| candidate(id, 1, 0, 1, 0))
            .collect(),
        4,
        4,
    );
    document["brief"]["hard_constraints"] = json!([
        {"type":"must-include","asset_ids":["zeta","alpha","middle","omega"]},
        {"type":"asset-order","asset_ids":["middle","zeta"]}
    ]);
    let report = planned(&document);
    assert_eq!(selected(&report), ["alpha", "middle", "zeta", "omega"]);
    assert_eq!(
        report["plan"]["assets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["omega", "middle", "alpha", "zeta"]
    );
    for (i, row) in report["selections"].as_array().unwrap().iter().enumerate() {
        assert_eq!(row["timeline_start"], time(i, 1));
        assert_eq!(row["duration"], time(1, 1));
        assert_eq!(row["shot_id"], format!("shot-{:06}", i + 1));
        assert_eq!(row["required"], true);
    }
    validate_plan_json(&serde_json::to_vec(&report["plan"]).unwrap()).unwrap();
}

fn planned(input: &Value) -> Value {
    serde_json::to_value(plan_story(&serde_json::to_vec(input).unwrap()).unwrap()).unwrap()
}

fn rejected(input: &Value, code: &str, path: &str) -> Value {
    let error = plan_story(&serde_json::to_vec(input).unwrap()).unwrap_err();
    assert_eq!(error.code, code);
    assert_eq!(error.path, path);
    serde_json::to_value(error).unwrap()
}

fn selected(report: &Value) -> Vec<&str> {
    report["selections"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["asset_id"].as_str().unwrap())
        .collect()
}

#[test]
fn emits_an_actual_m0_plan_with_deterministic_ids_and_immutable_source_facts() {
    let mut document = basic();
    document["candidates"][0]["source_range"] = json!({"start": time(1, 1), "end": time(3, 1)});
    let bytes = serde_json::to_vec(&document).unwrap();
    let before = bytes.clone();
    let first = serde_json::to_value(plan_story(&bytes).unwrap()).unwrap();
    let second = serde_json::to_value(plan_story(&bytes).unwrap()).unwrap();
    assert_eq!(bytes, before);
    assert_eq!(first, second);
    assert_eq!(first["profile"], "storytelling.fixed-window-planner.v1");
    assert_eq!(first["scope"], "declared-fixed-window-plan-only");
    assert_eq!(first["duration_analysis"], "exact-fixed-windows");
    assert_eq!(first["selection_planned"], true);
    for field in ["executable", "source_verified", "host_verified"] {
        assert_eq!(first[field], false, "{field}");
    }
    assert_eq!(first["plan"]["plan_id"], "finite-story");
    assert_eq!(
        first["plan"]["shots"],
        json!([{
            "id": "shot-000001", "asset_id": "a",
            "source_range": {"start": time(1, 1), "end": time(3, 1)}
        }])
    );
    assert_eq!(first["plan"]["assets"][0]["content_hash"], "a".repeat(64));
    assert_eq!(first["total_duration"], time(2, 1));
    let report = validate_plan_json(&serde_json::to_vec(&first["plan"]).unwrap()).unwrap();
    assert_eq!(report.shots, 1);
    assert_eq!(report.scope, "declared-metadata-only");
}

#[test]
fn preserves_manual_and_explicit_order_with_global_reason_paths_and_catalog_order() {
    let mut b = candidate("b", 2, 0, 1, 0);
    b["content_hash"] = json!("b".repeat(64));
    let mut document = input(
        &["a", "b", "c"],
        &["b", "a"],
        vec![candidate("a", 2, 0, 1, 0), b, candidate("c", 2, 0, 1, 0)],
        4,
        4,
    );
    document["brief"]["hard_constraints"] = json!([
        {"type":"must-include","asset_ids":["c"]},
        {"type":"asset-order","asset_ids":["c","b"]}
    ]);
    let report = planned(&document);
    assert_eq!(selected(&report), ["c", "b", "a"]);
    assert_eq!(
        report["plan"]["assets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["a", "b", "c"]
    );
    assert_eq!(report["plan"]["assets"][1]["content_hash"], "b".repeat(64));
    assert_eq!(
        report["selections"][0]["reason_paths"],
        json!([
            "$.brief.hard_constraints[0].asset_ids[0]",
            "$.brief.hard_constraints[1].asset_ids[0]",
            "$.candidates[2].source_range"
        ])
    );
    assert_eq!(
        report["selections"][2]["reason_paths"],
        json!([
            "$.brief.manual_selected_asset_ids[1]",
            "$.candidates[0].source_range"
        ])
    );
    assert_eq!(report["selections"][1]["timeline_start"], time(1, 1));
    assert_eq!(
        report["brief_audit"]["required_assets"][0]["origins"][0],
        "$.manual_selected_asset_ids[0]"
    );
}

#[test]
fn rejects_hard_brief_conflicts_without_filtering_unknown_or_excluded_choices() {
    let mut document = basic();
    document["brief"]["manual_selected_asset_ids"] = json!(["missing", "a"]);
    document["brief"]["hard_constraints"] = json!([{"type":"must-exclude","asset_ids":["a"]}]);
    let error = rejected(&document, "planner.brief_conflict", "$.brief");
    assert_eq!(
        error["conflicts"],
        json!([
            {"kind":"unknown-asset","path":"$.brief.manual_selected_asset_ids[0]","asset_ids":["missing"]},
            {"kind":"required-excluded","path":"$.brief.manual_selected_asset_ids[1]","asset_ids":["a"]}
        ])
    );
    assert!(error.get("plan").is_none());
}

#[test]
fn reports_actual_order_cycles_with_outer_input_conflict_paths() {
    let mut document = input(
        &["a", "b"],
        &["a", "b"],
        vec![candidate("a", 1, 0, 1, 0), candidate("b", 1, 0, 1, 0)],
        3,
        3,
    );
    document["brief"]["hard_constraints"] = json!([{"type":"asset-order","asset_ids":["b","a"]}]);
    let error = rejected(&document, "planner.brief_conflict", "$.brief");
    assert_eq!(
        error["conflicts"],
        json!([{"kind":"order-cycle","path":"$.brief.hard_constraints[0].asset_ids[1]","asset_ids":["a","b"]}])
    );
}

#[test]
fn missing_required_candidate_points_to_first_required_origin() {
    let mut document = input(&["a", "b"], &["b"], vec![candidate("a", 1, 0, 1, 0)], 3, 3);
    let error = rejected(
        &document,
        "planner.required_candidate_missing",
        "$.brief.manual_selected_asset_ids[0]",
    );
    assert_eq!(error["conflicts"], json!([]));
    document["brief"]["manual_selected_asset_ids"] = json!([]);
    document["brief"]["hard_constraints"] = json!([{"type":"must-include","asset_ids":["b"]}]);
    rejected(
        &document,
        "planner.required_candidate_missing",
        "$.brief.hard_constraints[0].asset_ids[0]",
    );
}

#[test]
fn required_shot_count_cannot_be_repaired_by_dropping_manual_choices() {
    let document = input(
        &["a", "b"],
        &["a", "b"],
        vec![candidate("a", 1, 0, 1, 0), candidate("b", 1, 0, 1, 0)],
        3,
        1,
    );
    rejected(&document, "planner.required_shots_exceeded", "$.max_shots");
}

#[test]
fn required_fixed_windows_over_hard_duration_fail_without_shortening() {
    let mut document = input(
        &["a", "b"],
        &["a", "b"],
        vec![candidate("a", 1, 0, 1, 0), candidate("b", 1, 0, 1, 0)],
        1,
        3,
    );
    document["brief"]["preferences"]["target_duration"] = time(1, 1);
    rejected(
        &document,
        "planner.required_duration_exceeded",
        "$.max_duration",
    );
}

#[test]
fn required_choices_can_exceed_soft_target_and_keep_all_windows() {
    let mut document = input(
        &["a", "b", "c"],
        &["a", "b"],
        vec![
            candidate("a", 1, 0, 1, 0),
            candidate("b", 1, 0, 1, 0),
            candidate("c", 1, 0, 1, 100),
        ],
        3,
        3,
    );
    document["brief"]["preferences"]["target_duration"] = time(1, 1);
    let report = planned(&document);
    assert_eq!(selected(&report), ["a", "b"]);
    assert_eq!(report["total_duration"], time(2, 1));
    assert_eq!(report["target_status"], "above");
    assert_eq!(report["skipped_candidates"][0]["reason"], "not-needed");
}

#[test]
fn optional_ranking_uses_preference_then_signed_score_then_ascii_id() {
    let mut document = input(
        &["a", "b", "c", "d"],
        &[],
        vec![
            candidate("b", 1, 0, 1, 1),
            candidate("d", 1, 0, 1, i32::MIN),
            candidate("c", 1, 0, 1, i32::MIN),
            candidate("a", 1, 0, 1, 1),
        ],
        4,
        3,
    );
    document["brief"]["preferences"]["preferred_asset_ids"] = json!(["c"]);
    let report = planned(&document);
    assert_eq!(selected(&report), ["c", "a", "b"]);
    assert_eq!(report["selections"][0]["declared_score"], i32::MIN);
    assert_eq!(report["selections"][0]["required"], false);
    assert_eq!(
        report["selections"][0]["reason_paths"],
        json!([
            "$.brief.preferences.preferred_asset_ids[0]",
            "$.candidates[2].score",
            "$.candidates[2].source_range"
        ])
    );
    assert_eq!(
        report["skipped_candidates"],
        json!([{"asset_id":"d","path":"$.candidates[1]","reason":"shot-limit"}])
    );
}

#[test]
fn too_long_optional_is_skipped_while_a_later_fitting_asset_is_selected() {
    let document = input(
        &["a", "r", "b"],
        &["r"],
        vec![
            candidate("a", 10, 0, 10, 100),
            candidate("r", 1, 0, 1, 0),
            candidate("b", 2, 0, 2, 0),
        ],
        3,
        3,
    );
    let report = planned(&document);
    assert_eq!(selected(&report), ["r", "b"]);
    assert_eq!(report["total_duration"], time(3, 1));
    assert_eq!(report["target_status"], "not-requested");
    assert_eq!(
        report["skipped_candidates"],
        json!([{"asset_id":"a","path":"$.candidates[0]","reason":"duration-limit"}])
    );
}

#[test]
fn skipped_reasons_prioritize_exclusion_then_met_target_over_other_limits() {
    let mut document = input(
        &["a", "b", "c"],
        &["a"],
        vec![
            candidate("c", 2, 0, 2, 100),
            candidate("a", 1, 0, 1, 0),
            candidate("b", 2, 0, 2, 100),
        ],
        1,
        1,
    );
    document["brief"]["preferences"]["target_duration"] = time(1, 1);
    document["brief"]["hard_constraints"] = json!([{"type":"must-exclude","asset_ids":["c"]}]);
    let report = planned(&document);
    assert_eq!(report["target_status"], "met");
    assert_eq!(
        report["skipped_candidates"],
        json!([
            {"asset_id":"c","path":"$.candidates[0]","reason":"excluded"},
            {"asset_id":"b","path":"$.candidates[2]","reason":"not-needed"}
        ])
    );
}

#[test]
fn shot_limit_reason_precedes_duration_limit_when_no_target_is_requested() {
    let document = input(
        &["a", "b"],
        &["a"],
        vec![candidate("a", 1, 0, 1, 0), candidate("b", 2, 0, 2, 100)],
        1,
        1,
    );
    let report = planned(&document);
    assert_eq!(report["skipped_candidates"][0]["reason"], "shot-limit");
    assert_eq!(report["target_status"], "not-requested");
}

#[test]
fn soft_target_shortfall_is_success_even_when_target_exceeds_hard_maximum() {
    let mut a = candidate("a", 1, 0, 1, 0);
    a["duration"] = time(1, 2);
    a["source_range"]["end"] = time(1, 2);
    let mut document = input(&["a"], &[], vec![a], 1, 1);
    document["brief"]["preferences"]["target_duration"] = time(4, 1);
    let report = planned(&document);
    assert_eq!(report["total_duration"], time(1, 2));
    assert_eq!(report["target_status"], "below");
    assert_eq!(selected(&report), ["a"]);
}

#[test]
fn fractional_windows_preserve_source_offsets_and_exact_accumulated_placements() {
    let mut a = candidate("a", 1, 0, 1, 0);
    a["source_range"] = json!({"start":time(1,10),"end":time(1,5)});
    let mut b = candidate("b", 1, 0, 1, 0);
    b["source_range"] = json!({"start":time(3,10),"end":time(1,2)});
    let document = input(&["a", "b"], &["a", "b"], vec![a, b], 1, 2);
    let report = planned(&document);
    assert_eq!(report["content_clock_rate"], "10");
    assert_eq!(report["total_duration"], time(3, 10));
    assert_eq!(report["selections"][0]["timeline_start"], time(0, 1));
    assert_eq!(report["selections"][0]["duration"], time(1, 10));
    assert_eq!(report["selections"][1]["timeline_start"], time(1, 10));
    assert_eq!(report["selections"][1]["duration"], time(1, 5));
    assert_eq!(
        report["plan"]["shots"][1]["source_range"],
        json!({"start":time(3,10),"end":time(1,2)})
    );
}

#[test]
fn exact_u64_maximum_total_is_admitted_without_float_conversion() {
    let document = input(
        &["a", "b"],
        &["a", "b"],
        vec![
            candidate("a", u64::MAX - 1, 0, u64::MAX - 1, 0),
            candidate("b", 1, 0, 1, 0),
        ],
        u64::MAX,
        2,
    );
    let report = planned(&document);
    assert_eq!(report["total_duration"], time(u64::MAX, 1));
    assert_eq!(
        report["selections"][1]["timeline_start"],
        time(u64::MAX - 1, 1)
    );
}

#[test]
fn required_overflowing_total_rejects_and_oversized_optional_skips_without_addition() {
    let required = input(
        &["a", "b"],
        &["a", "b"],
        vec![
            candidate("a", u64::MAX, 0, u64::MAX, 0),
            candidate("b", 1, 0, 1, 0),
        ],
        u64::MAX,
        2,
    );
    rejected(
        &required,
        "planner.required_duration_exceeded",
        "$.max_duration",
    );
    let optional = input(
        &["r", "large"],
        &["r"],
        vec![
            candidate("r", 1, 0, 1, 0),
            candidate("large", u64::MAX, 0, u64::MAX, 100),
        ],
        u64::MAX,
        2,
    );
    let report = planned(&optional);
    assert_eq!(selected(&report), ["r"]);
    assert_eq!(report["total_duration"], time(1, 1));
    assert_eq!(report["skipped_candidates"][0]["reason"], "duration-limit");
}

#[test]
fn coprime_denominator_lcm_over_u64_rejects_before_rounded_planning() {
    let mut a = candidate("a", 1, 0, 1, 0);
    a["source_range"]["end"] = time(1, 4_294_967_291_u64);
    let mut b = candidate("b", 1, 0, 1, 0);
    b["source_range"]["end"] = time(1, 4_294_967_311_u64);
    let document = input(&["a", "b"], &["a", "b"], vec![a, b], 1, 2);
    rejected(&document, "planner.unrepresentable_time", "$");
}

#[test]
fn scaled_declared_duration_overflow_has_its_exact_input_path() {
    let mut a = candidate("a", u64::MAX, 0, 1, 0);
    a["source_range"]["end"] = time(1, 2);
    let document = input(&["a"], &["a"], vec![a], 1, 1);
    rejected(
        &document,
        "planner.unrepresentable_time",
        "$.candidates[0].duration",
    );
}

#[test]
fn future_schema_is_probed_and_nested_unknown_missing_null_fields_reject() {
    let mut future = basic();
    future["schema_version"] = json!(2);
    future["future_field"] = json!(true);
    rejected(&future, "planner.unsupported_version", "$.schema_version");
    let mut future_brief = basic();
    future_brief["brief"]["schema_version"] = json!(2);
    rejected(
        &future_brief,
        "brief.unsupported_version",
        "$.brief.schema_version",
    );
    let mut cases = Vec::new();
    let mut root_unknown = basic();
    root_unknown["unknown"] = json!(1);
    cases.push(root_unknown);
    let mut candidate_unknown = basic();
    candidate_unknown["candidates"][0]["uri"] = json!("file:///never-open");
    cases.push(candidate_unknown);
    let mut nested_unknown = basic();
    nested_unknown["candidates"][0]["source_range"]["start"]["clock"] = json!(1);
    cases.push(nested_unknown);
    let mut missing = basic();
    missing.as_object_mut().unwrap().remove("max_shots");
    cases.push(missing);
    let mut null = basic();
    null["max_duration"] = Value::Null;
    cases.push(null);
    let mut null_target = basic();
    null_target["brief"]["preferences"]["target_duration"] = Value::Null;
    cases.push(null_target);
    for document in cases {
        let error = plan_story(&serde_json::to_vec(&document).unwrap()).unwrap_err();
        assert_eq!(error.code, "input.invalid_json");
    }
}

#[test]
fn duplicate_json_fields_are_rejected_at_root_candidate_and_time_boundaries() {
    let compact = serde_json::to_string(&basic()).unwrap();
    let root_duplicate = format!(
        "{},\"schema_version\":1}}",
        compact.strip_suffix('}').unwrap()
    );
    let candidate_duplicate = compact.replacen("\"score\":0", "\"score\":0,\"score\":1", 1);
    let time_duplicate = compact.replacen(
        "\"denominator\":\"1\"",
        "\"denominator\":\"1\",\"denominator\":\"2\"",
        1,
    );
    for bytes in [root_duplicate, candidate_duplicate, time_duplicate] {
        let error = plan_story(bytes.as_bytes()).unwrap_err();
        assert_eq!(error.code, "input.invalid_json");
    }
}

#[test]
fn invalid_hash_times_ranges_and_hard_bounds_keep_precise_domain_errors() {
    let mutations = [
        (
            "/candidates/0/content_hash",
            json!("bad"),
            "plan.invalid_hash",
            "$.candidates[0].content_hash",
        ),
        (
            "/candidates/0/duration",
            time(0, 1),
            "plan.invalid_range",
            "$.candidates[0].duration",
        ),
        (
            "/candidates/0/source_range/start",
            time(4, 1),
            "plan.invalid_range",
            "$.candidates[0].source_range",
        ),
        (
            "/candidates/0/source_range/end",
            time(5, 1),
            "plan.source_out_of_bounds",
            "$.candidates[0].source_range.end",
        ),
        (
            "/candidates/0/duration",
            time("04", 1),
            "plan.invalid_time",
            "$.candidates[0].duration",
        ),
        (
            "/max_duration",
            time(0, 1),
            "plan.invalid_range",
            "$.max_duration",
        ),
        (
            "/max_duration",
            time(2, 2),
            "plan.invalid_time",
            "$.max_duration",
        ),
        (
            "/max_shots",
            json!(0),
            "planner.invalid_max_shots",
            "$.max_shots",
        ),
        (
            "/max_shots",
            json!(1025),
            "planner.invalid_max_shots",
            "$.max_shots",
        ),
    ];
    for (pointer, value, code, path) in mutations {
        let mut document = basic();
        *document.pointer_mut(pointer).unwrap() = value;
        rejected(&document, code, path);
    }
    for bad_score in [json!(1.5), json!(2_147_483_648_u64), Value::Null] {
        let mut document = basic();
        document["candidates"][0]["score"] = bad_score;
        rejected(&document, "input.invalid_json", "$");
    }
}

#[test]
fn candidate_duplicates_unknown_ids_and_optional_catalog_omissions_are_distinct() {
    let mut duplicate = basic();
    duplicate["candidates"]
        .as_array_mut()
        .unwrap()
        .push(candidate("a", 4, 0, 4, 0));
    rejected(
        &duplicate,
        "planner.duplicate_candidate",
        "$.candidates[1].asset_id",
    );
    let mut unknown = basic();
    unknown["candidates"][0]["asset_id"] = json!("other");
    rejected(
        &unknown,
        "planner.unknown_candidate",
        "$.candidates[0].asset_id",
    );
    let optional_omitted = input(
        &["a", "optional"],
        &["a"],
        vec![candidate("a", 1, 0, 1, 0)],
        2,
        2,
    );
    assert_eq!(selected(&planned(&optional_omitted)), ["a"]);
}

#[test]
fn exact_input_budget_accepts_valid_json_and_rejects_the_next_byte() {
    let mut bytes = serde_json::to_vec(&basic()).unwrap();
    bytes.resize(1_048_576, b' ');
    assert!(plan_story(&bytes).is_ok());
    bytes.push(b' ');
    let error = plan_story(&bytes).unwrap_err();
    assert_eq!(
        (error.code.as_str(), error.path.as_str()),
        ("input.too_large", "$")
    );
    let error = plan_story(b"{invalid}").unwrap_err();
    assert_eq!(error.code, "input.invalid_json");
}

#[test]
fn full_candidate_budget_builds_a_bounded_valid_plan_and_next_candidate_rejects() {
    let ids: Vec<String> = (0..1024).map(|i| format!("asset-{i:04}")).collect();
    let id_refs: Vec<&str> = ids.iter().map(String::as_str).collect();
    let candidates = ids.iter().map(|id| candidate(id, 1, 0, 1, 0)).collect();
    let mut document = input(&id_refs, &[], candidates, 1024, 1024);
    let report = planned(&document);
    assert_eq!(report["plan"]["shots"].as_array().unwrap().len(), 1024);
    assert_eq!(report["total_duration"], time(1024, 1));
    assert!(serde_json::to_vec(&report).unwrap().len() <= 4_194_304);
    assert!(serde_json::to_vec(&report["plan"]).unwrap().len() <= 1_048_576);
    assert!(validate_plan_json(&serde_json::to_vec(&report["plan"]).unwrap()).is_ok());
    document["candidates"]
        .as_array_mut()
        .unwrap()
        .push(candidate("asset-0000", 1, 0, 1, 0));
    rejected(&document, "planner.too_many_candidates", "$.candidates");
}

#[test]
fn excluded_and_target_unused_candidates_still_require_valid_declarations() {
    let mut document = input(
        &["a", "b"],
        &["a"],
        vec![candidate("a", 1, 0, 1, 0), candidate("b", 1, 0, 1, 0)],
        1,
        1,
    );
    document["brief"]["preferences"]["target_duration"] = time(1, 1);
    document["candidates"][1]["content_hash"] = json!("invalid");
    rejected(
        &document,
        "plan.invalid_hash",
        "$.candidates[1].content_hash",
    );
    document["brief"]["hard_constraints"] = json!([{"type":"must-exclude","asset_ids":["b"]}]);
    rejected(
        &document,
        "plan.invalid_hash",
        "$.candidates[1].content_hash",
    );
    document["candidates"][1] = candidate("b", u64::MAX, 0, 1, 0);
    document["candidates"][1]["source_range"]["end"] = time(1, 2);
    rejected(
        &document,
        "planner.unrepresentable_time",
        "$.candidates[1].duration",
    );
}

#[test]
fn empty_and_all_excluded_candidate_sets_refuse_to_emit_an_empty_plan() {
    let empty = input(&["a"], &[], vec![], 1, 1);
    rejected(&empty, "planner.no_selection", "$.candidates");
    let mut excluded = input(&["a"], &[], vec![candidate("a", 1, 0, 1, 0)], 1, 1);
    excluded["brief"]["hard_constraints"] = json!([{"type":"must-exclude","asset_ids":["a"]}]);
    rejected(&excluded, "planner.no_selection", "$.candidates");
}
