// SPDX-License-Identifier: MIT
//! Pure audit of commercial-copy source identities and declared revisions.
use std::collections::{BTreeMap, BTreeSet};

use storytelling_core::commercial_copy::{
    CommercialCopyClaimReportV1, CommercialCopyClaimReportV2, CommercialCopyClaimV1,
    CommercialCopyDocumentV1, CommercialCopyKindV1, CommercialCopyReferenceReportV1,
    CommercialCopyReferenceReportV2, CommercialCopyReportV1, CommercialCopyReportV2,
};
use storytelling_core::narrative::NarrativeValidationErrorV1;
use storytelling_core::narrative_revision::NarrativeSourceRevisionV1;

use crate::narrative::{check_id, check_text, error, preflight_json, version};
use crate::{MAX_INPUT_BYTES, track_narrative_revisions};

const MAX_COPY_CLAIMS: usize = 128;
const MAX_COPY_REFERENCES: usize = 2048;
const MAX_EVIDENCE_REFS_PER_CLAIM: usize = 64;
const MAX_COPY_OUTPUT_BYTES: usize = 1_048_576;

fn remap_error(cause: NarrativeValidationErrorV1) -> NarrativeValidationErrorV1 {
    let code = cause
        .code
        .strip_prefix("narrative.")
        .map(|suffix| format!("copy.{suffix}"))
        .unwrap_or(cause.code);
    error(&code, cause.path)
}

fn parse_copy(bytes: &[u8]) -> Result<CommercialCopyDocumentV1, NarrativeValidationErrorV1> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    preflight_json(bytes).map_err(remap_error)?;
    version(bytes).map_err(remap_error)?;
    serde_json::from_slice(bytes).map_err(|cause| {
        let message = cause.to_string();
        let code = if message.starts_with("unknown field") {
            "copy.unknown_field"
        } else if message.starts_with("duplicate field") {
            "copy.duplicate_field"
        } else {
            "copy.invalid_shape"
        };
        error(code, "$")
    })
}

fn validate_claim(
    claim: &CommercialCopyClaimV1,
    path: &str,
    claim_ids: &mut BTreeSet<String>,
    total_refs: &mut usize,
) -> Result<(), NarrativeValidationErrorV1> {
    check_id(&claim.id, &format!("{path}.id")).map_err(remap_error)?;
    if !claim_ids.insert(claim.id.clone()) {
        return Err(error("copy.duplicate_claim_id", format!("{path}.id")));
    }
    check_text(&claim.wording, 8192, &format!("{path}.wording")).map_err(remap_error)?;
    if claim.evidence_refs.len() > MAX_EVIDENCE_REFS_PER_CLAIM {
        return Err(error(
            "copy.too_many_references",
            format!("{path}.evidence_refs"),
        ));
    }
    let mut refs = BTreeSet::new();
    for (index, reference) in claim.evidence_refs.iter().enumerate() {
        let ref_path = format!("{path}.evidence_refs[{index}]");
        check_id(&reference.unit_id, &format!("{ref_path}.unit_id")).map_err(remap_error)?;
        if reference.revision == 0 {
            return Err(error(
                "copy.invalid_revision",
                format!("{ref_path}.revision"),
            ));
        }
        if !refs.insert((reference.unit_id.as_str(), reference.revision)) {
            return Err(error("copy.duplicate_evidence_reference", ref_path));
        }
    }
    *total_refs += claim.evidence_refs.len();
    if *total_refs > MAX_COPY_REFERENCES {
        return Err(error("copy.too_many_references", "$.claims"));
    }
    Ok(())
}

fn latest_sources(
    history: &[NarrativeSourceRevisionV1],
) -> BTreeMap<&str, &NarrativeSourceRevisionV1> {
    let mut sources = BTreeMap::new();
    for source in history {
        sources.insert(source.unit_id.as_str(), source);
    }
    sources
}

fn reference_report(
    reference: &storytelling_core::commercial_copy::CommercialCopyEvidenceReferenceV1,
    history: &[NarrativeSourceRevisionV1],
    current_sources: &BTreeMap<&str, &NarrativeSourceRevisionV1>,
) -> CommercialCopyReferenceReportV2 {
    let cited = history.iter().find(|source| {
        source.unit_id == reference.unit_id && source.revision == reference.revision
    });
    let Some(current) = current_sources.get(reference.unit_id.as_str()) else {
        return CommercialCopyReferenceReportV2 {
            unit_id: reference.unit_id.clone(),
            revision: reference.revision,
            current_revision: None,
            cited_source_origin: cited.map(|source| source.origin),
            cited_source_text: cited.map(|source| source.text.clone()),
            current_source_text: None,
            reference_status: "dangling".to_owned(),
        };
    };
    let status = if current.revision == reference.revision {
        "current"
    } else if cited.is_some() {
        "revised"
    } else {
        "unknown_revision"
    };
    CommercialCopyReferenceReportV2 {
        unit_id: reference.unit_id.clone(),
        revision: reference.revision,
        current_revision: Some(current.revision),
        cited_source_origin: cited.map(|source| source.origin),
        cited_source_text: cited.map(|source| source.text.clone()),
        current_source_text: Some(current.text.clone()),
        reference_status: status.to_owned(),
    }
}

fn claim_report(
    claim: &CommercialCopyClaimV1,
    history: &[NarrativeSourceRevisionV1],
    current_sources: &BTreeMap<&str, &NarrativeSourceRevisionV1>,
) -> CommercialCopyClaimReportV2 {
    let evidence_refs: Vec<_> = claim
        .evidence_refs
        .iter()
        .map(|reference| reference_report(reference, history, current_sources))
        .collect();
    let reference_status = if evidence_refs.is_empty() {
        match claim.kind {
            CommercialCopyKindV1::FactualClaim => "missing",
            CommercialCopyKindV1::CreativeProposal => "not_applicable",
        }
    } else if evidence_refs
        .iter()
        .any(|reference| reference.reference_status == "dangling")
    {
        "dangling"
    } else if evidence_refs
        .iter()
        .any(|reference| reference.reference_status == "unknown_revision")
    {
        "unknown_revision"
    } else if evidence_refs
        .iter()
        .any(|reference| reference.reference_status == "revised")
    {
        "revised"
    } else {
        "current"
    };
    CommercialCopyClaimReportV2 {
        id: claim.id.clone(),
        kind: claim.kind.as_str().to_owned(),
        wording: claim.wording.clone(),
        reference_status: reference_status.to_owned(),
        support_status: match claim.kind {
            CommercialCopyKindV1::FactualClaim => "human_review_required",
            CommercialCopyKindV1::CreativeProposal => "not_applicable",
        }
        .to_owned(),
        evidence_refs,
    }
}

/// Produce a copy outline and report only source identity/revision status.
/// A current reference does not establish that its source supports the wording.
fn build_report(
    copy_bytes: &[u8],
    ledger_bytes: &[u8],
) -> Result<CommercialCopyReportV2, NarrativeValidationErrorV1> {
    if ledger_bytes.len() > MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$.evidence_ledger"));
    }
    let copy = parse_copy(copy_bytes)?;
    if copy.schema_version != 1 {
        return Err(error("copy.unsupported_version", "$.schema_version"));
    }
    check_id(&copy.copy_id, "$.copy_id").map_err(remap_error)?;
    if copy.copy_revision == 0 {
        return Err(error("copy.invalid_revision", "$.copy_revision"));
    }
    check_id(&copy.evidence_document_id, "$.evidence_document_id").map_err(remap_error)?;
    check_text(&copy.purpose, 512, "$.purpose").map_err(remap_error)?;
    check_text(&copy.audience, 512, "$.audience").map_err(remap_error)?;
    if copy.claims.is_empty() {
        return Err(error("copy.invalid_shape", "$.claims"));
    }
    if copy.claims.len() > MAX_COPY_CLAIMS {
        return Err(error("copy.too_many_claims", "$.claims"));
    }

    let mut claim_ids = BTreeSet::new();
    let mut total_refs = 0;
    for (index, claim) in copy.claims.iter().enumerate() {
        validate_claim(
            claim,
            &format!("$.claims[{index}]"),
            &mut claim_ids,
            &mut total_refs,
        )?;
    }
    validate_claim(
        &copy.call_to_action,
        "$.call_to_action",
        &mut claim_ids,
        &mut total_refs,
    )?;

    let evidence = track_narrative_revisions(ledger_bytes)?;
    if copy.evidence_document_id != evidence.document_id {
        return Err(error(
            "copy.evidence_document_mismatch",
            "$.evidence_document_id",
        ));
    }
    let current_sources = latest_sources(&evidence.source_history);
    let claims: Vec<_> = copy
        .claims
        .iter()
        .map(|claim| claim_report(claim, &evidence.source_history, &current_sources))
        .collect();
    let call_to_action = claim_report(
        &copy.call_to_action,
        &evidence.source_history,
        &current_sources,
    );
    let reference_attention_claim_ids = claims
        .iter()
        .chain(std::iter::once(&call_to_action))
        .filter(|claim| {
            matches!(
                claim.reference_status.as_str(),
                "missing" | "dangling" | "unknown_revision" | "revised"
            )
        })
        .map(|claim| claim.id.clone())
        .collect();
    let report = CommercialCopyReportV2 {
        schema_version: 2,
        copy_id: copy.copy_id,
        copy_revision: copy.copy_revision,
        evidence_document_id: evidence.document_id,
        purpose: copy.purpose,
        audience: copy.audience,
        claims,
        call_to_action,
        reference_attention_claim_ids,
        reference_scope: "reference-identity-and-revision-only",
    };
    Ok(report)
}

fn check_output_limit(report: &impl serde::Serialize) -> Result<(), NarrativeValidationErrorV1> {
    let bytes = serde_json::to_vec(report).map_err(|_| error("copy.invalid_shape", "$"))?;
    if bytes.len() > MAX_COPY_OUTPUT_BYTES {
        return Err(error("copy.output_too_large", "$"));
    }
    Ok(())
}

fn v1_claim(claim: CommercialCopyClaimReportV2) -> CommercialCopyClaimReportV1 {
    CommercialCopyClaimReportV1 {
        id: claim.id,
        kind: claim.kind,
        wording: claim.wording,
        reference_status: claim.reference_status,
        evidence_refs: claim
            .evidence_refs
            .into_iter()
            .map(|reference| CommercialCopyReferenceReportV1 {
                unit_id: reference.unit_id,
                revision: reference.revision,
                current_revision: reference.current_revision,
                cited_source_origin: reference.cited_source_origin,
                cited_source_text: reference.cited_source_text,
                reference_status: reference.reference_status,
            })
            .collect(),
    }
}

/// Retain the accepted V1 shape and enforce V1's own serialized output limit.
pub fn audit_commercial_copy(
    copy_bytes: &[u8],
    ledger_bytes: &[u8],
) -> Result<CommercialCopyReportV1, NarrativeValidationErrorV1> {
    let full = build_report(copy_bytes, ledger_bytes)?;
    let report = CommercialCopyReportV1 {
        schema_version: 1,
        copy_id: full.copy_id,
        copy_revision: full.copy_revision,
        evidence_document_id: full.evidence_document_id,
        purpose: full.purpose,
        audience: full.audience,
        claims: full.claims.into_iter().map(v1_claim).collect(),
        call_to_action: v1_claim(full.call_to_action),
        reference_scope: full.reference_scope,
    };
    check_output_limit(&report)?;
    Ok(report)
}

/// Opt-in V2 adds current text and attention IDs; it does not judge support.
pub fn audit_commercial_copy_v2(
    copy_bytes: &[u8],
    ledger_bytes: &[u8],
) -> Result<CommercialCopyReportV2, NarrativeValidationErrorV1> {
    let report = build_report(copy_bytes, ledger_bytes)?;
    check_output_limit(&report)?;
    Ok(report)
}
