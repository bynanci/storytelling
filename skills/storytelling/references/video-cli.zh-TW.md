# 用本機命令列工具交接影片提案

[English](video-cli.md) · 繁體中文

只有文字或照片描述時，可先提供分鏡或旁白草案。列出缺少素材的鏡頭。若作者要求重現選鏡或匯出時間軸，請先檢查輸入。

Storytelling 命令列工具（CLI）讀取呼叫端指定的檔案。它將結果寫至標準輸出。素材管理工具可提供片段供 CLI 規劃。文字、插畫與劇本也可直接使用 Skill。

## 選擇已具備的執行方式

| 目前環境 | 操作 |
| --- | --- |
| 已從原始碼建置 | 在原始碼根目錄執行 `./target/release/storytelling --help`。Windows 使用 `target/release/storytelling.exe`。 |
| 原始碼尚未建置 | 確認電腦已安裝 `rust-toolchain.toml` 指定的工具鏈。若依賴已快取，執行 `cargo build --offline --release --locked -p storytelling-cli`。 |
| 專案只有 Skill | 使用作者指定的執行檔，或從 `PATH` 執行 `storytelling --help`。確認所需的子命令存在。請提供自己的輸入檔。缺少工具或輸入時，先交付文字草案並列出缺項。 |

## 選擇輸入

OpenTimelineIO（OTIO）是時間軸交換格式。下列指令各有對應的輸入契約。

| 作者提供 | 指令 | 結果 |
| --- | --- | --- |
| 原生素材與鏡頭計畫（M0） | `validate <PLAN>` | 檢查宣告的素材 ID 與來源區間。 |
| 剪輯需求、候選片段、宣告分數、時長與鏡頭數上限 | `plan-story <REQUEST>` | 產生規劃結果。結果含 `planning.plan`、選取原因與略過原因。 |
| 完整規劃結果、相符的 `finite-v1` 剪輯快照與審閱設定 | `prepare-story-review <PROPOSAL> <SNAPSHOT> <CONTEXT>` | 產生含 OTIO 時間軸與審閱紀錄的 JSON。 |
| 同上三項輸入 | `export-story-otio <PROPOSAL> <SNAPSHOT> <CONTEXT>` | 輸出原生 OTIO 時間軸。可按作者要求另存。 |
| 上次的整份審閱結果與目前三項輸入 | `check-story-review <BUNDLE> <PROPOSAL> <SNAPSHOT> <CONTEXT>` | 檢查宣告與時間軸是否一致。若已過期或輸入遭拒，退出碼為 1。 |

請將 `plan-story` 的完整成功結果交給審閱指令。單獨的 `planning.plan` 可傳給 `validate`。素材 ID、宣告的雜湊值與可用時長必須對應同一份快照。

影片交接限於單一連續畫面軌上的照片與影片硬切。影片須明確接受 `picture_only`。音訊、效果與動畫請另行處理。作者核准與寫入剪輯器各需獨立步驟。

## 用隨附合成資料重播

先從原始碼完成 release 建置。在原始碼根目錄執行下列 Bash 範例。指令只將結果寫到終端機。合成審閱設定已選擇只保留畫面。

```bash
set -o pipefail
./target/release/storytelling plan-story tests/fixtures/story-review-v1/planner-request.json |
  ./target/release/storytelling prepare-story-review - \
    tests/fixtures/openfilm/finite-v1/video-trim.json \
    tests/fixtures/story-review-v1/context.json
```

成功後，結果包含 `ok`、`review` 與 `otio`。本例的來源區間從 1/3 秒到 4/3 秒。處理自己的素材時，請使用作者指定的剪點、來源 ID 與格式轉換選項。若指令失敗，請回報本次退出碼與診斷內容。[命令列指南](https://github.com/bynanci/storytelling/blob/main/docs/CLI_GUIDE.zh-TW.md)
