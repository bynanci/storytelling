<!-- SPDX-License-Identifier: MIT -->
# N-09 / #66: Repair workshop recording cue sheet

**Scope:** A separate, closed `NarrativeCueSheetV1` and local `render-cue-sheet` command. `NarrativeDocumentV1` remains unchanged.

## Request and source mapping

The `$storytelling` request is to provide a recording-ready invitation with a short voice line, a one-beat pause and two taps. The fixed workshop source facts are the Saturday event and “bring one umbrella.” The spoken cue is “週六修傘工作坊，請帶一把傘。” and references `umbrella-workshop@1 / oral / invite / S1@1`. The line is pending recording, so its duration remains unknown. The requested pause is an editorial production choice; 500 ms is an estimate. S3 is the supplied synthetic two-tap effect, not a field recording.

The merged #50 narration uses the neighbors-repairing-a-door material and is unrelated to this invitation. The fixture therefore uses the minimum source-faithful workshop document rather than reusing that line. The WAV header reports mono, 16-bit PCM, 16 kHz, 17,280 frames and 1,080 ms. `verify-source` confirms 34,604 supplied bytes against SHA-256 `4008e27e4c94376747a887df8d873820e4ab88e668816f79ddca5c9d69bb202b`. This checks the declared bytes only; it is not a listening or mix review.

## Local output and replay

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet \
  tests/fixtures/narrative-v1/n09-umbrella-source-v1.json \
  tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
```

The command exits 0 and prints the voice, pause and WAV cues in order. Each row identifies its role, track, source, direction and timing. Three linked intervals remain unresolved because the voice length is unknown; the renderer does not treat unknown time as zero or claim that unresolved cues cannot conflict. The actual standard output and exit status are saved in [`valid.stdout`](n09-cue-sheet/valid.stdout) and [`valid.exit`](n09-cue-sheet/valid.exit).

The human-readable projection JSON-quotes role, source and timing values so an input newline cannot create forged cue or direction lines. The CLI integration test `escapes_untrusted_multiline_cue_fields_in_human_readable_output` injects newlines into all three fields and checks they remain escaped in one output row. Asset paths reject control characters with `narrative.cue.invalid_asset_path`.

The original CLI output was run twice with identical inputs and compared byte-for-byte using `cmp`; the comparison exited 0. The integration tests also compare repeated CLI output, exercise stdin and confirm the input file is unchanged.

## Positive and negative cases

| Case | Expected result | Saved output | Exit |
| --- | --- | --- | --- |
| Pending voice, estimated pause and declared WAV duration | Accept; unknown voice length remains unresolved | [text](n09-cue-sheet/valid.stdout) | [0](n09-cue-sheet/valid.exit) |
| Overlap across two exclusive tracks | Accept; cross-track overlap is allowed | [text](n09-cue-sheet/cross-track-overlap.stdout) | [0](n09-cue-sheet/cross-track-overlap.exit) |
| Adjacent intervals on one exclusive voice track | Accept; `[0,1000)` and `[1000,1500)` do not overlap | [text](n09-cue-sheet/adjacent.stdout) | [0](n09-cue-sheet/adjacent.exit) |
| Overlap on the same exclusive track | Reject `narrative.cue.track_conflict` with both cue IDs | [diagnostic](n09-cue-sheet/track-conflict.stdout) | [1](n09-cue-sheet/track-conflict.exit) |
| Reversed interval | Reject `narrative.cue.invalid_range` | [diagnostic](n09-cue-sheet/reversed-interval.stdout) | [1](n09-cue-sheet/reversed-interval.exit) |
| Duplicate cue ID | Reject `narrative.cue.duplicate_id` | [diagnostic](n09-cue-sheet/duplicate-id.stdout) | [1](n09-cue-sheet/duplicate-id.exit) |
| Unsupported cue-sheet version | Reject `narrative.cue.unsupported_version` | Tested through stdin integration case | 1 |
| Stale document revision | Reject `narrative.cue.stale_document` | Tested through stdin integration case | 1 |
| Declared duration disagrees with WAV asset | Reject `narrative.cue.asset_duration_mismatch` | Tested through stdin integration case | 1 |
| Supplied `taps.wav` bytes match declared digest | Accept; 34,604 bytes | [verification JSON](n09-cue-sheet/taps-asset-check.stdout) | [0](n09-cue-sheet/taps-asset-check.exit) |

Each saved case is produced by the normal executable entry point, not synthesized from expected values:

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet DOCUMENT.json CUE_SHEET.json
cargo run --release --quiet --locked -p storytelling-cli -- verify-source skills/storytelling/assets/umbrella-workshop/taps.wav 4008e27e4c94376747a887df8d873820e4ab88e668816f79ddca5c9d69bb202b
```

Replay the six saved CLI cases:

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-cross-track-overlap-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-adjacent-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-conflict-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-reversed-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-duplicate-id-v1.json
```

The repository rebase for PR #78 uses latest `main` commit `b7fcf869d01727dcf408c457fdd42929dca80c7a`; the rebase preserves the N-08, N-11 and N-12 mainline additions. The PR's required CI checks are attached to its exact rebased head at [PR #78](https://github.com/bynanci/storytelling/pull/78).

The deliverable is a hand-operated recording cue sheet. It contains no narration recording, TTS, playback or mix, and no human listening result is claimed. Unknown intervals remain ineligible to prove an exclusive track conflict-free; rerun after the voice has been recorded and its duration supplied.
