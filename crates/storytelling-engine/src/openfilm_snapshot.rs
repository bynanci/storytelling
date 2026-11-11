// SPDX-License-Identifier: Apache-2.0
// Source semantics and attribution: docs/migration/provenance-m1-01.json, LICENSES/third-party/NOTICE.txt.
//! Read-only import of a finite declared visual snapshot. No host compatibility claim.
mod budget;
mod decimal;
mod dto;

use decimal::Exact;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::value::RawValue;
use std::collections::{BTreeMap, BTreeSet};
use storytelling_core::compatibility::{
    ClockFacts, ImportError, ImportLoss, ImportProfileV1, LossDisposition,
};
use storytelling_core::wire::{DeclaredAsset, DeclaredShot, PlanDocumentV1, SourceRange};

const MAX_OUTPUT_BYTES: usize = 4_194_304;

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
    bytes: &[u8],
    _profile: &ImportProfileV1,
) -> Result<ImportedPlanV1, ImportError> {
    if bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    let raw: Box<RawValue> =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    let version: dto::VersionProbe = serde_json::from_str(raw.get()).map_err(|cause| {
        if cause.to_string().starts_with("duplicate field") {
            error("input.invalid_json", "$")
        } else {
            error("import.invalid_snapshot", "$")
        }
    })?;
    if version.schema_version != "1.0.0" {
        return Err(error("import.unsupported_version", "$.schemaVersion"));
    }
    budget::preflight(bytes)?;
    check_features(&raw, &["metadata"], "$")?;
    let envelope: dto::Envelope = decode(&raw, "$")?;
    if envelope.assets.len() > crate::MAX_ASSETS {
        return Err(error("import.budget_exceeded", "$.assets"));
    }
    if envelope.assets.is_empty() {
        return Err(error("import.invalid_snapshot", "$.assets"));
    }
    check_features(
        &envelope.composition,
        &[
            "storyPresetSnapshot",
            "defaultLook",
            "defaultPrimaryColor",
            "beatPrimaryColors",
            "beatLooks",
            "captionTracks",
            "graphicScenes",
            "graphicSceneTemplates",
            "coverageAudioBindings",
            "effects",
        ],
        "$.composition",
    )?;
    let composition: dto::Composition = decode(&envelope.composition, "$.composition")?;
    id(&composition.id, "$.composition.id")?;
    id(&composition.story_id, "$.composition.storyId")?;
    let composition_duration = positive(&composition.duration, "$.composition.duration")?;
    if composition.tracks.is_empty() {
        return Err(error("import.invalid_snapshot", "$.composition.tracks"));
    }
    // Inspect all track types before rejecting multitrack input, so audio never disappears.
    let mut tracks = Vec::new();
    for (index, raw_track) in composition.tracks.iter().enumerate() {
        let path = format!("$.composition.tracks[{index}]");
        check_features(raw_track, &["timing", "effects"], &path)?;
        let track: dto::Track = decode(raw_track, &path)?;
        if track.kind != "video" {
            return Err(unsupported(&format!("{path}.type")));
        }
        if track.visible == Some(false) || track.muted == Some(true) || track.solo == Some(true) {
            return Err(unsupported(&path));
        }
        tracks.push(track);
    }
    if tracks.len() != 1 {
        return Err(unsupported("$.composition.tracks"));
    }
    let track = tracks.pop().expect("one admitted track");
    id(&track.id, "$.composition.tracks[0].id")?;
    if track.clips.len() > crate::MAX_SHOTS {
        return Err(error(
            "import.budget_exceeded",
            "$.composition.tracks[0].clips",
        ));
    }
    if track.clips.is_empty() {
        return Err(error(
            "import.invalid_snapshot",
            "$.composition.tracks[0].clips",
        ));
    }
    let clocks = clocks(&envelope.settings)?;
    let mut assets = Vec::new();
    let mut asset_indexes = BTreeMap::new();
    for (index, raw_asset) in envelope.assets.iter().enumerate() {
        let path = format!("$.assets[{index}]");
        let asset: dto::Asset = decode(raw_asset, &path)?;
        id(&asset.id, &format!("{path}.id"))?;
        if asset_indexes.insert(asset.id.clone(), index).is_some() {
            return Err(error("plan.duplicate_id", &format!("{path}.id")));
        }
        if asset.uri.is_empty() {
            return Err(error("import.invalid_snapshot", &format!("{path}.uri")));
        }
        if !matches!(asset.media_type.as_str(), "image" | "video") {
            return Err(unsupported(&format!("{path}.mediaType")));
        }
        probe(&asset.metadata, &format!("{path}.metadata"))?;
        let duration = asset
            .duration
            .as_ref()
            .map(|raw| positive(raw, &format!("{path}.duration")))
            .transpose()?;
        if asset.media_type == "video" && duration.is_none() {
            return Err(error(
                "import.invalid_snapshot",
                &format!("{path}.duration"),
            ));
        }
        let hash = asset.content_hash.as_deref().unwrap_or_default();
        if hash.len() != 64
            || !hash
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(error("plan.invalid_hash", &format!("{path}.contentHash")));
        }
        assets.push((asset, duration));
    }
    let mut used = BTreeSet::new();
    let mut shot_ids = BTreeSet::new();
    let mut holds = vec![Exact::ZERO; assets.len()];
    let mut cursor = Exact::ZERO;
    let mut shots = Vec::new();
    for (index, raw_clip) in track.clips.iter().enumerate() {
        let path = format!("$.composition.tracks[0].clips[{index}]");
        check_features(
            raw_clip,
            &[
                "userOverrides",
                "transform",
                "title",
                "titleRuns",
                "layout",
                "motion",
                "titleStyle",
                "titlePresetSnapshot",
                "audioAutomation",
                "audioTiming",
                "coverage",
                "look",
                "primaryColor",
                "transition",
                "transitions",
                "animation",
                "effects",
            ],
            &path,
        )?;
        let clip: dto::Clip = decode(raw_clip, &path)?;
        id(&clip.id, &format!("{path}.id"))?;
        id(&clip.asset_id, &format!("{path}.assetId"))?;
        if let Some(beat) = &clip.beat_id {
            id(beat, &format!("{path}.beatId"))?;
        }
        if !shot_ids.insert(clip.id.clone()) {
            return Err(error("plan.duplicate_id", &format!("{path}.id")));
        }
        let asset_index = *asset_indexes
            .get(&clip.asset_id)
            .ok_or_else(|| error("plan.unknown_asset", &format!("{path}.assetId")))?;
        used.insert(asset_index);
        let (asset, duration) = &assets[asset_index];
        let start = time(&clip.timeline_start, &format!("{path}.timelineStart"))?;
        let span = positive(&clip.timeline_duration, &format!("{path}.timelineDuration"))?;
        if start != cursor {
            return Err(error(
                "import.inconsistent_timeline",
                &format!("{path}.timelineStart"),
            ));
        }
        cursor = cursor
            .checked_add(span)
            .ok_or_else(|| error("import.invalid_time", &path))?;
        let source_in = clip
            .source_in
            .as_ref()
            .map(|raw| time(raw, &format!("{path}.sourceIn")))
            .transpose()?
            .unwrap_or(Exact::ZERO);
        let source_out = match &clip.source_out {
            Some(raw) => time(raw, &format!("{path}.sourceOut"))?,
            None => source_in
                .checked_add(span)
                .ok_or_else(|| error("import.invalid_time", &path))?,
        };
        if source_out.checked_sub(source_in) != Some(span)
            || (asset.media_type == "image" && source_in != Exact::ZERO)
            || (asset.media_type == "video" && Some(source_out) > *duration)
        {
            return Err(error("import.invalid_range", &path));
        }
        holds[asset_index] = holds[asset_index].max(source_out);
        shots.push(DeclaredShot {
            id: clip.id,
            asset_id: clip.asset_id,
            source_range: SourceRange {
                start: source_in.wire(),
                end: source_out.wire(),
            },
        });
    }
    if cursor != composition_duration {
        return Err(error(
            "import.inconsistent_timeline",
            "$.composition.duration",
        ));
    }
    if used.len() != assets.len() {
        return Err(error("import.invalid_snapshot", "$.assets"));
    }
    let mut losses = vec![loss(
        "media.not_verified",
        "$.assets",
        LossDisposition::NotVerified,
    )];
    let mut mapped_assets = Vec::new();
    for (index, (asset, duration)) in assets.into_iter().enumerate() {
        let image = asset.media_type == "image";
        let duration = if image {
            holds[index]
        } else {
            duration.expect("admitted video duration")
        };
        losses.push(loss(
            if image {
                "image.hold_window"
            } else {
                "audio.not_mapped"
            },
            &format!("$.assets[{index}]"),
            if image {
                LossDisposition::IntentionalDelta
            } else {
                LossDisposition::BlocksHostHandoff
            },
        ));
        mapped_assets.push(DeclaredAsset {
            id: asset.id,
            content_hash: asset.content_hash.expect("admitted declared hash"),
            duration: duration.wire(),
        });
    }
    let plan = PlanDocumentV1 {
        schema_version: 1,
        plan_id: composition.id,
        assets: mapped_assets,
        shots,
    };
    let plan_bytes =
        serde_json::to_vec(&plan).map_err(|_| error("import.invalid_snapshot", "$"))?;
    if plan_bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(error("import.output_too_large", "$.plan"));
    }
    crate::validate_plan_json(&plan_bytes)?;
    let compact =
        String::from_utf8(budget::compact(bytes)).map_err(|_| error("input.invalid_json", "$"))?;
    let result = ImportedPlanV1 {
        profile: "openfilm-finite-v1",
        scope: "declared-visual-mapping-only",
        executable: false,
        plan,
        clocks,
        losses,
        source_snapshot: RawValue::from_string(compact)
            .map_err(|_| error("input.invalid_json", "$"))?,
    };
    if serde_json::to_vec(&result)
        .map_err(|_| error("import.invalid_snapshot", "$"))?
        .len()
        > MAX_OUTPUT_BYTES
    {
        return Err(error("import.output_too_large", "$"));
    }
    Ok(result)
}

fn error(code: &str, path: &str) -> ImportError {
    ImportError {
        code: code.to_owned(),
        path: path.to_owned(),
        losses: Vec::new(),
    }
}

fn loss(code: &str, path: &str, disposition: LossDisposition) -> ImportLoss {
    ImportLoss {
        code: code.to_owned(),
        path: path.to_owned(),
        disposition,
    }
}

fn unsupported(path: &str) -> ImportError {
    ImportError {
        code: "import.unsupported_feature".to_owned(),
        path: path.to_owned(),
        losses: vec![loss(
            "import.unsupported_feature",
            path,
            LossDisposition::Unsupported,
        )],
    }
}

fn decode<T: DeserializeOwned>(raw: &RawValue, path: &str) -> Result<T, ImportError> {
    serde_json::from_str(raw.get()).map_err(|_| error("import.invalid_snapshot", path))
}

fn check_features(
    raw: &RawValue,
    unsupported_keys: &[&str],
    path: &str,
) -> Result<(), ImportError> {
    let fields: BTreeMap<String, Box<RawValue>> = decode(raw, path)?;
    for key in unsupported_keys {
        if fields.contains_key(*key) {
            return Err(unsupported(&format!("{path}.{key}")));
        }
    }
    Ok(())
}

fn id(value: &str, path: &str) -> Result<(), ImportError> {
    crate::identifier(value, path.to_owned()).map_err(Into::into)
}

fn time(raw: &RawValue, path: &str) -> Result<Exact, ImportError> {
    Exact::parse(raw.get()).ok_or_else(|| error("import.invalid_time", path))
}

fn positive(raw: &RawValue, path: &str) -> Result<Exact, ImportError> {
    let value = time(raw, path)?;
    if value == Exact::ZERO {
        Err(error("import.invalid_range", path))
    } else {
        Ok(value)
    }
}

fn clocks(raw: &RawValue) -> Result<ClockFacts, ImportError> {
    let settings: dto::Settings = decode(raw, "$.settings")?;
    if settings.width == 0
        || settings.width > 16384
        || settings.height == 0
        || settings.height > 16384
    {
        return Err(error("import.invalid_snapshot", "$.settings"));
    }
    let rate = positive(&settings.frame_rate, "$.settings.frameRate")?;
    if rate > Exact::from_parts(240, 1).expect("constant rate") {
        return Err(error("import.invalid_time", "$.settings.frameRate"));
    }
    if let Some(base) = &settings.timebase {
        let n = u64::from(base.rate_numerator);
        let d = u64::from(base.rate_denominator);
        let exact = Exact::from_parts(n, d)
            .ok_or_else(|| error("import.invalid_time", "$.settings.timebase"))?;
        if base.version != 1
            || n == 0
            || exact.numerator != n
            || exact.denominator != d
            || (base.drop_frame && !((n == 30000 || n == 60000) && d == 1001))
        {
            return Err(error("import.invalid_time", "$.settings.timebase"));
        }
        let left = u128::from(rate.numerator) * u128::from(d);
        let right = u128::from(n) * u128::from(rate.denominator);
        let distance = left
            .abs_diff(right)
            .checked_mul(1_000_000_000)
            .ok_or_else(|| error("import.invalid_time", "$.settings.timebase"))?;
        if distance > right {
            return Err(error("import.invalid_time", "$.settings.timebase"));
        }
    }
    Ok(ClockFacts {
        declared_frame_rate: settings.frame_rate.get().to_owned(),
        exact_frame_rate: rate.wire(),
        explicit_timebase: settings.timebase,
    })
}

fn probe(raw: &RawValue, path: &str) -> Result<(), ImportError> {
    let metadata: BTreeMap<String, Box<RawValue>> = decode(raw, path)?;
    if let Some(probe) = metadata.get("openfilm.ffprobe") {
        let probe: BTreeMap<String, Box<RawValue>> = decode(probe, path)?;
        let streams = probe
            .get("streams")
            .ok_or_else(|| error("import.invalid_snapshot", path))?;
        let streams: Vec<Box<RawValue>> = decode(streams, path)?;
        for stream in streams {
            let fields: BTreeMap<String, Box<RawValue>> = decode(&stream, path)?;
            let kind = fields
                .get("codec_type")
                .ok_or_else(|| error("import.invalid_snapshot", path))?;
            let kind: String = decode(kind, path)?;
            if kind == "audio" {
                return Err(unsupported(path));
            }
        }
    }
    Ok(())
}
