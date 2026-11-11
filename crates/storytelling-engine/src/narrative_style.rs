// SPDX-License-Identifier: MIT
//! Pure validation for separately versioned editorial order profiles.
use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use storytelling_core::narrative::{
    NarrativeStyleCheckV1, NarrativeStyleProfileV1, NarrativeStyleValidationErrorV1,
};

use crate::narrative::{OrderEdge, first_order_cycle, preflight_json};
use crate::{
    MAX_INPUT_BYTES, NarrativeProjectedVariantV1, NarrativeProjectionV1, render_narrative,
};

const MAX_STYLE_RULES: usize = 4096;

fn error(code: &str, path: impl Into<String>) -> NarrativeStyleValidationErrorV1 {
    NarrativeStyleValidationErrorV1 {
        code: code.to_owned(),
        path: path.into(),
        rule_ids: Vec::new(),
    }
}

fn error_with_rules(
    code: &str,
    path: impl Into<String>,
    rule_ids: Vec<String>,
) -> NarrativeStyleValidationErrorV1 {
    NarrativeStyleValidationErrorV1 {
        code: code.to_owned(),
        path: path.into(),
        rule_ids,
    }
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

fn profile_version(bytes: &[u8]) -> Result<(), NarrativeStyleValidationErrorV1> {
    let root: Value =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    let Some(version) = root.get("schema_version") else {
        return Err(error("narrative.style.invalid_shape", "$.schema_version"));
    };
    let Some(number) = version.as_number() else {
        return Err(error("narrative.style.invalid_shape", "$.schema_version"));
    };
    let supported = number.as_i64() == Some(1) || number.as_u64() == Some(1);
    if supported {
        return Ok(());
    }
    if number.is_i64() || number.is_u64() {
        return Err(error(
            "narrative.style.unsupported_version",
            "$.schema_version",
        ));
    }
    Err(error("narrative.style.invalid_shape", "$.schema_version"))
}

fn parse_profile(bytes: &[u8]) -> Result<NarrativeStyleProfileV1, NarrativeStyleValidationErrorV1> {
    serde_json::from_slice(bytes).map_err(|cause| {
        let message = cause.to_string();
        let code = if message.starts_with("unknown field") {
            "narrative.style.unknown_field"
        } else if message.starts_with("duplicate field") {
            "narrative.style.duplicate_field"
        } else {
            "narrative.style.invalid_shape"
        };
        error(code, "$")
    })
}

fn variant_maps(
    projection: &NarrativeProjectionV1,
) -> (BTreeMap<&str, usize>, Vec<BTreeMap<&str, usize>>) {
    let mut variants = BTreeMap::new();
    let mut beats = Vec::with_capacity(projection.variants.len());
    for (variant_index, variant) in projection.variants.iter().enumerate() {
        let mut positions = BTreeMap::new();
        match variant {
            NarrativeProjectedVariantV1::SpokenOutline { id, beats, .. } => {
                variants.insert(id.as_str(), variant_index);
                for (beat_index, beat) in beats.iter().enumerate() {
                    positions.insert(beat.id.as_str(), beat_index);
                }
            }
            NarrativeProjectedVariantV1::SingleIllustration { id, .. } => {
                variants.insert(id.as_str(), variant_index);
            }
        }
        beats.push(positions);
    }
    (variants, beats)
}

/// Check a separately versioned order profile against a closed NarrativeDocumentV1.
/// Only declared spoken-beat order is checked; this does not assess style or meaning.
pub fn check_narrative_style_profile(
    document_bytes: &[u8],
    profile_bytes: &[u8],
) -> Result<NarrativeStyleCheckV1, NarrativeStyleValidationErrorV1> {
    let projection =
        render_narrative(document_bytes).map_err(|cause| error(&cause.code, cause.path))?;
    if profile_bytes.len() > MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    preflight_json(profile_bytes).map_err(|cause| {
        let code = match cause.code.as_str() {
            "narrative.duplicate_field" => "narrative.style.duplicate_field",
            "narrative.depth_exceeded" => "narrative.style.depth_exceeded",
            _ => cause.code.as_str(),
        };
        error(code, cause.path)
    })?;
    profile_version(profile_bytes)?;
    let profile = parse_profile(profile_bytes)?;

    if !valid_id(&profile.profile_id) {
        return Err(error("narrative.style.invalid_id", "$.profile_id"));
    }
    if !valid_id(&profile.document_id) {
        return Err(error("narrative.style.invalid_id", "$.document_id"));
    }
    if profile.document_revision == 0 {
        return Err(error(
            "narrative.style.invalid_shape",
            "$.document_revision",
        ));
    }
    if profile.document_id != projection.document_id {
        return Err(error("narrative.style.document_mismatch", "$.document_id"));
    }
    if profile.document_revision != projection.document_revision {
        return Err(error(
            "narrative.style.stale_document",
            "$.document_revision",
        ));
    }
    if profile.rules.is_empty() {
        return Err(error("narrative.style.invalid_shape", "$.rules"));
    }
    if profile.rules.len() > MAX_STYLE_RULES {
        return Err(error("narrative.style.too_many_rules", "$.rules"));
    }

    let (variant_indexes, beat_positions) = variant_maps(&projection);
    let mut rule_ids = BTreeSet::new();
    let mut edges_by_variant = vec![Vec::new(); projection.variants.len()];
    for (rule_index, rule) in profile.rules.iter().enumerate() {
        let path = format!("$.rules[{rule_index}]");
        if !valid_id(&rule.id) {
            return Err(error("narrative.style.invalid_id", format!("{path}.id")));
        }
        if !rule_ids.insert(rule.id.as_str()) {
            return Err(error("narrative.style.duplicate_id", format!("{path}.id")));
        }
        if !valid_id(&rule.variant_id) {
            return Err(error(
                "narrative.style.invalid_id",
                format!("{path}.variant_id"),
            ));
        }
        if !valid_id(&rule.before_beat_id) {
            return Err(error(
                "narrative.style.invalid_id",
                format!("{path}.before_beat_id"),
            ));
        }
        if !valid_id(&rule.after_beat_id) {
            return Err(error(
                "narrative.style.invalid_id",
                format!("{path}.after_beat_id"),
            ));
        }
        let Some(&variant_index) = variant_indexes.get(rule.variant_id.as_str()) else {
            return Err(error(
                "narrative.style.dangling_reference",
                format!("{path}.variant_id"),
            ));
        };
        let NarrativeProjectedVariantV1::SpokenOutline { beats, .. } =
            &projection.variants[variant_index]
        else {
            return Err(error(
                "narrative.style.invalid_rule_scope",
                format!("{path}.variant_id"),
            ));
        };
        let positions = &beat_positions[variant_index];
        let Some(&before) = positions.get(rule.before_beat_id.as_str()) else {
            return Err(error(
                "narrative.style.dangling_reference",
                format!("{path}.before_beat_id"),
            ));
        };
        let Some(&after) = positions.get(rule.after_beat_id.as_str()) else {
            return Err(error(
                "narrative.style.dangling_reference",
                format!("{path}.after_beat_id"),
            ));
        };
        if before == after {
            return Err(error(
                "narrative.style.invalid_rule",
                format!("{path}.after_beat_id"),
            ));
        }
        edges_by_variant[variant_index].push(OrderEdge {
            before,
            after,
            relation_index: rule_index,
        });
        debug_assert_eq!(beats.len(), positions.len());
    }

    for (variant_index, variant) in projection.variants.iter().enumerate() {
        let NarrativeProjectedVariantV1::SpokenOutline { beats, .. } = variant else {
            continue;
        };
        let edges = &edges_by_variant[variant_index];
        if let Some(cycle) = first_order_cycle(beats.len(), edges) {
            let cycle_edges: BTreeSet<_> = cycle.into_iter().collect();
            let cycle_rule_ids = edges
                .iter()
                .enumerate()
                .filter(|(edge_index, _)| cycle_edges.contains(edge_index))
                .map(|(_, edge)| profile.rules[edge.relation_index].id.clone())
                .collect();
            return Err(error_with_rules(
                "narrative.style.order_conflict",
                "$.rules",
                cycle_rule_ids,
            ));
        }
        for edge in edges {
            if edge.before > edge.after {
                let rule = &profile.rules[edge.relation_index];
                return Err(error_with_rules(
                    "narrative.style.order_violation",
                    format!("$.rules[{}]", edge.relation_index),
                    vec![rule.id.clone()],
                ));
            }
        }
    }

    Ok(NarrativeStyleCheckV1 {
        profile_id: profile.profile_id,
        document_id: projection.document_id,
        document_revision: projection.document_revision,
        checked_rules: profile.rules.len(),
        scope: "declared-beat-order-only",
    })
}
