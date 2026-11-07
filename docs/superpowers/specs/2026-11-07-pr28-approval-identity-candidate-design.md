<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# PR #28 Approval Identity Candidate

This candidate is distinct from canonical M3-01/P1 immutable finite-OTIO
review binding. It does not complete that proposed task.

## Intent

Define a portable signed approval record that binds one reviewer key to one
exact proposal compilation, its verified source identities, a Host capability
snapshot, and the explicitly accepted losses. M3-01 supplies contract and
verification primitives. It must not turn a serialized `approved: true` field
into authority or execute a Host operation.

The roadmap only requires approval and compiled-plan identity. The user chose
to include signer identity and selected an Ed25519 contract, while keeping
Windows/macOS key storage outside this slice.

## Goals

- Freeze strict, versioned, bounded DTOs for a compiled-plan identity, a signed
  approval payload, and an externally trusted approver key.
- Sign deterministic domain-separated bytes so the signature cannot be reused
  as an unrelated protocol message.
- Verify Ed25519 signatures and compare every identity field against the
  current compiled-plan identity before returning a private verified token.
- Bind project, proposal digest, compiled artifact digest, source identities,
  Host profile/version/capability declarations, accepted loss details, signer
  principal/key identifiers, and a caller-generated nonce/timestamp.
- Keep source verification and Host capability observation as separately
  supplied evidence. The verifier never upgrades declarations to verified
  facts.
- Reject malformed, duplicate, stale, tampered, mismatched, oversized, or
  untrusted-key approvals with stable typed errors and no partial receipt.

## Non-goals

- Generating, storing, exporting, rotating, revoking, or authorizing private
  keys or actor accounts.
- Windows DPAPI, macOS Keychain/Secure Enclave, TPM, remote signing, or login.
- A CLI signing/apply command, approval database, replay ledger, expiration
  policy, operation journal, Host executor, or rollback.
- Proving that source bytes still exist after supplied-buffer verification or
  that a declared Host capability is true. Later adapters must provide those
  observations and M3-02 must handle replay/revocation persistence.
- Treating an Ed25519 key as a human identity without a trusted external mapping
  from `(principal_id, key_id)` to an active authorized public key.

## Approaches

1. **Ed25519 detached signature (selected).** The core serializes only a
   canonical public payload and verifies an externally created signature.
   Private keys stay in a future trusted Desktop/OS signer; a caller passes the
   active key binding from a trusted registry. This is cross-platform and lets
   M3-01 test cryptographic binding now. The registry and OS integration remain
   explicit future work.
2. **Platform-backed signing now.** Use DPAPI/Keychain/TPM directly in this
   slice. This would bind M3-01 to desktop platforms, require Windows/macOS
   workstation evidence, and pull UI/key lifecycle work ahead of the workflow
   journal. It is deferred to a trusted-surface task.
3. **Shared-secret MAC.** This is smaller but any verifier holding the same
   secret can mint approvals, so it cannot establish distinct signer identity.
   It is rejected.

## Contract and state flow

```text
verified source buffers + immutable proposal + compiled artifact
        + observed Host profile/capabilities + explicit loss choices
                           │
                           ▼
             CompiledPlanIdentityV1 (metadata only)
                           │
       trusted surface presents exact signing bytes to signer
                           │
                           ▼
             SignedApprovalV1 (Ed25519 signature)
                           │
 verifier receives current identity + trusted principal/key binding
                           │
          exact identity match + strict signature verification
                           ▼
       private VerifiedApprovalV1 token for a future executor
```

`CompiledPlanIdentityV1` uses lowercase SHA-256 digests for exact plan,
compiled-artifact and source bytes. Source records are ordered by stable asset
ID and include byte length plus hash. Host profile/version and sorted capability
IDs are explicit fields; a separate observation-state field distinguishes
`declared` from `observed`, and `observed` is only a claim by the caller until a
trusted Host adapter supplies it. Accepted losses are full `(feature, path,
disposition)` records, sorted and unique; their accepted state is explicit.

`SignedApprovalV1` includes the identity, approval ID, principal ID, key ID,
timestamp and nonce. A helper returns the exact canonical signing bytes with
the fixed `storytelling.approval.v1\0` domain prefix. The helper does not sign.
The verifier receives the expected current identity and a `TrustedApproverKey`
from a trusted caller, checks principal/key IDs and full identity equality,
then verifies the signature. It returns no token for an invalid or stale
approval. The verified token has private fields, no deserializer, and is not a
serializable bearer credential.

M3-01 does not decide key authorization. A valid signature proves control of a
key only. The trusted caller must ensure the key binding is active and
authorized for the project and operation. M3-02 adds durable replay/revocation
fencing; M3-03 obtains real Host observations and executes only a verified
approval.

## Constraints

- Keep the current MIT repository license and Apache-2.0 component policy and
  `NOTICE` attribution intact.
- No private key bytes or secret material may enter serialized contracts,
  logs, fixtures, CI artifacts, or the CLI.
- The signing payload has strict schema/version checks, duplicate/unknown-field
  rejection, explicit byte/count limits, canonical ordering, and no floating
  point values.
- Sign exact canonical bytes; do not sign an object whose serialization can
  change between platforms or language runtimes.
- Use the repository's already pinned `serde_json` implementation directly in
  core for strict parsing and frozen struct-order serialization; do not add a
  second JSON codec.
- No new command is allowed to execute or mutate a Host project.
- Add only the audited cryptographic dependency needed for Ed25519 verification;
  pin it in `Cargo.lock` and include its advisory/SBOM review.

## Acceptance

- RFC 8032 known-answer vector verifies; altered payload/signature, wrong key,
  wrong principal/key binding, and each stale identity dimension reject.
- JSON parsing rejects duplicate and unknown fields and checks byte, count,
  canonical ordering, timestamp and digest limits before producing a token.
- The public API cannot construct, deserialize, serialize, or mutate the verified
  token, and no signing/private-key API is exposed by core/engine/CLI.
- Unit/property-style tests establish deterministic signing bytes and that
  changing any source, plan, artifact, Host version/capability, or accepted loss
  invalidates the signed binding.
- Full debug/release, strict Clippy, docs, provenance and three-OS same-head CI
  pass. Independent G2 security review is required before acceptance.

## Calls left to the implementation

Use the existing strict Serde contract pattern and pinned `serde_json` wire
codec, `sha2` for digests, and one locked `ed25519-dalek` release for
verification. Tests store only public keys/signatures, never private seeds. The Desktop/key registry is
not yet in this repository; the M3-01 function accepts its explicit trusted-key
binding as an argument and never reads credentials or platform key stores.
