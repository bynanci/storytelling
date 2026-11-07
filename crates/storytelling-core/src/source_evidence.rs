// SPDX-License-Identifier: Apache-2.0
// Source-informed bounded native buffer boundary; see provenance-m1-03.json.
use crate::ValidationError;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub const MAX_VERIFIED_BUFFER_BYTES: usize = 16_777_216;

/// Hashes bytes already held in memory. This reports no source or authority claim.
pub fn sha256_digest(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

#[derive(Serialize)]
/// An owned, immutable buffer whose supplied expected SHA-256 matched.
/// Serialization exports a report, never bytes or a reusable trust token.
///
/// ```compile_fail
/// use storytelling_core::source_evidence::VerifiedBufferV1;
/// let forged = VerifiedBufferV1 { schema_version: 1 };
/// ```
///
/// ```compile_fail
/// use storytelling_core::source_evidence::VerifiedBufferV1;
/// fn untrusted<'de, T: serde::Deserialize<'de>>() {}
/// untrusted::<VerifiedBufferV1>();
/// ```
pub struct VerifiedBufferV1 {
    schema_version: u32,
    algorithm: &'static str,
    scope: &'static str,
    byte_length: usize,
    sha256: String,
    #[serde(skip_serializing)]
    bytes: Box<[u8]>,
}

impl VerifiedBufferV1 {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }
    pub fn sha256(&self) -> &str {
        &self.sha256
    }
}

/// Verifies only caller-supplied bytes; performs no path or host access.
pub fn verify_supplied_sha256(
    bytes: &[u8],
    expected_sha256: &str,
) -> Result<VerifiedBufferV1, ValidationError> {
    if bytes.len() > MAX_VERIFIED_BUFFER_BYTES {
        return Err(ValidationError::new("source.too_large", "$"));
    }
    if expected_sha256.len() != 64
        || !expected_sha256
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(ValidationError::new(
            "source.invalid_expected_hash",
            "$.expected_sha256",
        ));
    }
    let digest = sha256_digest(bytes);
    let sha256: String = digest
        .iter()
        .flat_map(|byte| {
            [
                char::from(b"0123456789abcdef"[usize::from(byte >> 4)]),
                char::from(b"0123456789abcdef"[usize::from(byte & 15)]),
            ]
        })
        .collect();
    if sha256 != expected_sha256 {
        return Err(ValidationError::new(
            "source.hash_mismatch",
            "$.expected_sha256",
        ));
    }
    Ok(VerifiedBufferV1 {
        schema_version: 1,
        algorithm: "sha256",
        scope: "provided-buffer-bytes",
        byte_length: bytes.len(),
        sha256,
        bytes: bytes.into(),
    })
}
