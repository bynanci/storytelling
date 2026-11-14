# CLI 一般檔案輸入回歸

[English](cli-file-input.md) · 繁體中文

本次共用 CLI I/O 修正支援 M3-RB07 與 N-R10-06A，base 為
`c562ce0d06b7330813fa42cad739f24f4a49e9da`。不修改契約、公開 DTO、診斷碼、
依賴或位元組上限，也不新增媒體探測與 Host 操作。

## 問題與範圍

原本先 `File::open` 才檢查 handle metadata。具名 FIFO 沒有 writer 時，開檔
本身就會等待，無法抵達既有的一般檔案檢查。回歸測試走正常 CLI 入口，涵蓋
`verify-source`、既有影片宣告、opt-in draft helper 與各審閱檔案參數位置。

共用 reader 現在於開檔前檢查路徑 metadata，保留開檔後的 handle 檢查，仍最多
讀取原上限加一位元組。一般命令與 `verify-source` 都使用同一 reader；參數
驗證、各操作上限、domain 錯誤優先序與檔案／stdin 輸出語意維持一致。靜態
非一般檔案回傳 `{"ok":false,"error":{"code":"input.io","path":"$"}}`，
退出碼 3，stderr 為空，錯誤不揭露路徑。

這項修正只處理靜態路徑 I/O，不是可信本機媒體 adapter 或作業系統 sandbox。
指向一般檔案的 symlink 仍可使用；metadata 與 open 之間若遭替換，仍有可能
阻塞，開檔後檢查也不能消除這個競態。stdin 仍須 EOF，呼叫者不關閉資料流
就可能繼續等待。不宣稱 TOCTOU 保護、目錄隔離、媒體身分、作者批准或 Host
驗收。N10-06C 與執行權限保留原本獨立門檻。

## 測試先行證據與重跑

測試先行 commit 為 `b05a083d4e911564b38f6a70e715fa71bcd4bd01`。
[RED 收據](cli-file-input/red-receipt.json)與[輸出](cli-file-input/red.txt)記錄
退出 1：33 個 FIFO 子案例各逾時五秒，受控子程序均被終止並等待結束；目錄、
一般檔案／stdin 與大小限制案例原本就通過。逾時是程式缺陷重現，不是作者
操作效能資料。

實作 commit `a6c9a367bedb5004fa39b5eafa763dc0ec4b6838` 通過相同測試，見
[GREEN 收據](cli-file-input/green-receipt.json)與[輸出](cli-file-input/green.txt)。
四個測試方法對 33 個明確檔案參數位置逐一測目錄與 FIFO，另核對 supplied
bytes 雜湊、一般檔案／stdin 重播、原 fixture 不變及既有超限診斷。FIFO 用
Python 標準函式庫 `os.mkfifo` 建立，不依賴外部工具；Windows 明確 skip FIFO，
其餘檢查在三平台 CI 都執行。所有子程序都有有界清理。

原 Linux [workspace 收據](cli-file-input/workspace-checks.json)與
[實際輸出節錄](cli-file-input/workspace-checks.txt)保留為 `cache_ambiguous`／
superseded。共用 target 混用另一 worktree 的產物：所標示來源只有十一項
journal tests，紀錄卻執行十三項。因此原觀察到的 312／331 項不能證明該
source commit 的完整驗證；合併前須以獨立全新 target 重跑整合版本。FIFO
缺陷重現與這項無效的 workspace 來源主張分開記錄；原輸出與數字均未刪除。

完整重跑命令見[英文紀錄](cli-file-input.md#test-first-evidence-and-replay)。
最短回歸命令如下；Windows 使用 `python` 與 `.exe` 副檔名。

```sh
cargo build --locked -p storytelling-cli --features draft-video-input
python3 scripts/test_cli_file_input.py target/debug/storytelling
cargo test --locked -p storytelling-cli --all-features --test video_coverage --test draft_video_input
```

既有 pinned-Python Rust matrix 在 all-features 整合測試後立即執行本腳本，
位於可能改寫 binary 的預設 feature 範例之前。本機證據只有 Linux；合併前仍
須讀取最終 head 的 hosted CI。這些測試不驗收發布、真人、真實媒體或 Host
恢復能力。
