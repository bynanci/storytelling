# Storytelling 開發任務

[English](TASKS.md) · 繁體中文

想開始寫故事，可從[八種敘事樣板](skills/storytelling/references/multimodal-templates.zh-TW.md)選一種。想參與開發，先從下表找到對應工作。每張任務票都有需求編號及驗收條件。

| 我要做的事 | 從哪裡開始 |
| --- | --- |
| 理解作者的問題 | [設計依據](docs/WHY_STORYTELLING.zh-TW.md)與[開發路線圖](ROADMAP.zh-TW.md) |
| 開發或試用敘事功能 | [敘事任務](#cross-media-narrative-backlog)與[需求對照表](docs/REQUIREMENTS.zh-TW.md#proposed-cross-media-narrative-requirements) |
| 處理聲音、流派或多種素材 | [多模態任務](#multimodal-rust-backlog) |
| 整合剪輯工具或移轉既有功能 | [影片工程任務](#next-actionable-work)與[移轉指南](docs/MIGRATION.zh-TW.md) |

<a id="status-rules"></a>

## 如何判讀狀態

依序使用 `proposed → ready → in-progress → review → verified → accepted`。工作受阻時標記 `blocked`。寫明原因、負責人與恢復條件。勾選實作項目代表程式已存在。請分別記錄自動測試、真人評估及正式發布的結果。PR 須標明受測提交碼。[開發流程](docs/SDLC.zh-TW.md) · [驗證紀錄範例](docs/validation/m0-foundation.md)

<a id="skill-distribution"></a>

## 安裝與散布

- [x] **SK-D01**　在新專案中從 GitHub 安裝 Skill，或透過 npm 加為開發依賴。兩種方式提供相同的 Skill 檔案。[安裝指南](docs/PLATFORMS.zh-TW.md) · [SK-R01 驗收](docs/REQUIREMENTS.zh-TW.md#skill-distribution)

維護者另行決定是否發布至公開套件登錄站。

<a id="cross-media-narrative-backlog"></a>

## 跨媒體敘事任務

| 任務與需求 | 狀態與依賴 | 交付物與驗收方式 |
| --- | --- | --- |
| [N-01／#48](https://github.com/bynanci/storytelling/issues/48) · N-R01 作者試用 | G0–G1。試用包已備妥，可與 N-03 並行。 | 比較一般筆記與 Skill 的完成率、修改時間、歸因錯誤與重複輸入。真人試用及維護者決定仍待完成。 |
| [N-02／#49](https://github.com/bynanci/storytelling/issues/49) · N-R02 共用契約 | G2 已採納。參考 N-01 試用包。 | [ADR 0005](docs/adr/0005-independent-narrative-document-v1.md) 與 [V1 規格](docs/narrative/NARRATIVE_DOCUMENT_V1.md)定義口述、構圖、來源和變體。維護者已同意開發 N-03。[PR #71](https://github.com/bynanci/storytelling/pull/71) |
| [N-03／#50](https://github.com/bynanci/storytelling/issues/50) · N-R03 口述與插畫 | 接續 N-02。主分支已有本機指令，作者試讀待完成。 | `render-narrative` 依同一份來源輸出口述與兩種構圖。檢查來源、故障案例、輸入唯讀與重播一致性。另跑既有影片與 OTIO 回歸。[命令與輸出](docs/validation/n03-narrative-cli.md) |
| [N-04／#51](https://github.com/bynanci/storytelling/issues/51) · N-R04 修訂與作者決定 | 接續 N-03。主分支已有修訂查詢。真人檢視另列。 | 先改共同事實，再改單一構圖。工具依直接引用列出受影響的版本。歷史保留舊稿、提案及宣告的作者決定。[修訂驗證紀錄](docs/validation/n04-narrative-revisions.md) |
| [N-05／#52](https://github.com/bynanci/storytelling/issues/52) · N-R05 文案與證據 | PR #83 加入唯讀 CLI 切片；仍需真人判斷來源是否支持措辭。 | `audit-copy-claims` 顯示受眾、主張、CTA、被引用的歷史文字與 revision 狀態。同 head CI 與語意審查分開追蹤。[契約](docs/narrative/COMMERCIAL_COPY_V1.md) · [驗證紀錄](docs/validation/n05-commercial-copy.md) |
| [N-06／#53](https://github.com/bynanci/storytelling/issues/53) · N-R06 選擇新模式 | G0–G2。接續 N-04。 | 比較未決的構思分支、劇本揭露順序與缺素材的影片對照。記錄作者需求、成本與相容範圍。最多採用一種，也可全部暫緩。 |
| [N-07／#54](https://github.com/bynanci/storytelling/issues/54) · N-R07 產品決定 | G4–G6。接續 N-05。 | 重做基準任務。分組報告完成數與總數、耗時及歸因錯誤。將可重現的失敗連到測試案例。維護者決定繼續、縮小或停止。另行評估發布。 |

將[前七段 Codex 指令](docs/CODEX_NARRATIVE_PROMPTS.zh-TW.md)派給對應任務。後續五票與既有任務的追加指令收在[多模態派工指南](docs/CODEX_MULTIMODAL_PROMPTS.zh-TW.md)。

<a id="starting-n-01"></a>

### 執行作者試用

若要另做盲讀或接手改稿試驗，可使用[情境範例](examples/README.zh-TW.md)。請與原 N-01 試用分開記錄。

1. 在 [#48](https://github.com/bynanci/storytelling/issues/48) 記錄主分支提交碼及可能重疊的 PR。
2. 開啟[真人試用包](docs/narrative/N-01-AUTHOR-STUDY.zh-TW.md)。請作者修改修繕者身分，再指出哪些稿件須改，並說明雨傘細節的來源。
3. 依試用包的門檻招募五位參與者。每位完成兩次各八分鐘的配對試驗。分開記錄一般筆記與 Skill 的結果。原生工具若另行試用，獨立記錄該組結果。
4. 將匿名結果附於 #48。尚無觀察的欄位留白。維護者依事前約定的門檻記錄決定。

比較兩種敘事手法時，固定受眾、來源與事實。每次改變一項選擇，例如聽者何時知道日期。請讀者描述差異，再請作者說明選擇、修改或拒絕的理由。工具可以驗證順序。作者是否更容易改稿，需要真人觀察。

<a id="multimodal-rust-backlog"></a>

## 多模態任務

[修傘工作坊](skills/storytelling/references/workshop-demo.zh-TW.md)提供文字通知、SVG 示意圖與合成 WAV 音檔。作者可用 Skill 或 Markdown 起草。下表列出需要機器驗證的工作。

| 用途 | 現有能力 | 後續任務 |
| --- | --- | --- |
| 日常口述與插畫 | 本機指令能輸出口述與構圖。修訂查詢可列出直接引用該來源的版本。輸入不需媒體檔或片長。 | [N-04 核對修訂結果](https://github.com/bynanci/storytelling/issues/51) |
| 商業文案 | Skill 可整理主張及依據。影片條件檢查器無法判定文案是否獲得佐證。 | [N-05 查核主張](https://github.com/bynanci/storytelling/issues/52) |
| 流派與手法 | `check-narrative-style` 以獨立且綁定 revision 的 profile 檢查口述 beat 順序，不判斷美感。真人盲讀仍待執行。 | [N-08／#65 驗證紀錄](docs/validation/n08-style-profile.md)。 |
| 聲音與 Podcast | `render-cue-sheet` 依 N-02 綁定旁白、停頓與已提供的音效，保留未知時間並檢查互斥聲軌衝突。錄音與混音仍由宿主處理。 | [N-09／#66](https://github.com/bynanci/storytelling/issues/66)：[規格](docs/narrative/NARRATIVE_CUE_SHEET_V1.zh-TW.md)與[重播證據](docs/validation/n09-cue-sheet.zh-TW.md)。 |
| 編劇、分支構思與影片對照 | 文字樣板可用。影片規劃器支援鏡頭順序。 | [N-06](https://github.com/bynanci/storytelling/issues/53) 明確選案後，[N-10／#67](https://github.com/bynanci/storytelling/issues/67) 才實作該模式。 |
| 跨媒體交接 | `render-handoff` 將檔案、描述、文字標註、提案及缺件宣告綁定 N-04 修訂，區分須改日期與可沿用的構圖、聲音。 | [N-11／#68](https://github.com/bynanci/storytelling/issues/68)：[規格](docs/narrative/NARRATIVE_HANDOFF_MANIFEST_V1.zh-TW.md)與[重播證據](docs/validation/n11-handoff.md)。語意仍由作者檢視。 |
| 實際產出圖檔或音檔 | N-12 在 Linux 用 Inkscape 1.4 將合成 SVG 匯出 PNG 並讀回；入口不可用時交付構圖說明。[可行性紀錄與真實產物](docs/validation/n12-inkscape-png.zh-TW.md) | [N-12／#69](https://github.com/bynanci/storytelling/issues/69) 的縮小採用建議待維護者決定；正式 adapter 另行排程。 |

<a id="next-actionable-work"></a>

## 影片工程任務

| 工作 | 狀態與下一步 |
| --- | --- |
| M0、M1、M2 | 基礎、限定格式的影片交接及特定範圍的規劃已採納。更多編輯器、完整求解器與人工驗收另列。 |
| M3 | P1 與 P2 的限定審閱流程及 M3-02 日誌已採納。M3-03 具備隔離測試。正式執行器、執行中逾時的處理與跨平台實機驗收仍待完成。 |
| M4 | 規劃 TypeScript 用戶端與單一寫入者。須先驗收 M2-03 和 M3-03。 |
| M5、v1.0 | 規劃第二個編輯器、跨平台發行與獨立使用者評估。穩定版須通過相容性、遷移與復原驗收。 |

每張工程票的優先序、依賴與驗收條件見[逐項工程清單](docs/delivery/VIDEO_TASK_HISTORY.md#next-actionable-work)。現有範圍不包含完整剪輯器、編解碼器、特效引擎或任意外掛。新增功能時，先說明哪一項使用情境受阻。

<a id="file-coordination"></a>

## 多人協作

同一時間只由一個 PR 修改公開資料或時間契約。格式固定後，可平行開發轉接器與使用介面。來源、授權與平台相容性須依各自條件驗收。

<a id="accepted-lifecycle"></a>

## 已驗收工作與歷史紀錄

維護者已授權合併完成且通過同一提交檢查的 PR。合併後須核對主分支結果。發布版本、建立標籤、修改儲存庫政策或權限仍須另外授權。實際媒體、編輯器與真人驗收各自留存證據。[合併與授權紀錄](docs/delivery/VIDEO_TASK_HISTORY.md#accepted-lifecycle)

影片工程的 PR、版本、測試範圍與待驗項目，收錄於[逐項驗收紀錄](docs/delivery/VIDEO_TASK_HISTORY.md)。上游來源、對照版本與適用授權可查閱[來源紀錄](PROVENANCE.md)及[移轉指南](docs/MIGRATION.zh-TW.md)。

<details>
<summary>依舊版任務章節查找驗收紀錄</summary>

- <a id="merged-candidate-pending-g2-acceptance"></a>[PR #28 的 G2 待驗項目](docs/delivery/VIDEO_TASK_HISTORY.md#merged-candidate-pending-g2-acceptance)
- <a id="accepted-m0-delivery"></a>[M0 基礎驗收](docs/delivery/VIDEO_TASK_HISTORY.md#accepted-m0-delivery)
- <a id="m1-01-delivery-and-work-tracking"></a>[M1-01 唯讀匯入](docs/delivery/VIDEO_TASK_HISTORY.md#m1-01-delivery-and-work-tracking)
- <a id="m1-02-finite-otio-work"></a>[M1-02 OTIO 匯出](docs/delivery/VIDEO_TASK_HISTORY.md#m1-02-finite-otio-work)
- <a id="m2-01-bounded-brief-audit"></a>[M2-01 企劃條件稽核](docs/delivery/VIDEO_TASK_HISTORY.md#m2-01-bounded-brief-audit)
- <a id="m1-03-bounded-host-proof-work"></a>[M1-03 編輯器實測](docs/delivery/VIDEO_TASK_HISTORY.md#m1-03-bounded-host-proof-work)
- <a id="m2-02-fixed-window-selection-and-planning"></a>[M2-02 固定區間規劃](docs/delivery/VIDEO_TASK_HISTORY.md#m2-02-fixed-window-selection-and-planning)
- <a id="m2-03-selected-domain-live-planner-shadow"></a>[M2-03 規劃器執行結果對照](docs/delivery/VIDEO_TASK_HISTORY.md#m2-03-selected-domain-live-planner-shadow)
- <a id="m2-02o1-stable-ordering-optimisation--accepted"></a>[M2-02/O1 排序](docs/delivery/VIDEO_TASK_HISTORY.md#m2-02o1-stable-ordering-optimisation--accepted)
- <a id="m2-02o2-cached-exact-time-construction--accepted"></a>[M2-02/O2 精確時間](docs/delivery/VIDEO_TASK_HISTORY.md#m2-02o2-cached-exact-time-construction--accepted)
- <a id="m3-01p1-finite-otio-review-binding--accepted-bounded-profile"></a>[M3-01/P1 審閱識別](docs/delivery/VIDEO_TASK_HISTORY.md#m3-01p1-finite-otio-review-binding--accepted-bounded-profile)
- <a id="m3-01p2-selected-proposal-to-read-only-otio"></a>[M3-01/P2 唯讀剪輯交接](docs/delivery/VIDEO_TASK_HISTORY.md#m3-01p2-selected-proposal-to-read-only-otio)
- <a id="m3-02-sqlite-operation-journal--accepted-bounded-storage-slice"></a>[M3-02 操作日誌](docs/delivery/VIDEO_TASK_HISTORY.md#m3-02-sqlite-operation-journal--accepted-bounded-storage-slice)
- <a id="m3-03-filmcraft-executionreadback--bounded-proof-only"></a>[M3-03 編輯器讀回](docs/delivery/VIDEO_TASK_HISTORY.md#m3-03-filmcraft-executionreadback--bounded-proof-only)
- <a id="not-in-the-current-backlog"></a>[暫緩項目](docs/delivery/VIDEO_TASK_HISTORY.md#not-in-the-current-backlog)
- <a id="openfilm-to-rust-transfer-ledger--2026-11-07"></a>[歷史來源移轉清單](docs/delivery/VIDEO_TASK_HISTORY.md#openfilm-to-rust-transfer-ledger--2026-11-07)

</details>

