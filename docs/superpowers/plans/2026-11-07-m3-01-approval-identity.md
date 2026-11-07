# PR #28 Approval Identity Candidate Implementation Plan

This plan records the separate cryptographic candidate only. The canonical
M3-01/P1 finite-OTIO review-binding task remains proposed and is not completed
by these steps.

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a bounded, versioned Ed25519 approval contract that verifies one signer-key binding against the exact current compiled-plan identity without storing keys or executing a Host operation.

**Architecture:** Add strict wire DTOs and private verified tokens in `storytelling-core`. Build the expected identity from exact proposal/artifact bytes and existing non-deserializable verified source buffers; expose canonical domain-separated signing bytes, but only verify signatures in this repository. Keep account/key authorization, platform key storage, persistence, and Host execution in later milestones.

**Tech Stack:** Rust 1.85 / edition 2024, Serde + the existing pinned `serde_json` wire codec, SHA-256, `ed25519-dalek` 3.0.0, RFC 8032 test vector.

**Spec:** `docs/superpowers/specs/2026-11-07-pr28-approval-identity-candidate-design.md`

## Global Constraints

- Keep workspace MSRV 1.85, forbid unsafe Rust, and do not add filesystem, network, media, or Host access to `storytelling-core`.
- No private key, seed, or secret may be serialized, logged, committed, or passed through the CLI.
- Only the Ed25519 verification API is used in production; canonical signing bytes may be exported for an external trusted signer. Tests may include public RFC vectors and an ephemeral signature fixture, never a private seed or key.
- A signature proves key control only. Actor authorization is only as trustworthy as the external principal/key binding passed by the caller.
- Bind the exact project, proposal bytes, compiled artifact bytes, source IDs/hashes/lengths, Host identity/capabilities, and accepted loss records.
- Preserve existing source verification, exact plan contracts, MIT licensing, Apache-2.0 component notices, and provenance.
- Do not add an approval command, database, replay ledger, apply function, executor, or “approved” boolean authority.

## Review Focus

- A correctly signed payload with any stale plan, compiled artifact, source, Host version/capability, or accepted loss must be refused.
- Duplicate, unknown, malformed, noncanonical, over-budget, or reordered identity entries must not produce a verified token.
- Source identity must be derived from `VerifiedBufferV1`, never from a caller-provided hash string alone.
- A valid signature from a different principal/key binding must be refused even when the public key itself is well formed.
- `VerifiedApprovalV1` must not be constructible, serializable, deserializable, or mutable by downstream callers.

---

### Task 1: Freeze bounded approval and compilation identity contracts

**Files:**
- Create: `crates/storytelling-core/src/approval.rs`
- Modify: `crates/storytelling-core/src/lib.rs`
- Test: `crates/storytelling-core/tests/approval.rs`

**Interfaces:**
- `CompiledPlanIdentityV1`: strict wire record for `project_id`, `proposal_id`, proposal SHA-256, compiled artifact SHA-256, ordered `SourceIdentityV1` rows, `HostBindingV1`, and ordered accepted-loss rows.
- `SourceIdentityV1`: `asset_id`, lowercase SHA-256, and a platform-neutral `u64` byte length. `HostBindingV1` carries `host_id`, `profile_id`, `version`, an explicit `declared`/`observed` claim, and sorted capabilities. `AcceptedLossV1` carries `feature`, logical `path`, disposition, and explicit `accepted: true`.
- `VerifiedSourceBufferV1<'a>` binds an ASCII asset ID to `&'a VerifiedBufferV1`; the buffer token cannot be deserialized or forged by downstream callers.
- `VerifiedCompiledPlanIdentityV1`: private-field, non-Serde token returned only by the bounded builder; exposes a read-only identity accessor.
- `build_compiled_plan_identity_v1(project_id, proposal_id, proposal_bytes, artifact_bytes, sources, host, accepted_losses)` hashes exact proposal/artifact byte slices, derives source rows from `VerifiedBufferV1`, enforces canonical IDs/order and uniqueness, and returns the private verified token.
- `validate_compiled_plan_identity_v1(identity)` applies the same structural, digest, count, string and canonical-order checks to a deserialized wire identity before it can be used in an approval payload.
- Wire limits: proposal ≤1 MiB; compiled artifact ≤4 MiB; approval envelope ≤1 MiB; ≤1,024 sources; ≤256 Host capability IDs; ≤1,024 accepted losses; ASCII identity strings ≤128 bytes; loss paths ≤512 bytes; nonce ≤128 bytes.
- Stable errors use `ValidationError` codes under `approval.*` with fixed JSON paths.

- [x] **Step 1: Write contract and boundary tests first**

Add tests named:
`source_identity_uses_verified_buffer_bytes_and_digest`,
`compiled_identity_hashes_exact_proposal_and_artifact_bytes`,
`compiled_identity_requires_sorted_unique_sources_capabilities_and_losses`,
and `compiled_identity_rejects_invalid_ids_digests_and_collection_budgets`.

Expected values use literal SHA-256 vectors and hand-written identities. Assert
that source rows cannot be built from a declared digest without a
`VerifiedBufferV1`, and that duplicate/unsorted rows return typed errors.

- [x] **Step 2: Run the new tests against the absent contract**

Run: `cargo test -p storytelling-core --test approval --locked`

Expected: after the test target exists, compilation fails because the contract
module and builders do not exist yet; this is the API-shape RED. A run before
creating the test file only reports a missing target and is not the RED proof.

- [x] **Step 3: Add minimal contract stubs so behavior tests compile**

Add the public module/types/signatures with all behavior returning a stable
`approval.not_implemented` error. Re-run the tests and confirm they compile and
fail for that typed error, not because of malformed test setup.

- [x] **Step 4: Implement strict identity DTOs and the bounded builder**

Add `#[serde(deny_unknown_fields)]` wire types and private verified identity
token in `approval.rs`. Compute lowercase SHA-256 over exact supplied plan and
artifact bytes; derive source rows only from existing `VerifiedBufferV1`;
require canonical ASCII ordering and reject duplicate IDs. Keep
Host/capability values as bound declarations without claiming they were
observed by a real Host.

- [x] **Step 5: Verify the contract tests**

Run: `cargo test -p storytelling-core --test approval --locked`

Expected: all Task 1 tests pass with stable errors and no input mutation.

### Task 2: Add canonical Ed25519 verification and opaque approval output

**Files:**
- Modify: workspace `Cargo.toml`, `crates/storytelling-core/Cargo.toml`, and `Cargo.lock`; `serde_json` is already pinned in the workspace and must be declared directly by core for JSON parsing/serialization.
- Modify: `crates/storytelling-core/src/approval.rs`
- Test: `crates/storytelling-core/tests/approval.rs`

**Interfaces:**
- `ApprovalPayloadV1`: strict envelope containing the complete compiled identity, approval ID, principal ID, key ID, bounded timestamp, and nonce.
- `SignedApprovalV1`: strict JSON record containing payload and lowercase-hex 64-byte Ed25519 signature.
- `TrustedApproverKeyV1`: explicit principal/key/public-key binding supplied by a trusted caller; not read from files or environment.
- `canonical_approval_signing_bytes(payload)`: returns `storytelling.approval.v1\0` plus deterministic serialized payload bytes after validation; it never signs.
- `verify_approval_json(bytes, expected, trusted_key)`: bounded strict parse, exact payload/identity comparison, trusted principal/key match, Ed25519 `verify_strict`, and opaque `VerifiedApprovalV1` output.
- `VerifiedApprovalV1`: private fields, no Serde implementation, read-only accessors only.

- [x] **Step 1: Write failing signature, tamper, stale-binding, and opacity tests**

Add tests named:
`rfc8032_ed25519_known_answer_verifies`,
`canonical_signing_bytes_are_domain_separated_and_deterministic`,
`rejects_modified_signature_payload_and_wrong_trusted_key`,
`rejects_stale_plan_artifact_source_host_capability_and_loss_bindings`,
`rejects_duplicate_unknown_malformed_and_oversized_approval_json`,
and compile-fail doctests proving `VerifiedApprovalV1` cannot be forged or
deserialized.

The RFC 8032 known-answer vector and expected domain-prefixed JSON bytes must be
literal test data. Mutations change one identity dimension at a time and
require stable typed rejection.

- [x] **Step 2: Run the new tests before the verifier exists**

Run: `cargo test -p storytelling-core --test approval --locked`

Expected: tests fail to compile only because the new approval types/functions
are absent; no product behavior has been assumed.

- [x] **Step 3: Add minimal approval wire/signature stubs**

Add the DTOs and public function signatures with typed
`approval.not_implemented` returns. Run the tests again and record functional
RED from each currently unimplemented behavior.

- [x] **Step 4: Add the narrowly scoped cryptographic dependency**

Pin `ed25519-dalek = 3.0.0` with default features disabled and the `fast`
verification feature. Confirm the crate MSRV matches 1.85 and its BSD-3-Clause
license; keep production code from calling signing/key-generation APIs. Update
the workspace dependency declaration and lockfile.

- [x] **Step 5: Implement canonical payload validation and strict verification**

Enforce the JSON byte, field, string, vector, ordering, and hex limits before
returning. Compare every payload identity field to
`VerifiedCompiledPlanIdentityV1`, compare trusted principal/key IDs, then verify
the domain-separated bytes with Ed25519 strict verification. Return no
`VerifiedApprovalV1` on any error.

- [x] **Step 6: Verify signature and boundary tests**

Run: `cargo test -p storytelling-core --test approval --locked`

Expected: valid RFC vector and exact binding pass; each tampered, stale,
untrusted, malformed, duplicate, or over-budget case rejects.

### Task 3: Integrate dependency review, requirements, architecture and gates

**Files:**
- Modify: `TASKS.md`, `ROADMAP.md`, `docs/CONTRACTS.md`, `docs/ARCHITECTURE.md`, `docs/REQUIREMENTS.md`, `docs/MIGRATION.md`, `scripts/check_repo.py`
- Create: `docs/migration/APPROVAL_IDENTITY_V1.md`, `docs/migration/records/M3-01-approval-candidate.md`, `docs/migration/m3-01-approval-candidate-work-index.json`, `docs/migration/dependency-review-pr28-approval-candidate.md`, `docs/validation/m3-01-approval-candidate.md`
- Do not create an upstream source-provenance manifest for first-party M3-01 code; retain registry/checksum/license/advisory evidence in the dependency review.

**Interfaces:**
- The contract guide documents exact JSON fields, signing bytes, stable errors, trust inputs, limits, and explicit non-authorities.
- The validation record distinguishes cryptographic verification from actor authorization, Host observation, replay/revocation, and execution.
- Task and requirement rows map every acceptance claim to a concrete test or mark it as a later M3-02/M3-03 gate.

- [x] **Step 1: Update the repository guard for the reviewed dependency boundary**

Update `scripts/check_repo.py` so the exact core dependency allowlist includes
only `serde`, `serde_json`, `sha2`, and `ed25519-dalek`; the workspace Ed25519
version/features are exactly pinned; and no private-key/signing capability is
added to core.
Add the M3-01 work index with only real requirement/test/path references.

- [x] **Step 2: Run the guard and observe the missing-entry failure**

Run: `python3 scripts/check_repo.py`

Expected: it reports the dependency-boundary mismatch until the guard and
dependency declarations agree; do not weaken the guard or admit unrelated
dependencies.

- [x] **Step 3: Add contract, migration, dependency review and validation documentation**

Document Ed25519 payload byte construction, trust registry responsibilities,
signature vs. identity limits, source/Host evidence provenance, error taxonomy,
dependency/license/advisory evidence and all deferred gates. Update TASKS and
ROADMAP to `in-progress`; do not mark accepted before independent review and
exact-head CI.

- [x] **Step 4: Run full local gates and dependency audit**

Run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked
cargo audit --file Cargo.lock
python3 scripts/check_repo.py
```

Expected: all commands exit 0; record actual counts, advisory database date,
dependency versions and any unresolved notices. Never suppress unrelated
advisories.

- [x] **Step 5: Commit the bounded implementation after independent review setup**

Stage only the M3-01 contract, tests and traceability files, then commit with the
configured maintainer sign-off policy. The PR must remain a draft until an
independent G2 security review and same-head three-OS CI are available.

---

## Execution Handoff

This is one cohesive Rust contract change with cryptographic and trust-boundary
risk. The plan recommends native implementation in this session, followed by an
independent whole-change security review. It does not authorize private-key
provisioning, host execution, platform signing integration, release, or merging
before review and exact-head gates.
