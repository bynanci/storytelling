// SPDX-License-Identifier: Apache-2.0
// Source-informed contracts: docs/migration/provenance-m2-01.json; NOTICE.openfilm.
//! Bounded declared StoryBrief inputs and typed audit diagnostics, never a plan.
use crate::wire::TimeValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StoryBriefDocumentV1 {
    pub schema_version: u32,
    pub brief_id: String,
    pub intent: String,
    pub assets: Vec<BriefAssetV1>,
    pub manual_selected_asset_ids: Vec<String>,
    pub hard_constraints: Vec<HardConstraintV1>,
    pub preferences: BriefPreferencesV1,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BriefAssetV1 {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HardConstraintV1 {
    MustInclude { asset_ids: Vec<String> },
    MustExclude { asset_ids: Vec<String> },
    AssetOrder { asset_ids: Vec<String> },
}

fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BriefPreferencesV1 {
    pub preferred_asset_ids: Vec<String>,
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_duration: Option<TimeValue>,
}

#[derive(Debug, Serialize)]
pub struct AssetBindingV1 {
    pub asset_id: String,
    pub origins: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PrecedenceEdgeV1 {
    pub before: String,
    pub after: String,
    pub origins: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BriefConflictKindV1 {
    UnknownAsset,
    RequiredExcluded,
    OrderCycle,
}

#[derive(Debug, Serialize)]
pub struct BriefConflictV1 {
    pub kind: BriefConflictKindV1,
    pub path: String,
    pub asset_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BriefWarningKindV1 {
    UnknownPreferredAsset,
    ExcludedPreference,
}

#[derive(Debug, Serialize)]
pub struct BriefWarningV1 {
    pub kind: BriefWarningKindV1,
    pub path: String,
    pub asset_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BriefAuditV1 {
    pub schema_version: u32,
    pub brief_id: String,
    pub intent: String,
    pub scope: &'static str,
    pub executable: bool,
    pub selection_planned: bool,
    pub duration_analysis: &'static str,
    pub consistent: bool,
    pub manual_selected_asset_ids: Vec<String>,
    pub required_assets: Vec<AssetBindingV1>,
    pub excluded_assets: Vec<AssetBindingV1>,
    pub precedence_edges: Vec<PrecedenceEdgeV1>,
    pub preferences: BriefPreferencesV1,
    pub conflicts: Vec<BriefConflictV1>,
    pub warnings: Vec<BriefWarningV1>,
}
