// SPDX-License-Identifier: MIT
//! Versioned, synchronous read-only consumer surface (ADR 0013).
//!
//! Calls delegate original immutable buffers once to accepted backends. Results
//! own their data; independent calls can use caller-controlled threads. Limits
//! and diagnostic order belong to each backend, with no shared pre-parser,
//! cache, implicit I/O or background work. Bound concurrency and memory at the
//! caller. Dropping a result performs no Host cancellation or rollback.
//!
//! V1 here versions the Rust facade, separately from document/profile/compiler
//! versions and the unpublished crate version. Existing root APIs remain
//! available. Unsupported document versions keep their original diagnostics;
//! there is no implicit upgrade. Breaking this surface needs a new facade major,
//! documented migration and compatibility interval before deprecation/removal.
//!
//! Unresolved coverage is an `Ok` domain result with planning blocked. A stale
//! review is `Ok` with `current=false`. Neither a record nor a bound handle grants
//! human approval, source verification, current authority or Host compatibility.
//! The portable [`ReviewRecordV1`] remains untrusted data when supplied again.
//!
//! Execution/approval/recovery is unavailable, including under all features:
//!
//! ```compile_fail
//! use storytelling_engine::consumer_v1::execute;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::approve;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::dispatch;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::retry;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::recover;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::write_journal;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::run_shell;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v2;
//! ```
//!
//! Prepared reviews cannot be reconstructed from JSON or mutable storage:
//!
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! fn decode<T: serde::de::DeserializeOwned>() {}
//! decode::<PreparedStoryReviewV1>();
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! let forged = PreparedStoryReviewV1 { review: todo!(), bundle: todo!() };
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! fn mutate(review: &mut PreparedStoryReviewV1) {
//!     review.compiled_otio()[0] = 0;
//! }
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! fn mutate(review: &mut PreparedStoryReviewV1) {
//!     review.record().executable = true;
//! }
//! ```
//!
//! SDK errors are opaque Rust values, not new JSON wire contracts:
//!
//! ```compile_fail
//! use storytelling_engine::consumer_v1::SdkErrorV1;
//! let forged = SdkErrorV1 { operation: "video_coverage", source: todo!() };
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::SdkErrorV1;
//! fn decode<T: serde::de::DeserializeOwned>() {}
//! decode::<SdkErrorV1>();
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::SdkEncodingErrorV1;
//! let forged = SdkEncodingErrorV1 { _private: () };
//! ```

pub use crate::BoundStoryReviewV1 as PreparedStoryReviewV1;
pub use crate::ValidationReport as PlanValidationV1;
pub use storytelling_core::brief::{
    AssetBindingV1, BriefAuditV1, BriefConflictKindV1, BriefConflictV1, BriefPreferencesV1,
    BriefWarningKindV1, BriefWarningV1, PrecedenceEdgeV1,
};
pub use storytelling_core::narrative::{
    NarrativeOriginV1, NarrativeProjectedBeatV1, NarrativeProjectedVariantV1,
    NarrativeProjectionV1, NarrativeSourceTagV1, NarrativeUnitReferenceV1,
};
pub use storytelling_core::narrative_revision::{
    NarrativeAuthorDecisionRecordV1, NarrativeAuthorDecisionV1, NarrativeCurrentVariantV1,
    NarrativeRevisionEventResultV1, NarrativeRevisionReportV1, NarrativeSourceRevisionV1,
    NarrativeSuggestionRecordV1, NarrativeVariantSnapshotV1,
};
pub use storytelling_core::narrative_video_coverage::{
    NarrativeVideoCoverageReferenceV1, NarrativeVideoCoverageResultV1,
    VideoAssetCatalogReferenceV1, VideoAssetReferenceV1, VideoCoverageDiagnosticV1,
    VideoCoverageMappingResultV1,
};
pub use storytelling_core::planner::{
    PlannedStoryV1, SelectionDecisionV1, SkippedCandidateV1, SkippedReasonV1, TargetStatusV1,
};
pub use storytelling_core::review::{
    ReviewContextFieldDigestsV1, ReviewFreshnessReportV1, ReviewIdentityV1, ReviewRecordV1,
};
pub use storytelling_core::wire::{
    DeclaredAsset, DeclaredShot, PlanDocumentV1, SourceRange, TimeValue,
};

#[derive(Debug)]
enum SourceError {
    Plan(storytelling_core::ValidationError),
    Narrative(storytelling_core::narrative::NarrativeValidationErrorV1),
    Coverage(storytelling_core::narrative_video_coverage::NarrativeVideoCoverageErrorV1),
    Planning(storytelling_core::planner::PlanningErrorV1),
    Review(storytelling_core::compiled_plan::OtioCompileError),
}

/// Original domain rejection plus its facade operation, with no public factory.
/// Match [`Self::code`] and [`Self::path`], never English text or `Display`.
#[derive(Debug)]
pub struct SdkErrorV1 {
    operation: &'static str,
    source: SourceError,
}

impl SdkErrorV1 {
    /// Exact facade function name, independent of source diagnostic category.
    pub fn operation(&self) -> &str {
        self.operation
    }

    /// Original backend code, including nested profile diagnostics.
    pub fn code(&self) -> &str {
        match &self.source {
            SourceError::Plan(error) => &error.code,
            SourceError::Narrative(error) => &error.code,
            SourceError::Coverage(error) => &error.code,
            SourceError::Planning(error) => &error.code,
            SourceError::Review(error) => &error.code,
        }
    }

    /// Original optional JSON path; a missing coverage path stays `None`.
    pub fn path(&self) -> Option<&str> {
        match &self.source {
            SourceError::Plan(error) => Some(&error.path),
            SourceError::Narrative(error) => Some(&error.path),
            SourceError::Coverage(error) => error.path.as_deref(),
            SourceError::Planning(error) => Some(&error.path),
            SourceError::Review(error) => Some(&error.path),
        }
    }

    /// Serialize the original error directly, without a CLI envelope or value
    /// round trip. Preserve field order, optional omissions, nulls and evidence
    /// arrays. Encoding failure leaves this error and its identifiers intact.
    pub fn source_details_json(&self) -> Result<Vec<u8>, SdkEncodingErrorV1> {
        match &self.source {
            SourceError::Plan(error) => encode_details(error),
            SourceError::Narrative(error) => encode_details(error),
            SourceError::Coverage(error) => encode_details(error),
            SourceError::Planning(error) => encode_details(error),
            SourceError::Review(error) => encode_details(error),
        }
    }
}

impl std::fmt::Display for SdkErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.operation(), self.code())
    }
}

impl std::error::Error for SdkErrorV1 {}

/// Failure to encode error details, separate from the retained domain error.
#[derive(Debug)]
pub struct SdkEncodingErrorV1 {
    _private: (),
}

impl SdkEncodingErrorV1 {
    /// Adapter-only diagnostic; never replaces the underlying domain code.
    pub const fn code(&self) -> &'static str {
        "sdk.error_details_encoding_failed"
    }
}

impl std::fmt::Display for SdkEncodingErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for SdkEncodingErrorV1 {}

fn encode_details(error: &impl serde::Serialize) -> Result<Vec<u8>, SdkEncodingErrorV1> {
    serde_json::to_vec(error).map_err(|_| SdkEncodingErrorV1 { _private: () })
}

/// Validate the existing V1 plan declarations, without checking media bytes.
pub fn validate_plan(bytes: &[u8]) -> Result<PlanValidationV1, SdkErrorV1> {
    crate::validate_plan_json(bytes).map_err(|source| SdkErrorV1 {
        operation: "validate_plan",
        source: SourceError::Plan(source),
    })
}

/// Project declared narrative units into the existing typed variants.
pub fn render_narrative(bytes: &[u8]) -> Result<NarrativeProjectionV1, SdkErrorV1> {
    crate::render_narrative(bytes).map_err(|source| SdkErrorV1 {
        operation: "render_narrative",
        source: SourceError::Narrative(source),
    })
}

/// Replay declared revisions and author choices; this does not authenticate an author.
pub fn track_revisions(bytes: &[u8]) -> Result<NarrativeRevisionReportV1, SdkErrorV1> {
    crate::track_narrative_revisions(bytes).map_err(|source| SdkErrorV1 {
        operation: "track_revisions",
        source: SourceError::Narrative(source),
    })
}

/// Query direct impact, preserving the backend's change-ID-first error order.
pub fn query_impact(
    bytes: &[u8],
    change_id: &str,
) -> Result<NarrativeRevisionEventResultV1, SdkErrorV1> {
    crate::query_narrative_impact(bytes, change_id).map_err(|source| SdkErrorV1 {
        operation: "query_impact",
        source: SourceError::Narrative(source),
    })
}

/// Validate declared coverage; unresolved rows succeed with planning blocked.
pub fn video_coverage(bytes: &[u8]) -> Result<NarrativeVideoCoverageResultV1, SdkErrorV1> {
    crate::render_narrative_video_coverage(bytes).map_err(|source| SdkErrorV1 {
        operation: "video_coverage",
        source: SourceError::Coverage(source),
    })
}

/// Plan the existing fixed windows, retaining full hard-conflict evidence.
pub fn plan_story(bytes: &[u8]) -> Result<PlannedStoryV1, SdkErrorV1> {
    crate::plan_story(bytes).map_err(|source| SdkErrorV1 {
        operation: "plan_story",
        source: SourceError::Planning(source),
    })
}

/// Compile and bind the exact existing planner-response bytes to reviewed OTIO.
/// Inputs are the original proposal response, composition snapshot and context.
/// This returns an immutable compiled result, never execution authority.
pub fn prepare_story_review(
    proposal: &[u8],
    snapshot: &[u8],
    context: &[u8],
) -> Result<PreparedStoryReviewV1, SdkErrorV1> {
    crate::prepare_story_review(proposal, snapshot, context).map_err(|source| SdkErrorV1 {
        operation: "prepare_story_review",
        source: SourceError::Review(source),
    })
}

/// Check the existing bundle against current declarations; stale is `Ok` with
/// `current=false`, while malformed input remains `Err`.
pub fn check_story_review(
    bundle: &[u8],
    proposal: &[u8],
    snapshot: &[u8],
    context: &[u8],
) -> Result<ReviewFreshnessReportV1, SdkErrorV1> {
    crate::check_story_review(bundle, proposal, snapshot, context).map_err(|source| SdkErrorV1 {
        operation: "check_story_review",
        source: SourceError::Review(source),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Serializer};

    // Catches an encoder that returns empty bytes, erases the failure, or leaks
    // serializer prose instead of the fixed adapter-only diagnostic.
    #[test]
    fn details_encoding_failure_returns_opaque_fixed_diagnostic() {
        struct CannotEncode;
        impl Serialize for CannotEncode {
            fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("private serializer cause"))
            }
        }
        let error = encode_details(&CannotEncode).unwrap_err();
        assert_eq!(error.code(), "sdk.error_details_encoding_failed");
        assert_eq!(error.to_string(), "sdk.error_details_encoding_failed");
    }
}
