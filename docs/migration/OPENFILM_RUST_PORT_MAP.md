# Rust port inventory for editing integrations

<a name="剪輯整合的-rust-技術移植對照表"></a>
<a name="怎麼讀"></a>

[繁體中文](OPENFILM_RUST_PORT_MAP.zh-TW.md) · [Machine-readable inventory](openfilm-port-index.json)

This inventory records a November 7, 2026 study of editing integrations. It covers 48 upstream capabilities and 16 engineering gaps. These are subtasks of the existing M0 to M5 milestones. Read the [release and task history](../../TASKS.md) for accepted features. An inventory entry describes scope. It cannot establish completion.

The source baseline covers 16 packages and selected application, CLI, server, desktop and test entry points. All upstream paths refer to [commit 7add4762fe8174d76f43b863375ea39b73121e76](https://github.com/bynanci/open-film/tree/7add4762fe8174d76f43b863375ea39b73121e76). The original study read source files. It did not copy code or run new tests.

| Storytelling baseline | Evidence at that date |
| --- | --- |
| `551ff38920e2085e1d53a7ba777c9680678e9725` | PR #1 was a draft at `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`. |
| `9a5232e97435f543238b90a2006460f319f4e651` | PRs #1, #2, #7 and #16 merged. The source snapshot importer and finite OTIO writer were available. |
| `c03d4c8ede33563451b8ad590a6df982d0d8b82a` | PR #17 added checks for declared constraints. PR #18 tested a supplied byte buffer and one synthetic Linux CPU preview at 25 fps. |
| `895257b1bb9993d70cae3f1267f5f6fa3de91905` | PR #22 covered exact fixed windows, hard limits and declared ranks. Later work compared a subset with the running upstream implementation. [Planner record](records/M2-02.md) · [Comparison](records/M2-03.md) |

Use these four classes when selecting a task.

| Class | Count | Boundary |
| --- | ---: | --- |
| `native-core` | 24 | Pure Rust data contracts and computation without I/O. |
| `native-runtime` | 10 | Rust file, transaction and process control. Review OS and SQLite dependencies. |
| `external-adapter` | 8 | Rust controls a separate tool, model or host. |
| `retain-host` | 6 | The host retains the feature, including its interface or renderer. |

M1 selected parts of OF-01 to OF-04, OF-29, OF-34, OF-35 and OF-36. Check GAP-01, GAP-02, GAP-03 and GAP-16 before implementation. Dependencies describe behavior or contract order. The proposed paths and regression names below identify design targets. Check each task record for files that exist and tests that ran.

[Engineering gaps](NATIVE_BOUNDARIES_AND_GAPS.md) · [Porting guide](CODEX_PORTING_RUNBOOK.md) · [Finite importer](records/M1-01.md) · [OTIO](records/M1-02.md) · [Host test](records/M1-03.md) · [Brief audit](records/M2-01.md)


<a name="a可原生化的純核心24-項"></a>

## native-core

<a name="of-01領域模型嚴格輸入與錯誤碼"></a>

### OF-01 · Domain models and errors

Keep M0 declarations. Separate evidence and brief DTOs from checked domain types. Reject unknown or duplicate fields and oversized input. A hash string alone cannot establish source identity.

- Milestone: `M1-01`. Recorded status: `partial-finite-profiles-accepted`.
- Prerequisites: `GAP-01`, `GAP-02`.
- Upstream files: `packages/core/src/models.ts`, `packages/core/src/validation.ts`, `packages/core/src/errors.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/wire.rs`, `crates/storytelling-core/src/model.rs`, `crates/storytelling-core/src/error.rs`.
- Proposed regression: `wire_rejects_unknown_duplicate_and_oversize`.
- Recorded delivery: M0 strict wire and finite M1 contracts accepted. accepted bounded M2 brief audit separately traced M2-Bxx. complete Evidence/domain inventory remains planned.

<a name="of-02精確時間影格與-sourcesample-clock"></a>

### OF-02 · Time, frames and samples

Keep source time, timeline time, frames and samples separate. Treat 30000/1001 and 29.97 as different rates. Test negative values, half-open intervals, overflow, seeks and rounding. Upstream merged PR #125.

- Milestone: `M1-01`. Recorded status: `partial-finite-profiles-accepted`.
- Prerequisites: `OF-01`, `GAP-03`.
- Upstream files: `packages/core/src/timebase.ts`, `packages/core/src/sequence.ts`, `packages/core/src/sequence-waveform.ts`, `packages/core/src/audio-policy.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/time.rs`, `crates/storytelling-core/src/frame_time.rs`.
- Proposed regression: `fractional_rate_half_open_ranges_and_overflow`.
- Recorded delivery: Exact positive rational/decimal finite mapping and safe content clocks accepted. complete source/sample/seek/audio/quantization policy remains planned.

<a name="of-03canonical-json語意-digest-與凍結快照"></a>

### OF-03 · Canonical JSON and digests

Specify collection order, array order, Unicode, negative zero and numeric handling. Bind each digest to the schema and algorithm version. Ordinary JSON serialization cannot establish canonical bytes across languages.

- Milestone: `M1-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `GAP-03`.
- Upstream files: `packages/core/src/preset-json.ts`, `packages/core/src/project-versions.ts`, `packages/core/src/pipeline.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/canonical.rs`, `crates/storytelling-core/src/identity.rs`.
- Proposed regression: `rust_ts_digest_vectors_match`.

<a name="of-04舊專案與唯讀剪輯快照匯入"></a>

### OF-04 · Read-only snapshot import

Read an explicit export. Preserve source IDs and each occurrence ID. Specify the conversion from floats to rationals. Refuse unknown schemas and report unsupported effects. Keep the source catalog unchanged.

- Milestone: `M1-01`. Recorded status: `partial-accepted-finite-M1-01`.
- Prerequisites: `OF-01`, `OF-02`, `OF-03`.
- Upstream files: `packages/core/src/migration.ts`, `packages/core/src/validation.ts`, `packages/core/src/models.ts`, `packages/application/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/openfilm/snapshot.rs`, `crates/storytelling-core/src/compatibility.rs`.
- Proposed regression: `snapshot_import_preserves_ids_and_reports_loss`.
- Recorded delivery: Finite explicit composition export mapper accepted. no live Catalog or complete legacy project migration. OF-03 semantic digest deferred.

<a name="of-05內容重複與感知-hash-比較"></a>

### OF-05 · Duplicate and perceptual hashes

Compare supplied hashes with the behavior of perceptualDistance and findDuplicates. Decode images separately to produce hashes. Distinguish exact, similar, cross-type and invalid inputs. Never delete sources automatically.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `GAP-02`.
- Upstream files: `packages/events/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-engine/src/duplicates.rs`.
- Proposed regression: `duplicate_groups_stable_and_unlike_hashes_rejected`.

<a name="of-06時間地理事件分群"></a>

### OF-06 · Time and location clusters

Compare with clusterEvents. Bound comparisons. Retain stable order, tie breaks, time gaps, spans, GPS, missing dates and midnight boundaries. Candidate clusters cannot identify people or name an event.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-07`.
- Upstream files: `packages/events/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-engine/src/events.rs`.
- Proposed regression: `event_clusters_handle_missing_clock_and_dateline`.

<a name="of-07metadata-時間判定與不確定性"></a>

### OF-07 · Metadata timestamps

Preserve originalValue, source, confidence and timezoneUncertain from resolveTimestamp. Keep unknown offsets unknown. Specify behavior for invalid dates, offsets and ambiguous local times.

- Milestone: `M1-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-02`.
- Upstream files: `packages/metadata/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/capture_time.rs`, `crates/storytelling-engine/src/metadata_time.rs`.
- Proposed regression: `unzoned_timestamp_stays_uncertain`.

<a name="of-08旅程gpx時鐘校正證據"></a>

### OF-08 · Journey and clock alignment

Bound GPX and time alignment. Keep import and file access in adapters. Preserve the original time and correction evidence. Omit GPS from public records by default. Alignment cannot establish a location.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-06`, `OF-07`.
- Upstream files: `packages/core/src/travel.ts`, `packages/application/src/travel-evidence.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/travel.rs`, `crates/storytelling-engine/src/travel_alignment.rs`.
- Proposed regression: `clock_correction_preserves_original_and_privacy`.

<a name="of-09素材評分排名與理由"></a>

### OF-09 · Asset ranking

Preserve score components, rejection, manual selection and duplicate penalties from createAssetScorer and scoreAsset. Use ordering independent of locale. Document intentional changes to ranking.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-05`, `OF-06`.
- Upstream files: `packages/story/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-engine/src/scoring.rs`.
- Proposed regression: `ranking_ties_have_frozen_order_and_reasons`.

<a name="of-10story-arc模板與段落意圖"></a>

### OF-10 · Story arcs and section intent

Port checkable template rules and declared voice or picture intent. Check rights for text and assets separately. Treat template values as proposals. Without a recording, give estimated duration only.

- Milestone: `M2-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `GAP-01`.
- Upstream files: `packages/story/src/story-arcs.ts`, `packages/story/src/content.ts`, `packages/story/src/scene-plan.ts`, `templates`.
- Proposed Rust targets: `crates/storytelling-core/src/brief.rs`, `crates/storytelling-engine/src/story_arcs.rs`.
- Proposed regression: `template_intent_does_not_invent_personal_facts`.

<a name="of-11story-graph-語意邊與引用"></a>

### OF-11 · Story graphs and references

Preserve theme, beat, asset and idea nodes. Keep containment, support, order, echoes, alternatives and evidence status. Canvas coordinates cannot change sequence. Removing a reference must preserve the source. The client controls layout.

- Milestone: `M2-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-03`.
- Upstream files: `packages/core/src/story-graph.ts`, `packages/story/src/story-graph-connections.ts`, `packages/application/src/story-graph.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/story_graph.rs`, `crates/storytelling-engine/src/graph_rules.rs`.
- Proposed regression: `precedes_cycle_rejected_layout_is_nonsemantic`.

<a name="of-12事件人物覆蓋與媒體比例硬約束"></a>

### OF-12 · Group coverage and media counts

Preserve manual group minima and maxima, image and video counts, and locked choices from upstream PR #121. Report conflicts. Never weaken hard limits or infer identities from faces.

- Milestone: `M2-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-11`.
- Upstream files: `packages/core/src/models.ts`, `packages/core/src/validation.ts`, `packages/solver/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/constraints.rs`, `crates/storytelling-engine/src/constraints.rs`.
- Proposed regression: `infeasible_coverage_never_drops_pins`.

<a name="of-13時長預算選材與粗剪-solver"></a>

### OF-13 · Duration budgets and rough cuts

Preserve section limits, available sources, order and manual choices. Produce an immutable proposal. Bound search and node counts. Global optimality for arbitrary inputs is outside scope.

- Milestone: `M2-02`. Recorded status: `partial-fixed-window-accepted`.
- Prerequisites: `OF-02`, `OF-09`, `OF-10`, `OF-12`, `OF-15`.
- Upstream files: `packages/solver/src/index.ts`, `packages/story/src/story-planner.ts`.
- Proposed Rust targets: `crates/storytelling-engine/src/planner.rs`, `crates/storytelling-engine/src/duration_budget.rs`.
- Proposed regression: `frozen_plan_is_deterministic_and_bounded`.
- Recorded delivery: M2-P01-P06 finite declared fixed windows only. full multi-beat solver/proposals remain planned.

<a name="of-14storyplan-驗證與提案生成"></a>

### OF-14 · Plans and proposals

Bind proposals to source, story and graph revisions. Preserve selection reasons, ranges and support status. Calculate previews without writes. Route adoption through OF-25 and GAP-06.

- Milestone: `M2-02`. Recorded status: `partial-fixed-window-accepted`.
- Prerequisites: `OF-03`, `OF-04`, `OF-13`.
- Upstream files: `packages/core/src/story-plan.ts`, `packages/story/src/story-planner.ts`, `packages/application/src/story-plan.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/story_plan.rs`, `crates/storytelling-engine/src/proposal.rs`.
- Proposed regression: `proposal_preview_has_no_authoritative_mutation`.
- Recorded delivery: M2-P01-P06 finite declared fixed windows only. full multi-beat solver/proposals remain planned.

<a name="of-15user-pins鎖定與重新規劃政策"></a>

### OF-15 · Pins and replanning

Apply the same complete policy to direct commands and each batch item. Require separate author action to release a pin. On failure, preserve prior state. Disclose hosts that cannot enforce pins.

- Milestone: `M2-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-03`.
- Upstream files: `packages/core/src/user-overrides.ts`, `packages/solver/src/editing.ts`, `packages/application/src/editor.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/overrides.rs`, `crates/storytelling-engine/src/edit_policy.rs`.
- Proposed regression: `nested_batch_cannot_bypass_pin_or_lock`.

<a name="of-16連續性節奏密度與情緒建議"></a>

### OF-16 · Continuity, rhythm and emotion

Base advice on evidence of action, duration and density. Request manual review when shot scale or eyeline evidence is missing. Keep advice separate from acceptance. Store accepted changes with their receipts.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-14`.
- Upstream files: `packages/core/src/continuity.ts`, `packages/application/src/continuity.ts`, `packages/story/src/scene-plan.ts`.
- Proposed Rust targets: `crates/storytelling-engine/src/continuity.rs`.
- Proposed regression: `advice_cites_observed_evidence_not_guessed_gaze`.

<a name="of-17逐字稿編修與可靠-ripple-投影"></a>

### OF-17 · Transcript edits and ripple

Distinguish reliable, estimated and manual times. Preserve reused sources and subtitle alignment. Bind the proposal to the transcript revision, source identity and plan version. A transcript edit invalidates earlier proposals.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-03`, `OF-15`.
- Upstream files: `packages/core/src/transcript-editing.ts`, `packages/core/src/transcript-ripple.ts`, `packages/application/src/transcript-editor.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/transcript.rs`, `crates/storytelling-engine/src/transcript_ripple.rs`.
- Proposed regression: `transcript_change_after_preview_invalidates_apply`.

<a name="of-18字幕與-sourcetimeline-匯出時鐘"></a>

### OF-18 · Subtitle timing

Distinguish SRT/VTT milliseconds from frame rounding. Check trims, speed and reuse within the supported range. Report cues with no visible frame and issues with text or language. The host burns subtitles into pictures.

- Milestone: `M1-02`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-04`.
- Upstream files: `packages/core/src/captions.ts`, `packages/core/src/timing.ts`, `packages/application/src/captions.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/captions.rs`, `crates/storytelling-engine/src/caption_projection.rs`.
- Proposed regression: `caption_reuse_speed_and_fractional_edges`.

<a name="of-19節拍證據與剪點吸附"></a>

### OF-19 · Beats and cut alignment

Check beat evidence, project times and propose cuts first. PCM onset and tempo detection can follow. A beat alone cannot locate a downbeat or semantic phrase. Source or algorithm changes invalidate cached evidence.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-03`.
- Upstream files: `packages/core/src/rhythm.ts`, `packages/analysis/src/snapping.ts`, `packages/application/src/rhythm.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/rhythm.rs`, `crates/storytelling-engine/src/snapping.rs`.
- Proposed regression: `rhythm_mapping_rejects_changed_source_and_clock`.

<a name="of-20低音量活動silence-區間提案"></a>

### OF-20 · Low-level audio intervals

Measure per-channel peaks over 20 ms windows of decoded 48 kHz PCM. OF-37 handles decoding and resampling. Keep rejection of unsupported audioTiming and nonzero source starts. Neither infer speech activity nor cut automatically.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-03`, `OF-15`.
- Upstream files: `packages/core/src/audio-activity.ts`, `packages/media/src/audio-activity.ts`, `packages/application/src/silence.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/audio_activity.rs`, `crates/storytelling-engine/src/silence.rs`.
- Proposed regression: `silence_pcm_antiphase_and_padding_projection`.

<a name="of-21音訊-source-clockgainfadeducking-映射"></a>

### OF-21 · Audio clocks, gain and fades

Use the versioned 48 kHz map from upstream PR #114. Keep DSP in the host. Reject unproven fractional samples, J/L cuts and speed changes. Repeated splits need distinct IDs.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-15`, `OF-20`.
- Upstream files: `packages/core/src/audio-policy.ts`, `packages/core/src/models.ts`, `packages/core/src/transcript-ripple.ts`, `packages/solver/src/audio-timing.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/audio_clock.rs`, `crates/storytelling-engine/src/audio_mapping.rs`.
- Proposed regression: `ripple_envelope_matches_retained_source_samples`.

<a name="of-22a-rollb-roll-與主旁白覆蓋"></a>

### OF-22 · Narration coverage

Preserve main narration and explicit extra mixing choices. Require the host to report which roles it can represent. Prevent double mixing. Report unsupported choices without muting the entire narration track.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-15`, `OF-21`.
- Upstream files: `packages/core/src/coverage.ts`, `packages/core/src/audio-policy.ts`, `packages/application/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/coverage.rs`, `crates/storytelling-engine/src/coverage_plan.rs`.
- Proposed regression: `broll_preserves_primary_audio_without_double_mix`.

<a name="of-23資料型-presets-與不可變接受快照"></a>

### OF-23 · Data presets

Accept only a data recipe, source and schema. Freeze the accepted snapshot against later registry edits. Reject unknown fields, executable scripts, arbitrary paths and URLs.

- Milestone: `M2-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-03`.
- Upstream files: `packages/core/src/presets.ts`, `packages/core/src/preset-json.ts`, `packages/story/src/preset-story-plan.ts`, `packages/application/src/preset-integrity.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/presets.rs`, `crates/storytelling-engine/src/preset_plan.rs`.
- Proposed regression: `preset_snapshot_tamper_and_executable_fields_rejected`.

<a name="of-24geometrypresentation-的可攜描述"></a>

### OF-24 · Geometry and presentation

Preserve bounded coordinates, aspect ratio, crop and effect descriptions. Declare supported capabilities. Hosts handle font shaping, grading, GPU effects and 2.5D rendering. A first handoff cannot promise matching pixels.

- Milestone: `M1-02`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-04`.
- Upstream files: `packages/core/src/geometry.ts`, `packages/core/src/presentation.ts`, `packages/core/src/photo-easing.ts`, `packages/core/src/title-layout.ts`, `packages/core/src/primary-color.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/presentation.rs`, `crates/storytelling-engine/src/capability_projection.rs`.
- Proposed regression: `visual_recipe_is_preserved_or_explicitly_unsupported`.

<a name="b可原生化的-ioruntime10-項"></a>

## native-runtime

<a name="of-25不可變-review接受與-durable-receipt"></a>

### OF-25 · Review and durable receipts

Bind source, revision, review, human approval and the compiled plan. After reopening, read an existing receipt before considering execution. In-session undo cannot establish cross-process history.

- Milestone: `M3-01`. Recorded status: `proposed`.
- Prerequisites: `OF-03`, `OF-14`, `OF-29`, `GAP-06`.
- Upstream files: `packages/application/src/silence.ts`, `packages/application/src/silence-jobs.ts`, `packages/application/src/editor.ts`, `packages/application/src/scene-script-review.ts`.
- Proposed Rust targets: `crates/storytelling-workflow/src/review.rs`, `crates/storytelling-workflow/src/approval.rs`.
- Proposed regression: `accept_exact_plan_replay_after_ack_failure`.

<a name="of-26durable-jobsreservation-與-owner-fencing"></a>

### OF-26 · Jobs and ownership

Only the successful claimant starts a reservation. Compare owner and attempt on progress and final writes. A timeout or reused PID cannot prove death. Keep unknown operation ownership unresolved.

- Milestone: `M3-02`. Recorded status: `proposed`.
- Prerequisites: `OF-03`, `OF-29`, `OF-30`, `OF-32`.
- Upstream files: `packages/core/src/models.ts`, `packages/catalog/src/intelligence-store.ts`, `packages/application/src/intelligence.ts`, `packages/application/src/review-owner.ts`, `packages/application/src/silence-jobs.ts`.
- Proposed Rust targets: `crates/storytelling-workflow/src/jobs.rs`, `crates/storytelling-adapters/src/sqlite/jobs.rs`, `crates/storytelling-adapters/src/platform/owner.rs`.
- Proposed regression: `live_unknown_owner_and_late_callback_fenced`.

<a name="of-27pipeline-狀態影響分析與重用"></a>

### OF-27 · Status and reuse

Keep execution status separate from freshness. Bound attempts and bind input digests. Reuse needs current rights, evidence and versions. Surface persistence errors. A read or poll must never start work.

- Milestone: `M3-02`. Recorded status: `proposed`.
- Prerequisites: `OF-03`, `OF-25`, `OF-26`.
- Upstream files: `packages/core/src/pipeline.ts`, `packages/application/src/pipeline-orchestrator.ts`.
- Proposed Rust targets: `crates/storytelling-workflow/src/pipeline.rs`, `crates/storytelling-engine/src/impact.rs`.
- Proposed regression: `reused_artifact_requires_current_inputs_and_receipt`.

<a name="of-28命名版本創作變體比較與還原"></a>

### OF-28 · Versions and restore

Track plan and receipt history with immutable snapshots and parent links. Restoring a version cannot revive an old approval. The editor owns its timeline and undo.

- Milestone: `M3-02`. Recorded status: `proposed`.
- Prerequisites: `OF-03`, `OF-25`, `OF-30`.
- Upstream files: `packages/core/src/project-versions.ts`, `packages/core/src/creative-variants.ts`, `packages/application/src/project-versions.ts`, `packages/application/src/creative-variants.ts`.
- Proposed Rust targets: `crates/storytelling-workflow/src/versions.rs`, `crates/storytelling-engine/src/plan_diff.rs`.
- Proposed regression: `restore_keeps_authority_and_invalidates_old_receipts`.

<a name="of-29來源位元組驗證快照與-toctou"></a>

### OF-29 · Source bytes and replacement

Read a controlled snapshot or verified open handle and record the actual SHA. Check replacement after hashing, symlinks, reparse points, offline disks and cancellation. Path metadata alone cannot establish identity.

- Milestone: `M1-01`. Recorded status: `partial-supplied-buffer-verification`.
- Prerequisites: `OF-01`, `OF-03`, `OF-34`, `GAP-10`.
- Upstream files: `packages/media/src/verified-source.ts`, `packages/media/src/index.ts`, `packages/application/src/path-safety.ts`, `packages/application/src/silence.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/filesystem/verified_source.rs`.
- Proposed regression: `replacement_between_hash_and_consume_is_fenced`.

<a name="of-30catalog索引cursor-與自己資料庫"></a>

### OF-30 · Catalog and cursors

Use an independent journal. Preserve keyset timestamps and literal FTS5 search, including short strings, wildcard characters, escapes, CJK, NULL and restore. Search indexes must not change logical revisions.

- Milestone: `M3-02`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-03`, `GAP-11`.
- Upstream files: `packages/catalog/src/index.ts`, `packages/catalog/src/intelligence-store.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/sqlite/catalog.rs`, `crates/storytelling-adapters/src/sqlite/migrations.rs`.
- Proposed regression: `cursor_fts_restore_and_read_snapshot_consistent`.

<a name="of-31多來源匯入批次-sealing-與共同分析"></a>

### OF-31 · Batch source import

Freeze the input set for each run. Distinguish source directories from events. Bound concurrency and document exclusions, permissions and retryable failures. The client owns file selection. Read-only import must preserve the project.

- Milestone: `M3-02`. Recorded status: `proposed`.
- Prerequisites: `OF-06`, `OF-29`, `OF-30`.
- Upstream files: `packages/core/src/multi-source.ts`, `packages/application/src/source-discovery.ts`, `packages/application/src/source-batches.ts`, `packages/application/src/source-analysis.ts`, `packages/application/src/source-analysis-review.ts`.
- Proposed Rust targets: `crates/storytelling-workflow/src/ingest.rs`, `crates/storytelling-adapters/src/filesystem/discovery.rs`.
- Proposed regression: `sealed_source_union_not_mutated_by_late_import`.

<a name="of-32子程序取消輸出限制與回收"></a>

### OF-32 · Child processes

Start only permitted executables without shell interpolation. Bound runtime and both output streams. Terminate only the owned process tree. A blocking thread can continue after cancellation.

- Milestone: `M1-03`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `GAP-10`.
- Upstream files: `packages/media/src/index.ts`, `packages/application/src/cancellable-provider-result.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/process/runner.rs`.
- Proposed regression: `cancel_reaps_owned_child_and_drains_both_pipes`.

<a name="of-33collect移動素材portable-package"></a>

### OF-33 · Gather and move assets

Preserve lineage, relative paths, permissions and content hashes. Write into separate staging. Never publish an incomplete package or include private caches and credentials. Fonts and models require explicit treatment.

- Milestone: `M1-03`. Recorded status: `proposed`.
- Prerequisites: `OF-29`, `OF-34`, `OF-36`.
- Upstream files: `packages/core/src/collect-delivery.ts`, `packages/application/src/collect-media.ts`, `packages/application/src/collect-derived-media.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/filesystem/collect.rs`, `crates/storytelling-core/src/lineage.rs`.
- Proposed regression: `package_is_complete_relative_and_hash_verified`.

<a name="of-34既有-rust-runtime-安全-helper-抽離"></a>

### OF-34 · Native runtime helpers

Evaluate hash, safe_path and verified_manifest separately. Check behavior and file replacement before porting. Retain Apache notices. Leave the desktop shell and Node launcher in the host.

- Milestone: `M1-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `GAP-01`.
- Upstream files: `apps/desktop/src-tauri/src/native_runtime.rs`, `apps/desktop/src-tauri/src/native_benchmark.rs`.
- Proposed Rust targets: `crates/storytelling-adapters/src/filesystem/manifest.rs`, `crates/storytelling-adapters/src/platform/resource_probe.rs`.
- Proposed regression: `manifest_digest_path_traversal_and_cancel`.

<a name="crust-adapter8-項"></a>

## external-adapter

<a name="of-35有限-otio-writerreader-oracle"></a>

### OF-35 · OTIO writer and reader checks

Generate only the admitted format. Preserve exact ExternalReference, Gap, Clip and TimeRange values. Test with the official OTIO parser and a real editor separately. Metadata does not implement a host effect.

- Milestone: `M1-02`. Recorded status: `partial-finite-otio-profile-accepted`.
- Prerequisites: `OF-02`, `OF-04`, `OF-36`.
- Upstream files: `packages/exporters/src/index.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/otio/writer.rs`, `crates/storytelling-adapters/src/otio/profile.rs`.
- Proposed regression: `official_parser_and_host_roundtrip_match`.
- Recorded delivery: Native finite writer and official OTIO0.18.1 oracle accepted. FilmCraft host roundtrip separate M1-03. not full exporter/reader support.

<a name="of-36handoff-compatibilityloss-report額外格式"></a>

### OF-36 · Compatibility and loss reports

Distinguish native, baked, metadata-only and unsupported handling. Report loss of pins, speed or audio automation. Separate FCPXML and EDL formats require new profiles. An extra format cannot establish second-host support.

- Milestone: `M1-02`. Recorded status: `partial-finite-otio-profile-accepted`.
- Prerequisites: `OF-02`, `OF-04`, `GAP-05`.
- Upstream files: `packages/exporters/src/index.ts`, `packages/application/src/nle-handoff.ts`, `packages/core/src/export-profile.ts`.
- Proposed Rust targets: `crates/storytelling-core/src/compiled_plan.rs`, `crates/storytelling-engine/src/host_compile.rs`, `crates/storytelling-adapters/src/interchange/profiles.rs`.
- Proposed regression: `unsupported_edit_requires_explicit_loss_choice`.
- Recorded delivery: Explicit finite picture-only/metadata-only lock loss choices accepted. full host capability, audio, effects and additional formats remain planned.

<a name="of-37ffmpegffprobe波形場景與pcm-producer"></a>

### OF-37 · Media tools and PCM

Rust can supervise processes and compute statistics. External tools decode, probe and resample. Pin tool builds and arguments. Check actual PCM samples and presentation times.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-19`, `OF-20`, `OF-29`, `OF-32`.
- Upstream files: `packages/media/src/index.ts`, `packages/media/src/intelligence.ts`, `packages/media/src/audio-activity.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/media/probe.rs`, `crates/storytelling-adapters/src/media/pcm.rs`.
- Proposed regression: `producer_identity_clock_limits_and_abort`.

<a name="of-38whisper逐字稿與-glossary-provider"></a>

### OF-38 · Speech and glossaries

Preserve the provider contract and reviewed word corrections. Review model runtime and weights separately. Assess transcription against human reference text. Preserve uncertainty in word times.

- Milestone: `M2-02`. Recorded status: `proposed`.
- Prerequisites: `OF-17`, `OF-29`, `OF-32`, `OF-39`.
- Upstream files: `packages/provider-whisper`, `packages/core/src/knowledge.ts`, `packages/application/src/knowledge.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/providers/transcription.rs`, `crates/storytelling-workflow/src/glossary_review.rs`.
- Proposed regression: `timestamps_provenance_consent_and_no_silent_correction`.

<a name="of-39provider-ports同意與結果-admission"></a>

### OF-39 · Provider consent

Registration alone cannot enable a provider. Bind consent to endpoint, data kinds and version. Define contracts for language, vision, embeddings, speech and motion. Do not download models, call remote services or incur charges automatically.

- Milestone: `M2-01`. Recorded status: `proposed`.
- Prerequisites: `OF-01`, `OF-03`, `OF-29`, `OF-32`, `GAP-10`.
- Upstream files: `packages/plugin-sdk`, `packages/analysis/src/index.ts`, `packages/application/src/audio-generation.ts`, `packages/application/src/motion-generation.ts`.
- Proposed Rust targets: `crates/storytelling-workflow/src/provider_ports.rs`, `crates/storytelling-adapters/src/providers/registry.rs`.
- Proposed regression: `provider_destination_change_revokes_consent`.

<a name="of-40pixelinsta360-與-metadata-專用來源"></a>

### OF-40 · Device-specific sources

Preserve recognition, pairing, sidecar and source rules. Declare ExifTool, raw decoding and 360-degree stitching separately. Require real-device evidence for RAW, HDR, orientation and time claims.

- Milestone: `M5-01`. Recorded status: `proposed`.
- Prerequisites: `OF-07`, `OF-29`, `OF-37`.
- Upstream files: `packages/source-pixel`, `packages/source-insta360`, `packages/metadata`.
- Proposed Rust targets: `crates/storytelling-adapters/src/sources/pixel.rs`, `crates/storytelling-adapters/src/sources/insta360.rs`.
- Proposed regression: `recognized_raw_does_not_claim_decode_support`.

<a name="of-41cliapi-client命令契約與mcp-adapter"></a>

### OF-41 · CLI, client and MCP

Start with CLI JSON and a thin TypeScript client. Add MCP later. Share service logic. Agents can propose and preview. Trusted execution requires approval. Reject request reuse for a different intent.

- Milestone: `M4-01`. Recorded status: `proposed`.
- Prerequisites: `OF-25`, `OF-26`, `GAP-06`, `GAP-14`.
- Upstream files: `apps/cli/src/index.ts`, `apps/server/src/server.ts`, `apps/desktop/src/api.ts`.
- Proposed Rust targets: `crates/storytelling-cli/src/main.rs`, `crates/storytelling-adapters/src/transport/json.rs`, `bindings/typescript`.
- Proposed regression: `all_transports_share_validation_and_authority`.

<a name="of-42recordauditionvisual-qa-的驗證證據"></a>

### OF-42 · Recording and visual review

Preserve read-only previews, source revision invalidation, typed unavailable states, sample numbers and presentation times. Check reference previews and final output separately. Different renderers need not match pixel for pixel. Hearing requires a listener.

- Milestone: `M1-03`. Recorded status: `proposed`.
- Prerequisites: `OF-02`, `OF-29`, `OF-32`, `OF-35`, `OF-36`, `GAP-09`.
- Upstream files: `packages/application/src/record-audio-preview.ts`, `packages/application/src/visual-qa.ts`, `packages/application/src/micro-preview.ts`, `packages/application/src/silence-jobs.ts`.
- Proposed Rust targets: `crates/storytelling-adapters/src/hosts/preview.rs`, `crates/storytelling-engine/src/verification.rs`.
- Proposed regression: `host_success_with_offline_media_is_not_verified`.

<a name="d保留宿主明確不移植6-項"></a>

## retain-host

<a name="of-43vue工作區graph-layout快捷鍵與i18n-ui"></a>

### OF-43 · Workspace and layout

The client owns Vue, DOM, graph layout, focus, shortcuts, styles and interface translation. A typed client can pass story locale metadata to Rust.

- Milestone: `consumer-only`. Recorded status: `not-planned-for-core`.
- Prerequisites: None.
- Upstream files: `apps/desktop/src/components`, `apps/desktop/src/api.ts`, `packages/story/src/story-graph-layout.ts`, `packages/ui`.
- Owner: the upstream client.
- Proposed regression: `ui_does_not_create_second_authoritative_plan`.

<a name="of-44完整影片renderercodecsgpu-compositor"></a>

### OF-44 · Renderer and codecs

Keep the full renderer, codecs and GPU compositor in their execution environment. Use the existing renderer for reference or preview. Storytelling supplies descriptions, required capabilities and checks.

- Milestone: `out-of-scope`. Recorded status: `not-planned-for-core`.
- Prerequisites: None.
- Upstream files: `packages/render`, `packages/media/src/index.ts`.
- Proposed Rust targets: `host-or-reference-renderer`.
- Proposed regression: `host_profile_never_claims_unimplemented_codec`.

<a name="of-45完整混音dsp調色graphicscene25d效果"></a>

### OF-45 · Mixing, grading and effects

OF-21 and OF-24 can preserve data, coordinates and time. Hosts execute mixing, layout, compositing and effects. Keeping metadata cannot establish equal pictures or sound.

- Milestone: `out-of-scope`. Recorded status: `not-planned-for-core`.
- Prerequisites: None.
- Upstream files: `packages/render`, `packages/core/src/graphic-scenes.ts`, `packages/core/src/looks.ts`, `packages/application/src/photo-presentation.ts`, `packages/application/src/primary-color-review.ts`.
- Proposed Rust targets: `host-finishing`.
- Proposed regression: `loss_report_covers_unmapped_effects`.

<a name="of-46tauri桌面node-runtime-bundle與安裝器"></a>

### OF-46 · Desktop distribution

Consider only the independent helpers in OF-34. Keep desktop shells and their dependency reviews separate. The standalone CLI requires GAP-13 installation evidence from its own release candidate.

- Milestone: `out-of-scope`. Recorded status: `not-planned-for-core`.
- Prerequisites: None.
- Upstream files: `apps/desktop/src-tauri/src/lib.rs`, `apps/desktop/src-tauri/src/main.rs`, `apps/server/src/native-launcher.ts`.
- Owner: the upstream distribution tools.
- Proposed regression: `sdk_dependency_tree_excludes_tauri_node_launcher`.

<a name="of-47可執行外掛市場與多租戶協作"></a>

### OF-47 · Plugins and collaboration

Retain data presets under OF-23 and provider interfaces under OF-39. Arbitrary executable plugins, marketplaces and cloud collaboration require separate requirements and security designs.

- Milestone: `out-of-scope`. Recorded status: `not-planned-for-core`.
- Prerequisites: None.
- Upstream files: `packages/plugin-sdk`, `packages/application/src/local-preset-registry.ts`, `ROADMAP.md`.
- Proposed Rust targets: `deferred-product`.
- Proposed regression: `data_preset_cannot_execute_arbitrary_code`.

<a name="of-48已合併來源視覺-keyframes-的-capability--loss-邊界"></a>

### OF-48 · Visual keyframes

Upstream PR #126 merged feature head `b4aba4c4bb70b8cb03edd54342f1b94cec649245` as `5f737a6c8f35dce48e213370ff27900962c9304c`. Its parent is the pinned baseline. The finite importer and OTIO profile reject animation. Keep evaluation and rendering in the host. Report loss by profile. Speed ramps, compound clips and multicamera editing remain outside scope.

- Any future extension requires a defined host profile and explicit capability and loss decisions for OF-35 and OF-36. The Rust core has no planned animation evaluator.
- Upstream files: `packages/core/src/sequence.ts`, `TASKS.md`.
- Upstream file after PR #126: `packages/core/src/clip-animation.ts`. The baseline predates this file.
- Current Rust boundary: the finite importer and compiler reject animation. They cannot return an accepted mapped plan or OTIO document for that input.
- Existing regression: `unknown_effect_and_current_animation_reject_with_one_explicit_loss`. The planning ledger also names `merged_keyframes_remain_explicitly_unsupported_by_finite_profile` as a proposed check. [Source and policy](records/M1-01.md)


<a name="來源核對層級與範圍"></a>

## Source inspection

The study inspected core, application, story, media, exporter and analysis entry points. It covered solver constraints, event grouping, duplicate comparison and metadata timestamps.
It inspected hash, path and manifest handling in `native_runtime.rs`. It also checked the package tree, story test inventory, and upstream PRs #121 and #125.
PR #121 added coverage constraints. PR #125 added frame time. A module reference locates the owner of a behavior. Each acceptance case still needs its own test evidence.

Check the source with `git cat-file -e <sha>:<path>`. Read the matching tests. Update the manifest when a file moves or cannot be found.
Record only the symbols, line ranges and blob hashes that you inspected.

Directory entries for the UI, desktop and templates identify the responsible components. Review the separate terms for FFmpeg, third-party fonts, models and materials.
Keep applicable Apache notices. Check the [per-file provenance record](../../PROVENANCE.md) before importing code or tests.
