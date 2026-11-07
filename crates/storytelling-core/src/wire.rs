//! Untrusted wire DTOs. Deserialization is not domain validation.
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlanDocumentV1 {
    pub schema_version: u32,
    pub plan_id: String,
    pub assets: Vec<DeclaredAsset>,
    pub shots: Vec<DeclaredShot>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredAsset {
    pub id: String,
    pub content_hash: String,
    pub duration: TimeValue,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredShot {
    pub id: String,
    pub asset_id: String,
    pub source_range: SourceRange,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRange {
    pub start: TimeValue,
    pub end: TimeValue,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimeValue {
    pub numerator: String,
    pub denominator: String,
}
