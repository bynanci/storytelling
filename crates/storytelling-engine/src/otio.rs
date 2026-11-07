// SPDX-License-Identifier: Apache-2.0
// Origin/modifications: docs/migration/provenance-m1-02.json; see NOTICE.openfilm.
//! Bounded finite OTIO serialization; no media, parser-runtime or host calls.
use serde::Serialize;
use serde_json::Value;
use storytelling_core::compiled_plan::{
    OtioCompileError, OtioCompileReportV1, OtioLossChoicesV1, OtioProfileV1,
};

#[derive(Debug, Serialize)]
pub struct CompiledOtioV1 {
    pub document: Value,
    pub report: OtioCompileReportV1,
}

/// Compile an admitted composition into format-only, non-executable OTIO JSON.
pub fn export_openfilm_otio(
    _bytes: &[u8],
    _profile: &OtioProfileV1,
    _choices: &OtioLossChoicesV1,
) -> Result<CompiledOtioV1, OtioCompileError> {
    Err(OtioCompileError {
        code: "otio.not_implemented".to_owned(),
        path: "$".to_owned(),
        losses: Vec::new(),
    })
}
