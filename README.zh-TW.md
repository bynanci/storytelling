# storytelling

以 Rust 建立的本機優先 Story SDK／可審閱剪輯工作流核心。

**M0 validator、M1-01 importer 與 M1-02 finite OTIO 已接受並合併 main；M2-01 audit 審查中。**
可驗證有界故事計畫 JSON，並將有限 OpenFilm composition export 唯讀映射為 declared visual plan。
不會讀取來源媒體、驗證真實檔案 SHA、產生故事、渲染或操作外部剪輯器。

## 直接執行

```sh
cargo test --workspace --locked
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
cargo run --locked -p storytelling-cli -- import-openfilm tests/fixtures/openfilm/finite-v1/still.json
cargo run --locked -p storytelling-cli -- export-otio tests/fixtures/openfilm/finite-v1/still.json --metadata-only-locks > finite.otio

cargo run --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/valid.json
```

合法範例回傳 `ok: true`，退出碼 0；超出來源範圍的範例回傳
`plan.source_out_of_bounds`，退出碼 1。標準輸入使用 `validate -`。
validate 成功報告明示 `declared-metadata-only`，不能把宣告格式合法當作來源內容已驗證。

`import-openfilm -` 可讀 stdin；支援單一連續影像軌的 still/video cuts，保留原始事實、
occurrence IDs 與數字 lexemes。成功仍為 `declared-visual-mapping-only`、`executable:false`；
video 帶 `audio.not_mapped / blocks-host-handoff`，明確 audio/effects/animation 拒絕且無 plan。
見 [finite-v1 契約](docs/migration/FINITE_V1.md)、[比對／證據](docs/migration/records/M1-01.md)
與 [工作任務](docs/migration/M1-01-WORK-QUEUE.md)。

`export-otio` 的 stdout 可直接存成 OTIO Timeline JSON。video 需 `--picture-only`，
來源有鎖定需 `--metadata-only-locks`；這些是格式損失選擇，不是宿主執行批准。
使用精確且有界的 content ticks，原始 FPS 另存 metadata；無法精確表示時明確拒絕。
見 [finite OTIO 契約](docs/migration/FINITE_OTIO_V1.md)。官方 parser 通過仍不等於實際宿主驗證。

`audit-brief FILE|-` 檢查 versioned StoryBrief 的硬限制參照、必含／排除矛盾與排序，
保留手動選擇並回傳 typed conflicts；偏好仍是軟條件。成功只表示已實作的宣告約束沒有
矛盾，`selection_planned:false`、`duration_analysis:"not-performed"`，沒有產生選片或
證明目標時長可達。見 [brief 契約](docs/migration/STORY_BRIEF_V1.md)
與 [M2-01 證據](docs/migration/records/M2-01.md)。

## 三層基礎架構

| 層 | 已實作責任 | 不負責 |
| --- | --- | --- |
| core | 嚴格 wire DTO、canonical rational time、錯誤契約 | I/O、資料庫、宿主與模型 |
| engine | 有界驗證、finite snapshot mapper、精確時間與 loss | 渲染、檔案修改與 Agent 批准 |
| CLI | 本機檔案／stdin 讀取、JSON、退出碼 | 另建 planner 或繞過驗證 |

使用 Rust 2024／resolver 3；固定 1.85.0 作為可重現 bootstrap 基線，並非宣稱最新版。
原 MIT LICENSE 不變；來源衍生 importer／regression component 採 Apache-2.0，
保留逐檔 [provenance](docs/migration/provenance-m1-01.json) 與適用 LICENSE／NOTICE。
使用者已明確授權在同 head 驗證通過後合併已完成 PR；發布、tag 與 repository policy
仍須獨立決策。實際合併紀錄見 [accepted lifecycle](TASKS.md#accepted-lifecycle)。

## SDLC 導覽

- [SDLC](docs/SDLC.md)：需求、設計、實作、驗證、發布與維運的 stage gates。
- [TASKS](TASKS.md)：工作 ID、優先級、依賴、範圍、完成條件。
- [ROADMAP](ROADMAP.md)：M0–M5 與 v1 的成果放行規則。
- [REQUIREMENTS](docs/REQUIREMENTS.md)：需求到檔案與測試的 traceability。
- [ARCHITECTURE](docs/ARCHITECTURE.md)：已實作與未來模組的明確分界。
- [MIGRATION](docs/MIGRATION.md)：從 OpenFilm 抽離、shadow 比較與漸進替換。
- [驗證紀錄](docs/validation/m0-foundation.md)：RED、最終 CI 與人工 gate 的區別。

M1-02 finite OTIO 已完成；下一個宿主任務是 M1-03 FilmCraft
真實 roundtrip；不先增加空 crate、全面 Rust 改寫、雙向同步或通用 Agent 平台。
