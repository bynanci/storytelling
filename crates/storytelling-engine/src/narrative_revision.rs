// SPDX-License-Identifier: MIT
//! Pure append-only source revision and variant-impact tracking.
use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use storytelling_core::narrative::{
    NarrativeOriginV1, NarrativeUnitReferenceV1, NarrativeValidationErrorV1, NarrativeVariantV1,
};
use storytelling_core::narrative_revision::{
    NarrativeAuthorDecisionRecordV1, NarrativeAuthorDecisionV1, NarrativeCurrentVariantV1,
    NarrativeRevisionEventResultV1, NarrativeRevisionEventV1, NarrativeRevisionLedgerV1,
    NarrativeRevisionReportV1, NarrativeSourceRevisionV1, NarrativeSuggestionRecordV1,
    NarrativeVariantSnapshotV1,
};

use crate::MAX_INPUT_BYTES;
use crate::narrative::{
    MAX_TEXT_CHARS, check_id, check_text, error, preflight_json, render_narrative, version,
};

const MAX_EVENTS: usize = 128;
const MAX_VARIANT_REFS: usize = 64;

#[derive(Clone)]
struct ActiveVariant {
    id: String,
    kind: String,
    name: String,
    revision: u32,
    source_refs: Vec<NarrativeUnitReferenceV1>,
    content: String,
    stale_source_refs: Vec<NarrativeUnitReferenceV1>,
}

fn parse_ledger(bytes: &[u8]) -> Result<NarrativeRevisionLedgerV1, NarrativeValidationErrorV1> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    preflight_json(bytes)?;
    version(bytes)?;
    let root: Value =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    if let Some(events) = root.get("events").and_then(Value::as_array) {
        for (index, event) in events.iter().enumerate() {
            if event.get("kind").and_then(Value::as_str) == Some("assistant_suggestion")
                && let Some(fields) = event.as_object()
                && fields.keys().any(|field| {
                    matches!(
                        field.as_str(),
                        "decision"
                            | "author_decision"
                            | "accepted"
                            | "author_confirmed"
                            | "confirmed_by_author"
                            | "accepted_by_author"
                    )
                })
            {
                return Err(error(
                    "revision.untrusted_decision",
                    format!("$.events[{index}].decision"),
                ));
            }
        }
    }
    match serde_json::from_slice(bytes) {
        Ok(ledger) => Ok(ledger),
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

fn source_record(
    unit_id: &str,
    revision: u32,
    origin: NarrativeOriginV1,
    text: &str,
    based_on: &[NarrativeUnitReferenceV1],
) -> NarrativeSourceRevisionV1 {
    NarrativeSourceRevisionV1 {
        unit_id: unit_id.to_owned(),
        revision,
        origin,
        text: text.to_owned(),
        based_on: based_on.to_vec(),
    }
}

fn source_refs(
    variant: &NarrativeVariantV1,
) -> (String, String, Vec<NarrativeUnitReferenceV1>, String) {
    match variant {
        NarrativeVariantV1::SpokenOutline { id, beats, .. } => (
            id.clone(),
            "spoken_outline".to_owned(),
            beats.iter().map(|beat| beat.unit_ref.clone()).collect(),
            beats
                .iter()
                .map(|beat| format!("[{}] {}", beat.unit_ref.unit_id, beat.text))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        NarrativeVariantV1::SingleIllustration {
            id,
            unit_refs,
            brief,
            ..
        } => (
            id.clone(),
            "single_illustration".to_owned(),
            unit_refs.clone(),
            brief.clone(),
        ),
    }
}

fn snapshot(state: &ActiveVariant) -> NarrativeVariantSnapshotV1 {
    NarrativeVariantSnapshotV1 {
        variant_id: state.id.clone(),
        revision: state.revision,
        kind: state.kind.clone(),
        name: state.name.clone(),
        source_refs: state.source_refs.clone(),
        content: state.content.clone(),
    }
}

fn current_variant(state: &ActiveVariant) -> NarrativeCurrentVariantV1 {
    NarrativeCurrentVariantV1 {
        variant_id: state.id.clone(),
        revision: state.revision,
        status: if state.stale_source_refs.is_empty() {
            "current".to_owned()
        } else {
            "needs_review".to_owned()
        },
        stale_source_refs: state.stale_source_refs.clone(),
        source_refs: state.source_refs.clone(),
        content: state.content.clone(),
    }
}

fn require_source<'a>(
    sources: &'a BTreeMap<String, NarrativeSourceRevisionV1>,
    reference: &NarrativeUnitReferenceV1,
    path: &str,
) -> Result<&'a NarrativeSourceRevisionV1, NarrativeValidationErrorV1> {
    check_id(&reference.unit_id, &format!("{path}.unit_id"))?;
    let Some(source) = sources.get(&reference.unit_id) else {
        return Err(error(
            "narrative.dangling_reference",
            format!("{path}.unit_id"),
        ));
    };
    if source.revision != reference.revision {
        return Err(error(
            "narrative.stale_revision",
            format!("{path}.revision"),
        ));
    }
    Ok(source)
}

fn variant_index(
    states: &[ActiveVariant],
    id: &str,
    path: &str,
) -> Result<usize, NarrativeValidationErrorV1> {
    check_id(id, path)?;
    states
        .iter()
        .position(|variant| variant.id == id)
        .ok_or_else(|| error("narrative.dangling_reference", path))
}

fn next_variant_revision(
    state: &ActiveVariant,
    path: &str,
) -> Result<u32, NarrativeValidationErrorV1> {
    state
        .revision
        .checked_add(1)
        .ok_or_else(|| error("revision.too_many_versions", path))
}

fn push_snapshot(history: &mut Vec<NarrativeVariantSnapshotV1>, state: &ActiveVariant) {
    history.push(snapshot(state));
}

fn event_result(
    id: &str,
    kind: &str,
    unit_id: Option<&str>,
    from_revision: Option<u32>,
    to_revision: Option<u32>,
    variant_id: Option<&str>,
    affected_variants: Vec<String>,
) -> NarrativeRevisionEventResultV1 {
    NarrativeRevisionEventResultV1 {
        id: id.to_owned(),
        kind: kind.to_owned(),
        unit_id: unit_id.map(str::to_owned),
        from_revision,
        to_revision,
        variant_id: variant_id.map(str::to_owned),
        affected_variants,
    }
}

fn has_pending_review(states: &[ActiveVariant]) -> bool {
    states
        .iter()
        .any(|variant| !variant.stale_source_refs.is_empty())
}

fn ensure_creative_proposal(
    sources: &BTreeMap<String, NarrativeSourceRevisionV1>,
    proposal_ref: &NarrativeUnitReferenceV1,
    path: &str,
) -> Result<(), NarrativeValidationErrorV1> {
    let source = require_source(sources, proposal_ref, path)?;
    if source.origin != NarrativeOriginV1::CreativeProposal {
        return Err(error("revision.decision_not_creative_proposal", path));
    }
    Ok(())
}

fn review_variant(
    id: &str,
    variant_id: &str,
    refs: &[NarrativeUnitReferenceV1],
    content: &str,
    event_index: usize,
    states: &mut [ActiveVariant],
    sources: &BTreeMap<String, NarrativeSourceRevisionV1>,
) -> Result<NarrativeRevisionEventResultV1, NarrativeValidationErrorV1> {
    let path = format!("$.events[{event_index}]");
    let index = variant_index(states, variant_id, &format!("{path}.variant_id"))?;
    let state = &mut states[index];
    if state.stale_source_refs.is_empty() {
        return Err(error(
            "revision.variant_not_pending",
            format!("{path}.variant_id"),
        ));
    }
    // The initial N-02 document has already enforced the per-kind reference
    // limit (512 beats for oral, 64 refs for illustration). Review cannot add
    // or remove references, so equality preserves that validated bound.
    if refs.len() != state.source_refs.len() {
        return Err(error(
            "revision.reference_set_changed",
            format!("{path}.source_refs"),
        ));
    }
    for (ref_index, (old, new)) in state.source_refs.iter().zip(refs).enumerate() {
        let ref_path = format!("{path}.source_refs[{ref_index}]");
        check_id(&new.unit_id, &format!("{ref_path}.unit_id"))?;
        if old.unit_id != new.unit_id {
            return Err(error("revision.reference_set_changed", ref_path));
        }
        require_source(sources, new, &ref_path)?;
    }
    check_text(content, MAX_TEXT_CHARS, &format!("{path}.content"))?;
    state.revision = next_variant_revision(state, &format!("{path}.variant_id"))?;
    state.source_refs = refs.to_vec();
    state.content = content.to_owned();
    state.stale_source_refs.clear();
    Ok(event_result(
        id,
        "variant_review",
        None,
        None,
        None,
        Some(variant_id),
        vec![variant_id.to_owned()],
    ))
}

/// Replay a bounded revision ledger without changing the input or writing any files.
/// Impact is computed from each active variant snapshot's direct source references.
pub fn track_narrative_revisions(
    bytes: &[u8],
) -> Result<NarrativeRevisionReportV1, NarrativeValidationErrorV1> {
    let ledger = parse_ledger(bytes)?;
    if ledger.events.is_empty() {
        return Err(error("narrative.invalid_shape", "$.events"));
    }
    if ledger.events.len() > MAX_EVENTS {
        return Err(error("revision.too_many_events", "$.events"));
    }

    // Validate the nested N-02 document with its frozen validator before replay.
    let base_bytes = serde_json::to_vec(&ledger.base_document)
        .map_err(|_| error("narrative.invalid_shape", "$.base_document"))?;
    let base = render_narrative(&base_bytes).map_err(|mut cause| {
        cause.path = if cause.path == "$" {
            "$.base_document".to_owned()
        } else {
            format!("$.base_document{}", &cause.path[1..])
        };
        cause
    })?;
    let _ = base;

    let mut sources = BTreeMap::new();
    let mut source_history =
        Vec::with_capacity(ledger.base_document.units.len() + ledger.events.len());
    for unit in &ledger.base_document.units {
        let source = source_record(
            &unit.id,
            unit.revision,
            unit.origin,
            &unit.text,
            &unit.based_on,
        );
        source_history.push(source.clone());
        sources.insert(unit.id.clone(), source);
    }

    let mut states = Vec::with_capacity(ledger.base_document.variants.len());
    let mut variant_history =
        Vec::with_capacity(ledger.base_document.variants.len() + ledger.events.len());
    for variant in &ledger.base_document.variants {
        let (id, kind, refs, content) = source_refs(variant);
        let name = match variant {
            NarrativeVariantV1::SpokenOutline { name, .. }
            | NarrativeVariantV1::SingleIllustration { name, .. } => name.clone(),
        };
        let state = ActiveVariant {
            id,
            kind,
            name,
            revision: 1,
            source_refs: refs,
            content,
            stale_source_refs: Vec::new(),
        };
        push_snapshot(&mut variant_history, &state);
        states.push(state);
    }

    let mut event_ids = BTreeSet::new();
    let mut event_results = Vec::with_capacity(ledger.events.len());
    let mut suggestions = Vec::new();
    let mut decisions = Vec::new();

    for (event_index, event) in ledger.events.iter().enumerate() {
        let path = format!("$.events[{event_index}]");
        check_id(event.id(), &format!("{path}.id"))?;
        if !event_ids.insert(event.id()) {
            return Err(error("revision.duplicate_event", format!("{path}.id")));
        }
        let result = match event {
            NarrativeRevisionEventV1::AssistantSuggestion {
                id,
                variant_id,
                proposal_ref,
                text,
            } => {
                let variant_position =
                    variant_index(&states, variant_id, &format!("{path}.variant_id"))?;
                if states[variant_position].kind != "single_illustration" {
                    return Err(error(
                        "revision.decision_not_illustration",
                        format!("{path}.variant_id"),
                    ));
                }
                ensure_creative_proposal(&sources, proposal_ref, &format!("{path}.proposal_ref"))?;
                check_text(text, MAX_TEXT_CHARS, &format!("{path}.text"))?;
                suggestions.push(NarrativeSuggestionRecordV1 {
                    id: id.clone(),
                    variant_id: variant_id.clone(),
                    proposal_ref: proposal_ref.clone(),
                    text: text.clone(),
                });
                event_result(
                    id,
                    "assistant_suggestion",
                    None,
                    None,
                    None,
                    Some(variant_id),
                    Vec::new(),
                )
            }
            NarrativeRevisionEventV1::AuthorDecision {
                id,
                variant_id,
                proposal_ref,
                decision,
                content,
            } => {
                let variant_position =
                    variant_index(&states, variant_id, &format!("{path}.variant_id"))?;
                let state = &mut states[variant_position];
                if state.kind != "single_illustration" {
                    return Err(error(
                        "revision.decision_not_illustration",
                        format!("{path}.variant_id"),
                    ));
                }
                if !state.stale_source_refs.is_empty() {
                    return Err(error(
                        "revision.pending_review",
                        format!("{path}.variant_id"),
                    ));
                }
                ensure_creative_proposal(&sources, proposal_ref, &format!("{path}.proposal_ref"))?;
                check_text(content, MAX_TEXT_CHARS, &format!("{path}.content"))?;
                match decision {
                    NarrativeAuthorDecisionV1::Accepted | NarrativeAuthorDecisionV1::Edited => {
                        if let Some(existing) = state
                            .source_refs
                            .iter()
                            .find(|reference| reference.unit_id == proposal_ref.unit_id)
                        {
                            if existing.revision != proposal_ref.revision {
                                return Err(error(
                                    "narrative.stale_revision",
                                    format!("{path}.proposal_ref.revision"),
                                ));
                            }
                        } else {
                            if state.source_refs.len() == MAX_VARIANT_REFS {
                                return Err(error(
                                    "narrative.too_many_items",
                                    format!("{path}.variant_id"),
                                ));
                            }
                            state.source_refs.push(proposal_ref.clone());
                        }
                    }
                    NarrativeAuthorDecisionV1::Declined => {
                        state
                            .source_refs
                            .retain(|reference| reference.unit_id != proposal_ref.unit_id);
                    }
                }
                state.revision = next_variant_revision(state, &format!("{path}.variant_id"))?;
                state.content = content.clone();
                push_snapshot(&mut variant_history, state);
                decisions.push(NarrativeAuthorDecisionRecordV1 {
                    id: id.clone(),
                    variant_id: variant_id.clone(),
                    proposal_ref: proposal_ref.clone(),
                    decision: *decision,
                    variant_revision: state.revision,
                });
                event_result(
                    id,
                    "author_decision",
                    Some(&proposal_ref.unit_id),
                    None,
                    None,
                    Some(variant_id),
                    vec![variant_id.clone()],
                )
            }
            NarrativeRevisionEventV1::SourceRevision {
                id,
                unit_id,
                expected_revision,
                text,
            } => {
                check_id(unit_id, &format!("{path}.unit_id"))?;
                check_text(text, MAX_TEXT_CHARS, &format!("{path}.text"))?;
                if has_pending_review(&states) {
                    return Err(error("revision.pending_review", format!("{path}.unit_id")));
                }
                let Some(old) = sources.get(unit_id).cloned() else {
                    return Err(error(
                        "narrative.dangling_reference",
                        format!("{path}.unit_id"),
                    ));
                };
                if old.revision != *expected_revision {
                    return Err(error(
                        "narrative.stale_revision",
                        format!("{path}.expected_revision"),
                    ));
                }
                let next = expected_revision.checked_add(1).ok_or_else(|| {
                    error("revision.too_many_versions", format!("{path}.unit_id"))
                })?;
                let affected: Vec<String> = states
                    .iter()
                    .filter(|variant| {
                        variant.source_refs.iter().any(|reference| {
                            reference.unit_id == *unit_id && reference.revision == old.revision
                        })
                    })
                    .map(|variant| variant.id.clone())
                    .collect();
                let revised = source_record(unit_id, next, old.origin, text, &old.based_on);
                source_history.push(revised.clone());
                sources.insert(unit_id.clone(), revised);
                for variant in &mut states {
                    if let Some(reference) = variant.source_refs.iter().find(|reference| {
                        reference.unit_id == *unit_id && reference.revision == old.revision
                    }) {
                        variant.stale_source_refs.push(reference.clone());
                    }
                }
                event_result(
                    id,
                    "source_revision",
                    Some(unit_id),
                    Some(old.revision),
                    Some(next),
                    None,
                    affected,
                )
            }
            NarrativeRevisionEventV1::VariantReview {
                id,
                variant_id,
                source_refs,
                content,
            } => {
                let result = review_variant(
                    id,
                    variant_id,
                    source_refs,
                    content,
                    event_index,
                    &mut states,
                    &sources,
                )?;
                let index = variant_index(&states, variant_id, &format!("{path}.variant_id"))?;
                push_snapshot(&mut variant_history, &states[index]);
                result
            }
        };
        event_results.push(result);
    }

    Ok(NarrativeRevisionReportV1 {
        document_id: ledger.base_document.document_id,
        events: event_results,
        source_history,
        variant_history,
        suggestions,
        author_decisions: decisions,
        current_variants: states.iter().map(current_variant).collect(),
    })
}

/// Return one source-change impact by event ID; callers cannot submit their own affected IDs.
pub fn query_narrative_impact(
    bytes: &[u8],
    change_id: &str,
) -> Result<NarrativeRevisionEventResultV1, NarrativeValidationErrorV1> {
    check_id(change_id, "$.change_id")?;
    let report = track_narrative_revisions(bytes)?;
    let Some(event) = report.events.iter().find(|event| event.id == change_id) else {
        return Err(error("revision.change_not_found", "$.change_id"));
    };
    if event.kind != "source_revision" {
        return Err(error("revision.not_source_change", "$.change_id"));
    }
    Ok(event.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_is_preserved_across_source_versions() {
        let original = NarrativeSourceRevisionV1 {
            unit_id: "C01".to_owned(),
            revision: 1,
            origin: NarrativeOriginV1::CreativeProposal,
            text: "red umbrella".to_owned(),
            based_on: Vec::new(),
        };
        let next = source_record(
            &original.unit_id,
            2,
            original.origin,
            "red umbrella moved",
            &original.based_on,
        );
        assert_eq!(next.origin, NarrativeOriginV1::CreativeProposal);
    }
}
