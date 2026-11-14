// SPDX-License-Identifier: MIT
//! Pure, opt-in draft range normalization. V1 remains the final authority.
mod exact_time;

use crate::narrative::preflight_json_with_max_nesting;
use crate::render_narrative_video_coverage;
use serde_json::{Value, json};

const VERSION: &str = "n10-06a.1-draft";

fn failure(code: &str, path: Option<&str>) -> Value {
    json!({"error":{"stage":"helper","code":code,"path":path}})
}
fn shape(path: &str) -> Value {
    failure("draft_time.invalid_shape", Some(path))
}
fn closed(value: &Value, keys: &[&str]) -> bool {
    value.as_object().is_some_and(|object| {
        object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key))
    })
}
fn string_at<'a>(value: &'a Value, field: &str, path: &str) -> Result<&'a str, Value> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| shape(&format!("{path}.{field}")))
}
fn rational(pair: (String, String)) -> Value {
    json!({"numerator":pair.0,"denominator":pair.1})
}
fn normalize_range(value: &Value, path: &str) -> Result<Value, Value> {
    let unit = string_at(value, "unit", path)?;
    let (start, end) = match unit {
        "seconds" => {
            if !closed(value, &["unit", "start", "end"]) {
                return Err(shape(path));
            }
            let start = string_at(value, "start", path)?;
            let start = exact_time::decimal(start)
                .map_err(|code| failure(code, Some(&format!("{path}.start"))))?;
            let end = string_at(value, "end", path)?;
            let end = exact_time::decimal(end)
                .map_err(|code| failure(code, Some(&format!("{path}.end"))))?;
            (start, end)
        }
        "frames" => {
            if value.get("timing").and_then(Value::as_str) != Some("constant") {
                return Err(failure(
                    "draft_time.frame_timing_not_supported",
                    Some(&format!("{path}.timing")),
                ));
            }
            if !closed(
                value,
                &["unit", "start_frame", "end_frame", "fps", "timing"],
            ) {
                return Err(shape(path));
            }
            let fps = &value["fps"];
            let fps_path = format!("{path}.fps");
            if !closed(fps, &["numerator", "denominator"]) {
                return Err(failure("draft_time.invalid_fps", Some(&fps_path)));
            }
            let n = fps["numerator"]
                .as_str()
                .ok_or_else(|| failure("draft_time.invalid_fps", Some(&fps_path)))?;
            let d = fps["denominator"]
                .as_str()
                .ok_or_else(|| failure("draft_time.invalid_fps", Some(&fps_path)))?;
            let rate = exact_time::fps(n, d).map_err(|code| failure(code, Some(&fps_path)))?;
            let start = string_at(value, "start_frame", path)?;
            let start = exact_time::frame(start, rate)
                .map_err(|code| failure(code, Some(&format!("{path}.start_frame"))))?;
            let end = string_at(value, "end_frame", path)?;
            let end = exact_time::frame(end, rate)
                .map_err(|code| failure(code, Some(&format!("{path}.end_frame"))))?;
            (start, end)
        }
        _ => return Err(shape(&format!("{path}.unit"))),
    };
    Ok(json!({"start":rational(start),"end":rational(end)}))
}

/// Normalizes the accepted draft G2 input, then invokes unchanged V1 validation.
/// This experimental feature reads supplied bytes only; its JSON response is not
/// the stable consumer SDK facade planned in #107.
pub fn normalize_draft_video_coverage(bytes: &[u8]) -> Result<Value, Value> {
    if bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(failure("draft_time.input_too_large", None));
    }
    preflight_json_with_max_nesting(bytes, 34).map_err(|error| match error.code.as_str() {
        "narrative.duplicate_field" => failure("draft_time.duplicate_field", Some("$")),
        "narrative.depth_exceeded" => failure("draft_time.depth_exceeded", Some("$")),
        _ => failure("draft_time.invalid_json", None),
    })?;
    let input: Value =
        serde_json::from_slice(bytes).map_err(|_| failure("draft_time.invalid_json", None))?;
    let version = input
        .get("draft_version")
        .and_then(Value::as_str)
        .ok_or_else(|| shape("$.draft_version"))?;
    if version != VERSION {
        return Err(failure(
            "draft_time.unsupported_version",
            Some("$.draft_version"),
        ));
    }
    if !closed(&input, &["draft_version", "coverage"]) || !input["coverage"].is_object() {
        return Err(shape("$"));
    }
    let mut coverage = input["coverage"].clone();
    let mut conversions = Vec::new();
    if let Some(mappings) = coverage.get_mut("mappings").and_then(Value::as_array_mut) {
        for (index, row) in mappings.iter_mut().enumerate() {
            if row.get("status").and_then(Value::as_str) != Some("mapped") {
                continue;
            }
            let Some(original) = row.get("source_range").cloned() else {
                continue;
            };
            let input_path = format!("$.coverage.mappings[{index}].source_range");
            let output_path = format!("$.mappings[{index}].source_range");
            let source_range = normalize_range(&original, &input_path)?;
            row["source_range"] = source_range.clone();
            conversions.push(json!({"input_path":input_path,"output_path":output_path,"original":original,"source_range":source_range}));
        }
    }
    let normalized = serde_json::to_vec(&coverage).map_err(|_| shape("$.coverage"))?;
    let validation = render_narrative_video_coverage(&normalized)
        .map_err(|error| json!({"error":{"stage":"v1","code":error.code,"path":error.path}}))?;
    let status = if validation.ready_for_planning {
        "declaration_ready_for_planning"
    } else {
        "declaration_unresolved"
    };
    Ok(
        json!({"draft_version":VERSION,"scope":"declarations_only","status":status,"conversions":conversions,"coverage":coverage,"validation":validation}),
    )
}
