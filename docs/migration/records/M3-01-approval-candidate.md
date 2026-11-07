<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# PR #28 Approval Identity candidate record

State: implemented candidate, not accepted. Canonical M3-01/P1 remains proposed
in `docs/migration/M3-01-NEXT.md`; this candidate does not satisfy that scope.
Independent G2 review is pending. The branch is being reconciled to main
`1d8d0e3652aa2afe77375550876639b726928eba` after PR #29 clarified the distinct
canonical task and accepted M2-03.

The initial worktree began at `867fcde6acd0dd1339122aaf4bb23003aac3942e` and was
rebased onto main after PR #25. PR #26 advanced main while Draft PR #28 was open,
and the branch included the accepted M2-03 integration. PR #29 advanced main
again and now defines the separate finite-OTIO review-binding proposal.

## Scope delivered in this branch

- Added pure `storytelling-core::approval` DTOs and bounded identity construction.
- Built source identity only from existing `VerifiedBufferV1`; proposal and
  artifact digests cover their exact supplied byte slices.
- Added strict compact domain-separated payload bytes and Ed25519 strict
  verification against a caller-supplied principal/key/public-key binding.
- Returned opaque non-Serde `VerifiedCompiledPlanIdentityV1` and
  `VerifiedApprovalV1` tokens. No approval is persisted or executed.
- Added RFC 8032 verification vector, a synthetic detached-signature fixture
  that contains only a public key/signature, malformed/stale/bounds tests and
  compile-fail API opacity checks.

This is first-party contract code, not copied from OpenFilm. No upstream
provenance manifest is appropriate. The OpenFilm MIT license and existing
Apache-2.0 component notices remain unchanged.

## Trust boundaries and deferred work

The signature proves key control, not a human identity or authorization. The
caller must provide an actually trusted active principal/key binding. `observed`
Host state is only a signed caller claim. The core has no key store, signer,
revocation/expiry registry, nonce persistence/replay check, operation journal,
CLI approval, apply operation or Host execution. Filesystem source freshness,
transactional Host operations and recovery remain separate durability and
Host-execution gates.

## Evidence

Focused tests and full local gates are recorded in
[validation/m3-01-approval-candidate.md](../../validation/m3-01-approval-candidate.md).
The cryptographic dependency and advisory status are in
[dependency-review-pr28-approval-candidate.md](../dependency-review-pr28-approval-candidate.md). Independent G2
security review and current exact-head hosted CI are still required before any
acceptance decision for this separate candidate.
