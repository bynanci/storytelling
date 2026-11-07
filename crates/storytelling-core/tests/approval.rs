// SPDX-License-Identifier: MIT
//! Approval identity contract boundary tests. All source bytes are synthetic.
use storytelling_core::ValidationError;
use storytelling_core::approval::{
    AcceptedLossV1, ApprovalIdentityV1, ApprovalPayloadV1, BoundApprovalIdentityV1,
    ExpectedApproverKeyV1, HostBindingV1, HostObservationStateV1, LossDispositionV1,
    SignedApprovalV1, VerifiedApprovalV1, VerifiedSourceBufferV1, bind_approval_identity_v1,
    canonical_approval_signing_bytes, validate_approval_identity_v1, verify_approval_json,
};
use storytelling_core::source_evidence::verify_supplied_sha256;

const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn verified(bytes: &[u8]) -> storytelling_core::source_evidence::VerifiedBufferV1 {
    let digest = if bytes == b"abc" {
        ABC_SHA256
    } else {
        EMPTY_SHA256
    };
    verify_supplied_sha256(bytes, digest).expect("synthetic source verifies")
}

fn host(capabilities: &[&str]) -> HostBindingV1 {
    HostBindingV1 {
        host_id: "filmcraft".to_owned(),
        profile_id: "isolated25fps".to_owned(),
        version: "0.18.0".to_owned(),
        observation: HostObservationStateV1::Declared,
        capabilities: capabilities
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
    }
}

fn loss(feature: &str, path: &str, disposition: LossDispositionV1) -> AcceptedLossV1 {
    AcceptedLossV1 {
        feature: feature.to_owned(),
        path: path.to_owned(),
        disposition,
        accepted: true,
    }
}

fn identity(
    proposal: &[u8],
    artifact: &[u8],
    sources: &[VerifiedSourceBufferV1<'_>],
    host: &HostBindingV1,
    losses: &[AcceptedLossV1],
) -> storytelling_core::approval::BoundApprovalIdentityV1 {
    bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        proposal,
        artifact,
        sources,
        host,
        losses,
    )
    .expect("valid bounded approval identity")
}

fn assert_error(error: ValidationError, code: &str, path: &str) {
    assert_eq!(error.code, code);
    assert_eq!(error.path, path);
}

#[test]
fn source_identity_uses_verified_buffer_bytes_and_digest() {
    let verified = verify_supplied_sha256(b"abc", ABC_SHA256).unwrap();
    let source = VerifiedSourceBufferV1::new("asset-a", &verified).unwrap();
    let identity = identity(b"proposal", b"artifact", &[source], &host(&[]), &[]);

    let source = &identity.identity().sources[0];
    assert_eq!(source.asset_id, "asset-a");
    assert_eq!(source.sha256, ABC_SHA256);
    assert_eq!(source.byte_length, 3);
}

#[test]
fn bound_identity_hashes_exact_bytes_and_accepts_empty_slices() {
    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let host = host(&[]);
    let first = identity(b"abc", b"", &[source], &host, &[]);
    assert_eq!(first.identity().proposal_sha256, ABC_SHA256);
    assert_eq!(first.identity().compiled_artifact_sha256, EMPTY_SHA256);

    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let second = identity(b"abc ", b"", &[source], &host, &[]);
    assert_ne!(
        first.identity().proposal_sha256,
        second.identity().proposal_sha256
    );

    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let nonempty_artifact = identity(b"abc", b"artifact", &[source], &host, &[]);
    assert_ne!(
        first.identity().compiled_artifact_sha256,
        nonempty_artifact.identity().compiled_artifact_sha256
    );

    let empty_slices = identity(b"", b"", &[], &host, &[]);
    assert_eq!(empty_slices.identity().proposal_sha256, EMPTY_SHA256);
    assert_eq!(
        empty_slices.identity().compiled_artifact_sha256,
        EMPTY_SHA256
    );
}

#[test]
fn bound_identity_accepts_exact_proposal_and_artifact_size_limits() {
    let proposal = vec![b'p'; storytelling_core::approval::MAX_PROPOSAL_BYTES];
    let artifact = vec![b'a'; storytelling_core::approval::MAX_COMPILED_ARTIFACT_BYTES];
    let identity = bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        &proposal,
        &artifact,
        &[],
        &host(&[]),
        &[],
    )
    .unwrap();

    assert_eq!(identity.identity().schema_version, 1);
}

#[test]
fn approval_identity_requires_sorted_unique_sources_capabilities_and_losses() {
    let bytes_a = verified(b"abc");
    let bytes_b = verified(b"");
    let source_a = VerifiedSourceBufferV1::new("asset-a", &bytes_a).unwrap();
    let source_b = VerifiedSourceBufferV1::new("asset-b", &bytes_b).unwrap();
    let valid_host = host(&["audio.read", "timeline.write"]);
    let losses = [
        loss(
            "audio.effects",
            "$.audio.effects",
            LossDispositionV1::MetadataOnly,
        ),
        loss(
            "video.keyframes",
            "$.video.keyframes",
            LossDispositionV1::Unsupported,
        ),
    ];
    identity(b"p", b"a", &[source_a, source_b], &valid_host, &losses);

    let bytes_a = verified(b"abc");
    let bytes_b = verified(b"");
    let source_a = VerifiedSourceBufferV1::new("asset-a", &bytes_a).unwrap();
    let source_b = VerifiedSourceBufferV1::new("asset-b", &bytes_b).unwrap();
    let error = bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        b"p",
        b"a",
        &[source_b, source_a],
        &valid_host,
        &losses,
    )
    .unwrap_err();
    assert_error(error, "approval.noncanonical_order", "$.sources");

    let bytes_a = verified(b"abc");
    let bytes_duplicate = verified(b"");
    let source_a = VerifiedSourceBufferV1::new("asset-a", &bytes_a).unwrap();
    let duplicate = VerifiedSourceBufferV1::new("asset-a", &bytes_duplicate).unwrap();
    let error = bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        b"p",
        b"a",
        &[source_a, duplicate],
        &valid_host,
        &losses,
    )
    .unwrap_err();
    assert_error(error, "approval.duplicate_id", "$.sources");

    let bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let unsorted_host = host(&["timeline.write", "audio.read"]);
    let error = bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        b"p",
        b"a",
        &[source],
        &unsorted_host,
        &[],
    )
    .unwrap_err();
    assert_error(error, "approval.noncanonical_order", "$.host.capabilities");

    let bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let duplicate_host = host(&["audio.read", "audio.read"]);
    let error = bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        b"p",
        b"a",
        &[source],
        &duplicate_host,
        &[],
    )
    .unwrap_err();
    assert_error(error, "approval.duplicate_id", "$.host.capabilities");

    let bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let unsorted_losses = [
        loss(
            "video.keyframes",
            "$.video.keyframes",
            LossDispositionV1::Unsupported,
        ),
        loss(
            "audio.effects",
            "$.audio.effects",
            LossDispositionV1::MetadataOnly,
        ),
    ];
    let error = bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        b"p",
        b"a",
        &[source],
        &host(&[]),
        &unsorted_losses,
    )
    .unwrap_err();
    assert_error(error, "approval.noncanonical_order", "$.accepted_losses");

    let bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let conflicting_losses = [
        loss(
            "video.keyframes",
            "$.video.keyframes",
            LossDispositionV1::MetadataOnly,
        ),
        loss(
            "video.keyframes",
            "$.video.keyframes",
            LossDispositionV1::Unsupported,
        ),
    ];
    let error = bind_approval_identity_v1(
        "project-1",
        "proposal-1",
        b"p",
        b"a",
        &[source],
        &valid_host,
        &conflicting_losses,
    )
    .unwrap_err();
    assert_error(error, "approval.duplicate_loss", "$.accepted_losses");
}

#[test]
fn approval_identity_rejects_invalid_ids_digests_and_collection_budgets() {
    let bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let invalid = bind_approval_identity_v1(
        "project with spaces",
        "proposal-1",
        b"p",
        b"a",
        &[source],
        &host(&[]),
        &[],
    )
    .unwrap_err();
    assert_error(invalid, "approval.invalid_id", "$.project_id");

    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let valid = identity(b"p", b"a", &[source], &host(&[]), &[]);
    let mut invalid_digest: ApprovalIdentityV1 = valid.identity().clone();
    invalid_digest.proposal_sha256 = "A".repeat(64);
    assert_error(
        validate_approval_identity_v1(&invalid_digest).unwrap_err(),
        "approval.invalid_digest",
        "$.proposal_sha256",
    );
    let mut invalid_source_length: ApprovalIdentityV1 = valid.identity().clone();
    invalid_source_length.sources[0].byte_length =
        storytelling_core::source_evidence::MAX_VERIFIED_BUFFER_BYTES as u64 + 1;
    assert_error(
        validate_approval_identity_v1(&invalid_source_length).unwrap_err(),
        "approval.invalid_source_length",
        "$.sources",
    );

    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let oversized_proposal = vec![b'p'; storytelling_core::approval::MAX_PROPOSAL_BYTES + 1];
    assert_error(
        bind_approval_identity_v1(
            "project-1",
            "proposal-1",
            &oversized_proposal,
            b"a",
            &[source],
            &host(&[]),
            &[],
        )
        .unwrap_err(),
        "approval.proposal_too_large",
        "$.proposal_bytes",
    );

    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let oversized_artifact =
        vec![b'a'; storytelling_core::approval::MAX_COMPILED_ARTIFACT_BYTES + 1];
    assert_error(
        bind_approval_identity_v1(
            "project-1",
            "proposal-1",
            b"p",
            &oversized_artifact,
            &[source],
            &host(&[]),
            &[],
        )
        .unwrap_err(),
        "approval.artifact_too_large",
        "$.compiled_artifact_bytes",
    );

    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let capabilities: Vec<String> = (0..=storytelling_core::approval::MAX_HOST_CAPABILITIES)
        .map(|index| format!("capability-{index:04}"))
        .collect();
    let oversized_host = HostBindingV1 {
        capabilities,
        ..host(&[])
    };
    assert_error(
        bind_approval_identity_v1(
            "project-1",
            "proposal-1",
            b"p",
            b"a",
            &[source],
            &oversized_host,
            &[],
        )
        .unwrap_err(),
        "approval.too_many",
        "$.host.capabilities",
    );

    let source_bytes = verified(b"abc");
    let source_ids: Vec<_> = (0..=storytelling_core::approval::MAX_SOURCES)
        .map(|index| format!("asset-{index:04}"))
        .collect();
    let too_many_sources: Vec<_> = source_ids
        .iter()
        .map(|asset_id| VerifiedSourceBufferV1::new(asset_id, &source_bytes).unwrap())
        .collect();
    assert_error(
        bind_approval_identity_v1(
            "project-1",
            "proposal-1",
            b"p",
            b"a",
            &too_many_sources,
            &host(&[]),
            &[],
        )
        .unwrap_err(),
        "approval.too_many",
        "$.sources",
    );

    let source_bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &source_bytes).unwrap();
    let too_many_losses: Vec<_> = (0..=storytelling_core::approval::MAX_ACCEPTED_LOSSES)
        .map(|index| {
            loss(
                &format!("feature-{index:04}"),
                "$.feature",
                LossDispositionV1::Unsupported,
            )
        })
        .collect();
    assert_error(
        bind_approval_identity_v1(
            "project-1",
            "proposal-1",
            b"p",
            b"a",
            &[source],
            &host(&[]),
            &too_many_losses,
        )
        .unwrap_err(),
        "approval.too_many",
        "$.accepted_losses",
    );
}

const APPROVAL_ID: &str = "approval-1";
const PRINCIPAL_ID: &str = "user-1";
const KEY_ID: &str = "key-1";
const APPROVED_AT: u64 = 1_794_000_000_000;
const NONCE: &str = "0123456789abcdef";
// The signature was created with a one-off local test key; only its public key
// and signature are retained. No private test key or seed is stored.
const TEST_PUBLIC_KEY_HEX: &str =
    "bfd29ebb8895207fe787080d3cdc9a2283f17f418465c73d61e90e6f0fc90304";
const TEST_SIGNATURE_HEX: &str = "a993cec85e8a45bc518bc641cb90a36e35aaf68d016b420843267d7e37e564c49d4eb0a140136f60e9658a332b1c3f3360076e991625a774d723e98321715507";

fn base_approval_identity() -> BoundApprovalIdentityV1 {
    let bytes = verified(b"abc");
    let source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    identity(b"abc", b"", &[source], &host(&[]), &[])
}

fn approval_payload(identity: &BoundApprovalIdentityV1) -> ApprovalPayloadV1 {
    ApprovalPayloadV1 {
        schema_version: 1,
        identity: identity.identity().clone(),
        approval_id: APPROVAL_ID.to_owned(),
        principal_id: PRINCIPAL_ID.to_owned(),
        key_id: KEY_ID.to_owned(),
        approved_at_unix_ms: APPROVED_AT,
        nonce: NONCE.to_owned(),
    }
}

fn expected_key() -> ExpectedApproverKeyV1 {
    ExpectedApproverKeyV1 {
        principal_id: PRINCIPAL_ID.to_owned(),
        key_id: KEY_ID.to_owned(),
        public_key_hex: TEST_PUBLIC_KEY_HEX.to_owned(),
    }
}

fn signed_json(identity: &BoundApprovalIdentityV1) -> Vec<u8> {
    serde_json::to_vec(&SignedApprovalV1 {
        payload: approval_payload(identity),
        signature_hex: TEST_SIGNATURE_HEX.to_owned(),
    })
    .unwrap()
}

#[test]
fn canonical_signing_bytes_are_domain_separated_and_deterministic() {
    let payload = approval_payload(&base_approval_identity());
    let first = canonical_approval_signing_bytes(&payload).unwrap();
    let second = canonical_approval_signing_bytes(&payload).unwrap();
    assert_eq!(first, second);
    assert!(first.starts_with(b"storytelling.approval.v1\0"));
    let json = &first[b"storytelling.approval.v1\0".len()..];
    assert_eq!(json.first(), Some(&b'{'));
    assert_eq!(json.last(), Some(&b'}'));
    assert!(
        std::str::from_utf8(json)
            .unwrap()
            .contains("\"approved_at_unix_ms\":1794000000000")
    );
    assert_eq!(
        json,
        br#"{"schema_version":1,"identity":{"schema_version":1,"project_id":"project-1","proposal_id":"proposal-1","proposal_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","compiled_artifact_sha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","sources":[{"asset_id":"asset-a","sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","byte_length":3}],"host":{"host_id":"filmcraft","profile_id":"isolated25fps","version":"0.18.0","observation":"declared","capabilities":[]},"accepted_losses":[]},"approval_id":"approval-1","principal_id":"user-1","key_id":"key-1","approved_at_unix_ms":1794000000000,"nonce":"0123456789abcdef"}"#
    );
}

#[test]
fn signed_approval_verifies_to_an_opaque_read_only_token() {
    let expected = base_approval_identity();
    let token: VerifiedApprovalV1 =
        verify_approval_json(&signed_json(&expected), &expected, &expected_key()).unwrap();
    assert_eq!(token.payload().approval_id, APPROVAL_ID);
    assert_eq!(token.payload().principal_id, PRINCIPAL_ID);
}

#[test]
fn rejects_modified_signature_payload_and_wrong_expected_key() {
    let expected = base_approval_identity();
    let signed = SignedApprovalV1 {
        payload: approval_payload(&expected),
        signature_hex: TEST_SIGNATURE_HEX.to_owned(),
    };

    let mut modified_signature = signed_json(&expected);
    let signature_offset = modified_signature
        .windows(TEST_SIGNATURE_HEX.len())
        .position(|window| window == TEST_SIGNATURE_HEX.as_bytes())
        .unwrap();
    modified_signature[signature_offset] = if modified_signature[signature_offset] == b'0' {
        b'1'
    } else {
        b'0'
    };
    assert_error(
        verify_approval_json(&modified_signature, &expected, &expected_key()).unwrap_err(),
        "approval.invalid_signature",
        "$.signature_hex",
    );

    let mut changed_payload = signed.payload;
    changed_payload.approval_id = "approval-2".to_owned();
    let changed_json = serde_json::to_vec(&SignedApprovalV1 {
        payload: changed_payload,
        signature_hex: TEST_SIGNATURE_HEX.to_owned(),
    })
    .unwrap();
    assert_error(
        verify_approval_json(&changed_json, &expected, &expected_key()).unwrap_err(),
        "approval.invalid_signature",
        "$.signature_hex",
    );

    let mut wrong_key = expected_key();
    wrong_key.principal_id = "someone-else".to_owned();
    assert_error(
        verify_approval_json(&signed_json(&expected), &expected, &wrong_key).unwrap_err(),
        "approval.approver_key_mismatch",
        "$.payload.principal_id",
    );

    let mut wrong_key = expected_key();
    wrong_key.key_id = "different-key".to_owned();
    assert_error(
        verify_approval_json(&signed_json(&expected), &expected, &wrong_key).unwrap_err(),
        "approval.approver_key_mismatch",
        "$.payload.key_id",
    );

    let malformed_signature = serde_json::to_vec(&SignedApprovalV1 {
        payload: approval_payload(&expected),
        signature_hex: "00".to_owned(),
    })
    .unwrap();
    assert_error(
        verify_approval_json(&malformed_signature, &expected, &expected_key()).unwrap_err(),
        "approval.invalid_signature_encoding",
        "$.signature_hex",
    );

    let mut wrong_public_key = expected_key();
    wrong_public_key.public_key_hex =
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a".to_owned();
    assert_error(
        verify_approval_json(&signed_json(&expected), &expected, &wrong_public_key).unwrap_err(),
        "approval.invalid_signature",
        "$.signature_hex",
    );

    let mut malformed_public_key = expected_key();
    malformed_public_key.public_key_hex = "00".to_owned();
    assert_error(
        verify_approval_json(&signed_json(&expected), &expected, &malformed_public_key)
            .unwrap_err(),
        "approval.invalid_key",
        "$.expected_key.public_key_hex",
    );
}

#[test]
fn rejects_stale_plan_artifact_source_host_capability_and_loss_bindings() {
    let base = base_approval_identity();
    let signed = signed_json(&base);
    let expected_key = expected_key();
    let bytes = verified(b"abc");
    let base_host = host(&[]);

    let project_source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let changed_project = bind_approval_identity_v1(
        "project-2",
        "proposal-1",
        b"abc",
        b"",
        &[project_source],
        &base_host,
        &[],
    )
    .unwrap();
    let proposal_id_source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let changed_proposal_id = bind_approval_identity_v1(
        "project-1",
        "proposal-2",
        b"abc",
        b"",
        &[proposal_id_source],
        &base_host,
        &[],
    )
    .unwrap();
    let source_id_source = VerifiedSourceBufferV1::new("asset-b", &bytes).unwrap();
    let changed_source_id = identity(b"abc", b"", &[source_id_source], &base_host, &[]);

    let plan_source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let changed_plan = identity(b"changed", b"", &[plan_source], &base_host, &[]);
    let artifact_source = VerifiedSourceBufferV1::new("asset-a", &bytes).unwrap();
    let changed_artifact = identity(b"abc", b"changed", &[artifact_source], &base_host, &[]);
    let changed_source_bytes = verified(b"");
    let changed_source = VerifiedSourceBufferV1::new("asset-a", &changed_source_bytes).unwrap();
    let changed_source = identity(b"abc", b"", &[changed_source], &base_host, &[]);
    let changed_host_bytes = verified(b"abc");
    let changed_host_source = VerifiedSourceBufferV1::new("asset-a", &changed_host_bytes).unwrap();
    let changed_host = HostBindingV1 {
        version: "0.19.0".to_owned(),
        ..base_host.clone()
    };
    let changed_host = identity(b"abc", b"", &[changed_host_source], &changed_host, &[]);
    let changed_host_id_bytes = verified(b"abc");
    let changed_host_id_source =
        VerifiedSourceBufferV1::new("asset-a", &changed_host_id_bytes).unwrap();
    let changed_host_id_binding = HostBindingV1 {
        host_id: "different-host".to_owned(),
        ..base_host.clone()
    };
    let changed_host_id = identity(
        b"abc",
        b"",
        &[changed_host_id_source],
        &changed_host_id_binding,
        &[],
    );
    let changed_profile_bytes = verified(b"abc");
    let changed_profile_source =
        VerifiedSourceBufferV1::new("asset-a", &changed_profile_bytes).unwrap();
    let changed_profile_binding = HostBindingV1 {
        profile_id: "different-profile".to_owned(),
        ..base_host.clone()
    };
    let changed_profile = identity(
        b"abc",
        b"",
        &[changed_profile_source],
        &changed_profile_binding,
        &[],
    );
    let changed_observation_bytes = verified(b"abc");
    let changed_observation_source =
        VerifiedSourceBufferV1::new("asset-a", &changed_observation_bytes).unwrap();
    let changed_observation_binding = HostBindingV1 {
        observation: HostObservationStateV1::Observed,
        ..base_host.clone()
    };
    let changed_observation = identity(
        b"abc",
        b"",
        &[changed_observation_source],
        &changed_observation_binding,
        &[],
    );
    let capability_bytes = verified(b"abc");
    let capability_source = VerifiedSourceBufferV1::new("asset-a", &capability_bytes).unwrap();
    let capability_host = host(&["timeline.read"]);
    let changed_capabilities = identity(b"abc", b"", &[capability_source], &capability_host, &[]);
    let loss_bytes = verified(b"abc");
    let loss_source = VerifiedSourceBufferV1::new("asset-a", &loss_bytes).unwrap();
    let accepted_losses = [loss(
        "video.keyframes",
        "$.video.keyframes",
        LossDispositionV1::Unsupported,
    )];
    let changed_loss = identity(b"abc", b"", &[loss_source], &base_host, &accepted_losses);

    for stale in [
        changed_project,
        changed_proposal_id,
        changed_source_id,
        changed_plan,
        changed_artifact,
        changed_source,
        changed_host,
        changed_host_id,
        changed_profile,
        changed_observation,
        changed_capabilities,
        changed_loss,
    ] {
        assert_error(
            verify_approval_json(&signed, &stale, &expected_key).unwrap_err(),
            "approval.identity_mismatch",
            "$.payload.identity",
        );
    }
}

#[test]
fn rejects_duplicate_unknown_malformed_and_oversized_approval_json() {
    let expected = base_approval_identity();
    let valid = signed_json(&expected);
    let text = std::str::from_utf8(&valid).unwrap();
    let duplicate = text.replacen("{\"payload\":", "{\"payload\":{},\"payload\":", 1);
    assert_error(
        verify_approval_json(duplicate.as_bytes(), &expected, &expected_key()).unwrap_err(),
        "approval.invalid_json",
        "$",
    );
    let unknown = text.replacen("{\"payload\":", "{\"unknown\":true,\"payload\":", 1);
    assert_error(
        verify_approval_json(unknown.as_bytes(), &expected, &expected_key()).unwrap_err(),
        "approval.invalid_json",
        "$",
    );
    let duplicate_nested = text.replacen(
        "\"schema_version\":1,",
        "\"schema_version\":1,\"schema_version\":1,",
        1,
    );
    assert_error(
        verify_approval_json(duplicate_nested.as_bytes(), &expected, &expected_key()).unwrap_err(),
        "approval.invalid_json",
        "$",
    );
    let unknown_nested = text.replacen(
        "\"project_id\":\"project-1\",",
        "\"project_id\":\"project-1\",\"future\":0,",
        1,
    );
    assert_error(
        verify_approval_json(unknown_nested.as_bytes(), &expected, &expected_key()).unwrap_err(),
        "approval.invalid_json",
        "$",
    );
    assert_error(
        verify_approval_json(b"{not-json", &expected, &expected_key()).unwrap_err(),
        "approval.invalid_json",
        "$",
    );
    let oversized = vec![b' '; storytelling_core::approval::MAX_APPROVAL_JSON_BYTES + 1];
    assert_error(
        verify_approval_json(&oversized, &expected, &expected_key()).unwrap_err(),
        "approval.too_large",
        "$",
    );
}

#[test]
fn rejects_out_of_range_timestamp_short_nonce_and_unaccepted_loss() {
    let expected = base_approval_identity();
    let mut payload = approval_payload(&expected);
    payload.approved_at_unix_ms = storytelling_core::approval::MAX_TIMESTAMP_UNIX_MS + 1;
    assert_error(
        canonical_approval_signing_bytes(&payload).unwrap_err(),
        "approval.invalid_timestamp",
        "$.payload.approved_at_unix_ms",
    );

    let mut payload = approval_payload(&expected);
    payload.nonce = "too-short".to_owned();
    assert_error(
        canonical_approval_signing_bytes(&payload).unwrap_err(),
        "approval.invalid_nonce",
        "$.payload.nonce",
    );

    let mut payload = approval_payload(&expected);
    payload.identity.accepted_losses.push(loss(
        "video.keyframes",
        "$.video.keyframes",
        LossDispositionV1::Unsupported,
    ));
    payload.identity.accepted_losses[0].accepted = false;
    assert_error(
        canonical_approval_signing_bytes(&payload).unwrap_err(),
        "approval.loss_not_accepted",
        "$.payload.identity.accepted_losses",
    );
}

#[test]
fn accepts_email_style_principal_identifiers_without_normalizing_them() {
    let expected = base_approval_identity();
    let mut payload = approval_payload(&expected);
    payload.principal_id = "by.nanci.tw@gmail.com".to_owned();
    let bytes = canonical_approval_signing_bytes(&payload).unwrap();
    assert!(
        std::str::from_utf8(&bytes)
            .unwrap()
            .contains("by.nanci.tw@gmail.com")
    );
}
