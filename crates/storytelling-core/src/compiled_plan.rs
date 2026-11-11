// SPDX-License-Identifier: Apache-2.0
// Origin/modifications: docs/migration/provenance-m1-02.json; see LICENSES/third-party/NOTICE.txt.
//! Explicit finite interchange choices and reports. These do not grant execution authority.
use crate::compatibility::{ClockFacts, ImportError, ImportLoss};
use crate::wire::TimeValue;
use serde::Serialize;

#[derive(Debug, Default)]
pub struct OtioProfileV1;

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct OtioLossChoicesV1 {
    pub picture_only: bool,
    pub metadata_only_locks: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FeatureDisposition {
    Native,
    Baked,
    MetadataOnly,
    Unsupported,
}

#[derive(Debug, Clone, Serialize)]
pub struct OtioFeatureV1 {
    pub feature: String,
    pub path: String,
    pub disposition: FeatureDisposition,
    pub accepted: bool,
}

#[derive(Debug, Serialize)]
pub struct OtioCompileError {
    pub code: String,
    pub path: String,
    pub losses: Vec<OtioFeatureV1>,
}

impl From<ImportError> for OtioCompileError {
    fn from(error: ImportError) -> Self {
        Self {
            code: error.code,
            path: error.path,
            losses: error
                .losses
                .into_iter()
                .map(|loss| OtioFeatureV1 {
                    feature: loss.code,
                    path: loss.path,
                    disposition: FeatureDisposition::Unsupported,
                    accepted: false,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OtioCompileReportV1 {
    pub schema_version: u32,
    pub profile: &'static str,
    pub scope: &'static str,
    pub executable: bool,
    pub source_verified: bool,
    pub host_verified: bool,
    pub host_name: Option<String>,
    pub host_version: Option<String>,
    pub required_capabilities: Vec<String>,
    pub content_clock_rate: String,
    pub timeline_duration: TimeValue,
    pub project_clocks: ClockFacts,
    pub choices: OtioLossChoicesV1,
    pub feature_matrix: Vec<OtioFeatureV1>,
    pub import_limitations: Vec<ImportLoss>,
}
