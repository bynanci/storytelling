# storytelling

以 Rust 建立的本機優先 Story SDK／可審閱剪輯工作流核心。

**本次交付是 M0 基礎驗證切片，不是已完成的 FilmCraft 整合，也不是另一套 NLE。**
目前可讀取有大小限制的故事計畫 JSON，驗證欄位、ID、宣告來源、精確時間及範圍，回傳結構化結果。
不會讀取來源媒體、驗證真實檔案 SHA、產生故事、渲染或操作外部剪輯器。

## 直接執行

```sh
cargo test --workspace --locked
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
```

合法範例回傳 `ok: true`，退出碼 0；超出來源範圍的範例回傳
`plan.source_out_of_bounds`，退出碼 1。標準輸入使用 `validate -`。
所有成功報告明示 `declared-metadata-only`，不能把宣告格式合法當作來源內容已驗證。

## 三層基礎架構

| 層 | 已實作責任 | 不負責 |
| --- | --- | --- |
| core | 嚴格 wire DTO、canonical rational time、錯誤契約 | I/O、資料庫、宿主與模型 |
| engine | 有界來源引用與時間範圍驗證 | 渲染、檔案修改與 Agent 批准 |
| CLI | 本機檔案／stdin 讀取、JSON、退出碼 | 另建 planner 或繞過驗證 |

使用 Rust 2024／resolver 3；固定 1.85.0 作為可重現 bootstrap 基線，並非宣稱最新版。
保留現有 MIT LICENSE，不直接搬入 OpenFilm 的 Apache 程式碼。

## SDLC 導覽

- [SDLC](docs/SDLC.md)：需求、設計、實作、驗證、發布與維運的 stage gates。
- [TASKS](TASKS.md)：工作 ID、優先級、依賴、範圍、完成條件。
- [ROADMAP](ROADMAP.md)：M0–M5 與 v1 的成果放行規則。
- [REQUIREMENTS](docs/REQUIREMENTS.md)：需求到檔案與測試的 traceability。
- [ARCHITECTURE](docs/ARCHITECTURE.md)：已實作與未來模組的明確分界。
- [MIGRATION](docs/MIGRATION.md)：從 OpenFilm 抽離、shadow 比較與漸進替換。
- [驗證紀錄](docs/validation/m0-foundation.md)：RED、最終 CI 與人工 gate 的區別。

下一個應執行的工作是 M1-01「有限交接契約與 OpenFilm fixture」，再做 OTIO／FilmCraft
真實 roundtrip；不先增加空 crate、全面 Rust 改寫、雙向同步或通用 Agent 平台。
