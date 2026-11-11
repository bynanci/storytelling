<!-- SPDX-License-Identifier: MIT -->
# N-07／N-R07：對照試用準備與工程報告

[English](n07-pilot.md) · [#54](https://github.com/bynanci/storytelling/issues/54) · G4–G6。

**原生 CLI 試用。** 維護者已[採納正式協議](https://github.com/bynanci/storytelling/issues/54#issuecomment-6796082010)，使用 ReportV1 與修門題包 `n07-door-repair@1`。已收到的五組「普通筆記／CLI」配對來自客戶用量修訂題。[收件核對](https://github.com/bynanci/storytelling/issues/54#issuecomment-6798143556)確認這次提交中可納入正式修門題的配對為 0 組。這輪原生 CLI 的產品決定仍待修門紀錄。

可用現有格式私下交付五組配對。請附實際順序、題包及 CLI 版本、時間、完成情形、歸因錯誤與重複輸入。[原始表單](../../examples/evaluation/n07-pilot/forms/)另有重工與回饋欄位。缺漏欄位留白。[N-01 的繼續決定](https://github.com/bynanci/storytelling/issues/48#issuecomment-6798756961)適用於 Skill 試用。

已核對 main `ce73836caa86502933f538821fdbf5dac92c1224`、已合併的 #52／PR #83 V1 及 #48 原判準。PR #87 後來已合併可選用的 V2。本輪仍採 V1。#53 為可選資料；其三者暫緩決定不阻擋 N-07。修門案例的單次請求者判讀沒有計時，不算作者對照樣本。

## 可直接使用的材料

- [試用包與重跑命令](../../examples/evaluation/n07-pilot/README.md)。
- [主持人逐步腳本與你需要做的最短操作](../../examples/evaluation/n07-pilot/protocol.zh-TW.md)。
- [參與者資料](../../examples/evaluation/n07-pilot/participant-kit/)與[主持人專用 key](../../examples/evaluation/n07-pilot/facilitator/key.md)，請分開發放。
- [私人紀錄表標頭](../../examples/evaluation/n07-pilot/forms/)：版本／環境、樣本與完成、作者時間、錯誤原句、重複輸入／重工、匿名回饋、獨立 CLI 時間及事前同意。

這是另列的普通筆記／native CLI 配對輪，不合併原 Skill 代理輪或工程執行時間。預定五位新作者，隨機三 AB／兩 BA，每臂八分鐘；維護者已同意此輪沿用門檻，並採納歸因比較與退出規則。

兩臂使用同詞的衍生基準 `n07-door-repair@1`：來源單元與原 N-01／N-03 相同，構圖用語對齊 N-01，不帶原 N-03 未佐證的室內設定或 PR #88 的 C02。紅傘只在 A。這是明示的 native 格式調整，不假稱與原試用文字逐位元相同。不能把改角色／移傘的 N-04 示範當成原改名題。

native 作者自己建立文件 revision 2、更新 S01／S02 和精確引用、手改三份稿件文字，再實際 render；I01／C01 的 lineage metadata 因此改版，但文字、origin、範圍不變。工具不自動重寫或核准。文案查核仍由人判讀 S03 的一次測試與無依據 U01。主持人不得先填好 after-JSON。

作者計時包含讀取、JSON／metadata、思考、CLI 等待、恢復與保存。timeout=未完成，原門檻分析時間 480 秒。CLI 程序毫秒數另存，不能從作者秒數扣除；版本號編輯與「同一更正事實的重複輸入」也要分開。

## 原門檻與現況

原 #48：B 至少 4/5 完成、無 major、歸因不比 A 差，且時間中位數至少快 20%，或每位少一次重複輸入，或每位少一次歸因錯誤，才有 continue 訊號。少於 3/5、至少兩位 major 或完全沒有品質／時間／重複／歸因改善，為 stop；其他情況 narrow/repeat。小於 20% 的改善不等於完全無改善。

不足五份可比較的配對、缺關鍵欄或事前協議時，維持**待真人證據**。配對紀錄可以包含未完成／timeout，不是只分析兩臂成功者。原門檻的商業 U01 判讀保留；額外兩主張／兩提案的查核率另報，不更改原完成定義。

## 工程實測，沒有真人結果

[完整結果](n07-pilot/engineering/results.json)有 11 個案例，每案一次不納入統計的暖機＋五次 process wall-clock。預期退出碼全部符合、重播一致、來源／manifest／binary 雜湊不變。原始 stdout/stderr、min／median／max 都保留。例：基準 render 中位數 1.088 ms、改版快照 1.147 ms、文案引用檢查 1.284 ms。這只是一台共享 Linux sandbox 的程序時間，含啟動/I/O；不代表作者更快或跨平台效能提升。[建置版本與限制](n07-pilot/build-provenance.json)。

工程預演實際找出：連續 S01／S02 source_revision 在第二筆遭既有 `revision.pending_review` 拒絕。[原始發現](n07-pilot/preflight-finding.json)及[合成重現](../../examples/evaluation/n07-pilot/facilitator/ledger-batch-pending.json)已保留；既有 engine 測試已保護該政策。主試用改走可執行的前後文件快照，不偽造中間 review，也不宣稱已完成原子合併日誌。

[下一題草案 N07-UX01](../../examples/evaluation/n07-pilot/facilitator/N07-UX01.md)先定義保留歷史的多單元更正／審核邊界。這是值得先設計的工程問題，不是已量測的最大 UX 缺陷。無真人失敗可轉回歸；腳本已規定將日後匿名輸入、預期／實際、版本與錯誤原句變成最小合成案例。

量測 helper 已先觀察 RED；獨立審查另發現並修正過深 JSON 的錯誤處理，補上 RED/GREEN。最終 8 項 helper 與 4 項材料測試全部通過。[技術審查](n07-pilot/technical-review.md)與[本機完整檢查](n07-pilot/local-checks.json)均保留。N-R07 對照、原始工程時程與所有驗證連結見 [English 報告](n07-pilot.md)。交付 head 的 CI 記於 PR。

沒有正式發布建議或授權。SDK／JSON／CLI 舊預設、digest、OTIO 與依賴不變；本次只有試用材料、寫入新資料夾的量測 helper 及其 CI。回退只需停止使用；無資料遷移。原生 CLI 的修門配對資料仍待核對，亦須評估學習效應、來源語意、版本欄位負擔與多來源護欄；正式 RC／發布由維護者依 G5 另行決定。
