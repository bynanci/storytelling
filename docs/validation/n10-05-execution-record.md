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

## Supplied media-probe report and its limits

The attached execution record reports that `ffprobe -v error -select_streams v:0 -count_frames ... clip-01.mp4` exited 0 and returned H.264, 1280×720, 30/1 fps, 5.000000 seconds, and 150 decoded frames. The record says the original media remains in an isolated environment. The media file was not among the attachments, so Codex did not rerun `ffprobe` or independently audit that output. No private media, path, hash, or identifying source text was added to this repository.

This is a maintainer-supplied report of a separate media-probe run. It does not show that `NarrativeVideoCoverageV1` read the file or verified that `CLIP-01` refers to it. The CLI only checked the supplied JSON declaration and rational range; it did not inspect media bytes, synchronization, OTIO, or an editor.

## N10-05 status

The output and error paths now have a post-merge CLI replay on the supplied inputs. The earlier VH-01 sessions remain maintainer-supplied and unaudited; the second session predates the CLI. A fresh author-operated CLI session, with author time and verbatim usability observations, remains pending. Keep #67 open until that human evidence is recorded and the maintainer resolves the remaining evidence gate. G5 publication remains separate.
