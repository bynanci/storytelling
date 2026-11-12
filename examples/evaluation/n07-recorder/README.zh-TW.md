<!-- SPDX-License-Identifier: MIT -->
# 原生試用私人紀錄工具 V1

[完整操作命令與限制（English）](README.md) · [v3 試用協議](../../../docs/validation/n07-native-v3.zh-TW.md)

這是 **N07-REC01／N-R07** 的本機工具，使用 Python 3.10+ 標準函式庫與 SQLite。它保存抽籤、篩查、開始、原生 CLI 呼叫、原始輸出和私人附件；真人完成評分、可比較配對及維護者決定仍須實際判讀。#54 維持 Stop，準備工具不代表採用試用或 G5 發布。

## 最短路徑

1. 先執行 v3 文件的固定版本建置，取得 `N07_RUN` 下的 ce73836 執行檔與 build-receipt.json。依英文操作建立新的 `--mode rehearsal` 私人目錄。工具一次產生 3 AB／2 BA 分配，保存 seed，拒絕覆寫；演練不能改名成真人輪次。
2. 在私人 screening.json 記匿名主持人碼、帶時區日期、曝光回答及 CLI／JSON 經驗，執行 `enroll`。有曝光仍保存篩查，但拒絕開始。`workspace` 產生 A、B 各自的新材料副本，驗證原材料雜湊；兩臂 shared 同詞，只有 B 多出 CLI 模板，不複製 key 或更正答案。
3. 先依分配表決定順序。讀完修改卡後啟動 `session`，保持程序開啟；作者可在另一個視窗改稿，計時包含紀錄工具、metadata、CLI 等待、修復及保存。主持人的分配表及 repo 不給作者看。
4. session 提示符使用 `render ["檔案路徑"]`、`audit ["文案路徑","ledger路徑"]`、`submit ["稿件","回答",...]`；這裡是 JSON 陣列，不是 shell，不能寫 `$N07_PRIVATE` 當展開變數。Windows 可用 `/`。B 呼叫原生執行檔，先保存輸入副本與開始紀錄，再執行；`stdout_path` 可開啟原樣結果。不經 jq，不改寫 ReportV1。A 禁止呼叫 CLI。
5. 交付時保存實際檔案，產生的 CLI 輸出已自動保留；多次呼叫時在回答或評分欄標明最後採用的 call ID。`submitted` 只代表已收到檔案，不是主要完成；未交齊、未改對仍須記失敗。480 秒記 timeout／未完成，分析時間 480，原始經過時間另存。
6. 私人填寫既有 sessions／errors／feedback／cli-calls 表，記開始、退出、完成分母、歸因原句與 major、重複輸入、重工及逐字回饋；用 `attach` 保存原始 bytes，再 `export` 到新的私人目錄。未知留空，重試不當成重複輸入。工具不會自動推論配對或決定 Continue。

計時器不能阻止外部編輯器繼續修改。timeout 時主持人須立即停止編輯、用 `attach --kind submission` 封存部分稿；其時間是實際附件時間，不能宣稱工具自動取得截止瞬間快照。原生呼叫各限最多 60 秒或剩餘作者時間；單次 call timeout 不等於整個作者 timeout，沒有自動重送。

## 中斷、替補與真人使用

EOF、Ctrl-C、時鐘異常或程序崩潰不能補成正常完成。既有開始／unknown call 留在資料庫；確認舊程序停止後，執行 `recover --session ID --reason "實際原因"`，追加 interrupted、保留未知時間及原始部分輸出，不重開計時、不重送 CLI。晚到結果只追加為 late，不覆蓋已結束場次；recover 本身不會終止仍存活的程序。

替補使用同一 slot 與 `--replacement-for`，只允許已退出／中斷者，最多五次；不可替換已記錄的 timeout 或低分以改善結果。先依協議人工判定五份可比較配對是否已達成；工具不從附件數量自動判定。每次開始與退出都留在 all-started 分母。

真人輪次需事前採用後另建 `--mode trial --agreement`，協議欄位見英文版。固定協議 commit／SHA、實際 recorder SHA、真實決定時間與維護者紀錄，不要代填未取得的同意。採用時把紀錄工具額外耗時及中斷規則一併固定。合併工具不等於採用協議，也不證明簽署身分或二進位來源。

資料只放本機可靠磁碟，避免網路／同步目錄；一位主持人一次一場。匯出含私人路徑與原句，**不要公開提交**，另整理經同意的匿名摘要。SQLite 保障正常交易的已提交紀錄，雜湊可核對 bytes；不能防止控制整個電腦者偽造資料。程式成功、JSON current 或附件名 decision 都不等於人類確認。

詳見[實作與 RED/GREEN 證據](../../../docs/validation/n07-recorder.md)。目前沒有新增真人紀錄，結果仍為「待真人證據」。
