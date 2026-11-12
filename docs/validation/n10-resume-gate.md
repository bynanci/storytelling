<!-- SPDX-License-Identifier: MIT -->
# N-10 / N-R10: resume gate for narrative-to-video coverage

English · [繁體中文](n10-resume-gate.zh-TW.md)

**Status:** G0–G2 *decision proposal*, not a selected native mode or an implementation.
**Issue:** [#67](https://github.com/bynanci/storytelling/issues/67) · **Discovery:** [#53](https://github.com/bynanci/storytelling/issues/53) · **Updated base:** `a2de070db0c485bf55af6b7553bbfdd49f5efc44`.
**Controlling decision:** [Defer all three modes](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714) remains in force until the maintainer makes a later, explicit single-mode decision. A request to resume issue work is **not** an implicit selection of a profile.

## Latest checkpoint · 2026-11-12

The latest request is to continue issue #67 and this preparation PR. It advances this review proposal; it does not choose V/S/B or D, adopt a profile/version, or provide a new concrete media task. The linked #53 decision to defer all three therefore remains controlling. The maintainer must record one option and its author need/version scope before G3.

## G0 — The author need and first observable gap

The project owner has asked to resume #67 and has described a broader goal of using a structured story to coordinate footage, images, narration and editing choices. A privacy-safe **candidate** task is: an editor selects two story beats, identifies a clip for one beat, lacks footage for the other, and wants a handoff that never drops the missing beat or pretends the cut is ready for export. This is an author-reported desired workflow, **not** evidence that an actual clip, user session, time saving or failure was observed. No personal recordings, names, device paths or media were submitted for this gate.

The existing [N-06 video-handoff case](../narrative/n06-cases/video-handoff.zh-TW.md) already demonstrates the *contract gap* with synthetic declarations: S02@1 has a declared clip and exact range [5/2, 9/2); S03@1 is selected but has no asset. Existing `plan-story` accepts the S02-only subset because it cannot know that S03 was selected upstream. That is **not a planner bug**. It means the proposed source-unit → catalog-asset coverage join must be checked **before** planning.

### Three choices, one possible next mode

| N-06 candidate | Smallest existing solution | Why it is not the first proposed native slice |
| --- | --- | --- |
| Branching brainstorm | Two named variants and an external pending-question table | No demonstrated requirement to persist an exclusive unresolved choice as typed state |
| Screenplay reveal | NarrativeDocumentV1 named beats plus N-08 order checks | No demonstrated requirement beyond existing reveal-order presentation for typed scenes/dialogue |
| **Story-to-video coverage (recommended for owner review)** | An explicit manual coverage table with missing rows | The end-to-end handoff has a specific integrity gap: selected narrative units can disappear before Planner V1 receives its input |

**Recommendation is not adoption.** A new maintainer decision must select *exactly one* candidate, name the author task and approve the version boundary; alternatively continue deferral. Do not describe an unanswered request or a proposed synthetic case as a successful user study.

## G1 — Proposed bounded acceptance *if* video mapping is selected

**Proposed slice:** `NarrativeVideoCoverageV1`, a separate, read-only, non-executable declaration and validation result. The name and fields below are **design candidates, not accepted wire format**. Video only; no image/audio synchronization, transcription, clip search, AI matching, renderer, export, OTIO generation, editor write or automatic authorization.

Input would explicitly declare document ID/revision, selected variant and exact ordered source-unit references, a catalog identity and asset IDs, and a coverage row for **every selected unit**. A row either declares a valid catalog asset plus exact half-open rational source range or is explicitly `unresolved` with a reason. A `declared` mapping proves only declaration consistency, not that footage depicts the event; no host asset is opened and no file digest is authenticated.

| Test | Expected deterministic result |
| --- | --- |
| A complete synthetic mapping of every selected unit | Structured declaration check passes; still not executable or media-verified |
| Selected S03@1 is unresolved | Report incomplete coverage; **block** ready-for-planning/export status |
| Selected S03@1 is silently omitted | Reject incomplete selected-unit coverage |
| One selected unit has duplicate mapping rows | Reject duplicates rather than arbitrarily pick a clip |
| S03@2 is requested but source is S03@1 | Reject stale revision |
| Mapping points to an asset absent from the declared catalog | Reject dangling asset |
| End > declared asset duration; start >= end; noncanonical rational | Reject invalid/out-of-bounds range |
| Narrative unit ID happens to equal an asset ID | Never infer a join by name; require explicit exact refs |
| Same valid input is checked twice | Same response bytes and stable diagnostic codes |
| Unsupported version / duplicate JSON field / excessive input | Safe, bounded rejection; no mutations |

Proposed diagnostic families are `coverage.missing_selected_unit`, `coverage.unresolved_required`, `coverage.stale_revision`, `coverage.dangling_asset`, and `coverage.invalid_range`; **do not freeze names or an API** before G2 architecture review.

**Non-goals and invariants:** keep NarrativeDocumentV1, N-04 ledger, N-08 profiles, N-11 manifest, StoryBriefV1, PlannedStoryV1 and existing video/OTIO contracts unchanged. Do not treat a narrative unit ID as an asset ID, infer a missing clip, upgrade a creative proposal to an observation, or publish a plan with unaccounted selected units. An `unresolved` row is a usable editorial warning, not an empty success.

## G2 — Proposed architecture and staged delivery (blocked pending owner choice)

| Stage | Deliverable and proof | Owner / gate |
| --- | --- | --- |
| N10-00 · Adoption | Owner selects **video mapping only**, a real author task and its version scope, or explicitly maintains deferral. Link the new decision to #53 and #67. | Maintainer · **BLOCKED until explicit response** |
| N10-01 · Contract | Separate versioned profile, closed JSON schema, exact source/catalog identity and bounded rational ranges; positive and negative fixtures reviewed before Rust code. | Core / reviewer · G2 |
| N10-02 · Pure validation | Test-first observed RED, pure Rust validation, then GREEN; no filesystem/model/host calls and no implicit asset selection. | Engine · G3 |
| N10-03 · CLI | Add an **explicit, opt-in** local read-only entry point with deterministic success/error envelopes and contract tests. | CLI · G3–G4 |
| N10-04 · Regression | Re-run N-02/N-04/N-08/N-11, Planner V1, video V1, OTIO, debug/release, formatting, Clippy, repository checks and exact-head CI. | Reviewer · G4 |
| N10-05 · Human/media evidence | An actual selected-unit list, real catalog and declared ranges, including one genuine absent asset; author reviews the handoff and reports whether the table is enough. File/hash/host playback/import are separate gates. | Author / maintainer · G4 / possible future G5 |

No G3 work starts before N10-00. Any author study must record the exact task, starting material, user words, observation and limits; a synthetic dry run cannot replace it. Preserve all original media in private storage; public fixtures remain invented. A new release or migration requires a separate G5 decision.

## Decision record to add to #67 (not yet answered)

- **Option V:** Adopt *story-to-video coverage* as the sole N-10 native profile for a read-only, video-only V1 validator. Confirm the author workflow and explicit missing-footage handling; exclude media loading and export.
- **Option S:** Adopt *screenplay reveal* instead, with a newly specified actual need and version scope.
- **Option B:** Adopt *branching brainstorm* instead, with a newly specified actual need and version scope.
- **Option D:** Keep the 2026-11-11 defer-all decision; use the existing manual coverage table.

Until an option is explicitly selected, #67 remains open/blocked and the above tests are **proposed**, not completed. Cancel native work if the author can safely finish with the current text table or cannot produce a concrete selected-unit/asset gap. This document is a reviewable starting point, **not** a release or an accepted new schema.

**Evidence separation:** existing [engineering probes](../narrative/n06-cases/evidence/probes.json) establish current contract behavior; this document adds no runtime results, human validation, verified asset IDs or new CI result.
