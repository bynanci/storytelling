// SPDX-License-Identifier: Apache-2.0
// Synthetic contract/security regressions; provenance-m3-planner-review.json.
use serde_json::{Value, json};
use std::fmt::Write as _;
use storytelling_engine::{
    MAX_INPUT_BYTES, MAX_REVIEW_CONTEXT_BYTES, MAX_STORY_PROPOSAL_BYTES, MAX_STORY_REVIEW_BYTES,
    check_story_review, plan_story, prepare_story_review,
};

fn snapshot() -> Value {
    serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/openfilm/finite-v1/video-trim.json"
    ))
    .unwrap()
}

fn context() -> Vec<u8> {
    serde_json::to_vec(&json!({"schema_version":1,"project_id":"project-A","target_revision":"read-only",
        "host":{"id":"filmcraft","version":"0.18.0","profile_id":"declared-only","binary_sha256":"a".repeat(64),
        "declared_capabilities":["arbitrary-content-tick-rate","external-reference","single-video-track","still-hold"]},
        "loss_choices":["picture_only"]})).unwrap()
}

fn proposal_for(assets: Vec<Value>, candidates: Vec<Value>, max_shots: u32) -> Vec<u8> {
    let request = json!({"schema_version":1,"brief":{"schema_version":1,"brief_id":"proposal-A","intent":"Synthetic",
        "assets":assets,"manual_selected_asset_ids":[],"hard_constraints":[],"preferences":{"preferred_asset_ids":[]}},
        "candidates":candidates,"max_duration":{"numerator":"10","denominator":"1"},"max_shots":max_shots});
    let planning = plan_story(&serde_json::to_vec(&request).unwrap()).unwrap();
    serde_json::to_vec(&json!({"ok":true,"planning":planning})).unwrap()
}

fn candidate(id: &str, hash: char, duration: u32, start: u32, end: u32, score: i32) -> Value {
    json!({"asset_id":id,"content_hash":hash.to_string().repeat(64),"duration":{"numerator":duration.to_string(),"denominator":"1"},
        "source_range":{"start":{"numerator":start.to_string(),"denominator":"1"},"end":{"numerator":end.to_string(),"denominator":"1"}},"score":score})
}

fn proposal() -> Vec<u8> {
    proposal_for(
        vec![json!({"id":"synthetic-video"})],
        vec![candidate("synthetic-video", 'b', 10, 1, 2, 1)],
        1,
    )
}

fn digest(bytes: &[u8]) -> String {
    storytelling_core::source_evidence::sha256_digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut text, byte| {
            write!(text, "{byte:02x}").unwrap();
            text
        })
}

#[test]
fn story_review_compiles_selected_subset_and_order_and_binds_the_exact_owned_artifact() {
    let mut snap = snapshot();
    let mut second = snap["assets"][0].clone();
    second["id"] = json!("other-video");
    second["contentHash"] = json!("c".repeat(64));
    second["uri"] = json!("synthetic://other.mov");
    snap["assets"].as_array_mut().unwrap().push(second);
    let mut clip = snap["composition"]["tracks"][0]["clips"][0].clone();
    clip["id"] = json!("other-occurrence");
    clip["assetId"] = json!("other-video");
    clip["timelineStart"] = json!(2.5);
    snap["composition"]["tracks"][0]["clips"]
        .as_array_mut()
        .unwrap()
        .push(clip);
    snap["composition"]["duration"] = json!(5);
    let snapshot_bytes = serde_json::to_vec(&snap).unwrap();
    for maximum in [1, 2] {
        let prop = proposal_for(
            vec![json!({"id":"synthetic-video"}), json!({"id":"other-video"})],
            vec![
                candidate("synthetic-video", 'b', 10, 1, 2, 1),
                candidate("other-video", 'c', 10, 2, 4, 2),
            ],
            maximum,
        );
        let review = prepare_story_review(&prop, &snapshot_bytes, &context()).unwrap();
        let exported =
            storytelling_engine::export_story_otio(&prop, &snapshot_bytes, &context()).unwrap();
        assert_eq!(exported.as_slice(), review.compiled_otio());
        let otio: Value = serde_json::from_slice(review.compiled_otio()).unwrap();
        let clips = otio["tracks"]["children"][0]["children"]
            .as_array()
            .unwrap();
        assert_eq!(clips.len(), maximum as usize);
        assert_eq!(
            clips[0]["metadata"]["storytelling"]["asset_id"],
            "other-video"
        );
        if maximum == 2 {
            assert_eq!(
                clips[1]["metadata"]["storytelling"]["asset_id"],
                "synthetic-video"
            );
        }
        assert_eq!(
            review.record().identity.artifact_sha256,
            digest(review.compiled_otio())
        );
        assert_eq!(
            review.record().identity.composition_sha256,
            digest(&snapshot_bytes)
        );
        assert_eq!(
            otio["metadata"]["storytelling"]["proposal_sha256"],
            digest(&prop)
        );
        assert_eq!(
            otio["metadata"]["storytelling"]["selected_proposal_json"],
            std::str::from_utf8(&prop).unwrap()
        );
        assert!(
            check_story_review(review.bundle_json(), &prop, &snapshot_bytes, &context())
                .unwrap()
                .current
        );
    }
}

#[test]
fn story_review_rejects_unknown_duplicate_forged_or_inconsistent_proposals() {
    let snap = serde_json::to_vec(&snapshot()).unwrap();
    let original: Value = serde_json::from_slice(&proposal()).unwrap();
    for pointer in [
        "",
        "/planning",
        "/planning/plan",
        "/planning/plan/assets/0",
        "/planning/plan/shots/0",
        "/planning/selections/0",
        "/planning/brief_audit",
        "/planning/brief_audit/preferences",
    ] {
        let mut changed = original.clone();
        changed
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected".to_owned(), json!(true));
        assert_eq!(
            prepare_story_review(&serde_json::to_vec(&changed).unwrap(), &snap, &context())
                .unwrap_err()
                .code,
            "story_review.invalid_proposal_json"
        );
    }
    let duplicate = format!(
        "{{\"ok\":true,\"ok\":true,\"planning\":{}}}",
        original["planning"]
    );
    assert_eq!(
        prepare_story_review(duplicate.as_bytes(), &snap, &context())
            .unwrap_err()
            .code,
        "story_review.invalid_proposal_json"
    );
    for flag in ["executable", "source_verified", "host_verified"] {
        let mut changed = original.clone();
        changed["planning"][flag] = json!(true);
        assert_eq!(
            prepare_story_review(&serde_json::to_vec(&changed).unwrap(), &snap, &context())
                .unwrap_err()
                .code,
            "story_review.invalid_proposal_claims"
        );
    }
    for pointer in [
        "/planning/selections/0/shot_id",
        "/planning/selections/0/asset_id",
        "/planning/selections/0/duration/numerator",
        "/planning/total_duration/numerator",
        "/planning/content_clock_rate",
    ] {
        let mut changed = original.clone();
        *changed.pointer_mut(pointer).unwrap() = if pointer.ends_with("content_clock_rate") {
            json!("0")
        } else {
            json!("9")
        };
        assert_eq!(
            prepare_story_review(&serde_json::to_vec(&changed).unwrap(), &snap, &context())
                .unwrap_err()
                .code,
            "story_review.selection_mismatch"
        );
    }
}

#[test]
fn story_review_requires_snapshot_asset_identity_duration_and_declared_capabilities() {
    let snap = serde_json::to_vec(&snapshot()).unwrap();
    for (key, value) in [
        ("content_hash", json!("c".repeat(64))),
        ("duration", json!({"numerator":"11","denominator":"1"})),
    ] {
        let mut prop: Value = serde_json::from_slice(&proposal()).unwrap();
        prop["planning"]["plan"]["assets"][0][key] = value;
        assert_eq!(
            prepare_story_review(&serde_json::to_vec(&prop).unwrap(), &snap, &context())
                .unwrap_err()
                .code,
            "story_review.asset_mismatch"
        );
    }
    let missing = proposal_for(
        vec![json!({"id":"missing"})],
        vec![candidate("missing", 'b', 10, 1, 2, 1)],
        1,
    );
    assert_eq!(
        prepare_story_review(&missing, &snap, &context())
            .unwrap_err()
            .code,
        "story_review.asset_mismatch"
    );
    let mut ctx: Value = serde_json::from_slice(&context()).unwrap();
    ctx["host"]["declared_capabilities"] = json!([]);
    assert_eq!(
        prepare_story_review(&proposal(), &snap, &serde_json::to_vec(&ctx).unwrap())
            .unwrap_err()
            .code,
        "review.context_missing_capability"
    );
}

#[test]
fn story_review_preserves_still_hold_and_rejects_still_source_offsets() {
    let snap = include_bytes!("../../../tests/fixtures/openfilm/finite-v1/repeat-source.json");
    let hold = proposal_for(
        vec![json!({"id":"reused-photo"})],
        vec![candidate("reused-photo", 'c', 3, 0, 2, 1)],
        1,
    );
    let review = prepare_story_review(&hold, snap, &context()).unwrap();
    let otio: Value = serde_json::from_slice(review.compiled_otio()).unwrap();
    assert_eq!(
        otio["tracks"]["children"][0]["children"][0]["source_range"]["duration"]["value"],
        2
    );
    let offset = proposal_for(
        vec![json!({"id":"reused-photo"})],
        vec![candidate("reused-photo", 'c', 3, 1, 2, 1)],
        1,
    );
    assert_eq!(
        prepare_story_review(&offset, snap, &context())
            .unwrap_err()
            .code,
        "story_review.still_offset_unsupported"
    );
}

#[test]
fn story_review_preflights_every_size_and_accepts_exact_snapshot_and_bundle_limits() {
    assert_eq!(
        prepare_story_review(&vec![b' '; MAX_STORY_PROPOSAL_BYTES + 1], b"{", b"{")
            .unwrap_err()
            .code,
        "story_review.proposal_too_large"
    );
    assert_eq!(
        prepare_story_review(b"{", &vec![b' '; MAX_INPUT_BYTES + 1], b"{")
            .unwrap_err()
            .code,
        "input.too_large"
    );
    assert_eq!(
        prepare_story_review(b"{", b"{", &vec![b' '; MAX_REVIEW_CONTEXT_BYTES + 1])
            .unwrap_err()
            .code,
        "review.context_too_large"
    );
    assert_eq!(
        check_story_review(&vec![b' '; MAX_STORY_REVIEW_BYTES + 1], b"{", b"{", b"{")
            .unwrap_err()
            .code,
        "story_review.bundle_too_large"
    );
    let mut snap = serde_json::to_vec(&snapshot()).unwrap();
    snap.resize(MAX_INPUT_BYTES, b' ');
    let review = prepare_story_review(&proposal(), &snap, &context()).unwrap();
    let mut bundle = review.bundle_json().to_vec();
    bundle.resize(MAX_STORY_REVIEW_BYTES, b' ');
    assert!(
        check_story_review(&bundle, &proposal(), &snap, &context())
            .unwrap()
            .current
    );
    let mut maximum_proposal = proposal();
    maximum_proposal.resize(MAX_STORY_PROPOSAL_BYTES, b' ');
    assert_eq!(
        prepare_story_review(&maximum_proposal, &snap, &context())
            .unwrap_err()
            .code,
        "otio.output_too_large"
    );
    let mut maximum_context = context();
    maximum_context.resize(MAX_REVIEW_CONTEXT_BYTES, b' ');
    assert!(prepare_story_review(&proposal(), &snap, &maximum_context).is_ok());
    let deep = [vec![b'['; 65], vec![b']'; 65]].concat();
    assert_eq!(
        prepare_story_review(&deep, &snap, &context())
            .unwrap_err()
            .code,
        "review.json_too_deep"
    );
    assert_eq!(
        check_story_review(&deep, &proposal(), &snap, &context())
            .unwrap_err()
            .code,
        "review.json_too_deep"
    );
}

#[test]
fn story_review_rejects_forged_bundle_claims_and_never_accepts_a_composition_only_record() {
    let snap = serde_json::to_vec(&snapshot()).unwrap();
    let bound = prepare_story_review(&proposal(), &snap, &context()).unwrap();
    let original: Value = serde_json::from_slice(bound.bundle_json()).unwrap();
    for flag in [
        "approval_granted",
        "executable",
        "source_verified",
        "host_verified",
    ] {
        let mut bundle = original.clone();
        bundle["review"][flag] = json!(true);
        assert_eq!(
            check_story_review(
                &serde_json::to_vec(&bundle).unwrap(),
                &proposal(),
                &snap,
                &context()
            )
            .unwrap_err()
            .code,
            "review.invalid_claims"
        );
    }
    let mut bundle = original.clone();
    bundle["extra"] = json!(true);
    assert_eq!(
        check_story_review(
            &serde_json::to_vec(&bundle).unwrap(),
            &proposal(),
            &snap,
            &context()
        )
        .unwrap_err()
        .code,
        "story_review.invalid_bundle_json"
    );
    let ctx = storytelling_engine::parse_review_context(&context()).unwrap();
    let old = storytelling_engine::prepare_review(&snap, &ctx).unwrap();
    let fake = json!({"ok":true,"review":old.record(),"otio":serde_json::from_slice::<Value>(old.compiled_otio()).unwrap()});
    assert_eq!(
        check_story_review(
            &serde_json::to_vec(&fake).unwrap(),
            &proposal(),
            &snap,
            &context()
        )
        .unwrap_err()
        .code,
        "story_review.wrong_profile"
    );
}

#[test]
fn story_review_is_stale_when_proposal_bytes_change_even_if_the_selected_plan_does_not() {
    let snap = serde_json::to_vec(&snapshot()).unwrap();
    let mut prop = proposal();
    let bound = prepare_story_review(&prop, &snap, &context()).unwrap();
    prop.push(b'\n');
    let stale = check_story_review(bound.bundle_json(), &prop, &snap, &context()).unwrap();
    assert!(!stale.current);
    assert_eq!(stale.stale_fields, ["artifact"]);
}

#[test]
fn story_review_fixture_has_a_fixed_cross_platform_artifact_and_binding_vector() {
    let input = include_bytes!("../../../tests/fixtures/story-review-v1/planner-request.json");
    let mut prop =
        serde_json::to_vec(&json!({"ok":true,"planning":plan_story(input).unwrap()})).unwrap();
    prop.push(b'\n'); // Exactly the actual plan-story CLI stdout, on all three OSes.
    let snapshot = include_bytes!("../../../tests/fixtures/openfilm/finite-v1/video-trim.json");
    let context = include_bytes!("../../../tests/fixtures/story-review-v1/context.json");
    let bound = prepare_story_review(&prop, snapshot, context).unwrap();
    assert_eq!(
        bound.record().identity.artifact_sha256,
        "5404c0b552ddc4ccc394755c0a9a95e262f47f4a579fdca9b41797b920a838d0"
    );
    assert_eq!(
        bound.record().binding_sha256,
        "ff87f54300ac968089fa6170f1248920a990284c3961f574225088ce85aad932"
    );
    assert!(!format!("{bound:?}").contains("synthetic://"));
}
