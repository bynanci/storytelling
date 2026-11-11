# Storytelling

[English](README.md) · [CLI 指南](docs/CLI_GUIDE.md) · [開發路線圖](ROADMAP.md) · [貢獻指南](CONTRIBUTING.md) · [CI 執行紀錄](https://github.com/bynanci/storytelling/actions/workflows/ci.yml)

Storytelling 是用 Rust 開發、在本機執行的敘事規劃工具。第一個能實際操作的用途是影片粗剪。提供候選片段、可用的起訖時間和剪輯規則後，規劃器會排出鏡頭順序、說明略過哪些片段，再將選定的剪點轉為可審閱的 OpenTimelineIO（OTIO）時間軸。

**目前可從原始碼建置 `0.1.0-dev.0`。** 專案尚未正式發布。[跨媒介路線圖](ROADMAP.md#cross-media-narrative-roadmap) 說明未來如何運用同一組材料，構思劇本、插畫、商業文案、腦力激盪與日常敘說。

## 快速開始

透過 [rustup](https://rustup.rs/) 安裝 Rust，再下載專案、執行內附的合成範例。專案會選用固定的 Rust 1.89.0 工具鏈。

```sh
git clone https://github.com/bynanci/storytelling.git
cd storytelling
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

1. `validate` 會回傳 `ok: true`，並列出 `travel-recap` 的 1 份素材與 2 個鏡頭。這項檢查驗證資料中宣告的 ID 和時間範圍。若要以摘要比對實際檔案內容，可使用 `verify-source`。
2. `plan-story` 會將 JSON 寫至標準輸出。[範例規劃需求](tests/fixtures/planner-v1/valid.json) 指定先用 `detail`、再用 `opening`。即使 `opening` 的分數較高，`planning.plan` 的兩個鏡頭仍保留作者指定的順序。

要試跑交接流程，可依 [CLI 指南](docs/CLI_GUIDE.md) 將儲存的規劃提案、對應的快照與審閱條件轉成審閱檔及 `.otio` 時間軸。

## 目前可用功能

| 使用目的 | 指令 | 支援範圍 |
| --- | --- | --- |
| 檢查規則並規劃剪輯 | `validate`、`audit-brief`、`plan-story` | 檢查參照、必選／排除素材與先後順序，再從給定區間選取符合鏡頭數與時長限制的片段。 |
| 交換粗剪時間軸 | `import-openfilm`、`export-otio` | 讀取單一連續影像軌上的照片與影片硬切。匯出影片時須明確選擇僅保留畫面。獨立音軌、特效和動畫尚未支援。 |
| 審閱選定的提案 | `prepare-story-review`、`export-story-otio`、`check-story-review` | 將提案和快照綁定到實際產出的 OTIO，再檢查輸入、格式轉換選項或時間軸是否已變更。 |
| 審閱既有剪輯 | `prepare-review`、`check-review` | 將支援範圍內的剪輯資料及其 OTIO 對應到指定的審閱條件。比對結果用來確認一致性。人工核准與寫入剪輯器屬於後續工作。 |
| 核對素材內容 | `verify-source` | 以 SHA-256 比對明確提供的檔案或標準輸入，最多 16 MiB。 |

OpenFilm 提供剪輯快照，工作中的時間軸由目標剪輯器管理。專案曾以 FilmCraft v0.18.0 在 Linux、25 fps 的單一條件下測試儲存、重開、重新匯入 OTIO，以及輸出 45 格 CPU 預覽。自動將剪輯套用至正式專案仍在規劃中。音訊交接和全解析度輸出也有待實測。[FilmCraft 測試紀錄](docs/migration/records/M1-03.md) · [審閱契約](docs/migration/PLANNER_OTIO_REVIEW_V1.md)

## 跨媒介發展方向

第一個規劃中的案例，是將同一段生活記憶整理成口述提綱與單幅插畫的不同構圖。為插畫增加的想像細節可以只留在該構圖，口述仍能追溯原始敘述。第二個案例則用商業文案驗證每項主張能否追溯到提供的佐證。兩項案例目前都在規劃階段，各自的依賴與驗收條件列於[跨媒介任務清單](TASKS.md#cross-media-narrative-backlog)。

## 架構與參與開發

Workspace 由四個 crate 組成。`storytelling-core` 定義資料交換型別與精確的有理數時間。`storytelling-engine` 在記憶體中執行驗證、規劃與匯出。`storytelling-cli` 負責檔案與標準輸入、標準輸出及退出碼。`storytelling-journal` 以 SQLite 持久化操作紀錄並隔離舊執行者；M3-02 有限日誌已由 PR #56 合併，但它不會呼叫剪輯器，也不授予執行權限。[架構說明](docs/ARCHITECTURE.md) · [資料與 CLI 契約](docs/CONTRACTS.md)

從[任務清單](TASKS.md)挑一個範圍明確的工作，查看[進行中的 PR](https://github.com/bynanci/storytelling/pulls)，並對應到[需求與驗收紀錄](docs/REQUIREMENTS.md)。本機檢查命令列於[貢獻指南](CONTRIBUTING.md)，開發、審查與發布條件見 [SDLC](docs/SDLC.md)。[回報問題](https://github.com/bynanci/storytelling/issues/new/choose)時可附上能重現問題的合成範例。涉及敏感內容請依[安全政策](SECURITY.md)處理。

## 授權

本專案原創程式碼及文件採 [MIT 授權](LICENSE)。參考 OpenFilm 實作的部分元件採 [Apache-2.0](LICENSES/Apache-2.0.txt)，保留[上游聲明](NOTICE.openfilm)。檔案對應的授權記錄於[逐檔來源清單](PROVENANCE.md)。
