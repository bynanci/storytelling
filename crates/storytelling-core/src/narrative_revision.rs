// SPDX-License-Identifier: MIT
//! Append-only source, output and author-choice history for NarrativeDocumentV1.
use serde::{Deserialize, Serialize};

use crate::narrative::{NarrativeDocumentV1, NarrativeOriginV1, NarrativeUnitReferenceV1};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeRevisionLedgerV1 {
    pub schema_version: u32,
    pub base_document: NarrativeDocumentV1,
    pub events: Vec<NarrativeRevisionEventV1>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum NarrativeRevisionEventV1 {
    #[serde(rename = "assistant_suggestion")]
    AssistantSuggestion {
        id: String,
        variant_id: String,
        proposal_ref: NarrativeUnitReferenceV1,
        text: String,
    },
    #[serde(rename = "author_decision")]
    AuthorDecision {
        id: String,
        variant_id: String,
        proposal_ref: NarrativeUnitReferenceV1,
        decision: NarrativeAuthorDecisionV1,
        content: String,
    },
    #[serde(rename = "source_revision")]
    SourceRevision {
        id: String,
        unit_id: String,
        expected_revision: u32,
        text: String,
    },
    #[serde(rename = "variant_review")]
    VariantReview {
        id: String,
        variant_id: String,
        source_refs: Vec<NarrativeUnitReferenceV1>,
        content: String,
    },
}

impl NarrativeRevisionEventV1 {
    pub fn id(&self) -> &str {
        match self {
            Self::AssistantSuggestion { id, .. }
            | Self::AuthorDecision { id, .. }
            | Self::SourceRevision { id, .. }
            | Self::VariantReview { id, .. } => id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeAuthorDecisionV1 {
    Accepted,
    Edited,
    Declined,
}

impl NarrativeAuthorDecisionV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Edited => "edited",
            Self::Declined => "declined",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeRevisionReportV1 {
    pub document_id: String,
    pub events: Vec<NarrativeRevisionEventResultV1>,
    pub source_history: Vec<NarrativeSourceRevisionV1>,
    pub variant_history: Vec<NarrativeVariantSnapshotV1>,
    pub suggestions: Vec<NarrativeSuggestionRecordV1>,
    pub author_decisions: Vec<NarrativeAuthorDecisionRecordV1>,
    pub current_variants: Vec<NarrativeCurrentVariantV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeRevisionEventResultV1 {
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_revision: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_revision: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    pub affected_variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeSourceRevisionV1 {
    pub unit_id: String,
    pub revision: u32,
    pub origin: NarrativeOriginV1,
    pub text: String,
    pub based_on: Vec<NarrativeUnitReferenceV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeVariantSnapshotV1 {
    pub variant_id: String,
    pub revision: u32,
    pub kind: String,
    pub name: String,
    pub source_refs: Vec<NarrativeUnitReferenceV1>,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeSuggestionRecordV1 {
    pub id: String,
    pub variant_id: String,
    pub proposal_ref: NarrativeUnitReferenceV1,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeAuthorDecisionRecordV1 {
    pub id: String,
    pub variant_id: String,
    pub proposal_ref: NarrativeUnitReferenceV1,
    pub decision: NarrativeAuthorDecisionV1,
    pub variant_revision: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeCurrentVariantV1 {
    pub variant_id: String,
    pub revision: u32,
    pub status: String,
    pub stale_source_refs: Vec<NarrativeUnitReferenceV1>,
    pub source_refs: Vec<NarrativeUnitReferenceV1>,
    pub content: String,
}
