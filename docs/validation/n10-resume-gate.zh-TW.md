<!-- SPDX-License-Identifier: MIT -->
# N-10 / N-R10：敘事單元對應影片素材的重新啟動門檻

[English](n10-resume-gate.md) · 繁體中文

**狀態：** G0–G1 範圍已選定；`NarrativeVideoCoverageV1` 的 G2 契約仍是待審草案，本文件不含 G3 實作。
**追蹤：** [Issue #67](https://github.com/bynanci/storytelling/issues/67) · [前置研究 #53](https://github.com/bynanci/storytelling/issues/53) · [V 選擇紀錄](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432) · **更新後基準 commit：** `a2de070db0c485bf55af6b7553bbfdd49f5efc44`。
**決策沿革：** 2026-11-11 的三項暫緩仍是 N-06 當時結論；2026-11-12 的 V 決定只對 N-10 取代該暫緩。編劇與分支構思仍暫緩。已選定模式邊界；欄位規則與診斷碼仍須 G2 審查。

## 最新進度｜2026-11-12

2026-11-12，請求者選擇 **V**，並指定作者任務：選兩個故事單元，一個宣告有影片、一個沒有素材；接手者必須保留缺件為 unresolved，並阻擋「可進入規劃／匯出」狀態。採用的版次邊界是獨立、僅限影片、唯讀的 `NarrativeVideoCoverageV1` 宣告與驗證器。它不讀取或驗證媒體、不媒合素材、不渲染、不產生 OTIO／匯出，也不寫入剪輯器；既有敘事及影片契約維持不變。此決定不是真人試用或真實媒體結果。G3 前仍須完成 G2 契約審查。

## G0｜具體作者需求與第一個可觀察缺口

專案維護者已要求推進 #67，也曾提出讓結構化故事協調影片、圖片、旁白和剪輯選擇的目標。本階段以不涉及私人素材的**候選使用情境**作為切入：作者選定兩個敘事單元，一個有可用影片，另一個沒有素材，希望交接時明確標出缺口，不能在匯出前悄悄漏掉選定段落。

以上是**使用者曾描述的工作需求方向，加上本次提出的合成案例**；尚無真實影片、正式操作時長、實機成果或使用性失敗證據。不得把這些情境當成已觀察到的真人試用。公開文件不放入私人的人名、錄音或素材路徑。

既有 [N-06 影片銜接案例](../narrative/n06-cases/video-handoff.zh-TW.md) 已用合成資料呈現**契約缺口**：S02@1 宣告素材及精確區間 [5/2, 9/2)，S03@1 雖被選定卻沒有素材。現有 `plan-story` 仍能規劃只有 S02 的子集，因為它無法知道上游曾選了 S03。這**不是 Planner 的缺陷**；若要保證涵蓋選定敘事單元，就必須在 Planner 之前驗證敘事單元與資產的覆蓋關係。

### 三個候選，只推薦一個供後續選擇

| 候選 | 現有最小做法 | 尚未直接進入原生實作的理由 |
| --- | --- | --- |
| 分支構思 | 命名草稿＋外部待決問題表 | 尚未證明需要以原生狀態保存互斥選項 |
| 編劇揭露 | NarrativeDocumentV1 節拍＋N-08 順序檢查 | 尚未證明需要現有揭露順序以外的場景／對白型別 |
| **故事→影片素材映射（N-10 已選）** | 明確保留缺素材列的覆蓋表 | 確有可描述的交接完整性缺口：選定敘事單元可能在交給 Planner 前消失 |

維護者已在 [#53](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432) 明確選定單一候選、作者任務及版本邊界。N-06 的合成案例仍只是工程示例，不是真人試用結果。

## G1｜已選定的最小驗收範圍

**已選切片：** `NarrativeVideoCoverageV1`，獨立、唯讀、不可執行的映射宣告與驗證結果。模式／版次邊界與排除項已選定；下列欄位規則仍是 G2 待審契約。第一版僅驗證影片映射，不含圖音同步、字幕辨識、素材搜尋、AI 媒合、渲染、OTIO 匯出、剪輯器寫入或自動授權。

候選契約會嵌入完整 `NarrativeDocumentV1` 快照並指定一個變體。`selected_unit_refs` 以明確順序選取該變體中一個以上、版本仍有效的單元；這只代表作者宣告的選擇，不推測作者原本意圖。資產目錄快照宣告目錄 ID／revision、asset ID／revision 及長度。`mappings` 必須依選取順序，對每項引用恰好對應一次。每列要嘛以精確目錄 asset revision 和半開有理數來源區間標成 `mapped`，要嘛以 `unresolved` 加原因保留缺口。目錄與媒體資訊都只是宣告：不開啟宿主素材、不驗證檔案雜湊或畫面內容。

待審的 [ADR 0010](../adr/0010-narrative-video-coverage-v1.md)、[封閉 JSON Schema](../../contracts/narrative-video-coverage-v1.schema.json) 與[成對 fixtures](../../tests/fixtures/narrative-video-coverage-v1/README.md)定義欄位、限制、驗證順序及診斷。含 `unresolved` 列的合法輸入會產生可重現的 blocked 結果，不是格式錯誤；缺漏、重複、過期或懸空引用才是契約錯誤。完整的宣告映射只允許進入規劃階段，不能證明可匯出或媒體真實有效。

| 預定測試 | 預期行為 |
| --- | --- |
| 每個選定單元都有有效合成映射 | 宣告檢查通過；仍非可執行或真實媒體驗證 |
| S03@1 明確 unresolved | 回報覆蓋不完整；**阻擋**可規劃／可匯出狀態 |
| 上游漏掉選定的 S03@1 | 拒絕缺少選定單元 |
| 同一選定單元重複映射 | 拒絕重複；不得任選一條 |
| 需要 S03@2，來源只有 S03@1 | 拒絕過期 revision |
| 指向目錄不存在的 asset_id | 拒絕懸空資產 |
| 區間終點超過宣告長度、起點不小於終點或有理數格式錯誤 | 拒絕無效區間 |
| 敘事單元 ID 剛好與 asset ID 相同 | 不以同名推定映射，仍需顯式引用 |
| 相同輸入重跑兩次 | 相同輸出 bytes 與穩定錯誤代碼 |
| 不支援版本、JSON 重複欄位、超出大小限制 | 受控拒絕、沒有副作用 |

候選穩定錯誤碼包括 `video_coverage.missing_selected_unit`、`video_coverage.unresolved_required`、`video_coverage.stale_unit_revision`、`video_coverage.dangling_asset`、`video_coverage.invalid_range`；完整分類和第一錯誤順序見 ADR 0010，仍待 G2 審查。

**相容界線：** 保持 NarrativeDocumentV1、N-04 revision ledger、N-08 profile、N-11 handoff、StoryBriefV1、PlannedStoryV1 及既有 video／OTIO 契約不變；不可將 narrative unit ID 當 asset ID、不可憑空補素材、不可將創作提議升格為來源觀察、不可遺漏選定單元卻宣稱交接完整。未解映射是必要的編輯警示，不是成功結果。

## G2｜已選範圍、待審契約與後續工作包

| 工作包 | 交付成果與驗證 | 負責角色／階段 |
| --- | --- | --- |
| N10-00 · 採用決定 | V、兩單元一有素材一缺件的作者任務，以及僅影片／唯讀 V1 邊界已記錄於 #53 與本文件。 | Maintainer · **已記錄** |
| N10-01 · 契約 | 審查獨立版次、封閉 schema、來源／目錄 identity、精確區間、成對 fixtures 與錯誤規則；程式碼前先凍結。 | Core／Maintainer · **G2 審查中** |
| N10-02 · 純驗證 | 先觀察測試 RED、再實作 Rust 純驗證並取得 GREEN；不呼叫 filesystem／model／host，也不自動選素材。 | Engine · 接受 N10-01 後進入 G3 |
| N10-03 · CLI | 新增明確 opt-in 的本機唯讀入口、固定成功／錯誤 envelope 及契約測試。 | CLI · G3–G4 |
| N10-04 · 回歸 | N-02/N-04/N-08/N-11、Planner V1、video V1、OTIO、debug/release、fmt、Clippy、repository checks 及 PR exact-head CI。 | Reviewer · G4 |
| N10-05 · 真人與素材 | 作者交實際選定單元、真實目錄、精確片段及一處真實缺素材情況，再評估人工表格是否已足夠；雜湊、實際媒體、宿主匯入播放另列。 | Author／Maintainer · G4／後續 G5；目前沒有證據 |

N10-01 的 G2 契約接受前，不進入 G3。真人試用須保存原題、起始資料、作者原話、觀察及限制；合成預演不可替代。私人素材不進公開 repo。發布與遷移另經 G5 核准。

## 決策紀錄

- **已選 V（2026-11-12）：** 作者選兩個故事單元，一個宣告有影片、一個沒有素材；確認缺口仍明確可見，並阻擋可進入規劃／匯出狀態。採用 `NarrativeVideoCoverageV1`，只限影片、只檢查宣告、唯讀。
- **S 與 B：** 仍暫緩，不屬於 N-10。
- **D：** 是 2026-11-11 的 N-06 歷史決定；仍保留為當時結果，但後來的 V 決定只在 N-10 範圍內取代它。

模式選擇門檻已完成。#67 仍開啟，等待 G2 審查及契約凍結；尚無程式實作、真人驗收或媒體證據。若作者可用既有文字表格安全完成交接，或沒有具體單元／素材缺口，仍可取消新增原生模式。

**證據區分：** [既有工程探針](../narrative/n06-cases/evidence/probes.json)僅證明目前契約的行為；本提案未新增 Rust 測試結果、真人成效、真實 asset identity 或 CI 證據。
