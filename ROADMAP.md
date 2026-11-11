# Storytelling roadmap

The roadmap is outcome-gated, not a promise of release dates. Package version
`0.1.0-dev.0` is a development identifier; no package or release is published.
See [TASKS](TASKS.md), [SDLC](docs/SDLC.md) and [architecture](docs/ARCHITECTURE.md).

## Product objective

Give authors and integrators a portable way to turn source material into a
structure they can explain, revise and hand off. A writing team can add the
[Agent Skill](skills/storytelling/SKILL.md) to its own workspace for
day-to-day drafting. A project with structured video data can use the local
[CLI](docs/CLI_GUIDE.md) in a script or automation. These entry points serve
different tasks and can be adopted independently. See the [platform guide](docs/PLATFORMS.md).

- **Video planning and handoff (M).** The working Rust CLI selects fixed source
  windows and exports a bounded picture timeline for review. The reference
  workflow targets an editable 60–90 second rough cut from 30–50 selected
  photos, clips and narration. Any project can supply the CLI's declared planning
  inputs. The current video interchange profile uses OpenFilm's finite snapshot
  format, and FilmCraft is its first tested editor. See [milestones](#milestones)
  for the scope of current evidence.
- **Cross-media narrative planning (N, proposed).** Test whether authors can
  turn the same observations into a spoken outline, an illustration brief or
  a supported commercial claim while retaining where each detail came from.
  The skill already guides a host agent through these drafting tasks. This
  track starts by comparing authoring examples and then tests the proposed
  deterministic cross-media workflows and their CLI contracts.

## Cross-media narrative roadmap

The [Storytelling skill](skills/storytelling/SKILL.md) is the author-facing
entry point. Authors can start with [eight templates](skills/storytelling/references/multimodal-templates.md)
or a [small image, sound and text workshop](skills/storytelling/references/workshop-demo.md).
A host can use the available media or the supplied descriptions to draft in each form.
The N milestones below add the separately testable contracts, local tool
integration, revision tracking and measured author outcomes. Keep a natural-language
example beside each runnable technical slice so authors can use it from the skill.

| Outcome | Tasks and dependency | Decision and evidence |
| --- | --- | --- |
| Establish what authors need to reuse | N-01 discovery, then N-02 contract design | Compare manually prepared outputs for a synthetic everyday event. Keep source units, their relationships and proposed presentations distinct. Review an additive contract against existing video V1 fixtures. Narrow the design if a second output requires invented media fields. |
| Reuse a source in two media and revise it | N-03 follows N-02. N-04 adds revision tracking to N-03. | Produce one ordered oral outline and two spatial composition briefs from the same input. The author can keep an imagined detail in one visual variant, change a cited observation and identify the affected outputs. Compare repeat runs and inspect the briefs with an author. |
| Test the model on commercial copy | N-05 follows N-04 | Trace each factual claim to declared evidence or flag it for substantiation. Try removing the evidence and revising a claim. A human judges whether the evidence actually supports the wording. |
| Choose the next use case | N-06 follows N-04 and draws on N-05 when available | Compare grouped and branching ideas for brainstorming, scene and reveal order for screenwriting, and a mapping from selected units to video assets. Advance one profile only if its fixture and author feedback identify a benefit the existing profiles cannot deliver. |
| Decide whether to distribute a pilot | N-07 follows N-05. N-06 is optional for the first pilot. | Compare the core workflows against plain notes using the measures recorded in N-01. Report task completion, time to revise, attribution mistakes and aggregate author feedback. Record a continue, narrow or stop decision. A release has its own G5 decision. |

The [multimodal Rust backlog](TASKS.md#multimodal-rust-backlog) adds style profiles (N-08),
audio cue sheets (N-09), the profile selected by N-06 (N-10), a mixed-media handoff
manifest (N-11) and a host feasibility study (N-12). The proposed backend work has
its own evidence and can be prioritized after the shared N-02 contract.

The N track can start alongside the video operation journal (M3-02). Shared
ideas include explicit constraints, deterministic output and traceable review
records. The existing time, shot, OTIO and FilmCraft contracts remain owned by
the M track. A future video mapping needs real asset identifiers and source
ranges before it can call the existing planner.

Pick up a [cross-media task](TASKS.md#cross-media-narrative-backlog) for its
SDLC gate, dependencies, counterexamples and acceptance evidence.

## Milestones

| Milestone | Current state | User-visible outcome | Exit gate |
| --- | --- | --- | --- |
| M0 foundation | Accepted and merged on main | Validate a declared StoryPlan through a usable Rust CLI | Strict contract, source bounds, immutable input, same-head 3-OS CI, maintainer acceptance |
| M1 / v0.1 handoff | Finite M1-01/M1-02 and bounded M1-03 Linux 25 fps proof accepted; broader acceptance open | Existing story choices become an editable FilmCraft rough cut | Tested picture-only profile passed import/CPU preview/readback/save/reopen; audio, other profiles and human acceptance remain open |
| M2 / v0.2 planning | M2-01 audit accepted; M2-02 fixed-window planner accepted; M2-03 selected-domain live shadow accepted; full solver open | Constraints and evidence produce an explainable shot proposal | Hard constraints satisfied or explicit conflicts; frozen-input reproducibility |
| M3 / v0.3 reviewed execution | Bounded P1 composition review and P2 read-only planner bridge accepted; M3-02 journal accepted; M3-03 has a local test-only host proof, production executor open | Accepted plans execute without silently damaging prior work | Trusted approval, single writer, selected proposal/artifact binding, owner fencing, durable journal, safe in-flight/partial/unknown recovery |
| M4 / v0.4 consumers | Planned | OpenFilm and an external TypeScript consumer use one SDK | Single authority, full Review/Accept workflow, optional thin MCP |
| M5 / beta | Planned | Independent users and a second host finish a real story | Measured user outcomes, target-host matrix, distribution evidence |
| v1.0 | Not scheduled | Stable public contract with a supported upgrade path | Schema/SDK migrations, security/recovery and compatibility gates accepted |

## Dependency path

```text
M0 -> M1-01 -> M1-02 -> M1-03
          \-> M2-01 -> M2-02 -> M2-03
M1-03 + M2-02 -> M3-01/P1 composition review (accepted)
             -> M3-01/P2 selected proposal -> read-only OTIO review
             -> M3-02 operation journal -> M3-03 FilmCraft apply + readback
M2-03 + M3-03 -> M4 -> M5 -> v1.0
```

Requirement drafting and fixtures can overlap; host execution cannot skip the
finite interchange proof. No deadlines or aggregate product completion percentage
are inferred from code or test counts.

M3-01/P2 is merged as the bounded bridge, with its [bounded contract](docs/migration/PLANNER_OTIO_REVIEW_V1.md).
M3-02's bounded SQLite journal is accepted through [PR #56](https://github.com/bynanci/storytelling/pull/56)
with exact-head and post-merge hosted checks passed. M3-03 now has a test-only
isolated FilmCraft apply/readback proof for a lost reply after successful host
exit; its [record](docs/validation/m3-03-filmcraft-execution.md) keeps production
execution open. A timeout has unknown outcome: recovery must read back the
project before retry. If the same operation may still be in flight or readback
is ambiguous, stop instead of resending the cut. M4 remains queued after an
accepted production executor and readback gate.

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
