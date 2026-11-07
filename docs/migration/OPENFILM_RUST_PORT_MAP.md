# OpenFilm → Storytelling：Rust 技術移轉對照表

日期：2026-11-07。此 48+16 inventory 主要是 **G1/G2 規劃對照**；Storytelling 已另完成有限 M1-01 visual snapshot importer，並不代表整份 capability map 已移植。

- OpenFilm 固定來源：`7add4762fe8174d76f43b863375ea39b73121e76`。涵蓋 16 個 packages 的責任邊界，另看 application、CLI/server、desktop/native 與測試入口。
- Storytelling current main checked 2026-11-07：`610ed49673d6f554e539008b34f5fc5479b9dbc1`。M0 PR #1, migration map PR #2, and finite read-only M1-01 PR #7 are merged; exact delivery heads and merge SHAs are tracked in [TASKS](../../TASKS.md).
- M0 只驗證宣告資料、ID、hash 形狀與有理數來源區間；M1-01僅新增有限 visual-snapshot 到 declared-plan 的唯讀映射。它不驗證被引用媒體的實際 bytes、不執行 planner、不產生OTIO，也不交接宿主。
- 本輪未修改 Rust 產品程式、未搬 OpenFilm 程式/測試/資產；未執行來源 TS 或新 Rust 測試。
- 來源索引/exports 與選定實作已讀取。這是模組級對照，不是每個函式均完成 correctness audit；Codex 必須在該 task 的 G1/G2 補讀實作與實際 tests。

## 怎麼讀

共 **48 項技術對照＋16 項新增補強 = 64 項**。原有 M0–M5 task IDs 保留；OF/GAP 是細分工作，不是另一條互相競爭的 roadmap。

| 類別 | 數量 | 定義 |
| --- | ---: | --- |
| native-core | 24 | 演算法/資料契約可原生 Rust，不呼叫 Node/Python/SQLite/FFmpeg/network |
| native-runtime | 10 | 檔案、交易、程序與owner控制可Rust實作；仍需作業系統/SQLite等依賴審查 |
| external-adapter | 8 | Rust控制介面，不把外部codec/model/宿主冒充純Rust |
| retain-host | 6 | 明確留在OpenFilm或宿主；不是要求Codex重寫的待辦 |

**先做 OF-01/02/03/04/29/34/35/36 的有限 M1 垂直切片，不是先做完34個native項目。**
每個 ready task 都必須完成 GAP-01/02/03/16 的對應門檻。`depends` 是行為/契約先後，不強迫先建完整service或大型crate。

來源連結規則：下方相對路徑均位於 [OpenFilm 固定快照](https://github.com/bynanci/open-film/tree/7add4762fe8174d76f43b863375ea39b73121e76)，不是未來浮動 main。
目標 Rust 路徑與測試名稱都是 **proposed**；不存在的路徑使用code格式，不代表已建立。OF-01/02只有M0局部基礎，仍不是完整移植。

配套：[補強與原生化邊界](NATIVE_BOUNDARIES_AND_GAPS.md) · [Codex執行規則](CODEX_PORTING_RUNBOOK.md) · [machine ledger](openfilm-port-index.json)。

## A｜可原生化的純核心：24 項
### OF-01｜領域模型、嚴格輸入與錯誤碼

**里程碑：** M1-01；**Rust狀態：** partial-M0-candidate；**前置：** GAP-01, GAP-02。

**OpenFilm來源：** `packages/core/src/models.ts`；`packages/core/src/validation.ts`；`packages/core/src/errors.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/wire.rs`；`crates/storytelling-core/src/model.rs`；`crates/storytelling-core/src/error.rs`。

**範圍／驗收：** 延用 M0 宣告驗證；新增 Evidence/Brief DTO 與 validated domain 分界。拒絕未知、重複、超限欄位；hash 格式合法不可稱為已驗證來源。

**新增回歸命名：** `wire_rejects_unknown_duplicate_and_oversize`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-02｜精確時間、影格與 source/sample clock

**里程碑：** M1-01；**Rust狀態：** partial-M0-candidate；**前置：** OF-01, GAP-03。

**OpenFilm來源：** `packages/core/src/timebase.ts`；`packages/core/src/sequence.ts`；`packages/core/src/sequence-waveform.ts`；`packages/core/src/audio-policy.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/time.rs`；`crates/storytelling-core/src/frame_time.rs`。

**範圍／驗收：** 延用 M0 RationalTime；分離來源、時間軸、影格與 sample。30000/1001 不等於 29.97；負值政策、半開邊界、溢位、任意 seek 與量化損失均須測試。PR #125 已在來源 main。

**新增回歸命名：** `fractional_rate_half_open_ranges_and_overflow`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-03｜Canonical JSON、語意 digest 與凍結快照

**里程碑：** M1-01；**Rust狀態：** proposed；**前置：** OF-01, GAP-03。

**OpenFilm來源：** `packages/core/src/preset-json.ts`；`packages/core/src/project-versions.ts`；`packages/core/src/pipeline.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/canonical.rs`；`crates/storytelling-core/src/identity.rs`。

**範圍／驗收：** 明定集合排序與語意陣列順序、Unicode、負零與數字政策；hash 綁定 schema/algorithm 版本。禁止直接以一般 serde_json 輸出當跨語言 canonicalization。

**新增回歸命名：** `rust_ts_digest_vectors_match`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-04｜舊專案與唯讀 OpenFilm snapshot importer

**里程碑：** M1-01；**Rust狀態：** proposed；**前置：** OF-01, OF-02, OF-03。

**OpenFilm來源：** `packages/core/src/migration.ts`；`packages/core/src/validation.ts`；`packages/core/src/models.ts`；`packages/application/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/openfilm/snapshot.rs`；`crates/storytelling-core/src/compatibility.rs`。

**範圍／驗收：** 只讀明確匯出的 fixture/snapshot，不開寫來源 Catalog；保留來源及 occurrence ID，記錄 float 轉有理數政策；未知 schema 拒絕，未知效果列 loss report，不直接套用整份舊 migration。

**新增回歸命名：** `snapshot_import_preserves_ids_and_reports_loss`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-05｜內容重複與感知 hash 比較

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-01, GAP-02。

**OpenFilm來源：** `packages/events/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-engine/src/duplicates.rs`。

**範圍／驗收：** 參考 perceptualDistance/findDuplicates。既有 hash 的比較可原生化，hash 影像生成另屬解碼 adapter。重複、近似、跨媒體型別與無效 hash 不混為一談；不自動刪素材。

**新增回歸命名：** `duplicate_groups_stable_and_unlike_hashes_rejected`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-06｜時間／地理事件分群

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-01, OF-07。

**OpenFilm來源：** `packages/events/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-engine/src/events.rs`。

**範圍／驗收：** 參考 clusterEvents；固定排序與 tie-break，保留 time gap/span/GPS 證據、日期缺漏與跨日界線。分群是候選，不推定人物或事件真實名稱；限制候選比較量。

**新增回歸命名：** `event_clusters_handle_missing_clock_and_dateline`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-07｜Metadata 時間判定與不確定性

**里程碑：** M1-01；**Rust狀態：** proposed；**前置：** OF-01, OF-02。

**OpenFilm來源：** `packages/metadata/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/capture_time.rs`；`crates/storytelling-engine/src/metadata_time.rs`。

**範圍／驗收：** 參考 resolveTimestamp/TimestampResolution；保留 originalValue/source/confidence/timezoneUncertain。未知時區不升格為已知 UTC；非法日期、偏移、DST 模糊值有明確政策。

**新增回歸命名：** `unzoned_timestamp_stays_uncertain`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-08｜旅程／GPX／時鐘校正證據

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-06, OF-07。

**OpenFilm來源：** `packages/core/src/travel.ts`；`packages/application/src/travel-evidence.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/travel.rs`；`crates/storytelling-engine/src/travel_alignment.rs`。

**範圍／驗收：** 把路徑與時間對位轉成有界純算法；匯入與檔案 I/O 另放 adapter。保留原時間與校正依據，GPS 預設不出公開紀錄；不因對位成功就聲稱地點已被人確認。

**新增回歸命名：** `clock_correction_preserves_original_and_privacy`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-09｜素材評分、排名與理由

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-05, OF-06。

**OpenFilm來源：** `packages/story/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-engine/src/scoring.rs`。

**範圍／驗收：** 參考 createAssetScorer/scoreAsset；評分分解、拒絕素材、人工選取、重複懲罰皆可追溯。跨平台排序不用 locale 相依比較；變更排序規則列 intentional delta，不能偷改結果。

**新增回歸命名：** `ranking_ties_have_frozen_order_and_reasons`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-10｜Story Arc、模板與段落意圖

**里程碑：** M2-01；**Rust狀態：** proposed；**前置：** OF-01, GAP-01。

**OpenFilm來源：** `packages/story/src/story-arcs.ts`；`packages/story/src/content.ts`；`packages/story/src/scene-plan.ts`；`templates`。

**Rust規劃落點：** `crates/storytelling-core/src/brief.rs`；`crates/storytelling-engine/src/story_arcs.rs`。

**範圍／驗收：** 移植可驗證模板規則與旁白/畫面意圖資料；文案與資產另核來源權利。範本值不是事實；無錄音只能給估時，不生成假 word timestamps。

**新增回歸命名：** `template_intent_does_not_invent_personal_facts`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-11｜Story Graph 語意、邊與引用

**里程碑：** M2-01；**Rust狀態：** proposed；**前置：** OF-01, OF-03。

**OpenFilm來源：** `packages/core/src/story-graph.ts`；`packages/story/src/story-graph-connections.ts`；`packages/application/src/story-graph.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/story_graph.rs`；`crates/storytelling-engine/src/graph_rules.rs`。

**範圍／驗收：** 保留 theme/beat/asset/idea、contains/supports/precedes/echoes/alternative 與 user-memory/planned/wish/unknown。畫布座標不改順序；移除圖引用不刪原素材；layout 演算法留 UI。

**新增回歸命名：** `precedes_cycle_rejected_layout_is_nonsemantic`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-12｜事件／人物覆蓋與媒體比例硬約束

**里程碑：** M2-01；**Rust狀態：** proposed；**前置：** OF-01, OF-11。

**OpenFilm來源：** `packages/core/src/models.ts`；`packages/core/src/validation.ts`；`packages/solver/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/constraints.rs`；`crates/storytelling-engine/src/constraints.rs`。

**範圍／驗收：** PR #121 已合併：人工分組的 minima/maxima、image/video source-count 與鎖定選取。不可行輸出具體衝突，不偷偷放寬、不做人臉身分推斷。

**新增回歸命名：** `infeasible_coverage_never_drops_pins`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-13｜時長預算、選材與粗剪 solver

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-02, OF-09, OF-10, OF-12, OF-15。

**OpenFilm來源：** `packages/solver/src/index.ts`；`packages/story/src/story-planner.ts`。

**Rust規劃落點：** `crates/storytelling-engine/src/planner.rs`；`crates/storytelling-engine/src/duration_budget.rs`。

**範圍／驗收：** 保留段落上下限、可用來源、先後與人工選取；輸出不可變提案，不建立第二個 live timeline。明定搜尋/節點預算；不承諾所有條件下全域最佳解。

**新增回歸命名：** `frozen_plan_is_deterministic_and_bounded`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-14｜StoryPlan 驗證與提案生成

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-03, OF-04, OF-13。

**OpenFilm來源：** `packages/core/src/story-plan.ts`；`packages/story/src/story-planner.ts`；`packages/application/src/story-plan.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/story_plan.rs`；`crates/storytelling-engine/src/proposal.rs`。

**範圍／驗收：** 來源與 story/graph revisions、選材理由、原始範圍、supported/unsupported 都綁定。Preview 純計算；adoption 必須改走 OF-25 與 GAP-06，不能移植為直接寫檔。

**新增回歸命名：** `proposal_preview_has_no_authoritative_mutation`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-15｜User Pins、鎖定與重新規劃政策

**里程碑：** M2-01；**Rust狀態：** proposed；**前置：** OF-01, OF-03。

**OpenFilm來源：** `packages/core/src/user-overrides.ts`；`packages/solver/src/editing.ts`；`packages/application/src/editor.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/overrides.rs`；`crates/storytelling-engine/src/edit_policy.rs`。

**範圍／驗收：** 直接命令與每一個 batch leaf 都走相同完整政策。人工解除 pin 與自動重新求解分開；失敗不可部分變更。宿主不支援 pins 時須揭露，不能宣稱宿主也強制執行。

**新增回歸命名：** `nested_batch_cannot_bypass_pin_or_lock`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-16｜連續性、節奏密度與情緒建議

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-14。

**OpenFilm來源：** `packages/core/src/continuity.ts`；`packages/application/src/continuity.ts`；`packages/story/src/scene-plan.ts`。

**Rust規劃落點：** `crates/storytelling-engine/src/continuity.rs`。

**範圍／驗收：** 診斷動作/時長/密度及有證據的情緒建議；缺景別或視線證據標人工檢查，不補造。建議與接受分開；接受後要有同一歷史/receipt。

**新增回歸命名：** `advice_cites_observed_evidence_not_guessed_gaze`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-17｜逐字稿編修與可靠 Ripple 投影

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-02, OF-03, OF-15。

**OpenFilm來源：** `packages/core/src/transcript-editing.ts`；`packages/core/src/transcript-ripple.ts`；`packages/application/src/transcript-editor.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/transcript.rs`；`crates/storytelling-engine/src/transcript_ripple.rs`。

**範圍／驗收：** 可靠/估算/人工時間分開；保留多次來源與字幕/圖層對位。提案綁 transcript revision、source identity、計畫版本；先改逐字稿再接受必須失效；不複製獨立編輯歷史。

**新增回歸命名：** `transcript_change_after_preview_invalidates_apply`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-18｜字幕與 source→timeline 匯出時鐘

**里程碑：** M1-02；**Rust狀態：** proposed；**前置：** OF-02, OF-04。

**OpenFilm來源：** `packages/core/src/captions.ts`；`packages/core/src/timing.ts`；`packages/application/src/captions.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/captions.rs`；`crates/storytelling-engine/src/caption_projection.rs`。

**範圍／驗收：** SRT/VTT millisecond 與影格量化分開；支援範圍內 trim/speed/reuse 的時間轉換有精確預期。文字安全、語系與無可見影格的 cue 不可靜默遺失；燒錄留宿主。

**新增回歸命名：** `caption_reuse_speed_and_fractional_edges`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-19｜節拍證據與剪點吸附

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-02, OF-03。

**OpenFilm來源：** `packages/core/src/rhythm.ts`；`packages/analysis/src/snapping.ts`；`packages/application/src/rhythm.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/rhythm.rs`；`crates/storytelling-engine/src/snapping.rs`。

**範圍／驗收：** 先原生化 evidence validation、時間投影、proposal；PCM onset/tempo producer 可分階段新增。Beat 不等於 downbeat/語意 phrase；來源與算法版本變動使 cache 失效。

**新增回歸命名：** `rhythm_mapping_rejects_changed_source_and_clock`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-20｜低音量活動／Silence 區間提案

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-02, OF-03, OF-15。

**OpenFilm來源：** `packages/core/src/audio-activity.ts`；`packages/media/src/audio-activity.ts`；`packages/application/src/silence.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/audio_activity.rs`；`crates/storytelling-engine/src/silence.rs`。

**範圍／驗收：** 可原生化已解碼 PCM 的 48 kHz/20 ms per-channel peak 統計與區間映射；demux/resample 留 OF-37。不是 VAD；非零來源起點與未支援 audioTiming 保留拒絕；不得自動剪輯。

**新增回歸命名：** `silence_pcm_antiphase_and_padding_projection`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-21｜音訊 source clock、gain/fade/ducking 映射

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-02, OF-15, OF-20。

**OpenFilm來源：** `packages/core/src/audio-policy.ts`；`packages/core/src/models.ts`；`packages/core/src/transcript-ripple.ts`；`packages/solver/src/audio-timing.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/audio_clock.rs`；`crates/storytelling-engine/src/audio_mapping.rs`。

**範圍／驗收：** 參考已合併 PR #114 的版本化 48 kHz clock map；先移植純映射與保護，實際 DSP 留宿主。fractional sample/J-L/後續 speed 等未證明情境拒絕；重複切割不產生重複 ID。

**新增回歸命名：** `ripple_envelope_matches_retained_source_samples`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-22｜A-roll/B-roll 與主旁白覆蓋

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-02, OF-15, OF-21。

**OpenFilm來源：** `packages/core/src/coverage.ts`；`packages/core/src/audio-policy.ts`；`packages/application/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/coverage.rs`；`crates/storytelling-engine/src/coverage_plan.rs`。

**範圍／驗收：** 把 coverage roles、主旁白與明確額外混音政策轉為計畫；宿主必須回報是否可表達。避免雙重混音；不支援時列 loss，不擅自把保留旁白變成全軌靜音。

**新增回歸命名：** `broll_preserves_primary_audio_without_double_mix`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-23｜資料型 Presets 與不可變接受快照

**里程碑：** M2-01；**Rust狀態：** proposed；**前置：** OF-01, OF-03。

**OpenFilm來源：** `packages/core/src/presets.ts`；`packages/core/src/preset-json.ts`；`packages/story/src/preset-story-plan.ts`；`packages/application/src/preset-integrity.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/presets.rs`；`crates/storytelling-engine/src/preset_plan.rs`。

**範圍／驗收：** 只允許資料型 recipe、來源與 schema；接受後快照不隨 registry 修改。未知字段、任意腳本/路徑/URL 拒絕；不引入執行外掛。

**新增回歸命名：** `preset_snapshot_tamper_and_executable_fields_rejected`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-24｜Geometry／presentation 的可攜描述

**里程碑：** M1-02；**Rust狀態：** proposed；**前置：** OF-02, OF-04。

**OpenFilm來源：** `packages/core/src/geometry.ts`；`packages/core/src/presentation.ts`；`packages/core/src/photo-easing.ts`；`packages/core/src/title-layout.ts`；`packages/core/src/primary-color.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/presentation.rs`；`crates/storytelling-engine/src/capability_projection.rs`。

**範圍／驗收：** 可移植 bounded 座標、aspect/crop 與效果描述；第一個 handoff 不承諾 renderer 等價。字型 shaping、調色/GPU、2.5D 實作留宿主；無 capability 不能假裝 native 保留。

**新增回歸命名：** `visual_recipe_is_preserved_or_explicitly_unsupported`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

## B｜可原生化的 I/O／Runtime：10 項
### OF-25｜不可變 Review、接受與 durable receipt

**里程碑：** M3-01；**Rust狀態：** proposed；**前置：** OF-03, OF-14, OF-29, GAP-06。

**OpenFilm來源：** `packages/application/src/silence.ts`；`packages/application/src/silence-jobs.ts`；`packages/application/src/editor.ts`；`packages/application/src/scene-script-review.ts`。

**Rust規劃落點：** `crates/storytelling-workflow/src/review.rs`；`crates/storytelling-workflow/src/approval.rs`。

**範圍／驗收：** 移植 source/revision/review 防線，重新設計人類批准與host compiled plan 的綁定。重開後 replay 讀 receipt，不再 apply；既有 in-session Undo 不冒充跨程序歷史。

**新增回歸命名：** `accept_exact_plan_replay_after_ack_failure`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-26｜Durable Jobs、reservation 與 owner fencing

**里程碑：** M3-02；**Rust狀態：** proposed；**前置：** OF-03, OF-29, OF-30, OF-32。

**OpenFilm來源：** `packages/core/src/models.ts`；`packages/catalog/src/intelligence-store.ts`；`packages/application/src/intelligence.ts`；`packages/application/src/review-owner.ts`；`packages/application/src/silence-jobs.ts`。

**Rust規劃落點：** `crates/storytelling-workflow/src/jobs.rs`；`crates/storytelling-adapters/src/sqlite/jobs.rs`；`crates/storytelling-adapters/src/platform/owner.rs`。

**範圍／驗收：** queued reservation 只被勝出者啟動；progress/final updates 需 owner+attempt CAS。alive/unknown 保守處理；unknown 不因 TTL 過期就可搶占；PID reuse 不當死亡證據。

**新增回歸命名：** `live_unknown_owner_and_late_callback_fenced`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-27｜Pipeline 狀態、影響分析與重用

**里程碑：** M3-02；**Rust狀態：** proposed；**前置：** OF-03, OF-25, OF-26。

**OpenFilm來源：** `packages/core/src/pipeline.ts`；`packages/application/src/pipeline-orchestrator.ts`。

**Rust規劃落點：** `crates/storytelling-workflow/src/pipeline.rs`；`crates/storytelling-engine/src/impact.rs`。

**範圍／驗收：** 移植 execution status/freshness 分離與 bounded attempts/input digest。重用需 evidence/rights/版本皆有效；原實作中吞掉 persistence error 的模式不得照搬；read/poll 不啟動工作。

**新增回歸命名：** `reused_artifact_requires_current_inputs_and_receipt`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-28｜命名版本、創作變體、比較與還原

**里程碑：** M3-02；**Rust狀態：** proposed；**前置：** OF-03, OF-25, OF-30。

**OpenFilm來源：** `packages/core/src/project-versions.ts`；`packages/core/src/creative-variants.ts`；`packages/application/src/project-versions.ts`；`packages/application/src/creative-variants.ts`。

**Rust規劃落點：** `crates/storytelling-workflow/src/versions.rs`；`crates/storytelling-engine/src/plan_diff.rs`。

**範圍／驗收：** 只管理 storytelling plans/receipts；不替宿主恢復整個時間軸。保留 immutable snapshot、parent/lineage；還原後舊批准不可自動復活；OpenFilm Undo 留既有 editor。

**新增回歸命名：** `restore_keeps_authority_and_invalidates_old_receipts`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-29｜來源位元組驗證、快照與 TOCTOU

**里程碑：** M1-01；**Rust狀態：** proposed；**前置：** OF-01, OF-03, OF-34, GAP-10。

**OpenFilm來源：** `packages/media/src/verified-source.ts`；`packages/media/src/index.ts`；`packages/application/src/path-safety.ts`；`packages/application/src/silence.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/filesystem/verified_source.rs`。

**範圍／驗收：** 以受控快照或受驗證開啟 handle 供消費；記錄實際 SHA。hash 後換檔、symlink/reparse point、失聯磁碟、取消均不錯綁；路徑 metadata 不等於可信來源。

**新增回歸命名：** `replacement_between_hash_and_consume_is_fenced`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-30｜Catalog、索引、cursor 與自己資料庫

**里程碑：** M3-02；**Rust狀態：** proposed；**前置：** OF-01, OF-03, GAP-11。

**OpenFilm來源：** `packages/catalog/src/index.ts`；`packages/catalog/src/intelligence-store.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/sqlite/catalog.rs`；`crates/storytelling-adapters/src/sqlite/migrations.rs`。

**範圍／驗收：** 參考 keyset canonical timestamp、FTS5 literal substring/triggers，但優先自有 journal；不直接寫 OpenFilm SQLite。短詞/%/_/反斜線/CJK/NULL/restore 維持語意；FTS index 不污染邏輯 revision。

**新增回歸命名：** `cursor_fts_restore_and_read_snapshot_consistent`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-31｜多來源匯入、批次 sealing 與共同分析

**里程碑：** M3-02；**Rust狀態：** proposed；**前置：** OF-06, OF-29, OF-30。

**OpenFilm來源：** `packages/core/src/multi-source.ts`；`packages/application/src/source-discovery.ts`；`packages/application/src/source-batches.ts`；`packages/application/src/source-analysis.ts`；`packages/application/src/source-analysis-review.ts`。

**Rust規劃落點：** `crates/storytelling-workflow/src/ingest.rs`；`crates/storytelling-adapters/src/filesystem/discovery.rs`。

**範圍／驗收：** 固定來源聯集，區分 source folder 與 event；限流、排除、權限、可重試失敗與不可變run。Picker 本身留使用端；read-only import 不在背景修改原專案。

**新增回歸命名：** `sealed_source_union_not_mutated_by_late_import`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-32｜子程序、取消、輸出限制與回收

**里程碑：** M1-03；**Rust狀態：** proposed；**前置：** OF-01, GAP-10。

**OpenFilm來源：** `packages/media/src/index.ts`；`packages/application/src/cancellable-provider-result.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/process/runner.rs`。

**範圍／驗收：** 參考 runProcess；Rust Command/Tokio 無 shell interpolation，有 executable allowlist、timeout、stdout/stderr 上限。只終止自己程序樹；spawn_blocking 不能假裝可強制 abort。

**新增回歸命名：** `cancel_reaps_owned_child_and_drains_both_pipes`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-33｜Collect／移動素材／portable package

**里程碑：** M1-03；**Rust狀態：** proposed；**前置：** OF-29, OF-34, OF-36。

**OpenFilm來源：** `packages/core/src/collect-delivery.ts`；`packages/application/src/collect-media.ts`；`packages/application/src/collect-derived-media.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/filesystem/collect.rs`；`crates/storytelling-core/src/lineage.rs`。

**範圍／驗收：** 保留來源與衍生 lineage、相對路徑、授權聲明與hash；中斷不可發布半包。排除私有cache/owner/credential；寫在獨立staging，不更改來源；字型與模型不自動包入。

**新增回歸命名：** `package_is_complete_relative_and_hash_verified`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-34｜既有 Rust runtime 安全 helper 抽離

**里程碑：** M1-01；**Rust狀態：** proposed；**前置：** OF-01, GAP-01。

**OpenFilm來源：** `apps/desktop/src-tauri/src/native_runtime.rs`；`apps/desktop/src-tauri/src/native_benchmark.rs`。

**Rust規劃落點：** `crates/storytelling-adapters/src/filesystem/manifest.rs`；`crates/storytelling-adapters/src/platform/resource_probe.rs`。

**範圍／驗收：** native_runtime.rs 的 hash/safe_path/verified_manifest 可評估 extraction；先補行為及race測試並保留Apache來源。不可整包帶入 Tauri/GTK/Node launcher；原有路徑檢查不等於已消除 TOCTOU。

**新增回歸命名：** `manifest_digest_path_traversal_and_cancel`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

## C｜Rust Adapter：8 項
### OF-35｜有限 OTIO writer／reader oracle

**里程碑：** M1-02；**Rust狀態：** proposed；**前置：** OF-02, OF-04, OF-36。

**OpenFilm來源：** `packages/exporters/src/index.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/otio/writer.rs`；`crates/storytelling-adapters/src/otio/profile.rs`。

**範圍／驗收：** Rust writer 只做批准 profile；ExternalReference/Gap/Clip/TimeRange 精確且可重現。官方 OTIO parser 與真實 FilmCraft 各自驗證；metadata 不等於宿主 native effect。

**新增回歸命名：** `official_parser_and_host_roundtrip_match`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-36｜Handoff compatibility／loss report／額外格式

**里程碑：** M1-02；**Rust狀態：** proposed；**前置：** OF-02, OF-04, GAP-05。

**OpenFilm來源：** `packages/exporters/src/index.ts`；`packages/application/src/nle-handoff.ts`；`packages/core/src/export-profile.ts`。

**Rust規劃落點：** `crates/storytelling-core/src/compiled_plan.rs`；`crates/storytelling-engine/src/host_compile.rs`；`crates/storytelling-adapters/src/interchange/profiles.rs`。

**範圍／驗收：** 參考 otioCompatibilityReport/NleHandoffService；native/baked/metadata-only/unsupported 分開。FCPXML/EDL 屬後續獨立profile，不算第二宿主驗證；不可靜默捨棄pins、速度或音訊automation。

**新增回歸命名：** `unsupported_edit_requires_explicit_loss_choice`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-37｜FFmpeg/FFprobe、波形、場景與PCM producer

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-19, OF-20, OF-29, OF-32。

**OpenFilm來源：** `packages/media/src/index.ts`；`packages/media/src/intelligence.ts`；`packages/media/src/audio-activity.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/media/probe.rs`；`crates/storytelling-adapters/src/media/pcm.rs`。

**範圍／驗收：** Rust 可原生管理程序與純統計，但解碼/探測/重採樣仍外部依賴。固定tool build與參數，actual PCM/PTS驗證；不宣稱已去除FFmpeg，也不重寫Codec。

**新增回歸命名：** `producer_identity_clock_limits_and_abort`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-38｜Whisper／逐字稿與 glossary provider

**里程碑：** M2-02；**Rust狀態：** proposed；**前置：** OF-17, OF-29, OF-32, OF-39。

**OpenFilm來源：** `packages/provider-whisper`；`packages/core/src/knowledge.ts`；`packages/application/src/knowledge.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/providers/transcription.rs`；`crates/storytelling-workflow/src/glossary_review.rs`。

**範圍／驗收：** 重用provider輸入輸出與字詞修正審閱邊界；模型runtime/weights另審查。Rust wrapper不等於原生語音模型，品質需要真人參考逐字稿；無依據時間不可升格。

**新增回歸命名：** `timestamps_provenance_consent_and_no_silent_correction`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-39｜Provider ports、同意與結果 admission

**里程碑：** M2-01；**Rust狀態：** proposed；**前置：** OF-01, OF-03, OF-29, OF-32, GAP-10。

**OpenFilm來源：** `packages/plugin-sdk`；`packages/analysis/src/index.ts`；`packages/application/src/audio-generation.ts`；`packages/application/src/motion-generation.ts`。

**Rust規劃落點：** `crates/storytelling-workflow/src/provider_ports.rs`；`crates/storytelling-adapters/src/providers/registry.rs`。

**範圍／驗收：** 參考 ProviderRegistry；註冊不等於啟用，endpoint/dataKinds/版本與同意綁定。language/vision/embedding/TTS/motion 先做契約，不自動下載模型、執行遠端或計費。

**新增回歸命名：** `provider_destination_change_revokes_consent`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-40｜Pixel／Insta360 與 metadata 專用來源

**里程碑：** M5-01；**Rust狀態：** proposed；**前置：** OF-07, OF-29, OF-37。

**OpenFilm來源：** `packages/source-pixel`；`packages/source-insta360`；`packages/metadata`。

**Rust規劃落點：** `crates/storytelling-adapters/src/sources/pixel.rs`；`crates/storytelling-adapters/src/sources/insta360.rs`。

**範圍／驗收：** 可移植來源辨識、配對/sidecar/provenance規則；ExifTool/原始格式解碼/360 stitching另列依賴。真機RAW/HDR/方向/時間測試缺少時標blocked，不推論可用。

**新增回歸命名：** `recognized_raw_does_not_claim_decode_support`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-41｜CLI/API client、命令契約與MCP adapter

**里程碑：** M4-01；**Rust狀態：** proposed；**前置：** OF-25, OF-26, GAP-06, GAP-14。

**OpenFilm來源：** `apps/cli/src/index.ts`；`apps/server/src/server.ts`；`apps/desktop/src/api.ts`。

**Rust規劃落點：** `crates/storytelling-cli/src/main.rs`；`crates/storytelling-adapters/src/transport/json.rs`；`bindings/typescript`。

**範圍／驗收：** 先CLI JSON/TS薄client，MCP後加；不把現有HTTP路由原樣複製一套業務。Agent只提案/預覽；approved execution走可信workflow，同request不同intent拒絕。

**新增回歸命名：** `all_transports_share_validation_and_authority`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-42｜Record／Audition／Visual QA 的驗證證據

**里程碑：** M1-03；**Rust狀態：** proposed；**前置：** OF-02, OF-29, OF-32, OF-35, OF-36, GAP-09。

**OpenFilm來源：** `packages/application/src/record-audio-preview.ts`；`packages/application/src/visual-qa.ts`；`packages/application/src/micro-preview.ts`；`packages/application/src/silence-jobs.ts`。

**Rust規劃落點：** `crates/storytelling-adapters/src/hosts/preview.rs`；`crates/storytelling-engine/src/verification.rs`。

**範圍／驗收：** 保留read-only preview、source/revision失效、typed unavailable與輸出sample/PTS指標。OpenFilm reference與host final output分開；跨renderer不要求任意逐像素同一，人工聽感另驗。

**新增回歸命名：** `host_success_with_offline_media_is_not_verified`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

## D｜保留宿主／明確不移植：6 項
### OF-43｜Vue工作區、Graph layout、快捷鍵與i18n UI

**里程碑：** consumer-only；**Rust狀態：** not-planned-for-core；**前置：** 不進核心開發排程。

**OpenFilm來源：** `apps/desktop/src/components`；`apps/desktop/src/api.ts`；`packages/story/src/story-graph-layout.ts`；`packages/ui`。

**Rust規劃落點：** `OpenFilm-consumer`。

**範圍／驗收：** 保留OpenFilm；只透過typed client接Rust。Graph/layout、DOM、焦點、快捷鍵與樣式不翻譯成SDK；故事內容locale可作metadata，UI翻譯仍在consumer。

**新增回歸命名：** `ui_does_not_create_second_authoritative_plan`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-44｜完整影片renderer／codecs／GPU compositor

**里程碑：** out-of-scope；**Rust狀態：** not-planned-for-core；**前置：** 不進核心開發排程。

**OpenFilm來源：** `packages/render`；`packages/media/src/index.ts`。

**Rust規劃落點：** `host-or-reference-renderer`。

**範圍／驗收：** 不移植整套FFmpegRenderer或造Rust Codec。現有renderer可作test oracle/preview/backend；Storytelling只輸出描述、capability與驗證結果。

**新增回歸命名：** `host_profile_never_claims_unimplemented_codec`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-45｜完整混音DSP／調色／GraphicScene／2.5D效果

**里程碑：** out-of-scope；**Rust狀態：** not-planned-for-core；**前置：** 不進核心開發排程。

**OpenFilm來源：** `packages/render`；`packages/core/src/graphic-scenes.ts`；`packages/core/src/looks.ts`；`packages/application/src/photo-presentation.ts`；`packages/application/src/primary-color-review.ts`。

**Rust規劃落點：** `host-finishing`。

**範圍／驗收：** 純資料/座標/時間描述可由OF-21/24保留，但DSP、字型排版、分層、特效執行留宿主。不要把metadata保留當成視覺/聲音等價。

**新增回歸命名：** `loss_report_covers_unmapped_effects`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-46｜Tauri桌面、Node runtime bundle與安裝器

**里程碑：** out-of-scope；**Rust狀態：** not-planned-for-core；**前置：** 不進核心開發排程。

**OpenFilm來源：** `apps/desktop/src-tauri/src/lib.rs`；`apps/desktop/src-tauri/src/main.rs`；`apps/server/src/native-launcher.ts`。

**Rust規劃落點：** `OpenFilm-distribution`。

**範圍／驗收：** 只抽OF-34的可分離helper；不搬整個桌面殼或其advisory closure。storytelling獨立CLI分發需GAP-13，不能用OpenFilm安裝CI代替。

**新增回歸命名：** `sdk_dependency_tree_excludes_tauri_node_launcher`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-47｜可執行外掛／市場與多租戶協作

**里程碑：** out-of-scope；**Rust狀態：** not-planned-for-core；**前置：** 不進核心開發排程。

**OpenFilm來源：** `packages/plugin-sdk`；`packages/application/src/local-preset-registry.ts`；`ROADMAP.md`。

**Rust規劃落點：** `deferred-product`。

**範圍／驗收：** 只保留OF-23資料preset與OF-39provider ports。任意程式外掛、marketplace、cloud collaboration不納入本次抽離，須新需求與安全設計。

**新增回歸命名：** `data_preset_cannot_execute_arbitrary_code`。先找到來源tests，保留授權與合成fixture來源；不得把這個規劃名稱寫成已存在或已通過。

### OF-48｜已合併視覺 keyframes 的宿主 capability / loss mapping

**里程碑：** M1-02 capability/loss mapping；**Rust狀態：** not-planned-for-core；**前置：** OF-35/36。

**OpenFilm來源：** `packages/core/src/sequence.ts`；`packages/core/src/clip-animation.ts`；`TASKS.md`。

**Rust規劃落點：** `crates/storytelling-engine/src/host_compile.rs`；`crates/storytelling-adapters/src/interchange/profiles.rs`。

**範圍／驗收：** OpenFilm PR #126 feature head `b4aba4c4bb70b8cb03edd54342f1b94cec649245` 已併入來源 main `5f737a6c8f35dce48e213370ff27900962c9304c`。Storytelling 按明確 host/profile 對視覺 keyframes 分類 native/baked/metadata-only/unsupported，並回報精確時間、easing、opacity 與 source mapping；不重做 OpenFilm evaluator/renderer。Speed ramps、compound editing、multicam維持延後。

此項不是 M1-01 已實作內容；需與 M1-02 OTIO/loss report 一起規劃，不得把未驗證宿主能力標為 native。

## 來源核對層級與範圍

已直接讀取：core/application/story/media/exporters/analysis 的入口、solver constraints、events分群/重複比較、metadata時間解析、native_runtime.rs的hash/path/manifest；也核對了package tree、Story tests inventory與來源PR #121/#125狀態。入口引用其他模組支持「定位到責任模組」，不證明本表每一個驗收案例已被上游滿足。

Codex要執行任何一項，先以 `git cat-file -e <sha>:<path>` 確認來源，再讀source與matching tests；若未找到或新main改名，更新manifest並回報，不猜函式或測試結果。明細可加symbols/line-ranges/blob hashes，但只填實際讀過的資料。

`packages/ui`/desktop/template等目錄級來源是責任邊界，不是整包複製授權。FFmpeg/第三方字型/模型/素材各有條款。原有Apache標記與NOTICE須保留；本次沒有把MIT改成Apache，也沒有授權將Apache內容改掛MIT。

原始來源索引建立時的 Draft/watch-only 觀察已由 M1-01 source recheck 校正：OpenFilm PR #126 已合併在來源 main `5f737a6c8f35dce48e213370ff27900962c9304c`。PR 描述或固定歷史索引中的舊狀態不能取代 GitHub merged metadata 與實際 source tree。
