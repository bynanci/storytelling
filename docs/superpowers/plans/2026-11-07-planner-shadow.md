# Live Planner Shadow Implementation Plan

> For agentic workers: use task-focused implementation with independent whole-branch review. Checkboxes record observed completion.

**Goal:** Run actual pinned TS compose and native plan-story on bounded synthetic shared domains, with fail-closed evidence and a completed PR register.
**Architecture:** Test-only Python case/policy/comparison/process modules orchestrate a source-only Node producer and the actual Rust CLI; production APIs remain unchanged.
**Tech Stack:** Existing Rust1.85, Python3.12 stdlib, official Node24.22.1, existing tsx4.23.15/esbuild0.28.2 integrity lock. No new production dependency/crate.
**Spec:** [profile](../../migration/PLANNER_SHADOW_V1.md), [design](../specs/2026-11-07-planner-shadow-design.md).

## Global Constraints

Pinned source7add4762, target base867fcde; source checkout remains clean before/after even on failure.
Default64cases, max144; batch1MiB, each stdout4MiB, stderr64KiB, child30s/global180s.
Linuxx64 shadow tooling only; existing3OS Rust gates and6/8/10/12frozen comparisons remain.
Original MIT/notices unchanged; source-informed test code/data has per-file Apache provenance.
Standing user authority covers completed verified PR merge/task completion, not release/admin/signature.

## Review Focus

Fractional optional budgets are outside agreement; precondition mutation must fail.
Intentional/unsupported labels cannot excuse arbitrary errors or altered windows/IDs/authority.
Missing/duplicate/reordered source results and both-errors generic matches must fail.
No unbounded child capture, lingering esbuild/tsx descendants, or fresh-success report after failed execution.
Current PR metadata must distinguish merged docs/implementation, superseded closed-unmerged and pending dependency work.

## P1 freeze domains and source/tool closure

Files: migration/PLANNER_SHADOW_V1.md; design/plan; records/M2-03.md and issue24.
- [x] Read actual compose/scoring/ordering sources and existing fixed-window producer/comparator.
- [x] Execute8paired research cases; independently confirm fractional optional counterexample.
- [x] Independent G2 domain/clock/error/process-budget review, then freeze exact schemas and error pairs.
- [x] Verify official Node archive SHA and npm ci --ignore-scripts temp closure by actual12case existing source replay.

## P2 live source/native shadow

Create tests/oracles/planner-shadow-v1/{cases.py,compare.py,process.py,run.py,producer.mts,test_shadow.py,README.md} (split helpers only where focused).
Interfaces: build_cases(seed:int,per_domain:int)->list[dict]; classify(case,source,native)->dict according to frozen report row; bounded process helper receives absolute argv/stdin/deadline/caps and returns actual exit/stdout/stderr receipt or typed execution failure. These are test-internal APIs, not production SDK.

- [x] Write compilable/importable stubs and full-baseline semantic/process regression tests; observe functional RED before implementation.
- [x] Generate48default seeded agreement cases plus8targeted/4delta/4unsupported probes with validated exact-domain preconditions.
- [x] Implement actual pinned source producer, strict schema/result identity, exact source-error/native-code pairs, source/native semantic projection and literal accepted differences.
- [x] Implement bounded process groups/global deadline, source pre/post provenance, actual tool hashes/versions, native validator consumer and complete external raw artifacts/failed evidence.
- [x] Run tests GREEN and actual64case live run; rerun deterministic inputs with another seed and verify semantic rejection mutations.

## P3 CI, PR organization and verification

Root owns CI/docs/records/index/provenance. PR inventory agent owns docs/delivery/PR_REGISTER.md +pr-register.json and authorized completed-PR lifecycle prepends only.

- [x] Add checksum-pinned Node archive, temp source fetch and temp integrity-locked npm tools/live shadow to Linux quality; no global/upstream installs.
- [x] Complete actual merged/superseded/pending PR register and preserve historical bodies/identities; don't merge dependency updates without their own review.
- [x] Add M2-S01–S06 traceability, source-informed inventory, comparison record, usage/status/docs links and scoped task states.
- [x] Run167Rust tests debug/release, fmt/clippy/docs/guard, old36comparisons, new shadow tests/live run and required fresh independent code/artifact review.
- [ ] Publish Draft PR with exact source/base/head/tree/RED/GREEN/tool/dependency/disposition evidence; read same-head3OS/quality/Required gates.
- [ ] Merge verified slice under standing authority; check/close24, refresh completed PR register and accepted lifecycle docs.

Rollback removes new test-only gate/profile/register additions; no production code or project is mutated. Full solver parity and all source/host/editorial/release/admin gates remain separate.
