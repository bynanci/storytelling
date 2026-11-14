# Storytelling 開發路線圖

[English](ROADMAP.md) · 繁體中文

Storytelling 的第一個跨媒體目標是讓作者能改稿。試想一張活動通知改了日期。作者應能找出受影響的口述稿和海報，也能保留單一版本的構圖選擇。[設計依據與使用情境](docs/WHY_STORYTELLING.zh-TW.md)

<a id="product-objective"></a>

## 現有功能

| 入口 | 可以交付的成果 | 如何查證 |
| --- | --- | --- |
| Storytelling Skill | 依受眾、目的與素材起草。可交給支援 Agent Skills 的工具，也可照 Markdown 手動操作。 | [八種敘事樣板](skills/storytelling/references/multimodal-templates.zh-TW.md)與[修傘工作坊](skills/storytelling/references/workshop-demo.zh-TW.md) |
| 本機敘事指令 | `render-narrative` 將已宣告的內容輸出成口述大綱與插畫構圖。範例含兩款構圖。 | [CLI 指南](docs/CLI_GUIDE.zh-TW.md)與 [N-03 驗證紀錄](docs/validation/n03-narrative-cli.md)已附請求者檢視及近景修訂稿；比較性真人試用仍待執行。 |
| 本機修訂與順序檢查 | `track-narrative-revisions` 和 `query-narrative-impact` 保留歷史並列出直接受影響版本；`check-narrative-style` 以綁定 revision 的獨立 profile 驗證口述順序，不判斷美感。 | [N-04 修訂契約](docs/narrative/NARRATIVE_REVISION_LEDGER_V1.md)、[N-04 紀錄](docs/validation/n04-narrative-revisions.md)與[N-08 紀錄](docs/validation/n08-style-profile.md)。N-08 依維護者證言具限制結案，隨機方法仍未記錄。 |
| 本機影片指令 | 以明確的素材區間和條件規劃鏡頭。可在支援範圍內匯出純畫面的剪輯時間軸。 | [影片里程碑](#milestones)列出已測的格式、編輯器與限制。 |

Skill 可以獨立安裝。目前套件版本為 `0.1.0-dev.0`。使用者可從 GitHub 安裝。維護者另行決定何時發布至公開套件登錄站。[安裝方式](docs/PLATFORMS.zh-TW.md)

<a id="cross-media-narrative-roadmap"></a>

N10-06A／#129 的[精確時間輸入輔助](docs/adr/0012-draft-exact-video-range-input.md)已由 [PR #130](https://github.com/bynanci/storytelling/pull/130) 以 `125e5b1` 合併。最終 head `b63552e` 與合併後 main 六項 CI 均通過；[驗證紀錄](docs/validation/n10-06a-exact-input.md)保留原 draft-only G2 決定及 RED／GREEN。[B/C/D 提案／檢查表](docs/validation/continuation-2026-11-14.md)也已納入 main，各自的技術 G2、實作及證據門檻仍待完成。獨立作者／媒體／Host 驗收及 G5 維持分開。

## 跨媒體敘事的開發順序

[研究設計對照](docs/research/README.zh-TW.md)連結證據、規則與流程優先順序。效益主張仍需人工觀察。

| 順序 | 工作與依賴 | 驗收依據 |
| --- | --- | --- |
| 1．改稿與修訂 | N-02 契約已採納。N-03 已提供本機輸出。作者可檢視口述順序、來源標記及兩款構圖。N-04 已可呈現來源修訂與作者決定。五位作者完成 [N-01 Skill 試用](https://github.com/bynanci/storytelling/issues/48#issuecomment-6798756961)，維護者決定繼續。 | 同一項修訂應找出所有直接引用該來源的版本。只改一款構圖時，另一款應保留原稿。[真人試用包](docs/narrative/N-01-AUTHOR-STUDY.zh-TW.md) |
| 2．文案與依據 | N-05／#52 的 V1 基礎已在 PR #83 合併；[PR #87](https://github.com/bynanci/storytelling/pull/87) 加入明選的產品 ReportV2。保留修門審查，產品案例另留作者欄。CLI 輸出引用 revision 與歷史文字，由請求者判斷佐證。N-07 接續 N-05。 | 保留一次測試的結果，移除無來源的成效主張並標示提案。維護者已 Stop 外部 wrapper 試用（0/5 主要完成、5/5 重大 U01 錯誤）。固定 ce73836 native 驗收未證實；未來評估須另訂版本並事前同意。 |
| 3．選擇其他模式 | N-06 比較分支構思、劇本揭露與影片對照。請求者在 2026-11-11 暫緩三者，並於 2026-11-12 只為 N-10 選擇影片限定 `NarrativeVideoCoverageV1`。N-08 檢查宣告順序，N-09 提供獨立聲音提示表。 | 保留[歷史 N-06 決定](docs/validation/n06-format-discovery.md)與[後續 N-10 選案](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432)。#67 由 PR #93 接受 G2；有界 G3 CLI 與自動化 G4 檢查已在 [PR #97](https://github.com/bynanci/storytelling/pull/97) 合併，同 head 六項工作全過，見 [run #376](https://github.com/bynanci/storytelling/actions/runs/43148901399)。N10-05 限定證據已完成：Codex 重播附件 JSON，維護者提供一場合併後作者自我操作 session（145 秒、先前已有接觸、無隨機化或獨立觀察者），Codex 也探測了提供的合成 MP4。此紀錄屬定性自我操作，不是獨立使用者研究；V1 不讀取媒體位元組。見[實作紀錄](docs/validation/n10-video-coverage-cli.zh-TW.md)及[合併後操作紀錄](docs/validation/n10-05-execution-record.zh-TW.md)。N-08 記錄兩次維護者證言的官方表單閱讀，保留順序與證據限制。G5 發布另行決定。 |
| 4．交接來源與產物 | N-11 使用 N-04 的來源修訂。N-12 可獨立測試一項宿主能力。 | 接手者能辨識缺件與過期來源。產出圖檔或音檔後，必須重新開啟或播放。缺少工具時交付可用的構圖或提示表。 |

工作項目與狀態見[敘事任務](TASKS.zh-TW.md#cross-media-narrative-backlog)及[多模態任務](TASKS.zh-TW.md#multimodal-rust-backlog)。[架構](docs/ARCHITECTURE.zh-TW.md)定義資料、驗證與輸出入的分工。作者決定稿件是否符合創作意圖。

<a id="milestones"></a>

## 影片 SDK 里程碑

影片工作另有一項目標。作者從 30 至 50 張照片、短片及旁白中，整理出 60 至 90 秒的可編輯粗剪。敘事稿要交給影片規劃器時，需提供素材 ID 與來源區間。

| 里程碑 | 交付狀態 | 驗收範圍 |
| --- | --- | --- |
| M0 基礎 | 已採納。Rust CLI 可驗證 StoryPlan。 | 嚴格資料契約、來源邊界、輸入唯讀與三種作業系統的 CI。 |
| M1 影片交接 | 限定格式的匯入與 OTIO 匯出已採納。M1-03 已在 Linux 驗證 25 fps 純畫面專案。 | FilmCraft 已實測匯入、CPU 預覽、讀回、存檔與重開。音訊、其他設定與人工驗收仍待測。 |
| M2 鏡頭規劃 | 限定範圍的條件稽核、固定區間規劃與執行結果比對已採納。完整求解器仍待開發。 | 相同輸入須能重現結果。硬性條件無法滿足時，應指出衝突。 |
| M3 審閱與執行 | P1 審閱、P2 唯讀交接與 M3-02 操作日誌已採納。M3-03 已有隔離測試。正式執行器仍待驗收。 | 核對授權、提案、成品與操作擁有者。部分完成或結果不明時，需從日誌和實際專案復原。 |
| M4 用戶端 | 唯讀 facade 設計已由 [PR #134](https://github.com/bynanci/storytelling/pull/134) 合併為 [G2 提案](docs/proposals/m4-01-readonly-sdk/README.zh-TW.md)；facade、TypeScript 與 Reference App 實作仍待完成。 | 同一服務只有一個寫入者。使用者能審閱、核准及檢查結果。MCP 介面為選用項目。 |
| M5 測試版 | M5-02／#112 仍受阻於 #109、#111 及已接受的 M3 執行／復原流程。[前瞻式 v1 規程](docs/validation/m5-02-rough-cut-beta-v1.zh-TW.md)僅為設計，Beta 尚未開始。 | 使用結果、實測平台組合與安裝證據。 |
| v1.0 穩定版 | 尚未排定。 | 採納公開契約、遷移方式、安全、復原及相容性驗收結果。 |

<a id="dependency-path"></a>

## 實作依賴

| 工作 | 開始條件 |
| --- | --- |
| M1-01 → M1-02 → M1-03 | 接續 M0。驗證限定格式的交接後，才執行編輯器測試。 |
| M2-01 → M2-02 → M2-03 | M2-01 接續 M1-01。需求與測試素材可先準備。 |
| M3-01/P1 → P2 → M3-02 → M3-03 | P1 需要 M1-03 與 M2-02。P2 將選定提案交給唯讀 OTIO 審閱流程。 |
| M3-04／#103 → #104 → #105 → #106 | [Execution V1](docs/migration/EXECUTION_CONTRACT_V1.zh-TW.md)已由 [PR #117](https://github.com/bynanci/storytelling/pull/117) 以 `a23a290` 合併為提案文件。W01 授權／實作與實測契約的最終 #103 G2 接受仍待完成，之後才可開始 #104；執行器、復原及宿主驗收各自有門檻。 |
| M4 → M5 → v1.0 | M4 需要 M2-03，以及可用於正式環境的 M3-03 執行與讀回流程。 |

[P2 契約](docs/migration/PLANNER_OTIO_REVIEW_V1.md)及 [M3-02 日誌](https://github.com/bynanci/storytelling/pull/56)已納入主分支。[M3-03 隔離測試](docs/validation/m3-03-filmcraft-execution.md)重現了編輯器執行成功、呼叫端卻未收到回覆的情況。逾時後須先讀回專案。若操作仍可能執行，或讀回結果不明，應停止重送。

<a id="investment-decisions"></a>

## 如何決定是否繼續

- **M1**　若交接遺失來源區間或基本片段，先縮小支援格式。
- **M2**　若 Rust 與參考規劃器結果不同，先查明原因。期間保留原規劃器，由 Rust 檢查與編譯。
- **M4**　比較透過 SDK 與直接使用 FilmCraft 搭配代理工具的同一項任務。記錄作者投入的時間、成品品質與可控制的選項。
- **M5**　#112 保留五位中三位完成粗剪、兩位願意回來的目標。提案規程把「完成」定義為安全完成，且招募前須由維護者接受；目前沒有試用結果或發布準備結論。另需一位獨立整合者。

<a id="release-and-operation"></a>

## 發布與維運

每個候選版本須記錄來源提交碼、鎖定的依賴、資料格式版本及測試產物。使用外部工具時，也須記錄工具版本。維護者核准後才能發布。

破壞相容性的格式變更須提供遷移方式。舊工具收到較新的格式時，應明確拒絕。執行器遇到部分完成或結果不明時，須保留狀態紀錄。實際媒體試聽、原生簽章與目標編輯器驗收各自留存證據。

<a id="native-migration-strategy"></a>

## 將既有功能移至 Rust

1. 固定資料模型、精確時間、唯讀匯入與 OTIO 格式。先證明一個編輯器可接手作品。
2. 依使用情境選取演算法。以固定素材比對新舊結果。旅遊、節奏和轉錄功能依需求選用。
3. 在執行契約獲採納後，才移轉審閱、操作日誌和復原流程。外部編輯器的動作無法由 SQLite 交易回滾。
4. 參考應用可繼續使用 SDK。介面、編解碼器、GPU 特效與完整剪輯功能由宿主負責。轉接器須列出依賴的工具或模型。
5. 移轉路徑與檔案清單工具前，先檢查來源及並行存取。可攜 SDK 無須載入 Tauri、GTK 或 Node 啟動程式。
6. 固定契約後，先並行比對新舊結果。再以功能開關切換。確認新實作接手後，才停用舊實作。唯讀匯入全程保留來源資料。

[移轉指南](docs/MIGRATION.zh-TW.md)列出來源、工作清單與驗收紀錄。判定一項移轉完成時，須核對新舊行為、執行依賴及回歸結果。Rust 二進位檔、呼叫 FFmpeg 的轉接器與純 Rust 演算法各有不同的依賴。

## Gemini R1技能評估紀錄（SK-EVAL-R1）

[40案模型評分報告與視覺化](docs/research/evaluations/2026-11-13-gemini-r1/README.zh-TW.md)保存原始數值、來源勘誤與宣告分組限制。這是W4/W6的診斷觀察，不新增已驗收需求、真人結果或完成的技能改善。
沿用[#115](https://github.com/bynanci/storytelling/issues/115)總追蹤。[SK-E01／#118](https://github.com/bynanci/storytelling/issues/118)已合併交付[離線紀錄與盲測流程](docs/research/evaluations/sk-e01/README.zh-TW.md)。[SK-E02／#119](https://github.com/bynanci/storytelling/issues/119)已建立評估專用的[證據計畫、裁定與資料包勘誤契約](docs/research/evaluations/sk-e02/README.zh-TW.md)；合成驗證不會補上 R1 來源脈絡，也不是新的評估結果。[SK-E03／#120](https://github.com/bynanci/storytelling/issues/120)、[SK-E04／#121](https://github.com/bynanci/storytelling/issues/121)及[SK-E05／#122](https://github.com/bynanci/storytelling/issues/122)仍是獨立後續工作。既有N與M驗收、#54處置及#112真人門檻維持各自證據範圍。
