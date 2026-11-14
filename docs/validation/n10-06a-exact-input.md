<!-- SPDX-License-Identifier: MIT -->
# N10-06A · Exact range input implementation evidence

Task N10-06A / N-R10-06A, [#129](https://github.com/bynanci/storytelling/issues/129), [PR #130](https://github.com/bynanci/storytelling/pull/130).
Base: `52dc6c315f7b3d20693f3c51e052509997ab791b`.
The requester accepted the draft-only G2 candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` and authorized A RED/GREEN; see the [decision record](n10-06a-g2-decision.md).

The [complete changed-file list](n10-06a/file-list.txt) accompanies the PR diff.

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

Implementation source commit: `2f906181d075d56a19d18fd29a0ec1a645ba0a78`. [RED/GREEN receipt index](n10-06a/red-green-receipts.json) links preserved test-output excerpts; the CLI RED commit is reproducible independently. Supplemental math-stub RED and diagnostic-order RED were transient working-tree observations and are labeled accordingly. CLI GREEN has 12 normal-entry tests; math GREEN has 8 tests. The unchanged `video_coverage` suite passed 7/7 in both default and opt-in workspace runs. Default builds additionally test that the draft command is absent.

[Workspace checks](n10-06a/workspace-checks.json): fmt, Clippy `--workspace --all-targets --all-features -- -D warnings`, workspace debug/release with default and all features, rustdoc `--all-features` with warnings denied, repository and diff checks all exited 0. Existing Planner, Narrative, N-11, OTIO, approval/review and journal tests run in these workspace suites. The real FilmCraft manual test remains ignored and supplies no host observation. An initial baseline fmt/clippy launcher needed the installed toolchain location; the corrected runs passed and this is not counted as behavior RED.

[Compatibility byte checks](n10-06a/compatibility-bytes.json) verify exact equality with base for both V1 schemas, public time/coverage DTOs, accepted V1 implementation, existing seven CLI tests and Cargo.lock. The remaining core sources also have no diff. The four crate dependency sets remain unchanged.

[Three actual CLI previews and replay receipts](n10-06a/fixture-replay.json) match their expected V1 documents, retain original input bytes and agree with a subsequent `render-video-coverage` invocation. The seconds and 30 fps examples yield identical V1 documents; the NTSC example yields `1001/200` at frame boundary 150. All three preserve S03 unresolved and declaration-only blocking. No media was opened.

Implementation head `59516f24f4d829a3d695c115d2f94e208a78cdea` passed all six hosted jobs in [CI run #483](https://github.com/bynanci/storytelling/actions/runs/43538270184). The CI matrix runs the opt-in regression suite on Windows/Linux/macOS in addition to the default suite, and release/Clippy/rustdoc exercise the opt-in path. The [later continuation](n10-06a-g2-decision.md#later-continuation-verified-engineering-merge-authorization) authorizes merging completed, verified engineering work after review and required CI on the current head. Any later head change requires fresh checks; independent author acceptance remains unobserved.

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
| Engineering PR merge | Authorized by the later continuation after current-head checks and review |
| G5 publication | Not authorized |

B adds actionable multi-field advisory UX after A is reviewed; V1 stays authoritative. C requires a trusted read-only adapter contract/threat model, explicit file-selection/privacy scope and reliable byte/metadata binding. D stays a dry-run checklist with five separate evidence states. If any binding is ambiguous, retain unknown/not_verified; never infer host support, self-approval or safe retry from declaration readiness. Removing the opt-in feature/command is the rollback; existing inputs need no migration.
