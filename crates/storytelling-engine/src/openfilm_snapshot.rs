// SPDX-License-Identifier: Apache-2.0
// Source semantics and attribution: docs/migration/provenance-m1-01.json, NOTICE.openfilm.
//! Read-only import of a finite declared visual snapshot. No host compatibility claim.
use serde::Serialize;
use serde_json::value::RawValue;
use storytelling_core::compatibility::{ClockFacts, ImportError, ImportLoss, ImportProfileV1};
use storytelling_core::wire::PlanDocumentV1;

#[derive(Debug, Serialize)]
pub struct ImportedPlanV1 {
    pub profile: &'static str,
    pub scope: &'static str,
    pub executable: bool,
    pub plan: PlanDocumentV1,
    pub clocks: ClockFacts,
    pub losses: Vec<ImportLoss>,
    pub source_snapshot: Box<RawValue>,
}

/// Map a frozen export without opening any declared source URI.
pub fn import_openfilm_snapshot(
    _bytes: &[u8],
    _profile: &ImportProfileV1,
) -> Result<ImportedPlanV1, ImportError> {
    Err(ImportError {
        code: "import.not_implemented".to_owned(),
        path: "$".to_owned(),
        losses: Vec::new(),
    })
}
