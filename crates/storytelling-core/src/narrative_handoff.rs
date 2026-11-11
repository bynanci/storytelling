// SPDX-License-Identifier: MIT
//! Separately versioned, declarative mixed-media handoff inventory.
use serde::{Deserialize, Serialize};

use crate::narrative_revision::NarrativeRevisionLedgerV1;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeHandoffManifestV1 {
    pub schema_version: u32,
    pub manifest_id: String,
    pub ledger: NarrativeRevisionLedgerV1,
    pub change_id: String,
    pub sources: Vec<HandoffSourceV1>,
    pub artifacts: Vec<HandoffArtifactV1>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffSourceV1 {
    pub unit_id: String,
    pub revision: u32,
    pub representation: HandoffRepresentationV1,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffMediumV1 {
    Text,
    Image,
    Audio,
}

impl HandoffMediumV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Audio => "audio",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum HandoffRepresentationV1 {
    File {
        media_kind: HandoffMediumV1,
        relative_path: String,
        sha256: String,
    },
    Description {
        media_kind: HandoffMediumV1,
        text: String,
    },
    Transcript {
        text: String,
    },
    CreativeProposal {
        variant_id: String,
        text: String,
    },
    Missing {
        media_kind: HandoffMediumV1,
        reason: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffArtifactV1 {
    pub id: String,
    pub name: String,
    pub medium: HandoffMediumV1,
    pub variant_id: String,
    pub components: Vec<HandoffComponentV1>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffComponentV1 {
    pub id: String,
    pub unit_ids: Vec<String>,
}
