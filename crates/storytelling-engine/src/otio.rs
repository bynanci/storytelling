// SPDX-License-Identifier: Apache-2.0
// Origin/modifications: docs/migration/provenance-m1-02.json; see NOTICE.openfilm.
//! Bounded finite OTIO serialization; no media, parser-runtime or host calls.
mod clock;

use clock::ContentClock;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use storytelling_core::compiled_plan::{
    FeatureDisposition, OtioCompileError, OtioCompileReportV1, OtioFeatureV1, OtioLossChoicesV1,
    OtioProfileV1,
};

const MAX_OUTPUT_BYTES: usize = 4_194_304;

#[derive(Debug, Serialize)]
pub struct CompiledOtioV1 {
    pub document: Value,
    pub report: OtioCompileReportV1,
}

/// Compile an admitted composition into format-only, non-executable OTIO JSON.
pub fn export_openfilm_otio(
    bytes: &[u8],
    _profile: &OtioProfileV1,
    choices: &OtioLossChoicesV1,
) -> Result<CompiledOtioV1, OtioCompileError> {
    let imported = crate::import_openfilm_snapshot(bytes, &Default::default())?;
    // Projection DTOs consume only an already strictly validated snapshot. Opaque
    // metadata is skipped without numeric conversion and retained as source text.
    let facts: SnapshotFacts = serde_json::from_str(imported.source_snapshot.get())
        .map_err(|_| error("import.invalid_snapshot", "$"))?;
    let track = facts
        .composition
        .tracks
        .first()
        .ok_or_else(|| error("import.invalid_snapshot", "$.composition.tracks"))?;
    let features = features(&facts, choices);
    let unaccepted: Vec<_> = features
        .iter()
        .filter(|row| !row.accepted)
        .cloned()
        .collect();
    if !unaccepted.is_empty() {
        return Err(OtioCompileError {
            code: "otio.loss_choice_required".to_owned(),
            path: "$.choices".to_owned(),
            losses: unaccepted,
        });
    }
    let clock = ContentClock::for_plan(&imported.plan)?;
    let source_assets: BTreeMap<_, _> = facts
        .assets
        .iter()
        .map(|asset| (asset.id.as_str(), asset))
        .collect();
    let mut asset_indexes = BTreeMap::new();
    let mut references = Vec::new();
    for (index, asset) in imported.plan.assets.iter().enumerate() {
        let source = source_assets
            .get(asset.id.as_str())
            .ok_or_else(|| error("import.invalid_snapshot", "$.assets"))?;
        let available = clock.ticks(&asset.duration, &format!("$.assets[{index}].duration"))?;
        asset_indexes.insert(asset.id.as_str(), index);
        references.push(json!({
            "OTIO_SCHEMA": "ExternalReference.1",
            "name": source.name,
            "target_url": source.uri,
            "available_range": native_range(0, available, clock.rate()),
            "metadata": {"storytelling": {
                "asset_id": asset.id,
                "declared_content_hash": asset.content_hash,
                "media_type": source.media_type,
                "available_range_kind": if source.media_type == "image" { "synthetic-still-hold" } else { "declared-video-duration" }
            }}
        }));
    }
    let mut total_ticks = 0;
    let mut projections = Vec::new();
    for (index, shot) in imported.plan.shots.iter().enumerate() {
        let path = format!("$.shots[{index}].source_range");
        let start = clock.ticks(&shot.source_range.start, &path)?;
        let end = clock.ticks(&shot.source_range.end, &path)?;
        let duration = end
            .checked_sub(start)
            .filter(|value| *value > 0)
            .ok_or_else(|| error("otio.invalid_time", &path))?;
        total_ticks = clock.add_timeline_ticks(total_ticks, duration)?;
        let asset_index = *asset_indexes
            .get(shot.asset_id.as_str())
            .ok_or_else(|| error("import.invalid_snapshot", "$.assets"))?;
        projections.push((asset_index, start, duration));
    }
    let report = OtioCompileReportV1 {
        schema_version: 1,
        profile: "storytelling.otio-visual-ticks.v1",
        scope: "declared-otio-visual-interchange",
        executable: false,
        source_verified: false,
        host_verified: false,
        host_name: None,
        host_version: None,
        required_capabilities: [
            "single-video-track",
            "external-reference",
            "still-hold",
            "arbitrary-content-tick-rate",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        content_clock_rate: clock.rate().to_string(),
        timeline_duration: clock.seconds(total_ticks),
        project_clocks: imported.clocks,
        choices: *choices,
        feature_matrix: features,
        import_limitations: imported.losses,
    };
    let mut document = json!({
        "OTIO_SCHEMA": "Timeline.1",
        "name": imported.plan.plan_id,
        "global_start_time": null,
        "metadata": {"storytelling": {
            "report": &report,
            "source_snapshot_json": imported.source_snapshot.get()
        }},
        "tracks": {
            "OTIO_SCHEMA": "Stack.1", "name": "storytelling",
            "source_range": null, "effects": [], "markers": [],
            "children": [{
                "OTIO_SCHEMA": "Track.1", "name": track.id, "kind": "Video",
                "source_range": null, "effects": [], "markers": [], "children": []
            }]
        }
    });
    let mut output_bytes = serialized_size(&document)?;
    if output_bytes > MAX_OUTPUT_BYTES {
        return Err(error("otio.output_too_large", "$"));
    }
    let children = document["tracks"]["children"][0]["children"]
        .as_array_mut()
        .ok_or_else(|| error("import.invalid_snapshot", "$"))?;
    for (index, (shot, &(asset_index, start, duration))) in
        imported.plan.shots.iter().zip(&projections).enumerate()
    {
        let reference = &references[asset_index];
        let clip = json!({
            "OTIO_SCHEMA": "Clip.1", "name": reference["name"],
            "effects": [], "markers": [],
            "source_range": native_range(start, duration, clock.rate()),
            "media_reference": reference,
            "metadata": {"storytelling": {
                "occurrence_id": shot.id, "asset_id": shot.asset_id,
                "source_range": shot.source_range
            }}
        });
        output_bytes = output_bytes
            .checked_add(serialized_size(&clip)?)
            .and_then(|size| size.checked_add(usize::from(index > 0)))
            .filter(|size| *size <= MAX_OUTPUT_BYTES)
            .ok_or_else(|| error("otio.output_too_large", "$"))?;
        children.push(clip);
    }
    if serialized_size(&document)? > MAX_OUTPUT_BYTES {
        return Err(error("otio.output_too_large", "$"));
    }
    Ok(CompiledOtioV1 { document, report })
}

fn error(code: &str, path: &str) -> OtioCompileError {
    OtioCompileError {
        code: code.to_owned(),
        path: path.to_owned(),
        losses: Vec::new(),
    }
}

fn feature(
    name: &str,
    path: &str,
    disposition: FeatureDisposition,
    accepted: bool,
) -> OtioFeatureV1 {
    OtioFeatureV1 {
        feature: name.to_owned(),
        path: path.to_owned(),
        disposition,
        accepted,
    }
}

fn features(facts: &SnapshotFacts, choices: &OtioLossChoicesV1) -> Vec<OtioFeatureV1> {
    use FeatureDisposition::{MetadataOnly, Native};
    let mut rows = vec![
        feature("timeline.cuts", "$.composition", Native, true),
        feature("media.references", "$.assets", Native, true),
    ];
    if facts.assets.iter().any(|asset| asset.media_type == "image") {
        rows.push(feature("image.holds", "$.assets", Native, true));
    }
    rows.extend([
        feature("clock.content_ticks", "$.settings", Native, true),
        feature(
            "project_rate.metadata_only",
            "$.settings",
            MetadataOnly,
            true,
        ),
        feature("source_facts.metadata_only", "$", MetadataOnly, true),
    ]);
    if facts.assets.iter().any(|asset| asset.media_type == "video") {
        rows.push(feature(
            "audio.picture_only",
            "$.assets",
            MetadataOnly,
            choices.picture_only,
        ));
    }
    let locks = facts
        .assets
        .iter()
        .any(|asset| asset.state.locked == Some(true))
        || facts.composition.tracks.iter().any(|track| {
            track.locked == Some(true) || track.clips.iter().any(|clip| clip.locked == Some(true))
        });
    if locks {
        rows.push(feature(
            "editing.locks_metadata_only",
            "$",
            MetadataOnly,
            choices.metadata_only_locks,
        ));
    }
    rows
}

fn native_range(start: u64, duration: u64, rate: u64) -> Value {
    json!({
        "OTIO_SCHEMA": "TimeRange.1",
        "start_time": {"OTIO_SCHEMA":"RationalTime.1","value":start,"rate":rate},
        "duration": {"OTIO_SCHEMA":"RationalTime.1","value":duration,"rate":rate}
    })
}

fn serialized_size(value: &Value) -> Result<usize, OtioCompileError> {
    serde_json::to_vec(value)
        .map(|bytes| bytes.len())
        .map_err(|_| error("otio.output_too_large", "$"))
}

// These are projections of importer-validated data, not independent wire admission.
// Unknown fields are retained in source_snapshot_json and are not parsed as numbers.
#[derive(Deserialize)]
struct SnapshotFacts {
    composition: CompositionFacts,
    assets: Vec<AssetFacts>,
}

#[derive(Deserialize)]
struct CompositionFacts {
    tracks: Vec<TrackFacts>,
}

#[derive(Deserialize)]
struct TrackFacts {
    id: String,
    locked: Option<bool>,
    clips: Vec<ClipFacts>,
}

#[derive(Deserialize)]
struct ClipFacts {
    locked: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssetFacts {
    id: String,
    name: String,
    uri: String,
    media_type: String,
    state: StateFacts,
}

#[derive(Deserialize)]
struct StateFacts {
    locked: Option<bool>,
}
