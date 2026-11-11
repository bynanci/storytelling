# 用本機 Storytelling CLI 交接影片提案

只有文字、照片描述或剪輯靈感時，直接交付分鏡／旁白草案，並列出尚待對應的素材。作者要可重現的選鏡或 OTIO，且提供相應的結構化輸入時，才使用下列指令。這些指令只讀明確提供的輸入，結果寫到 stdout。

## 選擇已具備的執行方式

- **已從原始碼建置**：在 Storytelling 原始碼根目錄，執行 `./target/release/storytelling --help`；Windows 的執行檔是 `target/release/storytelling.exe`。
- **有本機原始碼，尚未建置**：若已安裝該 checkout 的 `rust-toolchain.toml` 指定工具鏈且依賴已快取，在該 repo 根目錄執行 `cargo build --offline --release --locked -p storytelling-cli`。
- **只把 Skill 裝進其他專案**：用使用者指定的本機執行檔或 PATH 上可用的 `storytelling --help`，確認支援需要的子命令。下方範例資料在 Storytelling 原始碼 repo，獨立使用時改傳使用者提供的明確檔案。若缺少工具或輸入，交付文字草案並指出缺少哪一項。

## 選擇輸入

| 使用者提供 | 指令 | 可交付的結果 |
| --- | --- | --- |
| 素材與鏡頭組成的 M0 plan | `validate <PLAN>` | 檢查已宣告的素材 ID 與來源區間 |
| Brief、候選視窗、宣告分數與時長／鏡頭數硬上限 | `plan-story <REQUEST>` | 完整規劃回應，含 `planning.plan` 與採用／略過原因 |
| 規劃回應、相符的有限 OpenFilm 快照、審閱 context | `prepare-story-review <PROPOSAL> <SNAPSHOT> <CONTEXT>` | 包含實際 OTIO 及其審閱紀錄的 JSON bundle |
| 同上三個輸入 | `export-story-otio <PROPOSAL> <SNAPSHOT> <CONTEXT>` | 原生 OTIO Timeline JSON，可依使用者要求另存 |
| 上次的完整 bundle 與目前三個輸入 | `check-story-review <BUNDLE> <PROPOSAL> <SNAPSHOT> <CONTEXT>` | 判定宣告及時間軸是否仍一致。已過期或遭拒絕時退出 1 |

將 `plan-story` 的完整成功 JSON 原樣交給後續命令。單獨的 `planning.plan` 可交給 `validate`。審閱時，素材 ID、宣告的 hash 與可用時長須對應到同一份快照。只支援單一連續畫面軌的照片／影片硬切；影片需要作者明示接受 `picture_only`。音訊、效果與動畫仍須其他工作流。作者核准和宿主寫入是後續獨立動作。

## 用隨附合成資料重播

在已完成 release 建置的 Storytelling repo 根目錄，以下 Bash 範例只輸出至終端機。合成 context 已選擇僅保留畫面。

```bash
set -o pipefail
./target/release/storytelling plan-story tests/fixtures/story-review-v1/planner-request.json |
  ./target/release/storytelling prepare-story-review - \
    tests/fixtures/openfilm/finite-v1/video-trim.json \
    tests/fixtures/story-review-v1/context.json
```

成功時可見 `ok`、`review` 和 `otio`。範例使用提案中 `1/3` 到 `4/3` 秒的來源區間。實際工作只使用作者提供的剪點、來源識別與明確接受的格式轉換選項。若命令失敗，依退出碼與實際診斷回報，不把範例結果當成這次執行的輸出。[上游 CLI 指南](https://github.com/bynanci/storytelling/blob/main/docs/CLI_GUIDE.md)
