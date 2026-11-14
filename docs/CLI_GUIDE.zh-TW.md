<!-- SPDX-License-Identifier: MIT -->
# 命令列使用指南

<a id="command-line-guide"></a>

[English](CLI_GUIDE.md) · 繁體中文

Storytelling 命令列工具（CLI）可獨立執行。它從檔案或標準輸入讀取資料，將結果寫至標準輸出。腳本、應用程式或代理程式可依退出碼處理結果。各指令接受的格式見[資料契約](CONTRACTS.zh-TW.md)。

若只有筆記、旁白構想或插畫主題，可用 Skill 起草。已經有結構化敘事、素材清單或剪輯需求時，可直接使用 CLI。[Storytelling Skill](../skills/storytelling/SKILL.zh-TW.md) · [平台指南](PLATFORMS.zh-TW.md)

<a id="build-the-executable"></a>

## 建置與依賴

安裝 Rust 工具鏈。在原始碼根目錄執行下列指令。`rust-toolchain.toml` 指定 Rust 1.89.0。`--locked` 使用已提交的依賴鎖定檔。

```sh
cargo build --release --locked -p storytelling-cli
cargo run --release --quiet --locked -p storytelling-cli -- --help
```

執行檔位於 `target/release/storytelling`。Windows 使用 `target/release/storytelling.exe`。其他專案可用絕對路徑呼叫執行檔，也可將其目錄加入 `PATH`。建置後可在本機使用，無須啟動 AI 服務或剪輯器。

將[計畫範例](../examples/minimal-plan.json)複製為 `plan.json`。若執行檔已在 `PATH`，執行下列指令。

```sh
storytelling validate plan.json
```

預期退出碼為 0，`validation.plan_id` 為 `travel-recap`。也可以將同一份 JSON 送入 `storytelling validate -` 的標準輸入。

<a id="choose-a-command"></a>

## 選擇入口

「快照」指符合 `finite-v1` 契約的剪輯組合匯出檔。OpenTimelineIO（OTIO）是時間軸交換格式。一般敘事或計畫驗證可使用各自的原生契約。

| 指令 | 輸入 | 結果 |
| --- | --- | --- |
| `render-narrative` | `NarrativeDocumentV1`，無須媒體或片長 | 依宣告順序輸出口述大綱與插畫構圖，附來源標籤。 |
| `render-video-coverage` | `NarrativeVideoCoverageV1` | 依宣告目錄與精確區間驗證指定變體的映射；保留 unresolved 並阻擋規劃就緒狀態，不開啟媒體。 |
| `render-handoff` | `NarrativeHandoffManifestV1` | 來源表示法、精確的部分影響與草稿審閱狀態，沿用 N-04 修訂簿。 |
| `track-narrative-revisions` | `NarrativeRevisionLedgerV1` 修訂簿 | 列出來源與變體的修訂歷史、作者決定和待審狀態。 |
| `query-narrative-impact` | 修訂簿與來源變更事件 ID | 列出直接引用該來源版本的變體。由引擎計算影響範圍。 |
| `check-narrative-style` | 敘事文件與綁定 revision 的風格 profile | 以 JSON 回報宣告的口述 beat 順序。 |
| `audit-copy-claims` | CommercialCopyDocumentV1 與 NarrativeRevisionLedgerV1，各至多 1 MiB | JSON 顯示文案、被引用的來源文字與 revision 狀態；不判斷語意佐證。 |
| `validate` | `PlanDocumentV1` 計畫 | 檢查宣告的 ID、雜湊格式、引用與精確來源區間。 |
| `audit-brief` | `StoryBriefV1` 剪輯需求 | 檢查必選、排除、順序及引用衝突。保留作者的手動選擇。 |
| `plan-story` | 剪輯需求、候選固定區間、宣告分數與硬性上限 | 提出可重現的固定區間提案，列出選擇原因。 |
| `import-openfilm` | 剪輯快照 | 輸出已宣告的畫面計畫、來源資料與限制報告。 |
| `export-otio` | 剪輯快照與必要的格式轉換選項 | 輸出原生 OTIO 時間軸。 |
| `verify-source` | 檔案或標準輸入的位元組、預期 SHA-256 | 核對本次提供的資料與預期雜湊值。 |
| `prepare-review` | 剪輯快照與審閱設定 | 建立同一輸入的匯入結果與 OTIO 識別紀錄。 |
| `check-review` | 審閱紀錄、目前快照與設定 | 核對目前宣告與紀錄，列出過期欄位。 |
| `prepare-story-review` | 完整規劃結果、相符的快照與設定 | 輸出含所選提案 OTIO 與審閱紀錄的 JSON。 |
| `export-story-otio` | 同上三項輸入 | 輸出同一份原生 OTIO。報告與提案存於中繼資料。 |
| `check-story-review` | 上次的完整審閱結果與目前三項輸入 | 檢查 OTIO，並重新編譯比對。結果過期時退出 1。 |

一般 JSON 輸入上限為 1 MiB。`verify-source` 資料上限為 16 MiB。提案審閱流程另有兩項上限。

- 提案最多 4,194,336 位元組。
- 完整審閱結果最多 4,292,672 位元組。

提案審閱流程只允許用 `-` 傳入提案。快照、設定與審閱結果須為一般檔案。使用標準輸入時，請傳送檔案結尾（EOF）。各契約另有限制集合、中繼資料、診斷及輸出大小。

具名輸入會在開檔前及開檔後分別確認一般檔案 metadata。靜態目錄、FIFO 與裝置會回傳不含路徑的 `input.io`，退出碼為 3；標準輸入仍須等待 EOF。這項檢查不能防止路徑在檢查後遭替換，也不限制 symlink 範圍。見[CLI 檔案輸入回歸與限制](validation/cli-file-input.zh-TW.md)。

<a id="render-a-spoken-outline-and-two-illustration-briefs"></a>

## 素材無關的口述與插畫提綱

下列修門案例含一份口述與兩種構圖。每份輸出都帶來源 ID、修訂版號（revision）及來源類別（origin）。創作道具紅傘 `C01` 只出現在寬景插畫。輸入無須媒體、素材路徑、片長或剪點。

目前範例為文件 revision 2，保留原有口述與寬景，將修正後的近景取景標為創作提案 `C02`。[請求者檢視及前後對照](validation/n03-narrative-cli.md#requester-inspection-and-corrective-draft--2026-11-11)記錄修改理由；原始 fixture 仍保留供回歸。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-narrative tests/fixtures/narrative-v1/n03-author-reviewed-v1.json
```

也可從標準輸入讀取同一份內容。

```sh
cat tests/fixtures/narrative-v1/n03-author-reviewed-v1.json | cargo run --release --quiet --locked -p storytelling-cli -- render-narrative -
```

成功時，標準輸出為多行 UTF-8 文字。用檔案或標準輸入重跑，輸出位元組相同。資料錯誤時，標準輸出為單行 JSON，退出碼為 1。用法錯誤為 2，輸入輸出錯誤為 3。指令只讀資料，不呼叫模型或剪輯器。[敘事契約](narrative/NARRATIVE_DOCUMENT_V1.md) · [N-03 命令列實測與待驗項目](validation/n03-narrative-cli.md)

## 驗證敘事到影片素材的覆蓋（N-10）

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-video-coverage tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

範例選取兩個單元：一個映射到已宣告的影片區間，另一個保留為 unresolved。
合法的缺件列退出碼仍是 0，但輸出 `ready_for_planning=false` 及
`video_coverage.unresolved_required`。完整映射可能回傳
`ready_for_planning=true`。無效宣告輸出一行 JSON 並退出 1；用法與輸入輸出錯誤
分別為 2 和 3。使用 `-` 可從標準輸入讀取，相同輸入會產生相同位元組。
指令只讀提供的 JSON；asset ID、revision 與片長是宣告，不會開啟或雜湊影片、檢查同步、產生 OTIO 或寫入剪輯器。
見[已接受契約](validation/n10-resume-gate.zh-TW.md)、[fixtures](../tests/fixtures/narrative-video-coverage-v1/README.md)
與[G3 驗證紀錄](validation/n10-video-coverage-cli.zh-TW.md)。

## 查核跨媒體交接（N-11）

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/before-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/after-v1.json
```

週六改週日後，邀請文、海報日期與兩版口述日期列為 `UPDATE`，
海報構圖與原始 WAV 列為 `REUSE`。`UPDATE` 表示歷史影響；
`status=needs_review` 或 `status=current` 表示草稿是否有相符審閱。
目前來源描述必須使用最新 revision；舊稿引用保留為歷史。

命令檢查引用、提案範圍與完整部分對照，不檢視原檔內容。
只有文字時可用 `descriptions-only-v1.json`，保留可用構圖並明列原檔未交付。
成功輸出可重播的 UTF-8 文字，退出碼 0；資料錯誤輸出一行 JSON，退出碼 1。
用法錯誤為 2，I/O 錯誤為 3。[規格](narrative/NARRATIVE_HANDOFF_MANIFEST_V1.zh-TW.md) · [實際證據](validation/n11-handoff.md)

<a id="track-revisions-and-query-their-impact"></a>

## 來源修訂與變體影響（N-04）

修訂簿記錄同一份敘事的版本變化。起始快照使用 `NarrativeDocumentV1`。獨立的 `NarrativeRevisionLedgerV1` 記錄來源修訂、建議、作者決定與各版輸出的引用。[修訂簿契約](narrative/NARRATIVE_REVISION_LEDGER_V1.md)

```sh
cargo run --release --quiet --locked -p storytelling-cli -- track-narrative-revisions tests/fixtures/narrative-v1/n04-revision-history-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json b-only-detail-change
```

| 查詢的來源變更 | 預期列出的變體 |
| --- | --- |
| `shared-observation-change` 修正兩位鄰居的分工 | `oral`、`illustration-wide`、`illustration-close` |
| `b-only-detail-change` 更正只用在近景構圖的細節 | `illustration-close` |

第二次更改後，口述與寬景構圖仍為 `current`。近景構圖為 `needs_review`，保留先前的 `C01@1` 引用。用歷史指令可取得先前的來源與變體快照。任一變體仍待審時，修訂簿會拒絕下一筆來源修訂。即使要修訂另一個來源，也須先處理待審變體。

助手的建議與作者的決定必須是不同事件。如果 `assistant_suggestion` 含 `decision`、`accepted` 或 `author_confirmed`，錯誤碼為 `revision.untrusted_decision`。工具只查核記錄的宣告，無法驗明作者身分。影響範圍依輸出的直接引用計算。`based_on` 的間接關係不會自動造成下游失效。[N-04 實測與待驗項目](validation/n04-narrative-revisions.md)

<a id="check-a-narrative-order-rule-n-08"></a>

## 檢查敘事順序規則（N-08）

工作坊範例固定活動資訊與攜帶雨傘的要求。直接版先說活動，再播敲擊；意象版先安排兩聲合成敲擊。獨立的 `NarrativeStyleProfileV1` 以文件 ID／revision 及口述變體／beat ID 綁定 `must_precede` 規則。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-v1.json
```

成功時輸出一行 JSON，只驗證宣告的順序。規則互相矛盾時回報 `narrative.style.order_conflict` 和規則 ID；profile 版本不支援時回報 `narrative.style.unsupported_version`。它不評價美感，也不代表真人核准。P1 是由 S3 延伸的意象提議，並非對敲擊來源的實際說明。盲讀回饋仍為 `manual-pending`。[Profile 契約（英文）](narrative/EDITORIAL_STYLE_PROFILE_V1.md) · [N-08 驗證紀錄（英文）](validation/n08-style-profile.md)

## 輸出錄音提示表（N-09）

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet \
  tests/fixtures/narrative-v1/n09-umbrella-source-v1.json \
  tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
```

指令讀取獨立且封閉的 cue sheet，並綁定指定的 `NarrativeDocumentV1` 與口述變體。旁白提示取自其引用的 beat，接著列出估時停頓與宣告的 `taps.wav` 音效。旁白長度未知時保持未解析；停頓估時與 WAV 已知長度不混為一談。相同互斥軌上已解析的疊音回報 `narrative.cue.track_conflict`；跨軌疊音及半開區間首尾相接則允許。輸出為可重播的可讀文字，不錄音、播放或混音。[Cue sheet 規格](narrative/NARRATIVE_CUE_SHEET_V1.zh-TW.md) · [N-09 重播證據](validation/n09-cue-sheet.zh-TW.md)

<a id="export-a-selected-proposal-as-otio"></a>

## 以來源修訂簿檢查文案主張（N-05）

預設指令保留 ReportV1。加上 `--report-v2` 才輸出下列增強資訊。

`audit-copy-claims` 以虛構產品的結構化輸入整理受眾、原始主張、引用、提議標語與 CTA，沿用 N-04 修訂簿。`current` 只表示來源 ID 與 revision 相符；事實主張一律保留 `support_status: human_review_required`。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-no-reference-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-dangling-ref-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-revised-v1.json --report-v2
```

「試行組平均整理時間由 10 分鐘降到 8 分鐘」與「所有團隊效率都提升 20%」都引用 `PILOT_LOG@1`，所以兩者的引用都為 current。但第二句擴大了受測群體並改稱整體效率，仍需作者判斷措辭依據。其他指令分別示範缺漏引用、不存在來源及試行紀錄修訂。來源修訂後，報告只將兩句列入 `reference_attention_claim_ids`，保留原措辭、歷史來源並附上目前來源文字。產品說明、提議標語與 CTA 不受影響。[契約](narrative/COMMERCIAL_COPY_V1.md) · [預期／實際輸出與空白作者審閱欄](validation/n05-commercial-copy.md)

## 將所選提案交付為 OTIO

從原始碼根目錄執行下列合成案例。`>` 指示執行環境將標準輸出存至檔案。請在可寫入的目錄使用不會覆蓋既有成果的檔名。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/story-review-v1/planner-request.json > proposal-A.json
cargo run --release --quiet --locked -p storytelling-cli -- prepare-story-review proposal-A.json tests/fixtures/openfilm/finite-v1/video-trim.json tests/fixtures/story-review-v1/context.json > review-A.json
cargo run --release --quiet --locked -p storytelling-cli -- export-story-otio proposal-A.json tests/fixtures/openfilm/finite-v1/video-trim.json tests/fixtures/story-review-v1/context.json > proposal-A.otio
cargo run --release --quiet --locked -p storytelling-cli -- check-story-review review-A.json proposal-A.json tests/fixtures/openfilm/finite-v1/video-trim.json tests/fixtures/story-review-v1/context.json
```

`review-A.json` 包含審閱紀錄與實際 OTIO。`proposal-A.otio` 與其中的 `otio` 欄位內容相同。標準輸出末尾的換行不計入產物雜湊。請保留時間軸的原始位元組。重新排版會改變比對結果。

提案剪點從 1/3 秒到 4/3 秒。原始快照的剪點從 0.1 秒到 2.6 秒。素材 ID、雜湊值與可用長度須與快照相符。工具只將素材 URI 當作字串。

若改動快照、提案剪點、順序或轉換選項，檢查會回報過期或拒絕輸入。退出碼為 1。OTIO 保留剪點、順序、精確時間與素材引用。專案幀率、來源宣告及已接受的鎖定設定只保存在中繼資料。影片須明確接受 `picture_only`。獨立音軌、效果及其他不支援的語意會遭拒。

這些操作只讀輸入。寫入剪輯器或取得作者批准需另行處理。[提案審閱契約](migration/PLANNER_OTIO_REVIEW_V1.md)

<a id="1-check-a-plan"></a>

## 1. 驗證已宣告的 plan

```sh
cargo run --release --quiet --locked -p storytelling-cli -- validate examples/minimal-plan.json
```

預期退出碼為 0，回應為 `{"ok":true,"validation":...}`。`validation.scope` 為 `declared-metadata-only`。輸入的 64 字元雜湊值屬於宣告。這項檢查不會開啟素材或計算內容雜湊。

另執行下列反例。預期退出碼為 1，錯誤碼為 `plan.source_out_of_bounds`。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
```

<a id="2-check-the-hard-constraints-in-a-brief"></a>

## 2. 稽核 Brief 的硬限制

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/valid.json
```

預期退出碼為 0，回應為 `{"ok":true,"audit":...}`。工具保留手動必選項、順序、硬限制的來源位置與偏好。結果含 `selection_planned:false` 與 `duration_analysis:"not-performed"`。此步驟只檢查宣告，尚未規劃鏡頭。

下例的格式有效，但需求互相衝突。預期退出碼為 1，回應含完整的 `audit`。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/conflicting.json
```

請讀取 `audit.conflicts`，再修正需求。工具會保留手動選擇。若 JSON 格式無效、有未知欄位或輸入超限，回應為 `{"ok":false,"error":...}`。[剪輯需求契約](migration/STORY_BRIEF_V1.md)

<a id="3-plan-fixed-shot-windows"></a>

## 3. 從 Brief 提出固定視窗 plan

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

預期退出碼為 0，回應為 `{"ok":true,"planning":...}`。`planning.scope` 為 `declared-fixed-window-plan-only`。結果含 `selection_planned:true` 與 `duration_analysis:"exact-fixed-windows"`。

提供原始剪輯需求與候選片段。每個候選項均須宣告雜湊值、來源時長、固定來源區間與整數分數。另提供時長上限 `max_duration` 與鏡頭數上限 `max_shots`。工具依宣告的資料規劃。來源媒體與評分品質需另行驗證。

範例要求 `detail` 排在 `opening` 之前。即使後者的分數與偏好較高，結果仍保留指定順序。預期時間精度為每秒 10 個刻度。總長為 `3/10` 秒，`target_status` 為 `met`。

| 鏡頭 | 素材 | 來源區間（秒） | 時間軸區間（秒） |
| --- | --- | --- | --- |
| `shot-000001` | `detail` | `[1/10,3/10)` | `[0,1/5)` |
| `shot-000002` | `opening` | `[0,1/10)` | `[1/5,3/10)` |

工具先保留必要項目，再依偏好、分數與 ASCII ID 排序。最後套用需求中的先後關係。工具會保留固定的來源區間。必選項超過時長或數量上限，或缺少候選時，工具會拒絕輸入。若未達軟性目標，結果會附上提案與目標狀態。這個狀態只適用於目前的固定區間規劃。各選擇的 `reason_paths` 指向原始宣告。

下例同時必選與排除 `opening`。預期退出碼為 1，錯誤碼為 `planner.brief_conflict`。衝突指向 `$.brief.manual_selected_asset_ids[1]`。結果不含計畫。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/conflicting.json
```

`planning.plan` 可直接傳給 `validate`。完整的規劃回應另有格式。`export-otio` 只接受剪輯快照。`plan-story` 的 `brief` 欄位只接受原始需求，不接受稽核結果。

目前範圍涵蓋固定區間與排序。多段敘事的配置、覆蓋率、鎖定及完整求解器仍在範圍外。`executable`、`source_verified` 與 `host_verified` 均為 `false`。[固定區間契約](migration/FIXED_WINDOW_PLANNER_V1.md)

<a id="4-import-a-composition-snapshot"></a>

## 4. 唯讀匯入剪輯快照

```sh
cargo run --release --quiet --locked -p storytelling-cli -- import-openfilm tests/fixtures/openfilm/finite-v1/still.json
```

預期退出碼為 0，回應為 `{"ok":true,"import":...}`。`import.plan` 符合 M0 計畫契約。呼叫端需要計畫時，請取出此欄位。`import.source_snapshot` 保留來源宣告與原始數字字串。[快照匯入格式與欄位](migration/FINITE_V1.md)

請提供符合契約的剪輯組合匯出檔。專案資料庫或需求文件各有其他格式。支援範圍是單一連續畫面軌的靜態圖片與影片硬切。未知效果、動畫與明確宣告的音軌均會遭拒。成功結果標為 `declared-visual-mapping-only` 與 `executable:false`。素材是否可用與音訊是否保留，需要對應證據。

<a id="5-export-a-timeline"></a>

## 5. 輸出有限 OTIO

```sh
cargo run --release --quiet --locked -p storytelling-cli -- export-otio tests/fixtures/openfilm/finite-v1/still.json --metadata-only-locks
```

範例宣告了鎖定，因此明確使用 `--metadata-only-locks`。預期退出碼為 0，標準輸出為 `Timeline.1` 原生 OTIO JSON。若要存檔，請指定新檔名。指令不會開啟素材 URI。

影片需使用 `--picture-only`。鎖定值為 true 時需使用 `--metadata-only-locks`。將選項放在輸入路徑後。缺少必要選項時，錯誤碼為 `otio.loss_choice_required`。這些選項指定可接受的格式轉換損失。指令只接受原始剪輯快照，不能直接讀取匯入或稽核結果。內容時間刻度與專案幀率分開記錄。[OTIO 契約](migration/FINITE_OTIO_V1.md)

<a id="6-compare-supplied-bytes"></a>

## 6. 比對明確提供的 bytes

下例使用原始碼庫的範例 JSON。預期雜湊值對應使用 LF 換行的原始檔。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- verify-source examples/minimal-plan.json 54e254ed9d62c604306305f95fc844d635f43bfc89cf835e05d4d74318cb6408
```

預期退出碼為 0，回應為 `{"ok":true,"verification":...}`。`scope` 為 `provided-buffer-bytes`。換行或內容有變動，雜湊值就可能不同。檔案內引用的媒體、使用權與來源身分須另行確認。

請從可信任的來源取得預期雜湊值。值的格式正確但內容不符時，退出碼為 1。錯誤碼為 `source.hash_mismatch`。CLI 不會輸出素材內容或不符的實際雜湊值。

Rust API 的成功結果會私下持有本次驗證的位元組。CLI 紀錄則只有中繼資料。紀錄無法還原成可信任的資料緩衝區。後續重新開檔時，仍需檢查檔案身分與驗證後遭替換的風險。[位元組驗證契約](migration/FILMCRAFT_25_V1.md#native-supplied-buffer-contract)

<a id="7-prepare-and-check-a-review-record"></a>

## 7. 建立與檢查有限審核識別

M3-01/P1 已接受同一輸入的審閱識別。此範圍涵蓋快照、審閱設定與編譯結果之間的關係。匯入的剪輯快照可用 `-` 從標準輸入讀取。審閱設定與紀錄須為具名檔案。

將下列內容存為 `review-context.json`。設定的結構版本為 1。請提供所有欄位，並保留指定欄位名稱。

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

用下列步驟建立紀錄。Python 指令會擷取 `review` 欄位，供下一行核對。請使用新的輸出檔名。

```sh
cargo run --release --quiet --locked -p storytelling-cli -- prepare-review tests/fixtures/openfilm/finite-v1/repeat-source.json review-context.json > prepared.json
python3 -c 'import json; print(json.dumps(json.load(open("prepared.json"))["review"]))' > review-record.json
cargo run --release --quiet --locked -p storytelling-cli -- check-review review-record.json tests/fixtures/openfilm/finite-v1/repeat-source.json review-context.json
```

`project_id`、`target_revision` 與宿主的 ID、版本、設定檔名稱均須使用 ASCII 識別字元。宿主是提供剪輯功能的外部應用程式。宿主 SHA-256 必須為 64 個小寫十六進位字元。能力名稱依 ASCII 升冪排列，最多 64 項。格式轉換選項也須依列舉名稱排序。兩份清單都須排除重複項目。

`prepare-review` 依快照位元組與審閱設定重新編譯。結果包含計畫、OTIO 及完整報告的識別雜湊。`scope` 為 `declared-review-binding-only`。權限與證據旗標均為 `false`。紀錄不含設定的明文，也不構成可還原的授權物件。

`check-review` 將紀錄視為未受信任的 JSON。它檢查結構、旗標、雜湊格式與紀錄綁定值，再用目前的輸入重新編譯。一致時回傳 `current:true`。有效但過期的紀錄回傳 `current:false` 與排序後的 `stale_fields`，退出碼仍為 0。過期欄位可包含 `composition`、`plan`、`artifact`、`report` 或 `context.project_id`。

| 輸入或輸出 | 上限 |
| --- | --- |
| 剪輯快照 | 1 MiB |
| 審閱設定 | 65,536 位元組 |
| 審閱紀錄 | 98,304 位元組 |
| OTIO 產物 | 4 MiB |
| 審閱 JSON 巢狀深度 | 64 層 |
| 識別字元 | 128 位元組 |

超限輸入會在解析或編譯前遭拒。錯誤使用一般 JSON 的 `error` 欄位。診斷不含本機路徑或輸入內容。

| 原因 | 可能的錯誤碼 |
| --- | --- |
| JSON 無效或版本不支援 | `review.invalid_context_json`、`review.invalid_record_json`、`review.unsupported_version` |
| 內容宣告或雜湊無效 | `review.invalid_claims`、`review.invalid_digest`、`review.record_digest_mismatch` |
| 結構、排序、能力或大小不符 | `review.json_too_deep`、`review.too_many`、`review.noncanonical_order`、`review.context_missing_capability`、`review.context_too_large`、`review.record_too_large` |

本次核對只驗證提供的剪輯快照與據此編譯的時間軸。宿主識別、執行檔雜湊、修訂版號與能力均由呼叫端宣告。任何人都可建立紀錄。SHA-256 無法證明紀錄的簽署者或授權狀態。此流程不會檢查宿主執行檔、呼叫剪輯器或驗證媒體。

若需核對不同規劃提案，請使用提案審閱指令。執行授權、金鑰信任、撤銷與防重播另有驗收要求。此處的有限範圍已有簽署驗收紀錄。[M3-01 審閱契約](CONTRACTS.zh-TW.md#m3-01-p1-accepted-bounded-review-binding) · [簽署驗收紀錄](migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md)

<a id="interpret-exit-codes-and-file-behavior"></a>

## 回應、退出碼與檔案處理

| 退出碼 | 呼叫端應採取的動作 |
| --- | --- |
| 0 | 讀取該指令的結果與 `scope`。若使用 `check-review`，另查 `current`。 |
| 1 | 查閱 `error` 或 `audit`。格式、輸入限制或規則衝突皆可能觸發此代碼。 |
| 2 | 檢查子命令及參數。 |
| 3 | 檢查輸入來源及標準輸出是否可用。 |

一般成功與錯誤回應均為單行 JSON。敘事指令成功時輸出可讀文字：`render-narrative`、`render-handoff`、`track-narrative-revisions`、`query-narrative-impact`。說明與版本指令也使用純文字。若標準輸出失敗，呼叫端可能只收到部分回應。

CLI 可讀取指定的本機檔案，也可解析指向一般檔案的符號連結。它不提供伺服器路徑管制或作業系統沙箱。路徑、檔名與內容都無法授予新權限。

目前沒有通用的批准、套用或媒體轉譯指令。需求檢查、位元組比對與 OTIO 解析各有驗證範圍。正式執行、復原及作者驗收仍需獨立證據。

<a id="replay-the-reference-and-format-tests"></a>

## 重播來源與官方格式驗證

執行 Rust 測試。

```sh
cargo test --workspace --locked
cargo test --workspace --release --locked
```

| 測試 | 重播條件 |
| --- | --- |
| 六個快照匯入案例 | 使用 Python 標準函式庫。[匯入測試](../tests/oracles/openfilm-finite-v1/README.md) |
| 八個來源與 OTIO 案例 | 使用指定的 Linux、CPython 3.12 與雜湊值固定的 OTIO 0.18.1 套件。[OTIO 測試](../tests/oracles/otio-finite-v1/README.md) |
| 十個硬限制案例 | 使用 Python 標準函式庫重播既存的基準輸出。[剪輯需求測試](../tests/oracles/story-brief-v1/README.md) |
| 十二個固定區間規劃案例 | 使用 Python 標準函式庫。測試保留視窗與排名政策的已知差異。[規劃測試](../tests/oracles/fixed-window-planner-v1/README.md) |

若要重新產生來源基準，才需要固定版本的 Node、tsx 與來源原始碼。請將輸出寫入獨立目錄。審查差異後，再更新測試資料。重播既存輸出只會比對保存的基準。正式 Rust 執行環境不依賴這些測試工具。

<a id="compare-two-planners-on-identical-input"></a>

## 執行 M2-03 即時規劃比對

工具使用固定版本的參考實作與 Rust CLI。兩者處理相同的合成案例。指定 Linux x86_64 的 Node、tsx 與來源原始碼路徑。選用尚不存在的外部輸出目錄。

```sh
python3 tests/oracles/planner-shadow-v1/run.py --source /absolute/reference-checkout --node /absolute/node --tsx /absolute/oracle-tools/node_modules/tsx/dist/loader.mjs --rust-cli /absolute/storytelling/target/release/storytelling --run-root /absolute/new-shadow-run
```

測試工具不會下載或安裝程式。預設執行 64 組案例，涵蓋已選定的一致範圍、四種已知差異與四個不支援案例。未分類的差異會導致測試失敗。這項測試會實際執行兩套規劃器。完整求解器、宿主整合與人工驗收各有獨立範圍。[工具版本、亂數種子、40 項回歸測試與證據格式](../tests/oracles/planner-shadow-v1/README.md)

<a id="reproduce-the-linux-editor-test"></a>

## 重現有界 FilmCraft 實跑

這個案例使用 FilmCraft v0.18.0 的 Linux x86_64 命令列工具。請先核對發行版本、壓縮檔與執行檔識別。測試工具會在執行前再次核對宿主的執行檔雜湊值。它不會自行安裝工具。[官方發行檔](https://github.com/storytold/filmcraft/releases/tag/v0.18.0) · [固定值與實測紀錄](migration/records/M1-03.md)

使用工具的實際絕對路徑。先建立父目錄。請選擇尚不存在的輸出目錄。

```sh
python3 tests/oracles/filmcraft-25-v1/run.py --storytelling /absolute/storytelling --filmcraft /absolute/filmcraft-cli --ffmpeg /absolute/ffmpeg --ffprobe /absolute/ffprobe --run-root /absolute/new-filmcraft-proof
```

測試將合成素材、原生專案、影片、日誌與 `evidence.json` 存入新目錄。這個目錄不提供作業系統層級的隔離。既有紀錄只代表當時執行的結果。

此案例限 Linux、無圖形介面、純畫面、每秒 25 格。原生序列為 1920×1080。CPU 輸出的 H.264 預覽為 192×108，共 45 格、1.8 秒。測試涵蓋實際匯入、原生區間、存檔重開、OTIO 再匯入、來源雜湊及逐格順序。

其他時鐘、媒體、音訊、GUI、GPU 與全解析度需另行實測。人工剪輯驗收與正式宿主執行器仍待驗證。[宿主測試說明](../tests/oracles/filmcraft-25-v1/README.md)

<a id="reproduce-recovery-after-a-lost-reply"></a>

## 重現 M3-03 遺失回應／回讀證明

這個測試只操作合成案例。請準備指定版本的 FilmCraft、FFmpeg 與 Cargo。請選擇新的絕對輸出路徑。先建置 CLI，再執行測試。

```sh
cargo build --release --locked -p storytelling-cli
python3 tests/oracles/m3-03-filmcraft-v1/run.py \
  --storytelling /absolute/storytelling \
  --filmcraft /absolute/filmcraft-cli \
  --ffmpeg /absolute/ffmpeg \
  --cargo /absolute/cargo \
  --run-root /absolute/new-m3-03-run
```

1. 工具建立合成來源與通過目前版本核對的提案。
2. 工具將建立空專案與一次匯入分別寫入 SQLite 日誌。
3. 宿主成功退出後，工具刻意保留回應。
4. 呼叫端先記為 `unknown`，再讀回原生專案與序列。
5. 剪點相符時，結果記為 `applied-observed`。過程不會重送或復原操作。

所有產物都存於新的輸出目錄。此案例限於每秒 25 格與單一畫面剪點。它不涵蓋人工批准、可信任的簽章授權或正式執行器。[測試說明](../tests/oracles/m3-03-filmcraft-v1/README.md) · [驗證紀錄](validation/m3-03-filmcraft-execution.md)

