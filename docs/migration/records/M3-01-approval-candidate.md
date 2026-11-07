<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# PR #28 Approval Identity candidate record

State: merged candidate, not accepted. Canonical M3-01/P1 remains proposed in
`docs/migration/M3-01-NEXT.md`; this candidate does not satisfy that scope.
Independent G2 review is pending. PR #28 head `2d23b21dcfbfc49c3d76087f054cc76724c23053` merged to main as `5a5702ba4c1392d763929f89da3cf7d16d5f13f4`
on 2026-11-07 at 13:47:15 UTC. Both exact-head runs passed all six jobs and the
main merge run passed all six gates. Hosted Cargo Audit loaded 1,422 advisories
and scanned 40 locked dependencies without findings. Merge is integration, not
G2 approval or product acceptance.

The initial worktree began at `867fcde6acd0dd1339122aaf4bb23003aac3942e` and was
rebased onto main after PR #25. PR #26 advanced main while PR #28 was open,
and the branch included the accepted M2-03 integration. PR #29 advanced main
again and defines the separate finite-OTIO review-binding proposal. The later
routine dependency merges are recorded in `TASKS.md` and do not change this
candidate's trust boundary.

## Scope delivered in this branch

- Added pure `storytelling-core::approval` DTOs and bounded identity construction.
- Built source identity only from existing `VerifiedBufferV1`; proposal and
  artifact digests cover their exact supplied byte slices, including empty slices;
  this does not prove compilation or same-snapshot linkage.
- Added strict compact domain-separated payload bytes and Ed25519 strict
  verification against a caller-supplied principal/key/public-key binding.
- Returned opaque non-Serde `BoundApprovalIdentityV1` and
  `VerifiedApprovalV1` tokens. No approval is persisted or executed.
- Added RFC 8032 verification vector, a synthetic detached-signature fixture
  that contains only a public key/signature, malformed/stale/bounds tests and
  compile-fail API opacity checks.

This is first-party contract code, not copied from OpenFilm. No upstream
provenance manifest is appropriate. The OpenFilm MIT license and existing
Apache-2.0 component notices remain unchanged.

## Trust boundaries and deferred work

The signature proves key control, not a human identity or authorization. The
caller must supply an expected principal/key binding from its own trusted authority;
core does not check registry provenance, active status, or authorization. `observed`
Host state is only a signed caller claim. The core has no key store, signer,
revocation/expiry registry, nonce persistence/replay check, operation journal,
CLI approval, apply operation or Host execution. Filesystem source freshness,
transactional Host operations and recovery remain separate durability and
Host-execution gates.

## Evidence

Focused tests and full local gates are recorded in
[validation/m3-01-approval-candidate.md](../../validation/m3-01-approval-candidate.md).
The cryptographic dependency and advisory status are in
[dependency-review-pr28-approval-candidate.md](../dependency-review-pr28-approval-candidate.md). The exact-head hosted CI and Cargo Audit passed before merge. Independent G2
security review remains required before any acceptance decision for this separate
candidate.
