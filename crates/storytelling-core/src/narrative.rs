// SPDX-License-Identifier: MIT
//! Versioned, media-independent narrative declarations and their pure projection types.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeDocumentV1 {
    pub schema_version: u32,
    pub document_id: String,
    pub document_revision: u32,
    pub purpose: String,
    pub audience: String,
    pub units: Vec<NarrativeUnitV1>,
    pub relations: Vec<NarrativeRelationV1>,
    pub variants: Vec<NarrativeVariantV1>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeUnitV1 {
    pub id: String,
    pub revision: u32,
    pub origin: NarrativeOriginV1,
    pub text: String,
    pub based_on: Vec<NarrativeUnitReferenceV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeOriginV1 {
    SuppliedObservation,
    AuthorInterpretation,
    CreativeProposal,
}

impl NarrativeOriginV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuppliedObservation => "supplied_observation",
            Self::AuthorInterpretation => "author_interpretation",
            Self::CreativeProposal => "creative_proposal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeUnitReferenceV1 {
    pub unit_id: String,
    pub revision: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum NarrativeRelationV1 {
    #[serde(rename = "association")]
    Association {
        id: String,
        from: NarrativeUnitReferenceV1,
        to: NarrativeUnitReferenceV1,
    },
    #[serde(rename = "must_precede")]
    MustPrecede {
        id: String,
        variant_id: String,
        before: NarrativeUnitReferenceV1,
        after: NarrativeUnitReferenceV1,
    },
}

impl NarrativeRelationV1 {
    pub fn id(&self) -> &str {
        match self {
            Self::Association { id, .. } | Self::MustPrecede { id, .. } => id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum NarrativeVariantV1 {
    #[serde(rename = "spoken_outline")]
    SpokenOutline {
        id: String,
        name: String,
        beats: Vec<NarrativeBeatV1>,
    },
    #[serde(rename = "single_illustration")]
    SingleIllustration {
        id: String,
        name: String,
        unit_refs: Vec<NarrativeUnitReferenceV1>,
        brief: String,
    },
}

impl NarrativeVariantV1 {
    pub fn id(&self) -> &str {
        match self {
            Self::SpokenOutline { id, .. } | Self::SingleIllustration { id, .. } => id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeBeatV1 {
    pub id: String,
    pub unit_ref: NarrativeUnitReferenceV1,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeProjectionV1 {
    pub schema_version: u32,
    pub document_id: String,
    pub document_revision: u32,
    pub purpose: String,
    pub audience: String,
    pub variants: Vec<NarrativeProjectedVariantV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NarrativeProjectedVariantV1 {
    SpokenOutline {
        id: String,
        name: String,
        beats: Vec<NarrativeProjectedBeatV1>,
    },
    SingleIllustration {
        id: String,
        name: String,
        sources: Vec<NarrativeSourceTagV1>,
        brief: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeProjectedBeatV1 {
    pub id: String,
    pub source: NarrativeSourceTagV1,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeSourceTagV1 {
    pub unit_id: String,
    pub revision: u32,
    pub origin: NarrativeOriginV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeValidationErrorV1 {
    pub code: String,
    pub path: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub relation_ids: Vec<String>,
}

/// A style profile is a separate versioned document; it does not extend NarrativeDocumentV1.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeStyleProfileV1 {
    pub schema_version: u32,
    pub profile_id: String,
    pub document_id: String,
    pub document_revision: u32,
    pub rules: Vec<NarrativeStyleRuleV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeStyleRuleKindV1 {
    MustPrecede,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeStyleRuleV1 {
    pub id: String,
    pub kind: NarrativeStyleRuleKindV1,
    pub variant_id: String,
    pub before_beat_id: String,
    pub after_beat_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeStyleValidationErrorV1 {
    pub code: String,
    pub path: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rule_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeStyleCheckV1 {
    pub profile_id: String,
    pub document_id: String,
    pub document_revision: u32,
    pub checked_rules: usize,
    pub scope: &'static str,
}

impl NarrativeValidationErrorV1 {
    pub fn new(code: &str, path: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            path: path.into(),
            relation_ids: Vec::new(),
        }
    }

    pub fn with_relation_ids(
        code: &str,
        path: impl Into<String>,
        relation_ids: Vec<String>,
    ) -> Self {
        Self {
            code: code.to_owned(),
            path: path.into(),
            relation_ids,
        }
    }
}
