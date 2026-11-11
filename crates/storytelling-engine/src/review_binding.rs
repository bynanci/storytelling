// SPDX-License-Identifier: Apache-2.0
//! Same-input finite OTIO review identity and declared-context freshness.
//! No source URI, media file, filesystem, host, or authority is observed here.

use serde::Serialize;
use storytelling_core::review::{
    ReviewContextFieldDigestsV1, ReviewContextV1, ReviewFreshnessReportV1, ReviewIdentityV1,
    ReviewRecordV1,
};
use storytelling_core::{ValidationError, review::ReviewLossChoiceV1};

pub const MAX_REVIEW_CONTEXT_BYTES: usize = 65_536;
pub const MAX_REVIEW_RECORD_BYTES: usize = 98_304;
const MAX_REVIEW_ID_BYTES: usize = 128;
const MAX_REVIEW_CAPABILITIES: usize = 64;
const MAX_JSON_DEPTH: usize = 64;
const REVIEW_SCOPE: &str = "declared-review-binding-only";
const IDENTITY_DOMAIN: &[u8] = b"storytelling.finite-otio-review-binding.v1";
const CONTEXT_DOMAIN: &[u8] = b"storytelling.review-context.v1";

/// An in-process binding produced only after bounded validation and compilation.
/// It is intentionally neither serializable nor deserializable.
///
/// ```compile_fail
/// use storytelling_engine::BoundReviewV1;
/// let forged = BoundReviewV1 { record: todo!(), compiled_otio: todo!() };
/// ```
///
/// ```compile_fail
/// use serde::de::DeserializeOwned;
/// use storytelling_engine::BoundReviewV1;
/// fn requires_deserialize<T: DeserializeOwned>() {}
/// requires_deserialize::<BoundReviewV1>();
/// ```
///
/// ```compile_fail
/// use storytelling_engine::BoundReviewV1;
/// let _defaulted = BoundReviewV1::default();
/// ```
///
/// ```compile_fail
/// use storytelling_engine::BoundReviewV1;
/// fn mutate(review: &mut BoundReviewV1) { review.compiled_otio[0] = 0; }
/// ```
#[derive(Debug)]
pub struct BoundReviewV1 {
    record: ReviewRecordV1,
    compiled_otio: Box<[u8]>,
}

impl BoundReviewV1 {
    pub fn record(&self) -> &ReviewRecordV1 {
        &self.record
    }

    pub fn compiled_otio(&self) -> &[u8] {
        &self.compiled_otio
    }
}

#[derive(Serialize)]
struct IdentityPayloadV1<'a> {
    schema_version: u32,
    identity: &'a ReviewIdentityV1,
}

/// Parse and validate a bounded strict context document.
pub fn parse_review_context(bytes: &[u8]) -> Result<ReviewContextV1, ValidationError> {
    if bytes.len() > MAX_REVIEW_CONTEXT_BYTES {
        return Err(ValidationError::new("review.context_too_large", "$"));
    }
    check_json_depth(bytes)?;
    let context: ReviewContextV1 = serde_json::from_slice(bytes)
        .map_err(|_| ValidationError::new("review.invalid_context_json", "$"))?;
    validate_context(&context)?;
    Ok(context)
}

/// Derive a review binding from one exact admitted composition byte slice.
/// Hashing those bytes is not source verification or proof of their origin.
pub fn prepare_review(
    composition_bytes: &[u8],
    context: &ReviewContextV1,
) -> Result<BoundReviewV1, ValidationError> {
    validate_context(context)?;

    let imported = crate::import_openfilm_snapshot(composition_bytes, &Default::default())
        .map_err(|error| ValidationError::new(&error.code, error.path))?;
    let plan_bytes = serde_json::to_vec(&imported.plan)
        .map_err(|_| ValidationError::new("review.serialization_failed", "$.plan"))?;
    let choices = loss_choices(context);
    let compiled = crate::export_openfilm_otio(composition_bytes, &Default::default(), &choices)
        .map_err(|error| ValidationError::new(&error.code, error.path))?;
    bind_compiled_review(composition_bytes, &plan_bytes, compiled, context)
}

pub(crate) fn bind_compiled_review(
    composition_bytes: &[u8],
    plan_bytes: &[u8],
    compiled: crate::CompiledOtioV1,
    context: &ReviewContextV1,
) -> Result<BoundReviewV1, ValidationError> {
    let artifact_bytes = serde_json::to_vec(&compiled.document)
        .map_err(|_| ValidationError::new("review.serialization_failed", "$.artifact"))?;
    if artifact_bytes.len() > crate::MAX_OTIO_OUTPUT_BYTES {
        return Err(ValidationError::new("otio.output_too_large", "$"));
    }
    if !compiled
        .report
        .required_capabilities
        .iter()
        .all(|required| {
            context
                .host
                .declared_capabilities
                .binary_search(required)
                .is_ok()
        })
    {
        return Err(ValidationError::new(
            "review.context_missing_capability",
            "$.host.declared_capabilities",
        ));
    }

    let report_bytes = serde_json::to_vec(&compiled.report)
        .map_err(|_| ValidationError::new("review.serialization_failed", "$.report"))?;
    let context_fields = context_field_digests(context)?;
    let context_bytes = serde_json::to_vec(context)
        .map_err(|_| ValidationError::new("review.serialization_failed", "$.context"))?;
    let identity = ReviewIdentityV1 {
        composition_sha256: sha256_hex(composition_bytes),
        plan_sha256: hash_framed(b"storytelling.review-plan.v1", plan_bytes),
        artifact_sha256: sha256_hex(&artifact_bytes),
        report_sha256: hash_framed(b"storytelling.review-report.v1", &report_bytes),
        compiler_version: concat!("storytelling-engine-", env!("CARGO_PKG_VERSION")).to_owned(),
        profile_id: compiled.report.profile.to_owned(),
        context_sha256: hash_framed(CONTEXT_DOMAIN, &context_bytes),
        context_fields,
    };
    let binding_sha256 = identity_digest(&identity)?;
    let record = ReviewRecordV1 {
        schema_version: 1,
        scope: REVIEW_SCOPE.to_owned(),
        binding_sha256,
        identity,
        executable: false,
        approval_granted: false,
        source_verified: false,
        host_verified: false,
    };

    Ok(BoundReviewV1 {
        record,
        compiled_otio: artifact_bytes.into_boxed_slice(),
    })
}

/// Recompile the current composition and compare it with an untrusted review DTO.
/// A matching record proves only consistency of supplied declarations.
pub fn check_review(
    record_bytes: &[u8],
    current_composition_bytes: &[u8],
    current_context_bytes: &[u8],
) -> Result<ReviewFreshnessReportV1, ValidationError> {
    // Respect argument order and reject all size excesses before any JSON parsing.
    if record_bytes.len() > MAX_REVIEW_RECORD_BYTES {
        return Err(ValidationError::new("review.record_too_large", "$"));
    }
    if current_composition_bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(ValidationError::new("input.too_large", "$"));
    }
    if current_context_bytes.len() > MAX_REVIEW_CONTEXT_BYTES {
        return Err(ValidationError::new("review.context_too_large", "$"));
    }
    check_json_depth(record_bytes)?;
    let record: ReviewRecordV1 = serde_json::from_slice(record_bytes)
        .map_err(|_| ValidationError::new("review.invalid_record_json", "$"))?;
    validate_record(&record)?;
    let context = parse_review_context(current_context_bytes)?;
    let current = prepare_review(current_composition_bytes, &context)?;

    let mut stale_fields = identity_differences(&record.identity, &current.record.identity);
    if record.binding_sha256 != current.record.binding_sha256 && stale_fields.is_empty() {
        stale_fields.push("binding".to_owned());
    }
    stale_fields.sort();
    stale_fields.dedup();
    Ok(ReviewFreshnessReportV1 {
        schema_version: 1,
        scope: REVIEW_SCOPE,
        current: stale_fields.is_empty(),
        stale_fields,
        executable: false,
        approval_granted: false,
        source_verified: false,
        host_verified: false,
    })
}

fn validate_context(context: &ReviewContextV1) -> Result<(), ValidationError> {
    if context.schema_version != 1 {
        return Err(ValidationError::new(
            "review.unsupported_version",
            "$.schema_version",
        ));
    }
    validate_id(&context.project_id, "$.project_id")?;
    validate_id(&context.target_revision, "$.target_revision")?;
    validate_id(&context.host.id, "$.host.id")?;
    validate_id(&context.host.version, "$.host.version")?;
    validate_id(&context.host.profile_id, "$.host.profile_id")?;
    validate_sha256(&context.host.binary_sha256, "$.host.binary_sha256")?;
    if context.host.declared_capabilities.len() > MAX_REVIEW_CAPABILITIES {
        return Err(ValidationError::new(
            "review.too_many",
            "$.host.declared_capabilities",
        ));
    }
    validate_sorted_ids(
        &context.host.declared_capabilities,
        "$.host.declared_capabilities",
    )?;
    let choices: Vec<_> = context
        .loss_choices
        .iter()
        .map(ReviewLossChoiceV1::as_str)
        .collect();
    if choices.len() > 2 {
        return Err(ValidationError::new("review.too_many", "$.loss_choices"));
    }
    for pair in choices.windows(2) {
        if pair[0] == pair[1] {
            return Err(ValidationError::new(
                "review.duplicate_id",
                "$.loss_choices",
            ));
        }
        if pair[0] > pair[1] {
            return Err(ValidationError::new(
                "review.noncanonical_order",
                "$.loss_choices",
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_record(record: &ReviewRecordV1) -> Result<(), ValidationError> {
    if record.schema_version != 1 {
        return Err(ValidationError::new(
            "review.unsupported_version",
            "$.schema_version",
        ));
    }
    if record.scope != REVIEW_SCOPE
        || record.executable
        || record.approval_granted
        || record.source_verified
        || record.host_verified
    {
        return Err(ValidationError::new("review.invalid_claims", "$.scope"));
    }
    validate_sha256(&record.binding_sha256, "$.binding_sha256")?;
    let identity = &record.identity;
    validate_sha256(
        &identity.composition_sha256,
        "$.identity.composition_sha256",
    )?;
    validate_sha256(&identity.plan_sha256, "$.identity.plan_sha256")?;
    validate_sha256(&identity.artifact_sha256, "$.identity.artifact_sha256")?;
    validate_sha256(&identity.report_sha256, "$.identity.report_sha256")?;
    validate_sha256(&identity.context_sha256, "$.identity.context_sha256")?;
    validate_id(&identity.compiler_version, "$.identity.compiler_version")?;
    validate_id(&identity.profile_id, "$.identity.profile_id")?;
    let fields = &identity.context_fields;
    validate_sha256(&fields.project_id, "$.identity.context_fields.project_id")?;
    validate_sha256(
        &fields.target_revision,
        "$.identity.context_fields.target_revision",
    )?;
    validate_sha256(&fields.host_id, "$.identity.context_fields.host_id")?;
    validate_sha256(
        &fields.host_version,
        "$.identity.context_fields.host_version",
    )?;
    validate_sha256(
        &fields.host_profile_id,
        "$.identity.context_fields.host_profile_id",
    )?;
    validate_sha256(
        &fields.host_binary_sha256,
        "$.identity.context_fields.host_binary_sha256",
    )?;
    validate_sha256(
        &fields.host_declared_capabilities,
        "$.identity.context_fields.host_declared_capabilities",
    )?;
    validate_sha256(
        &fields.loss_choices,
        "$.identity.context_fields.loss_choices",
    )?;
    if identity_digest(identity)? != record.binding_sha256 {
        return Err(ValidationError::new(
            "review.record_digest_mismatch",
            "$.binding_sha256",
        ));
    }
    Ok(())
}

fn validate_id(value: &str, path: &str) -> Result<(), ValidationError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > MAX_REVIEW_ID_BYTES
        || !bytes[0].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(byte))
    {
        return Err(ValidationError::new("review.invalid_id", path));
    }
    Ok(())
}

fn validate_sha256(value: &str, path: &str) -> Result<(), ValidationError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(ValidationError::new("review.invalid_digest", path));
    }
    Ok(())
}

fn validate_sorted_ids(values: &[String], path: &str) -> Result<(), ValidationError> {
    for value in values {
        validate_id(value, path)?;
    }
    for pair in values.windows(2) {
        if pair[0] == pair[1] {
            return Err(ValidationError::new("review.duplicate_id", path));
        }
        if pair[0] > pair[1] {
            return Err(ValidationError::new("review.noncanonical_order", path));
        }
    }
    Ok(())
}

pub(crate) fn loss_choices(context: &ReviewContextV1) -> crate::OtioLossChoicesV1 {
    crate::OtioLossChoicesV1 {
        picture_only: context
            .loss_choices
            .contains(&ReviewLossChoiceV1::PictureOnly),
        metadata_only_locks: context
            .loss_choices
            .contains(&ReviewLossChoiceV1::MetadataOnlyLocks),
    }
}

fn context_field_digests(
    context: &ReviewContextV1,
) -> Result<ReviewContextFieldDigestsV1, ValidationError> {
    let host = &context.host;
    Ok(ReviewContextFieldDigestsV1 {
        project_id: hash_field("project_id", &context.project_id)?,
        target_revision: hash_field("target_revision", &context.target_revision)?,
        host_id: hash_field("host.id", &host.id)?,
        host_version: hash_field("host.version", &host.version)?,
        host_profile_id: hash_field("host.profile_id", &host.profile_id)?,
        host_binary_sha256: hash_field("host.binary_sha256", &host.binary_sha256)?,
        host_declared_capabilities: hash_field(
            "host.declared_capabilities",
            &host.declared_capabilities,
        )?,
        loss_choices: hash_field("loss_choices", &context.loss_choices)?,
    })
}

fn hash_field<T: Serialize>(field: &str, value: &T) -> Result<String, ValidationError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| ValidationError::new("review.serialization_failed", "$.context"))?;
    let domain = format!("storytelling.review-context-field.v1/{field}");
    Ok(hash_framed(domain.as_bytes(), &bytes))
}

fn identity_digest(identity: &ReviewIdentityV1) -> Result<String, ValidationError> {
    let payload = IdentityPayloadV1 {
        schema_version: 1,
        identity,
    };
    let bytes = serde_json::to_vec(&payload)
        .map_err(|_| ValidationError::new("review.serialization_failed", "$.identity"))?;
    Ok(hash_framed(IDENTITY_DOMAIN, &bytes))
}

fn hash_framed(domain: &[u8], payload: &[u8]) -> String {
    let mut bytes = Vec::with_capacity(domain.len() + 8 + payload.len());
    bytes.extend_from_slice(domain);
    bytes.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    bytes.extend_from_slice(payload);
    sha256_hex(&bytes)
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let digest = storytelling_core::source_evidence::sha256_digest(bytes);
    digest
        .iter()
        .flat_map(|byte| {
            [
                char::from(b"0123456789abcdef"[usize::from(byte >> 4)]),
                char::from(b"0123456789abcdef"[usize::from(byte & 15)]),
            ]
        })
        .collect()
}

pub(crate) fn check_json_depth(bytes: &[u8]) -> Result<(), ValidationError> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for byte in bytes {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match *byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth = depth.saturating_add(1);
                if depth > MAX_JSON_DEPTH {
                    return Err(ValidationError::new("review.json_too_deep", "$"));
                }
            }
            b'}' | b']' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    Ok(())
}

pub(crate) fn identity_differences(
    previous: &ReviewIdentityV1,
    current: &ReviewIdentityV1,
) -> Vec<String> {
    let mut stale = Vec::new();
    if previous.composition_sha256 != current.composition_sha256 {
        stale.push("composition".to_owned());
    }
    if previous.plan_sha256 != current.plan_sha256 {
        stale.push("plan".to_owned());
    }
    if previous.artifact_sha256 != current.artifact_sha256 {
        stale.push("artifact".to_owned());
    }
    if previous.report_sha256 != current.report_sha256 {
        stale.push("report".to_owned());
    }
    if previous.compiler_version != current.compiler_version {
        stale.push("compiler_version".to_owned());
    }
    if previous.profile_id != current.profile_id {
        stale.push("profile_id".to_owned());
    }
    let a = &previous.context_fields;
    let b = &current.context_fields;
    for (field, changed) in [
        ("context.project_id", a.project_id != b.project_id),
        (
            "context.target_revision",
            a.target_revision != b.target_revision,
        ),
        ("context.host.id", a.host_id != b.host_id),
        ("context.host.version", a.host_version != b.host_version),
        (
            "context.host.profile_id",
            a.host_profile_id != b.host_profile_id,
        ),
        (
            "context.host.binary_sha256",
            a.host_binary_sha256 != b.host_binary_sha256,
        ),
        (
            "context.host.declared_capabilities",
            a.host_declared_capabilities != b.host_declared_capabilities,
        ),
        ("context.loss_choices", a.loss_choices != b.loss_choices),
    ] {
        if changed {
            stale.push(field.to_owned());
        }
    }
    if previous.context_sha256 != current.context_sha256 && stale.is_empty() {
        stale.push("context".to_owned());
    }
    stale
}

#[cfg(test)]
mod tests {
    use super::{MAX_JSON_DEPTH, check_json_depth};

    #[test]
    fn json_depth_scanner_accepts_exact_limit_and_ignores_string_contents() {
        let at_limit = format!(
            "{}0{}",
            "[".repeat(MAX_JSON_DEPTH),
            "]".repeat(MAX_JSON_DEPTH)
        );
        assert!(check_json_depth(at_limit.as_bytes()).is_ok());

        let over_limit = format!(
            "{}0{}",
            "[".repeat(MAX_JSON_DEPTH + 1),
            "]".repeat(MAX_JSON_DEPTH + 1)
        );
        assert_eq!(
            check_json_depth(over_limit.as_bytes()).unwrap_err().code,
            "review.json_too_deep"
        );

        assert!(check_json_depth(br#"{"text":"[[[[[["}"#).is_ok());
    }
}
