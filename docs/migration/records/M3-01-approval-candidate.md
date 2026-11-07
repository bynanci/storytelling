<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# PR #28 Approval Identity candidate record

State: implemented candidate, not accepted. Canonical M3-01/P1 remains proposed
in `docs/migration/M3-01-NEXT.md`; this candidate does not satisfy that scope.
Independent G2 review is pending. After dependency PRs #8, #21, #10 and #9 merged,
main advanced to `fefb038ec1699727da2c0f436a097bbdedcfd08a`. Synchronized head
`9670be947535a3e15b70280fd7500a83b373d55f` passed both hosted same-head runs, including all five code/quality gates
and Dependency Audit; the audit scanned the 40-package lockfile with no findings.
The current record updates those results and therefore requires a new exact-head
CI run before a merge. A successful merge will not itself grant G2 or acceptance.

The initial worktree began at `867fcde6acd0dd1339122aaf4bb23003aac3942e` and was
rebased onto main after PR #25. PR #26 advanced main while Draft PR #28 was open,
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
[dependency-review-pr28-approval-candidate.md](../dependency-review-pr28-approval-candidate.md). Independent G2
security review, current exact-head hosted CI, and the new hosted Cargo Audit
job are still required before any acceptance decision for this separate candidate.
