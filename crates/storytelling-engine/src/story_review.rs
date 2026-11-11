// SPDX-License-Identifier: Apache-2.0
// Source-informed bridge; see provenance-m3-planner-review.json and LICENSES/third-party/NOTICE.txt.
//! Bind a selected fixed-window proposal to an actual read-only OTIO artifact.
//! All source/host/context facts remain caller declarations.

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use storytelling_core::ValidationError;
use storytelling_core::compiled_plan::OtioCompileError;
use storytelling_core::planner::PlannedStoryV1;
use storytelling_core::review::{ReviewFreshnessReportV1, ReviewRecordV1};
use storytelling_core::wire::TimeValue;

use crate::otio::clock::ContentClock;
use crate::review_binding::{
    bind_compiled_review, check_json_depth, identity_differences, loss_choices, sha256_hex,
    validate_record,
};

pub const MAX_STORY_PROPOSAL_BYTES: usize = 4_194_336;
pub const MAX_STORY_REVIEW_BYTES: usize =
    crate::MAX_OTIO_OUTPUT_BYTES + crate::MAX_REVIEW_RECORD_BYTES + 64;
const PROFILE: &str = "storytelling.planner-otio-visual-ticks.v1";

/// Created only after matching the selected plan to the snapshot and compiling.
/// Neither the artifact nor the record can be mutated through this result.
/// Portable bundle bytes are untrusted data, never an approval grant.
///
/// ```compile_fail
/// use storytelling_engine::BoundStoryReviewV1;
/// let forged = BoundStoryReviewV1 { review: todo!(), bundle: todo!() };
/// ```
///
/// ```compile_fail
/// use serde::de::DeserializeOwned;
/// use storytelling_engine::BoundStoryReviewV1;
/// fn untrusted<T: DeserializeOwned>() {}
/// untrusted::<BoundStoryReviewV1>();
/// ```
///
/// ```compile_fail
/// use storytelling_engine::BoundStoryReviewV1;
/// fn mutate(review: &mut BoundStoryReviewV1) { review.compiled_otio()[0] = 0; }
/// ```
pub struct BoundStoryReviewV1 {
    review: crate::BoundReviewV1,
    bundle: Box<[u8]>,
}

impl std::fmt::Debug for BoundStoryReviewV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoundStoryReviewV1")
            .field("record", self.record())
            .field("otio_bytes", &self.compiled_otio().len())
            .finish_non_exhaustive()
    }
}

impl BoundStoryReviewV1 {
    pub fn record(&self) -> &ReviewRecordV1 {
        self.review.record()
    }
    pub fn compiled_otio(&self) -> &[u8] {
        self.review.compiled_otio()
    }
    pub fn bundle_json(&self) -> &[u8] {
        &self.bundle
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Proposal {
    ok: bool,
    planning: PlannedStoryV1,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IncomingBundle {
    ok: bool,
    review: ReviewRecordV1,
    otio: Box<RawValue>,
}

#[derive(Serialize)]
struct OutgoingBundle<'a> {
    ok: bool,
    review: &'a ReviewRecordV1,
    otio: &'a RawValue,
}

#[derive(Deserialize)]
struct SnapshotMedia {
    assets: Vec<Media>,
}
#[derive(Deserialize)]
struct Media {
    id: String,
    #[serde(rename = "mediaType")]
    media_type: String,
}

fn error(code: &str, path: &str) -> OtioCompileError {
    OtioCompileError {
        code: code.to_owned(),
        path: path.to_owned(),
        losses: Vec::new(),
    }
}

fn validation(cause: ValidationError) -> OtioCompileError {
    OtioCompileError {
        code: cause.code,
        path: cause.path,
        losses: Vec::new(),
    }
}

fn check_sizes(proposal: &[u8], snapshot: &[u8], context: &[u8]) -> Result<(), OtioCompileError> {
    if proposal.len() > MAX_STORY_PROPOSAL_BYTES {
        return Err(error("story_review.proposal_too_large", "$"));
    }
    if snapshot.len() > crate::MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    if context.len() > crate::MAX_REVIEW_CONTEXT_BYTES {
        return Err(error("review.context_too_large", "$"));
    }
    Ok(())
}

fn same_time(left: &TimeValue, right: &TimeValue) -> bool {
    left.numerator == right.numerator && left.denominator == right.denominator
}

fn validate_proposal(proposal: &Proposal) -> Result<Vec<u8>, OtioCompileError> {
    let p = &proposal.planning;
    let audit = &p.brief_audit;
    if !proposal.ok
        || p.schema_version != 1
        || p.profile != "storytelling.fixed-window-planner.v1"
        || p.scope != "declared-fixed-window-plan-only"
        || p.executable
        || p.source_verified
        || p.host_verified
        || !p.selection_planned
        || p.duration_analysis != "exact-fixed-windows"
        || audit.schema_version != 1
        || audit.scope != "declared-constraint-audit-only"
        || audit.executable
        || audit.selection_planned
        || !audit.consistent
        || audit.duration_analysis != "not-performed"
        || !audit.conflicts.is_empty()
        || audit.brief_id != p.plan.plan_id
    {
        return Err(error("story_review.invalid_proposal_claims", "$.planning"));
    }
    let bytes =
        serde_json::to_vec(&p.plan).map_err(|_| error("review.serialization_failed", "$.plan"))?;
    crate::validate_plan_json(&bytes).map_err(validation)?;
    if p.plan.shots.len() > 1024 || p.selections.len() != p.plan.shots.len() {
        return Err(error(
            "story_review.selection_mismatch",
            "$.planning.selections",
        ));
    }
    let clock = ContentClock::for_plan(&p.plan)?;
    let mut cursor = 0;
    for (shot, selection) in p.plan.shots.iter().zip(&p.selections) {
        let start = clock.ticks(&shot.source_range.start, "$.planning.plan.shots")?;
        let end = clock.ticks(&shot.source_range.end, "$.planning.plan.shots")?;
        if shot.id != selection.shot_id
            || shot.asset_id != selection.asset_id
            || !same_time(&selection.timeline_start, &clock.seconds(cursor))
            || !same_time(&selection.duration, &clock.seconds(end - start))
        {
            return Err(error(
                "story_review.selection_mismatch",
                "$.planning.selections",
            ));
        }
        cursor = clock.add_timeline_ticks(cursor, end - start)?;
    }
    if !same_time(&p.total_duration, &clock.seconds(cursor)) {
        return Err(error(
            "story_review.selection_mismatch",
            "$.planning.total_duration",
        ));
    }
    let rate = p.content_clock_rate.parse::<u64>().ok().filter(|rate| {
        *rate > 0 && rate.to_string() == p.content_clock_rate && rate % clock.rate() == 0
    });
    if rate.is_none() {
        return Err(error(
            "story_review.selection_mismatch",
            "$.planning.content_clock_rate",
        ));
    }
    Ok(bytes)
}

/// Compile the selected proposal using only facts from the same supplied snapshot.
/// No source URI, Catalog, project, signing key or editor is opened.
pub fn prepare_story_review(
    proposal_bytes: &[u8],
    snapshot_bytes: &[u8],
    context_bytes: &[u8],
) -> Result<BoundStoryReviewV1, OtioCompileError> {
    let review = compile_story_review(proposal_bytes, snapshot_bytes, context_bytes)?;
    let artifact_text = std::str::from_utf8(review.compiled_otio())
        .map_err(|_| error("review.serialization_failed", "$.otio"))?;
    let artifact = RawValue::from_string(artifact_text.to_owned())
        .map_err(|_| error("review.serialization_failed", "$.otio"))?;
    let bundle = serde_json::to_vec(&OutgoingBundle {
        ok: true,
        review: review.record(),
        otio: &artifact,
    })
    .map_err(|_| error("review.serialization_failed", "$"))?;
    if bundle.len() > MAX_STORY_REVIEW_BYTES {
        return Err(error("story_review.bundle_too_large", "$"));
    }
    Ok(BoundStoryReviewV1 {
        review,
        bundle: bundle.into_boxed_slice(),
    })
}

/// Compile and bind the selected proposal, returning only the native OTIO bytes.
/// This path avoids constructing the portable review bundle when the caller only
/// needs the interchange artifact.
pub fn export_story_otio(
    proposal_bytes: &[u8],
    snapshot_bytes: &[u8],
    context_bytes: &[u8],
) -> Result<Vec<u8>, OtioCompileError> {
    let review = compile_story_review(proposal_bytes, snapshot_bytes, context_bytes)?;
    Ok(review.compiled_otio().to_vec())
}

fn compile_story_review(
    proposal_bytes: &[u8],
    snapshot_bytes: &[u8],
    context_bytes: &[u8],
) -> Result<crate::BoundReviewV1, OtioCompileError> {
    check_sizes(proposal_bytes, snapshot_bytes, context_bytes)?;
    compile_story_review_after_size_check(proposal_bytes, snapshot_bytes, context_bytes)
}

fn compile_story_review_after_size_check(
    proposal_bytes: &[u8],
    snapshot_bytes: &[u8],
    context_bytes: &[u8],
) -> Result<crate::BoundReviewV1, OtioCompileError> {
    let context = crate::parse_review_context(context_bytes).map_err(validation)?;
    check_json_depth(proposal_bytes).map_err(validation)?;
    let proposal: Proposal = serde_json::from_slice(proposal_bytes)
        .map_err(|_| error("story_review.invalid_proposal_json", "$"))?;
    let plan_bytes = validate_proposal(&proposal)?;
    let mut imported = crate::import_openfilm_snapshot(snapshot_bytes, &Default::default())?;
    let available: BTreeMap<_, _> = imported
        .plan
        .assets
        .iter()
        .map(|a| (a.id.as_str(), a))
        .collect();
    for (index, asset) in proposal.planning.plan.assets.iter().enumerate() {
        let matched = available.get(asset.id.as_str()).is_some_and(|source| {
            source.content_hash == asset.content_hash
                && same_time(&source.duration, &asset.duration)
        });
        if !matched {
            return Err(error(
                "story_review.asset_mismatch",
                &format!("$.planning.plan.assets[{index}]"),
            ));
        }
    }
    // Projection is only from the importer-validated snapshot, never a second admission path.
    let facts: SnapshotMedia = serde_json::from_str(imported.source_snapshot.get())
        .map_err(|_| error("import.invalid_snapshot", "$"))?;
    let kinds: BTreeMap<_, _> = facts
        .assets
        .iter()
        .map(|a| (a.id.as_str(), a.media_type.as_str()))
        .collect();
    for (index, shot) in proposal.planning.plan.shots.iter().enumerate() {
        if kinds.get(shot.asset_id.as_str()) == Some(&"image")
            && shot.source_range.start.numerator != "0"
        {
            return Err(error(
                "story_review.still_offset_unsupported",
                &format!("$.planning.plan.shots[{index}].source_range"),
            ));
        }
    }
    imported.plan = proposal.planning.plan;
    let proposal_text = std::str::from_utf8(proposal_bytes)
        .map_err(|_| error("story_review.invalid_proposal_json", "$"))?;
    let compiled =
        crate::otio::compile_imported_otio(imported, &loss_choices(&context), Some(proposal_text))?;
    bind_compiled_review(snapshot_bytes, &plan_bytes, compiled, &context).map_err(validation)
}

/// Check the actual artifact in an untrusted bundle, then recompile current inputs.
/// A true result means declaration/artifact consistency, never human acceptance.
pub fn check_story_review(
    bundle_bytes: &[u8],
    proposal_bytes: &[u8],
    snapshot_bytes: &[u8],
    context_bytes: &[u8],
) -> Result<ReviewFreshnessReportV1, OtioCompileError> {
    if bundle_bytes.len() > MAX_STORY_REVIEW_BYTES {
        return Err(error("story_review.bundle_too_large", "$"));
    }
    check_sizes(proposal_bytes, snapshot_bytes, context_bytes)?;
    check_json_depth(bundle_bytes).map_err(validation)?;
    let bundle: IncomingBundle = serde_json::from_slice(bundle_bytes)
        .map_err(|_| error("story_review.invalid_bundle_json", "$"))?;
    validate_record(&bundle.review).map_err(validation)?;
    if !bundle.ok || bundle.review.identity.profile_id != PROFILE {
        return Err(error("story_review.wrong_profile", "$.review"));
    }
    if sha256_hex(bundle.otio.get().as_bytes()) != bundle.review.identity.artifact_sha256 {
        return Err(error("story_review.artifact_digest_mismatch", "$.otio"));
    }
    let current =
        compile_story_review_after_size_check(proposal_bytes, snapshot_bytes, context_bytes)?;
    let mut stale_fields =
        identity_differences(&bundle.review.identity, &current.record().identity);
    stale_fields.sort();
    stale_fields.dedup();
    Ok(ReviewFreshnessReportV1 {
        schema_version: 1,
        scope: "declared-planner-otio-review-only",
        current: stale_fields.is_empty(),
        stale_fields,
        executable: false,
        approval_granted: false,
        source_verified: false,
        host_verified: false,
    })
}
