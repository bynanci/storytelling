// SPDX-License-Identifier: MIT
//! Pure validation and deterministic projection for NarrativeVideoCoverageV1.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use serde_json::value::RawValue;
use storytelling_core::narrative::{NarrativeDocumentV1, NarrativeUnitReferenceV1};
use storytelling_core::narrative_video_coverage::{
    NarrativeVideoCoverageErrorV1, NarrativeVideoCoverageReferenceV1,
    NarrativeVideoCoverageResultV1, VideoAssetCatalogReferenceV1, VideoAssetCatalogV1,
    VideoAssetV1, VideoCoverageDiagnosticV1, VideoCoverageMappingResultV1, VideoCoverageMappingV1,
};
use storytelling_core::time::RationalTime;
use storytelling_core::wire::TimeValue;

use crate::narrative::{preflight_json_with_max_nesting, render_narrative};

const MAX_INPUT_BYTES: usize = crate::MAX_INPUT_BYTES;
const MAX_DEPTH: usize = 33;
const MAX_UNITS: usize = 512;
const MAX_ASSETS: usize = 1024;

type AssetIndex<'a> = BTreeMap<&'a str, &'a VideoAssetV1>;
type DurationIndex<'a> = BTreeMap<&'a str, RationalTime>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCoverageV1<'a> {
    schema_version: u32,
    coverage_id: String,
    #[serde(borrow)]
    narrative_document: &'a RawValue,
    variant_id: String,
    selected_unit_refs: Vec<NarrativeUnitReferenceV1>,
    asset_catalog: VideoAssetCatalogV1,
    mappings: Vec<VideoCoverageMappingV1>,
}

fn failure(code: &str, path: impl Into<String>) -> NarrativeVideoCoverageErrorV1 {
    NarrativeVideoCoverageErrorV1::new(code, path)
}

fn valid_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 128
        && bytes[0].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(byte))
}

fn exact_time(value: &TimeValue) -> Result<RationalTime, ()> {
    RationalTime::from_strings(&value.numerator, &value.denominator).map_err(|_| ())
}

fn map_shape_error(message: &str) -> &'static str {
    if message.contains("unknown field") {
        "video_coverage.unknown_field"
    } else {
        "video_coverage.invalid_shape"
    }
}

fn nested_path(path: &str) -> String {
    if path == "$" {
        "$.narrative_document".to_owned()
    } else {
        format!(
            "$.narrative_document{}",
            path.strip_prefix('$').unwrap_or(path)
        )
    }
}

fn narrative_error(
    error: storytelling_core::narrative::NarrativeValidationErrorV1,
) -> NarrativeVideoCoverageErrorV1 {
    NarrativeVideoCoverageErrorV1::new(&error.code, nested_path(&error.path))
}

fn variant_refs(
    projection: &crate::NarrativeProjectionV1,
    variant_id: &str,
) -> Result<Vec<NarrativeUnitReferenceV1>, NarrativeVideoCoverageErrorV1> {
    for variant in &projection.variants {
        match variant {
            crate::NarrativeProjectedVariantV1::SpokenOutline { id, beats, .. }
                if id == variant_id =>
            {
                return Ok(beats
                    .iter()
                    .map(|beat| NarrativeUnitReferenceV1 {
                        unit_id: beat.source.unit_id.clone(),
                        revision: beat.source.revision,
                    })
                    .collect());
            }
            crate::NarrativeProjectedVariantV1::SingleIllustration { id, sources, .. }
                if id == variant_id =>
            {
                return Ok(sources
                    .iter()
                    .map(|source| NarrativeUnitReferenceV1 {
                        unit_id: source.unit_id.clone(),
                        revision: source.revision,
                    })
                    .collect());
            }
            _ => {}
        }
    }
    Err(failure("video_coverage.dangling_variant", "$.variant_id"))
}

fn validate_catalog(
    catalog: &VideoAssetCatalogV1,
) -> Result<(AssetIndex<'_>, DurationIndex<'_>), NarrativeVideoCoverageErrorV1> {
    if !valid_id(&catalog.catalog_id) {
        return Err(failure(
            "video_coverage.invalid_shape",
            "$.asset_catalog.catalog_id",
        ));
    }
    if catalog.revision == 0 {
        return Err(failure(
            "video_coverage.invalid_shape",
            "$.asset_catalog.revision",
        ));
    }
    if catalog.assets.len() > MAX_ASSETS {
        return Err(failure(
            "video_coverage.too_many_items",
            "$.asset_catalog.assets",
        ));
    }
    let mut assets = BTreeMap::new();
    let mut durations = BTreeMap::new();
    for (index, asset) in catalog.assets.iter().enumerate() {
        let path = format!("$.asset_catalog.assets[{index}]");
        if !valid_id(&asset.asset_id) {
            return Err(failure(
                "video_coverage.invalid_shape",
                format!("{path}.asset_id"),
            ));
        }
        if asset.revision == 0 {
            return Err(failure(
                "video_coverage.invalid_shape",
                format!("{path}.revision"),
            ));
        }
        if assets.insert(asset.asset_id.as_str(), asset).is_some() {
            return Err(failure(
                "video_coverage.duplicate_asset",
                format!("{path}.asset_id"),
            ));
        }
        let duration = exact_time(&asset.duration)
            .map_err(|_| failure("video_coverage.invalid_range", format!("{path}.duration")))?;
        if duration.is_zero() {
            return Err(failure(
                "video_coverage.invalid_range",
                format!("{path}.duration"),
            ));
        }
        durations.insert(asset.asset_id.as_str(), duration);
    }
    Ok((assets, durations))
}

/// Validates only the supplied declarations. It does not inspect media bytes or paths.
pub fn render_narrative_video_coverage(
    bytes: &[u8],
) -> Result<NarrativeVideoCoverageResultV1, NarrativeVideoCoverageErrorV1> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(NarrativeVideoCoverageErrorV1::with_optional_path(
            "input.too_large",
            None,
        ));
    }
    preflight_json_with_max_nesting(bytes, MAX_DEPTH).map_err(|error| {
        if error.code == "narrative.duplicate_field" {
            failure("video_coverage.duplicate_field", "$")
        } else if error.code == "narrative.depth_exceeded" {
            failure("video_coverage.depth_exceeded", "$")
        } else {
            NarrativeVideoCoverageErrorV1::with_optional_path("input.invalid_json", None)
        }
    })?;

    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|_| {
        NarrativeVideoCoverageErrorV1::with_optional_path("input.invalid_json", None)
    })?;
    let version = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| failure("video_coverage.invalid_shape", "$.schema_version"))?;
    if version != 1 {
        return Err(failure(
            "video_coverage.unsupported_version",
            "$.schema_version",
        ));
    }

    let raw: RawCoverageV1<'_> = serde_json::from_slice(bytes)
        .map_err(|error| failure(map_shape_error(&error.to_string()), "$"))?;
    if raw.schema_version != 1 {
        return Err(failure(
            "video_coverage.unsupported_version",
            "$.schema_version",
        ));
    }
    if !valid_id(&raw.coverage_id) {
        return Err(failure("video_coverage.invalid_shape", "$.coverage_id"));
    }
    if !valid_id(&raw.variant_id) {
        return Err(failure("video_coverage.invalid_shape", "$.variant_id"));
    }
    if raw.selected_unit_refs.is_empty() {
        return Err(failure(
            "video_coverage.invalid_shape",
            "$.selected_unit_refs",
        ));
    }
    if raw.selected_unit_refs.len() > MAX_UNITS {
        return Err(failure(
            "video_coverage.too_many_items",
            "$.selected_unit_refs",
        ));
    }
    if raw.mappings.is_empty() {
        return Err(failure("video_coverage.invalid_shape", "$.mappings"));
    }
    if raw.mappings.len() > MAX_UNITS {
        return Err(failure("video_coverage.too_many_items", "$.mappings"));
    }

    let projection =
        render_narrative(raw.narrative_document.get().as_bytes()).map_err(narrative_error)?;
    let narrative: NarrativeDocumentV1 = serde_json::from_str(raw.narrative_document.get())
        .map_err(|_| failure("video_coverage.invalid_shape", "$.narrative_document"))?;
    let allowed_refs = variant_refs(&projection, &raw.variant_id)?;
    let unit_revisions: BTreeMap<&str, u32> = narrative
        .units
        .iter()
        .map(|unit| (unit.id.as_str(), unit.revision))
        .collect();

    let mut selected_ids = BTreeSet::new();
    let mut previous_position = None;
    for (index, reference) in raw.selected_unit_refs.iter().enumerate() {
        let path = format!("$.selected_unit_refs[{index}]");
        if !valid_id(&reference.unit_id) {
            return Err(failure(
                "video_coverage.invalid_shape",
                format!("{path}.unit_id"),
            ));
        }
        if reference.revision == 0 {
            return Err(failure(
                "video_coverage.invalid_shape",
                format!("{path}.revision"),
            ));
        }
        if !selected_ids.insert(reference.unit_id.as_str()) {
            return Err(failure(
                "video_coverage.duplicate_selection",
                format!("{path}.unit_id"),
            ));
        }
        let Some(&current_revision) = unit_revisions.get(reference.unit_id.as_str()) else {
            return Err(failure(
                "video_coverage.dangling_unit",
                format!("{path}.unit_id"),
            ));
        };
        if reference.revision != current_revision {
            return Err(failure(
                "video_coverage.stale_unit_revision",
                format!("{path}.revision"),
            ));
        }
        let Some(position) = allowed_refs
            .iter()
            .position(|candidate| candidate == reference)
        else {
            return Err(failure(
                "video_coverage.unit_outside_variant",
                format!("{path}.unit_id"),
            ));
        };
        if previous_position.is_some_and(|previous| position <= previous) {
            return Err(failure(
                "video_coverage.selection_order_mismatch",
                format!("{path}.unit_id"),
            ));
        }
        previous_position = Some(position);
    }

    let (assets, durations) = validate_catalog(&raw.asset_catalog)?;

    let mut mapping_by_unit = BTreeMap::new();
    for (index, mapping) in raw.mappings.iter().enumerate() {
        let reference = mapping.unit_ref();
        let path = format!("$.mappings[{index}]");
        if !valid_id(&reference.unit_id) {
            return Err(failure(
                "video_coverage.invalid_shape",
                format!("{path}.unit_ref.unit_id"),
            ));
        }
        if reference.revision == 0 {
            return Err(failure(
                "video_coverage.invalid_shape",
                format!("{path}.unit_ref.revision"),
            ));
        }
        if !unit_revisions.contains_key(reference.unit_id.as_str()) {
            return Err(failure(
                "video_coverage.dangling_unit",
                format!("{path}.unit_ref.unit_id"),
            ));
        }
        if unit_revisions.get(reference.unit_id.as_str()) != Some(&reference.revision) {
            return Err(failure(
                "video_coverage.stale_unit_revision",
                format!("{path}.unit_ref.revision"),
            ));
        }
        if !selected_ids.contains(reference.unit_id.as_str()) {
            return Err(failure(
                "video_coverage.mapping_outside_selection",
                format!("{path}.unit_ref.unit_id"),
            ));
        }
        if mapping_by_unit
            .insert(reference.unit_id.as_str(), (index, mapping))
            .is_some()
        {
            return Err(failure(
                "video_coverage.duplicate_mapping",
                format!("{path}.unit_ref.unit_id"),
            ));
        }
    }
    for (index, reference) in raw.selected_unit_refs.iter().enumerate() {
        let Some(&(mapping_index, mapping)) = mapping_by_unit.get(reference.unit_id.as_str())
        else {
            return Err(failure(
                "video_coverage.missing_selected_unit",
                format!("$.selected_unit_refs[{index}]"),
            ));
        };
        if mapping_index != index {
            return Err(failure(
                "video_coverage.mapping_order_mismatch",
                format!("$.mappings[{mapping_index}].unit_ref"),
            ));
        }
        if mapping.unit_ref() != reference {
            return Err(failure(
                "video_coverage.stale_unit_revision",
                format!("$.mappings[{mapping_index}].unit_ref.revision"),
            ));
        }
    }
    if raw.mappings.len() > raw.selected_unit_refs.len() {
        return Err(failure(
            "video_coverage.mapping_outside_selection",
            format!("$.mappings[{}].unit_ref", raw.selected_unit_refs.len()),
        ));
    }

    let mut results = Vec::with_capacity(raw.mappings.len());
    let mut diagnostics = Vec::new();
    for (index, mapping) in raw.mappings.iter().enumerate() {
        let mapping_path = format!("$.mappings[{index}]");
        let result = match mapping {
            VideoCoverageMappingV1::Mapped {
                unit_ref,
                asset_ref,
                source_range,
            } => {
                if !valid_id(&asset_ref.asset_id) || asset_ref.revision == 0 {
                    return Err(failure(
                        "video_coverage.invalid_shape",
                        format!("{mapping_path}.asset_ref"),
                    ));
                }
                let Some(asset) = assets.get(asset_ref.asset_id.as_str()) else {
                    return Err(failure(
                        "video_coverage.dangling_asset",
                        format!("{mapping_path}.asset_ref.asset_id"),
                    ));
                };
                if asset.revision != asset_ref.revision {
                    return Err(failure(
                        "video_coverage.stale_asset_revision",
                        format!("{mapping_path}.asset_ref.revision"),
                    ));
                }
                let start = exact_time(&source_range.start).map_err(|_| {
                    failure(
                        "video_coverage.invalid_range",
                        format!("{mapping_path}.source_range.start"),
                    )
                })?;
                let end = exact_time(&source_range.end).map_err(|_| {
                    failure(
                        "video_coverage.invalid_range",
                        format!("{mapping_path}.source_range.end"),
                    )
                })?;
                let Some(&duration) = durations.get(asset_ref.asset_id.as_str()) else {
                    return Err(failure(
                        "video_coverage.invalid_range",
                        format!("{mapping_path}.source_range.end"),
                    ));
                };
                if start >= end || end > duration {
                    return Err(failure(
                        "video_coverage.invalid_range",
                        format!("{mapping_path}.source_range.end"),
                    ));
                }
                VideoCoverageMappingResultV1 {
                    unit_ref: unit_ref.clone(),
                    status: "mapped",
                    asset_ref: Some(asset_ref.clone()),
                    source_range: Some(source_range.clone()),
                    reason: None,
                }
            }
            VideoCoverageMappingV1::Unresolved { unit_ref, reason } => {
                if reason.is_empty() {
                    return Err(failure(
                        "video_coverage.invalid_shape",
                        format!("{mapping_path}.reason"),
                    ));
                }
                if reason.chars().count() > 512 {
                    return Err(failure(
                        "video_coverage.too_many_items",
                        format!("{mapping_path}.reason"),
                    ));
                }
                diagnostics.push(VideoCoverageDiagnosticV1 {
                    code: "video_coverage.unresolved_required".to_owned(),
                    path: mapping_path,
                });
                VideoCoverageMappingResultV1 {
                    unit_ref: unit_ref.clone(),
                    status: "unresolved",
                    asset_ref: None,
                    source_range: None,
                    reason: Some(reason.clone()),
                }
            }
        };
        results.push(result);
    }

    Ok(NarrativeVideoCoverageResultV1 {
        schema_version: 1,
        profile: "storytelling.narrative-video-coverage.v1",
        coverage_id: raw.coverage_id,
        narrative_ref: NarrativeVideoCoverageReferenceV1 {
            document_id: narrative.document_id,
            document_revision: narrative.document_revision,
            variant_id: raw.variant_id,
        },
        asset_catalog_ref: VideoAssetCatalogReferenceV1 {
            catalog_id: raw.asset_catalog.catalog_id,
            revision: raw.asset_catalog.revision,
        },
        selected_unit_refs: raw.selected_unit_refs,
        ready_for_planning: diagnostics.is_empty(),
        mappings: results,
        diagnostics,
    })
}
