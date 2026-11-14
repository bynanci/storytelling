<!-- SPDX-License-Identifier: MIT -->
# 依證據選擇 W01、唯讀 SDK 與預檢 B 的技術路線

[English](g2-roundtable-2026-11-14.md) · 繁體中文 · [來源索引](g2-roundtable-2026-11-14/evidence-index.json)

基準：`main@bca998784a0e9d3891cd7b28aab660d7d4ad7db0`，2026-11-14 UTC 核對；當時沒有開放中的 PR。本文件只記錄 [#103](https://github.com/bynanci/storytelling/issues/103)、[#107](https://github.com/bynanci/storytelling/issues/107) 與 [#129 B](https://github.com/bynanci/storytelling/issues/129) 的文件層技術選擇。

## 選定的路線

| 技術選擇 | 選定建議 | 邊界 |
| --- | --- | --- |
| #103／EX-W01 | 推進聚焦的純契約候選：嚴格 wire 解析、精確身分向量、P2 擁有的編譯綁定與不透明型別測試。 | 不提供 `ExecutionPermitV1`、authority provider、專案 writer、派送或 Host 呼叫。W01 本身不能完成 #103 最終 G2，也不能解鎖 #104。 |
| #107／ADR 0013 | 在現有 engine crate 採用八項唯讀 facade 操作，直接委派 backend 並完整保留錯誤。 | 沒有 execute／approve／recover 方法、transport、第五個 crate 或穩定版主張。Golden parity 與實際 consumer 使用尚未觀察。 |
| #129 B | 在採納下文的 signature／順序／escaping 小幅澄清後，使用 A 原輸入提供純文字預檢。 | A／V1 保持最終權威；拒絕時不輸出 V1 payload，不做媒體 probe、Reference App 或執行動作。 |

以上是選定、**供採納的工程建議**，尚非已記錄的真人 G2 接受。請求者要求 Codex 透過圓桌判斷並加入證據，其原話如下：

> 根據圓桌會議判斷選擇內容，並加入真人與Host 支持研究證據 已送出的 #103 W01、#107 SDK、#129 B 三項技術選擇。

本文件將這段指示記為選擇與佐證提案的請求，不將它改寫成簽章、某人已審查最終精確候選的主張，或真實專案操作批准。來源索引中的正式維護者接受、接受者身分及來源仍為 null。**三項建議仍須依原本 G2 規則實際採納後，才能進入 G3。** 合併本文件只記錄分析。

## 本次圓桌如何進行

本次 Codex session 中，四個非同步、唯讀的 **AI 審查角色**檢視相同基準：執行／安全、SDK／API、作者端 CLI／UX，以及證據／方法；由 Codex 整理建議。儲存庫文字與 GitHub Issues 的 `圓桌`、`roundtable` 搜尋沒有找到先前公開圓桌紀錄；可能存在的私人會議未經檢視。

這些角色不是人類參與者，也不是現場會議。它們共用 session context；SDK 與 B 的審查角色曾參與前期設計，判斷彼此相關，不能視為盲測、獨立真人審查或外部安全稽核。本次沒有新增作者 session、模型成效實驗或 Host 執行。

| 角色 | 建議與保留的不同意見 |
| --- | --- |
| 執行／安全 AI | 接受純 W01 候選；暫緩 production admission、可信 provider／fencing／quiescence 及 #103 最終接受。宣告的 Host descriptor 不能變成已驗證的 Host 真實狀態。 |
| SDK／API AI | 在記錄 operation IDs、result imports 與錯誤 accessor／trait 行為後，接受有界 facade 基礎。既有 backend tests 沒有建立 wrapper parity。 |
| CLI／UX AI | 先修訂 B 的少數細節，再接受限定提案。目前資料包明列 bridge 與呈現規則待決，不能在決定前標為 G3-ready。 |
| 證據／方法 AI | 既有觀察支持問題與有限的下一步實驗。須保留已停止的 N07 wrapper 試用；不得把 Skill、讀者或舊 Host 結果當成新 API 的驗收。 |

本次沒有無條件核准整份正式執行契約，也沒有四位真人投票。

## 真人證據與相反結果

各來源 ID 均可由[來源索引](g2-roundtable-2026-11-14/evidence-index.json)查回。各 cohort 分開分析；不同紀錄間的參與者重疊未知，不推算不重複人數總計。

| ID | 既有觀察 | 可支持的用途與限制 |
| --- | --- | --- |
| H-N01 | 維護者證言的五位作者普通筆記／Skill pilot：完成 3/5 與 5/5，修訂時間中位數 310／185 秒，重複輸入中位數 3／1。回報順序為 3 AB／2 BA，並有首次接觸篩選。 | 支持調查來源／變體修訂工作。私人紀錄未獨立稽核；精確 Skill prompt、host、model／settings 不完整。Continue 決定只適用該 Skill 任務，不證明 CLI／SDK 效益。 |
| H-102-READ | 三份首次閱讀預約公告紀錄辨認出網站；一位開窯讀者區分了日誌事件與已交付的可剪畫面。 | 支持分開呈現來源描述與缺素材。沒有量測實際預約行動或檢視實際媒體；第二次閱讀不是另一位首次接觸參與者。 |
| H-102-HAND | 球場交接於不同改稿卡耗時 72／55 秒；開窯交接耗時 68／52 秒，逐稿核對回報沒有漏改、誤改或連帶破壞。 | 支持測試修訂範圍是否可見。沒有相同任務的對照、來源圖建立成本或已知參與者重疊；這些時間不能證明省時。來源報告未依本輪提議新的 guardrail／schema。 |
| VH-12 | 維護者提供的一位作者，先用試算表，再做結構契約的人工 walkthrough；當時 CLI 尚未存在。 | 只支持問題假設。原帳未提供日期、時間或先前接觸狀態；預期診斷不是程式實際輸出。 |
| VH-3 | 維護者／作者自操作，2026-11-12 13:02 +08，已有 S1／S2 接觸，自述 145 秒。OUT `6/1` 被拒絕，改為 `5/1` 後保留 unresolved；作者指出 rational strings 與逐次首錯修正的摩擦。 | 最接近 B 的動機。Codex 未觀察操作或計時；沒有隨機化、獨立作者或 B 預覽。S1／S3 時間不能建立因果改善。 |
| H-N07STOP | 提交的 external-wrapper 流程：0/5 主要完成、5/5 作者出現重大 U01 來源歸因錯誤；維護者選擇 Stop。 | 須保留這項相反證據。提交的 build／argv／envelope 無法綁定凍結的原生 `ce73836`；合格原生配對仍為 0，**待真人證據**。這既不證明原生失敗，也不支持 wrapper 效益。#54 的 Stop 與尚未採納的 v3 提案維持原狀。 |

H-N07STOP 尤其支持保留精確 build、argv 與原始輸出，並分開記錄「引用存在」、「主張有佐證」與「某人已批准」。它不能支持重寫 R1，也不能把本次審查變成新參與者證據。

## 機器重播與實際 Host 證據

| ID | 既有觀察 | 不能證明的事項 |
| --- | --- | --- |
| M-V1 | Codex 以記錄的合併後 CLI 重播提供的 N10 JSON：exit／JSONPath 正確、檔案未變、file／stdin 等價；七項 CLI 回歸通過。 | 作者行動、使用性或效益。記錄的 3.568／1.839 ms 單次 process 測量不是作者耗時或 benchmark。 |
| M-MEDIA | Codex 探測提供的**合成 FFmpeg testsrc MP4**：H.264、1280×720、30/1、5 秒、150 frames。 | 未證明宣告 `CLIP-01` 與這些 bytes 的綁定、敘事內容符合、實際拍攝素材或編輯器套用。 |
| HOST-M1 | 歷史 Codex harness 在 Linux CPU 用真實 FilmCraft 0.18.0 與合成媒體，檢查精確 25 fps／ticks、preview、儲存／重開及 OTIO 再匯入。 | 真人剪輯品質、audio／GPU、其他 OS／profile、正式執行權限，或一般檔案系統／媒體驗證。 |
| HOST-M3 | 歷史真實 FilmCraft 隔離匯入：Host 成功退出後隱藏回應，journal 保留 unknown，native／project-byte 回讀解析為 applied；一次 import、零 retries／Undo，拒絕晚到回應。 | 仍在執行／卡住的 Host、process-group quiescence、安全 NotApplied 重試、正式 provider 或當前權限。紀錄明列 `approval_granted=false`、`executable=false`。 |

「真實 Host」表示確實操作過應用程式；媒體與專案仍是合成且隔離的。這些歷史觀察未驗收目前的正式 profile，也不是本任務新增的 Host run。

## 實作前須採納的細節

### W01：編譯身分不等於執行許可

使用基準中的精確 [ADR 0011](../adr/0011-production-execution-contract-v1.md)、[Execution V1](../migration/EXECUTION_CONTRACT_V1.md) 與[案例／工作包](m3-04-execution-contract.md)，將第一個實作限定如下：

- 嚴格解析 request／result **宣告**；保留 65,536-byte／depth-8 上限、重複鍵拒絕、確定性錯誤順序與既有 domain-separated identity／receipt 向量。
- 透過 `BoundStoryReviewV1` 保留實際由編譯器擁有的 P2 bytes 與 binding；不另選剪點／loss，也不把呼叫者提供的 artifact／digest 當成編譯證據。
- 私有欄位、不能 Deserialize 的 compiler-bound candidate 與 compile-fail 檢查；此 candidate **不是 `ExecutionPermitV1`**。
- 先做 EX01／EX03／EX24–EX27／EX30–EX36 的純函式部分及獨立向量。案例清單屬計畫行為，尚非已通過測試。

純檢查不得產生規範執行狀態 `ready`，也不得將它改作「contract valid／compiler bound」標籤。可攜結果狀態經解析後仍是不可信的宣告。本機「contract valid／compiler bound」標籤須與當前 authority、已驗證 Host 狀態及 dispatch readiness 分開。Provider／approval consumption、專案 fencing、readback／quiescence 與 mutation 案例留給各自審查的 runtime 工作。#103 最終接受仍需指定的 tested-contract 證據及維護者實際決定；W01 的部分測試不能完成全部案例。

### SDK：保留單一有型別的委派介面

採用 [ADR 0013](../adr/0013-read-only-consumer-facade-v1.md) 的八個名稱／結果：`validate_plan`、`render_narrative`、`track_revisions`、`query_impact`、`video_coverage`、`plan_story`、`prepare_story_review`、`check_story_review`。Operation IDs 就是這些名稱。只整理既有 result aliases 與 immutable accessors 所需的 `ReviewRecordV1`；不重新匯出無關 backend constructors。

選擇可失敗的 `source_details_json() -> Result<Vec<u8>, SdkEncodingErrorV1>`，使用 adapter-only 的 `sdk.error_details_encoding_failed`。即使編碼失敗，原始錯誤仍須保留。候選 `Debug`／`Display` 只公開 operation／code／path；`std::error::Error` 不得暴露私有 backend 型別或 raw input。Code／path 與完整的 opt-in details 才是機器介面；unresolved／stale 保留原本成功的 domain result。

[Golden plan](../proposals/m4-01-readonly-sdk/test-plan.md) 須建立實際 external-crate compile／opacity 與 CLI／SDK parity。Narrative projections 尚無 `Serialize`，應用 typed assertions 與既有 formatter，不新增 wire schema。AI 撰寫的 external-crate test 是工程證據，沒有構成獨立真人開發者研究。

### B：權威結果優先、私有投影與已轉義文字

選擇既有候選命令 `preflight-video-coverage-input <FILE|-> [--lang zh-TW|en]`，預設 zh-TW，使用 opt-in `draft-video-input`；保留 exits 0／1／2／3。Bridge 提案為 `render_draft_video_preflight(bytes: &[u8], english: bool) -> (String, bool)`，第二個值僅代表宣告有效。跨 crate linkage 可能需要 feature-gated、doc-hidden 的 public function；這不等於 Rust privacy，也不是穩定 SDK 承諾。View／row／hint 型別保持私有，不新增 public DTO／wire schema。

先讓完整原始 bytes 通過 A，再投影提示。Syntax／duplicate／budget 失敗不產生 hints；helper 失敗須明說 V1 尚未執行。重用 A 精確算法與未改動的全輸入 V1 結果，不新增第二套區間／引用驗證器。保留所有 selected indices／順序及 unmatched mapping 列。無法解讀的引用只顯示原 index 與固定標籤，不補造 unit／revision。

候選提示順序：mapping index，再依固定欄位順序 `unit_ref`、`status`、`asset_ref`、`source_range.start`、`source_range.end`、`reason`；缺 mapping 的 selection 提示接續依 selection index 排列。巢狀時間診斷保留 A 順序。將 C0 U+0000–001F、DEL U+007F、C1 U+0080–009F，以及 bidi controls U+061C／U+200E–200F／U+202A–202E／U+2066–2069 轉成可見資料，不改 source bytes。補 malformed／duplicate selection、unmatched mapping、C1 CSI、CR／tab 與 bidi 案例。被拒絕輸入的 display-only 轉換須標成暫定；拒絕時不輸出 V1 payload。採納這些差異前，[B 資料包](../proposals/n10-06b/README.md)仍為提案依據。**目前沒有 B 程式或作者成果。**

## 下一步證據與決定交接

使用候選[空白私人蒐集輔助表](g2-roundtable-2026-11-14/handoff.blank.json)；它沒有採納研究 protocol，也不能驗證批准身分。重用既有 B／SDK test plans 與 M3 工作包，不新增重複研究。任何新量測 session 前，須選定並凍結 protocol、build、材料、任務、上限與評分。另經授權的候選 Host 測試可以在 #106 最終接受前蒐集驗收證據；本文件不授予這項權限。保留五項核心紀錄：**原題、起始資料、作者逐字回覆、觀察行為、限制／失敗**；另記日期／時區、匿名 operator／observer、先前接觸、實際順序、完整 argv／raw output 及偏離。作者耗時包含等待；process time 另記。保留失敗與非預期答案；不得把既有參與者改標為首次接觸的新 cohort，後續參與須如實記錄接觸史。

| 工作 | 下一份實際證據 | 保留的 gate |
| --- | --- | --- |
| W01 | 實際 test-first 純契約／opacity RED／GREEN 與最終 head 相容性檢查；不呼叫 Host。 | #103 實際最終決定；#104–#106 的 production／candidate-Host 授權、實作與驗收。 |
| SDK | 先取得 external-crate golden parity，再由未參與 API 設計的真人開發者嘗試文件流程，記錄錯誤／求助／回饋。 | 實際真人使用性及後續 transport／mutation／stable-release 接受。 |
| B | 實作後由新作者進行 mapped／unresolved、秒數／影格、stale／range 修正，並解釋宣告就緒的含義。 | 尚無完成的 B 作者試用；省時比較須有自己的前瞻式等價對照。 |
| 後續 Host | 另行授權的隔離 profile 測試：正常 apply／readback／reopen、已完成但回應遺失、執行中／卡住行程、歧義回讀、晚到回應與 competing writer。 | 現在不執行，不以舊 lost-reply 證據推論 quiescence，不重試 unknown，也不測真實使用者專案。 |

原始個人／媒體／路徑／token 證據保持私有，只公開經同意的匿名摘要或合成重現。既有 #54／#112 protocols 與 G5 保持各自範圍。本次交付選定三條有界路線、整理支持與反證；正式採納、實作及新增真人／Host 結果均不能由這次 AI 審查填入。
