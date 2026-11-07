// SPDX-License-Identifier: Apache-2.0
// Independently authored vectors and buffer-boundary regression; see provenance-m1-03.json.
use storytelling_core::source_evidence::{MAX_VERIFIED_BUFFER_BYTES, verify_supplied_sha256};

#[test]
fn accepts_independent_sha256_vectors_and_owns_verified_bytes() {
    for (bytes, expected) in [
        (
            b"".as_slice(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            b"abc".as_slice(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            [0, 255, 0, 128].as_slice(),
            "787a4bce1c8b1ef7f111820f3840c98b4ddf51d55f5e7a9b007fb6224bb563bc",
        ),
    ] {
        let verified = verify_supplied_sha256(bytes, expected).unwrap();
        assert_eq!(verified.bytes(), bytes);
        assert_eq!(verified.sha256(), expected);
        assert_eq!(verified.byte_length(), bytes.len());
    }
    let mut original = b"abc".to_vec();
    let verified = verify_supplied_sha256(
        &original,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    )
    .unwrap();
    original[0] = b'z';
    assert_eq!(verified.bytes(), b"abc");
    assert!(
        verify_supplied_sha256(
            &vec![b'a'; 1_000_000],
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        )
        .is_ok()
    );
}

#[test]
fn rejects_mismatched_and_noncanonical_expected_hashes() {
    let mismatch = verify_supplied_sha256(b"abc", &"0".repeat(64))
        .err()
        .unwrap();
    assert_eq!(mismatch.code, "source.hash_mismatch");
    assert_eq!(mismatch.path, "$.expected_sha256");
    for expected in [
        String::new(),
        "a".repeat(63),
        "a".repeat(65),
        "A".repeat(64),
        "g".repeat(64),
        format!(" {}", "a".repeat(64)),
        format!("sha256:{}", "a".repeat(64)),
    ] {
        let error = verify_supplied_sha256(b"abc", &expected).err().unwrap();
        assert_eq!(error.code, "source.invalid_expected_hash");
        assert_eq!(error.path, "$.expected_sha256");
    }
}

#[test]
fn accepts_exact_buffer_budget_and_rejects_larger_before_hash_syntax() {
    assert_eq!(MAX_VERIFIED_BUFFER_BYTES, 16_777_216);
    let bytes = vec![0; MAX_VERIFIED_BUFFER_BYTES];
    let verified = verify_supplied_sha256(
        &bytes,
        "080acf35a507ac9849cfcba47dc2ad83e01b75663a516279c8b9d243b719643e",
    )
    .unwrap();
    assert_eq!(verified.byte_length(), MAX_VERIFIED_BUFFER_BYTES);
    let error = verify_supplied_sha256(&vec![0; MAX_VERIFIED_BUFFER_BYTES + 1], "invalid")
        .err()
        .unwrap();
    assert_eq!(error.code, "source.too_large");
    assert_eq!(error.path, "$");
}
