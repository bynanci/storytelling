# Storytelling roadmap

The first cross-media milestone is a correction an author can trust. If a
source note changes, the author should be able to update the spoken account and
identify the affected illustrations while keeping a deliberate choice in one
version. The [product rationale](docs/WHY_STORYTELLING.md) explains the research
and the cases where ordinary notes may already be sufficient.

## Product objective

| Where the work stands | What an author or integrator can do |
| --- | --- |
| Available · portable authoring skill | Bring an audience, purpose and material to the [Agent Skill](skills/storytelling/SKILL.md). Draft in a host agent, compare treatments or follow the [eight templates](skills/storytelling/references/multimodal-templates.md) by hand. The [workshop](skills/storytelling/references/workshop-demo.md) includes text, a diagram and a sound file. |
| Available · cross-media Rust CLI | `render-narrative` organizes one declared source into an ordered oral outline and two illustration briefs. `track-narrative-revisions` and `query-narrative-impact` are in the N-04 implementation PR. The [N-03](docs/validation/n03-narrative-cli.md) and [N-04](docs/validation/n04-narrative-revisions.md) records separate executable evidence from hosted CI and human inspection. |
| Available · bounded video CLI | Supply structured clips and planning constraints to the [CLI](docs/CLI_GUIDE.md). It can select fixed windows and export a picture-only timeline under its documented limits. See [video milestones](#milestones) for the OpenFilm snapshot adapter and FilmCraft evidence. |

The skill can be installed independently from the Rust workspace. The package
version is `0.1.0-dev.0`; [installation from GitHub](docs/PLATFORMS.md) is
available. Public registry publication remains a separate release decision.

## Cross-media narrative roadmap

**N-03 is merged; N-04 source revision tracking is in review.** [N-02](https://github.com/bynanci/storytelling/issues/49) is
accepted at the design gate. The local CLI produces one ordered oral outline
and two distinct illustration briefs from the same declared source. N-04 adds
append-only revisions, choice records and variant impact queries. Automated
evidence is recorded in the [N-03](docs/validation/n03-narrative-cli.md) and
[N-04 validation records](docs/validation/n04-narrative-revisions.md); author
inspection remains pending. **Evaluate in parallel.** The
[N-01 study packet](docs/narrative/N-01-AUTHOR-STUDY.zh-TW.md) is ready, with
the separate human trial and study decision still outstanding.

| Priority and outcome | Work in order | Evidence for the next decision |
| --- | --- | --- |
| 1 · Review and revise the first authoring workflow | Inspect the N-03 outputs and N-04 revision history with an author. Run N-01 separately. | Check the oral sequence, two distinct compositions, source tags, both impact queries and variant-local choice. Record actual author corrections separately from CLI/CI results. |
| 2 · Measure value and try a different type of writing | N-05 adds claims and their evidence after N-04. N-07 compares the shipped workflow against the ordinary-notes baseline after N-05. | A reviewer traces a claim as a reference changes. The maintainer uses completion, correction effort and attribution errors to decide whether to continue, narrow or stop. Report the host-assisted skill trial separately from a future trial using the native tool. |
| 3 · Select additional profiles by demonstrated need | N-06 compares brainstorm branches, screenplay reveals and video mapping after N-04. N-10 implements one profile only if N-06 selects it. N-08 style rules and N-09 audio cues can be explored after N-02. | Use one author brief in two treatments. Have a reader or listener identify the intended change in attention or reveal. If the template already serves the author, retain it and defer native implementation. |
| 4 · Hand work to another tool | N-11 builds on N-04 source revisions. N-12 can evaluate one external host independently. | A second person can locate the right sources and identify missing or superseded material. For a real media artifact, open or play the result. If a host is unavailable, show which part of the brief remains usable. |

The technical work belongs in the [task and dependency list](TASKS.md#cross-media-narrative-backlog)
and [multimodal backlog](TASKS.md#multimodal-rust-backlog). The
[architecture](docs/ARCHITECTURE.md) places declared data in core, local
validation in the engine and I/O in the CLI. Drafts and author judgments remain
with the skill, host and author. Video SDK work proceeds on its own track toward
an editable 60–90 second rough cut from 30–50 selected photos, clips and
narration. A future bridge to its planner needs real asset IDs and source ranges.

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
