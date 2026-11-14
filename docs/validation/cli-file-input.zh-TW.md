# CLI 一般檔案輸入回歸

[English](cli-file-input.md) · 繁體中文

本次共用 CLI I/O 修正支援 M3-RB07 與 N-R10-06A，base 為
`c562ce0d06b7330813fa42cad739f24f4a49e9da`；首次整合 base 為
`5bbe80dc2cf8286abb15ad5278e86ec7384cdb4d`。不修改契約、公開 DTO、診斷碼、
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
source commit 的完整驗證；以下獨立全新 target 重跑取代這項主張。FIFO
缺陷重現與這項無效的 workspace 來源主張分開記錄；原輸出與數字均未刪除。

### 整合版本的全新驗證

開始驗證時，整合來源 `ed5bfa6a7d66cce9b66faf6ac8bef653b5e43485` 為 clean。
使用原本不存在、只供本 checkout 使用的全新 target，並設定
`CARGO_INCREMENTAL=0`。[獨立收據](cli-file-input/isolated-checks.json)與
[實際輸出節錄](cli-file-input/isolated-checks.txt)保留命令、binary／source blob
雜湊、原始輸出雜湊及數量。這是取代舊收據的有效本機 workspace 結果，原
疑義紀錄仍保留。

全新 all-feature build、四項 CLI Python 方法、fmt、all-target／all-feature
Clippy、預設及 all-feature debug／release tests、禁止警告的 rustdoc、repo
checks 與完整分支 diff checks 全部通過。預設 Rust 各 312 項、all-feature
各 331 項，含整合後十三項 journal、七項 V1 coverage 及十二項 draft-input
CLI tests；總數含 doc tests，不含 Python 方法。既有 pilot、recorder、graph、
Host 證據拒絕、SK-E01／E02／E04／E06 及 R1 Python checks 亦通過。另有一次
補充命令誤用不存在的腳本名稱，退出 2，沒有執行測試；改用既有 E04 suite
後通過，兩次呼叫都保留。

不要讓 worktrees 共用 `CARGO_TARGET_DIR`。乾淨重跑須選擇本 checkout 專用、
原本不存在的 target，並將該目錄的 binary 傳給 Python。以下命令採預設
checkout-local `target`；先移除共用 target 的環境設定。新收據保留 rebase
前後 patch 對應；有效的新驗證綁定上列整合來源。

### 測試探索相容性修正

PR #139 合併前，廣泛 Python discovery 發現 harness 設定錯誤：模組的 `CLI`
尚未綁定，四個方法嘗試啟動 `None`，產生十項 errors。
[設定 RED 收據](cli-file-input/discovery-red-receipt.json)與
[輸出](cli-file-input/discovery-red.txt)記錄在 `fdd8283` 的實際重現。這不是
原本 FIFO 產品缺陷的 RED，也不增加產品測試成果。

以 merge 接入最新 main `3e0f0d0992df38acc53402f0eb1d1926f63cf8a7`，未改寫
已推送歷史。在來源 `d1d8b64f8167eebe9a2e1ee915d3b0b3e2ef1923`，`setUp` 只在
未綁定明確 binary 時 skip；standalone 入口先綁定 binary，所以四項產品
測試方法維持必須執行且未修改。既有獨立 binary 與 Rust 來源雜湊均符合之前全新
build，因此這次修正不需重編 Rust。

[Discovery GREEN 收據](cli-file-input/discovery-green-receipt.json)與
[完整輸出](cli-file-input/discovery-green.txt)記錄：

| 呼叫 | 實際結果 |
| --- | --- |
| 明確指定 feature-enabled CLI binary | 四項產品方法通過；Linux 無 skip。 |
| 未指定 binary 的 targeted discovery | 四項設定 skip，未執行產品斷言。 |
| 廣泛 `scripts/test_*.py` discovery | 記錄 103 項：99 通過，四項設定 skip。 |
| 整合後 SK-E04 suite | 24 項通過。 |
| Repository 與完整分支 diff checks | 通過。 |

CI 分成兩個各一條命令的步驟，先執行必須通過的明確 binary 呼叫，再做
targeted discovery，避免 Windows PowerShell 讓後續成功的 discovery 蓋掉
前面 binary 測試的失敗退出碼。Windows 的
明確 binary 呼叫仍只 skip Unix FIFO 案例。最終新 head 必須重跑 hosted CI；
舊 `fdd8283` 的結果不是最終 head 驗證。

完整重跑命令見[英文紀錄](cli-file-input.md#test-first-evidence-and-replay)。
最短回歸命令如下；Windows 使用 `python` 與 `.exe` 副檔名。

```sh
cargo build --locked -p storytelling-cli --features draft-video-input
python3 scripts/test_cli_file_input.py target/debug/storytelling
python3 -m unittest discover -s scripts -p 'test_cli_file_input.py'
cargo test --locked -p storytelling-cli --all-features --test video_coverage --test draft_video_input
```

既有 pinned-Python Rust matrix 在 all-features 整合測試後立即執行本腳本，
位於可能改寫 binary 的預設 feature 範例之前。本機證據只有 Linux；合併前仍
須讀取最終 head 的 hosted CI。這些測試不驗收發布、真人、真實媒體或 Host
恢復能力。
