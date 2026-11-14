<!-- SPDX-License-Identifier: MIT -->
# N10-06D · 故事至影片交接的 dry-run 檢查表

[English](README.md) · 繁體中文 · [Issue #129](https://github.com/bynanci/storytelling/issues/129)

**狀態：證據檢查表／驗收規格提案，沒有 runtime API 或剪輯器整合。** 準備 base 為 `52dc6c315f7b3d20693f3c51e052509997ab791b`；PR 狀態於 2026-11-14 UTC 再查，當時觀察到 main 為 `125e5b1bade5be2d406fc33de67773873d2a07f5`。作者與審閱者可用此資料包列出未來交接前缺少的證據；檢查表不產生執行權限。

## 範圍與目前門檻

沿用既有 [NarrativeVideoCoverageV1](../../adr/0010-narrative-video-coverage-v1.md) 與 [V1 Schema](../../../contracts/narrative-video-coverage-v1.schema.json)。A 的精確輸入輔助層另在 [PR #130](https://github.com/bynanci/storytelling/pull/130)；B 的提示體驗及 C 的可信本機素材 adapter 各有自己的決定。D 記錄它們的證據，不重新實作 coverage 驗證器。

本次核對時，[#103](https://github.com/bynanci/storytelling/issues/103) 為 Open，技術 G2 決定尚待接受。[PR #117](https://github.com/bynanci/storytelling/pull/117) 在 `aa6b4cf6bfe86295ad43ad1813e8057130d6d1d1` 完成審查，於 2026-11-14 以 `a23a2909cf3734d207c66b372f29a85c5e541c86` 合併，**範圍僅提案文件與設計完整性檢查**。Schema 檢查或文件 merge 不代表 authority 契約已接受；W01 的 deterministic／no-host 契約實作及維護者針對實測 commit 的決定仍須另外完成。

[#104](https://github.com/bynanci/storytelling/issues/104) 受阻於 #103；[#105](https://github.com/bynanci/storytelling/issues/105) 的復原實作受阻於 #104；[#106](https://github.com/bynanci/storytelling/issues/106) 須先驗證 #104／#105，取得真實宿主證據，再由維護者記錄 accept／narrow／stop。目前沒有一般正式環境已接受的 FilmCraft profile。[M3-03 限定證據](../../validation/m3-03-filmcraft-execution.md)包含一次隔離匯入／回讀及遺失回應，不涵蓋正式行程監督、當前權限或所有中斷路徑。

D 目前交付的是**仍有阻擋項目的審閱資料包**。不得加入 Execute／可匯出動作、呼叫宿主、建立新 operation、編譯產物、修改 Catalog／專案、重送 unknown 操作或執行 Undo。

## 五種獨立證據狀態

逐項記錄，不互相推導。`passed`、`failed`、`not_verified`、`unknown`、`blocked` 是人工檢查表標籤，**不是新 SDK enum 或 V1 診斷碼**。JSON 自稱通過時，審閱者仍須核對證據；此表不能建立不可偽造的 permit。

| 狀態 ID | 若要記錄通過，須有什麼實際證據 | 不能由此證明什麼 |
| --- | --- | --- |
| `declared_valid` | 精確輸入位元組／雜湊、Storytelling commit、實際 `render-video-coverage` 命令／exit／輸出、目前有序的 `unit_id@revision`、Catalog revision、素材／區間宣告，並保留 unresolved 列。 | 檔案存在、實際片長、畫面內容、Host 支援、批准或匯出。有效 unresolved 可通過宣告合法性，仍須保留 `ready_for_planning=false`。 |
| `media_probed` | 由另經接受的 C adapter，將每個 mapped `asset_id@revision` 綁到使用者明選的本機檔案、已驗證的位元組指紋、有界 metadata／串流／時間證據及區間相容性；保留改檔、替換及競態結果。 | 畫面符合敘事單元，或剪輯器實際用了相同位元組。Nominal fps 不能解決 VFR 影格時間戳；缺檔或無可靠綁定時保留 `not_verified`／`unknown`。 |
| `content_reviewed` | 真實作者檢視已識別的位元組與選定區間，以匿名代碼、日期、證據引用及原話記錄針對哪個單元／revision 確認什麼；列出未解判斷及範圍。 | 僅從檔名／縮圖推斷素材身分、組織執行權限、審美優越或未看過的畫面／聲音驗收。Metadata／hash 不會補足此項。 |
| `host_qualified` | 精確且已接受的 Host／profile／build／OS 能力及損失、已測試的目標專案身分與當前 revision，以及經 #104–#106 接受的匯入／回讀／儲存／重開證據。 | 特定 operation 已套用，或目前有權執行。Profile 提案或舊隔離測試不足以通過。 |
| `authority_accepted` | 已接受的 #103 界線與可信當前 provider／policy 紀錄，綁定精確的已審閱／編譯產物、目標 revision、Host、操作者批准及 operation 身分；未來 executor 須在實際 dispatch 邊界驗證有效性。 | 僅從數學有效簽章推斷真人身分或目前授權。宣告或勾選不能產生批准紀錄、消耗批准、取得 ownership 或 dispatch。 |

另須獨立保存**審閱與編譯的綁定**：選定 proposal、來源 snapshot、compiler／profile／版本、精確 artifact bytes 及已接受的轉換損失 report，必須和審閱身分一致。此表可引用既有已驗證紀錄，不會因接受呼叫者填入的雜湊而建立這項綁定。缺少編譯證據時維持 `not_verified`；即使宣告及簽章都通過，仍阻擋未來執行審閱。

簽章可能在呼叫者提供的金鑰下數學有效，但簽署者並無目前可信權限；可信批准也可能是針對與實際編譯提案不同的產物。各自保留來源與日期。

## 人工 dry-run 步驟

1. 將 [`checklist.blank.json`](checklist.blank.json) 複製到主持人經授權的私人工作區。填匿名 case／reviewer code、source commit、日期、來源／變體 revision 及精確選取順序。未知欄位留 null，每個選定單元都須可見。
2. 只對提供的結構化輸入執行現有宣告檢查。實際輸入／輸出／exit 私下保存；表內填不透明 receipt ID 與 hash。保留 `ready_for_planning` 與 unresolved 引用，不轉成執行狀態。不得手填虛構成功收據。
3. 對每個 mapped 列列出原 IN／OUT 單位、canonical 精確秒數、宣告 asset revision／duration，以及另外提供的 C 證據。沒有已接受 C 與實際授權觀察時，`media_probed=not_verified`；mapping 不會授權路徑或檔案存取。保留 unresolved 原因，不以猜測片段補缺。
4. 有素材時，請作者只檢視明確授權且已識別的內容。保留作者原話及不確定處；只對相符的單元／區間／位元組記錄 `content_reviewed`。本資料包沒有執行觀看或招募。
5. 分別核對 Host／目標驗收、authority 及編譯證據。依目前 #103–#106 門檻，Host／authority 維持 `blocked`。提案 Schema、已簽身分或已編譯 OTIO 都不代表目前有權或宿主已接受。
6. 產生阻擋清單，列出 owner、缺少證據、受影響單元／素材／revision 與恢復條件。完整表單仍是**審閱資料包**；即使未來五項都實際通過，D 也沒有 dispatch 動作。

此階段只使用明確 manifest 與既有經授權觀察。媒體讀取、metadata probe、外部命令、目標檢查及 protected authority 查詢，各須已接受的介面與本機權限。私人路徑、可識別人的檔名、影片、敏感原始證據及簽署者細節不可提交到公開儲存庫；公開範例只用不透明 evidence ID。此表是本機人工紀錄提案，不是已接受的可攜 wire format。

## 改動後，按證據範圍失效

| 改動 | 必須採取的處理 |
| --- | --- |
| 單元／revision、選取順序、mapping、剪點、Catalog revision 或宣告片長改動 | 重跑原 V1，保留舊輸入／輸出為歷史；重查編譯／審閱綁定及受影響的媒體／內容證據。 |
| 位元組、asset revision、觀察到的串流／時間戳、本機檔案身分或已接受的轉換損失改動 | 使受影響媒體／內容與審閱／編譯證據失效；保留舊證據，要求新的授權觀察與作者決定。 |
| Host／profile／build／OS、目標專案身分或 target revision 改動 | 此目標的 Host 驗收改為 `not_verified`；目標綁定的審閱／批准須重查。 |
| 批准到期／撤回、policy／provider 或執行身分改動 | 可信已接受 provider 提供當前紀錄前，authority 為 `unknown`／`blocked`；舊簽章或批准不能自行刷新。 |
| 先前 operation 為 `unknown`／`partial`、仍有執行／排隊工作，或舊 owner 未解 | 保留 quarantine，換新 operation ID 或新批准也不得另寫；使用已接受的復原 gate，不重送或 Undo。 |

不得用晚到回應覆蓋舊觀察，或原地更換其 source revision。新增帶日期的 checklist revision，保留原始 receipt。D 不會替 SQLite journal 新增狀態轉換。

## 驗收案例及復原設計重用

[`acceptance-cases.json`](acceptance-cases.json) 固定合成的預期審閱結果。[`checklist.synthetic-reference.json`](checklist.synthetic-reference.json) 是依既有雙單元 fixture 填寫的教學範例，不是已觀察操作。其宣告預期為有效／unresolved；所有實際觀察欄維持未驗證，整體為 blocked。真實表單的實測欄留 null。

| 案例 | 預期審閱結果／所需證據 |
| --- | --- |
| D01 · 有效宣告但一個單元 unresolved | 保留缺件單元與原因；宣告合法性可通過，planning false，交接維持 blocked。 |
| D02 · 完整宣告但沒有檔案證據 | 只顯示宣告可進規劃；`media_probed=not_verified`，不標示媒體已驗證或可匯出。 |
| D03 · 相同 asset ID 下位元組改動 | 舊 hash／作者內容確認不可重用；媒體／審閱證據失效，維持 blocked。 |
| D04 · 簽章有效但缺當前可信授權 | 只記數學有效性；authority 仍 blocked。 |
| D05 · 審閱 proposal A，卻提供 artifact B | 不將呼叫者 hash 提升為 compiler binding；即使簽章有效仍阻擋。 |
| D06 · Host／profile／target revision 改動 | 重查 Host／target 及目標綁定的審閱／批准。 |
| D07–D10 · 遺失回應、部分套用、缺少 quiescence、舊回應晚到 | 保留 unknown／partial 與 quarantine；不重送、Undo 或自動重試。 |
| D11 · 假設未來證據完整 | 只能標示 `review_packet_complete`；D 沒有執行動作，實際執行仍須另經接受的邊界。 |

重用 [PR #117 既有案例](https://github.com/bynanci/storytelling/blob/aa6b4cf6bfe86295ad43ad1813e8057130d6d1d1/tests/fixtures/execution-contract-v1/cases.json)：EX14（新批准不能解除 unknown）、EX16（遺失回應與完整相關回讀）、EX19（partial）、EX20（baseline 與正向 quiescence）、EX21（晚到 owner／attempt）、EX37（實際來源位元組改變）。這些仍是未來 runtime 的斷言提案；Python 設計檢查不提供 Host／復原執行證據。D 不重複實作引擎，也不標記上述測試已實作。

逾時／遺失回應時，列出最後持久化的 operation／attempt receipt 及不可變身分，結果記為 unknown。未來已接受的 #105 復原流程須有相關回讀及先前請求已靜止的正向證據，才可判定 applied／not_applied；舊或排隊工作仍可能執行時，單看起來相符的回讀不夠。確認 not_applied 也只可能在已接受重試規則、重新核對來源／審閱／權限及剩餘次數後，由操作者明確發起新動作，不能自動重送。目前 D 不執行這些動作。

## 檢查、決定及停止條件

本交付只改文件與合成表單。JSON 解析、連結／repository 及 whitespace 檢查驗證資料包結構；沒有 Rust 行為改動，因此不宣稱行為 RED／GREEN。Review 意見與後續作者／媒體／Host 證據分開記錄；資料包或 merge 不關閉 #103–#106／#129，也不提供 G5 決定。

驗收此資料包時，審閱者須能逐項指出缺少的 lane 及受影響單元、區分有效宣告與缺素材、保留原始來源／區間／決定，並解釋簽章或過期 receipt 為何不能啟用 operation。若以一項證據推導另一項、過濾缺件列、公開私人路徑、允許 unknown 重試，或 dry-run 標籤暗示已執行 Host，應停止或縮小。維護者仍須分別接受公共 consumer 介面（#107–#109）、C 契約／隱私範圍、#103／W01 實測契約、#104／#105 行為及 #106 宿主證據。尚無獨立作者操作，不宣稱使用性或省時結果。
