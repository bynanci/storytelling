// SPDX-License-Identifier: MIT
//! Strict, bounded approval identity contracts. This module has no host or I/O access.
use crate::{
    ValidationError,
    source_evidence::{MAX_VERIFIED_BUFFER_BYTES, VerifiedBufferV1},
};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

pub const MAX_PROPOSAL_BYTES: usize = 1_048_576;
pub const MAX_COMPILED_ARTIFACT_BYTES: usize = 4_194_304;
pub const MAX_APPROVAL_JSON_BYTES: usize = 1_048_576;
pub const MAX_SOURCES: usize = 1_024;
pub const MAX_HOST_CAPABILITIES: usize = 256;
pub const MAX_ACCEPTED_LOSSES: usize = 1_024;
pub const MAX_ID_BYTES: usize = 128;
pub const MAX_LOSS_PATH_BYTES: usize = 512;
pub const MAX_NONCE_BYTES: usize = 128;
pub const MAX_TIMESTAMP_UNIX_MS: u64 = 253_402_300_799_999;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceIdentityV1 {
    pub asset_id: String,
    pub sha256: String,
    pub byte_length: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HostObservationStateV1 {
    Declared,
    Observed,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostBindingV1 {
    pub host_id: String,
    pub profile_id: String,
    pub version: String,
    pub observation: HostObservationStateV1,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LossDispositionV1 {
    Native,
    Baked,
    MetadataOnly,
    Unsupported,
}

impl LossDispositionV1 {
    fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Baked => "baked",
            Self::MetadataOnly => "metadata_only",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptedLossV1 {
    pub feature: String,
    pub path: String,
    pub disposition: LossDispositionV1,
    pub accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalIdentityV1 {
    pub schema_version: u32,
    pub project_id: String,
    pub proposal_id: String,
    pub proposal_sha256: String,
    pub compiled_artifact_sha256: String,
    pub sources: Vec<SourceIdentityV1>,
    pub host: HostBindingV1,
    pub accepted_losses: Vec<AcceptedLossV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalPayloadV1 {
    pub schema_version: u32,
    pub identity: ApprovalIdentityV1,
    pub approval_id: String,
    pub principal_id: String,
    pub key_id: String,
    pub approved_at_unix_ms: u64,
    pub nonce: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedApprovalV1 {
    pub payload: ApprovalPayloadV1,
    pub signature_hex: String,
}

/// Caller-supplied expected principal/key/public-key binding.
///
/// The verifier checks that the signed IDs and signature match this value. It
/// does not establish that the caller obtained it from an authoritative key
/// registry or that its principal is authorized to approve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedApproverKeyV1 {
    pub principal_id: String,
    pub key_id: String,
    pub public_key_hex: String,
}

/// A private receipt that the supplied expected key verifies this payload and
/// that its identity matches the caller's current bounded identity token. It
/// does not represent signer authorization, freshness, or replay protection.
///
/// ```compile_fail
/// use storytelling_core::approval::VerifiedApprovalV1;
/// let forged = VerifiedApprovalV1 { payload: todo!() };
/// ```
///
/// ```compile_fail
/// use storytelling_core::approval::VerifiedApprovalV1;
/// fn requires_deserialize<T: serde::de::DeserializeOwned>() {}
/// requires_deserialize::<VerifiedApprovalV1>();
/// ```
#[derive(Debug)]
pub struct VerifiedApprovalV1 {
    payload: ApprovalPayloadV1,
}

impl VerifiedApprovalV1 {
    pub fn payload(&self) -> &ApprovalPayloadV1 {
        &self.payload
    }
}

/// A caller-supplied stable asset ID bound to an existing verified byte buffer.
/// It cannot be forged from a digest or deserialized from untrusted data.
pub struct VerifiedSourceBufferV1<'a> {
    asset_id: String,
    buffer: &'a VerifiedBufferV1,
}

impl<'a> VerifiedSourceBufferV1<'a> {
    pub fn new(asset_id: &str, buffer: &'a VerifiedBufferV1) -> Result<Self, ValidationError> {
        validate_id(asset_id, "$.asset_id")?;
        Ok(Self {
            asset_id: asset_id.to_owned(),
            buffer,
        })
    }
}

/// An opaque binding of caller-supplied bytes and metadata. Wire data cannot
/// construct this type. It does not prove that an importer/compiler produced
/// the supplied artifact bytes or that the inputs belong to one source snapshot.
///
/// ```compile_fail
/// use storytelling_core::approval::BoundApprovalIdentityV1;
/// let forged = BoundApprovalIdentityV1 { identity: todo!() };
/// ```
///
/// ```compile_fail
/// use storytelling_core::approval::BoundApprovalIdentityV1;
/// fn requires_deserialize<T: serde::de::DeserializeOwned>() {}
/// requires_deserialize::<BoundApprovalIdentityV1>();
/// ```
#[derive(Debug)]
pub struct BoundApprovalIdentityV1 {
    identity: ApprovalIdentityV1,
}

impl BoundApprovalIdentityV1 {
    pub fn identity(&self) -> &ApprovalIdentityV1 {
        &self.identity
    }
}

fn validate_builder_inputs(
    project_id: &str,
    proposal_id: &str,
    proposal_bytes: &[u8],
    artifact_bytes: &[u8],
    sources: &[VerifiedSourceBufferV1<'_>],
    host: &HostBindingV1,
    accepted_losses: &[AcceptedLossV1],
) -> Result<(), ValidationError> {
    if proposal_bytes.len() > MAX_PROPOSAL_BYTES {
        return Err(error("approval.proposal_too_large", "$.proposal_bytes"));
    }
    if artifact_bytes.len() > MAX_COMPILED_ARTIFACT_BYTES {
        return Err(error(
            "approval.artifact_too_large",
            "$.compiled_artifact_bytes",
        ));
    }
    if sources.len() > MAX_SOURCES {
        return Err(error("approval.too_many", "$.sources"));
    }
    if accepted_losses.len() > MAX_ACCEPTED_LOSSES {
        return Err(error("approval.too_many", "$.accepted_losses"));
    }

    validate_id(project_id, "$.project_id")?;
    validate_id(proposal_id, "$.proposal_id")?;

    let mut source_ids = HashSet::with_capacity(sources.len());
    let mut previous_source: Option<&str> = None;
    for source in sources {
        validate_id(&source.asset_id, "$.sources")?;
        if !source_ids.insert(source.asset_id.as_str()) {
            return Err(error("approval.duplicate_id", "$.sources"));
        }
        if previous_source.is_some_and(|value| value >= source.asset_id.as_str()) {
            return Err(error("approval.noncanonical_order", "$.sources"));
        }
        previous_source = Some(&source.asset_id);
    }

    validate_host(host)?;
    validate_losses(accepted_losses)?;
    Ok(())
}

/// Bind exact caller-supplied proposal/artifact bytes and verified source buffers.
///
/// This hashes the provided bytes; it does not compile them or establish a
/// semantic relationship between the proposal, artifact, and source buffers.
pub fn bind_approval_identity_v1(
    project_id: &str,
    proposal_id: &str,
    proposal_bytes: &[u8],
    artifact_bytes: &[u8],
    sources: &[VerifiedSourceBufferV1<'_>],
    host: &HostBindingV1,
    accepted_losses: &[AcceptedLossV1],
) -> Result<BoundApprovalIdentityV1, ValidationError> {
    validate_builder_inputs(
        project_id,
        proposal_id,
        proposal_bytes,
        artifact_bytes,
        sources,
        host,
        accepted_losses,
    )?;

    let source_identities = sources
        .iter()
        .map(|source| {
            let byte_length = u64::try_from(source.buffer.byte_length())
                .map_err(|_| error("approval.invalid_source_length", "$.sources"))?;
            Ok(SourceIdentityV1 {
                asset_id: source.asset_id.clone(),
                sha256: source.buffer.sha256().to_owned(),
                byte_length,
            })
        })
        .collect::<Result<Vec<_>, ValidationError>>()?;

    let identity = ApprovalIdentityV1 {
        schema_version: 1,
        project_id: project_id.to_owned(),
        proposal_id: proposal_id.to_owned(),
        proposal_sha256: sha256_hex(proposal_bytes),
        compiled_artifact_sha256: sha256_hex(artifact_bytes),
        sources: source_identities,
        host: host.clone(),
        accepted_losses: accepted_losses.to_vec(),
    };
    validate_approval_identity_v1(&identity)?;
    Ok(BoundApprovalIdentityV1 { identity })
}

/// Validate an untrusted wire identity before comparing it with a locally built token.
pub fn validate_approval_identity_v1(
    identity: &ApprovalIdentityV1,
) -> Result<(), ValidationError> {
    if identity.schema_version != 1 {
        return Err(error("approval.unsupported_version", "$.schema_version"));
    }
    validate_id(&identity.project_id, "$.project_id")?;
    validate_id(&identity.proposal_id, "$.proposal_id")?;
    validate_digest(&identity.proposal_sha256, "$.proposal_sha256")?;
    validate_digest(
        &identity.compiled_artifact_sha256,
        "$.compiled_artifact_sha256",
    )?;
    if identity.sources.len() > MAX_SOURCES {
        return Err(error("approval.too_many", "$.sources"));
    }
    validate_source_rows(&identity.sources)?;
    validate_host(&identity.host)?;
    if identity.accepted_losses.len() > MAX_ACCEPTED_LOSSES {
        return Err(error("approval.too_many", "$.accepted_losses"));
    }
    validate_losses(&identity.accepted_losses)?;
    Ok(())
}

/// Return the frozen JSON payload bytes prefixed by its protocol domain.
/// This function never signs and does not assert who is authorized to sign.
pub fn canonical_approval_signing_bytes(
    payload: &ApprovalPayloadV1,
) -> Result<Vec<u8>, ValidationError> {
    validate_payload(payload)?;
    let json = serde_json::to_vec(payload)
        .map_err(|_| error("approval.serialization_failed", "$.payload"))?;
    if json.len() > MAX_APPROVAL_JSON_BYTES {
        return Err(error("approval.too_large", "$.payload"));
    }
    let mut bytes = Vec::with_capacity(APPROVAL_DOMAIN.len() + json.len());
    bytes.extend_from_slice(APPROVAL_DOMAIN);
    bytes.extend_from_slice(&json);
    Ok(bytes)
}

pub fn verify_approval_json(
    bytes: &[u8],
    expected: &BoundApprovalIdentityV1,
    expected_key: &ExpectedApproverKeyV1,
) -> Result<VerifiedApprovalV1, ValidationError> {
    if bytes.len() > MAX_APPROVAL_JSON_BYTES {
        return Err(error("approval.too_large", "$"));
    }
    let signed: SignedApprovalV1 =
        serde_json::from_slice(bytes).map_err(|_| error("approval.invalid_json", "$"))?;
    validate_payload(&signed.payload)?;
    if signed.payload.identity != expected.identity {
        return Err(error("approval.identity_mismatch", "$.payload.identity"));
    }
    validate_id(&expected_key.principal_id, "$.expected_key.principal_id")?;
    validate_id(&expected_key.key_id, "$.expected_key.key_id")?;
    if signed.payload.principal_id != expected_key.principal_id {
        return Err(error(
            "approval.approver_key_mismatch",
            "$.payload.principal_id",
        ));
    }
    if signed.payload.key_id != expected_key.key_id {
        return Err(error(
            "approval.approver_key_mismatch",
            "$.payload.key_id",
        ));
    }

    let public_key = decode_hex::<32>(
        &expected_key.public_key_hex,
        "approval.invalid_key",
        "$.expected_key.public_key_hex",
    )?;
    let signature = decode_hex::<64>(
        &signed.signature_hex,
        "approval.invalid_signature_encoding",
        "$.signature_hex",
    )?;
    let message = canonical_approval_signing_bytes(&signed.payload)?;
    verify_ed25519_strict(&message, &public_key, &signature)?;
    Ok(VerifiedApprovalV1 {
        payload: signed.payload,
    })
}

const APPROVAL_DOMAIN: &[u8] = b"storytelling.approval.v1\0";

fn validate_payload(payload: &ApprovalPayloadV1) -> Result<(), ValidationError> {
    if payload.schema_version != 1 {
        return Err(error(
            "approval.unsupported_version",
            "$.payload.schema_version",
        ));
    }
    validate_approval_identity_v1(&payload.identity).map_err(|mut validation| {
        validation.path = format!(
            "$.payload.identity{}",
            validation.path.trim_start_matches('$')
        );
        validation
    })?;
    validate_id(&payload.approval_id, "$.payload.approval_id")?;
    validate_id(&payload.principal_id, "$.payload.principal_id")?;
    validate_id(&payload.key_id, "$.payload.key_id")?;
    if payload.approved_at_unix_ms > MAX_TIMESTAMP_UNIX_MS {
        return Err(error(
            "approval.invalid_timestamp",
            "$.payload.approved_at_unix_ms",
        ));
    }
    let nonce = payload.nonce.as_bytes();
    if !(16..=MAX_NONCE_BYTES).contains(&nonce.len())
        || !nonce.iter().all(|byte| (0x21..=0x7e).contains(byte))
    {
        return Err(error("approval.invalid_nonce", "$.payload.nonce"));
    }
    Ok(())
}

fn decode_hex<const N: usize>(
    value: &str,
    code: &str,
    path: &str,
) -> Result<[u8; N], ValidationError> {
    if value.len() != N * 2 {
        return Err(error(code, path));
    }
    let mut output = [0; N];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = decode_lower_hex_digit(pair[0]).ok_or_else(|| error(code, path))?;
        let low = decode_lower_hex_digit(pair[1]).ok_or_else(|| error(code, path))?;
        output[index] = (high << 4) | low;
    }
    Ok(output)
}

fn decode_lower_hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn verify_ed25519_strict(
    message: &[u8],
    public_key_bytes: &[u8; 32],
    signature_bytes: &[u8; 64],
) -> Result<(), ValidationError> {
    let public_key = VerifyingKey::from_bytes(public_key_bytes)
        .map_err(|_| error("approval.invalid_key", "$.expected_key.public_key_hex"))?;
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| error("approval.invalid_signature_encoding", "$.signature_hex"))?;
    public_key
        .verify_strict(message, &signature)
        .map_err(|_| error("approval.invalid_signature", "$.signature_hex"))
}

fn validate_source_rows(sources: &[SourceIdentityV1]) -> Result<(), ValidationError> {
    let mut ids = HashSet::with_capacity(sources.len());
    let mut previous: Option<&str> = None;
    for source in sources {
        validate_id(&source.asset_id, "$.sources")?;
        validate_digest(&source.sha256, "$.sources")?;
        if source.byte_length > MAX_VERIFIED_BUFFER_BYTES as u64 {
            return Err(error("approval.invalid_source_length", "$.sources"));
        }
        if !ids.insert(source.asset_id.as_str()) {
            return Err(error("approval.duplicate_id", "$.sources"));
        }
        if previous.is_some_and(|value| value >= source.asset_id.as_str()) {
            return Err(error("approval.noncanonical_order", "$.sources"));
        }
        previous = Some(&source.asset_id);
    }
    Ok(())
}

fn validate_host(host: &HostBindingV1) -> Result<(), ValidationError> {
    validate_id(&host.host_id, "$.host.host_id")?;
    validate_id(&host.profile_id, "$.host.profile_id")?;
    validate_id(&host.version, "$.host.version")?;
    if host.capabilities.len() > MAX_HOST_CAPABILITIES {
        return Err(error("approval.too_many", "$.host.capabilities"));
    }
    let mut ids = HashSet::with_capacity(host.capabilities.len());
    let mut previous: Option<&str> = None;
    for capability in &host.capabilities {
        validate_id(capability, "$.host.capabilities")?;
        if !ids.insert(capability.as_str()) {
            return Err(error("approval.duplicate_id", "$.host.capabilities"));
        }
        if previous.is_some_and(|value| value >= capability.as_str()) {
            return Err(error("approval.noncanonical_order", "$.host.capabilities"));
        }
        previous = Some(capability);
    }
    Ok(())
}

fn validate_losses(losses: &[AcceptedLossV1]) -> Result<(), ValidationError> {
    let mut previous: Option<(&str, &str, &str)> = None;
    for loss in losses {
        validate_id(&loss.feature, "$.accepted_losses")?;
        validate_text(&loss.path, MAX_LOSS_PATH_BYTES, "$.accepted_losses")?;
        if !loss.accepted || loss.disposition == LossDispositionV1::Native {
            return Err(error("approval.loss_not_accepted", "$.accepted_losses"));
        }
        let key = (
            loss.feature.as_str(),
            loss.path.as_str(),
            loss.disposition.as_str(),
        );
        if let Some(previous_key) = previous {
            if previous_key.0 == key.0 && previous_key.1 == key.1 {
                return Err(error("approval.duplicate_loss", "$.accepted_losses"));
            }
            if previous_key > key {
                return Err(error("approval.noncanonical_order", "$.accepted_losses"));
            }
        }
        previous = Some(key);
    }
    Ok(())
}

fn validate_id(value: &str, path: &str) -> Result<(), ValidationError> {
    let bytes = value.as_bytes();
    let valid = !bytes.is_empty()
        && bytes.len() <= MAX_ID_BYTES
        && bytes[0].is_ascii_alphanumeric()
        && bytes.iter().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(*byte, b'.' | b'_' | b'-' | b':' | b'+' | b'@')
        });
    if valid {
        Ok(())
    } else {
        Err(error("approval.invalid_id", path))
    }
}

fn validate_text(value: &str, limit: usize, path: &str) -> Result<(), ValidationError> {
    if !value.is_empty()
        && value.len() <= limit
        && value.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
    {
        Ok(())
    } else {
        Err(error("approval.invalid_text", path))
    }
}

fn validate_digest(value: &str, path: &str) -> Result<(), ValidationError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(error("approval.invalid_digest", path))
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .flat_map(|byte| {
            [
                char::from(b"0123456789abcdef"[usize::from(byte >> 4)]),
                char::from(b"0123456789abcdef"[usize::from(byte & 15)]),
            ]
        })
        .collect()
}

fn error(code: &str, path: &str) -> ValidationError {
    ValidationError::new(code, path)
}

#[cfg(test)]
mod tests {
    use super::{
        HostBindingV1, HostObservationStateV1, MAX_ID_BYTES, validate_builder_inputs,
        verify_ed25519_strict,
    };

    fn decode_hex(value: &str) -> Vec<u8> {
        value
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let digit = |byte: u8| match byte {
                    b'0'..=b'9' => byte - b'0',
                    b'a'..=b'f' => byte - b'a' + 10,
                    _ => panic!("invalid literal hex"),
                };
                (digit(pair[0]) << 4) | digit(pair[1])
            })
            .collect()
    }

    #[test]
    fn rfc8032_ed25519_known_answer_verifies() {
        let public_key: [u8; 32] =
            decode_hex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
                .try_into()
                .unwrap();
        let signature: [u8; 64] = decode_hex(concat!(
            "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155",
            "5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
        ))
        .try_into()
        .unwrap();
        verify_ed25519_strict(b"", &public_key, &signature).unwrap();
    }

    #[test]
    fn builder_preflight_rejects_oversized_borrowed_metadata() {
        let host = HostBindingV1 {
            host_id: "h".repeat(MAX_ID_BYTES + 1),
            profile_id: "profile".to_owned(),
            version: "1".to_owned(),
            observation: HostObservationStateV1::Declared,
            capabilities: Vec::new(),
        };

        let validation =
            validate_builder_inputs("project", "proposal", b"p", b"a", &[], &host, &[])
                .expect_err("oversized borrowed metadata must be rejected during preflight");

        assert_eq!(validation.code, "approval.invalid_id");
        assert_eq!(validation.path, "$.host.host_id");
    }
}
