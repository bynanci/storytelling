// SPDX-License-Identifier: Apache-2.0
// Source-informed finite planner contract; see provenance-m2-02.json and NOTICE.openfilm.
//! Strict declared candidates and finite planning reports; no execution authority.
use crate::brief::{BriefAuditV1, BriefConflictV1};
use crate::wire::{PlanDocumentV1, SourceRange, TimeValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlanningCandidateV1 {
    pub asset_id: String,
    pub content_hash: String,
    pub duration: TimeValue,
    pub source_range: SourceRange,
    pub score: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetStatusV1 {
    NotRequested,
    Met,
    Below,
    Above,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkippedReasonV1 {
    Excluded,
    NotNeeded,
    ShotLimit,
    DurationLimit,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkippedCandidateV1 {
    pub asset_id: String,
    pub path: String,
    pub reason: SkippedReasonV1,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionDecisionV1 {
    pub asset_id: String,
    pub shot_id: String,
    pub required: bool,
    pub declared_score: i32,
    pub timeline_start: TimeValue,
    pub duration: TimeValue,
    pub reason_paths: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlannedStoryV1 {
    pub schema_version: u32,
    pub profile: String,
    pub scope: String,
    pub executable: bool,
    pub source_verified: bool,
    pub host_verified: bool,
    pub selection_planned: bool,
    pub duration_analysis: String,
    pub content_clock_rate: String,
    pub total_duration: TimeValue,
    pub target_status: TargetStatusV1,
    pub brief_audit: BriefAuditV1,
    pub plan: PlanDocumentV1,
    pub selections: Vec<SelectionDecisionV1>,
    pub skipped_candidates: Vec<SkippedCandidateV1>,
}

#[derive(Debug, Serialize)]
pub struct PlanningErrorV1 {
    pub code: String,
    pub path: String,
    pub conflicts: Vec<BriefConflictV1>,
}
