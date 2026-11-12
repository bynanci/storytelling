<!-- SPDX-License-Identifier: MIT -->
# N-10 / N-R10 post-merge execution record

English · [繁體中文](n10-05-execution-record.zh-TW.md)

**Issue:** [#67](https://github.com/bynanci/storytelling/issues/67) · **Implementation:** [PR #97](https://github.com/bynanci/storytelling/pull/97), merged as `169bf6baff33cd452238337237d437e15ac5b292`. **Execution source:** the `main` tree at [PR #98 merge](https://github.com/bynanci/storytelling/commit/5499f91c94cd12c6c82779978dc986843d403fe5), using Rust/Cargo 1.89.0 on Linux x86_64. The two attached JSON inputs were not copied into the repository.

## CLI replay on the supplied inputs

Codex ran the merged CLI against `vh01-valid-unresolved.json` and `vh01-invalid-range.json` from the supplied attachments. This is a machine replay of the author's declared inputs, **not an author-operated usability session**.

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage vh01-valid-unresolved.json
```

Exit 0; the CLI retained the declared missing unit and blocked planning:

```json
{"schema_version":1,"profile":"storytelling.narrative-video-coverage.v1","coverage_id":"vh01-two-unit-cut","narrative_ref":{"document_id":"vh01-gate-short","document_revision":2,"variant_id":"var-main"},"asset_catalog_ref":{"catalog_id":"cat-local-01","revision":1},"selected_unit_refs":[{"unit_id":"u-open","revision":2},{"unit_id":"u-test","revision":1}],"mappings":[{"unit_ref":{"unit_id":"u-open","revision":2},"status":"mapped","asset_ref":{"asset_id":"CLIP-01","revision":1},"source_range":{"start":{"numerator":"0","denominator":"1"},"end":{"numerator":"5","denominator":"1"}}},{"unit_ref":{"unit_id":"u-test","revision":1},"status":"unresolved","reason":"Missing close-up take of the door bottom clearing the ground."}],"ready_for_planning":false,"diagnostics":[{"code":"video_coverage.unresolved_required","path":"$.mappings[1]"}]}
```

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage vh01-invalid-range.json
```

Exit 1:

```json
{"error":{"code":"video_coverage.invalid_range","path":"$.mappings[0].source_range.end"}}
```

For both inputs, the file and stdin forms returned byte-identical stdout and exit states; the input files were unchanged. A one-shot direct-process measurement after compilation was 3.568 ms for the valid case and 1.839 ms for the invalid case, including process launch and input/output. These numbers describe this Codex Linux environment only; they are not author time or a performance benchmark. No participant time, reaction, or usability observation was collected during this replay.

The merged CLI regression suite was also rerun from this source tree:

```sh
cargo +1.89.0 test --locked -p storytelling-cli --test video_coverage
```

Result: **7 passed, 0 failed**.

## VH-01-S3: post-merge author-operated session

The maintainer's [#67 execution record](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001) adds a single-person session after the CLI became available. It is recorded separately from the Codex replay above.

| Item | VH-01-S3 record |
| --- | --- |
| Operator and time | VH-01, the project maintainer/author, on 2026-11-12 at 13:02 +08:00, after PRs #97 and #99 merged. This is a self-session with no external recruit or independent observer. |
| Prior exposure and order | VH-01 had previously used a spreadsheet in S1 and walked through the contract before the CLI existed in S2. This session was not randomized or blinded. |
| Prompt and starting data | Prepare a video handoff for two selected units: use the declared `CLIP-01` range for `u-open@2` and retain `u-test@1` as missing footage. The synthetic test clip is declared as five seconds long. |
| Actions and result | The author first entered an end range of `6/1`; the CLI returned exit 1, `video_coverage.invalid_range`, at `$.mappings[0].source_range.end`. After changing it to `5/1`, the rerun returned exit 0, `ready_for_planning: false`, and `video_coverage.unresolved_required` at `$.mappings[1]`. |
| Author time | 145 seconds, including checking the duration, editing JSON, both CLI runs and reviewing their outputs. This is total author task time, separate from Codex's single-process measurements. |
| Verbatim author statement | “實際用合併後的 `render-video-coverage` 跑這次雙單元交接，一開始我把 `CLIP-01` 的結束時間填成 `6/1` 秒，CLI 馬上以 exit `1` 指到 `$.mappings[0].source_range.end` 報 `video_coverage.invalid_range`；我對照 `ffprobe` 的 5 秒長度改成 `5/1` 秒重跑後就順利拿到 exit `0`，而且因為 `u-test@1` 還缺門底特寫，輸出明確標出 `ready_for_planning: false` 跟 `$.mappings[1]` 的 `video_coverage.unresolved_required`。不過手寫 JSON 時發現有理數的 `numerator` 跟 `denominator` 必須加引號寫成字串（像 `"5"`、`"1"`），如果直接寫整數 `5` 會觸發 `video_coverage.invalid_shape`。” |
| Observations and limits | The JSONPath helped the author locate the out-of-range field. Requiring JSON strings for rational fields was a friction point, and first-error-wins requires iterative fixes and reruns. The author also reports that spreadsheet S1 took 260 seconds and missed the range error. Prior exposure, fixed order and practice effects confound this comparison, so the times are descriptive and do not establish that the tool caused a reduction. The quote and author timing come from the maintainer-supplied account; Codex did not observe the session. A replay checks output, not the author's actions. |

The record supplies the prompt, starting data, verbatim statement, observations, and limitations/failure log. It is qualitative usability evidence from one maintainer-reported self-session, not an independent user study, randomized comparison or statistical sample. Earlier VH-01-S2 predates the CLI and remains a manual contract walkthrough, not a CLI execution.

## Read-only probe of the supplied synthetic MP4

The supplied `clip-01.mp4` is described as a synthetic test pattern generated with FFmpeg `testsrc`, not recorded footage. Codex ran `ffprobe -v error -select_streams v:0 -count_frames -show_entries stream=codec_name,width,height,r_frame_rate,duration,nb_read_frames:format=size,bit_rate -of json` directly on the attachment. It exited 0 and reported H.264, 1280×720, 30/1 fps, 5.000000 seconds, 150 frames and a file size of 75,276 bytes. Only these non-sensitive media metadata are recorded; the video, hash and local path were not added to the repository.

This `ffprobe` check establishes readable metadata for the supplied synthetic file. `NarrativeVideoCoverageV1` did not read or validate its media bytes and did not prove that JSON asset ID `CLIP-01` identifies this attachment. It does not check synchronization, generate OTIO or operate an editor.

## N10-05 status

With the maintainer-supplied VH-01-S3 record, the post-merge CLI replay and this synthetic-media probe, the bounded N10-05 acceptance evidence is complete. The source and evidence limits are preserved in the [#67 record comment](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001). This is not an external or randomized user study. G5 publication remains a separate maintainer decision.
