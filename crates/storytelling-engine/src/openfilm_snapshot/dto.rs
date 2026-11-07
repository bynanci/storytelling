// SPDX-License-Identifier: Apache-2.0
// Pinned source contracts: docs/migration/provenance-m1-01.json; see NOTICE.openfilm.
//! Strict input DTOs. Preserved editing facts are retained in the raw source snapshot.
use serde::Deserialize;
use serde_json::value::RawValue;
use storytelling_core::compatibility::DeclaredTimebaseV1;

// Absence is optional; a present null is never silently erased into absence.
fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct VersionProbe {
    pub schema_version: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Envelope {
    #[serde(rename = "schemaVersion")]
    pub _schema_version: String,
    pub composition: Box<RawValue>,
    pub assets: Vec<Box<RawValue>>,
    pub settings: Box<RawValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Composition {
    pub id: String,
    pub story_id: String,
    pub duration: Box<RawValue>,
    pub tracks: Vec<Box<RawValue>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Track {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub clips: Vec<Box<RawValue>>,
    #[serde(rename = "locked")]
    #[serde(default, deserialize_with = "present")]
    pub _locked: Option<bool>,
    #[serde(rename = "target")]
    #[serde(default, deserialize_with = "present")]
    pub _target: Option<bool>,
    #[serde(default, deserialize_with = "present")]
    pub visible: Option<bool>,
    #[serde(default, deserialize_with = "present")]
    pub muted: Option<bool>,
    #[serde(default, deserialize_with = "present")]
    pub solo: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Clip {
    pub id: String,
    pub asset_id: String,
    #[serde(default, deserialize_with = "present")]
    pub beat_id: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub source_in: Option<Box<RawValue>>,
    #[serde(default, deserialize_with = "present")]
    pub source_out: Option<Box<RawValue>>,
    pub timeline_start: Box<RawValue>,
    pub timeline_duration: Box<RawValue>,
    #[serde(rename = "locked")]
    #[serde(default, deserialize_with = "present")]
    pub _locked: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Asset {
    pub id: String,
    pub uri: String,
    pub media_type: String,
    #[serde(rename = "name")]
    pub _name: String,
    #[serde(rename = "tags")]
    pub _tags: Vec<String>,
    #[serde(rename = "state")]
    pub _state: AssetState,
    pub metadata: Box<RawValue>,
    pub content_hash: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub duration: Option<Box<RawValue>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AssetState {
    #[serde(rename = "favorite")]
    #[serde(default, deserialize_with = "present")]
    pub _favorite: Option<bool>,
    #[serde(rename = "rejected")]
    #[serde(default, deserialize_with = "present")]
    pub _rejected: Option<bool>,
    #[serde(rename = "locked")]
    #[serde(default, deserialize_with = "present")]
    pub _locked: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Settings {
    pub width: u32,
    pub height: u32,
    pub frame_rate: Box<RawValue>,
    #[serde(default, deserialize_with = "present")]
    pub timebase: Option<DeclaredTimebaseV1>,
}
