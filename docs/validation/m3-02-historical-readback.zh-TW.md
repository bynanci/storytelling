<!-- SPDX-License-Identifier: MIT -->
# M3-02 歷史 readback 收據重送回歸

[English / 詳細收據](m3-02-historical-readback.md) · M3-JL02／JL04／JL06 · 相關 [#105](https://github.com/bynanci/storytelling/issues/105)

Base 為 `91bf8f80139f59c0406340eb3565d296bd562931`。獨立提示的 AI 日誌審查提出失敗路徑，主代理用無宿主測試重現。這是合成技術證據，不是真人或 FilmCraft 觀察。

原程式只與最新 attempt 狀態／雜湊比較。相同有效 lease／attempt 下，`Unknown(D) → NotApplied(E) → Ambiguous(F) → 重送 NotApplied(E)` 會覆蓋較新的 unknown、追加事件、花掉 readback 次數，並讓下一次操作成為 `Ok(2)`。[RED](m3-02-historical-readback/red.log)保留此實際結果，違反既有「重複收據冪等／較新矛盾讀回阻擋重試」契約。

修正會先檢查當前 lease、fence 與 attempt，再在同一 SQLite immediate transaction 內查找相同 operation／attempt 下已記錄的 readback digest。已消耗收據回傳**當前 snapshot**，不改狀態、歷史或預算；跨重啟、較新觀察、terminal 及預算已滿仍適用。response 與 readback 種類分開，不能因 response 同雜湊漏掉首次 readback。新 evidence 仍走原狀態機，未變更 schema、公開型別、錯誤、依賴或上限。

測試先行 commit `c55d5b2`：17 項中 14 通過／3 失敗。實作 `4dee6e8` 的 [GREEN](m3-02-historical-readback/green.log)為 17／17 通過。新增四項測試涵蓋重啟後舊收據不能解鎖重試、八種後續狀態組合、四次 readback 預算、response／readback 同 hash 區分，以及 operation／attempt／lease 隔離。[Workspace 收據](m3-02-historical-readback/workspace-checks.json)保留精確 source、獨占 target、指令、退出碼與原始 log；最終 hosted 結果另記於 PR。

同一 attempt 的 readback hash 代表已消耗的不可變證據。改送較晚 timestamp 或另一個 result 參數，不會使舊 hash 成為新觀察。呼叫者必須看回傳的當前 snapshot；`Ok` 不是接受新 result 的證明。新的觀察必須攜帶可區分的新證據 bytes／digest；只有專案 bytes 沒變，不能證明舊請求已停止或可安全重試。

v1 事件不保存歷史 result subtype，因此不能驗證同一舊 hash 的 subtype 是否相同；此修正採已消耗證據 no-op。它不認證 caller 的 NotApplied、Host 真實狀態、當前權限或 in-flight 請求已停止。#103–#106 的 adapter／authority／實際宿主驗收仍待完成。本次沒有 Host 呼叫、重試、Undo 或真實專案操作。

資料庫不須 migration，既有事件不改寫；舊版本若已被重送改變狀態，本修正無法重建或自動修復，疑似受影響操作須由操作者檢視後再決定。回退會恢復此缺陷；unknown 應繼續阻擋派送，而不是降版當復原。獨立 AI review 與最終 head／合併後 CI 見 PR；G2、正式 M3 與 G5 未由本測試取代。
