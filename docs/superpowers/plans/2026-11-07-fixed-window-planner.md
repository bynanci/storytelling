# Fixed-window Planner Implementation Plan

> For agentic workers: use executing-plans for native implementation with independent task/whole-branch reviews. Checkboxes record actual completion only.

**Goal:** Produce one deterministic declared StoryPlan from an audited brief and fixed candidate windows.
**Architecture:** core strict candidate/output types; pure engine admission, integer clock and stable selection/order; CLI explicit input/stdout.
**Tech Stack:** Existing Rust1.85, serde/serde_json/sha2; no new dependency/crate. Python/Node are test-only oracles.
**Spec:** [frozen profile](../../migration/FIXED_WINDOW_PLANNER_V1.md), [design](../specs/2026-11-07-fixed-window-planner-design.md).

## Global constraints

Input1,048,576 bytes, candidates<=1024, max_shots1..1024, nested plan<=1MiB, report<=4MiB.
All clock/rational fields are exact bounded u64; no float or silent quantization.
Original MIT unchanged; Apache source-informed code/tests/oracle has per-file origin/notices.
Core/engine remain I/O-free; preserve all existing command/schema contracts.
User standing authorization covers completed verified PR merges, not release/admin/human approval.

## Review focus

Required choices exceed soft target: retain them or report hard-budget failure.
A too-long optional cannot hide a later fitting asset; score i32MIN cannot wrap.
Arithmetic reaches u64MAX: compare remaining budget before addition, never wrap.
Future/duplicate/nested fields and missing required candidate cannot create a clean partial plan.
Source comparisons classify fixed-window/ranking/ID changes rather than claim full solver parity.

## P1 contract/source admission

Files: migration/FIXED_WINDOW_PLANNER_V1.md; this plan/design; records/M2-02.md;
source oracle tests/oracles/fixed-window-planner-v1; provenance/work index.

- [x] Fetch current target/source identity, read actual source symbols/tests and M2-01/M0 APIs.
- [x] Independent G2 and clock review; freeze errors, optional target, skip priority and CLI option-input rules.
- [x] Freeze actual synthetic source cases/runner/hash chain and scoped expected differences.

## P2 pure planner

Create core/src/planner.rs (PlanningCandidateV1, PlannedStoryV1, PlanningErrorV1,
SelectionDecisionV1, SkippedCandidateV1, TargetStatusV1, SkippedReasonV1).
Create engine/src/planner.rs and planner/clock.rs; export plan_story(bytes:&[u8])
->Result<PlannedStoryV1,PlanningErrorV1>. Tests: engine/tests/planner.rs.

- [x] Add compiling not-implemented stub and regressions; observe functional RED.
- [x] Implement strict envelope and real brief audit; require candidates without dropping hard conflicts.
- [x] Implement bounded common clock, required-first selection, ranked fitting optional scan and stable Kahn order.
- [x] Build exact immutable source windows, deterministic shot IDs/placements, reason paths/skips, real M0 validation and output bounds.
- [x] Selected tests GREEN, then debug/release workspace and independent review.

Pinned test expectations: manual b,a emits b then a; hard include/order never disappears;
required two1s windows with target1/max3 succeeds2s; max1 fails; required u64MAX-1
plus1 succeeds; adding required1 to u64MAX fails; optional u64MAX after required1
skips without adding;1/10+1/5 placements are0,1/10,total3/10 at clock10.
Coprime denominators4294967291/4294967311 reject LCM; u64MAX duration scaled
by clock2 rejects at its duration path. Empty output rejects explicitly.

## P3 CLI, docs and delivery

Modify only existing CLI dispatcher, create cli/tests/planner.rs and planner-v1
synthetic fixtures/README. CLI plan-story FILE|- emits frozen envelope and exits0/1/2/3.

- [x] Observe real CLI RED, wire minimal input composition root, then GREEN.
- [x] Update M2-P01–P06 tests/requirements/work index, provenance and source comparison record.
- [x] Improve SDLC/contracts/source-status docs and runnable CLI guide; link from both READMEs.
- [x] Run fmt, strict clippy, debug/release, Rustdoc, repo guards,6/8/10 old comparisons and new actual source replay.
- [x] Independent whole-branch code and artifact/guide reviews; exact RED/source/local-check evidence recorded.
- [x] Published PR #22; read all 10 successful exact-head hosted results.
- [x] Same-head three-platform CI passed; PR #22 merged under user authorization, #20 checked/closed and accepted lifecycle reconciled.

Rollback removes new command/profile; no source or project was mutated. M2-03 full
planner shadow, multi-beat/coverage/locks/graph, production host execution and human
editorial acceptance remain separate.
