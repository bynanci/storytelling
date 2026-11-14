<!-- SPDX-License-Identifier: MIT -->
# N10-06B · 唯讀影片對照預檢提案

[English](README.md) · 繁體中文 · [Issue #129](https://github.com/bynanci/storytelling/issues/129)

**狀態：G2 設計提案；實作與格式接受等待維護者聚焦決定。** 本資料包只處理 B，沒有新增可執行命令、runtime DTO、公開 Schema 或已實作行為。設計分支以 A 的 PR #130 head `59516f24f4d829a3d695c115d2f94e208a78cdea` 為 base；A 起始盤點的 main 是 `52dc6c315f7b3d20693f3c51e052509997ab791b`。實作前仍須重讀最新 main 與 PR #130 結果；這個 base 紀錄沒有宣稱 A 已合併。

A 的 [draft-only G2 決定](../../validation/n10-06a-g2-decision.md)與 [ADR 0012](../../adr/0012-draft-exact-video-range-input.md)已定義精確秒數／影格轉換及 helper 錯誤。B 只提議將同一份輸入做成可讀預覽。[既有 V1 驗證器](../../adr/0010-narrative-video-coverage-v1.md)繼續決定宣告是否有效，21 個診斷碼與 first-error 順序完整保留。未來 SDK、TypeScript 與 Reference App 由 #107–#109 決定；本提案不建置 App 或 transport。

## G0／G1：作者任務與不可破壞的條件

作者依序選擇 `S02@1`、`S03@1`，將 S02 用 IN／OUT 對應至宣告素材，S03 保留 unresolved。預覽必須依原順序顯示兩列、原時間單位與數字、精確轉換後的秒數、宣告素材身分／片長、固定 JSONPath 及缺素材原因。多處輸入錯誤可以顯示多個**非權威輸入提示**，同時只顯示既有 A／V1 全文件流程實際回傳的權威結果。預覽不得修正輸入、刪掉選定單元、夾限區間，或將宣告升格為實體檔案／編輯器證據。

[合成情境](scenarios.json)是預先固定的候選斷言，不是操作觀察。案例涵蓋完整／缺素材輸入、多個 helper 錯誤、先過期版本再區間錯誤、缺列／順序不同、VFR 及原始 JSON 重複鍵。後續真人操作使用[空白證據表](evidence-form.blank.json)；所有實測結果欄都保留 null。

## 最小互動提案

候選入口只輸出文字預覽：

```text
storytelling preflight-video-coverage-input <FILE|-> [--lang zh-TW|en]
```

本資料包**尚未實作此命令**。G2 須先接受命令、繁中預設標籤、opt-in `draft-video-input` feature、輸出順序、提示邊界與 exit codes，才能進 G3。輸入沿用 `n10-06a.1-draft`，不增加第二套文法、V1 Schema、穩定結果 DTO 或可供程式剖析的文字契約。未來 client 不得將此文字輸出當成 SDK API。

此命令只預覽、不寫檔。A 保持唯一的正規化／輸出命令：只有最終 V1 驗證通過，作者才可明確要求 A 的既有 `--emit-v1`。B 不新增同名旗標，也不能讓無效輸入變成可輸出。有效但 unresolved 的文件仍可列印為 **V1 宣告 JSON**，但不可進入規劃，也不能標示可匯出。

候選 exits 沿用 A 慣例：完整或 unresolved 的有效宣告為 0；A helper／V1 拒絕為 1；usage 為 2；明確輸入／輸出 I/O 錯誤為 3。即使提示指出其他欄位，預覽仍須呈現實際的第一個錯誤。不增加 V1 碼，也不能將拒絕改成 exit 0。

## 先顯示權威結果，再顯示提示

先對完整原始 bytes 呼叫未改變的 `normalize_draft_video_coverage`。其結果決定權威區塊：

| A／V1 結果 | 必須顯示的權威區塊 | 能否經 A 輸出 V1 JSON |
| --- | --- | --- |
| byte／UTF-8／語法／重複鍵／深度失敗 | A helper 拒絕；顯示原 code／path。不得由歧義 JSON 建立列或提示。V1 未執行。 | 否 |
| helper 轉換／shape 失敗 | A helper 拒絕；顯示原 code／path，V1 未執行。安全解碼的列與 helper 提示可放在後面。 | 否 |
| 轉換成功、V1 拒絕 | V1 拒絕；顯示原本唯一的 V1 code／path，不將稍後可能出現的錯誤列為實際診斷。 | 否 |
| V1 有效且含 unresolved | 宣告輸入有效；缺素材仍阻擋規劃。顯示原結果診斷。 | 是；只限宣告 JSON |
| V1 有效且全部 mapped | **宣告可進入規劃／Declarations can enter planning**。 | 是；只限宣告 JSON |

提示收集器必須服從全文件結果。時間提示重用 A 的精確文法／算法，不另寫轉換器或第二套引用／區間驗證器。可以提示不支援的秒數／影格語法、無效 fps／timing 或明顯缺少 mapping 列，也可以將宣告 OUT 與宣告片長並列供作者核對。不得補造 V1 尚未回報的 `video_coverage.invalid_range`、revision、coverage 或順序診斷。缺列的非權威描述不能取代 V1 的缺 mapping 錯誤。

例如 `S02@2` 對上 revision 1 的 narrative，同時 OUT=6、宣告片長=5 時，必須先顯示實際 stale-unit 錯誤。精確 OUT 與片長仍可見；作者只修正引用後，下一次執行才可能看到實際 V1 區間錯誤。不能把兩次執行合成虛構的 V1 多錯誤回應。

## 列投影與文字順序

固定呈現順序：

1. 權威區塊：階段、狀態，以及實際唯一錯誤或有效結果診斷。
2. 範圍說明：只檢查宣告；檔案位元組／metadata、作者內容核對、宿主支援與執行權限需要各自證據。
3. 依輸入順序顯示**每個**選定引用，包含無效或重複引用。保留原 `unit_id@revision`、selection index 與所有完全相符的 mapping indices，不去重、不排序、不自動修 revision。
4. 未對應選定引用的 mapping 另依原 index 列出，不能先重排輸入再驗證。
5. 明標「非權威」的輸入提示，依原列／欄位順序排列。
6. A 是否可輸出有效 V1 宣告 JSON；規劃就緒另列，沒有 Execute 動作。

Mapped 列顯示原 `seconds`／`frames` 單位、IN 包含／OUT 不包含、原字串、明確 fps／timing、可精確換算的秒數、asset ID／revision 與 catalog 宣告片長。全文件被拒絕時，單欄位轉換須標成僅供顯示的暫定值；無法解釋的時間不得顯示精確結果。Unresolved 列保留狀態與原原因，IN／OUT 留空。沒有 mapping 的選定引用須顯示缺列，不能自動新增 unresolved 列。

時間欄位在可用時同時列出兩個位置：

```text
Input:  $.coverage.mappings[0].source_range.start
Output: $.mappings[0].source_range.start
IN:     "2.5" seconds = {"numerator":"5","denominator":"2"} seconds
OUT:    "4.5" seconds = {"numerator":"9","denominator":"2"} seconds (exclusive)
```

`unit_id`、revision 與原因以 quoted／escaped data 顯示。控制字元與 terminal escape 必須轉義；不得將無效 ID、未知 key、parser 訊息、隱藏路徑或任意來源文字直接寫進終端控制文字。錯誤路徑只用固定／索引 JSONPath，不回顯未知欄位及值。原始有界輸入保持不變。

### 僅供設計檢視的預覽

以下是作者撰寫的設計文字，**不是命令擷取結果**：

```text
宣告預檢：有效；缺素材單元仍阻擋規劃
範圍：僅檢查宣告；實體素材與剪輯器尚未驗證
1. "S02"@1 · mapped · mapping[0] · asset "CLIP-01"@1
   IN  "2.5" seconds = 5/2 seconds
   OUT "4.5" seconds = 9/2 seconds [exclusive]
   $.coverage.mappings[0].source_range → $.mappings[0].source_range
2. "S03"@1 · unresolved · mapping[1]
   Reason: "No video asset was selected for this unit."
V1: video_coverage.unresolved_required @ $.mappings[1]
V1 宣告 JSON：可由 A 明確輸出；宣告不可進入規劃
```

## G2 決定後的實作邊界

| 層級 | 候選責任 |
| --- | --- |
| Core | V1 DTO、精確時間型別、wire 欄位與診斷不變。 |
| Engine | 純函式、opt-in、私有預檢投影，重用既有 raw JSON preflight、A 精確轉換與全文件 A／V1 結果；沒有 I/O、媒體 probe、網路、資料庫、模型或 host 呼叫。可整理內部函式供提示重用，但 A 原公開輸出／錯誤順序須逐 bytes 維持一致。 |
| CLI | 明確有界的 manifest／stdin 讀取、語言選擇、確定性且已轉義的文字輸出；A 命令不變，不預設寫檔。 |
| 未來 consumer | 公共 facade 由 #107 決定；#108 transport 與 #109 App 不能把這份文字預覽當成已接受 API。 |

不得把輸入拆成各自有效的小文件，再推論全文件有效。無效 coverage 不輸出 V1 payload。沿用 A raw 1 MiB／depth 34 與 V1 獨立 1 MiB／depth 33 上限；budget／syntax 失敗後不得進行提示遍歷。Parser 或提示不得先刪除未知 key，再讓最終驗證器看到較弱的輸入。

精確 Rust 投影 signature 與 CLI 文字契約均待聚焦 G2。Scenario JSON 沒有凍結任何新公開 DTO／Schema 或穩定提示 ID；scenario IDs 只用於測試規格。

## G3／G4 計畫與剩餘證據

A 經審查／合併、B 有自己的聚焦 G2 後，先為固定情境寫正常入口測試，保留實際 RED 輸出／commit，再實作最小純投影並保留 GREEN 與實際輸出。檢查：

- selection／mapping indices、原時間字串、unit／revision 順序及 unresolved 原因；
- 多提示與未改動的首個 A／V1 結果並存；後續 V1 錯誤只在新執行出現；
- 不輸出部分 V1、不夾限／補列／去重，文字安全轉義，不回顯未知 key／值；
- file／stdin／語言重播確定、輸入與目錄 sentinel 不變，PATH 無媒體工具也能執行；
- A 原有 JSON／錯誤輸出與七項已接受 V1 CLI 回歸保持不變；
- workspace default／opt-in debug／release、fmt、Clippy、rustdoc、repository checks 及最終 head Windows／Linux／macOS CI。

目前沒有實際獨立作者操作。操作發生後才記錄原題、來源／候選版本、起始資料、實際錯誤／行動、完成、耗時及逐字回饋。合成斷言及示意文字不能填入真人結果，也不證明省時。空白表分開保留機器重播、作者、媒體、host、維護者及 release 證據。

G2 須先決定唯一的 preview 命令／feature／語言／exits、輸出順序／escaping、提示界線、無效輸入輸出規則與私有重用方式。若文字表格已足夠，就維持文字；任何隱藏 V1 首錯、靜默修正 coverage、標示實體檔案已驗證或啟用 host 執行的方案，都應停止。移除 B 不需遷移 A／V1 輸入。C 本機 sidecar／威脅模型、D 五狀態 dry-run、#103／PR #117、#104–#106 與 G5 各自待決定。

本資料包為新寫 MIT 設計材料；合成來源依上述 base 的 A fixtures，沒有匯入外部程式、私人路徑、真實媒體或參與者紀錄。

目前 A 已合併為 `125e5b1bade5be2d406fc33de67773873d2a07f5`；這不構成 B 的 G2 接受，B 實作仍待聚焦決定。
