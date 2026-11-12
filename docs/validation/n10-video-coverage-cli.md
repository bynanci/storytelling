<!-- SPDX-License-Identifier: MIT -->
# N-10 G3 CLI implementation and validation

English · [繁體中文](n10-video-coverage-cli.zh-TW.md)

**Requirement:** N-R10 / [Issue #67](https://github.com/bynanci/storytelling/issues/67)  
**Accepted contract:** G2 decision in [#67](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322), delivered by [PR #93](https://github.com/bynanci/storytelling/pull/93), merged as `80577b7334110abf0c0765ec1d1c3e4cf4ae3996`.  
**G3 branch:** `feat/n10-video-coverage-cli-20261112`, based on that main commit.
**Implementation:** [PR #97](https://github.com/bynanci/storytelling/pull/97), merged as `169bf6baff33cd452238337237d437e15ac5b292` from exact PR head `aa9cafae6dcd51bc78188ac5b80b9bbe3155d234`.
**Hosted verification:** all six jobs passed for that exact head in [run #376](https://github.com/bynanci/storytelling/actions/runs/43148901399); automated code review completed with no findings.

## Delivered behavior

`storytelling render-video-coverage <FILE|->` validates the accepted closed `NarrativeVideoCoverageV1` input and emits one compact JSON line. It accepts either a spoken-outline or a single-illustration variant. The engine checks current narrative unit revisions, selection membership and order, declared catalog identities and exact rational ranges, then emits mapping rows in the selected order.

A valid `unresolved` row remains in the result, returns exit 0, sets `ready_for_planning` to `false`, and adds `video_coverage.unresolved_required`. A complete declaration can set `ready_for_planning` to `true`; that means the declarations pass this profile, not that footage exists, is authentic, or is suitable for export. Contract errors return one stable error and safe path with exit 1. Usage and I/O errors return 2 and 3.

Core owns wire DTOs. Engine performs bounded pure validation and deterministic projection. CLI reads only the supplied JSON and writes only stdout. It makes no catalog, filesystem, media, network, model, OTIO, project or editor calls. Existing NarrativeDocumentV1, video V1 and OTIO schemas are unchanged.

## Test-first evidence

The initial four normal-entry-point tests were run before implementation and all failed because `render-video-coverage` was not registered: valid inputs returned usage exit 2 instead of 0, and invalid inputs returned usage exit 2 instead of contract exit 1. After expanding coverage to seven tests, the full seven-test file was independently run against an archive of the exact base commit `80577b7334110abf0c0765ec1d1c3e4cf4ae3996`; it again observed **0 passed, 7 failed**, all at the unrecognized-command usage boundary. This confirms the new assertions distinguish the base behavior. The same command on the implementation branch passed **7/7**.

```sh
cargo +1.89.0 test --locked -p storytelling-cli --test video_coverage
```

The seven tests are:

- `accepts_complete_video_coverage_without_media_access`
- `reports_unresolved_video_unit_and_blocks_planning`
- `rejects_contract_failures_with_stable_codes_and_safe_paths`
- `compares_file_stdin_replay_and_preserves_input_bytes`
- `accepts_illustration_variant_sources`
- `rejects_shape_order_and_json_limit_violations`
- `rejects_cross_reference_duplicate_and_bound_errors_by_contract_code`

They exercise all accepted structural and cross-reference error codes across the cases, both variant kinds, valid complete and unresolved mappings, the nine accepted synthetic fixtures, missing/stale/dangling references, invalid exact ranges, unsupported versions, duplicate raw keys, unknown fields, selection order, asset revision, first-error order, the 1 MiB limit, depth 33, byte-identical file/stdin replay, input and neighboring-file preservation, and the absence of media files. The unresolved result and its expected fixture are compared after normalizing both platform-specific JSON-line endings; repeated runs and file/stdin forms must remain byte-identical on that host.

## Reproduction

From the repository root:

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

Expected and actual stdout (one line; exit 0):

```json
{"schema_version":1,"profile":"storytelling.narrative-video-coverage.v1","coverage_id":"two-unit-cut","narrative_ref":{"document_id":"n10-synthetic-story","document_revision":1,"variant_id":"rough-cut"},"asset_catalog_ref":{"catalog_id":"synthetic-catalog","revision":1},"selected_unit_refs":[{"unit_id":"S02","revision":1},{"unit_id":"S03","revision":1}],"mappings":[{"unit_ref":{"unit_id":"S02","revision":1},"status":"mapped","asset_ref":{"asset_id":"CLIP-01","revision":1},"source_range":{"start":{"numerator":"5","denominator":"2"},"end":{"numerator":"9","denominator":"2"}}},{"unit_ref":{"unit_id":"S03","revision":1},"status":"unresolved","reason":"No video asset was selected for this unit."}],"ready_for_planning":false,"diagnostics":[{"code":"video_coverage.unresolved_required","path":"$.mappings[1]"}]}
```

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage tests/fixtures/narrative-video-coverage-v1/invalid-range.json
```

Actual stdout (exit 1):

```json
{"error":{"code":"video_coverage.invalid_range","path":"$.mappings[0].source_range.end"}}
```

The standard-input form is also covered:

```sh
cat tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json | cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage -
```

It produces exactly the same bytes as the file form. No MP4 or MOV file is present in the fixture; `CLIP-01` and its duration are synthetic catalog declarations.

## Human and media evidence limits

A maintainer supplied one anonymous code, `VH-01`, and two earlier session descriptions in [the original #67 record](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322). The supplied order was spreadsheet first and a manual structured-contract walkthrough second. Session 2 predates this CLI, so its structured result codes were compared manually with the accepted contract rather than obtained from Rust CLI execution. The maintainer later supplied [VH-01-S3](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001), one post-merge author-operated self-session: the author reports 145 seconds, an initial `invalid_range`, correction to `5/1`, and then an unresolved result that blocks planning, with a verbatim statement and usability observations. VH-01 had prior exposure; the session was not randomized, blinded or independently observed. Codex did not observe or independently verify the author's actions or timing. Codex separately replayed the supplied JSON inputs and reproduced the reported output states; that replay corroborates outputs, not author operation. See the [post-merge execution record](n10-05-execution-record.md).

The supplied `clip-01.mp4` is described as a synthetic FFmpeg `testsrc` pattern. Codex independently ran `ffprobe -count_frames` on that attachment and confirmed H.264, 1280×720, 30/1 fps, 5.000000 seconds, 150 frames and 75,276 bytes. The file was not copied into the repository. This probe verifies metadata for that supplied file only. V1 checks declared rational duration and range; the product does not read or hash media bytes, verify frame rate or synchronization, prove that the asset ID names that file, generate OTIO, or import/play footage in an editor. See the [execution record](n10-05-execution-record.md) for this evidence boundary.

## G4 verification status

Local verification on the implementation branch passed:

- Focused CLI tests: 7/7.
- Full workspace tests: debug and release each reported 309 passed, 0 failed, 3 ignored (the ignored FilmCraft case requires its pinned host).
- Formatting, Clippy with warnings denied, rustdoc with warnings denied, and `scripts/check_repo.py`: passed.
- N-07 helper regressions: 12 tests; story-graph plan and trace: 6 and 5 tests; FilmCraft rejection checks: 6 tests; frozen OpenFilm finite-v1 differential: 6 cases, all passed.
- The existing Rust video V1, OTIO and story-review CLI regressions passed in both workspace profiles. The optional official Python OTIO parser is not installed in this local environment; the exact-head hosted Quality and Contracts job installed the pinned parser and passed as part of run #376.

Windows initially exposed a fixture line-ending mismatch after the CLI output was normalized; the tests now normalize both the CLI JSON line and expected fixture while preserving byte-identical same-host replays and input bytes. The final Windows job passed on the exact head.

Automated G4 verification and the bounded N10-05 evidence gate are complete. The human evidence is one maintainer/author self-session with prior exposure and self-reported timing, alongside a separate machine replay and synthetic-media probe. It is not an independent or randomized user study. No G5 release or publication is authorized by this work.
