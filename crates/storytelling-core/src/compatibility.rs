// SPDX-License-Identifier: Apache-2.0
// Source semantics and attribution: docs/migration/provenance-m1-01.json, NOTICE.openfilm.
//! Finite import policy and explicit limitations; no approval or media verification.
use crate::wire::TimeValue;
use serde::{Deserialize, Serialize};

/// The fixed, read-only OpenFilm composition-export profile.
#[derive(Debug, Default)]
pub struct ImportProfileV1;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LossDisposition {
    NotVerified,
    BlocksHostHandoff,
    IntentionalDelta,
    Unsupported,
}

#[derive(Debug, Serialize)]
pub struct ImportLoss {
    pub code: String,
    pub path: String,
    pub disposition: LossDisposition,
}

#[derive(Debug, Serialize)]
pub struct ImportError {
    pub code: String,
    pub path: String,
    pub losses: Vec<ImportLoss>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeclaredTimebaseV1 {
    pub version: u32,
    pub rate_numerator: u32,
    pub rate_denominator: u32,
    pub drop_frame: bool,
}

#[derive(Debug, Serialize)]
pub struct ClockFacts {
    pub declared_frame_rate: String,
    pub exact_frame_rate: TimeValue,
    pub explicit_timebase: Option<DeclaredTimebaseV1>,
}
