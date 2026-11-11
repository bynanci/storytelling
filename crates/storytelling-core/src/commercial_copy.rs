// SPDX-License-Identifier: MIT
//! Versioned declarations for source-referenced commercial copy.
use serde::{Deserialize, Serialize};

use crate::narrative::NarrativeOriginV1;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommercialCopyDocumentV1 {
    pub schema_version: u32,
    pub copy_id: String,
    pub copy_revision: u32,
    pub evidence_document_id: String,
    pub purpose: String,
    pub audience: String,
    pub claims: Vec<CommercialCopyClaimV1>,
    pub call_to_action: CommercialCopyClaimV1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommercialCopyClaimV1 {
    pub id: String,
    pub kind: CommercialCopyKindV1,
    pub wording: String,
    pub evidence_refs: Vec<CommercialCopyEvidenceReferenceV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommercialCopyKindV1 {
    FactualClaim,
    CreativeProposal,
}

impl CommercialCopyKindV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FactualClaim => "factual_claim",
            Self::CreativeProposal => "creative_proposal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommercialCopyEvidenceReferenceV1 {
    pub unit_id: String,
    pub revision: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommercialCopyReportV1 {
    pub schema_version: u32,
    pub copy_id: String,
    pub copy_revision: u32,
    pub evidence_document_id: String,
    pub purpose: String,
    pub audience: String,
    pub claims: Vec<CommercialCopyClaimReportV1>,
    pub call_to_action: CommercialCopyClaimReportV1,
    pub reference_scope: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommercialCopyClaimReportV1 {
    pub id: String,
    pub kind: String,
    pub wording: String,
    pub reference_status: String,
    pub evidence_refs: Vec<CommercialCopyReferenceReportV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommercialCopyReferenceReportV1 {
    pub unit_id: String,
    pub revision: u32,
    pub current_revision: Option<u32>,
    pub cited_source_origin: Option<NarrativeOriginV1>,
    pub cited_source_text: Option<String>,
    pub reference_status: String,
}

// Opt-in report V2; the accepted V1 declarations above remain unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommercialCopyReportV2 {
    pub schema_version: u32,
    pub copy_id: String,
    pub copy_revision: u32,
    pub evidence_document_id: String,
    pub purpose: String,
    pub audience: String,
    pub claims: Vec<CommercialCopyClaimReportV2>,
    pub call_to_action: CommercialCopyClaimReportV2,
    pub reference_attention_claim_ids: Vec<String>,
    pub reference_scope: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommercialCopyClaimReportV2 {
    pub id: String,
    pub kind: String,
    pub wording: String,
    pub reference_status: String,
    pub support_status: String,
    pub evidence_refs: Vec<CommercialCopyReferenceReportV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommercialCopyReferenceReportV2 {
    pub unit_id: String,
    pub revision: u32,
    pub current_revision: Option<u32>,
    pub cited_source_origin: Option<NarrativeOriginV1>,
    pub cited_source_text: Option<String>,
    pub current_source_text: Option<String>,
    pub reference_status: String,
}
