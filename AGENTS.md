# Agent instructions

<a name="agent-execution-rules"></a>
<a name="開發代理工作規則"></a>

English · [繁體中文](AGENTS.zh-TW.md)

<a name="開始任務"></a>

## Start a task

1. Read the [project overview](README.md), [tasks](TASKS.md), [roadmap](ROADMAP.md) and [development process](docs/SDLC.md).
2. Identify the requirement, base commit and any open PR for the same work.
3. Use an isolated branch for one coherent change. Respect the scope of the user's authorization.
4. Deliver one usable result. Keep codecs, a second editor, plug-in markets and distributed workers outside the current scope.

<a name="實作與驗證"></a>

## Implement and test

- Keep core and engine pure. Do not add filesystem, Tokio, database, network, model or host calls to either crate. The CLI connects the components. Hosts manage their timelines.
- Write a failing test before you change behavior. Record the observed failure and the passing result. Report a missing toolchain as blocked verification. Preserve the assertions.
- Parse untrusted JSON strictly at each boundary. Preserve rational times, source and sample clocks, source identity, locks, user choices and review versions.
- A validator checks declarations. Actual byte verification, human approval, host compatibility and output quality require separate evidence.
- Never write source media, call a remote model or command an editor implicitly. Execution requires approval for the exact plan, host isolation, receipts and recovery from unknown results.
- Keep the MIT license and applicable third-party notices. Record origin, revision and license before you import code or tests. A language change does not remove license obligations.
- Use synthetic inputs or materials with public authorization. Keep private paths, media, GPS, tokens and signed URLs out of fixtures, logs and pull requests.
- Run formatting, Clippy, workspace tests, release tests, documentation and repository checks. Read the hosted results for the submitted commit. Distinguish failed, blocked and cancelled checks.
- Update task IDs, requirements, architecture decisions and verification records. Separate implementation, automated verification and manual acceptance. Never derive a product completion percentage from checkboxes.

<a name="既有功能與驗收範圍"></a>

## Accepted scope

| Work | Evidence to read |
| --- | --- |
| M0, M1-01, M1-02 | Preserve the [snapshot profile](docs/migration/FINITE_V1.md) and [OTIO profile](docs/migration/FINITE_OTIO_V1.md). Read the corresponding [snapshot](docs/migration/records/M1-01.md) and [OTIO](docs/migration/records/M1-02.md) comparisons before format changes. |
| M1-03 | The accepted test covers supplied bytes and one isolated 25 fps FilmCraft CPU case. See the [host record](docs/migration/records/M1-03.md). |
| M2-01 to M2-03 | The brief audit, [fixed-window planner](docs/migration/FIXED_WINDOW_PLANNER_V1.md) and [selected-domain comparison](docs/migration/PLANNER_SHADOW_V1.md) each have a defined scope. See PRs #22 and #26 and the [comparison record](docs/migration/records/M2-03.md). |
| M3-01/P1 | The user authorized the reviewed merge on 2026-11-08. PR #32 integrated the CLI and review binding. Read the [scope](docs/migration/M3-01-NEXT.md) and [signed acceptance](docs/migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md). |
| M3-01/P2 | PR #44 integrated read-only OTIO review for the selected proposal. Preserve the accepted P1 digest. See the [profile](docs/migration/PLANNER_OTIO_REVIEW_V1.md) and [record](docs/migration/records/M3-01-P2.md). |

P1 includes the runtime comparison, fixed digest and error-order tests. Issue #27 closed after the post-merge checks for PR #42. The acceptance record documents the waiver for the missing historical test failure. Do not invent evidence for that test.

PR #28 uses a separate approval contract. Independent G2 acceptance remains pending. The full solver, trusted approval, host execution and human editorial acceptance each require separate evidence.

The execution sequence is M3-02 journal, M3-03 apply and readback, then M4 feature flag. An apply timeout leaves the result unknown. Read back the host state before retry. The historical test-first scaffold does not represent a release.

<a name="合併授權"></a>

## Merge authority

For the continuation on 2026-11-07, the user authorized merging completed, verified PRs after all required checks pass on the exact head. Check completed tasks and post-merge results. Overwriting another branch, publishing, tagging, changing repository policies or permissions, and signing for a maintainer require separate authority. Real-media and host claims still need direct evidence.

<a name="移植外部元件"></a>

## Port an external component

1. Read the [transfer map](docs/migration/OPENFILM_RUST_PORT_MAP.md), [gaps](docs/migration/NATIVE_BOUNDARIES_AND_GAPS.md), [ledger](docs/migration/openfilm-port-index.json) and [porting guide](docs/migration/CODEX_PORTING_RUNBOOK.md).
2. Select one transfer or gap within an existing milestone.
3. Record the upstream SHA, files, symbols, tests, target state, license decision and intentional differences.
4. Distinguish pure Rust, native runtime, external adapters and host-owned features. Treat proposed paths and tests as planning information until evidence exists.
5. Keep upstream draft PRs separate from merged source. Preserve the existing editor boundary.
