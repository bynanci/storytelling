# Storytelling roadmap

The roadmap is outcome-gated, not a promise of release dates. Package version
`0.1.0-dev.0` is a development identifier; no package or release is published.
See [TASKS](TASKS.md), [SDLC](docs/SDLC.md) and [architecture](docs/ARCHITECTURE.md).

## Product objective

Turn 30–50 selected photos, clips and narration into a reviewed, editable
60–90 second story rough cut, using a host editor for finishing. These are
reference-workflow targets, not current supported media claims.

Storytelling owns story intent, evidence references, proposals and receipts.
OpenFilm provides a first consumer and existing analysis. FilmCraft is the first
planned host, followed by a real second-editor compatibility trial.

## Milestones

| Milestone | Current state | User-visible outcome | Exit gate |
| --- | --- | --- | --- |
| M0 foundation | Accepted and merged on main | Validate a declared StoryPlan through a usable Rust CLI | Strict contract, source bounds, immutable input, same-head 3-OS CI, maintainer acceptance |
| M1 / v0.1 handoff | Finite M1-01/M1-02 and bounded M1-03 Linux 25 fps proof accepted; broader acceptance open | Existing story choices become an editable FilmCraft rough cut | Tested picture-only profile passed import/CPU preview/readback/save/reopen; audio, other profiles and human acceptance remain open |
| M2 / v0.2 planning | M2-01 audit accepted; M2-02 fixed-window planner accepted; full solver open | Constraints and evidence produce an explainable shot proposal | Hard constraints satisfied or explicit conflicts; frozen-input reproducibility |
| M3 / v0.3 reviewed execution | Planned | Accepted plans execute without silently damaging prior work | Approval binding, owner fencing, durable journal, partial/unknown recovery |
| M4 / v0.4 consumers | Planned | OpenFilm and an external TypeScript consumer use one SDK | Single authority, full Review/Accept workflow, optional thin MCP |
| M5 / beta | Planned | Independent users and a second host finish a real story | Measured user outcomes, target-host matrix, distribution evidence |
| v1.0 | Not scheduled | Stable public contract with a supported upgrade path | Schema/SDK migrations, security/recovery and compatibility gates accepted |

## Dependency path

```text
M0 -> M1-01 -> M1-02 -> M1-03
          \-> M2-01 -> M2-02 -> M2-03
M1-03 + M2-02 -> M3-01 -> M3-02 -> M3-03
M2-03 + M3-03 -> M4 -> M5 -> v1.0
```

Requirement drafting and fixtures can overlap; host execution cannot skip the
finite interchange proof. No deadlines or aggregate product completion percentage
are inferred from code or test counts.

## Investment decisions

At M1: if source ranges/basic clips cannot survive handoff, narrow the compatibility
profile before adding hosts or a general SDK.
At M2: if Rust cannot match the selected invariants, retain the TS planner and
use Rust only for validation/compilation until the discrepancy is understood.
At M4: compare the same task with direct FilmCraft + Agent use. Continue investment
only when story decisions reduce effort without losing quality/control.
At M5: seek repeated usage and an independent integrator; stars or passing tests
are not demand evidence. A suggested early target is 3 of 5 testers completing a
rough cut and two returning to another project; these are goals, not results.

## Release and operation

Each candidate names a full source SHA, locked dependencies, schema/compiler/adapter
versions, host version and test artifacts. Publish is an explicit maintainer action.
Backward-incompatible schema changes need a migration and refusal behavior for newer
schemas. Future executor failures preserve partial/unknown receipts instead of
claiming rollback. Real-media listening, native trust/signing and target editor
acceptance remain separate from synthetic CI.

## Native migration strategy

Use the [OpenFilm transfer map](docs/migration/OPENFILM_RUST_PORT_MAP.md) and
[gap gates](docs/migration/NATIVE_BOUNDARIES_AND_GAPS.md) as the detailed task inventory.
The original planning inventory itself changed no runtime behavior. The accepted
runtime scope includes M0, finite importer/OTIO, explicit supplied-byte verification
and the bounded brief audit. The actual synthetic25fps host proof is recorded in
[TASKS](TASKS.md#accepted-lifecycle); broader M1 acceptance and M2–M5 remain open.

- First native boundary: strict models, precise clocks, read-only snapshot mapping,
  capability/loss compilation and finite OTIO. Prove one host handoff before a full planner.
- Then port selected duplicate/event/scoring/graph/constraint/planner algorithms using
  frozen synthetic fixtures and source-aware differential tests. Optional features remain optional.
- Move stateful reviews, jobs and recovery only behind an admitted execution contract.
  An external editor is not part of a local SQLite transaction; partial/unknown receipts matter.
- Preserve OpenFilm as a consumer/reference renderer. Retain Vue, codecs, GPU/finishing
  and full NLE features outside the SDK. Rust adapters disclose actual tool/model dependencies.
- Extract appropriate native Rust manifest/path helpers only after provenance and race review;
  do not import Tauri/GTK/Node launching into the portable SDK.
- Freeze -> Shadow -> Feature flag -> Retire, one authority per service. Never rewrite
  a user's live OpenFilm Catalog as part of read-only import.

A per-task native implementation is complete only when its source-to-target behavior,
production dependency boundary, regressions and actual declared acceptance gates are proven.
A native binary, an external FFmpeg wrapper and a pure Rust algorithm are different scopes.
