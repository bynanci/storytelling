# Storytelling

[English](README.md) · [跨平台使用指南](docs/PLATFORMS.md) · [範例](.agents/skills/storytelling/references/examples.md) · [貢獻指南](CONTRIBUTING.md) · [CI](https://github.com/bynanci/storytelling/actions/workflows/ci.yml)

Storytelling 是一套開源 [Agent Skill](.agents/skills/storytelling/SKILL.md)，可將手邊的材料整理成故事、劇本場景、插畫構圖、商業文案、腦力激盪提案或分鏡。例如，把一頁訪談筆記交給它，請它寫一段口述開場，再提出兩款海報構圖。三份作品可以各有觀點，同時讓人看得出哪些話出自受訪者、哪些是為了畫面而加的創作。

整個 Skill 裝在一個資料夾裡。可以放進寫作專案、客戶工作區或軟體儲存庫，使用你選擇的 Agent 和素材起草。影片工作另有可獨立執行的 Rust CLI，能在本機對結構化提案做可重現的檢查。

## 從 Skill 開始

將完整的 [`storytelling` 資料夾](.agents/skills/storytelling) 複製到下表對應位置，保留其中的 `SKILL.md`、範例、工具指引、選用腳本及授權檔。

| 使用環境 | 開始方式 |
| --- | --- |
| 專案中的 Codex | 放入專案的 `.agents/skills/storytelling/`，輸入 `$storytelling`。本儲存庫已放在此位置。 |
| 本機專案中的 Claude Code | 將同一份資料夾放入專案的 `.claude/skills/storytelling/`，輸入 `/storytelling`。 |
| 其他支援 Agent Skills 的工具 | 依該工具的文件載入 Skill。安裝位置、叫用方式與可用工具由各平台決定。 |
| 直接閱讀 Markdown | 開啟 [SKILL.md](.agents/skills/storytelling/SKILL.md) 和它連到的參考文件。作者或團隊可手動依步驟整理材料。 |
| 腳本、CI 或自有應用程式 | 使用[獨立 CLI](#快速開始)，傳入結構化資料，讀取 JSON 輸出與退出碼。 |

在 Codex 或 Claude Code 叫用 Skill 後，可用這段素材試做。

```text
兩位鄰居一起修好了一扇門。請寫兩句話給社區電子報，再想兩款海報構圖。
只有第二款海報可以加一把想像的紅傘。
```

檢查電子報是否忠於原始觀察，並確認兩款海報採用不同的視覺選擇，紅傘只出現在第二款。個人安裝、其他平台與各項驗證狀態，見[跨平台使用指南](docs/PLATFORMS.md)。

## 目前可用功能

- **創作與改寫。** 把筆記整理成作品、比較不同表達方向並保留來源。某個細節改動時，找出哪些版本用到了它。[使用範例](.agents/skills/storytelling/references/examples.md)
- **安排複雜工作。** 流程手冊涵蓋相依步驟、獨立分支、審閱與有限次重試。隨附的 Python 腳本可選用來排列工作依賴或檢查已記錄的執行順序。[情境與迴圈手冊](.agents/skills/storytelling/references/graph-loops.md)
- **追查佐證。** 若問題橫跨多份文件，且已有適用的 Graphify 索引，可以先定位候選段落，再讀原文核對。[Graphify 工具指引](.agents/skills/storytelling/references/graphify-tools.md)

## 快速開始

### 選用 Rust CLI

目前可從原始碼建置 `0.1.0-dev.0`，專案指定 Rust 1.89.0。下列範例無須 AI 訂閱、模型 API 金鑰或影片編輯器。以 [rustup](https://rustup.rs/) 準備工具鏈後，即可執行下列命令。

```sh
git clone https://github.com/bynanci/storytelling.git
cd storytelling
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

`validate` 會回傳 `ok: true`，列出 1 份素材、2 個鏡頭。`plan-story` 會輸出 JSON。範例中的 `opening` 分數較高，但作者已指定順序，所以 `detail` 排在它之前。要從自己的應用程式呼叫，可從這兩種輸入格式及 [CLI 契約](docs/CONTRACTS.md)開始整合。[規劃輸入範例](tests/fixtures/planner-v1/valid.json) · [CLI 指南](docs/CLI_GUIDE.md)

### 影片整合

| 輸入 | 指令 | 產出 |
| --- | --- | --- |
| 影片計畫、故事需求或固定區間的選鏡請求 | `validate`、`audit-brief`、`plan-story` | 檢查宣告的參照與限制，或產生鏡頭順序提案。 |
| 本機資料與預期 SHA-256 摘要 | `verify-source` | 比對來自指定檔案或標準輸入的資料，最多 16 MiB。 |
| 符合支援規格的 OpenFilm 剪輯快照 | `import-openfilm`、`export-otio` | 將單一連續畫面軌轉成計畫或 OpenTimelineIO（OTIO）。 |
| 剪輯快照與審閱條件 | `prepare-review`、`check-review` | 先儲存審閱紀錄，後續再用這份紀錄比對目前輸入。 |
| 已儲存的提案、對應快照和審閱條件 | `prepare-story-review`、`export-story-otio`、`check-story-review` | 將選定剪點匯出為 OTIO。使用先前儲存的審閱資料與目前輸入比對，確認是否有變更。 |

剪輯快照轉接器可讀取 OpenFilm 匯出的剪輯快照，接受支援畫面軌上的照片與影片硬切，影片匯出須明確選擇僅保留畫面。專案曾以 FilmCraft v0.18.0 在 Linux、25 fps 下驗證儲存與重開、OTIO 重新匯入，以及輸出 45 格 CPU 預覽。另一次隔離測試在匯入回應遺失時讀回專案狀態，確認實際結果。正式寫入剪輯器仍在規劃中。音訊交接與全解析度輸出有待實測。[FilmCraft 測試紀錄](docs/migration/records/M1-03.md) · [復原測試](docs/validation/m3-03-filmcraft-execution.md) · [審閱契約](docs/migration/PLANNER_OTIO_REVIEW_V1.md)

## 跨媒介發展方向

目前已可用 Agent 起草多種形式的作品。具版本的跨媒介資料格式、可重跑的來源修訂檢查、商業主張追溯及作者試用則列入後續開發。[開發路線圖](ROADMAP.md) · [跨媒介任務清單](TASKS.md#cross-media-narrative-backlog)

## 架構與參與開發

可攜式 Skill 位於 `.agents/skills/storytelling/`。Rust workspace 包含四個 crate。`storytelling-core` 定義資料交換型別與精確時間，`storytelling-engine` 在記憶體內規劃與驗證，`storytelling-cli` 負責程序輸入輸出，`storytelling-journal` 以 SQLite 保存操作紀錄。[架構說明](docs/ARCHITECTURE.md)

歡迎其他領域和 Agent 平台的使用者參與。提交新工作流時，請附最小合成案例、預期輸出與檢查方式。回報相容性時，請附用戶端、版本、安裝位置和實際觀察。可參閱[貢獻指南](CONTRIBUTING.md)、[工作清單](TASKS.md)、[需求追溯](docs/REQUIREMENTS.md)與[開發流程](docs/SDLC.md)。[回報問題](https://github.com/bynanci/storytelling/issues/new/choose) · [安全政策](SECURITY.md)

## 授權

可攜 Skill 內附 [MIT 授權](.agents/skills/storytelling/LICENSE)。專案的原創程式碼及文件採 [MIT 授權](LICENSE)。參考 OpenFilm 實作的元件依逐檔清單採 [Apache-2.0](LICENSES/Apache-2.0.txt)，保留[上游聲明](NOTICE.openfilm)。[逐檔來源清單](PROVENANCE.md)
