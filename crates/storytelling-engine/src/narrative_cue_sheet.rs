// SPDX-License-Identifier: MIT
//! Pure validation and deterministic projection for a versioned audio cue sheet.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Number, Value};
use storytelling_core::narrative::{NarrativeProjectedVariantV1, NarrativeValidationErrorV1};
use storytelling_core::narrative_cue::{
    CueAssetV1, CueDurationV1, CueIntervalStatusV1, CueTimingV1, CueTrackChannelV1,
    CueTrackPolicyV1, NarrativeCueKindV1, NarrativeCueSheetV1, NarrativeCueSheetValidationErrorV1,
    NarrativeCueSourceV1,
};

use crate::MAX_INPUT_BYTES;
use crate::narrative::preflight_json;
use crate::{NarrativeProjectionV1, render_narrative};

const MAX_TRACKS: usize = 32;
const MAX_ASSETS: usize = 128;
const MAX_CUES: usize = 512;
const MAX_ID_BYTES: usize = 128;
const MAX_TEXT_CHARS: usize = 8192;
const MAX_DURATION_MS: u64 = 3_600_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CueSheetTrackProjectionV1 {
    pub id: String,
    pub channel: &'static str,
    pub policy: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeCueProjectionV1 {
    pub order: u32,
    pub id: String,
    pub kind: &'static str,
    pub role: String,
    pub track_id: String,
    pub source: String,
    pub instruction: String,
    pub spoken_text: Option<String>,
    pub timing: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeCueSheetProjectionV1 {
    pub cue_sheet_id: String,
    pub cue_sheet_revision: u32,
    pub document_id: String,
    pub document_revision: u32,
    pub variant_id: String,
    pub time_unit: &'static str,
    pub tracks: Vec<CueSheetTrackProjectionV1>,
    pub cues: Vec<NarrativeCueProjectionV1>,
    pub resolved_intervals: usize,
    pub unresolved_intervals: usize,
}

#[derive(Debug, Clone, Copy)]
struct PartialInterval {
    start_ms: Option<u64>,
    end_ms: Option<u64>,
}

impl PartialInterval {
    fn unresolved() -> Self {
        Self {
            start_ms: None,
            end_ms: None,
        }
    }

    fn is_resolved(self) -> bool {
        self.start_ms.is_some() && self.end_ms.is_some()
    }

    fn overlaps(self, other: Self) -> bool {
        let (Some(left_start), Some(left_end), Some(right_start), Some(right_end)) =
            (self.start_ms, self.end_ms, other.start_ms, other.end_ms)
        else {
            return false;
        };
        left_start < right_end && right_start < left_end
    }
}

fn error(code: &str, path: impl Into<String>) -> NarrativeCueSheetValidationErrorV1 {
    NarrativeCueSheetValidationErrorV1::new(code, path)
}

fn map_narrative_error(cause: NarrativeValidationErrorV1) -> NarrativeCueSheetValidationErrorV1 {
    error(&cause.code, cause.path)
}

fn cue_id(value: &str, path: &str) -> Result<(), NarrativeCueSheetValidationErrorV1> {
    let bytes = value.as_bytes();
    if bytes.len() > MAX_ID_BYTES {
        return Err(error("narrative.cue.too_many_items", path));
    }
    if bytes.is_empty()
        || !bytes[0].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(byte))
    {
        return Err(error("narrative.cue.invalid_shape", path));
    }
    Ok(())
}

fn text(value: &str, path: &str) -> Result<(), NarrativeCueSheetValidationErrorV1> {
    if value.trim().is_empty() {
        return Err(error("narrative.cue.invalid_shape", path));
    }
    if value.chars().count() > MAX_TEXT_CHARS {
        return Err(error("narrative.cue.too_many_items", path));
    }
    Ok(())
}

fn integer_value(number: &Number) -> Option<u64> {
    number.as_u64()
}

fn check_version(bytes: &[u8]) -> Result<(), NarrativeCueSheetValidationErrorV1> {
    let root: Value =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    let Some(number) = root.get("schema_version").and_then(Value::as_number) else {
        return Err(error("narrative.cue.invalid_shape", "$.schema_version"));
    };
    match integer_value(number) {
        Some(1) => Ok(()),
        Some(_) => Err(error(
            "narrative.cue.unsupported_version",
            "$.schema_version",
        )),
        None => Err(error("narrative.cue.invalid_shape", "$.schema_version")),
    }
}

fn map_preflight_error(cause: NarrativeValidationErrorV1) -> NarrativeCueSheetValidationErrorV1 {
    match cause.code.as_str() {
        "narrative.duplicate_field" => error("narrative.cue.duplicate_field", cause.path),
        "narrative.depth_exceeded" => error("narrative.cue.depth_exceeded", cause.path),
        _ => error(&cause.code, cause.path),
    }
}

fn parse_sheet(bytes: &[u8]) -> Result<NarrativeCueSheetV1, NarrativeCueSheetValidationErrorV1> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    preflight_json(bytes).map_err(map_preflight_error)?;
    check_version(bytes)?;
    serde_json::from_slice(bytes).map_err(|cause| {
        let message = cause.to_string();
        if message.starts_with("unknown field") {
            error("narrative.cue.unknown_field", "$")
        } else if message.starts_with("duplicate field") {
            error("narrative.cue.duplicate_field", "$")
        } else {
            error("narrative.cue.invalid_shape", "$")
        }
    })
}

fn track_channel(channel: CueTrackChannelV1) -> &'static str {
    match channel {
        CueTrackChannelV1::Voice => "voice",
        CueTrackChannelV1::SoundEffect => "sound_effect",
        CueTrackChannelV1::Marker => "marker",
    }
}

fn track_policy(policy: CueTrackPolicyV1) -> &'static str {
    match policy {
        CueTrackPolicyV1::Exclusive => "exclusive",
        CueTrackPolicyV1::Mixable => "mixable",
        CueTrackPolicyV1::Marker => "marker",
    }
}

fn cue_kind(kind: NarrativeCueKindV1) -> &'static str {
    match kind {
        NarrativeCueKindV1::Narration => "narration",
        NarrativeCueKindV1::Silence => "silence",
        NarrativeCueKindV1::SoundEffect => "sound_effect",
    }
}

fn duration_value(duration: &CueDurationV1) -> (Option<u64>, &'static str) {
    match duration {
        CueDurationV1::Known { value_ms } => (Some(*value_ms), "KNOWN"),
        CueDurationV1::Estimated { value_ms } => (Some(*value_ms), "ESTIMATED"),
        CueDurationV1::Unknown => (None, "UNKNOWN"),
    }
}

fn timing_label(timing: &CueTimingV1) -> String {
    match timing {
        CueTimingV1::Absolute {
            start_ms,
            end_ms,
            status,
        } => {
            let status = match status {
                CueIntervalStatusV1::Known => "KNOWN",
                CueIntervalStatusV1::Estimated => "ESTIMATED",
            };
            format!("{status} [{start_ms}, {end_ms}) ms")
        }
        CueTimingV1::After { cue_id, duration } => {
            let (value, status) = duration_value(duration);
            match value {
                Some(value) => format!("after {cue_id}; {status} duration {value} ms"),
                None => format!("after {cue_id}; UNKNOWN duration"),
            }
        }
        CueTimingV1::Unknown { reason } => format!("UNKNOWN — {reason}"),
    }
}

fn valid_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(char::is_control)
        && !value.starts_with('/')
        && !value.starts_with("//")
        && !value.contains('\\')
        && value.split('/').enumerate().all(|(index, component)| {
            !component.is_empty()
                && component != "."
                && component != ".."
                && (index != 0 || !component.ends_with(':'))
        })
}

fn validate_asset(
    asset: &CueAssetV1,
    path: &str,
    assets: &mut BTreeMap<String, CueAssetV1>,
) -> Result<(), NarrativeCueSheetValidationErrorV1> {
    cue_id(&asset.id, &format!("{path}.id"))?;
    if asset.revision == 0 {
        return Err(error(
            "narrative.cue.invalid_shape",
            format!("{path}.revision"),
        ));
    }
    if !valid_relative_path(&asset.relative_path) {
        return Err(error(
            "narrative.cue.invalid_asset_path",
            format!("{path}.relative_path"),
        ));
    }
    if asset.relative_path.chars().count() > 1024 {
        return Err(error(
            "narrative.cue.too_many_items",
            format!("{path}.relative_path"),
        ));
    }
    if asset.sha256.len() != 64
        || !asset
            .sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(error(
            "narrative.cue.invalid_hash",
            format!("{path}.sha256"),
        ));
    }
    if asset.duration_ms == 0 || asset.duration_ms > MAX_DURATION_MS {
        return Err(error(
            "narrative.cue.invalid_duration",
            format!("{path}.duration_ms"),
        ));
    }
    if assets.insert(asset.id.clone(), asset.clone()).is_some() {
        return Err(error("narrative.cue.duplicate_id", format!("{path}.id")));
    }
    Ok(())
}

fn spoken_beat<'a>(
    document: &'a NarrativeProjectionV1,
    variant_id: &str,
    beat_id: &str,
    path: &str,
) -> Result<
    &'a storytelling_core::narrative::NarrativeProjectedBeatV1,
    NarrativeCueSheetValidationErrorV1,
> {
    for variant in &document.variants {
        if let NarrativeProjectedVariantV1::SpokenOutline { id, beats, .. } = variant
            && id == variant_id
        {
            return beats
                .iter()
                .find(|beat| beat.id == beat_id)
                .ok_or_else(|| error("narrative.cue.dangling_reference", path));
        }
    }
    Err(error("narrative.cue.invalid_variant", path))
}

fn validate_source_and_instruction(
    sheet: &NarrativeCueSheetV1,
    document: &NarrativeProjectionV1,
    assets: &BTreeMap<String, CueAssetV1>,
    cue_index: usize,
) -> Result<(String, Option<String>), NarrativeCueSheetValidationErrorV1> {
    let cue = &sheet.cues[cue_index];
    let path = format!("$.cues[{cue_index}]");
    let instruction = &cue.instruction;
    match (&cue.kind, &cue.source) {
        (
            NarrativeCueKindV1::Narration,
            NarrativeCueSourceV1::NarrativeBeat {
                variant_id,
                beat_id,
                unit_id,
                revision,
            },
        ) => {
            text(instruction, &format!("{path}.instruction"))?;
            if variant_id != &sheet.variant_id {
                return Err(error(
                    "narrative.cue.invalid_variant",
                    format!("{path}.source.variant_id"),
                ));
            }
            let beat = spoken_beat(
                document,
                variant_id,
                beat_id,
                &format!("{path}.source.beat_id"),
            )?;
            if beat.source.unit_id != *unit_id {
                return Err(error(
                    "narrative.cue.source_mismatch",
                    format!("{path}.source.unit_id"),
                ));
            }
            if beat.source.revision != *revision {
                return Err(error(
                    "narrative.cue.stale_revision",
                    format!("{path}.source.revision"),
                ));
            }
            Ok((
                format!(
                    "NarrativeDocument {}@{} · {}@{} ({}) · beat {}",
                    document.document_id,
                    document.document_revision,
                    beat.source.unit_id,
                    beat.source.revision,
                    beat.source.origin.as_str(),
                    beat.id
                ),
                Some(beat.text.clone()),
            ))
        }
        (
            NarrativeCueKindV1::SoundEffect,
            NarrativeCueSourceV1::AudioAsset { asset_id, revision },
        ) => {
            text(instruction, &format!("{path}.instruction"))?;
            let asset = assets.get(asset_id).ok_or_else(|| {
                error(
                    "narrative.cue.dangling_reference",
                    format!("{path}.source.asset_id"),
                )
            })?;
            if asset.revision != *revision {
                return Err(error(
                    "narrative.cue.stale_revision",
                    format!("{path}.source.revision"),
                ));
            }
            let declared_duration = match &cue.timing {
                CueTimingV1::Absolute {
                    start_ms,
                    end_ms,
                    status: CueIntervalStatusV1::Known,
                } => end_ms.checked_sub(*start_ms),
                CueTimingV1::After {
                    duration: CueDurationV1::Known { value_ms },
                    ..
                } => Some(*value_ms),
                _ => None,
            };
            if declared_duration.is_some_and(|duration| duration != asset.duration_ms) {
                return Err(error(
                    "narrative.cue.asset_duration_mismatch",
                    format!("{path}.timing"),
                ));
            }
            Ok((
                format!(
                    "{}@{} · {} · synthetic · SHA-256 {} · declared duration {} ms",
                    asset.id, asset.revision, asset.relative_path, asset.sha256, asset.duration_ms
                ),
                None,
            ))
        }
        (NarrativeCueKindV1::Silence, NarrativeCueSourceV1::EditorialChoice { description }) => {
            text(instruction, &format!("{path}.instruction"))?;
            text(description, &format!("{path}.source.description"))?;
            Ok((format!("Editorial choice · {description}"), None))
        }
        _ => Err(error(
            "narrative.cue.source_kind_mismatch",
            format!("{path}.source"),
        )),
    }
}

fn interval_for(
    cue_index: usize,
    sheet: &NarrativeCueSheetV1,
    previous_intervals: &BTreeMap<String, PartialInterval>,
) -> Result<PartialInterval, NarrativeCueSheetValidationErrorV1> {
    let cue = &sheet.cues[cue_index];
    let path = format!("$.cues[{cue_index}].timing");
    match &cue.timing {
        CueTimingV1::Absolute {
            start_ms, end_ms, ..
        } => {
            if start_ms >= end_ms {
                return Err(error("narrative.cue.invalid_range", path));
            }
            Ok(PartialInterval {
                start_ms: Some(*start_ms),
                end_ms: Some(*end_ms),
            })
        }
        CueTimingV1::Unknown { reason } => {
            text(reason, &format!("{path}.reason"))?;
            Ok(PartialInterval::unresolved())
        }
        CueTimingV1::After { cue_id, duration } => {
            if !sheet.cues[..cue_index]
                .iter()
                .any(|candidate| candidate.id == *cue_id)
            {
                return Err(error(
                    "narrative.cue.dangling_reference",
                    format!("{path}.cue_id"),
                ));
            }
            let anchor_interval = previous_intervals
                .get(cue_id)
                .copied()
                .unwrap_or_else(PartialInterval::unresolved);
            let (duration_ms, _) = duration_value(duration);
            if let Some(value) = duration_ms
                && (value == 0 || value > MAX_DURATION_MS)
            {
                return Err(error(
                    "narrative.cue.invalid_duration",
                    format!("{path}.duration.value_ms"),
                ));
            }
            let start_ms = anchor_interval.end_ms;
            let end_ms = match (start_ms, duration_ms) {
                (Some(start), Some(duration)) => Some(
                    start
                        .checked_add(duration)
                        .ok_or_else(|| error("narrative.cue.time_overflow", path.clone()))?,
                ),
                _ => None,
            };
            Ok(PartialInterval { start_ms, end_ms })
        }
    }
}

/// Validate a cue sheet against its exact NarrativeDocumentV1 identity and render projection.
/// This checks declared ranges only; it does not inspect or mix audio bytes.
pub fn render_narrative_cue_sheet(
    document_bytes: &[u8],
    sheet_bytes: &[u8],
) -> Result<NarrativeCueSheetProjectionV1, NarrativeCueSheetValidationErrorV1> {
    if document_bytes.len() > MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    let document = render_narrative(document_bytes).map_err(map_narrative_error)?;
    let sheet = parse_sheet(sheet_bytes)?;
    if sheet.schema_version != 1 {
        return Err(error(
            "narrative.cue.unsupported_version",
            "$.schema_version",
        ));
    }
    cue_id(&sheet.cue_sheet_id, "$.cue_sheet_id")?;
    cue_id(&sheet.document_id, "$.document_id")?;
    cue_id(&sheet.variant_id, "$.variant_id")?;
    if sheet.cue_sheet_revision == 0 {
        return Err(error("narrative.cue.invalid_shape", "$.cue_sheet_revision"));
    }
    if sheet.document_revision == 0 {
        return Err(error("narrative.cue.invalid_shape", "$.document_revision"));
    }
    if sheet.document_id != document.document_id {
        return Err(error("narrative.cue.document_mismatch", "$.document_id"));
    }
    if sheet.document_revision != document.document_revision {
        return Err(error("narrative.cue.stale_document", "$.document_revision"));
    }
    if !document.variants.iter().any(|variant| {
        matches!(variant, NarrativeProjectedVariantV1::SpokenOutline { id, .. } if id == &sheet.variant_id)
    }) {
        return Err(error("narrative.cue.invalid_variant", "$.variant_id"));
    }
    if sheet.tracks.is_empty() || sheet.tracks.len() > MAX_TRACKS {
        return Err(error("narrative.cue.too_many_items", "$.tracks"));
    }
    if sheet.assets.len() > MAX_ASSETS {
        return Err(error("narrative.cue.too_many_items", "$.assets"));
    }
    if sheet.cues.is_empty() || sheet.cues.len() > MAX_CUES {
        return Err(error("narrative.cue.too_many_items", "$.cues"));
    }

    let mut tracks = BTreeMap::new();
    let mut track_rows = Vec::with_capacity(sheet.tracks.len());
    for (index, track) in sheet.tracks.iter().enumerate() {
        let path = format!("$.tracks[{index}]");
        cue_id(&track.id, &format!("{path}.id"))?;
        if tracks.insert(track.id.clone(), track).is_some() {
            return Err(error("narrative.cue.duplicate_id", format!("{path}.id")));
        }
        match (track.channel, track.policy) {
            (CueTrackChannelV1::Marker, CueTrackPolicyV1::Marker)
            | (
                CueTrackChannelV1::Voice | CueTrackChannelV1::SoundEffect,
                CueTrackPolicyV1::Exclusive,
            )
            | (
                CueTrackChannelV1::Voice | CueTrackChannelV1::SoundEffect,
                CueTrackPolicyV1::Mixable,
            ) => {}
            _ => {
                return Err(error(
                    "narrative.cue.invalid_track_policy",
                    format!("{path}.policy"),
                ));
            }
        }
        track_rows.push(CueSheetTrackProjectionV1 {
            id: track.id.clone(),
            channel: track_channel(track.channel),
            policy: track_policy(track.policy),
        });
    }

    let mut assets = BTreeMap::new();
    for (index, asset) in sheet.assets.iter().enumerate() {
        validate_asset(asset, &format!("$.assets[{index}]"), &mut assets)?;
    }

    let mut cue_ids = BTreeSet::new();
    for (index, cue) in sheet.cues.iter().enumerate() {
        let path = format!("$.cues[{index}]");
        cue_id(&cue.id, &format!("{path}.id"))?;
        if !cue_ids.insert(cue.id.as_str()) {
            return Err(error("narrative.cue.duplicate_id", format!("{path}.id")));
        }
        if cue.order != index as u32 + 1 {
            return Err(error(
                "narrative.cue.invalid_order",
                format!("{path}.order"),
            ));
        }
        text(&cue.role, &format!("{path}.role"))?;
        if cue.role.chars().count() > 128 {
            return Err(error(
                "narrative.cue.too_many_items",
                format!("{path}.role"),
            ));
        }
        let track = tracks.get(&cue.track_id).ok_or_else(|| {
            error(
                "narrative.cue.dangling_reference",
                format!("{path}.track_id"),
            )
        })?;
        match (cue.kind, track.channel) {
            (NarrativeCueKindV1::Narration, CueTrackChannelV1::Voice)
            | (NarrativeCueKindV1::SoundEffect, CueTrackChannelV1::SoundEffect)
            | (NarrativeCueKindV1::Silence, CueTrackChannelV1::Voice | CueTrackChannelV1::Marker) =>
                {}
            _ => {
                return Err(error(
                    "narrative.cue.track_kind_mismatch",
                    format!("{path}.track_id"),
                ));
            }
        }
    }

    let mut cue_rows = Vec::with_capacity(sheet.cues.len());
    let mut intervals = BTreeMap::new();
    let mut interval_rows = Vec::with_capacity(sheet.cues.len());
    for (index, cue) in sheet.cues.iter().enumerate() {
        let (source, spoken_text) =
            validate_source_and_instruction(&sheet, &document, &assets, index)?;
        let interval = interval_for(index, &sheet, &intervals)?;
        intervals.insert(cue.id.clone(), interval);
        interval_rows.push((cue.id.clone(), cue.track_id.clone(), interval));
        cue_rows.push(NarrativeCueProjectionV1 {
            order: cue.order,
            id: cue.id.clone(),
            kind: cue_kind(cue.kind),
            role: cue.role.clone(),
            track_id: cue.track_id.clone(),
            source,
            instruction: cue.instruction.clone(),
            spoken_text,
            timing: timing_label(&cue.timing),
        });
    }

    for first in 0..interval_rows.len() {
        let (first_id, first_track_id, first_interval) = &interval_rows[first];
        let track = tracks
            .get(first_track_id)
            .expect("track reference was checked");
        if track.policy != CueTrackPolicyV1::Exclusive || !first_interval.is_resolved() {
            continue;
        }
        for (second_id, second_track_id, second_interval) in interval_rows.iter().skip(first + 1) {
            if first_track_id == second_track_id && first_interval.overlaps(*second_interval) {
                let mut cue_ids = vec![first_id.clone(), second_id.clone()];
                cue_ids.sort();
                return Err(error("narrative.cue.track_conflict", "$.cues").with_cues(cue_ids));
            }
        }
    }

    let resolved_intervals = interval_rows
        .iter()
        .filter(|(_, _, interval)| interval.is_resolved())
        .count();
    Ok(NarrativeCueSheetProjectionV1 {
        cue_sheet_id: sheet.cue_sheet_id,
        cue_sheet_revision: sheet.cue_sheet_revision,
        document_id: document.document_id,
        document_revision: document.document_revision,
        variant_id: sheet.variant_id,
        time_unit: sheet.time_unit.as_str(),
        tracks: track_rows,
        cues: cue_rows,
        resolved_intervals,
        unresolved_intervals: interval_rows.len() - resolved_intervals,
    })
}
