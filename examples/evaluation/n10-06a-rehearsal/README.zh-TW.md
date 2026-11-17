# N10-06A 合成演練與作者交接

[English / 完整規程與主持人答案](README.md) · [模擬圓桌與機器收據](../../../docs/validation/n10-06a-roundtable-2026-11-17.zh-TW.md)

請求者於 2026-11-17 選擇「模擬圓桌」。本資料包保留八個合成案例的實際 CLI 輸出，以及未執行的作者觀察提案。**真人參與者為 0；四個視角由同一 AI 編寫**。角色發言不是作者原話、獨立審查或 G2 接受，也不補造 #121 原始研究輸入或 #122 holdout。

機器重播（先建置有 `draft-video-input` feature 的實際 binary；每個 worktree 使用新的獨占 target）：

```sh
cargo build --release --locked -p storytelling-cli --features draft-video-input --target-dir /tmp/n10-a-unique-target
python3 examples/evaluation/n10-06a-rehearsal/test_replay.py /tmp/n10-a-unique-target/release/storytelling
python3 examples/evaluation/n10-06a-rehearsal/replay.py --cli /tmp/n10-a-unique-target/release/storytelling --source-commit 06164f19a8b3614f438dd2f97ef810c94e64c3d3 --output /tmp/n10-machine-receipt-new.json
```

Windows 請使用實際 `.exe` 與 Python 路徑。這些是重播命令，不是 Windows 真人操作紀錄。source SHA 是呼叫者宣告，工具不驗證其與 binary 的關係；另存建置收據。工具保存原始輸入、stdout／stderr base64、雜湊、退出碼與程序時間；舊收據不覆蓋、失敗不換成成功。只使用既有公開合成 fixture，不讀媒體、不呼叫宿主。37 次 CLI 呼叫不是 37 名參與者，程序耗時不能當作者耗時。

PR #145 原有四項測試及歷史收據保留不變；目前共七個測試方法，另三項回歸涵蓋 18 種原生輸出故障注入。獨立精確算術 oracle 檢查每個有效 mapped 列的兩端點、固定單元參照及 unresolved 原因，並保留原有 V1 一致性檢查。[修正與 RED／GREEN 紀錄](../../../docs/validation/n10-06a-replay-oracle-2026-11-17.zh-TW.md)針對評估工具的誤判通過，沒有觀察到原生轉換錯誤。未指定 binary 的 discovery 會跳過原生測試；既有 CI 明確傳入 binary，七個方法必須全部通過且無跳過。

若之後要真人操作，先由主持人／維護者同意觀察規程與隱私範圍。這是一名作者的非比較性可行性觀察；不是 N07 或 #112 Beta，不能估計比較省時。作者已被告知工具唯讀，第四題是在這個提示後檢查理解，不是無提示的首次理解測量。

最短交接：

1. 找一名真實作者，如實記錄先前接觸、與專案關係及 CLI／JSON 經驗；取得私人紀錄、匿名摘要、公開引文各自同意。維護者本人須標為自操作。
2. 在 Git 外的私人目錄準備 [作者卡](author-sheet.zh-TW.md)及兩份起始檔（[秒數](seconds-start.draft.json)、[影格](frames-start.draft.json)）；時間欄位留空，由作者輸入。主持人先完成安裝，另記準備時間。作者不看本 README、模擬發言或主持人答案。
3. 作者收到題目時開始連續計時，包含閱讀、編輯、CLI 等待、錯誤修正、幫助與回答。保存每版輸入與每次實際輸出，十分鐘未完成就停止並保留部分成果；中斷或時間未知須如實登記。
4. 討論答案前保存四題原話與意外回答。填寫[私人空白表](observation.blank.json)的原題、起始資料、作者原話、觀察及限制／使用性失敗，未測欄留 null。最後請作者確認匿名摘要。

事先判準：完成三個操作任務、能根據輸出說明缺素材及宣告邊界時，才建議再做一個質性觀察，並記錄所有協助。若轉換成功但作者誤以為媒體已驗證或宿主已套用，縮小後續設計，保留反例。退出、敏感資料、時間未知或十分鐘到時停止。真人決策由維護者看真實紀錄後作出，目前沒有決策。#129 的獨立作者及 B/C/D、G2、Host、G5 門檻仍保留。
