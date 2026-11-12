<!-- SPDX-License-Identifier: MIT -->
# N-07／N-R07：試用處置與固定 native 版本證據

[English](n07-pilot.md) · [#54](https://github.com/bynanci/storytelling/issues/54) · G4–G6。

**維護者實際決定：Stop 提交的外部 wrapper 試用，不發布。** 2026-11-12（Asia/Taipei）的[處置紀錄](https://github.com/bynanci/storytelling/issues/54#issuecomment-6809828575)將 #54 以 not planned／stopped 關閉，接受 **0/5 主要完成、5/5 作者至少一項重大 U01 歸因錯誤**。這是提交產物的評分；維護者不要求這五位作者重做，也不把它判成 native Rust 缺陷或正式試用通過。

[已同意的 native 協議](https://github.com/bynanci/storytelling/issues/54#issuecomment-6796082010)固定 `n07-door-repair@1`、`ce73836caa86502933f538821fdbf5dac92c1224` 與 ReportV1。**該固定版本的已驗證可比較配對為 0；native 驗收仍為待真人證據。** 這項缺口與外部流程的實際 Stop 決定分開。[N-01 的 Continue](https://github.com/bynanci/storytelling/issues/48#issuecomment-6798756961)只適用於獨立 Skill 試用，不合併分析。

| 項目 | 已有證據與限制 |
| --- | --- |
| 提交分母 | 五位作者、十次報告的計時嘗試；三 AB、兩 BA。先前客戶用量題與不同維修情境留作補充，不累加人數。 |
| 主要完成與重大錯誤 | CLI 標示組 0/5 完成、5/5 作者有重大 U01 錯誤，至少五件；同一判斷在 log 與主張重複出現只算一次。取代早期暫收的五完成／一錯誤標籤，保留原錯答。 |
| 作者時間 | 報告中位數為筆記 390 秒、CLI 標示流程 190 秒，十次均低於 480 秒。51.3% 只是描述差異，不能推論固定 native 版本效益。 |
| 未知或未核實 | 筆記組評分、退出、手動重複輸入、重工、回饋及該輪程序時間。retry 次數不是重複事實輸入次數；缺漏留白。 |
| 建置與產物身分 | manifest 指向較後的 `489273b`，Rust tree 與固定 `ce73836` 不同。同版號與相容 ReportV1 不證明同一執行檔。 |
| 命令與 jq 追查 | native 不支援提交的 `storytelling process --scenario ...`；正常 `audit-copy-claims` 回傳 `ok`／`commercial_copy`，U01 引用為 missing。jq 從替代 envelope 複製既有字串，對保存的 native V1 會失敗。外部產生程序未辨明。 |
| 實際處置 | Stop／不發布，#54 關閉為 not planned；固定 native 驗收未獲證實。 |

依據：[評分](https://github.com/bynanci/storytelling/issues/54#issuecomment-6800753461)、[建置核對](https://github.com/bynanci/storytelling/issues/54#issuecomment-6800945586)、[命令與轉換追查](https://github.com/bynanci/storytelling/issues/54#issuecomment-6809399303)、[維護者實際處置](https://github.com/bynanci/storytelling/issues/54#issuecomment-6809828575)。公開摘要不重刊私人逐字答案或身分。

已核對 main `ce73836caa86502933f538821fdbf5dac92c1224`、已合併的 #52／PR #83 V1 及 #48 原判準。PR #87 後來已合併可選用的 V2。本輪仍採 V1。#53 為可選資料；其三者暫緩決定不阻擋 N-07。修門案例的單次請求者判讀沒有計時，不算作者對照樣本。

## 保留的主持人交接材料

- [試用包與重跑命令](../../examples/evaluation/n07-pilot/README.md)。
- [主持人逐步腳本與你需要做的最短操作](../../examples/evaluation/n07-pilot/protocol.zh-TW.md)。
- [參與者資料](../../examples/evaluation/n07-pilot/participant-kit/)與[主持人專用 key](../../examples/evaluation/n07-pilot/facilitator/key.md)，請分開發放。
- [私人紀錄表標頭](../../examples/evaluation/n07-pilot/forms/)：版本／環境、樣本與完成、作者時間、錯誤原句、重複輸入／重工、匿名回饋、獨立 CLI 時間及事前同意。

此包與既有分配表保留為歷史交接，沒有啟動新的招募。未來 native 評估須另訂版本並在執行前同意，使用新的作者與同詞原稿、三 AB／兩 BA、實際順序和開始／退出紀錄，依原規則處理五份可比較配對；未完成／timeout 不因失敗而剔除。維護者已採納的舊輪規則與實際 Stop 均保留。

兩臂使用同詞的衍生基準 `n07-door-repair@1`：來源單元與原 N-01／N-03 相同，構圖用語對齊 N-01，不帶原 N-03 未佐證的室內設定或 PR #88 的 C02。紅傘只在 A。這是明示的 native 格式調整，不假稱與原試用文字逐位元相同。不能把改角色／移傘的 N-04 示範當成原改名題。

native 作者自己建立文件 revision 2、更新 S01／S02 和精確引用、手改三份稿件文字，再實際 render；I01／C01 的 lineage metadata 因此改版，但文字、origin、範圍不變。工具不自動重寫或核准。文案查核仍由人判讀 S03 的一次測試與無依據 U01。主持人不得先填好 after-JSON。

作者計時包含讀取、JSON／metadata、思考、CLI 等待、恢復與保存。timeout=未完成，原門檻分析時間 480 秒。CLI 程序毫秒數另存，不能從作者秒數扣除；版本號編輯與「同一更正事實的重複輸入」也要分開。

## 原門檻與現況

原 #48：B 至少 4/5 完成、無 major、歸因不比 A 差，且時間中位數至少快 20%，或每位少一次重複輸入，或每位少一次歸因錯誤，才有 continue 訊號。少於 3/5、至少兩位 major 或完全沒有品質／時間／重複／歸因改善，為 stop；其他情況 narrow/repeat。小於 20% 的改善不等於完全無改善。

不足五份可比較的配對、缺關鍵欄或事前協議時，維持**待真人證據**。配對紀錄可以包含未完成／timeout，不是只分析兩臂成功者。原門檻的商業 U01 判讀保留；額外兩主張／兩提案的查核率另報，不更改原完成定義。

## 保留的工程實測，與真人提交分開

[完整結果](n07-pilot/engineering/results.json)有 11 個案例，每案一次不納入統計的暖機＋五次 process wall-clock。預期退出碼全部符合、重播一致、來源／manifest／binary 雜湊不變。原始 stdout/stderr、min／median／max 都保留。例：基準 render 中位數 1.088 ms、改版快照 1.147 ms、文案引用檢查 1.284 ms。這只是一台共享 Linux sandbox 的程序時間，含啟動/I/O；不代表作者更快或跨平台效能提升。[建置版本與限制](n07-pilot/build-provenance.json)。

工程預演實際找出：連續 S01／S02 source_revision 在第二筆遭既有 `revision.pending_review` 拒絕。[原始發現](n07-pilot/preflight-finding.json)及[合成重現](../../examples/evaluation/n07-pilot/facilitator/ledger-batch-pending.json)已保留；既有 engine 測試已保護該政策。主試用改走可執行的前後文件快照，不偽造中間 review，也不宣稱已完成原子合併日誌。

[既有草案 N07-UX01](../../examples/evaluation/n07-pilot/facilitator/N07-UX01.md)保留多單元更正／審核歷史的工程題，不是已證實的最大 UX 缺陷。已提交的 U01 錯誤是流程觀察；目前命令與產物不能建立可重現的 native 缺陷，維護者已停止 wrapper 追查。未來具名修正任務需先保存實際版本、匿名輸入、預期／實際和最小合成重現，再依行為修正記錄 RED/GREEN。

量測 helper 已先觀察 RED；獨立審查另發現並修正過深 JSON 的錯誤處理，補上 RED/GREEN。最終 8 項 helper 與 4 項材料測試全部通過。[技術審查](n07-pilot/technical-review.md)與[本機完整檢查](n07-pilot/local-checks.json)均保留。N-R07 對照、原始工程時程與所有驗證連結見 [English 報告](n07-pilot.md)。交付 head 的 CI 記於 PR。

**沒有 RC 或發布授權。** 本次同步文件，保留外部流程的 Stop 與固定 native 驗收缺口；SDK／JSON／CLI／digest／OTIO、依賴、原始輸入與既有輸出不變。準備期 observations.json／空白表單仍是歷史快照；[材料雜湊](n07-pilot/material-sha256.json)對應 [8e2c080 交付版](https://github.com/bynanci/storytelling/tree/8e2c08094f6bce70a7507e11ed84265bd98e3820/examples/evaluation/n07-pilot)，README 後續狀態更新不重寫該收據。回退只影響文件，不刪除原提交或維護者處置；未來評估及 G5 發布另案決定。
