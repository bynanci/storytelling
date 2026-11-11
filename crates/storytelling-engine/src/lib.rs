//! Bounded, deterministic validation and fixed-window planning of declared metadata.
//! Source evidence is a re-export of pure supplied-buffer verification;
//! no filesystem, media probing or editor execution occurs here.
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use storytelling_core::ValidationError;
use storytelling_core::time::RationalTime;
use storytelling_core::wire::{PlanDocumentV1, TimeValue};

mod brief;
pub use brief::audit_story_brief;

mod openfilm_snapshot;
pub use openfilm_snapshot::{ImportedPlanV1, import_openfilm_snapshot};
pub use storytelling_core::source_evidence::{
    MAX_VERIFIED_BUFFER_BYTES, VerifiedBufferV1, verify_supplied_sha256,
};

mod otio;
pub use otio::{CompiledOtioV1, MAX_OTIO_OUTPUT_BYTES, export_openfilm_otio};
pub use storytelling_core::compiled_plan::{OtioLossChoicesV1, OtioProfileV1};

mod review_binding;
pub use review_binding::{
    BoundReviewV1, MAX_REVIEW_CONTEXT_BYTES, MAX_REVIEW_RECORD_BYTES, check_review,
    parse_review_context, prepare_review,
};

pub const MAX_INPUT_BYTES: usize = 1_048_576;
pub const MAX_ASSETS: usize = 1024;
pub const MAX_SHOTS: usize = 4096;

#[derive(Debug, Serialize)]
pub struct ValidationReport {
    pub schema_version: u32,
    pub plan_id: String,
    pub assets: usize,
    pub shots: usize,
    pub scope: &'static str,
}

fn identifier(value: &str, path: String) -> Result<(), ValidationError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 128
        || !bytes[0].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(byte))
    {
        return Err(ValidationError::new("plan.invalid_id", path));
    }
    Ok(())
}

fn time(value: &TimeValue, path: String) -> Result<RationalTime, ValidationError> {
    RationalTime::from_strings(&value.numerator, &value.denominator)
        .map_err(|_| ValidationError::new("plan.invalid_time", path))
}

/// Checks syntax, identities, declared source bounds and rational time invariants.
/// A successful report explicitly does NOT certify source bytes or host support.
pub fn validate_plan_json(input: &[u8]) -> Result<ValidationReport, ValidationError> {
    if input.len() > MAX_INPUT_BYTES {
        return Err(ValidationError::new("input.too_large", "$"));
    }
    let plan: PlanDocumentV1 = serde_json::from_slice(input)
        .map_err(|_| ValidationError::new("input.invalid_json", "$"))?;
    if plan.schema_version != 1 {
        return Err(ValidationError::new(
            "plan.unsupported_version",
            "$.schema_version",
        ));
    }
    identifier(&plan.plan_id, "$.plan_id".to_owned())?;
    if plan.assets.is_empty() || plan.shots.is_empty() {
        return Err(ValidationError::new("plan.empty_collection", "$"));
    }
    if plan.assets.len() > MAX_ASSETS || plan.shots.len() > MAX_SHOTS {
        return Err(ValidationError::new("plan.too_many_items", "$"));
    }
    let mut assets = BTreeMap::new();
    for (index, asset) in plan.assets.iter().enumerate() {
        let path = format!("$.assets[{index}]");
        identifier(&asset.id, format!("{path}.id"))?;
        if asset.content_hash.len() != 64
            || !asset
                .content_hash
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(ValidationError::new(
                "plan.invalid_hash",
                format!("{path}.content_hash"),
            ));
        }
        let duration = time(&asset.duration, format!("{path}.duration"))?;
        if duration.is_zero() {
            return Err(ValidationError::new(
                "plan.invalid_range",
                format!("{path}.duration"),
            ));
        }
        if assets.insert(asset.id.as_str(), duration).is_some() {
            return Err(ValidationError::new(
                "plan.duplicate_id",
                format!("{path}.id"),
            ));
        }
    }
    let mut shot_ids = BTreeSet::new();
    for (index, shot) in plan.shots.iter().enumerate() {
        let path = format!("$.shots[{index}]");
        identifier(&shot.id, format!("{path}.id"))?;
        identifier(&shot.asset_id, format!("{path}.asset_id"))?;
        if !shot_ids.insert(shot.id.as_str()) {
            return Err(ValidationError::new(
                "plan.duplicate_id",
                format!("{path}.id"),
            ));
        }
        let duration = assets.get(shot.asset_id.as_str()).ok_or_else(|| {
            ValidationError::new("plan.unknown_asset", format!("{path}.asset_id"))
        })?;
        let start = time(
            &shot.source_range.start,
            format!("{path}.source_range.start"),
        )?;
        let end = time(&shot.source_range.end, format!("{path}.source_range.end"))?;
        if start >= end {
            return Err(ValidationError::new(
                "plan.invalid_range",
                format!("{path}.source_range"),
            ));
        }
        if end > *duration {
            return Err(ValidationError::new(
                "plan.source_out_of_bounds",
                format!("{path}.source_range.end"),
            ));
        }
    }
    Ok(ValidationReport {
        schema_version: plan.schema_version,
        plan_id: plan.plan_id,
        assets: plan.assets.len(),
        shots: plan.shots.len(),
        scope: "declared-metadata-only",
    })
}

mod planner;
pub use planner::plan_story;

mod story_review;
pub use story_review::{
    BoundStoryReviewV1, MAX_STORY_PROPOSAL_BYTES, MAX_STORY_REVIEW_BYTES, check_story_review,
    prepare_story_review,
};
