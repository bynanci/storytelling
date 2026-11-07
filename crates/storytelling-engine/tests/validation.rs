use serde_json::{Value, json};
use storytelling_engine::validate_plan_json;

fn valid() -> Value {
    serde_json::from_str(include_str!("../../../examples/minimal-plan.json")).unwrap()
}

fn code(value: &Value) -> String {
    validate_plan_json(&serde_json::to_vec(value).unwrap())
        .unwrap_err()
        .code
}

#[test]
fn accepts_declared_plan_and_reused_source() {
    let report = validate_plan_json(include_bytes!("../../../examples/minimal-plan.json")).unwrap();
    assert_eq!(report.plan_id, "travel-recap");
    assert_eq!((report.assets, report.shots), (1, 2));
    assert_eq!(report.scope, "declared-metadata-only");
}

#[test]
fn rejects_source_range_outside_declared_duration() {
    let error = validate_plan_json(include_bytes!(
        "../../../tests/fixtures/invalid-source-range.json"
    ))
    .unwrap_err();
    assert_eq!(error.code, "plan.source_out_of_bounds");
}

#[test]
fn rejects_unsupported_version() {
    let mut value = valid();
    value["schema_version"] = json!(2);
    assert_eq!(code(&value), "plan.unsupported_version");
}

#[test]
fn rejects_unknown_and_duplicate_json_fields() {
    let mut value = valid();
    value["execute"] = json!(true);
    assert_eq!(code(&value), "input.invalid_json");
    let bytes = serde_json::to_string(&valid())
        .unwrap()
        .replacen("{", "{\"schema_version\":1,", 1);
    assert_eq!(
        validate_plan_json(bytes.as_bytes()).unwrap_err().code,
        "input.invalid_json"
    );
    let mut value = valid();
    value["assets"][0]["uri"] = json!("https://example.invalid/private");
    assert_eq!(code(&value), "input.invalid_json");
}

#[test]
fn rejects_duplicate_identities() {
    let mut value = valid();
    let asset = value["assets"][0].clone();
    value["assets"].as_array_mut().unwrap().push(asset);
    assert_eq!(code(&value), "plan.duplicate_id");
    let mut value = valid();
    value["shots"][1]["id"] = value["shots"][0]["id"].clone();
    assert_eq!(code(&value), "plan.duplicate_id");
}

#[test]
fn rejects_missing_source_reference() {
    let mut value = valid();
    value["shots"][0]["asset_id"] = json!("unknown-source");
    assert_eq!(code(&value), "plan.unknown_asset");
}

#[test]
fn rejects_invalid_identity_and_hash() {
    let mut value = valid();
    value["plan_id"] = json!("../private");
    assert_eq!(code(&value), "plan.invalid_id");
    let mut value = valid();
    value["assets"][0]["content_hash"] = json!("not-a-hash");
    assert_eq!(code(&value), "plan.invalid_hash");
}

#[test]
fn rejects_noncanonical_and_zero_denominator_time() {
    for time in [
        json!({"numerator":"00","denominator":"1"}),
        json!({"numerator":"1","denominator":"0"}),
        json!({"numerator":"2","denominator":"2"}),
        json!({"numerator":"-1","denominator":"1"}),
        json!({"numerator":0,"denominator":"1"}),
        json!({"numerator":"18446744073709551616","denominator":"1"}),
    ] {
        let mut value = valid();
        value["shots"][0]["source_range"]["start"] = time;
        let result = code(&value);
        assert!(result == "plan.invalid_time" || result == "input.invalid_json");
    }
}

#[test]
fn rejects_empty_or_reversed_ranges() {
    for end in ["0", "1"] {
        let mut value = valid();
        value["shots"][0]["source_range"]["start"]["numerator"] = json!("1");
        value["shots"][0]["source_range"]["end"]["numerator"] = json!(end);
        assert_eq!(code(&value), "plan.invalid_range");
    }
}

#[test]
fn compares_large_rationals_without_float_rounding() {
    let mut value = valid();
    value["assets"][0]["duration"]["numerator"] = json!("18446744073709551615");
    value["shots"][0]["source_range"]["start"] =
        json!({"numerator":"18446744073709551613","denominator":"1"});
    value["shots"][0]["source_range"]["end"] =
        json!({"numerator":"18446744073709551614","denominator":"1"});
    assert!(validate_plan_json(&serde_json::to_vec(&value).unwrap()).is_ok());
}

#[test]
fn rejects_empty_and_over_budget_collections() {
    let mut value = valid();
    value["shots"] = json!([]);
    assert_eq!(code(&value), "plan.empty_collection");
    let mut value = valid();
    value["assets"] = json!(vec![value["assets"][0].clone(); 1025]);
    assert_eq!(code(&value), "plan.too_many_items");
    let mut value = valid();
    value["shots"] = json!(vec![value["shots"][0].clone(); 4097]);
    assert_eq!(code(&value), "plan.too_many_items");
}

#[test]
fn rejects_malformed_or_oversized_input() {
    assert_eq!(
        validate_plan_json(b"{").unwrap_err().code,
        "input.invalid_json"
    );
    assert_eq!(
        validate_plan_json(&vec![b' '; 1_048_577]).unwrap_err().code,
        "input.too_large"
    );
}

#[test]
fn accepts_exact_rational_intervals_and_is_deterministic() {
    let mut value = valid();
    value["shots"][0]["source_range"]["end"] =
        json!({"numerator":"1001","denominator":"30000"});
    let bytes = serde_json::to_vec(&value).unwrap();
    let first = serde_json::to_vec(&validate_plan_json(&bytes).unwrap()).unwrap();
    let second = serde_json::to_vec(&validate_plan_json(&bytes).unwrap()).unwrap();
    assert_eq!(first, second);
}

#[test]
fn rejects_zero_duration_and_preserves_input_bytes() {
    let mut value = valid();
    value["assets"][0]["duration"]["numerator"] = json!("0");
    assert_eq!(code(&value), "plan.invalid_range");
    let input = serde_json::to_vec(&valid()).unwrap();
    let original = input.clone();
    validate_plan_json(&input).unwrap();
    assert_eq!(input, original);
}
