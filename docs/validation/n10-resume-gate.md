<!-- SPDX-License-Identifier: MIT -->
# N-10 / N-R10: resume gate for narrative-to-video coverage

English · [繁體中文](n10-resume-gate.zh-TW.md)

**Status:** G2 contract accepted; G3 CLI and automated G4 verification merged in PR #97. Codex replayed the supplied inputs with the merged CLI, and the maintainer supplied an `ffprobe` report. A fresh author-operated CLI usability session remains open; the probe report is not independently audited here.
**Issue:** [#67](https://github.com/bynanci/storytelling/issues/67) · **Discovery:** [#53](https://github.com/bynanci/storytelling/issues/53) · **Selected V decision:** [#53 comment](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432) · **G2 acceptance:** [maintainer decision](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322) · **G2 merge:** PR #93, merge `80577b7334110abf0c0765ec1d1c3e4cf4ae3996`.
**G3/G4 implementation:** [PR #97](https://github.com/bynanci/storytelling/pull/97), merge `169bf6baff33cd452238337237d437e15ac5b292`; exact PR head `aa9cafae6dcd51bc78188ac5b80b9bbe3155d234` passed all six jobs in [run #376](https://github.com/bynanci/storytelling/actions/runs/43148901399). Automated code review completed with no findings.
**Decision history:** the 2026-11-11 defer-all decision remains the N-06 outcome; the 2026-11-12 V decision supersedes it only for N-10. Screenplay and brainstorm remain deferred. The maintainer accepted the exact field rules, 21 diagnostics and ordering, result status, and nine synthetic fixtures. The separate G3 Rust CLI work does not alter that frozen contract.

## Latest checkpoint · 2026-11-12

On 2026-11-12 the requester selected **V**, with this concrete author task: choose two story units, declare footage for one and no asset for the other, then confirm that the handoff preserves the uncovered unit as unresolved and blocks ready-for-planning/export status. The selected version boundary is a separate, video-only, read-only `NarrativeVideoCoverageV1` declaration and validator. It performs no media reads or verification, matching, rendering, OTIO/export, or editor writes; existing narrative and video contracts remain unchanged. This selection is not itself a human trial or real-media result. The maintainer later supplied a two-session author account; its source and limits are recorded below and in the [#67 decision comment](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322).

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

**Selected slice:** `NarrativeVideoCoverageV1`, a separate, read-only, non-executable declaration and validation result. Its profile/version boundary and exact wire rules are accepted at G2. Video only; no image/audio synchronization, transcription, clip search, AI matching, renderer, export, OTIO generation, editor write or automatic authorization.

The candidate contract embeds the complete `NarrativeDocumentV1` snapshot and names one variant. An explicit ordered `selected_unit_refs` list selects one or more current units that occur in that variant; this is the author's declared selection, not an inference about intent. A declared asset-catalog snapshot supplies catalog ID/revision and asset IDs/revisions/durations. `mappings` must account for every selected reference exactly once and in the same order. Each row is either `mapped` to an exact catalog asset revision and half-open rational source range, or `unresolved` with a reason. Catalog and media facts remain declarations: no host asset is opened, no file digest is authenticated, and no footage content is checked.

The accepted [ADR 0010](../adr/0010-narrative-video-coverage-v1.md), [input schema](../../contracts/narrative-video-coverage-v1.schema.json), [result schema](../../contracts/narrative-video-coverage-result-v1.schema.json), and [paired fixtures](../../tests/fixtures/narrative-video-coverage-v1/README.md) define the accepted field names, limits, validation order and diagnostics. A valid input with an `unresolved` row returns a deterministic blocked result; it is not a malformed document. Missing, duplicate, stale or dangling references are contract errors. A complete declared mapping may enter planning, but never proves export readiness or actual media validity.

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

The accepted stable profile codes include `video_coverage.missing_selected_unit`, `video_coverage.unresolved_required`, `video_coverage.stale_unit_revision`, `video_coverage.dangling_asset`, and `video_coverage.invalid_range`; the complete taxonomy and first-error order are in ADR 0010.

**Non-goals and invariants:** keep NarrativeDocumentV1, N-04 ledger, N-08 profiles, N-11 manifest, StoryBriefV1, PlannedStoryV1 and existing video/OTIO contracts unchanged. Do not treat a narrative unit ID as an asset ID, infer a missing clip, upgrade a creative proposal to an observation, or publish a plan with unaccounted selected units. An `unresolved` row is a usable editorial warning, not an empty success.

## G2 — Accepted contract and staged delivery

| Stage | Deliverable and proof | Owner / gate |
| --- | --- | --- |
| N10-00 · Adoption | V, the two-unit/one-footage author task, and the video-only read-only V1 boundary are recorded in #53 and this packet. | Maintainer · **recorded** |
| N10-01 · Contract | Accepted the versioned profile, closed schemas, source/catalog identity, exact ranges, diagnostic policy and nine fixtures in PR #93. | Maintainer · **G2 accepted** |
| N10-02 · Pure validation | Test-first observed RED then GREEN; bounded Rust validation has no filesystem/model/host calls or implicit asset selection. | Engine · **G3 implemented and merged in PR #97** |
| N10-03 · CLI | Add an **explicit, opt-in** local read-only entry point with deterministic success/error envelopes and contract tests. | CLI · **G3 implemented and merged in PR #97** |
| N10-04 · Regression | N-02/N-04/N-08/N-11, Planner V1, video V1 and OTIO regressions plus debug/release, formatting, Clippy, repository checks, pinned official OTIO oracle and exact-head hosted CI passed; all six jobs passed in run #376. Automated code review reported no findings. | Reviewer · **automated G4 passed** |
| N10-05 · Human/media evidence | The prior VH-01 sessions remain maintainer-supplied and unaudited; session 2 predates the CLI. Codex replayed the two supplied JSON inputs after merge and recorded exit 0/unresolved and exit 1/invalid-range results, with unchanged inputs and identical file/stdin output. The attachment reports an `ffprobe` run of 5.000000 seconds / 150 frames at 30 fps, but the media was not attached and the report was not independently audited by Codex. | Author / maintainer · **machine replay and probe report recorded; fresh author usability session pending** |

G2 acceptance is recorded before G3 implementation. The supplied author account contains the exact task, starting material, verbatim words, observations and limitations; it was not independently audited and does not replace a CLI-based re-run. Preserve all original media in private storage; public fixtures remain invented. A new release or migration requires a separate G5 decision.

## Decision record

- **Selected V (2026-11-12):** the author chooses two story units, declares footage for one and no asset for the other, and verifies that the gap remains visible and blocks ready-for-planning/export. Adopt the profile/version boundary `NarrativeVideoCoverageV1`, video-only, declaration-only and read-only.
- **S and B:** remain deferred; they are not part of N-10.
- **D:** was the 2026-11-11 N-06 decision and remains a valid historical outcome, but the later V decision supersedes it for N-10 only.

The mode-selection, G2 contract, G3 implementation and automated G4 regression gates are complete. #67 remains open for a fresh author-operated CLI session with recorded author time and usability observations. A maintainer supplied a separate `ffprobe` execution record; because the media file was not provided, Codex did not independently reproduce or audit that measurement. Session 2 in the earlier author account predates the CLI. V1 reads only JSON declarations and exact rational ranges, so its green result never proves that media bytes match a catalog claim. Cancel native work if authors can safely finish with the current text table or cannot produce a concrete selected-unit/asset gap. No G5 release or publication is authorized.

**Evidence separation:** the [G3 CLI validation record](n10-video-coverage-cli.md) records code and CI evidence; the [post-merge execution record](n10-05-execution-record.md) records the supplied-input replay and the media-probe report's limits. The two-session author account is preserved in [#67](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322) and is maintainer-supplied, not independently audited. Catalog IDs and duration are declarations; this implementation verifies no media bytes, file digest, synchronization, OTIO, or host behavior.
