<!-- SPDX-License-Identifier: MIT -->
# ADR 0011: bind production execution to current authority and verified readback

Date: 2026-11-13. Status: **proposed for G2; not accepted**.
Task: [M3-04 / #103](https://github.com/bynanci/storytelling/issues/103).
Baseline: `48c1ca005c0087a585f2028d59ebb45fcdab5a46`.

## Problem and existing evidence

P2 already compiles the selected proposal into an immutable owned OTIO artifact.
M3-02 already records operation identity, dispatch attempts and stale-owner fences.
The M3-03 proof already imported once into an isolated FilmCraft 0.18.0 Linux
project and resolved a deliberately lost reply by native readback. Its child had
exited before the reply was lost. It did not test an in-flight hang or safe retry.

These components do not establish current execution authority. Journal ownership
is per operation, so two different operations can still own the same project.
An expired lease cannot stop a running host. The journal accepts caller-supplied
readback enums; it does not verify the evidence. PR #28 verifies signatures using
a caller-supplied expected key and has separate, still-pending G2 acceptance.

## Proposed decision

Adopt the [Execution Contract V1](../migration/EXECUTION_CONTRACT_V1.md) and its
[test design](../validation/m3-04-execution-contract.md) as the boundary for #104.

1. Derive a new versioned execution identity from P2's private compiled result,
   registered target/profile and exact action. Keep P1/P2 digests and PR #28's
   signing format unchanged.
2. Use a trusted local authority provider, explicit current human approval, and
   a protected single-use approval reference. Portable requests, a valid
   signature and `VerifiedApprovalV1` cannot construct an execution permit.
   Signed/offline grants are outside V1; admitting them needs a new accepted
   grant contract and independent security review.
3. Add persistent project-level exclusion and quarantine to the SQLite adapter,
   retaining existing per-operation epochs and attempt numbers. Resolve target
   aliases before ownership. Lease takeover permits reconciliation, not dispatch.
4. Commit immutable execution references, approval consumption, project fence
   and `dispatching` attempt before any host mutation. Persist exact private
   artifact inputs before this commit. Invoke the owned artifact once.
5. Classify `applied` only from correlated, complete native readback against the
   expected projection. Preserve `unknown` and `partial` across restart. Neither
   an exit code nor an unchanged snapshot while a child runs permits retry.
6. #104 is single-attempt. #105 implements explicit recovery/retry under these
   rules; automatic retry and automatic Undo are excluded.
7. Initially admit only a newly created, isolated empty project copy under an
   accepted profile. Existing/shared user projects require a later accepted
   profile with enforceable revision/exclusion semantics. The existing FilmCraft
   proof is evidence for profile design, not production-profile acceptance.

## Alternatives and consequences

| Alternative | Cost / reason |
| --- | --- |
| Execute from the portable review JSON or its flags | A caller can manufacture declarations; it loses the private compiler-to-artifact boundary. Rejected. |
| Treat PR #28's signature as an execution grant | Omits operation, target revision, review-binding, host binary, policy epoch and expiry; independent G2 remains pending. Rejected. |
| Use only operation leases | Two operation IDs can mutate one target, and an expired host process may still finish. Rejected. |
| Call a successful host command `applied` | Cannot distinguish missing, partial, or wrong-target effects. Rejected. |
| Introduce an entirely new journal/state machine | Duplicates accepted recovery semantics. Extend storage under a new schema while retaining the existing state model. |

The first executor is narrower than a general editor adapter. It needs a trusted
authority integration, private evidence storage and host-specific readback.
These costs are explicit prerequisites, not guarantees supplied by a digest.

## Compatibility, rollback and acceptance

No runtime behavior, existing wire contract, signing domain or database changes
in this design PR. The proposed production store uses a new schema version;
legacy V1 databases remain readable by the old journal and are never upgraded
implicitly. Disable the future execution entry point to roll back; retain all
operation/evidence records and reconcile unknown effects before any new writer.

The [G2 checklist](../validation/m3-04-execution-contract.md#g2-acceptance) requires
a maintainer decision on the exact document/schema/fixture commit. This ADR does
not grant that acceptance, accept PR #28, authorize user-project edits, close
#103, unblock #104 or authorize publication.
