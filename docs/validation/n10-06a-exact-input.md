<!-- SPDX-License-Identifier: MIT -->
# N10-06A · Exact range input implementation evidence

Task N10-06A / N-R10-06A, [#129](https://github.com/bynanci/storytelling/issues/129), [Draft PR #130](https://github.com/bynanci/storytelling/pull/130).
Base: `52dc6c315f7b3d20693f3c51e052509997ab791b`.
The requester accepted the draft-only G2 candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` and authorized A RED/GREEN; see the [decision record](n10-06a-g2-decision.md).

## Implemented scope

The disabled-by-default `draft-video-input` feature enables a separate pure engine module and explicit CLI `draft-video-coverage-input <FILE|-> [--emit-v1]`. Exact seconds and declared constant-rate frame boundaries become reduced nonnegative u64 rational strings. Bounded decimal factor cancellation precedes checked u128 arithmetic, GCD and final u64 checks; no floating point, rounding or clamping occurs. Existing JSON preflight checks syntax, duplicates and depth before `Value` parsing.

The final call is the unchanged `render_narrative_video_coverage`. Successful output contains only a valid existing V1 input and its V1 result, plus original time objects and fixed/indexed JSONPaths. `--emit-v1` yields the validated input for the unchanged CLI. Unresolved rows and selection order remain visible; true readiness is labeled `declaration_ready_for_planning` and `scope: declarations_only`.

No existing V1 schema, DTO, 21 diagnostics, implementation or seven CLI tests changed. No dependency or lockfile change. The new candidate API remains feature-gated; stable SDK adoption follows #107. B/C/D and #103/PR #117 retain independent gates.

## Observed RED and GREEN

- CLI RED at test-first commit `231b9cacb3ee3f9f283dca0cc6ff1ebda958a1fe`: normal entry-point command absent, cargo exit 101, 0 passed / 11 failed. Replayed in a clean detached checkout of that exact commit.
- Math RED: 0/8 tests passed against explicit not-implemented stubs; exit 101. Then exact arithmetic GREEN: 8/8 passed.
- During review, a compound-error test made start-before-end ordering explicit: an invalid start plus a wrong-typed end first reported the end type. Actual RED was retained before moving start conversion ahead of end lookup. Subsequent CLI GREEN includes that preserved test.

These are synthetic machine tests. The test-first commit and raw-output excerpts are linked in the final receipts below. No historical failure or participant result was reconstructed.

## Verification receipts

Final workspace/CLI receipts and final-head hosted CI are being collected. The PR remains Draft and must not merge automatically. New implementation/manual acceptance is not inferred from the earlier baseline CI.

## Evidence states and follow-ups

| Evidence | State |
| --- | --- |
| Draft G2 grammar/API/CLI acceptance | Requester accepted exact candidate f148b6d; recorded separately |
| Machine implementation and replay | Implemented; final receipts tracked below |
| Independent author entering seconds/frames | Not observed; no completion/time/usability claim |
| Real media identity, bytes or metadata | Not verified; C requires independent G2/privacy scope |
| Author content confirmation | Not observed |
| Host capabilities, target revision/readback | Not verified; #104–#106 remain pending |
| Current execution authority | Not supplied by this helper; #103/PR #117 remains independent |
| G5 release / merge | Not authorized by this task; keep Draft |

B adds actionable multi-field advisory UX after A is reviewed; V1 stays authoritative. C requires a trusted read-only adapter contract/threat model, explicit file-selection/privacy scope and reliable byte/metadata binding. D stays a dry-run checklist with five separate evidence states. If any binding is ambiguous, retain unknown/not_verified; never infer host support, self-approval or safe retry from declaration readiness. Removing the opt-in feature/command is the rollback; existing inputs need no migration.
