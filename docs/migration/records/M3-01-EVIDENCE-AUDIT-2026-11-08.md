<!-- SPDX-License-Identifier: MIT -->
# M3-01/P1 evidence audit — 2026-11-08

Task: [Issue #27](https://github.com/bynanci/storytelling/issues/27), M3-RB01–RB07.
Reviewer: Codex, this separate evidence-audit session, not the original PR #32
implementation session. This is an AI technical review, not human approval,
a GitHub approving review or a security certification. The reviewer authored the
corrections in this document; those corrections have not received a second review.

Reviewed main: `72e619c1ca25d363a6c62ee38164be6a4bec421f`, tree
`1ce3cfbdc8a61e628e72fcf4134a562ced41fee8`. GitHub showed PRs #31/#32/#38 merged
and no open PR in the initial repository observation. Existing Codex task state
was supplied by the delegation (completed); this environment exposes no task
read/send API to independently confirm it. No replacement Codex task was created.

## Source inspection and oracle disposition

Pinned source: OpenFilm `7add4762fe8174d76f43b863375ea39b73121e76`.
Read actual `nle-handoff.ts` snapshot/review/publish, `scene-script-review.ts`
savedScriptProjectIdentity, `story-plan.ts` digest and stale guard, and the
story-plan and silence-review negative tests. Compared them with the Rust
review DTOs, builder/freshness implementation, CLI and existing tests.
All nine source entries in `provenance-m3-01-p1.json` were resolved with
`git rev-parse HEAD:<path>` in the pinned source checkout. Eight matched. The
story-plan test entry was transcribed incorrectly; its actual blob is
`7ac3357de7aa9885f893aece8a9ec3effc2bb5c5`. Correct the manifest and comparison
matrix; the source commit and test content do not change. The local repository
guard only checks source SHA syntax, not remote blob existence, so a prior green
guard did not establish this provenance fact.

The inspected upstream behavior supports these bounded correspondences:

- Handoff review binds project/revision/plan and publish rechecks current state.
  Rust reimports/recompiles supplied bytes; it deliberately does not read that
  live saved project, media sources or host state.
- Story-plan acceptance rejects changed story/graph revision; silence review
  rejects modified/cross-project envelopes and `approved:true`. Rust stale-input
  and strict false-claim tests cover related policy, not the same input domain.
- savedScriptProjectIdentity strips incidental updatedAt after migration, whereas
  Rust binds exact composition bytes (including trailing whitespace). Upstream
  local hash encodings differ; typed framed SHA-256 is an intentional delta.

**Disposition:** retain the matrix as static source-informed design evidence.
It is insufficient to claim a paired runtime oracle, digest parity, live-source
freshness or full review-service migration. Do not check Issue #27's oracle item
solely from this audit. A useful runtime follow-up must name the common admitted
policy domain, run actual pinned upstream services and native CLI on corresponding
synthetic cases, record unchanged/changed-project/changed-content/forged-claim
outcomes, and classify encoding and input-domain differences. It must not rewrite
the upstream algorithm in a test and call that upstream execution. Alternatively,
a future explicit scoped acceptance may waive runtime comparison while retaining
these limitations; this audit makes no such acceptance decision.

## G2 profile assessment

No blocking production correctness defect was found in the inspected bounded
supplied-declaration path. The builder imports and compiles the same slice, owns
serialized OTIO privately, and derives context loss choices once. Required
capabilities must be a subset of declared capabilities. Strict DTO parsing,
limits, domain/length framing and false authority flags are consistent with the
narrow identity purpose. Matching forgeable portable JSON is consistency, not
an approval or verified origin. PR #28 is a separate unreviewed trust candidate.

Full profile freeze remains open for these concrete evidence gaps:

1. RB01's cross-platform deterministic encoding is currently tested by comparing
   two executions in one process. The 10-test review suite contains no fixed
   expected binding/artifact/plan/report digests shared across OS lanes. Add a
   reviewed golden vector with exact source/context bytes and field ordering;
   three green suites alone do not prove identical cross-platform digests.
2. The original required RED matrix is broader than its checked evidence. The
   existing suite covers order, whitespace, contexts, limits and forged claims,
   but the historical record documents unresolved imports, not assertions.
3. The proposal's error-precedence prose needs an exact surface-specific freeze:
   CLI reads and size-checks each argument before opening the next; direct
   prepare_review validates its typed context before importing composition;
   check_review checks all byte sizes, then record, then context, then recompiles.
   These are observable distinctions. Add multi-invalid-input cases before
   describing a universal precedence contract.

These are acceptance/evidence findings, not claims of an observed host fault or
unsafe edit. Keep RB01–RB07 provisional until the remaining criteria are disposed.
No schema, runtime code, dependency, release or host execution is changed here.

## Historical RED correction and present-day experiments

The original record says the initial command could not resolve the review module
and functions. The test executable therefore never ran. Preserve that compile
failure as history, but remove the label functional RED. Git history first adds
the review integration tests alongside implementation in `84f6d9b`; absence of a
separate commit does not prove tests were not tried locally, but it supplies no
missing assertion log. The original historical functional-RED criterion is
unproven, not retroactively completed by this audit.

On the reviewed baseline, Rust 1.85.0 with Cargo.lock ran all 10 review-binding
integration tests successfully. Then two temporary local mutations were applied
one at a time, with the original source restored in a finally block:

| Mutation | Exact existing test | Observed result |
| --- | --- | --- |
| Replace `current: stale_fields.is_empty()` with `current: true` | `changed_composition_and_each_declared_context_field_are_reported_stale` | Compilation succeeded; test failed at `assert!(!report.current)`, exit 101, 0 passed / 1 failed |
| Make the required-capability `.all` predicate always true | `context_must_declare_compiler_required_capabilities_and_record_digest_is_checked` | Compilation succeeded; `unwrap_err()` received Ok, exit 101, 0 passed / 1 failed |

Command for each: `cargo test -p storytelling-engine --test review_binding --locked <test> -- --exact`.
After restoring the original implementation, `cargo test -p storytelling-engine
--test review_binding --locked` passed all 10 again. No injected fault is committed.
This is current regression sensitivity for two faults, not pre-implementation
TDD evidence, exhaustive mutation coverage, upstream execution or host acceptance.

## Delivery and remaining gates

This change corrects documentation/provenance and records the independent
inspection. Local verification results and PR/head CI are reported in the PR;
this record does not preclaim their success. Previous head/main run IDs remain
historical evidence. Issue #27 stays open: item 2 needs its historical functional
RED qualification, items 1/4/5 still need the profile/oracle/final-review evidence.
The CLI, implementation and merged PR delivery should not be redone.

## Observed local verification and publication blocker

On 2026-11-08, after restoring the temporary mutations: formatting check,
strict clippy, debug and release workspace tests (217 passed each, two ignored
informational benchmarks), warnings-denied rustdoc, repository/provenance/link
guard and git diff whitespace checks passed. The unchanged Rust suite was run
with the repository-pinned Rust 1.85.0 and locked dependencies. No new hosted CI,
paired M3 runtime oracle or host/media test was run. Existing frozen/live lanes
were not rerun locally for this documentation-only correction.

The initial attempt to publish the six evidence files through GitHub create_tree
was denied by automatic approval review. A retry was also denied because the
external write to the public repository still required explicit authorization.
The user later explicitly authorized publishing these six files and opening a
Draft PR. That authorization resolved the publication block: the files were
published on PR #39 at head `d96a2a3fab35389214be20d5884006a928619e3a` and its
hosted checks passed. The earlier denial is retained as historical evidence; it
does not block this scoped documentation change. No Issue #27 checklist update
was made by this PR, and the remaining source-oracle/profile and independent
review evidence stays open.

## Follow-up after this audit

After PR #39's evidence correction, the pinned OpenFilm source runtime was
executed with the synthetic finite composition. `NleHandoffService.review`
returned the same revision for unchanged inputs and a timestamp-only `updatedAt`
change; changing project ID or a clip occurrence ID changed the revision and
made publishing the previous review fail as stale. An extra `approved: true`
request field was rejected. Corresponding native CLI checks reported current,
stale project/content and rejected-claim outcomes. The exact source command,
fixture hash and results are in [the follow-up runtime record](M3-01-RUNTIME-ORACLE-2026-11-08.md).

The Rust review suite also gained fixed digest assertions and multi-invalid-input
API/CLI precedence checks. These later results resolve the earlier audit's
missing small runtime execution and evidence gaps for the tested cases. They do
not retroactively establish historical functional RED, cross-runtime digest
parity, full profile acceptance, or independent G2 review. Issue #27 remains
open for the unchecked acceptance evidence.
