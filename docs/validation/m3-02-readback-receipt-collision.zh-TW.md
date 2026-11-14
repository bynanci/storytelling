# M3-02 首次回讀紀錄回歸

[English](m3-02-readback-receipt-collision.md) · 繁體中文

此維護修正維持已接受的 M3-JL02／M3-JL06 API 與 SQLite schema。
起點為 `c562ce0d06b7330813fa42cad739f24f4a49e9da`。不新增宿主呼叫、
執行權限、W01 實作或 #103 G2 核准。

## 重現與修正

準備操作、取得租約並開始第 1 次嘗試。以摘要 D 記錄 `Unknown` 回應，
再以同一 D 記錄首次 `Ambiguous` 回讀。兩者都對應 `outcome_unknown`，
但回應與回讀是不同種類的收據。

修正前，首次回讀回傳成功，歷史卻只有 `Prepared`、`AttemptStarted` 與
`ResponseRecorded`，沒有持久化 `ReadbackRecorded`，也未消耗四次回讀中的
一次額度。再送入四次不同回讀仍成功，合計接受五次觀察。

修正後，回讀冪等捷徑另要求 `readback_count > 0`。首次回讀一定留下事件並
消耗額度；同一回讀重送仍冪等，重新開啟資料庫後亦然。未知結果仍禁止重試。
第五次不同觀察回傳 `journal.limit_exceeded`，歷史保持不變。

## RED／GREEN 證據

[機器收據](m3-02-readback-receipt-collision/red-green-receipts.json)、
[RED 原始輸出](m3-02-readback-receipt-collision/red.log)及
[GREEN 原始輸出](m3-02-readback-receipt-collision/green.log)保存實際執行。
[Workspace 收據](m3-02-readback-receipt-collision/workspace-checks.json)分開記錄
容量恢復後的通過結果與首次基礎設施失敗。

| 階段 | 實際結果 |
| --- | --- |
| RED：`182822797470c812d7c1238d67d0fd443c19b953` | 11 通過、2 失敗，分別為缺少事件及未消耗額度 |
| 修正：`dd38313e96906aad835fc975e63665599bc44ce9` | 相同實作位元組已在本機測試：13 通過、0 失敗 |
| 獨立 AI 靜態審查 | 未發現阻擋項；fence／租約／attempt 檢查仍先執行，不構成人工驗收 |
| Workspace 重跑 | debug／release 各 312 通過；全功能 debug／release 各 331 通過；每次 3 項忽略。首次磁碟不足已排除，fmt、Clippy、rustdoc 與儲存庫檢查均通過 |
| Hosted CI | 待最終 PR head 的必要檢查通過後才可合併 |

可重跑命令見[英文紀錄](m3-02-readback-receipt-collision.md#test-first-evidence)。
兩個新增測試分別驗證回讀事件、重新開啟與冪等重送，以及四次額度和第五次
失敗不留下部分寫入。

## 相容性與限制

不改 DTO、診斷碼、資料庫 schema、依賴或重試狀態轉換，既有資料庫無須遷移。
此修正無法重建舊程式漏記的歷史回讀。若尚未記錄後續回讀，再次提交同一回讀
會新增缺少的首次觀察；不得虛構原先的時間或宿主觀察。回退會重新引入紀錄缺陷，
應保留資料庫。

測試只使用合成的暫存 SQLite，未驗證真實宿主、媒體身分、簽署授權、真人
核准或 G5 發布。#103／W01 與生產宿主驗收仍須分別處理。
