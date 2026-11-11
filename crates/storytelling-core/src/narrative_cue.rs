// SPDX-License-Identifier: MIT
//! Separate, versioned audio cue declarations for closed NarrativeDocumentV1 inputs.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeCueSheetV1 {
    pub schema_version: u32,
    pub cue_sheet_id: String,
    pub cue_sheet_revision: u32,
    pub document_id: String,
    pub document_revision: u32,
    pub variant_id: String,
    pub time_unit: CueTimeUnitV1,
    pub tracks: Vec<CueTrackV1>,
    pub assets: Vec<CueAssetV1>,
    pub cues: Vec<NarrativeCueV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CueTimeUnitV1 {
    Milliseconds,
}

impl CueTimeUnitV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Milliseconds => "milliseconds",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CueTrackV1 {
    pub id: String,
    pub channel: CueTrackChannelV1,
    pub policy: CueTrackPolicyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CueTrackChannelV1 {
    Voice,
    SoundEffect,
    Marker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CueTrackPolicyV1 {
    Exclusive,
    Mixable,
    Marker,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CueAssetV1 {
    pub id: String,
    pub revision: u32,
    pub media_kind: CueAssetKindV1,
    pub relative_path: String,
    pub sha256: String,
    pub provenance: CueAssetProvenanceV1,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CueAssetKindV1 {
    Audio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CueAssetProvenanceV1 {
    Synthetic,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeCueV1 {
    pub id: String,
    pub order: u32,
    pub kind: NarrativeCueKindV1,
    pub role: String,
    pub track_id: String,
    pub source: NarrativeCueSourceV1,
    pub instruction: String,
    pub timing: CueTimingV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeCueKindV1 {
    Narration,
    Silence,
    SoundEffect,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum NarrativeCueSourceV1 {
    #[serde(rename = "narrative_beat")]
    NarrativeBeat {
        variant_id: String,
        beat_id: String,
        unit_id: String,
        revision: u32,
    },
    #[serde(rename = "audio_asset")]
    AudioAsset { asset_id: String, revision: u32 },
    #[serde(rename = "editorial_choice")]
    EditorialChoice { description: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum CueTimingV1 {
    #[serde(rename = "absolute")]
    Absolute {
        start_ms: u64,
        end_ms: u64,
        status: CueIntervalStatusV1,
    },
    #[serde(rename = "after")]
    After {
        cue_id: String,
        duration: CueDurationV1,
    },
    #[serde(rename = "unknown")]
    Unknown { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CueIntervalStatusV1 {
    Known,
    Estimated,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "status", deny_unknown_fields)]
pub enum CueDurationV1 {
    #[serde(rename = "known")]
    Known { value_ms: u64 },
    #[serde(rename = "estimated")]
    Estimated { value_ms: u64 },
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeCueSheetValidationErrorV1 {
    pub code: String,
    pub path: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cue_ids: Vec<String>,
}

impl NarrativeCueSheetValidationErrorV1 {
    pub fn new(code: &str, path: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            path: path.into(),
            cue_ids: Vec::new(),
        }
    }

    pub fn with_cues(mut self, cue_ids: Vec<String>) -> Self {
        self.cue_ids = cue_ids;
        self
    }
}
