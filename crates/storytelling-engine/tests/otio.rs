// SPDX-License-Identifier: Apache-2.0
// Source-informed contract tests; independently authored synthetic declarations.
// Origin: bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76,
// packages/exporters/src/index.ts and packages/core/src/{models,timebase}.ts.
// Native content ticks intentionally differ from upstream project-rate OTIO.
// No upstream implementation, upstream fixture or private media is copied.

use serde_json::{Value, json};
use storytelling_core::compatibility::ImportProfileV1;
use storytelling_core::compiled_plan::OtioCompileError;
use storytelling_engine::{
    OtioLossChoicesV1, OtioProfileV1, export_openfilm_otio, import_openfilm_snapshot,
};

const STILL: &[u8] = include_bytes!("../../../tests/fixtures/openfilm/finite-v1/still.json");
const VIDEO: &[u8] = include_bytes!("../../../tests/fixtures/openfilm/finite-v1/video-trim.json");
const REPEATED: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/repeat-source.json");
const FRACTIONAL: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/fractional-rate.json");
const UNKNOWN_EFFECT: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/unknown-effect.json");
const FUTURE_SCHEMA: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/future-schema.json");
const MAX_NATIVE_INTEGER: u64 = 9_007_199_254_740_991;

fn value(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap()
}

fn accepting_choices() -> OtioLossChoicesV1 {
    OtioLossChoicesV1 {
        picture_only: true,
        metadata_only_locks: true,
    }
}

fn accepted_with(bytes: &[u8], choices: &OtioLossChoicesV1) -> Value {
    let result = export_openfilm_otio(bytes, &OtioProfileV1, choices)
        .expect("accepted finite OTIO input");
    assert_eq!(
        result.document["metadata"]["storytelling"]["report"],
        serde_json::to_value(&result.report).unwrap(),
        "the public typed report and embedded report must agree"
    );
    result.document
}

fn accepted(bytes: &[u8]) -> Value {
    accepted_with(bytes, &accepting_choices())
}

fn accepted_value(snapshot: &Value) -> Value {
    accepted(&serde_json::to_vec(snapshot).unwrap())
}

fn rejected_with(bytes: &[u8], choices: &OtioLossChoicesV1) -> OtioCompileError {
    export_openfilm_otio(bytes, &OtioProfileV1, choices)
        .expect_err("rejected compilation must not return a document")
}

fn rejected(bytes: &[u8]) -> OtioCompileError {
    rejected_with(bytes, &accepting_choices())
}

fn reject_value(snapshot: &Value, code: &str, path: &str) -> OtioCompileError {
    let error = rejected(&serde_json::to_vec(snapshot).unwrap());
    assert_eq!(error.code, code);
    assert_eq!(error.path, path);
    error
}

fn report(document: &Value) -> &Value {
    &document["metadata"]["storytelling"]["report"]
}

fn track(document: &Value) -> &Value {
    &document["tracks"]["children"][0]
}

fn clips(document: &Value) -> &[Value] {
    track(document)["children"].as_array().unwrap()
}

fn source_clips(snapshot: &mut Value) -> &mut Vec<Value> {
    snapshot["composition"]["tracks"][0]["clips"]
        .as_array_mut()
        .unwrap()
}

fn rational(numerator: &str, denominator: &str) -> Value {
    json!({"numerator":numerator,"denominator":denominator})
}

fn native_time(value: u64, rate: u64) -> Value {
    json!({"OTIO_SCHEMA":"RationalTime.1","value":value,"rate":rate})
}

fn native_range(start: u64, duration: u64, rate: u64) -> Value {
    json!({
        "OTIO_SCHEMA":"TimeRange.1",
        "start_time":native_time(start, rate),
        "duration":native_time(duration, rate)
    })
}

fn feature(feature: &str, path: &str, disposition: &str, accepted: bool) -> Value {
    json!({"feature":feature,"path":path,"disposition":disposition,"accepted":accepted})
}

fn assert_unrepresentable(error: OtioCompileError, path: &str) {
    assert_eq!(error.code, "otio.unrepresentable_time");
    assert_eq!(error.path, path);
    assert_eq!(
        serde_json::to_value(error.losses).unwrap(),
        json!([feature("clock.unrepresentable", path, "unsupported", false)])
    );
}

fn one_image_hold(duration: u64) -> Value {
    let mut snapshot = value(REPEATED);
    snapshot["composition"]["duration"] = json!(duration);
    source_clips(&mut snapshot).truncate(1);
    source_clips(&mut snapshot)[0]["timelineDuration"] = json!(duration);
    snapshot
}

fn encode_number_marker(snapshot: &Value, marker: &str, number: &str) -> Vec<u8> {
    serde_json::to_string(snapshot)
        .unwrap()
        .replace(&serde_json::to_string(marker).unwrap(), number)
        .into_bytes()
}

#[test]
fn still_emits_native_timeline_track_clip_and_synthetic_hold_reference() {
    let document = accepted(STILL);
    assert_eq!(document["OTIO_SCHEMA"], "Timeline.1");
    assert_eq!(document["name"], "finite-still");
    assert_eq!(document.get("global_start_time"), Some(&Value::Null));
    assert_eq!(document["tracks"]["OTIO_SCHEMA"], "Stack.1");
    assert_eq!(document["tracks"]["name"], "storytelling");
    assert_eq!(document["tracks"]["children"].as_array().unwrap().len(), 1);
    assert_eq!(document["tracks"].get("source_range"), Some(&Value::Null));
    assert_eq!(track(&document)["OTIO_SCHEMA"], "Track.1");
    assert_eq!(track(&document)["name"], "picture-track");
    assert_eq!(track(&document)["kind"], "Video");
    assert_eq!(track(&document).get("source_range"), Some(&Value::Null));
    assert_eq!(clips(&document).len(), 1);
    let clip = &clips(&document)[0];
    assert_eq!(clip["OTIO_SCHEMA"], "Clip.1");
    assert_eq!(clip["name"], "Synthetic photo");
    assert_eq!(clip["source_range"], native_range(0, 5, 1));
    assert_eq!(
        clip["metadata"]["storytelling"],
        json!({
            "occurrence_id":"still-occurrence", "asset_id":"synthetic-photo",
            "source_range":{"start":rational("0","1"),"end":rational("5","1")}
        })
    );
    let reference = &clip["media_reference"];
    assert_eq!(reference["OTIO_SCHEMA"], "ExternalReference.1");
    assert_eq!(reference["name"], "Synthetic photo");
    assert_eq!(reference["target_url"], "synthetic://finite/photo.jpg");
    assert_eq!(reference["available_range"], native_range(0, 5, 1));
    assert_eq!(
        reference["metadata"]["storytelling"],
        json!({
            "asset_id":"synthetic-photo", "declared_content_hash":"a".repeat(64),
            "media_type":"image", "available_range_kind":"synthetic-still-hold"
        })
    );
    for object in [&document["tracks"], track(&document), clip] {
        assert_eq!(object["effects"], json!([]));
        assert_eq!(object["markers"], json!([]));
    }
    assert!(clip.get("media_references").is_none());
}

#[test]
fn video_trim_preserves_exact_source_clock_and_declared_available_duration() {
    let document = accepted(VIDEO);
    let clip = &clips(&document)[0];
    assert_eq!(clip["source_range"], native_range(1, 25, 10));
    assert_eq!(clip["media_reference"]["available_range"], native_range(0, 100, 10));
    assert_eq!(
        clip["media_reference"]["metadata"]["storytelling"]["available_range_kind"],
        "declared-video-duration"
    );
    assert_eq!(
        clip["metadata"]["storytelling"]["source_range"],
        json!({"start":rational("1","10"),"end":rational("13","5")})
    );
    assert_eq!(report(&document)["content_clock_rate"], "10");
    assert_eq!(report(&document)["timeline_duration"], rational("5", "2"));
}

#[test]
fn repeated_source_keeps_distinct_occurrences_order_and_maximum_hold_window() {
    let document = accepted(REPEATED);
    let occurrences = clips(&document);
    assert_eq!(occurrences.len(), 2);
    assert_eq!(occurrences[0]["metadata"]["storytelling"]["occurrence_id"], "occurrence-z");
    assert_eq!(occurrences[1]["metadata"]["storytelling"]["occurrence_id"], "occurrence-a");
    for clip in occurrences {
        assert_eq!(clip["metadata"]["storytelling"]["asset_id"], "reused-photo");
        assert_eq!(clip["name"], "Synthetic repeated photo");
        assert_eq!(clip["media_reference"]["target_url"], "synthetic://finite/reused.jpg");
        assert_eq!(clip["media_reference"]["available_range"], native_range(0, 3, 1));
    }
    assert_eq!(occurrences[0]["source_range"], native_range(0, 2, 1));
    assert_eq!(occurrences[1]["source_range"], native_range(0, 3, 1));
    assert_eq!(report(&document)["timeline_duration"], rational("5", "1"));
}

#[test]
fn fractional_cuts_share_integer_content_clock_without_project_fps_quantization() {
    let document = accepted(FRACTIONAL);
    assert_eq!(report(&document)["content_clock_rate"], "10");
    assert_eq!(clips(&document)[0]["source_range"], native_range(0, 1, 10));
    assert_eq!(clips(&document)[1]["source_range"], native_range(0, 2, 10));
    assert_eq!(report(&document)["timeline_duration"], rational("3", "10"));
    assert_eq!(report(&document)["project_clocks"]["declared_frame_rate"], "29.97");
    assert_eq!(report(&document)["project_clocks"]["exact_frame_rate"], rational("2997", "100"));
    assert!(document["global_start_time"].is_null());
    let total: u64 = clips(&document)
        .iter()
        .map(|clip| clip["source_range"]["duration"]["value"].as_u64().unwrap())
        .sum();
    assert_eq!(total, 3);
}

#[test]
fn original_rate_lexeme_exact_rational_and_explicit_timebase_remain_separate() {
    let mut snapshot = value(REPEATED);
    snapshot["settings"]["frameRate"] = json!("synthetic-rate-number");
    let timebase = json!({"version":1,"rateNumerator":30000,"rateDenominator":1001,"dropFrame":true});
    snapshot["settings"]["timebase"] = timebase.clone();
    let input = encode_number_marker(&snapshot, "synthetic-rate-number", "2.997002997002997e1");
    let document = accepted(&input);
    let clocks = &report(&document)["project_clocks"];
    assert_eq!(clocks["declared_frame_rate"], "2.997002997002997e1");
    assert_eq!(clocks["exact_frame_rate"], rational("2997002997002997", "100000000000000"));
    assert_eq!(clocks["explicit_timebase"], timebase);
    assert_eq!(report(&document)["content_clock_rate"], "1");
}

#[test]
fn report_retains_import_limitations_and_never_claims_source_host_or_execution_verification() {
    for input in [STILL, VIDEO, REPEATED, FRACTIONAL] {
        let imported = import_openfilm_snapshot(input, &ImportProfileV1).unwrap();
        let document = accepted(input);
        let report = report(&document);
        assert_eq!(report["schema_version"], 1);
        assert_eq!(report["profile"], "storytelling.otio-visual-ticks.v1");
        assert_eq!(report["scope"], "declared-otio-visual-interchange");
        for flag in ["executable", "source_verified", "host_verified"] {
            assert_eq!(report[flag], false);
        }
        assert!(report["host_name"].is_null());
        assert!(report["host_version"].is_null());
        assert_eq!(report["choices"], json!({"picture_only":true,"metadata_only_locks":true}));
        assert_eq!(report["required_capabilities"], json!([
            "single-video-track", "external-reference", "still-hold", "arbitrary-content-tick-rate"
        ]));
        assert_eq!(report["import_limitations"], serde_json::to_value(imported.losses).unwrap());
        assert_eq!(report["project_clocks"], serde_json::to_value(imported.clocks).unwrap());
        for forbidden in ["digest", "approval", "approval_token", "verified"] {
            assert!(report.get(forbidden).is_none());
        }
    }
}

#[test]
fn feature_matrix_has_exact_order_dispositions_and_metadata_only_lock_disclosure() {
    let document = accepted(STILL);
    assert_eq!(report(&document)["feature_matrix"], json!([
        feature("timeline.cuts", "$.composition", "native", true),
        feature("media.references", "$.assets", "native", true),
        feature("image.holds", "$.assets", "native", true),
        feature("clock.content_ticks", "$.settings", "native", true),
        feature("project_rate.metadata_only", "$.settings", "metadata-only", true),
        feature("source_facts.metadata_only", "$", "metadata-only", true),
        feature("editing.locks_metadata_only", "$", "metadata-only", true)
    ]));
}

#[test]
fn unlocked_images_need_no_loss_choice_and_default_choices_are_false() {
    let choices = OtioLossChoicesV1::default();
    assert!(!choices.picture_only);
    assert!(!choices.metadata_only_locks);
    let document = accepted_with(REPEATED, &choices);
    assert_eq!(report(&document)["choices"], json!({"picture_only":false,"metadata_only_locks":false}));
    assert_eq!(report(&document)["feature_matrix"].as_array().unwrap().len(), 6);
    assert!(report(&document)["feature_matrix"].as_array().unwrap().iter().all(|row| row["accepted"] == true));
}

#[test]
fn video_requires_explicit_picture_only_even_without_a_declared_audio_stream() {
    let error = rejected_with(VIDEO, &OtioLossChoicesV1::default());
    assert_eq!(error.code, "otio.loss_choice_required");
    assert_eq!(error.path, "$.choices");
    assert_eq!(serde_json::to_value(error.losses).unwrap(), json!([
        feature("audio.picture_only", "$.assets", "metadata-only", false)
    ]));
    let document = accepted_with(VIDEO, &OtioLossChoicesV1 {
        picture_only: true,
        metadata_only_locks: false,
    });
    assert_eq!(report(&document)["feature_matrix"], json!([
        feature("timeline.cuts", "$.composition", "native", true),
        feature("media.references", "$.assets", "native", true),
        feature("clock.content_ticks", "$.settings", "native", true),
        feature("project_rate.metadata_only", "$.settings", "metadata-only", true),
        feature("source_facts.metadata_only", "$", "metadata-only", true),
        feature("audio.picture_only", "$.assets", "metadata-only", true)
    ]));
    assert_eq!(report(&document)["import_limitations"][1]["code"], "audio.not_mapped");
    assert_eq!(report(&document)["import_limitations"][1]["disposition"], "blocks-host-handoff");
}

#[test]
fn each_admitted_lock_path_requires_its_explicit_metadata_only_choice() {
    for path in [
        "/composition/tracks/0/locked",
        "/composition/tracks/0/clips/0/locked",
        "/assets/0/state/locked",
    ] {
        let mut snapshot = value(STILL);
        snapshot["composition"]["tracks"][0]["locked"] = json!(false);
        source_clips(&mut snapshot)[0]["locked"] = json!(false);
        snapshot["assets"][0]["state"]["locked"] = json!(false);
        *snapshot.pointer_mut(path).unwrap() = json!(true);
        let input = serde_json::to_vec(&snapshot).unwrap();
        let error = rejected_with(&input, &OtioLossChoicesV1::default());
        assert_eq!(error.code, "otio.loss_choice_required", "{path}");
        assert_eq!(error.path, "$.choices");
        assert_eq!(serde_json::to_value(error.losses).unwrap(), json!([
            feature("editing.locks_metadata_only", "$", "metadata-only", false)
        ]));
        let document = accepted_with(&input, &OtioLossChoicesV1 {
            picture_only: false,
            metadata_only_locks: true,
        });
        let rows = report(&document)["feature_matrix"].as_array().unwrap();
        assert_eq!(rows.last().unwrap(), &feature("editing.locks_metadata_only", "$", "metadata-only", true));
        assert_eq!(report(&document)["host_verified"], false);
        let retained = document["metadata"]["storytelling"]["source_snapshot_json"].as_str().unwrap();
        assert_eq!(value(retained.as_bytes()).pointer(path), Some(&json!(true)));
    }
}

#[test]
fn audio_and_lock_losses_are_ordered_and_each_choice_only_accepts_its_own_loss() {
    let mut snapshot = value(VIDEO);
    snapshot["composition"]["tracks"][0]["locked"] = json!(true);
    let input = serde_json::to_vec(&snapshot).unwrap();
    for (picture_only, metadata_only_locks, features) in [
        (false, false, vec!["audio.picture_only", "editing.locks_metadata_only"]),
        (true, false, vec!["editing.locks_metadata_only"]),
        (false, true, vec!["audio.picture_only"]),
    ] {
        let error = rejected_with(&input, &OtioLossChoicesV1 { picture_only, metadata_only_locks });
        assert_eq!(error.code, "otio.loss_choice_required");
        assert_eq!(error.path, "$.choices");
        let losses = serde_json::to_value(error.losses).unwrap();
        assert_eq!(losses.as_array().unwrap().iter().map(|loss| loss["feature"].as_str().unwrap()).collect::<Vec<_>>(), features);
        assert!(losses.as_array().unwrap().iter().all(|loss| loss["accepted"] == false));
    }
    assert_eq!(report(&accepted(&input))["host_verified"], false);
}

#[test]
fn unsupported_source_features_keep_import_codes_paths_and_unaccepted_rows() {
    let mut audio = value(VIDEO);
    audio["assets"][0]["metadata"]["openfilm.ffprobe"]["streams"] = json!([{"codec_type":"audio"}]);
    for (input, path) in [
        (UNKNOWN_EFFECT.to_vec(), "$.composition.tracks[0].clips[0].effects"),
        (serde_json::to_vec(&audio).unwrap(), "$.assets[0].metadata"),
    ] {
        let original = import_openfilm_snapshot(&input, &ImportProfileV1).unwrap_err();
        let error = rejected(&input);
        assert_eq!(error.code, original.code);
        assert_eq!(error.path, original.path);
        assert_eq!(error.path, path);
        assert_eq!(serde_json::to_value(error.losses).unwrap(), json!([
            feature("import.unsupported_feature", path, "unsupported", false)
        ]));
    }
}

#[test]
fn future_schema_keeps_import_error_without_loss_choices_or_document() {
    let error = rejected_with(FUTURE_SCHEMA, &OtioLossChoicesV1::default());
    assert_eq!(error.code, "import.unsupported_version");
    assert_eq!(error.path, "$.schemaVersion");
    assert!(error.losses.is_empty());
    assert!(serde_json::to_value(error).unwrap().get("document").is_none());
}

#[test]
fn malformed_duplicate_or_over_budget_input_preserves_strict_import_errors() {
    let duplicate = std::str::from_utf8(REPEATED).unwrap().replacen('{', "{\"schemaVersion\":\"1.0.0\",", 1);
    let mut oversized = REPEATED.to_vec();
    oversized.resize(1_048_577, b' ');
    for input in [b"{".to_vec(), b"[]".to_vec(), duplicate.into_bytes(), oversized] {
        let original = import_openfilm_snapshot(&input, &ImportProfileV1).unwrap_err();
        let error = rejected(&input);
        assert_eq!(error.code, original.code);
        assert_eq!(error.path, original.path);
        assert!(error.losses.is_empty());
    }
}

#[test]
fn import_validation_precedes_loss_choices_and_loss_choices_precede_clock_projection() {
    let mut snapshot = one_image_hold(MAX_NATIVE_INTEGER + 1);
    snapshot["composition"]["tracks"][0]["locked"] = json!(true);
    let input = serde_json::to_vec(&snapshot).unwrap();
    let error = rejected_with(&input, &OtioLossChoicesV1::default());
    assert_eq!(error.code, "otio.loss_choice_required");
    assert_unrepresentable(rejected(&input), "$.assets[0].duration");
    source_clips(&mut snapshot)[0]["effects"] = json!([]);
    let input = serde_json::to_vec(&snapshot).unwrap();
    let error = rejected_with(&input, &OtioLossChoicesV1::default());
    assert_eq!(error.code, "import.unsupported_feature");
    assert_eq!(error.path, "$.composition.tracks[0].clips[0].effects");
}

#[test]
fn owner_snapshot_is_a_string_preserving_large_integer_decimal_and_opaque_exponent_lexemes() {
    let input = std::str::from_utf8(FRACTIONAL).unwrap()
        .replace("\"synthetic.text\": \"合成\"", "\"synthetic.text\":\"合成\",\"synthetic.opaque\":1e10000")
        .into_bytes();
    let imported = import_openfilm_snapshot(&input, &ImportProfileV1).unwrap();
    let document = accepted(&input);
    let retained = document["metadata"]["storytelling"]["source_snapshot_json"].as_str().unwrap();
    assert_eq!(retained, imported.source_snapshot.get());
    for lexeme in ["9007199254740993", "0.1000", "1e-1", "0.10", "0.20", "1e10000"] {
        assert!(retained.contains(lexeme), "lost source lexeme {lexeme}");
    }
    let encoded = serde_json::to_vec(&document).unwrap();
    let decoded: Value = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(decoded["metadata"]["storytelling"]["source_snapshot_json"], retained);
}

#[test]
fn retained_source_strings_keep_whitespace_escapes_and_unicode() {
    let mut snapshot = value(REPEATED);
    let text = "  quote: \"; backslash: \\; tab: \t; newline: \n; Unicode: 合成  ";
    snapshot["assets"][0]["metadata"] = json!({"synthetic.text":text});
    let document = accepted_value(&snapshot);
    let retained = document["metadata"]["storytelling"]["source_snapshot_json"].as_str().unwrap();
    assert_eq!(value(retained.as_bytes())["assets"][0]["metadata"]["synthetic.text"], text);
    assert!(!serde_json::to_vec(&document).unwrap().contains(&b'\n'));
}

#[test]
fn opaque_external_uris_are_preserved_without_resolution_or_media_access() {
    for uri in [
        "file:///synthetic-missing-otio-source/photo%20one.jpg?raw=%2F#keep",
        "https://synthetic.invalid/%2f//photo.jpg?x=one%20two&x=three#合成",
        "synthetic:opaque ../unresolved%2520name",
    ] {
        let mut snapshot = value(REPEATED);
        snapshot["assets"][0]["uri"] = json!(uri);
        let document = accepted_value(&snapshot);
        for clip in clips(&document) {
            assert_eq!(clip["media_reference"]["target_url"], uri);
        }
        assert_eq!(report(&document)["source_verified"], false);
    }
}

#[test]
fn compilation_is_deterministic_and_preserves_input_bytes() {
    for original in [STILL, VIDEO, REPEATED, FRACTIONAL] {
        let input = original.to_vec();
        let before = input.clone();
        let first = export_openfilm_otio(&input, &OtioProfileV1, &accepting_choices()).unwrap();
        let second = export_openfilm_otio(&input, &OtioProfileV1, &accepting_choices()).unwrap();
        assert_eq!(serde_json::to_vec(&first).unwrap(), serde_json::to_vec(&second).unwrap());
        assert_eq!(input, before);
    }
}

fn assert_native_clocks(node: &Value, common_rate: u64, count: &mut usize) {
    match node {
        Value::Object(fields) => {
            if fields.get("OTIO_SCHEMA").and_then(Value::as_str) == Some("RationalTime.1") {
                let value = fields["value"].as_u64().expect("native value must be an integer number");
                let rate = fields["rate"].as_u64().expect("native rate must be an integer number");
                assert!(value <= MAX_NATIVE_INTEGER);
                assert!((1..=MAX_NATIVE_INTEGER).contains(&rate));
                assert_eq!(rate, common_rate);
                *count += 1;
            }
            for child in fields.values() {
                assert_native_clocks(child, common_rate, count);
            }
        }
        Value::Array(children) => {
            for child in children {
                assert_native_clocks(child, common_rate, count);
            }
        }
        _ => {}
    }
}

#[test]
fn every_native_time_is_an_integer_number_on_one_safe_common_clock() {
    for input in [STILL, VIDEO, REPEATED, FRACTIONAL] {
        let document = accepted(input);
        let common_rate = report(&document)["content_clock_rate"].as_str().unwrap().parse().unwrap();
        let mut count = 0;
        assert_native_clocks(&document, common_rate, &mut count);
        assert_eq!(count, clips(&document).len() * 4);
    }
}

#[test]
fn exact_safe_integer_boundary_is_accepted_and_larger_native_windows_reject() {
    let document = accepted_value(&one_image_hold(MAX_NATIVE_INTEGER));
    assert_eq!(clips(&document)[0]["source_range"], native_range(0, MAX_NATIVE_INTEGER, 1));
    assert_eq!(report(&document)["timeline_duration"], rational("9007199254740991", "1"));
    let mut count = 0;
    assert_native_clocks(&document, 1, &mut count);
    for duration in [MAX_NATIVE_INTEGER + 1, MAX_NATIVE_INTEGER + 2, u64::MAX] {
        let snapshot = one_image_hold(duration);
        let input = serde_json::to_vec(&snapshot).unwrap();
        assert!(import_openfilm_snapshot(&input, &ImportProfileV1).is_ok());
        assert_unrepresentable(rejected(&input), "$.assets[0].duration");
    }
}

#[test]
fn common_denominator_lcm_rejects_even_when_each_input_denominator_is_safe() {
    let mut snapshot = value(REPEATED);
    snapshot["composition"]["duration"] = json!("synthetic-total");
    source_clips(&mut snapshot)[0]["timelineDuration"] = json!("synthetic-first");
    source_clips(&mut snapshot)[1]["timelineStart"] = json!("synthetic-first");
    source_clips(&mut snapshot)[1]["timelineDuration"] = json!("synthetic-second");
    let input = serde_json::to_string(&snapshot).unwrap()
        .replace("\"synthetic-total\"", "0.0000152587956161")
        .replace("\"synthetic-first\"", "0.0000152587890625")
        .replace("\"synthetic-second\"", "0.0000000000065536")
        .into_bytes();
    let imported = import_openfilm_snapshot(&input, &ImportProfileV1).unwrap();
    assert_eq!(imported.plan.shots[0].source_range.end.denominator, "65536");
    assert_eq!(imported.plan.shots[1].source_range.end.denominator, "152587890625");
    assert_unrepresentable(rejected(&input), "$.content_clock");
    let mut snapshot = one_image_hold(1);
    snapshot["composition"]["duration"] = json!("synthetic-small");
    source_clips(&mut snapshot)[0]["timelineDuration"] = json!("synthetic-small");
    let input = encode_number_marker(&snapshot, "synthetic-small", "1e-16");
    assert!(import_openfilm_snapshot(&input, &ImportProfileV1).is_ok());
    assert_unrepresentable(rejected(&input), "$.content_clock");
}

#[test]
fn declared_available_duration_must_fit_after_scaling_to_an_otherwise_safe_clock() {
    let mut snapshot = value(VIDEO);
    snapshot["composition"]["duration"] = json!("synthetic-small");
    source_clips(&mut snapshot)[0]["sourceIn"] = json!(0);
    source_clips(&mut snapshot)[0]["sourceOut"] = json!("synthetic-small");
    source_clips(&mut snapshot)[0]["timelineDuration"] = json!("synthetic-small");
    let input = encode_number_marker(&snapshot, "synthetic-small", "1e-15");
    assert!(import_openfilm_snapshot(&input, &ImportProfileV1).is_ok());
    assert_unrepresentable(rejected(&input), "$.assets[0].duration");
}

#[test]
fn accumulated_timeline_duration_checks_the_safe_integer_boundary_without_rounding() {
    let half = 4_503_599_627_370_496_u64;
    for (first, expected_total) in [(half - 1, MAX_NATIVE_INTEGER), (half, MAX_NATIVE_INTEGER + 1)] {
        let mut snapshot = value(REPEATED);
        snapshot["composition"]["duration"] = json!(expected_total);
        source_clips(&mut snapshot)[0]["timelineDuration"] = json!(first);
        source_clips(&mut snapshot)[1]["timelineStart"] = json!(first);
        source_clips(&mut snapshot)[1]["timelineDuration"] = json!(half);
        let input = serde_json::to_vec(&snapshot).unwrap();
        assert!(import_openfilm_snapshot(&input, &ImportProfileV1).is_ok());
        if expected_total == MAX_NATIVE_INTEGER {
            let document = accepted(&input);
            assert_eq!(report(&document)["timeline_duration"], rational("9007199254740991", "1"));
            assert_eq!(clips(&document)[0]["source_range"], native_range(0, first, 1));
            assert_eq!(clips(&document)[1]["source_range"], native_range(0, half, 1));
        } else {
            assert_unrepresentable(rejected(&input), "$.timeline_duration");
        }
    }
}

#[test]
fn bounded_source_with_amplified_repeated_names_rejects_the_final_otio_budget() {
    let mut snapshot = value(REPEATED);
    snapshot["assets"][0]["name"] = json!("n".repeat(4096));
    snapshot["composition"]["duration"] = json!(512);
    snapshot["composition"]["tracks"][0]["clips"] = json!((0..512).map(|index| json!({
        "id":format!("occurrence-{index}"), "assetId":"reused-photo",
        "timelineStart":index, "timelineDuration":1
    })).collect::<Vec<_>>());
    let input = serde_json::to_vec(&snapshot).unwrap();
    assert!(input.len() <= 1_048_576);
    assert!(import_openfilm_snapshot(&input, &ImportProfileV1).is_ok());
    let error = rejected(&input);
    assert_eq!(error.code, "otio.output_too_large");
    assert_eq!(error.path, "$");
    assert!(error.losses.is_empty());
    assert!(serde_json::to_value(error).unwrap().get("document").is_none());
}
