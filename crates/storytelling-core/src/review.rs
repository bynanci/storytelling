// SPDX-License-Identifier: Apache-2.0
//! Strict wire contracts for finite review identity and declared context.
//! These DTOs carry declarations; they do not represent approval authority.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewLossChoiceV1 {
    MetadataOnlyLocks,
    PictureOnly,
}

impl ReviewLossChoiceV1 {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MetadataOnlyLocks => "metadata_only_locks",
            Self::PictureOnly => "picture_only",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewHostContextV1 {
    pub id: String,
    pub version: String,
    pub profile_id: String,
    pub binary_sha256: String,
    pub declared_capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewContextV1 {
    pub schema_version: u32,
    pub project_id: String,
    pub target_revision: String,
    pub host: ReviewHostContextV1,
    pub loss_choices: Vec<ReviewLossChoiceV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewContextFieldDigestsV1 {
    pub project_id: String,
    pub target_revision: String,
    pub host_id: String,
    pub host_version: String,
    pub host_profile_id: String,
    pub host_binary_sha256: String,
    pub host_declared_capabilities: String,
    pub loss_choices: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewIdentityV1 {
    pub composition_sha256: String,
    pub plan_sha256: String,
    pub artifact_sha256: String,
    pub report_sha256: String,
    pub compiler_version: String,
    pub profile_id: String,
    pub context_sha256: String,
    pub context_fields: ReviewContextFieldDigestsV1,
}

/// Portable JSON DTO. It is untrusted data and is not an approval token.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewRecordV1 {
    pub schema_version: u32,
    pub scope: String,
    pub binding_sha256: String,
    pub identity: ReviewIdentityV1,
    pub executable: bool,
    pub approval_granted: bool,
    pub source_verified: bool,
    pub host_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReviewFreshnessReportV1 {
    pub schema_version: u32,
    pub scope: &'static str,
    pub current: bool,
    pub stale_fields: Vec<String>,
    pub executable: bool,
    pub approval_granted: bool,
    pub source_verified: bool,
    pub host_verified: bool,
}
