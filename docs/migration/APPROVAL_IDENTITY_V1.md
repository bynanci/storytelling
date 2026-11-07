<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# Approval Identity v1 (concurrent PR #28 candidate; not accepted)

This candidate is separate from the canonical proposed M3-01/P1 immutable
finite-OTIO review-binding scope in [M3-01-NEXT](M3-01-NEXT.md). It does not
satisfy that task. Independent G2 review is pending; this document describes a
candidate API, not an accepted public contract.

This is a bounded pure-core candidate contract and signature verifier. It does
not sign, store keys, authorize a person, execute an editor, or persist/replay
approvals. The candidate remains unaccepted until independent security review
and same-head hosted checks pass; even then, it does not complete canonical
M3-01/P1.

## Identity construction

`bind_approval_identity_v1` accepts project/proposal IDs, caller-supplied exact
proposal/artifact byte slices, `VerifiedSourceBufferV1` rows, a Host binding,
and accepted-loss records. Proposal bytes are capped at 1 MiB; artifact bytes at
4 MiB. Empty proposal or artifact slices are accepted and hashed as supplied;
callers that require nonempty compiled output must enforce that policy before
calling this byte-binding primitive. Source identities are derived from the existing non-deserializable
`VerifiedBufferV1` token's SHA-256 and byte length. Each source buffer keeps the
existing 16 MiB verified-buffer maximum. The builder does not re-open a file or
prove that later filesystem bytes remain unchanged. It validates byte, count,
ID, Host, source-order and loss bounds before copying caller-owned metadata into
the resulting identity. The opaque `BoundApprovalIdentityV1` means those
supplied bytes and fields passed this bounded binding step; it does not prove
that a compiler produced the artifact or that proposal, artifact and sources
came from one validated snapshot.

`ApprovalIdentityV1` is strict wire data but is untrusted after parsing. It
contains schema version 1; project and proposal IDs; lowercase SHA-256 for the
exact proposal and compiled artifact; ordered source IDs/hash/byte lengths; a
Host ID, profile, version, declared/observed claim and capability list; and full
accepted-loss `(feature, path, disposition, accepted)` rows. The builder returns
an opaque `BoundApprovalIdentityV1` only after validating the same shape.
The verifier validates the payload identity and compares every field against this
locally built token before signature verification.

## Wire bounds and canonical ordering

| Field | Rule |
| --- | --- |
| IDs | 1–128 printable ASCII token bytes; first byte alphanumeric; remaining bytes alphanumeric or `. _ - : + @` (supports email-style principal IDs without normalization) |
| Source rows | At most 1,024; strictly ascending unique `asset_id`; lowercase 64-hex digest; byte length ≤16 MiB |
| Host capability IDs | At most 256; strictly ascending and unique |
| Accepted losses | At most 1,024; strictly ascending `(feature, path, disposition)`, with no repeated `(feature, path)` pair; logical ASCII path ≤512 bytes; each row must be accepted and cannot have `native` disposition |
| Proposal / artifact bytes at identity build | ≤1 MiB / ≤4 MiB |
| Approval JSON envelope | ≤1 MiB, strict typed fields, duplicate/unknown fields rejected |
| Approval/key/signature IDs | Same bounded ID rule; key is 32 bytes / 64 lowercase hex; signature is 64 bytes / 128 lowercase hex |
| Timestamp | Unsigned milliseconds ≤253,402,300,799,999; this is a representable-range check, not expiry policy |
| Nonce | 16–128 printable non-space ASCII bytes; entropy and replay protection are caller/M3-02 responsibilities |

DTOs contain no maps or floating-point values. Canonical signing bytes are the
fixed byte prefix `storytelling.approval.v1\0` followed by compact JSON emitted
by the repository-pinned `serde_json` over the frozen `ApprovalPayloadV1` field
order. Arrays have the canonical ordering above. A future cross-language signer
must reproduce these exact bytes; semantic JSON equality is insufficient. The
canonicalization version is bound by the domain prefix and schema version.

## Signed approval and trust input

`SignedApprovalV1` is strict JSON with `payload` and lowercase `signature_hex`.
`ApprovalPayloadV1` binds its own schema version, complete approval identity,
approval ID, principal ID, key ID, bounded timestamp and nonce. The caller passes
`ExpectedApproverKeyV1` containing an explicit principal/key/public-key binding.
The caller is responsible for obtaining this expected binding from its own
trusted source. Core validates Ed25519 key and signature encodings, compares
principal/key IDs, then calls Ed25519 `verify_strict` on the domain-separated
bytes; it does not perform registry lookup or authorization.

Stable errors use `ValidationError { code, path }` with `approval.*` codes,
including `invalid_json`, `too_large`, `unsupported_version`, `invalid_id`,
`invalid_digest`, `invalid_source_length`, `too_many`,
`noncanonical_order`, `duplicate_id`, `duplicate_loss`, `loss_not_accepted`,
`invalid_timestamp`, `invalid_nonce`, `identity_mismatch`,
`approver_key_mismatch`,
`invalid_key`, `invalid_signature_encoding`, and `invalid_signature`.
Callers branch on codes and paths, not prose.

## Verified result and non-authorities

`VerifiedApprovalV1` has private fields, no Serialize/Deserialize implementation,
and exposes only immutable payload access. Its construction proves that the
provided expected public key strictly verifies the exact payload and that the payload
matches the provided current identity token. It does not establish that the
expected key came from a trusted registry, that a human principal is authorized,
that a Host `observed` claim is true, that a key is active or unrevoked, that a
nonce has not been replayed, that an approval is unexpired, or that any operation
was executed. The token is not a serialized bearer credential.

No private key or seed is stored in the repository, fixture, test output or CLI.
The signer/keychain, authorization registry, persistence/replay ledger,
revocation/expiry policy and Host execution are outside this candidate. M3-02 owns durable
operation/owner fencing; M3-03 owns actual Host observations and execution.
