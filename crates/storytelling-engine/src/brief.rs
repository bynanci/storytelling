// SPDX-License-Identifier: Apache-2.0
// Source-informed behavior: docs/migration/provenance-m2-01.json; NOTICE.openfilm.
//! Pure bounded audit of declared brief constraints; no selection or feasibility proof.
use storytelling_core::ValidationError;
use storytelling_core::brief::BriefAuditV1;

pub fn audit_story_brief(_bytes: &[u8]) -> Result<BriefAuditV1, ValidationError> {
    Err(ValidationError::new("brief.not_implemented", "$"))
}
