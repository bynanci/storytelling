// SPDX-License-Identifier: MIT
//! Pure bounded validation and deterministic presentation of NarrativeDocumentV1.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;
use serde_json::{Number, Value};
use storytelling_core::narrative::{
    NarrativeDocumentV1, NarrativeOriginV1, NarrativeProjectedBeatV1, NarrativeProjectedVariantV1,
    NarrativeProjectionV1, NarrativeRelationV1, NarrativeSourceTagV1, NarrativeUnitReferenceV1,
    NarrativeValidationErrorV1, NarrativeVariantV1,
};

const MAX_NESTING: usize = 32;
const MAX_UNITS: usize = 512;
const MAX_RELATIONS: usize = 4096;
const MAX_VARIANTS: usize = 16;
const MAX_BEATS: usize = 512;
const MAX_BASES: usize = 32;
const MAX_ILLUSTRATION_REFS: usize = 64;
const MAX_ID_BYTES: usize = 128;
const MAX_PURPOSE_AUDIENCE_CHARS: usize = 512;
const MAX_NAME_CHARS: usize = 128;
pub(crate) const MAX_TEXT_CHARS: usize = 8192;

#[derive(Clone, Copy)]
enum PreflightFailure {
    DuplicateField,
    DepthExceeded,
}

#[derive(Default)]
struct JsonPreflight {
    failure: Option<PreflightFailure>,
}

impl JsonPreflight {
    fn scan(&mut self, raw: &RawValue, depth: usize) -> Result<(), serde_json::Error> {
        match raw.get().as_bytes()[0] {
            b'{' => {
                if depth >= MAX_NESTING {
                    self.failure = Some(PreflightFailure::DepthExceeded);
                    return Err(de::Error::custom("narrative nesting limit exceeded"));
                }
                let mut deserializer = serde_json::Deserializer::from_str(raw.get());
                de::Deserializer::deserialize_map(
                    &mut deserializer,
                    ObjectVisitor {
                        preflight: self,
                        depth: depth + 1,
                    },
                )?;
                deserializer.end()
            }
            b'[' => {
                if depth >= MAX_NESTING {
                    self.failure = Some(PreflightFailure::DepthExceeded);
                    return Err(de::Error::custom("narrative nesting limit exceeded"));
                }
                let mut deserializer = serde_json::Deserializer::from_str(raw.get());
                de::Deserializer::deserialize_seq(
                    &mut deserializer,
                    ArrayVisitor {
                        preflight: self,
                        depth: depth + 1,
                    },
                )?;
                deserializer.end()
            }
            _ => Ok(()),
        }
    }
}

pub(crate) fn preflight_json(bytes: &[u8]) -> Result<(), NarrativeValidationErrorV1> {
    let raw: &RawValue =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    let mut preflight = JsonPreflight::default();
    if preflight.scan(raw, 0).is_err() {
        return Err(match preflight.failure {
            Some(PreflightFailure::DuplicateField) => error("narrative.duplicate_field", "$"),
            Some(PreflightFailure::DepthExceeded) => error("narrative.depth_exceeded", "$"),
            None => error("input.invalid_json", "$"),
        });
    }
    Ok(())
}

struct ObjectVisitor<'a> {
    preflight: &'a mut JsonPreflight,
    depth: usize,
}

impl<'de> Visitor<'de> for ObjectVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<(), A::Error> {
        let mut keys = BTreeSet::new();
        while let Some(key) = object.next_key::<String>()? {
            if !keys.insert(key) {
                self.preflight.failure = Some(PreflightFailure::DuplicateField);
                return Err(de::Error::custom("duplicate JSON object key"));
            }
            let value: &RawValue = object.next_value()?;
            self.preflight.scan(value, self.depth).map_err(|_| {
                <A::Error as de::Error>::custom("narrative JSON preflight rejected")
            })?;
        }
        Ok(())
    }
}

struct ArrayVisitor<'a> {
    preflight: &'a mut JsonPreflight,
    depth: usize,
}

impl<'de> Visitor<'de> for ArrayVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON array")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut array: A) -> Result<(), A::Error> {
        while let Some(value) = array.next_element::<&RawValue>()? {
            self.preflight.scan(value, self.depth).map_err(|_| {
                <A::Error as de::Error>::custom("narrative JSON preflight rejected")
            })?;
        }
        Ok(())
    }
}

pub(crate) fn error(code: &str, path: impl Into<String>) -> NarrativeValidationErrorV1 {
    NarrativeValidationErrorV1::new(code, path)
}

pub(crate) fn version(bytes: &[u8]) -> Result<(), NarrativeValidationErrorV1> {
    let root: Value =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    let Some(value) = root.get("schema_version") else {
        return Err(error("narrative.invalid_shape", "$.schema_version"));
    };
    let Some(number) = value.as_number() else {
        return Err(error("narrative.invalid_shape", "$.schema_version"));
    };
    match integer_value(number) {
        Some(1) => Ok(()),
        Some(_) => Err(error("narrative.unsupported_version", "$.schema_version")),
        None => Err(error("narrative.invalid_shape", "$.schema_version")),
    }
}

fn integer_value(number: &Number) -> Option<i128> {
    number
        .as_i64()
        .map(i128::from)
        .or_else(|| number.as_u64().map(i128::from))
}

fn parse(bytes: &[u8]) -> Result<NarrativeDocumentV1, NarrativeValidationErrorV1> {
    match serde_json::from_slice(bytes) {
        Ok(document) => Ok(document),
        Err(cause) => {
            let message = cause.to_string();
            if message.starts_with("unknown field") {
                Err(error("narrative.unknown_field", "$"))
            } else if message.starts_with("duplicate field") {
                Err(error("narrative.duplicate_field", "$"))
            } else {
                Err(error("narrative.invalid_shape", "$"))
            }
        }
    }
}

pub(crate) fn check_id(value: &str, path: &str) -> Result<(), NarrativeValidationErrorV1> {
    let bytes = value.as_bytes();
    if bytes.len() > MAX_ID_BYTES {
        return Err(error("narrative.too_many_items", path));
    }
    if bytes.is_empty()
        || !bytes[0].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(byte))
    {
        return Err(error("narrative.invalid_shape", path));
    }
    Ok(())
}

pub(crate) fn check_text(
    value: &str,
    max_chars: usize,
    path: &str,
) -> Result<(), NarrativeValidationErrorV1> {
    if value.is_empty() {
        return Err(error("narrative.invalid_shape", path));
    }
    if value.chars().count() > max_chars {
        return Err(error("narrative.too_many_items", path));
    }
    Ok(())
}

fn resolve_reference(
    reference: &NarrativeUnitReferenceV1,
    path: &str,
    document: &NarrativeDocumentV1,
    units: &BTreeMap<&str, usize>,
) -> Result<NarrativeSourceTagV1, NarrativeValidationErrorV1> {
    let id_path = format!("{path}.unit_id");
    check_id(&reference.unit_id, &id_path)?;
    let Some(index) = units.get(reference.unit_id.as_str()).copied() else {
        return Err(error("narrative.dangling_reference", id_path));
    };
    let unit = &document.units[index];
    if unit.revision != reference.revision {
        return Err(error(
            "narrative.stale_revision",
            format!("{path}.revision"),
        ));
    }
    Ok(NarrativeSourceTagV1 {
        unit_id: unit.id.clone(),
        revision: unit.revision,
        origin: unit.origin,
    })
}

fn detect_derivation_cycle(edges: &[Vec<(usize, String)>]) -> Option<String> {
    fn visit(node: usize, edges: &[Vec<(usize, String)>], states: &mut [u8]) -> Option<String> {
        states[node] = 1;
        for (target, path) in &edges[node] {
            match states[*target] {
                1 => return Some(path.clone()),
                0 => {
                    if let Some(path) = visit(*target, edges, states) {
                        return Some(path);
                    }
                }
                _ => {}
            }
        }
        states[node] = 2;
        None
    }

    let mut states = vec![0; edges.len()];
    for node in 0..edges.len() {
        if states[node] == 0
            && let Some(path) = visit(node, edges, &mut states)
        {
            return Some(path);
        }
    }
    None
}

#[derive(Clone, Copy)]
pub(super) struct OrderEdge {
    before: usize,
    after: usize,
    relation_index: usize,
}

pub(super) fn first_order_cycle(beat_count: usize, edges: &[OrderEdge]) -> Option<Vec<usize>> {
    fn visit(
        node: usize,
        outgoing: &[Vec<usize>],
        edges: &[OrderEdge],
        states: &mut [u8],
        nodes: &mut Vec<usize>,
        path_edges: &mut Vec<usize>,
    ) -> Option<Vec<usize>> {
        states[node] = 1;
        nodes.push(node);
        for &edge_index in &outgoing[node] {
            let target = edges[edge_index].after;
            match states[target] {
                1 => {
                    let cycle_start = nodes.iter().position(|seen| *seen == target)?;
                    let mut cycle = path_edges[cycle_start..].to_vec();
                    cycle.push(edge_index);
                    return Some(cycle);
                }
                0 => {
                    path_edges.push(edge_index);
                    if let Some(cycle) = visit(target, outgoing, edges, states, nodes, path_edges) {
                        return Some(cycle);
                    }
                    path_edges.pop();
                }
                _ => {}
            }
        }
        nodes.pop();
        states[node] = 2;
        None
    }

    let mut outgoing = vec![Vec::new(); beat_count];
    for (edge_index, edge) in edges.iter().enumerate() {
        outgoing[edge.before].push(edge_index);
    }
    let mut states = vec![0; beat_count];
    let mut nodes = Vec::new();
    let mut path_edges = Vec::new();
    for node in 0..beat_count {
        if states[node] == 0
            && let Some(cycle) = visit(
                node,
                &outgoing,
                edges,
                &mut states,
                &mut nodes,
                &mut path_edges,
            )
        {
            return Some(cycle);
        }
    }
    None
}

/// Validate and deterministically project the declared oral and illustration variants.
/// This performs no media access, generation, file I/O, or external calls.
pub fn render_narrative(bytes: &[u8]) -> Result<NarrativeProjectionV1, NarrativeValidationErrorV1> {
    if bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    preflight_json(bytes)?;
    version(bytes)?;
    let document = parse(bytes)?;

    check_id(&document.document_id, "$.document_id")?;
    if document.document_revision == 0 {
        return Err(error("narrative.invalid_shape", "$.document_revision"));
    }
    check_text(&document.purpose, MAX_PURPOSE_AUDIENCE_CHARS, "$.purpose")?;
    check_text(&document.audience, MAX_PURPOSE_AUDIENCE_CHARS, "$.audience")?;
    if document.units.is_empty() {
        return Err(error("narrative.invalid_shape", "$.units"));
    }
    if document.units.len() > MAX_UNITS {
        return Err(error("narrative.too_many_items", "$.units"));
    }
    let mut units = BTreeMap::new();
    for (index, unit) in document.units.iter().enumerate() {
        let path = format!("$.units[{index}]");
        check_id(&unit.id, &format!("{path}.id"))?;
        if units.insert(unit.id.as_str(), index).is_some() {
            return Err(error("narrative.duplicate_id", format!("{path}.id")));
        }
        if unit.revision == 0 {
            return Err(error("narrative.invalid_shape", format!("{path}.revision")));
        }
        check_text(&unit.text, MAX_TEXT_CHARS, &format!("{path}.text"))?;
        if unit.based_on.len() > MAX_BASES {
            return Err(error(
                "narrative.too_many_items",
                format!("{path}.based_on"),
            ));
        }
        for (basis_index, reference) in unit.based_on.iter().enumerate() {
            check_id(
                &reference.unit_id,
                &format!("{path}.based_on[{basis_index}].unit_id"),
            )?;
        }
    }

    if document.relations.len() > MAX_RELATIONS {
        return Err(error("narrative.too_many_items", "$.relations"));
    }
    let mut relation_ids = BTreeSet::new();
    for (relation_index, relation) in document.relations.iter().enumerate() {
        let path = format!("$.relations[{relation_index}]");
        check_id(relation.id(), &format!("{path}.id"))?;
        if !relation_ids.insert(relation.id()) {
            return Err(error("narrative.duplicate_id", format!("{path}.id")));
        }
        match relation {
            NarrativeRelationV1::Association { from, to, .. } => {
                check_id(&from.unit_id, &format!("{path}.from.unit_id"))?;
                check_id(&to.unit_id, &format!("{path}.to.unit_id"))?;
            }
            NarrativeRelationV1::MustPrecede {
                variant_id,
                before,
                after,
                ..
            } => {
                check_id(variant_id, &format!("{path}.variant_id"))?;
                check_id(&before.unit_id, &format!("{path}.before.unit_id"))?;
                check_id(&after.unit_id, &format!("{path}.after.unit_id"))?;
            }
        }
    }

    if document.variants.is_empty() {
        return Err(error("narrative.invalid_shape", "$.variants"));
    }
    if document.variants.len() > MAX_VARIANTS {
        return Err(error("narrative.too_many_items", "$.variants"));
    }
    let mut variants = BTreeMap::new();
    for (variant_index, variant) in document.variants.iter().enumerate() {
        let path = format!("$.variants[{variant_index}]");
        check_id(variant.id(), &format!("{path}.id"))?;
        if variants.insert(variant.id(), variant_index).is_some() {
            return Err(error("narrative.duplicate_id", format!("{path}.id")));
        }
        match variant {
            NarrativeVariantV1::SpokenOutline { name, beats, .. } => {
                check_text(name, MAX_NAME_CHARS, &format!("{path}.name"))?;
                if beats.is_empty() {
                    return Err(error("narrative.invalid_shape", format!("{path}.beats")));
                }
                if beats.len() > MAX_BEATS {
                    return Err(error("narrative.too_many_items", format!("{path}.beats")));
                }
                let mut beat_ids = BTreeSet::new();
                for (beat_index, beat) in beats.iter().enumerate() {
                    let beat_path = format!("{path}.beats[{beat_index}]");
                    check_id(&beat.id, &format!("{beat_path}.id"))?;
                    if !beat_ids.insert(beat.id.as_str()) {
                        return Err(error("narrative.duplicate_id", format!("{beat_path}.id")));
                    }
                    check_id(
                        &beat.unit_ref.unit_id,
                        &format!("{beat_path}.unit_ref.unit_id"),
                    )?;
                    check_text(&beat.text, MAX_TEXT_CHARS, &format!("{beat_path}.text"))?;
                }
            }
            NarrativeVariantV1::SingleIllustration {
                name,
                unit_refs,
                brief,
                ..
            } => {
                check_text(name, MAX_NAME_CHARS, &format!("{path}.name"))?;
                if unit_refs.is_empty() {
                    return Err(error(
                        "narrative.invalid_shape",
                        format!("{path}.unit_refs"),
                    ));
                }
                if unit_refs.len() > MAX_ILLUSTRATION_REFS {
                    return Err(error(
                        "narrative.too_many_items",
                        format!("{path}.unit_refs"),
                    ));
                }
                for (reference_index, reference) in unit_refs.iter().enumerate() {
                    check_id(
                        &reference.unit_id,
                        &format!("{path}.unit_refs[{reference_index}].unit_id"),
                    )?;
                }
                check_text(brief, MAX_TEXT_CHARS, &format!("{path}.brief"))?;
            }
        }
    }

    let mut derivation_edges = vec![Vec::new(); document.units.len()];
    for (index, unit) in document.units.iter().enumerate() {
        for (basis_index, reference) in unit.based_on.iter().enumerate() {
            let path = format!("$.units[{index}].based_on[{basis_index}]");
            let tag = resolve_reference(reference, &path, &document, &units)?;
            derivation_edges[index].push((units[tag.unit_id.as_str()], path));
        }
    }
    for (relation_index, relation) in document.relations.iter().enumerate() {
        let path = format!("$.relations[{relation_index}]");
        match relation {
            NarrativeRelationV1::Association { from, to, .. } => {
                resolve_reference(from, &format!("{path}.from"), &document, &units)?;
                resolve_reference(to, &format!("{path}.to"), &document, &units)?;
            }
            NarrativeRelationV1::MustPrecede { before, after, .. } => {
                resolve_reference(before, &format!("{path}.before"), &document, &units)?;
                resolve_reference(after, &format!("{path}.after"), &document, &units)?;
            }
        }
    }

    let mut oral_positions = vec![None; document.variants.len()];
    for (variant_index, variant) in document.variants.iter().enumerate() {
        let path = format!("$.variants[{variant_index}]");
        match variant {
            NarrativeVariantV1::SpokenOutline { beats, .. } => {
                let mut positions = BTreeMap::new();
                for (beat_index, beat) in beats.iter().enumerate() {
                    let beat_path = format!("{path}.beats[{beat_index}]");
                    resolve_reference(
                        &beat.unit_ref,
                        &format!("{beat_path}.unit_ref"),
                        &document,
                        &units,
                    )?;
                    if positions
                        .insert(beat.unit_ref.unit_id.as_str(), beat_index)
                        .is_some()
                    {
                        return Err(error(
                            "narrative.duplicate_reference",
                            format!("{beat_path}.unit_ref.unit_id"),
                        ));
                    }
                }
                oral_positions[variant_index] = Some(positions);
            }
            NarrativeVariantV1::SingleIllustration { unit_refs, .. } => {
                let mut references = BTreeSet::new();
                for (reference_index, reference) in unit_refs.iter().enumerate() {
                    let reference_path = format!("{path}.unit_refs[{reference_index}]");
                    resolve_reference(reference, &reference_path, &document, &units)?;
                    if !references.insert(reference.unit_id.as_str()) {
                        return Err(error(
                            "narrative.duplicate_reference",
                            format!("{reference_path}.unit_id"),
                        ));
                    }
                }
            }
        }
    }

    // Origin/basis checks and derivation cycle detection follow all reference checks.
    for (index, unit) in document.units.iter().enumerate() {
        let path = format!("$.units[{index}].based_on");
        match unit.origin {
            NarrativeOriginV1::SuppliedObservation if !unit.based_on.is_empty() => {
                return Err(error("narrative.observation_has_basis", path));
            }
            NarrativeOriginV1::AuthorInterpretation | NarrativeOriginV1::CreativeProposal
                if unit.based_on.is_empty() =>
            {
                return Err(error("narrative.missing_basis", path));
            }
            _ => {}
        }
    }
    if let Some(path) = detect_derivation_cycle(&derivation_edges) {
        return Err(error("narrative.derivation_cycle", path));
    }

    let mut order_edges = vec![Vec::<OrderEdge>::new(); document.variants.len()];
    for (relation_index, relation) in document.relations.iter().enumerate() {
        let path = format!("$.relations[{relation_index}]");
        if let NarrativeRelationV1::MustPrecede {
            variant_id,
            before,
            after,
            ..
        } = relation
        {
            let Some(variant_index) = variants.get(variant_id.as_str()).copied() else {
                return Err(error(
                    "narrative.dangling_reference",
                    format!("{path}.variant_id"),
                ));
            };
            let Some(positions) = &oral_positions[variant_index] else {
                return Err(error("narrative.invalid_relation_scope", path));
            };
            if before.unit_id == after.unit_id {
                return Err(error("narrative.invalid_relation_scope", path));
            }
            let Some(&before_position) = positions.get(before.unit_id.as_str()) else {
                return Err(error("narrative.invalid_relation_scope", path));
            };
            let Some(&after_position) = positions.get(after.unit_id.as_str()) else {
                return Err(error("narrative.invalid_relation_scope", path));
            };
            order_edges[variant_index].push(OrderEdge {
                before: before_position,
                after: after_position,
                relation_index,
            });
        }
    }

    for (variant_index, edges) in order_edges.iter().enumerate() {
        if edges.is_empty() {
            continue;
        }
        let beat_count = oral_positions[variant_index]
            .as_ref()
            .map(BTreeMap::len)
            .unwrap_or_default();
        if let Some(mut cycle) = first_order_cycle(beat_count, edges) {
            cycle.sort_unstable_by_key(|edge_index| edges[*edge_index].relation_index);
            let mut relation_indices: Vec<usize> = cycle
                .iter()
                .map(|edge_index| edges[*edge_index].relation_index)
                .collect();
            relation_indices.sort_unstable();
            relation_indices.dedup();
            let ids = relation_indices
                .iter()
                .map(|index| document.relations[*index].id().to_owned())
                .collect();
            let first = relation_indices[0];
            return Err(NarrativeValidationErrorV1::with_relation_ids(
                "narrative.order_conflict",
                format!("$.relations[{first}]"),
                ids,
            ));
        }
        for edge in edges {
            if edge.before >= edge.after {
                return Err(error(
                    "narrative.order_violation",
                    format!("$.relations[{}]", edge.relation_index),
                ));
            }
        }
    }

    let mut projected = Vec::with_capacity(document.variants.len());
    for variant in &document.variants {
        match variant {
            NarrativeVariantV1::SpokenOutline { id, name, beats } => {
                let mut projected_beats = Vec::with_capacity(beats.len());
                for beat in beats {
                    let unit_index = units[beat.unit_ref.unit_id.as_str()];
                    let unit = &document.units[unit_index];
                    projected_beats.push(NarrativeProjectedBeatV1 {
                        id: beat.id.clone(),
                        source: NarrativeSourceTagV1 {
                            unit_id: unit.id.clone(),
                            revision: unit.revision,
                            origin: unit.origin,
                        },
                        text: beat.text.clone(),
                    });
                }
                projected.push(NarrativeProjectedVariantV1::SpokenOutline {
                    id: id.clone(),
                    name: name.clone(),
                    beats: projected_beats,
                });
            }
            NarrativeVariantV1::SingleIllustration {
                id,
                name,
                unit_refs,
                brief,
            } => {
                let sources = unit_refs
                    .iter()
                    .map(|reference| {
                        let unit = &document.units[units[reference.unit_id.as_str()]];
                        NarrativeSourceTagV1 {
                            unit_id: unit.id.clone(),
                            revision: unit.revision,
                            origin: unit.origin,
                        }
                    })
                    .collect();
                projected.push(NarrativeProjectedVariantV1::SingleIllustration {
                    id: id.clone(),
                    name: name.clone(),
                    sources,
                    brief: brief.clone(),
                });
            }
        }
    }

    Ok(NarrativeProjectionV1 {
        schema_version: document.schema_version,
        document_id: document.document_id,
        document_revision: document.document_revision,
        purpose: document.purpose,
        audience: document.audience,
        variants: projected,
    })
}
