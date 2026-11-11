// SPDX-License-Identifier: MIT
//! Pure source inventory binding and component impact for a mixed-media handoff.
use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use storytelling_core::narrative::{NarrativeOriginV1, NarrativeValidationErrorV1};
use storytelling_core::narrative_handoff::{HandoffRepresentationV1, NarrativeHandoffManifestV1};

use crate::narrative::{check_id, check_text, error, preflight_json};
use crate::{MAX_INPUT_BYTES, NarrativeRevisionEventResultV1, track_narrative_revisions};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffSourceProjectionV1 {
    pub unit_id: String,
    pub revision: u32,
    pub representation: &'static str,
    pub detail: String,
    pub source_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffComponentProjectionV1 {
    pub id: String,
    pub affected: bool,
    pub references: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffArtifactProjectionV1 {
    pub id: String,
    pub name: String,
    pub medium: &'static str,
    pub variant_id: String,
    pub variant_revision: u32,
    pub status: String,
    pub content: String,
    pub components: Vec<HandoffComponentProjectionV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarrativeHandoffProjectionV1 {
    pub manifest_id: String,
    pub document_id: String,
    pub change: NarrativeRevisionEventResultV1,
    pub sources: Vec<HandoffSourceProjectionV1>,
    pub artifacts: Vec<HandoffArtifactProjectionV1>,
}

fn bounded(count: usize, max: usize, path: &str) -> Result<(), NarrativeValidationErrorV1> {
    if count == 0 || count > max {
        return Err(error("handoff.collection_limit", path));
    }
    Ok(())
}

fn parse(bytes: &[u8]) -> Result<NarrativeHandoffManifestV1, NarrativeValidationErrorV1> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    preflight_json(bytes).map_err(|cause| match cause.code.as_str() {
        "narrative.duplicate_field" => error("handoff.duplicate_field", cause.path),
        "narrative.depth_exceeded" => error("handoff.depth_exceeded", cause.path),
        _ => cause,
    })?;
    let root: Value =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    match root.get("schema_version").and_then(Value::as_u64) {
        Some(1) => {}
        Some(_) => return Err(error("handoff.unsupported_version", "$.schema_version")),
        None => return Err(error("handoff.invalid_shape", "$.schema_version")),
    }
    serde_json::from_slice(bytes).map_err(|cause| {
        if cause.to_string().starts_with("unknown field") {
            error("handoff.unknown_field", "$")
        } else {
            error("handoff.invalid_shape", "$")
        }
    })
}

fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && path.chars().count() <= 1024
        && !path.starts_with('/')
        && !path.contains(['\\', ':'])
        && !path.chars().any(char::is_control)
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

fn representation(
    source: &HandoffRepresentationV1,
    path: &str,
) -> Result<(&'static str, String), NarrativeValidationErrorV1> {
    match source {
        HandoffRepresentationV1::File {
            media_kind,
            relative_path,
            sha256,
        } => {
            if !safe_path(relative_path) {
                return Err(error(
                    "handoff.invalid_path",
                    format!("{path}.relative_path"),
                ));
            }
            if sha256.len() != 64
                || !sha256
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err(error("handoff.invalid_hash", format!("{path}.sha256")));
            }
            Ok((
                "file",
                format!(
                    "{} · {} · SHA-256 {} (declared, not verified)",
                    media_kind.as_str(),
                    relative_path,
                    sha256
                ),
            ))
        }
        HandoffRepresentationV1::Description { media_kind, text } => {
            check_text(text, 8192, &format!("{path}.text"))?;
            Ok((
                "description",
                format!("{} · supplied description: {text}", media_kind.as_str()),
            ))
        }
        HandoffRepresentationV1::Transcript { text } => {
            check_text(text, 8192, &format!("{path}.text"))?;
            Ok((
                "transcript",
                format!("supplied transcript; underlying recording not inspected: {text}"),
            ))
        }
        HandoffRepresentationV1::CreativeProposal { variant_id, text } => {
            check_id(variant_id, &format!("{path}.variant_id"))?;
            check_text(text, 8192, &format!("{path}.text"))?;
            Ok((
                "creative_proposal",
                format!("variant-only {variant_id}: {text}"),
            ))
        }
        HandoffRepresentationV1::Missing { media_kind, reason } => {
            check_text(reason, 8192, &format!("{path}.reason"))?;
            Ok((
                "missing",
                format!("{} pending: {reason}", media_kind.as_str()),
            ))
        }
    }
}

/// Reuse N-04's direct-reference impact, without opening media or changing the ledger.
pub fn render_narrative_handoff(
    bytes: &[u8],
) -> Result<NarrativeHandoffProjectionV1, NarrativeValidationErrorV1> {
    let manifest = parse(bytes)?;
    check_id(&manifest.manifest_id, "$.manifest_id")?;
    check_id(&manifest.change_id, "$.change_id")?;
    bounded(manifest.sources.len(), 256, "$.sources")?;
    bounded(manifest.artifacts.len(), 64, "$.artifacts")?;
    let ledger_bytes = serde_json::to_vec(&manifest.ledger)
        .map_err(|_| error("handoff.invalid_shape", "$.ledger"))?;
    let report = track_narrative_revisions(&ledger_bytes)?;
    let change = report
        .events
        .iter()
        .find(|event| event.id == manifest.change_id)
        .ok_or_else(|| error("revision.change_not_found", "$.change_id"))?;
    if change.kind != "source_revision" {
        return Err(error("revision.not_source_change", "$.change_id"));
    }
    let mut latest = BTreeMap::new();
    for source in &report.source_history {
        latest.insert(source.unit_id.as_str(), source);
    }
    let variants: BTreeMap<_, _> = report
        .current_variants
        .iter()
        .map(|v| (v.variant_id.as_str(), v))
        .collect();
    let mut sources = BTreeMap::new();
    let mut source_rows = Vec::new();
    for (index, source) in manifest.sources.iter().enumerate() {
        let path = format!("$.sources[{index}]");
        check_id(&source.unit_id, &format!("{path}.unit_id"))?;
        let current = latest
            .get(source.unit_id.as_str())
            .ok_or_else(|| error("handoff.dangling_reference", format!("{path}.unit_id")))?;
        if source.revision != current.revision {
            return Err(error("handoff.stale_revision", format!("{path}.revision")));
        }
        if sources.insert(source.unit_id.as_str(), source).is_some() {
            return Err(error("handoff.duplicate_id", format!("{path}.unit_id")));
        }
        match (&source.representation, current.origin) {
            (
                HandoffRepresentationV1::CreativeProposal { variant_id, .. },
                NarrativeOriginV1::CreativeProposal,
            ) => {
                if !variants.contains_key(variant_id.as_str()) {
                    return Err(error(
                        "handoff.dangling_reference",
                        format!("{path}.representation.variant_id"),
                    ));
                }
            }
            (HandoffRepresentationV1::CreativeProposal { .. }, _)
            | (_, NarrativeOriginV1::CreativeProposal) => {
                return Err(error(
                    "handoff.origin_mismatch",
                    format!("{path}.representation"),
                ));
            }
            _ => {}
        }
        let (kind, detail) =
            representation(&source.representation, &format!("{path}.representation"))?;
        source_rows.push(HandoffSourceProjectionV1 {
            unit_id: source.unit_id.clone(),
            revision: source.revision,
            representation: kind,
            detail,
            source_text: current.text.clone(),
        });
    }
    let mut artifact_ids = BTreeSet::new();
    let mut artifact_rows = Vec::new();
    for (index, artifact) in manifest.artifacts.iter().enumerate() {
        let path = format!("$.artifacts[{index}]");
        check_id(&artifact.id, &format!("{path}.id"))?;
        check_id(&artifact.variant_id, &format!("{path}.variant_id"))?;
        check_text(&artifact.name, 8192, &format!("{path}.name"))?;
        if !artifact_ids.insert(&artifact.id) {
            return Err(error("handoff.duplicate_id", format!("{path}.id")));
        }
        let variant = variants
            .get(artifact.variant_id.as_str())
            .ok_or_else(|| error("handoff.dangling_reference", format!("{path}.variant_id")))?;
        // Check the whole bound snapshot before considering the author's component grouping.
        // Omitting a reference from that grouping cannot conceal a scoped proposal.
        for reference in &variant.source_refs {
            let source = sources
                .get(reference.unit_id.as_str())
                .ok_or_else(|| error("handoff.dangling_reference", format!("{path}.variant_id")))?;
            if let HandoffRepresentationV1::CreativeProposal { variant_id, .. } =
                &source.representation
                && variant_id != &artifact.variant_id
            {
                return Err(error("handoff.variant_scope", format!("{path}.variant_id")));
            }
        }
        bounded(artifact.components.len(), 64, &format!("{path}.components"))?;
        let mut component_ids = BTreeSet::new();
        let mut covered_units = BTreeSet::new();
        let mut component_rows = Vec::new();
        for (component_index, component) in artifact.components.iter().enumerate() {
            let component_path = format!("{path}.components[{component_index}]");
            check_id(&component.id, &format!("{component_path}.id"))?;
            if !component_ids.insert(&component.id) {
                return Err(error(
                    "handoff.duplicate_id",
                    format!("{component_path}.id"),
                ));
            }
            bounded(
                component.unit_ids.len(),
                64,
                &format!("{component_path}.unit_ids"),
            )?;
            let mut unit_ids = BTreeSet::new();
            let mut references = Vec::new();
            for unit_id in &component.unit_ids {
                check_id(unit_id, &format!("{component_path}.unit_ids"))?;
                if !unit_ids.insert(unit_id) {
                    return Err(error(
                        "handoff.duplicate_id",
                        format!("{component_path}.unit_ids"),
                    ));
                }
                covered_units.insert(unit_id.as_str());
                let source = sources.get(unit_id.as_str()).ok_or_else(|| {
                    error(
                        "handoff.dangling_reference",
                        format!("{component_path}.unit_ids"),
                    )
                })?;
                if let HandoffRepresentationV1::CreativeProposal { variant_id, .. } =
                    &source.representation
                    && variant_id != &artifact.variant_id
                {
                    return Err(error(
                        "handoff.variant_scope",
                        format!("{component_path}.unit_ids"),
                    ));
                }
                let draft_ref = variant
                    .source_refs
                    .iter()
                    .find(|r| &r.unit_id == unit_id)
                    .ok_or_else(|| {
                        error(
                            "handoff.dangling_reference",
                            format!("{component_path}.unit_ids"),
                        )
                    })?;
                references.push(format!(
                    "{}@{} -> {}@{}",
                    unit_id, draft_ref.revision, unit_id, source.revision
                ));
            }
            let affected = change.affected_variants.contains(&artifact.variant_id)
                && change
                    .unit_id
                    .as_ref()
                    .is_some_and(|id| component.unit_ids.contains(id));
            component_rows.push(HandoffComponentProjectionV1 {
                id: component.id.clone(),
                affected,
                references,
            });
        }
        let variant_units: BTreeSet<_> = variant
            .source_refs
            .iter()
            .map(|r| r.unit_id.as_str())
            .collect();
        if covered_units != variant_units {
            return Err(error(
                "handoff.incomplete_mapping",
                format!("{path}.components"),
            ));
        }
        artifact_rows.push(HandoffArtifactProjectionV1 {
            id: artifact.id.clone(),
            name: artifact.name.clone(),
            medium: artifact.medium.as_str(),
            variant_id: artifact.variant_id.clone(),
            variant_revision: variant.revision,
            status: variant.status.clone(),
            content: variant.content.clone(),
            components: component_rows,
        });
    }
    Ok(NarrativeHandoffProjectionV1 {
        manifest_id: manifest.manifest_id,
        document_id: report.document_id,
        change: change.clone(),
        sources: source_rows,
        artifacts: artifact_rows,
    })
}
