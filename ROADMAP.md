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
| M0 foundation | Implementation candidate | Validate a declared StoryPlan through a usable Rust CLI | Strict contract, source bounds, immutable input, same-head 3-OS CI, maintainer review |
| M1 / v0.1 handoff | Planned | Existing story choices become an editable FilmCraft rough cut | Real import/render/readback/save/reopen; supported profile and loss report |
| M2 / v0.2 planning | Planned | Constraints and evidence produce an explainable shot proposal | Hard constraints satisfied or explicit conflicts; frozen-input reproducibility |
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
