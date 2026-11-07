// SPDX-License-Identifier: Apache-2.0
// Source-inspired contract tests; independently authored synthetic declarations.
// Origin: bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76,
// packages/exporters/src/index.ts and packages/core/src/{models,timebase}.ts.
// No upstream implementation, upstream fixture or private media is copied.

use serde::Deserialize;
use serde_json::{Value, json};
use storytelling_core::compatibility::{ImportError, ImportProfileV1};
use storytelling_engine::{import_openfilm_snapshot, validate_plan_json};

const STILL: &[u8] = include_bytes!("../../../tests/fixtures/openfilm/finite-v1/still.json");
const VIDEO: &[u8] = include_bytes!("../../../tests/fixtures/openfilm/finite-v1/video-trim.json");
const REPEATED: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/repeat-source.json");
const FRACTIONAL: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/fractional-rate.json");

fn value(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap()
}

fn accepted(bytes: &[u8]) -> Value {
    let result =
        import_openfilm_snapshot(bytes, &ImportProfileV1).expect("accepted finite-v1 input");
    serde_json::to_value(result).unwrap()
}

fn accepted_value(snapshot: &Value) -> Value {
    accepted(&serde_json::to_vec(snapshot).unwrap())
}

fn rejected(bytes: &[u8]) -> ImportError {
    import_openfilm_snapshot(bytes, &ImportProfileV1)
        .expect_err("snapshot must reject without an accepted plan")
}

fn reject_value(snapshot: &Value, expected: &str) -> ImportError {
    let error = rejected(&serde_json::to_vec(snapshot).unwrap());
    assert_eq!(error.code, expected);
    error
}

fn rational(numerator: &str, denominator: &str) -> Value {
    json!({"numerator": numerator, "denominator": denominator})
}

fn unsupported(snapshot: &Value) -> Value {
    let error = reject_value(snapshot, "import.unsupported_feature");
    let encoded = serde_json::to_value(error).unwrap();
    assert!(encoded.get("plan").is_none());
    assert_eq!(encoded["losses"].as_array().unwrap().len(), 1);
    assert_eq!(encoded["losses"][0]["code"], "import.unsupported_feature");
    assert_eq!(encoded["losses"][0]["disposition"], "unsupported");
    assert_eq!(encoded["losses"][0]["path"], encoded["path"]);
    encoded
}

fn clips(snapshot: &mut Value) -> &mut Vec<Value> {
    snapshot["composition"]["tracks"][0]["clips"]
        .as_array_mut()
        .unwrap()
}

#[test]
fn still_hold_maps_declared_ids_hash_and_passes_m0_validation() {
    let result = accepted(STILL);
    assert_eq!(result["profile"], "openfilm-finite-v1");
    assert_eq!(result["scope"], "declared-visual-mapping-only");
    assert_eq!(result["executable"], false);
    assert_eq!(
        result["plan"],
        json!({
            "schema_version": 1,
            "plan_id": "finite-still",
            "assets": [{
                "id": "synthetic-photo",
                "content_hash": "a".repeat(64),
                "duration": rational("5", "1")
            }],
            "shots": [{
                "id": "still-occurrence",
                "asset_id": "synthetic-photo",
                "source_range": {"start": rational("0", "1"), "end": rational("5", "1")}
            }]
        })
    );
    let plan = serde_json::to_vec(&result["plan"]).unwrap();
    let report = validate_plan_json(&plan).unwrap();
    assert_eq!(report.scope, "declared-metadata-only");
    assert_eq!(
        result["losses"],
        json!([
            {"code":"media.not_verified", "path":"$.assets", "disposition":"not-verified"},
            {"code":"image.hold_window", "path":"$.assets[0]", "disposition":"intentional-delta"}
        ])
    );
    assert!(result.get("verified").is_none());
    assert!(result.get("digest").is_none());
}

#[test]
fn locks_selection_story_and_beats_remain_original_declarations() {
    let result = accepted(STILL);
    assert_eq!(result["source_snapshot"], value(STILL));
    let source = &result["source_snapshot"];
    assert_eq!(source["composition"]["storyId"], "synthetic-story");
    assert_eq!(source["composition"]["tracks"][0]["locked"], true);
    assert_eq!(source["composition"]["tracks"][0]["target"], true);
    assert_eq!(
        source["composition"]["tracks"][0]["clips"][0]["locked"],
        true
    );
    assert_eq!(
        source["composition"]["tracks"][0]["clips"][0]["beatId"],
        "opening-beat"
    );
    assert_eq!(source["assets"][0]["state"]["favorite"], true);
    assert_eq!(source["assets"][0]["state"]["locked"], true);
    assert_eq!(source["assets"][0]["duration"], 0.5);
}

#[test]
fn repeat_source_preserves_occurrence_order_and_uses_maximum_still_hold() {
    let result = accepted(REPEATED);
    let plan = &result["plan"];
    assert_eq!(plan["assets"].as_array().unwrap().len(), 1);
    assert_eq!(plan["shots"].as_array().unwrap().len(), 2);
    assert_eq!(plan["shots"][0]["id"], "occurrence-z");
    assert_eq!(plan["shots"][1]["id"], "occurrence-a");
    assert_eq!(plan["shots"][0]["asset_id"], "reused-photo");
    assert_eq!(plan["shots"][1]["asset_id"], "reused-photo");
    assert_eq!(plan["assets"][0]["duration"], rational("3", "1"));
    assert_eq!(plan["shots"][0]["source_range"]["end"], rational("2", "1"));
    assert_eq!(plan["shots"][1]["source_range"]["end"], rational("3", "1"));
    assert_eq!(result["losses"].as_array().unwrap().len(), 2);
    assert!(validate_plan_json(&serde_json::to_vec(plan).unwrap()).is_ok());
}

#[test]
fn video_trim_uses_exact_source_seconds_and_reports_unmapped_audio() {
    let result = accepted(VIDEO);
    let plan = &result["plan"];
    assert_eq!(plan["assets"][0]["duration"], rational("10", "1"));
    assert_eq!(
        plan["shots"][0]["source_range"]["start"],
        rational("1", "10")
    );
    assert_eq!(plan["shots"][0]["source_range"]["end"], rational("13", "5"));
    assert_eq!(
        result["losses"],
        json!([
            {"code":"media.not_verified", "path":"$.assets", "disposition":"not-verified"},
            {"code":"audio.not_mapped", "path":"$.assets[0]", "disposition":"blocks-host-handoff"}
        ])
    );
    assert_eq!(result["executable"], false);
    assert!(validate_plan_json(&serde_json::to_vec(plan).unwrap()).is_ok());
}

#[test]
fn asset_array_order_and_one_loss_per_source_survive_mixed_media_reuse() {
    let mut snapshot = value(REPEATED);
    let mut video = value(VIDEO)["assets"][0].clone();
    video["id"] = json!("video-source");
    snapshot["assets"].as_array_mut().unwrap().insert(0, video);
    snapshot["composition"]["duration"] = json!(6);
    clips(&mut snapshot).push(json!({
        "id":"video-last", "assetId":"video-source", "timelineStart":5,
        "timelineDuration":1, "sourceIn":1, "sourceOut":2
    }));
    let result = accepted_value(&snapshot);
    assert_eq!(result["plan"]["assets"][0]["id"], "video-source");
    assert_eq!(result["plan"]["assets"][1]["id"], "reused-photo");
    assert_eq!(result["plan"]["shots"][0]["id"], "occurrence-z");
    assert_eq!(result["plan"]["shots"][2]["id"], "video-last");
    assert_eq!(
        result["losses"],
        json!([
            {"code":"media.not_verified", "path":"$.assets", "disposition":"not-verified"},
            {"code":"audio.not_mapped", "path":"$.assets[0]", "disposition":"blocks-host-handoff"},
            {"code":"image.hold_window", "path":"$.assets[1]", "disposition":"intentional-delta"}
        ])
    );
}

#[test]
fn omitted_video_source_out_uses_source_in_plus_timeline_duration() {
    let mut snapshot = value(VIDEO);
    clips(&mut snapshot)[0]
        .as_object_mut()
        .unwrap()
        .remove("sourceOut");
    let result = accepted_value(&snapshot);
    assert_eq!(
        result["plan"]["shots"][0]["source_range"]["end"],
        rational("13", "5")
    );
}

#[test]
fn exact_decimal_and_exponent_continuity_does_not_infer_ntsc() {
    let result = accepted(FRACTIONAL);
    assert_eq!(result["clocks"]["declared_frame_rate"], "29.97");
    assert_eq!(
        result["clocks"]["exact_frame_rate"],
        rational("2997", "100")
    );
    assert!(result["clocks"]["explicit_timebase"].is_null());
    assert_eq!(
        result["plan"]["shots"][0]["source_range"]["end"],
        rational("1", "10")
    );
    assert_eq!(
        result["plan"]["shots"][1]["source_range"]["end"],
        rational("1", "5")
    );
    assert_eq!(result["plan"]["assets"][0]["duration"], rational("1", "5"));
}

#[derive(Deserialize)]
struct RawSnapshot {
    source_snapshot: Box<serde_json::value::RawValue>,
}

#[test]
fn source_snapshot_preserves_large_numbers_lexemes_and_input_bytes() {
    let input = FRACTIONAL.to_vec();
    let before = input.clone();
    let imported = import_openfilm_snapshot(&input, &ImportProfileV1).unwrap();
    let output = serde_json::to_vec(&imported).unwrap();
    let retained: RawSnapshot = serde_json::from_slice(&output).unwrap();
    assert_eq!(
        value(retained.source_snapshot.get().as_bytes()),
        value(&input)
    );
    assert!(retained.source_snapshot.get().contains("9007199254740993"));
    assert!(retained.source_snapshot.get().contains("0.1000"));
    assert!(retained.source_snapshot.get().contains("1e-1"));
    assert_eq!(input, before);
    let again = import_openfilm_snapshot(&input, &ImportProfileV1).unwrap();
    assert_eq!(output, serde_json::to_vec(&again).unwrap());
}

#[test]
fn source_snapshot_minification_preserves_escaped_strings_whitespace_and_unicode() {
    let token = r#""  quote: \"; backslash: \\; combined: \\\"; tab: \t; newline: \n; Unicode: 合成 \u5408  ""#;
    let input = std::str::from_utf8(FRACTIONAL)
        .unwrap()
        .replace(
            "\"synthetic.text\": \"合成\"",
            &format!("\"synthetic.text\": {token}"),
        )
        .into_bytes();
    let before = input.clone();
    let imported = import_openfilm_snapshot(&input, &ImportProfileV1).unwrap();
    let output = serde_json::to_vec(&imported).unwrap();
    let retained: RawSnapshot = serde_json::from_slice(&output).unwrap();
    assert!(retained.source_snapshot.get().contains(token));
    let snapshot = value(retained.source_snapshot.get().as_bytes());
    assert_eq!(
        snapshot["assets"][0]["metadata"]["synthetic.text"],
        "  quote: \"; backslash: \\; combined: \\\"; tab: \t; newline: \n; Unicode: 合成 合  "
    );
    assert!(retained.source_snapshot.get().contains("9007199254740993"));
    assert!(retained.source_snapshot.get().contains("0.1000"));
    assert!(retained.source_snapshot.get().contains("1e-1"));
    assert!(!output.contains(&b'\n'));
    assert_eq!(input, before);
}

#[test]
fn seconds_above_f64_integer_precision_remain_exact_and_u64_max_is_allowed() {
    for integer in ["9007199254740993", "18446744073709551615"] {
        let mut snapshot = value(REPEATED);
        clips(&mut snapshot).truncate(1);
        let raw = serde_json::to_string(&snapshot).unwrap();
        let raw = raw
            .replace("\"duration\":5", &format!("\"duration\":{integer}"))
            .replace(
                "\"timelineDuration\":2",
                &format!("\"timelineDuration\":{integer}"),
            );
        let result = accepted(raw.as_bytes());
        assert_eq!(
            result["plan"]["assets"][0]["duration"],
            rational(integer, "1")
        );
        assert_eq!(
            result["plan"]["shots"][0]["source_range"]["end"],
            rational(integer, "1")
        );
    }
}

#[test]
fn explicit_timebase_is_separate_from_its_agreeing_decimal_declaration() {
    let mut snapshot = value(STILL);
    snapshot["settings"]["frameRate"] = json!(29.97002997002997);
    let timebase =
        json!({"version":1, "rateNumerator":30000, "rateDenominator":1001, "dropFrame":true});
    snapshot["settings"]["timebase"] = timebase.clone();
    let result = accepted_value(&snapshot);
    assert_eq!(result["clocks"]["declared_frame_rate"], "29.97002997002997");
    assert_eq!(
        result["clocks"]["exact_frame_rate"],
        rational("2997002997002997", "100000000000000")
    );
    assert_eq!(result["clocks"]["explicit_timebase"], timebase);
    assert_ne!(
        result["clocks"]["exact_frame_rate"],
        rational("30000", "1001")
    );
}

#[test]
fn rejects_mismatched_nonreduced_or_invalid_drop_frame_timebase() {
    for (rate, timebase) in [
        (
            29.97,
            json!({"version":1,"rateNumerator":30000,"rateDenominator":1001,"dropFrame":true}),
        ),
        (
            30.0,
            json!({"version":1,"rateNumerator":60,"rateDenominator":2,"dropFrame":false}),
        ),
        (
            30.0,
            json!({"version":1,"rateNumerator":30,"rateDenominator":1,"dropFrame":true}),
        ),
        (
            30.0,
            json!({"version":1,"rateNumerator":30,"rateDenominator":0,"dropFrame":false}),
        ),
        (
            30.0,
            json!({"version":2,"rateNumerator":30,"rateDenominator":1,"dropFrame":false}),
        ),
        (
            30.0,
            json!({"version":1,"rateNumerator":4294967296_u64,"rateDenominator":1,"dropFrame":false}),
        ),
    ] {
        let mut snapshot = value(STILL);
        snapshot["settings"]["frameRate"] = json!(rate);
        snapshot["settings"]["timebase"] = timebase;
        let error = rejected(&serde_json::to_vec(&snapshot).unwrap());
        assert!(error.code == "import.invalid_snapshot" || error.code == "import.invalid_time");
        assert!(error.losses.is_empty());
    }
}

#[test]
fn rejects_negative_zero_excess_exponents_and_unrepresentable_seconds() {
    for lexeme in [
        "-0",
        "-0.0",
        "-1",
        "1e19",
        "1e-19",
        "18446744073709551616",
        "340282366920938463463374607431768211456",
    ] {
        let raw = std::str::from_utf8(REPEATED).unwrap().replacen(
            "\"timelineStart\": 0",
            &format!("\"timelineStart\": {lexeme}"),
            1,
        );
        assert_eq!(
            rejected(raw.as_bytes()).code,
            "import.invalid_time",
            "{lexeme}"
        );
    }
    let too_long = format!("0.{}1", "0".repeat(63));
    let raw = std::str::from_utf8(REPEATED).unwrap().replacen(
        "\"timelineStart\": 0",
        &format!("\"timelineStart\": {too_long}"),
        1,
    );
    assert_eq!(rejected(raw.as_bytes()).code, "import.invalid_time");
}

#[test]
fn exponent_limits_and_reduction_admit_representable_rationals() {
    for (lexeme, numerator, denominator) in [
        ("1e18", "1000000000000000000", "1"),
        ("1e-18", "1", "1000000000000000000"),
        ("184467440737095516150e-1", "18446744073709551615", "1"),
    ] {
        let mut snapshot = value(REPEATED);
        clips(&mut snapshot).truncate(1);
        let raw = serde_json::to_string(&snapshot)
            .unwrap()
            .replace("\"duration\":5", &format!("\"duration\":{lexeme}"))
            .replace(
                "\"timelineDuration\":2",
                &format!("\"timelineDuration\":{lexeme}"),
            );
        let result = accepted(raw.as_bytes());
        assert_eq!(
            result["plan"]["assets"][0]["duration"],
            rational(numerator, denominator)
        );
    }
}

#[test]
fn rejects_future_envelope_before_invalid_nested_domain_data() {
    let error = rejected(include_bytes!(
        "../../../tests/fixtures/openfilm/finite-v1/future-schema.json"
    ));
    assert_eq!(error.code, "import.unsupported_version");
    assert_eq!(error.path, "$.schemaVersion");
    assert!(error.losses.is_empty());
    let mut snapshot = value(STILL);
    snapshot["schemaVersion"] = json!("2.0.0");
    snapshot["assets"][0]["metadata"]["values"] = json!(vec![Value::Null; 65_536]);
    reject_value(&snapshot, "import.unsupported_version");
}

#[test]
fn unknown_effect_and_current_animation_reject_with_one_explicit_loss() {
    let snapshot = value(include_bytes!(
        "../../../tests/fixtures/openfilm/finite-v1/unknown-effect.json"
    ));
    let error = unsupported(&snapshot);
    assert_eq!(error["path"], "$.composition.tracks[0].clips[0].effects");
    for field in [
        "animation",
        "audioTiming",
        "coverage",
        "userOverrides",
        "transition",
        "transitions",
        "transform",
    ] {
        let mut snapshot = value(STILL);
        snapshot["composition"]["tracks"][0]["clips"][0][field] = json!({});
        unsupported(&snapshot);
    }
    let mut snapshot = value(STILL);
    snapshot["composition"]["tracks"][0]["clips"][0]["transform"] =
        json!({"scale":1,"rotation":0,"x":0,"y":0,"speed":1,"volume":1});
    unsupported(&snapshot);
}

#[test]
fn creative_composition_and_track_extensions_reject_even_when_empty() {
    for field in [
        "storyPresetSnapshot",
        "defaultLook",
        "defaultPrimaryColor",
        "beatPrimaryColors",
        "beatLooks",
        "captionTracks",
        "graphicScenes",
        "graphicSceneTemplates",
        "coverageAudioBindings",
    ] {
        let mut snapshot = value(STILL);
        snapshot["composition"][field] = json!({});
        unsupported(&snapshot);
    }
    let mut snapshot = value(STILL);
    snapshot["composition"]["tracks"][0]["timing"] = json!({});
    unsupported(&snapshot);
}

#[test]
fn audio_tracks_reject_whether_empty_or_populated() {
    for audio_clips in [
        json!([]),
        json!([{"id":"audio-occurrence","assetId":"synthetic-photo","timelineStart":0,"timelineDuration":5}]),
    ] {
        let mut snapshot = value(STILL);
        snapshot["composition"]["tracks"][0]["type"] = json!("audio");
        snapshot["composition"]["tracks"][0]["clips"] = audio_clips.clone();
        unsupported(&snapshot);
        let mut snapshot = value(STILL);
        snapshot["composition"]["tracks"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"audio-track","type":"audio","clips":audio_clips}));
        unsupported(&snapshot);
    }
}

#[test]
fn embedded_audio_rejects_and_missing_or_empty_probe_still_blocks_audio_handoff() {
    let mut snapshot = value(VIDEO);
    snapshot["assets"][0]["metadata"]["openfilm.ffprobe"]["streams"] =
        json!([{"codec_type":"video"}, {"codec_type":"audio"}]);
    unsupported(&snapshot);
    for metadata in [json!({}), json!({"openfilm.ffprobe":{"streams":[]}})] {
        let mut snapshot = value(VIDEO);
        snapshot["assets"][0]["metadata"] = metadata;
        let result = accepted_value(&snapshot);
        assert_eq!(result["losses"][1]["code"], "audio.not_mapped");
        assert_eq!(result["losses"][1]["disposition"], "blocks-host-handoff");
        assert_eq!(result["executable"], false);
    }
}

#[test]
fn malformed_probe_shape_is_not_treated_as_evidence_of_silence() {
    for probe in [
        json!(null),
        json!([]),
        json!({}),
        json!({"streams":{}}),
        json!({"streams":[null]}),
        json!({"streams":[{}]}),
        json!({"streams":[{"codec_type":1}]}),
    ] {
        let mut snapshot = value(VIDEO);
        snapshot["assets"][0]["metadata"]["openfilm.ffprobe"] = probe;
        reject_value(&snapshot, "import.invalid_snapshot");
    }
}

#[test]
fn hidden_muted_or_solo_picture_rejects_instead_of_becoming_visible_default() {
    for (field, setting) in [("visible", false), ("muted", true), ("solo", true)] {
        let mut snapshot = value(STILL);
        snapshot["composition"]["tracks"][0][field] = json!(setting);
        unsupported(&snapshot);
    }
}

#[test]
fn rejects_gaps_overlap_reordering_and_unrepresented_composition_tail() {
    for start in [json!(1), json!(3), json!(2.000000000000001)] {
        let mut snapshot = value(REPEATED);
        snapshot["composition"]["tracks"][0]["clips"][1]["timelineStart"] = start;
        reject_value(&snapshot, "import.inconsistent_timeline");
    }
    let mut snapshot = value(REPEATED);
    clips(&mut snapshot).reverse();
    reject_value(&snapshot, "import.inconsistent_timeline");
    let mut snapshot = value(REPEATED);
    snapshot["composition"]["duration"] = json!(6);
    reject_value(&snapshot, "import.inconsistent_timeline");
    let mut snapshot = value(REPEATED);
    snapshot["composition"]["tracks"][0]["clips"][0]["timelineStart"] = json!(1);
    reject_value(&snapshot, "import.inconsistent_timeline");
}

#[test]
fn rejects_speed_changes_source_overruns_empty_ranges_and_nonzero_image_in() {
    for (source_in, source_out) in [(0.1, 2.5), (9.0, 11.5), (0.1, 0.1), (2.6, 0.1)] {
        let mut snapshot = value(VIDEO);
        snapshot["composition"]["tracks"][0]["clips"][0]["sourceIn"] = json!(source_in);
        snapshot["composition"]["tracks"][0]["clips"][0]["sourceOut"] = json!(source_out);
        reject_value(&snapshot, "import.invalid_range");
    }
    let mut snapshot = value(STILL);
    snapshot["composition"]["tracks"][0]["clips"][0]["sourceIn"] = json!(1);
    snapshot["composition"]["tracks"][0]["clips"][0]["sourceOut"] = json!(6);
    reject_value(&snapshot, "import.invalid_range");
    for duration in [json!(0), json!(-1)] {
        let mut snapshot = value(VIDEO);
        snapshot["assets"][0]["duration"] = duration;
        assert!(
            import_openfilm_snapshot(&serde_json::to_vec(&snapshot).unwrap(), &ImportProfileV1)
                .is_err()
        );
    }
    let mut snapshot = value(VIDEO);
    snapshot["assets"][0]
        .as_object_mut()
        .unwrap()
        .remove("duration");
    reject_value(&snapshot, "import.invalid_snapshot");
}

#[test]
fn rejects_missing_duplicate_unused_source_and_duplicate_occurrence_ids() {
    let mut snapshot = value(REPEATED);
    snapshot["composition"]["tracks"][0]["clips"][1]["assetId"] = json!("missing-source");
    assert!(
        import_openfilm_snapshot(&serde_json::to_vec(&snapshot).unwrap(), &ImportProfileV1)
            .is_err()
    );
    let mut snapshot = value(REPEATED);
    snapshot["composition"]["tracks"][0]["clips"][1]["id"] = json!("occurrence-z");
    reject_value(&snapshot, "plan.duplicate_id");
    let mut snapshot = value(REPEATED);
    let extra = snapshot["assets"][0].clone();
    snapshot["assets"].as_array_mut().unwrap().push(extra);
    reject_value(&snapshot, "plan.duplicate_id");
    let mut snapshot = value(REPEATED);
    let mut extra = snapshot["assets"][0].clone();
    extra["id"] = json!("unused-source");
    snapshot["assets"].as_array_mut().unwrap().push(extra);
    reject_value(&snapshot, "import.invalid_snapshot");
}

#[test]
fn reuses_m0_identity_and_declared_hash_syntax_without_inventing_missing_hashes() {
    for path in [
        "/composition/id",
        "/composition/storyId",
        "/composition/tracks/0/id",
        "/composition/tracks/0/clips/0/id",
        "/composition/tracks/0/clips/0/assetId",
        "/composition/tracks/0/clips/0/beatId",
        "/assets/0/id",
    ] {
        let mut snapshot = value(STILL);
        *snapshot.pointer_mut(path).unwrap() = json!("../synthetic-invalid-id");
        reject_value(&snapshot, "plan.invalid_id");
    }
    for hash in [
        Value::Null,
        json!("A".repeat(64)),
        json!("not-a-hash"),
        json!("a".repeat(63)),
    ] {
        let mut snapshot = value(STILL);
        snapshot["assets"][0]["contentHash"] = hash;
        reject_value(&snapshot, "plan.invalid_hash");
    }
    let mut snapshot = value(STILL);
    snapshot["assets"][0]
        .as_object_mut()
        .unwrap()
        .remove("contentHash");
    reject_value(&snapshot, "plan.invalid_hash");
}

#[test]
fn unknown_typed_fields_and_unadmitted_asset_facts_reject_instead_of_disappearing() {
    for path in [
        "",
        "/composition",
        "/composition/tracks/0",
        "/composition/tracks/0/clips/0",
        "/assets/0",
        "/assets/0/state",
        "/settings",
    ] {
        let mut snapshot = value(STILL);
        snapshot
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("syntheticUnknown".to_owned(), json!(true));
        reject_value(&snapshot, "import.invalid_snapshot");
    }
    for field in [
        "source",
        "capturedAt",
        "dimensions",
        "frameRate",
        "codec",
        "gps",
        "proxyUri",
        "rating",
    ] {
        let mut snapshot = value(STILL);
        snapshot["assets"][0][field] = json!({});
        reject_value(&snapshot, "import.invalid_snapshot");
    }
    let mut snapshot = value(STILL);
    snapshot["settings"]["timebase"] = json!({"version":1,"rateNumerator":30,"rateDenominator":1,"dropFrame":false,"syntheticUnknown":true});
    reject_value(&snapshot, "import.invalid_snapshot");
}

#[test]
fn malformed_json_duplicate_keys_and_wrong_envelope_shapes_reject() {
    for bytes in [b"{".as_slice(), b"[]", b"null", b"true", b"{} {}"] {
        let error = rejected(bytes);
        assert!(error.code == "input.invalid_json" || error.code == "import.invalid_snapshot");
        assert!(error.losses.is_empty());
    }
    let raw = std::str::from_utf8(STILL).unwrap();
    for duplicate in [
        raw.replacen('{', "{\"schemaVersion\":\"1.0.0\",", 1),
        raw.replacen(
            "\"metadata\": {",
            "\"metadata\": {\"duplicate\":1,\"duplicate\":2,",
            1,
        ),
        raw.replacen("\"state\": {", "\"state\": {\"locked\":true,", 1),
    ] {
        assert_eq!(rejected(duplicate.as_bytes()).code, "input.invalid_json");
    }
}

#[test]
fn required_fields_types_and_positive_dimension_rate_limits_are_enforced() {
    for (path, field) in [
        ("", "composition"),
        ("", "assets"),
        ("", "settings"),
        ("/composition", "storyId"),
        ("/composition/tracks/0", "id"),
        ("/composition/tracks/0/clips/0", "assetId"),
        ("/assets/0", "uri"),
        ("/assets/0", "name"),
        ("/assets/0", "tags"),
        ("/assets/0", "state"),
        ("/assets/0", "metadata"),
        ("/settings", "frameRate"),
    ] {
        let mut snapshot = value(STILL);
        snapshot
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(field);
        reject_value(&snapshot, "import.invalid_snapshot");
    }
    for (path, bad) in [
        ("/assets/0/tags", json!([1])),
        ("/assets/0/state/locked", json!(1)),
        ("/assets/0/metadata", json!([])),
        ("/assets/0/uri", json!("")),
        ("/composition/tracks/0/locked", json!("true")),
    ] {
        let mut snapshot = value(STILL);
        *snapshot.pointer_mut(path).unwrap() = bad;
        reject_value(&snapshot, "import.invalid_snapshot");
    }
    for (field, bad) in [
        ("width", json!(0)),
        ("height", json!(16385)),
        ("width", json!(1.5)),
        ("frameRate", json!(0)),
        ("frameRate", json!(240.0001)),
    ] {
        let mut snapshot = value(STILL);
        snapshot["settings"][field] = bad;
        assert!(
            import_openfilm_snapshot(&serde_json::to_vec(&snapshot).unwrap(), &ImportProfileV1)
                .is_err()
        );
    }
    let mut snapshot = value(STILL);
    snapshot["settings"] = json!({"width":16384,"height":16384,"frameRate":240});
    accepted_value(&snapshot);
    for path in [
        "/assets",
        "/composition/tracks",
        "/composition/tracks/0/clips",
    ] {
        let mut snapshot = value(STILL);
        *snapshot.pointer_mut(path).unwrap() = json!([]);
        reject_value(&snapshot, "import.invalid_snapshot");
    }
    for path in [
        "/composition/duration",
        "/composition/tracks/0/clips/0/timelineDuration",
    ] {
        let mut snapshot = value(STILL);
        *snapshot.pointer_mut(path).unwrap() = json!(0);
        assert!(
            import_openfilm_snapshot(&serde_json::to_vec(&snapshot).unwrap(), &ImportProfileV1)
                .is_err()
        );
    }
}

#[test]
fn rejects_oversized_input_and_bounded_metadata_depth_strings_keys_and_values() {
    let mut input = STILL.to_vec();
    input.resize(1_048_577, b' ');
    assert_eq!(rejected(&input).code, "input.too_large");
    let mut nested = Value::Null;
    for _ in 0..33 {
        nested = json!([nested]);
    }
    let mut long_key = serde_json::Map::new();
    long_key.insert("k".repeat(129), Value::Null);
    for metadata in [
        json!({"nested":nested}),
        json!({"text":"合".repeat(1366)}),
        Value::Object(long_key),
        json!({"values":vec![Value::Null;65_536]}),
    ] {
        let mut snapshot = value(STILL);
        snapshot["assets"][0]["metadata"] = metadata;
        reject_value(&snapshot, "import.budget_exceeded");
    }
    let mut allowed_metadata = serde_json::Map::new();
    allowed_metadata.insert("k".repeat(128), json!("a".repeat(4096)));
    let mut snapshot = value(STILL);
    snapshot["assets"][0]["metadata"] = Value::Object(allowed_metadata);
    accepted_value(&snapshot);
}

#[test]
fn rejects_over_budget_assets_and_clips_before_mapping_identities() {
    let mut snapshot = value(STILL);
    snapshot["assets"] = json!(vec![snapshot["assets"][0].clone(); 1025]);
    reject_value(&snapshot, "import.budget_exceeded");
    let mut snapshot = value(STILL);
    snapshot["composition"]["tracks"][0]["clips"] = json!(vec![
        json!({"id":"repeated","assetId":"synthetic-photo","timelineStart":0,"timelineDuration":1});
        4097
    ]);
    reject_value(&snapshot, "import.budget_exceeded");
}

#[test]
fn compact_source_whose_mapped_plan_exceeds_m0_input_limit_rejects() {
    let asset_id = format!("a{}", "s".repeat(64));
    let occurrences: Vec<Value> = (0..4096)
        .map(|index| {
            json!({
                "id":format!("c{index:04}{}", "s".repeat(60)),
                "assetId":asset_id,
                "timelineStart":index,
                "timelineDuration":1
            })
        })
        .collect();
    let mut snapshot = value(REPEATED);
    snapshot["composition"]["duration"] = json!(4096);
    snapshot["composition"]["tracks"][0]["clips"] = json!(occurrences);
    snapshot["assets"][0]["id"] = json!(asset_id);
    let bytes = serde_json::to_vec(&snapshot).unwrap();
    assert!(
        bytes.len() <= 1_048_576,
        "regression must fit source input budget"
    );
    assert_eq!(rejected(&bytes).code, "import.output_too_large");
}

#[test]
fn errors_use_fixed_paths_without_untrusted_metadata_keys_uris_or_parser_prose() {
    let synthetic_marker = "synthetic-sensitive-marker";
    let mut metadata = serde_json::Map::new();
    metadata.insert(synthetic_marker.to_owned(), json!("a".repeat(4097)));
    let mut snapshot = value(STILL);
    snapshot["assets"][0]["uri"] = json!(format!("synthetic://{synthetic_marker}/photo.jpg"));
    snapshot["assets"][0]["metadata"] = Value::Object(metadata);
    let error = reject_value(&snapshot, "import.budget_exceeded");
    let encoded = serde_json::to_string(&error).unwrap();
    assert!(!encoded.contains(synthetic_marker));
    assert!(!encoded.contains("synthetic://"));
    assert_eq!(error.path, "$");
    assert!(error.losses.is_empty());
    let error = rejected(format!("{{\"{synthetic_marker}\":").as_bytes());
    assert_eq!(error.code, "input.invalid_json");
    assert_eq!(error.path, "$");
    assert!(
        !serde_json::to_string(&error)
            .unwrap()
            .contains(synthetic_marker)
    );
}

#[test]
fn optional_fields_reject_explicit_null_instead_of_erasing_declarations() {
    for (parent, field) in [
        ("/composition/tracks/0", "locked"),
        ("/composition/tracks/0", "target"),
        ("/composition/tracks/0", "visible"),
        ("/composition/tracks/0", "muted"),
        ("/composition/tracks/0", "solo"),
        ("/composition/tracks/0/clips/0", "beatId"),
        ("/composition/tracks/0/clips/0", "locked"),
        ("/composition/tracks/0/clips/0", "sourceIn"),
        ("/composition/tracks/0/clips/0", "sourceOut"),
        ("/assets/0/state", "favorite"),
        ("/assets/0/state", "rejected"),
        ("/assets/0/state", "locked"),
        ("/assets/0", "duration"),
        ("/settings", "timebase"),
    ] {
        let mut snapshot = value(STILL);
        snapshot
            .pointer_mut(parent)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(field.to_owned(), Value::Null);
        assert!(
            import_openfilm_snapshot(&serde_json::to_vec(&snapshot).unwrap(), &ImportProfileV1)
                .is_err(),
            "{parent}/{field}"
        );
    }
}
