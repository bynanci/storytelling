<!-- SPDX-License-Identifier: MIT -->
# Command-line guide

<a name="cli-使用指南"></a>
<a id="命令列使用指南"></a>

English · [繁體中文](CLI_GUIDE.zh-TW.md)

Use the Storytelling command-line interface (CLI) to process structured input locally. It reads files or standard input. A script can read the result from standard output and inspect the exit code. Each command has its own [input contract](CONTRACTS.md).

Start with the [Skill](../skills/storytelling/SKILL.md) for notes, narration or a drawing idea. Use the CLI when you have a narrative document, a list of assets or an editing brief. See the [platform guide](PLATFORMS.md) for installation.

<a id="建置與依賴"></a>

## Build the executable

Install the Rust toolchain. The repository pins Rust 1.89.0 in `rust-toolchain.toml`. Run the following command from the repository root. The `--locked` option uses the committed dependency lockfile.

```sh
cargo build --release --locked -p storytelling-cli
cargo run --release --quiet --locked -p storytelling-cli -- --help
```

The executable is `target/release/storytelling` or `target/release/storytelling.exe` on Windows. Call it by its absolute path or add its directory to `PATH`. The executable needs no AI service or editor.

Copy the [sample plan](../examples/minimal-plan.json) to `plan.json`. With the executable on `PATH`, run this command.

```sh
storytelling validate plan.json
```

Expect exit 0 and `validation.plan_id` equal to `travel-recap`. To read the same JSON from standard input, use `storytelling validate -`.

<a id="選擇入口"></a>

## Choose a command

A **snapshot** is a composition export that follows the `finite-v1` contract. OpenTimelineIO (OTIO) is a timeline interchange format. Narrative documents and plans each have a separate native format.

| Command | Input | Result |
| --- | --- | --- |
| `render-narrative` | `NarrativeDocumentV1` | A spoken outline and illustration briefs with source labels. No media or duration required. |
| `render-handoff` | `NarrativeHandoffManifestV1` | Source representations, exact component impact and draft review status, using the N-04 ledger. |
| `track-narrative-revisions` | `NarrativeRevisionLedgerV1` | Source and variant history, author decisions and review status. |
| `query-narrative-impact` | Ledger and source change event ID | Variants that directly reference this source revision. The engine computes the list. |
| `check-narrative-style` | Narrative document and revision-bound style profile | JSON result for declared spoken-beat order only. |
| `audit-copy-claims` | CommercialCopyDocumentV1 and NarrativeRevisionLedgerV1, each up to 1 MiB | JSON with proposed wording, cited source text and reference revision status; it does not judge semantic support. |
| `validate` | `PlanDocumentV1` | Checks declared IDs, hash format, references and exact source ranges. |
| `audit-brief` | `StoryBriefV1` | Checks required and excluded items, order and references. Preserves manual choices. |
| `plan-story` | Brief, fixed candidate windows, scores and limits | A reproducible proposal with reasons for each choice. |
| `import-openfilm` | Snapshot | A declared visual plan, source facts and limitations. |
| `export-otio` | Snapshot and required conversion options | A native OTIO timeline. |
| `verify-source` | File or standard input and expected SHA-256 | Compares the supplied bytes with the expected hash. |
| `prepare-review` | Snapshot and review context | A record that identifies the import and its compiled OTIO. |
| `check-review` | Record, current snapshot and context | Lists stale fields when the declarations differ. |
| `prepare-story-review` | Full planning response, matching snapshot and context | JSON with the selected cuts in OTIO and their review record. |
| `export-story-otio` | The same three inputs | The same OTIO. Its metadata contains the report and proposal. |
| `check-story-review` | Previous review bundle and current inputs | Checks the supplied OTIO. Recompiles the proposal. Returns exit 1 for a stale result. |

Most JSON inputs have a 1 MiB limit. The source verification limit is 16 MiB. Proposal review accepts up to 4,194,336 bytes for a proposal and 4,292,672 bytes for a review bundle. Only the proposal can use `-` for standard input in this workflow. Supply regular files for the snapshot, context and bundle. Close standard input to send end-of-file (EOF). Each contract sets limits for collections, metadata, diagnostics and output.

<a id="素材無關的口述與插畫提綱"></a>

## Render a spoken outline and two illustration briefs

The example about two neighbors and a gate contains a spoken outline, a wide composition and a close composition. Each output identifies its source, revision and origin. Only the wide composition contains the proposed red umbrella, `C01`. The input needs no media, path, duration or cut.

The current example is document revision 2. It keeps the original spoken and wide
drafts and marks the corrected close framing as creative proposal `C02`. The
[requester review and before/after record](validation/n03-narrative-cli.md#requester-inspection-and-corrective-draft--2026-11-11)
explain the change; the original fixture remains available for regression.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-narrative tests/fixtures/narrative-v1/n03-author-reviewed-v1.json
```

To use standard input, run the following command.

```sh
cat tests/fixtures/narrative-v1/n03-author-reviewed-v1.json | cargo run --release --quiet --locked -p storytelling-cli -- render-narrative -
```

Success produces multiline UTF-8 text. File and standard input produce identical bytes. Invalid data produces one JSON line on standard output and exit 1. Usage errors return 2. I/O errors return 3. The command only reads input. See the [narrative contract](narrative/NARRATIVE_DOCUMENT_V1.md) and [N-03 evidence](validation/n03-narrative-cli.md).

## Check a mixed-media handoff (N-11)

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/before-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/after-v1.json
```

The Saturday-to-Sunday change marks invitation, poster date and both spoken
dates as `UPDATE`. Poster layout and the original WAV are `REUSE`.
`UPDATE` records historical impact; `status=needs_review` or `status=current`
shows whether the draft has a matching review. Current source descriptors
must use the latest revision; old draft references remain visible as history.

The command checks references, proposal scope and complete component coverage.
It does not inspect file contents. For a text-only handoff, use
`descriptions-only-v1.json`; the report retains usable briefs and marks the
absence of original files. Success is reproducible UTF-8 text (exit 0);
invalid data is one JSON line (exit 1), usage errors use 2 and I/O errors use 3.
See the [profile](narrative/NARRATIVE_HANDOFF_MANIFEST_V1.md) and
[actual evidence](validation/n11-handoff.md).

<a id="來源修訂與變體影響n-04"></a>

## Track revisions and query their impact

The ledger starts with one `NarrativeDocumentV1` snapshot. A separate `NarrativeRevisionLedgerV1` records source revisions, suggestions, author decisions and references for each variant. See the [revision ledger contract](narrative/NARRATIVE_REVISION_LEDGER_V1.md).

```sh
cargo run --release --quiet --locked -p storytelling-cli -- track-narrative-revisions tests/fixtures/narrative-v1/n04-revision-history-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json b-only-detail-change
```

| Source change | Expected variants |
| --- | --- |
| `shared-observation-change` changes the neighbors' roles | `oral`, `illustration-wide`, `illustration-close` |
| `b-only-detail-change` changes a close composition detail | `illustration-close` |

After the second change, the oral and wide variants remain `current`. The close variant becomes `needs_review` and retains its prior reference to `C01@1`. The history includes earlier source and variant snapshots. Resolve all pending reviews before the next source revision, even when it changes a different source.

Keep author decisions in separate events. An `assistant_suggestion` with `decision`, `accepted` or `author_confirmed` produces `revision.untrusted_decision`. The tool checks declarations. It cannot authenticate the author. The engine follows direct references only. Authors must check indirect effects through `based_on`. See the [N-04 evidence](validation/n04-narrative-revisions.md).

<a id="check-a-narrative-order-rule-n-08"></a>

## Check a narrative order rule (N-08)

The workshop fixture keeps the event and requested umbrella fixed. The documentary opening states the event before the taps; the imagery opening puts the synthetic taps first. The separate `NarrativeStyleProfileV1` binds `must_precede` rules to a document ID/revision and spoken variant/beat IDs.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-v1.json
```

Success is one JSON line. The command validates only the declared order. A contradictory rule returns `narrative.style.order_conflict` and its rule IDs. An unsupported profile version returns `narrative.style.unsupported_version`. The profile does not assess aesthetics or establish human approval. P1 is an imagery proposal derived from S3, not an observed explanation of its cause. Reader feedback remains `manual-pending`. See the [profile contract](narrative/EDITORIAL_STYLE_PROFILE_V1.md) and [N-08 validation record](validation/n08-style-profile.md).

## Audit copy claims against a narrative ledger (N-05)

The default command retains ReportV1. Append `--report-v2` for the enhanced report below.

`audit-copy-claims <COPY> <LEDGER> --report-v2` organizes a supplied outline for a fictional product into audience, original factual wording, citations, a proposed slogan and a CTA. It reads the existing N-04 revision ledger. `current` means only that the source ID and revision match; all factual claims remain `support_status: human_review_required`.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-no-reference-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-dangling-ref-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-revised-v1.json --report-v2
```

Both “試行組平均整理時間由 10 分鐘降到 8 分鐘” and “所有團隊效率都提升 20%” cite `PILOT_LOG@1`. The references are both current; the second statement exceeds the trial record's population and measured outcome. The author must judge support. The other runs show a missing citation, nonexistent source and revised trial record. The revised report lists exactly two `reference_attention_claim_ids`, retains old wording and cited text, and displays the current source text alongside them. The product description, proposed slogan and CTA stay unchanged. [Contract](narrative/COMMERCIAL_COPY_V1.md) · [Expected/actual outputs and blank author review](validation/n05-commercial-copy.md)

## Render a recording cue sheet (N-09)

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet \
  tests/fixtures/narrative-v1/n09-umbrella-source-v1.json \
  tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
```

The command reads a separate closed cue sheet bound to the supplied `NarrativeDocumentV1` and spoken variant. It renders a narration cue from its referenced beat, followed by the estimated pause and the declared `taps.wav` effect. Unknown narration length remains unresolved; the pause estimate and known WAV duration stay distinct. Resolved overlapping intervals on one exclusive track return `narrative.cue.track_conflict`; overlaps across tracks and touching half-open intervals are allowed. Output is deterministic, human-readable text. It does not record, play, or mix audio. See the [cue-sheet profile](narrative/NARRATIVE_CUE_SHEET_V1.md) and [N-09 replay evidence](validation/n09-cue-sheet.md).


<a id="將所選提案交付為-otio"></a>
<a name="選定提案-a--可審核-otio已合併唯讀切片"></a>

## Export a selected proposal as OTIO

Run the synthetic example from the repository root. The shell operator `>` writes standard output to a file. Choose unused output names in a writable directory.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/story-review-v1/planner-request.json > proposal-A.json
cargo run --release --quiet --locked -p storytelling-cli -- prepare-story-review proposal-A.json tests/fixtures/openfilm/finite-v1/video-trim.json tests/fixtures/story-review-v1/context.json > review-A.json
cargo run --release --quiet --locked -p storytelling-cli -- export-story-otio proposal-A.json tests/fixtures/openfilm/finite-v1/video-trim.json tests/fixtures/story-review-v1/context.json > proposal-A.otio
cargo run --release --quiet --locked -p storytelling-cli -- check-story-review review-A.json proposal-A.json tests/fixtures/openfilm/finite-v1/video-trim.json tests/fixtures/story-review-v1/context.json
```

`review-A.json` contains the review record and the actual OTIO. The content of `proposal-A.otio` matches the `otio` field. The artifact hash excludes the final stdout newline. Preserve the original timeline bytes. Reformatting changes the comparison.

The selected cut spans 1/3 to 4/3 seconds. The snapshot cut spans 0.1 to 2.6 seconds. The asset ID, hash and available duration must match. The tool treats the asset URI as text.

A changed snapshot, cut, order or conversion choice can make the result stale or invalid. Either outcome returns exit 1. OTIO preserves cuts, order, exact time and asset references. Metadata holds the project frame rate, source declarations and accepted lock choices. For video, explicitly accept `picture_only`. The tool rejects separate audio tracks, effects and other unsupported features.

This workflow reads its inputs. Editor writes and author approval require separate steps. See the [proposal review contract](migration/PLANNER_OTIO_REVIEW_V1.md).

<a id="1-驗證已宣告的-plan"></a>

## 1. Check a plan

```sh
cargo run --release --quiet --locked -p storytelling-cli -- validate examples/minimal-plan.json
```

Expect exit 0 and `{"ok":true,"validation":...}`. The value of `validation.scope` is `declared-metadata-only`. The 64-character hash is a declaration. This command does not open the media or calculate its hash.

Run the invalid example. Expect exit 1 and `plan.source_out_of_bounds`.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
```

<a id="2-稽核-brief-的硬限制"></a>

## 2. Check the hard constraints in a brief

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/valid.json
```

Expect exit 0 and `{"ok":true,"audit":...}`. The audit preserves required items, order, constraint locations and preferences. Its report includes `selection_planned:false` and `duration_analysis:"not-performed"`. This command checks declarations without planning shots.

The next brief has valid syntax and conflicting requirements. Expect exit 1 and the full `audit` report.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/conflicting.json
```

Read `audit.conflicts` before you revise the brief. The tool preserves manual choices. Invalid JSON, unknown fields or oversized input produce `{"ok":false,"error":...}`. See the [brief contract](migration/STORY_BRIEF_V1.md).

<a id="3-從-brief-提出固定視窗-plan"></a>

## 3. Plan fixed shot windows

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

Expect exit 0 and `{"ok":true,"planning":...}`. The value of `planning.scope` is `declared-fixed-window-plan-only`. The response includes `selection_planned:true` and `duration_analysis:"exact-fixed-windows"`.

Supply the original brief and candidate clips. Each candidate needs a hash, duration, fixed source range and integer score. Supply limits for duration (`max_duration`) and shot count (`max_shots`). Planning uses these declarations. Media and score quality require separate verification.

The example requires `detail` before `opening`, even though `opening` has a higher score and preference. Expect 10 ticks per second, a total of `3/10` seconds and `target_status` equal to `met`.

| Shot | Asset | Source range in seconds | Timeline range in seconds |
| --- | --- | --- | --- |
| `shot-000001` | `detail` | `[1/10,3/10)` | `[0,1/5)` |
| `shot-000002` | `opening` | `[0,1/10)` | `[1/5,3/10)` |

The planner reserves required items first. It ranks candidates by preference, score and ASCII ID. It then applies the declared precedence constraints. Source windows remain fixed. A missing required candidate or an exceeded hard limit produces a rejection. An unmet soft target produces a proposal and a target status for these windows. Each `reason_paths` entry points to an input declaration.

The next example both requires and excludes `opening`. Expect exit 1 and `planner.brief_conflict`. The conflict points to `$.brief.manual_selected_asset_ids[1]`. The response contains no plan.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/conflicting.json
```

Pass `planning.plan` to `validate`. The full planning response uses a different format. `export-otio` accepts a snapshot. The planner's `brief` field accepts the original brief.

The planner selects and orders fixed windows. Multi-beat allocation, coverage, locks and general constraint solving remain outside this profile. The flags `executable`, `source_verified` and `host_verified` remain `false`. See the [fixed-window contract](migration/FIXED_WINDOW_PLANNER_V1.md).

<a id="4-唯讀匯入剪輯快照"></a>

## 4. Import a composition snapshot

```sh
cargo run --release --quiet --locked -p storytelling-cli -- import-openfilm tests/fixtures/openfilm/finite-v1/still.json
```

Expect exit 0 and `{"ok":true,"import":...}`. Extract `import.plan` for an M0 plan. The `import.source_snapshot` field retains source declarations and the original numeric strings. See the [snapshot format](migration/FINITE_V1.md).

Supply a composition export that follows this contract. The profile supports a single continuous picture track with still images and hard video cuts. It rejects unknown effects, animation and explicit audio tracks. A success reports `declared-visual-mapping-only` and `executable:false`. Media availability and audio preservation require separate evidence.

<a id="5-輸出有限-otio"></a>

## 5. Export a timeline

```sh
cargo run --release --quiet --locked -p storytelling-cli -- export-otio tests/fixtures/openfilm/finite-v1/still.json --metadata-only-locks
```

The sample declares locks, so it needs `--metadata-only-locks`. Expect exit 0 and native OTIO `Timeline.1` JSON. Choose a new output filename. The tool does not open the asset URI.

Video requires `--picture-only`. A true lock requires `--metadata-only-locks`. Place these options after the input path. A missing choice produces `otio.loss_choice_required`. These options describe accepted conversion losses. Supply the original snapshot. The exporter records content ticks separately from the project frame rate. See the [OTIO contract](migration/FINITE_OTIO_V1.md).

<a id="6-比對明確提供的-bytes"></a>

## 6. Compare supplied bytes

This example uses the repository's sample JSON. The expected hash corresponds to the original file with LF line endings.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- verify-source examples/minimal-plan.json 54e254ed9d62c604306305f95fc844d635f43bfc89cf835e05d4d74318cb6408
```

Expect exit 0 and `{"ok":true,"verification":...}`. The scope is `provided-buffer-bytes`. A change in content or line endings can change the hash. Media references, rights and source identity require separate checks.

Obtain the expected hash from a trusted source. A valid hash with a different value produces exit 1 and `source.hash_mismatch`. The CLI omits the input content and the actual mismatched hash.

On success, the Rust API privately owns the verified bytes. The CLI returns only metadata. Its record cannot reconstruct a trusted buffer. A later file read requires its own identity and replacement checks. See the [buffer contract](migration/FILMCRAFT_25_V1.md#native-supplied-buffer-contract).

<a id="7-建立與檢查有限審核識別"></a>

## 7. Prepare and check a review record

The accepted M3-01/P1 profile binds one snapshot, review context and compiled artifact. The snapshot can use standard input. Supply named files for the context and record.

Save the following complete version 1 context in `review-context.json`. Keep the field names exactly as shown.

```json
{
  "schema_version": 1,
  "project_id": "project-1",
  "target_revision": "new-isolated-project",
  "host": {
    "id": "filmcraft",
    "version": "0.18.0",
    "profile_id": "isolated25fps",
    "binary_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "declared_capabilities": [
      "arbitrary-content-tick-rate",
      "external-reference",
      "single-video-track",
      "still-hold"
    ]
  },
  "loss_choices": ["metadata_only_locks", "picture_only"]
}
```

Choose new output filenames. The Python command extracts the `review` field before the check.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- prepare-review tests/fixtures/openfilm/finite-v1/repeat-source.json review-context.json > prepared.json
python3 -c 'import json; print(json.dumps(json.load(open("prepared.json"))["review"]))' > review-record.json
cargo run --release --quiet --locked -p storytelling-cli -- check-review review-record.json tests/fixtures/openfilm/finite-v1/repeat-source.json review-context.json
```

Use ASCII identifiers for `project_id`, `target_revision` and the host ID, version and profile name. The host is the external editor application. The host SHA-256 needs 64 lowercase hexadecimal characters. Sort capability names in ascending ASCII order. Include at most 64. Sort conversion choices by enum name. Remove duplicates from both lists.

`prepare-review` compiles the snapshot bytes under the supplied context. It returns identifying hashes for the plan, OTIO and complete report. Its scope is `declared-review-binding-only`. Permission and evidence flags remain `false`. The portable record omits the context plaintext. It cannot grant authority.

`check-review` treats the record as untrusted JSON. It checks the shape, flags, hash format and binding digest. It recompiles the current input. A match returns `current:true`. A valid stale record returns exit 0 with `current:false` and sorted `stale_fields`. Stale fields can include `composition`, `plan`, `artifact`, `report` and `context.project_id`.

| Input or output | Limit |
| --- | --- |
| Composition snapshot | 1 MiB |
| Review context | 65,536 bytes |
| Review record | 98,304 bytes |
| OTIO artifact | 4 MiB |
| Review JSON nesting | 64 levels |
| Identifier | 128 bytes |

The tool rejects oversized input before parsing or compilation. The JSON `error` field contains the diagnostic. Diagnostics omit local paths and input content.

| Cause | Possible code |
| --- | --- |
| Invalid JSON or unsupported version | `review.invalid_context_json`, `review.invalid_record_json`, `review.unsupported_version` |
| Invalid claims or digest | `review.invalid_claims`, `review.invalid_digest`, `review.record_digest_mismatch` |
| Invalid structure, order, capability or size | `review.json_too_deep`, `review.too_many`, `review.noncanonical_order`, `review.context_missing_capability`, `review.context_too_large`, `review.record_too_large` |

The check covers the supplied snapshot and its compiled timeline. The caller declares host identity, binary hash, revision and capabilities. Anyone can construct a record. SHA-256 cannot identify a signer or grant permission. This process does not inspect host binaries, invoke an editor or check media.

For planner proposals, use the proposal review commands above. Execution authority, key trust, revocation and replay protection require separate verification. See the [review contract](CONTRACTS.md#m3-01-p1-accepted-bounded-review-binding) and [signed acceptance record](migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md).

<a id="回應退出碼與檔案處理"></a>

## Interpret exit codes and file behavior

| Exit | Caller action |
| --- | --- |
| 0 | Read the result and `scope`. For `check-review`, inspect `current`. |
| 1 | Inspect the diagnostic, audit, or freshness result. The reason may be invalid input, a conflict or a stale proposal review. |
| 2 | Check the command and arguments. |
| 3 | Check the input source and standard output. |

JSON responses use one line. The three narrative commands produce readable text on success. Help and version also produce plain text. A failed write to standard output can leave an incomplete response.

The CLI reads named local files. It can resolve a symbolic link to a regular file. It provides no server path policy or operating system sandbox. A path, name or file content cannot grant permission.

No general approval, apply or media render command exists. A brief audit, byte comparison and OTIO parse each check different properties. Production execution, recovery and editorial acceptance require separate evidence.

<a id="重播來源與官方格式驗證"></a>

## Replay the reference and format tests

Run the Rust tests.

```sh
cargo test --workspace --locked
cargo test --workspace --release --locked
```

| Test | Replay requirements |
| --- | --- |
| Six snapshot imports | Python standard library. [Importer tests](../tests/oracles/openfilm-finite-v1/README.md) |
| Eight source and OTIO cases | Pinned Linux, CPython 3.12 and hash-pinned OTIO 0.18.1. [OTIO tests](../tests/oracles/otio-finite-v1/README.md) |
| Ten hard-constraint cases | Python standard library and frozen reference outputs. [Brief tests](../tests/oracles/story-brief-v1/README.md) |
| Twelve fixed-window plans | Python standard library. Tests retain known window and ranking policy differences. [Planner tests](../tests/oracles/fixed-window-planner-v1/README.md) |

To regenerate the source baseline, supply the pinned Node, tsx and source checkout. Write to a separate directory. Inspect differences before changing fixtures. Routine replay compares the frozen results. Production Rust code does not depend on these test tools.

<a id="執行-m2-03-即時規劃比對"></a>

## Compare two planners on identical input

The M2-03 checker runs the pinned reference planner and Rust CLI on the same synthetic cases. Specify Linux x86_64 Node, tsx and source paths. Select an external output directory that does not yet exist.

```sh
python3 tests/oracles/planner-shadow-v1/run.py --source /absolute/reference-checkout --node /absolute/node --tsx /absolute/oracle-tools/node_modules/tsx/dist/loader.mjs --rust-cli /absolute/storytelling/target/release/storytelling --run-root /absolute/new-shadow-run
```

The checker installs and downloads nothing. Its default 64 pairs cover the selected agreement domain, four known differences and four unsupported cases. An unclassified difference fails the run. This exercise checks both running planners. A full solver, host integration and human acceptance need separate evidence. See [tool versions, seeds, 40 regression checks and evidence format](../tests/oracles/planner-shadow-v1/README.md).

<a id="重現有界-filmcraft-實跑"></a>

## Reproduce the Linux editor test

The M1-03 test uses FilmCraft v0.18.0 for Linux x86_64. Check the release, archive and executable identities. The runner checks the host executable hash before the test. It installs no tools. See the [official release](https://github.com/storytold/filmcraft/releases/tag/v0.18.0) and [pinned values and evidence](migration/records/M1-03.md).

Use absolute tool paths. Create the parent directory first. Choose an output directory that does not exist.

```sh
python3 tests/oracles/filmcraft-25-v1/run.py --storytelling /absolute/storytelling --filmcraft /absolute/filmcraft-cli --ffmpeg /absolute/ffmpeg --ffprobe /absolute/ffprobe --run-root /absolute/new-filmcraft-proof
```

The runner saves synthetic media, a native project, video, logs and `evidence.json` in the new directory. The directory provides no operating system isolation. A saved record describes its own run.

This headless test covers picture only at 25 fps. The native sequence is 1920×1080. CPU encoding produces a 192×108 H.264 preview with 45 frames over 1.8 seconds. The test checks import, native ranges, save and reopen, OTIO reimport, source hashes and frame order.

Other clocks, media, audio, GUI, GPU and full-resolution output need independent tests. Human editorial acceptance and the production executor remain pending. See the [host test guide](../tests/oracles/filmcraft-25-v1/README.md).

<a id="重現-m3-03-遺失回應回讀證明"></a>

## Reproduce recovery after a lost reply

The M3-03 test uses synthetic material. Supply the specified FilmCraft, FFmpeg and Cargo versions. Choose a new absolute output path. Build the CLI before running the proof.

```sh
cargo build --release --locked -p storytelling-cli
python3 tests/oracles/m3-03-filmcraft-v1/run.py \
  --storytelling /absolute/storytelling \
  --filmcraft /absolute/filmcraft-cli \
  --ffmpeg /absolute/ffmpeg \
  --cargo /absolute/cargo \
  --run-root /absolute/new-m3-03-run
```

1. The runner creates synthetic sources and a current proposal.
2. It records the empty project creation and one import in SQLite.
3. It withholds the reply after the host exits successfully.
4. The caller records `unknown` and reads back the native project and sequence.
5. Matching cuts yield `applied-observed`. The runner does not resend or undo the operation.

All artifacts stay in the new directory. The proof covers 25 fps and one picture cut. Human approval, trusted signer authorization and the production executor remain separate. See the [test guide](../tests/oracles/m3-03-filmcraft-v1/README.md) and [validation record](validation/m3-03-filmcraft-execution.md).

