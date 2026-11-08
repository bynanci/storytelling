# storytelling

以 Rust 建立的本機優先 Story SDK／可審閱剪輯工作流核心。

**M0、有限 M1-01／M1-02、M2-01 稽核與 M1-03 宿主證據均已接受並合併 main；M2-02 固定視窗規劃已接受；M2-03 選定範圍即時比對已由 PR #26 接受並合併；尚未發布 release。**
可驗證有界故事計畫 JSON，並將有限 OpenFilm composition export 唯讀映射為 declared visual plan。
`verify-source` 另可雜湊明確提供的來源 bytes；規劃器依明確提供的區間與限制提出 StoryPlan，正式宿主執行仍為後續工作流任務。
獨立測試工具已完成一個有界 FilmCraft 實際往返案例。
main 另已整合 PR #32 的 `prepare-review`／`check-review` 有界 profile：同輸入 OTIO 綁定與 declared-context freshness；
批准、來源、宿主與執行旗標均為 false。有限 OpenFilm review runtime 對照與 Rust digest/error-order 回歸已補入 [Issue #27](https://github.com/bynanci/storytelling/issues/27) 證據；M3-01/P1 有界 profile 已依[簽署驗收紀錄](docs/migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md)完成 G2 接受，此接受不涵蓋 PR #28 或更廣泛的 M3 工作。

完整的八個指令、輸入格式及可執行範例見 [CLI 使用指南](docs/CLI_GUIDE.md)；
已合併／已關閉／待審 PR 見 [交付登錄](docs/delivery/PR_REGISTER.md)。

## 直接執行

```sh
cargo test --workspace --locked
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
cargo run --locked -p storytelling-cli -- import-openfilm tests/fixtures/openfilm/finite-v1/still.json
cargo run --locked -p storytelling-cli -- export-otio tests/fixtures/openfilm/finite-v1/still.json --metadata-only-locks > finite.otio

cargo run --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/valid.json
cargo run --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
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

`verify-source FILE|- SHA256` 比對最多 16 MiB 的明確輸入與預期小寫 SHA-256，
只輸出 metadata；不可從 receipt 反序列化出可信 buffer，也不證明往後的檔案讀取。
[M1-03 證據](docs/migration/records/M1-03.md) 已驗證 FilmCraft v0.18.0 Linux／25 fps
原生 1920×1080 時間區間、儲存重開與 OTIO 再匯入，以及 45 格 192×108 CPU H.264
預覽輸出。音訊、GUI／GPU、全解析度輸出及人工剪輯驗收仍是後續工作。

`plan-story FILE|-` 在硬時長／shot 預算內選取固定來源區間，保留手動必選與順序，
目標時長維持軟偏好。輸出 `planning.plan` 可交給既有 M0 validator；選取理由引用
輸入宣告位置，未當成已驗證的語意證據。見 [規劃契約](docs/migration/FIXED_WINDOW_PLANNER_V1.md)
與 [CLI 使用指南](docs/CLI_GUIDE.md)。

## 三層基礎架構

| 層 | 已實作責任 | 不負責 |
| --- | --- | --- |
| core | 嚴格 wire DTO、canonical rational time、錯誤契約 | I/O、資料庫、宿主與模型 |
| engine | 有界驗證、finite snapshot mapper、精確時間與 loss | 渲染、檔案修改與 Agent 批准 |
| CLI | 本機檔案／stdin 讀取、JSON、退出碼 | 另建 planner 或繞過驗證 |

使用 Rust 2024／resolver 3；固定 1.85.0 作為可重現 bootstrap 基線，並非宣稱最新版。
原 MIT LICENSE 不變；來源衍生 importer／regression component 採 Apache-2.0，
各來源衍生 component 均有逐檔 [provenance](PROVENANCE.md) 與適用 LICENSE／NOTICE。
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

有限 OTIO 與有界 FilmCraft 往返證據已完成；M2-02 固定視窗規劃已接受，[M2-03 即時 shadow](tests/oracles/planner-shadow-v1/README.md) 已完成選定範圍的 64 組實際比對（56 一致、4 已定義政策差異、4 不支援案例），已通過同 head CI 並合併 PR #26；完整 solver 仍為後續工作。
正式宿主執行、更多媒體與人工驗收仍有獨立門檻。
