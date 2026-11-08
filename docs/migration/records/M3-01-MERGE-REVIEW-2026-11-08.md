<!-- SPDX-License-Identifier: MIT -->
# M3-01/P1 refreshed merge review — 2026-11-08

The user stated “審查完畢 merge” and explicitly requested merging the two remaining
PRs #31/#32. This is recorded as maintainer-reported review completion and merge
authority in the task conversation. GitHub review lists and inline threads were
empty at inspection. No independent reviewer, GitHub approval, maintainer
signature or cryptographic editorial grant is fabricated from that instruction.

## Technical result

No blocking correctness defect was found within the existing declared-input scope.
The review inspected core review DTOs; engine builder, record/context validation,
digest framing and stale-field comparison; CLI read limits/argument handling;
10 engine integration tests, three real CLI tests and four opacity doctests.

- prepare_review imports and compiles the same immutable supplied composition
  slice; the private result owns its exact serialized OTIO bytes. Callers cannot
  inject a precompiled artifact or reconstruct the opaque type through Serde.
- SHA-256 input framing separates identity/report/plan/context/field domains and
  prefixes payload length. Record digests are unkeyed and forgeable by a caller;
  they establish consistency only. Signature verification is a separate API, and
  a valid signature still requires an external trust/authorization policy.
- Context/record DTOs reject unknown/duplicate fields. Size/depth/identifier/
  capability bounds and sorted unique declarations apply. All authority/evidence
  flags remain false; a forged true claim is rejected.
- Freshness recompiles current composition bytes and compares current declared
  fields. No source URI, host binary or editor state is observed. There is no
  expiry, revocation, replay/consume-once store, signed review, host executor or
  planner-to-OTIO bridge. Error paths are fixed known fields/indices.

## Integration and remaining evidence

Refresh target main from 2cc746afda1f92a86b0d265ffb374ca438a9323a. PR #31's TASKS
conflict is resolved using the latest accepted Planner records and current M3
candidate information. PR #32's TASKS/register conflicts retain both the November 7
candidate observation and later accepted delivery observations. No pre-existing
Rust M3 implementation/test blob is changed by the conflict resolution.

The source comparison remains static and pinned; no paired OpenFilm TypeScript
review runtime is claimed. The user's review-complete/merge instruction resolves
merge authority, but this record does not invent the missing runtime oracle or
an independent review artifact. Issue #27 is reconciled only for observed scoped
work; full acceptance/source-comparison evidence remains separately traceable.

Local and hosted refreshed results and actual merge identities are appended after
observation. Existing historical head/run/source records remain intact.


Local refreshed integration validation passed format, warnings-denied clippy,
217 debug tests, 217 release tests, warnings-denied rustdoc, repository/provenance
guard and whitespace checks. PR #31 refreshed head 937cf5a passed all 12 jobs
in runs 42449119799/42449115934 and merged as baa97903e3d3c02e2eb2747140d62f1ddca3c60c
at 2026-11-08T05:04:52Z. Its main run 42449339199 succeeded. PR #32 final head
and frozen/live/hosted results are recorded only after observation.


## Observed integration receipts — 2026-11-08

PR #31 refreshed head `937cf5a1db61d1bccb401241c8d3001bc02e49f5` passed 12 jobs
in runs 42449119799/42449115934; merge
`baa97903e3d3c02e2eb2747140d62f1ddca3c60c` passed all six jobs in main run
42449339199. PR #32 head `58b536d0bc597b43c7f5a1eb87eef922cfadd062` passed
12 jobs in runs 42449548994/42449545432; merge
`3d121398e2c14c4a43d101a5b9a55a4616a81174` at 05:10:16 UTC passed all six
jobs in main run 42449705252. Both head/merge file trees match.

Refreshed debug/release each passed 217 tests. All 36 existing frozen importer/
OTIO/Brief/Planner cases, six host-checker regressions, 40 shadow regressions and
a fresh 64-pair live Planner run passed (56 agreement, four intentional deltas,
four unsupported, zero unclassified). Both new review CLI commands passed with
no external tools on PATH; source/host/approval/execution flags stayed false.
These existing source lanes are not a paired M3 review-runtime oracle.

The merge instruction and reported review completion are attributed to the user
in the task conversation; no GitHub approval/independent reviewer identity or
signature is invented. Issue #27 now checks observed items 2/3/6 and retains
1/4/5 for complete source-oracle/profile review evidence. It remains open.
This integrates the bounded candidate, not trusted approval or host execution.
