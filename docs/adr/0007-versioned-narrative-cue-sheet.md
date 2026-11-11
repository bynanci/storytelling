<!-- SPDX-License-Identifier: MIT -->
# ADR 0007: Versioned narrative audio cue sheet

- **Status:** Accepted bounded CLI implementation; recording, playback, mixing and human listening remain outside this decision.
- **Date:** 2026-11-11
- **Task / requirement:** Issue #66 · N-09 / N-R09
- **Base:** `main` `b7fcf869d01727dcf408c457fdd42929dca80c7a` (latest main used for the PR #78 rebase)
- **Initial implementation base:** `main` `213c4963a1d72daf9d902cb0d75f737de86c1caf`
- **Depends on:** accepted ADR 0005 / `NarrativeDocumentV1`; merged N-03 and N-04 CLI foundations

## Context

`NarrativeDocumentV1` describes source units and spoken beats but intentionally
has no audio assets or media clocks. N-09 needs a recording-ready sequence of
speech, silence and sound cues, with IDs/revisions connected to the existing
narrative contract. Adding time and audio fields to N-02 would change a closed
contract and impose those fields on authors who only need text or illustration.

The merged N-03 example is about neighbors repairing a door, not the umbrella
workshop. N-09 can use a small, source-faithful umbrella document based on S1
without reusing unrelated speech. The supplied S3 WAV is a synthetic effect;
the engine must not imply that it was recorded at the event or inspect media.

## Decision

Define a separate closed `NarrativeCueSheetV1`. Bind it to one exact document
ID/revision and one spoken variant. A narration cue references a beat and its
source unit revision; the renderer obtains the words from that beat instead of
copying them into a second editable field. An effect cue points to a declared
audio asset ID/revision, relative path, SHA-256 declaration and duration.
Editorial directions such as a pause remain labeled as production choices.

Use milliseconds as the V1 time unit. Support absolute half-open intervals,
anchors after earlier cue IDs, and explicitly unknown time. Known, estimated
and unknown durations remain distinct. Unknown time is not zero; a cue whose
anchor or duration cannot resolve remains visible and is excluded from
conflict checks. Separate track IDs may overlap. On an `exclusive` track,
overlapping resolved intervals reject, while touching half-open ranges pass.
The CLI renders a deterministic cue table and does not record, play or mix.

Keep `NarrativeDocumentV1` unchanged. Retain the existing `verify-source`
command as the separate way to check supplied bytes against a declared digest.
Passing the cue-sheet validator alone does not verify asset bytes, prove
audio quality or establish host compatibility.

## Alternatives

| Option | Cost / failure mode | Decision |
| --- | --- | --- |
| Add audio and time fields to NarrativeDocumentV1 | Breaks the accepted closed shape and couples text-only work to media timing. | Reject |
| Extend a video plan or OTIO profile | Imports shot, frame-rate or NLE semantics into a hand-operated audio script. | Reject |
| Separate versioned cue sheet bound to a narrative revision | Adds one strict reader and explicit timing semantics while preserving the existing source contract. | Adopt |

## Consequences and review evidence

The core owns wire types; the pure engine validates identity, source refs,
track rules and intervals; the CLI reads the two inputs and prints text. This
slice does not contain media bytes or invoke a recording or mixing host.

The [profile](../narrative/NARRATIVE_CUE_SHEET_V1.md),
[schema](../../contracts/narrative-cue-sheet-v1.schema.json),
[fixtures](../../tests/fixtures/narrative-v1/README.md) and
[validation record](../validation/n09-cue-sheet.md) implement the bounded
Issue #66 profile. This decision does not accept host playback, recording, TTS,
mixing, or human listening. Those require separate capabilities and evidence.
