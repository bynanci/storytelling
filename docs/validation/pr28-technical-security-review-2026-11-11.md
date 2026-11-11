# PR #28 bounded technical security review — 2026-11-11

Status: read-only AI technical review completed. This is not a human security
review, formal G2 approval or candidate acceptance. The merged implementation
remains unaccepted pending an independent human G2 decision.

## Finding

Within the candidate's stated byte-verification scope, the review found no direct
mathematical bypass of Ed25519 verification. The verifier uses a domain-separated
payload, strict structural parsing, full identity comparison, canonical lowercase
hex and `verify_strict`; the tests cover known-answer vectors, mutations, wrong
keys, stale identity, encoding errors, duplicates, limits and opaque result types.
This is a bounded code-review conclusion, not a claim that all downstream use is
safe.

Three distinct guarantees must remain separate:

| Guarantee | What PR #28 establishes | What remains absent |
| --- | --- | --- |
| Mathematical signature validity | The supplied expected public key validates the exact canonical payload and bound identity | It does not establish who controls that key |
| Trusted signer authorization | Nothing beyond the caller-provided key binding | Governed trust root, principal/role policy, key lifecycle, active/revoked status and expiry |
| Review bound to the actual compiled artifact | Hashes the exact bytes supplied to the identity builder and binds declared source rows | It does not compile from the same immutable snapshot, verify artifact origin or prove proposal/artifact/source correspondence; empty proposal/artifact/source lists are permitted |

The nonce is a payload field, not a replay defense. PR #28 has no durable
consumption/replay store, expiry policy or revocation check. The accepted M3-01/P1
review identity and M3-01/P2 planner-to-OTIO compiler bridge have separate
contracts and still do not grant signer authority or host execution. No consumer
may treat `VerifiedApprovalV1` as an execution permit until those policies exist.

`accepted=true` on a loss row can coexist with an unsupported disposition in this
candidate. That may be valid as signed metadata; an executor must still reject
unsupported semantics. Additional strict-negative vectors for non-canonical
scalar S and invalid/low-order points would make `verify_strict`'s rejected
encodings explicit in this candidate's tests.

## Exact historical evidence

GitHub reports PR #28 merged at head
`2d23b21dcfbfc49c3d76087f054cc76724c23053`, merge
`5a5702ba4c1392d763929f89da3cf7d16d5f13f4`. Exact-head runs
[42378356866](https://github.com/bynanci/storytelling/actions/runs/42378356866)
and [42378352463](https://github.com/bynanci/storytelling/actions/runs/42378352463),
plus merge run
[42378550750](https://github.com/bynanci/storytelling/actions/runs/42378550750),
passed Quality, Required, Linux, Windows, macOS and Dependency Audit. The
checksum-pinned Cargo Audit 0.22.1 scanned 40 locked packages against 1,422
advisories with no findings at that run. This is a dated snapshot, not a guarantee
about later advisories or the current main lockfile.

Issue #27 is closed for its bounded M3-01/P1 RB01–RB07 scope. Its scoped
acceptance does not accept PR #28. The PR review list has no formal G2 security
approval; a general automated code-review result is not that approval.

## Human G2 decision still required

An independent security reviewer must inspect the exact merged candidate and
record a decision tied to its SHA. The review should disposition the strict
negative-vector gap; confirm the trust-key and replay limitations are not
represented as guarantees; confirm no execution path treats the opaque verifier
result as signer authorization; and verify that source/artifact/compiler claims
remain bounded to supplied bytes. The decision may accept only that limited
verifier scope, with trust registry, replay protection and compiler-derived
artifact origin explicitly reserved for later tasks. Record the reviewer,
reviewed SHA, findings and accepted boundaries. No such human decision is present
in this record.
