<!-- SPDX-License-Identifier: MIT -->
# NarrativeCueSheetV1 profile

**Status:** bounded N-09 implementation profile.

**Schema:** [JSON Schema Draft 2020-12](../../contracts/narrative-cue-sheet-v1.schema.json).

**Example and CLI evidence:** [N-09 validation record](../validation/n09-cue-sheet.md).

[繁體中文規格](NARRATIVE_CUE_SHEET_V1.zh-TW.md)

## Boundary

`NarrativeCueSheetV1` is a separate, closed, versioned document. It identifies
one exact `NarrativeDocumentV1` revision and one `spoken_outline` variant. It
does not add audio, clock, duration or asset fields to the accepted
`NarrativeDocumentV1` contract. A narration cue points to a beat and unit
revision; the CLI renders the beat's declared words from that document. Audio
assets are declarations with repository-relative paths, revisions, hashes,
provenance and duration. The engine does not read those paths, authenticate
hashes, play audio, record speech, synthesize speech or mix tracks.

A successful check means that the two documents bind, their references resolve,
the declared cue order and track ranges satisfy this profile, and the supplied
metadata is within bounds. It does not establish that media bytes match their
declarations or that a recording sounds right.

## Time and track semantics

The V1 time unit is milliseconds. Cue array order and the `order` field both
start at one and increase together. An absolute interval is half-open
`[start_ms, end_ms)`, so two cues whose end and start are equal are adjacent,
not overlapping. Its status is `known` or `estimated`.

An `after` cue anchors to an earlier cue ID and declares a known, estimated or
unknown duration. Its interval resolves only if the anchor's end and its own
duration resolve. An `unknown` timing carries a reason and remains in the cue
sheet. Unresolved timing is counted, is not converted to zero, and cannot prove
that an exclusive track is conflict-free. The CLI makes that gap visible.

Track IDs describe independent voice, sound-effect or marker tracks. Different
tracks may overlap, including two tracks each marked `exclusive`. On one
`exclusive` track, two fully resolved intervals conflict only when their
half-open ranges intersect. A `mixable` track permits overlap; a `marker` track
holds non-audio directions and is not mixed. Track channel and cue kind must
match: voice tracks hold narration, sound-effect tracks hold effects, and voice
or marker tracks can hold silence cues.

## Sources and revisions

The sheet's document identity and each narration beat's variant, beat ID, unit
ID and unit revision must match the supplied `NarrativeDocumentV1`. A stale
document or beat revision is rejected. Sound-effect cues refer to a declared
audio asset by ID and revision. Asset IDs and cue IDs are unique in their own
collections. Editorial choices, such as a pause length, remain identified as
production choices instead of source facts.

The engine validates a declared SHA-256 spelling and a safe relative path; it
does not calculate the hash. Use the existing `verify-source` command when the
actual bytes need checking. The N-09 example checks `taps.wav` against its
declared SHA-256 before calling its duration known.

## Limits and diagnostics

Input reuses the 1 MiB JSON bound and duplicate-key/depth preflight from the
narrative reader. This profile allows up to 32 tracks, 128 audio assets, and
512 cues, with IDs up to 128 ASCII bytes, nonzero revisions, and declared
durations no longer than one hour. Objects are closed; unknown fields, unknown
versions, duplicate IDs, unsupported source kinds, stale references, bad
intervals and exclusive-track conflicts reject with one JSON diagnostic.

Diagnostics include `narrative.cue.unsupported_version`,
`narrative.cue.stale_document`, `narrative.cue.stale_revision`,
`narrative.cue.invalid_range`, `narrative.cue.duplicate_id`, and
`narrative.cue.track_conflict`. A conflict includes the two cue IDs. Errors do
not echo source text or paths from untrusted JSON.

## Local entry point

From the repository root:

```sh
cargo run --locked -p storytelling-cli -- render-cue-sheet \
  tests/fixtures/narrative-v1/n09-umbrella-source-v1.json \
  tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
```

The command reads two explicit files, or one file and `-` for stdin, then emits
a deterministic human-readable cue sheet. Successful output uses exit 0;
invalid declarations emit one JSON line and exit 1; usage errors exit 2; input
I/O errors exit 3. The CLI is a read-only renderer, not an audio host.

Role, source and timing values in the human-readable output are JSON-quoted so
control characters remain visible within their field and cannot add cue lines.
