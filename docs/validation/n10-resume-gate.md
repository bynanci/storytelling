<!-- SPDX-License-Identifier: MIT -->
# N-10 / N-R10: resume gate for narrative-to-video coverage

English · [繁體中文](n10-resume-gate.zh-TW.md)

**Status:** G0–G1 scope selected; the G2 `NarrativeVideoCoverageV1` contract is proposed for review. No G3 implementation is included here.
**Issue:** [#67](https://github.com/bynanci/storytelling/issues/67) · **Discovery:** [#53](https://github.com/bynanci/storytelling/issues/53) · **Selected V decision:** [#53 comment](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432) · **Updated base:** `a2de070db0c485bf55af6b7553bbfdd49f5efc44`.
**Decision history:** the 2026-11-11 defer-all decision remains the N-06 outcome; the 2026-11-12 V decision supersedes it only for N-10. Screenplay and brainstorm remain deferred. The profile boundary is selected; exact field rules and diagnostics still need G2 review.

## Latest checkpoint · 2026-11-12

On 2026-11-12 the requester selected **V**, with this concrete author task: choose two story units, declare footage for one and no asset for the other, then confirm that the handoff preserves the uncovered unit as unresolved and blocks ready-for-planning/export status. The selected version boundary is a separate, video-only, read-only `NarrativeVideoCoverageV1` declaration and validator. It performs no media reads or verification, matching, rendering, OTIO/export, or editor writes; existing narrative and video contracts remain unchanged. This selection is not a human trial or real-media result. G2 review must accept the detailed wire contract before G3.

## G0 — The author need and first observable gap

The project owner has asked to resume #67 and has described a broader goal of using a structured story to coordinate footage, images, narration and editing choices. A privacy-safe **candidate** task is: an editor selects two story beats, identifies a clip for one beat, lacks footage for the other, and wants a handoff that never drops the missing beat or pretends the cut is ready for export. This is an author-reported desired workflow, **not** evidence that an actual clip, user session, time saving or failure was observed. No personal recordings, names, device paths or media were submitted for this gate.

The existing [N-06 video-handoff case](../narrative/n06-cases/video-handoff.zh-TW.md) already demonstrates the *contract gap* with synthetic declarations: S02@1 has a declared clip and exact range [5/2, 9/2); S03@1 is selected but has no asset. Existing `plan-story` accepts the S02-only subset because it cannot know that S03 was selected upstream. That is **not a planner bug**. It means the proposed source-unit → catalog-asset coverage join must be checked **before** planning.

### Three choices, one possible next mode

| N-06 candidate | Smallest existing solution | Why it is not the first proposed native slice |
| --- | --- | --- |
| Branching brainstorm | Two named variants and an external pending-question table | No demonstrated requirement to persist an exclusive unresolved choice as typed state |
| Screenplay reveal | NarrativeDocumentV1 named beats plus N-08 order checks | No demonstrated requirement beyond existing reveal-order presentation for typed scenes/dialogue |
| **Story-to-video coverage (selected for N-10)** | An explicit manual coverage table with missing rows | The end-to-end handoff has a specific integrity gap: selected narrative units can disappear before Planner V1 receives its input |

The maintainer selected exactly one candidate and recorded the author task and version boundary in [#53](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432). The synthetic N-06 case remains an engineering example, not a successful user study.

## G1 — Selected bounded acceptance

**Selected slice:** `NarrativeVideoCoverageV1`, a separate, read-only, non-executable declaration and validation result. The profile/version boundary and exclusions are selected; the exact wire rules below are a G2 proposal, not yet an accepted contract. Video only; no image/audio synchronization, transcription, clip search, AI matching, renderer, export, OTIO generation, editor write or automatic authorization.

The candidate contract embeds the complete `NarrativeDocumentV1` snapshot and names one variant. An explicit ordered `selected_unit_refs` list selects one or more current units that occur in that variant; this is the author's declared selection, not an inference about intent. A declared asset-catalog snapshot supplies catalog ID/revision and asset IDs/revisions/durations. `mappings` must account for every selected reference exactly once and in the same order. Each row is either `mapped` to an exact catalog asset revision and half-open rational source range, or `unresolved` with a reason. Catalog and media facts remain declarations: no host asset is opened, no file digest is authenticated, and no footage content is checked.

The proposed [ADR 0010](../adr/0010-narrative-video-coverage-v1.md), [input schema](../../contracts/narrative-video-coverage-v1.schema.json), [result schema](../../contracts/narrative-video-coverage-result-v1.schema.json), and [paired fixtures](../../tests/fixtures/narrative-video-coverage-v1/README.md) define candidate field names, limits, validation order and diagnostics. A valid input with an `unresolved` row returns a deterministic blocked result; it is not a malformed document. Missing, duplicate, stale or dangling references are contract errors. A complete declared mapping may enter planning, but never proves export readiness or actual media validity.

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

The proposed stable profile codes include `video_coverage.missing_selected_unit`, `video_coverage.unresolved_required`, `video_coverage.stale_unit_revision`, `video_coverage.dangling_asset`, and `video_coverage.invalid_range`; the complete taxonomy and first-error order are in ADR 0010 and remain subject to G2 review.

**Non-goals and invariants:** keep NarrativeDocumentV1, N-04 ledger, N-08 profiles, N-11 manifest, StoryBriefV1, PlannedStoryV1 and existing video/OTIO contracts unchanged. Do not treat a narrative unit ID as an asset ID, infer a missing clip, upgrade a creative proposal to an observation, or publish a plan with unaccounted selected units. An `unresolved` row is a usable editorial warning, not an empty success.

## G2 — Selected boundary, proposed contract and staged delivery

| Stage | Deliverable and proof | Owner / gate |
| --- | --- | --- |
| N10-00 · Adoption | V, the two-unit/one-footage author task, and the video-only read-only V1 boundary are recorded in #53 and this packet. | Maintainer · **recorded** |
| N10-01 · Contract | Review the separate versioned profile, closed schema, source/catalog identity, exact ranges, paired fixtures and error policy in ADR 0010. Freeze before code. | Core / maintainer · **G2 review pending** |
| N10-02 · Pure validation | Test-first observed RED, pure Rust validation, then GREEN; no filesystem/model/host calls and no implicit asset selection. | Engine · G3, after N10-01 acceptance |
| N10-03 · CLI | Add an **explicit, opt-in** local read-only entry point with deterministic success/error envelopes and contract tests. | CLI · G3–G4 |
| N10-04 · Regression | Re-run N-02/N-04/N-08/N-11, Planner V1, video V1, OTIO, debug/release, formatting, Clippy, repository checks and exact-head CI. | Reviewer · G4 |
| N10-05 · Human/media evidence | An actual selected-unit list, real catalog and declared ranges, including one genuine absent asset; author reviews the handoff and reports whether the table is enough. File/hash/host playback/import are separate gates. | Author / maintainer · G4 / possible future G5; no evidence yet |

No G3 work starts before the G2 contract is accepted at N10-01. Any author study must record the exact task, starting material, user words, observation and limits; a synthetic dry run cannot replace it. Preserve all original media in private storage; public fixtures remain invented. A new release or migration requires a separate G5 decision.

## Decision record

- **Selected V (2026-11-12):** the author chooses two story units, declares footage for one and no asset for the other, and verifies that the gap remains visible and blocks ready-for-planning/export. Adopt the profile/version boundary `NarrativeVideoCoverageV1`, video-only, declaration-only and read-only.
- **S and B:** remain deferred; they are not part of N-10.
- **D:** was the 2026-11-11 N-06 decision and remains a valid historical outcome, but the later V decision supersedes it for N-10 only.

The mode-selection gate is complete. #67 remains open while G2 reviews and freezes the candidate schema; no implementation or human/media acceptance is complete. Cancel native work if authors can safely finish with the current text table or cannot produce a concrete selected-unit/asset gap. This packet is a reviewable design, **not** a release or proof of actual media.

**Evidence separation:** existing [engineering probes](../narrative/n06-cases/evidence/probes.json) establish current contract behavior; this document adds no runtime results, human validation, verified asset IDs or new CI result.
