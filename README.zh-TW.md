# Storytelling

<a name="storytelling-繁體中文文件"></a>

[English](README.md) · [試用範例](#把同一份素材做成廣播與海報) · [安裝](#從-skill-開始) · [貢獻指南](https://github.com/bynanci/storytelling/blob/main/CONTRIBUTING.zh-TW.md) · [持續整合](https://github.com/bynanci/storytelling/actions/workflows/ci.yml)

Storytelling 是一套開源敘事工具。作者可以用同一份素材起草口述、劇本、插畫構圖或文案。每份草稿都可以標出素材來源與創作提議。

專案的入口是 **Agent Skill**，也就是 AI 助理可載入的一組指引。你可以在 Codex 或 Claude Code 使用這組指引。你也可以直接閱讀 Markdown，依步驟整理素材。[開啟敘事指引](skills/storytelling/SKILL.zh-TW.md)

<a name="why-storytelling"></a>

## 為什麼需要 Storytelling？

兩位鄰居一起修門。社區電子報這樣開場，讀者先看見兩人的合作。

> 一個人扶著門，另一個人把鉸鏈上的螺絲鎖緊。

口述從修門的結果開始，聽者會想知道先前發生了什麼。

> 這一次，門關上時沒有再磨到地面。

同一項事實可以有不同的開場。作者決定讀者先知道什麼，也決定哪些細節只屬於某一版。若作者只同意在一幅插畫中加入紅傘，其他草稿仍依原始筆記撰寫。[作者練習](https://github.com/bynanci/storytelling/blob/main/docs/narrative/N-01-AUTHOR-STUDY.zh-TW.md) · [設計依據](https://github.com/bynanci/storytelling/blob/main/docs/WHY_STORYTELLING.zh-TW.md)

<a name="try-one-source-set-in-two-media"></a>

## 把同一份素材做成廣播與海報

範例包含三份素材。公告提供活動事實。示意圖提供物件位置。音檔提供可播放的音效。[活動公告與素材說明](skills/storytelling/references/workshop-demo.zh-TW.md) · [工作桌示意圖（SVG）](skills/storytelling/assets/umbrella-workshop/bench.svg) · [兩聲敲擊（WAV）](skills/storytelling/assets/umbrella-workshop/taps.wav)

![工作桌左側是收起的傘，右側是線軸。](skills/storytelling/assets/umbrella-workshop/bench.svg)

把下列請求貼給 AI 助理。若工具無法讀取附件，請使用文字描述。

```text
請邀請沒來過的居民參加修傘工作坊。素材有三份。
公告寫著週六舉辦，請每人帶一把傘。
工作桌示意圖左邊是一把收好的傘，右邊是一捲線。
音檔是合成的兩聲敲擊，沒有人聲。
請寫一段廣播開場，再給兩款海報構圖，一款用瑞士網格，一款用剪紙拼貼。
標出哪些細節來自素材，哪些是你的創作提議。
```

| 草稿 | 作者可以取得的內容 | 來源與創作提議 |
| --- | --- | --- |
| 廣播開場 | **音效**　播出兩聲敲擊，停一拍。**口白**　「帶一把傘。這個週六，我們在修傘工作坊見。」 | 日期與攜帶物取自公告。敲擊音效取自提供的音檔。停頓是製作提議。 |
| 海報 A · 瑞士網格 | 保留左傘、右線軸。在圖下方靠左對齊「週六」和「帶一把傘」。 | 物件位置取自示意圖。文字對齊方式屬於這份草稿。 |
| 海報 B · 剪紙拼貼 | 用黃紙剪出張開的傘面弧形。讓一段線頭穿過弧形，指向「週六」兩字。 | 張開的傘與黃紙都是這一版的創作提議。 |

配音員可以依腳本試錄。插畫師可以依構圖起稿。完成圖檔或音檔需要能輸出該格式的工具。[跨媒體樣板](skills/storytelling/references/multimodal-templates.zh-TW.md#mixed)

<a name="choose-a-template-and-an-approach"></a>

## 依情境選樣板與流派

每個樣板提供一項請求、兩種處理方式與檢查方法。 要觀察讀者如何理解兩版，以及接手者實際改了哪些稿，可使用[盲讀與改稿情境](https://github.com/bynanci/storytelling/blob/main/examples/README.zh-TW.md)。

| 情境 | 可以改變的敘事選擇 | 交付物 |
| --- | --- | --- |
| [日常敘事](skills/storytelling/references/multimodal-templates.zh-TW.md#everyday) | 從現場動作開始，或先說明結果。 | 一段可直接口述的故事，區分所見與感受。 |
| [編劇](skills/storytelling/references/multimodal-templates.zh-TW.md#screenplay) | 依事件順序寫實呈現，或用倒敘揭露物件的重要性。 | 場景、動作、對白與揭露時機。 |
| [插畫構思](skills/storytelling/references/multimodal-templates.zh-TW.md#illustration) | 用瑞士網格安排閱讀順序，或用剪紙形狀引導視線。 | 主體、視覺焦點、構圖與文字位置。 |
| [商業文案](skills/storytelling/references/multimodal-templates.zh-TW.md#copy) | 從引起注意寫到行動，或從讀者的問題寫到解法。 | 標題、內文、行動呼籲與待查證的主張。 |
| [腦力激盪](skills/storytelling/references/multimodal-templates.zh-TW.md#brainstorm) | 用替換或合併改造舊點子，或從失敗情境反推解法。 | 可比較的方案、篩選條件與試驗方法。 |
| [聲音敘事](skills/storytelling/references/multimodal-templates.zh-TW.md#audio) | 聲音日記先播放環境音。口述隨筆先交代敘事觀點。 | 旁白、音效、停頓與聲音來源。 |
| [影片規劃](skills/storytelling/references/multimodal-templates.zh-TW.md#video) | 用動作接續鏡頭，或用形狀與概念連接畫面。 | 鏡頭順序、對應素材與缺少的畫面。 |

<a name="start-with-the-skill"></a>

## 從 Skill 開始

<a name="install-for-an-agent"></a>

<a name="安裝到-agent"></a>

### 安裝到 AI 助理

1. 準備 Node.js 22.20 或更新版本。
2. 開啟要使用 Skill 的專案目錄。
3. 執行安裝與查詢命令。

```sh
npx skills add bynanci/storytelling --skill storytelling -a codex claude-code
npx skills list
```

若只使用一個平台，請在 `-a` 後保留該平台名稱。省略 `-a codex claude-code` 可在安裝時選擇平台。

<a name="install-as-an-npm-dependency"></a>

<a name="作為-npm-開發依賴安裝"></a>

### 使用 npm 鎖定版本

1. 開啟含有 `package.json` 的專案。
2. 從 GitHub 安裝套件。
3. 把 Skill 複製到 AI 助理的目錄。

```sh
npm install -D github:bynanci/storytelling
npx skills add ./node_modules/@bynanci/storytelling/skills/storytelling --skill storytelling -a codex claude-code --copy -y
```

將 `package.json` 與 `package-lock.json` 提交至版本庫。更新依賴版本後，請再執行複製命令。npm 套件內含指引、樣板、範例素材與 Python 腳本。Rust 命令列工具另從原始碼建置。

<a name="use-the-installed-skill"></a>

<a name="叫用已安裝的-skill"></a>

### 開始使用

| 環境 | 如何開始 |
| --- | --- |
| Codex | 專案目錄為 `.agents/skills/storytelling/`。輸入 `$storytelling`。 |
| Claude Code | 專案目錄為 `.claude/skills/storytelling/`。輸入 `/storytelling`。 |
| 其他支援 Agent Skills 的工具 | 依該工具的說明載入 Skill。可用工具與呼叫方式由平台決定。 |
| 直接使用 Markdown | 開啟[敘事指引](skills/storytelling/SKILL.zh-TW.md)。依工作情境選擇樣板。 |
| 應用程式或自動化工作 | 呼叫[本機命令列工具](#快速開始)。依命令讀取文字或 JSON 輸出與結束碼。 |

手動安裝時，請複製含授權檔的完整 Skill 目錄。[Skill 目錄](skills/storytelling) · [跨平台安裝與驗證](https://github.com/bynanci/storytelling/blob/main/docs/PLATFORMS.zh-TW.md)

<a name="working-with-sources-and-revisions"></a>

## 核對素材與修改草稿

- **核對來源。** 對照原始筆記與草稿。分別標示作者提供的事實、推論與創作提議。[修訂範例](skills/storytelling/references/examples.zh-TW.md)
- **安排工作順序。** 先完成後續步驟需要的工作。Python 腳本可規劃相依步驟，也可檢查執行紀錄。[流程與重試規則](skills/storytelling/references/graph-loops.zh-TW.md)
- **查找佐證。** 若多份文件已有 Graphify 索引，可用索引定位段落。請回到原文核對主張。[索引查詢指引](skills/storytelling/references/graphify-tools.zh-TW.md)

<a name="quick-start"></a>
<a name="optional-rust-cli"></a>
<a name="cross-media-rust-support"></a>

<a name="選用-rust-cli"></a>

<a name="rust-對跨模態創作的支援進度"></a>

## 快速開始

命令列工具（CLI）可獨立在本機執行。以下範例使用 Rust 1.89.0 與開發版 `0.1.0-dev.0`。執行範例不需要 AI 訂閱、模型金鑰或剪輯器。[Rust 工具鏈安裝程式](https://rustup.rs/)

```sh
git clone https://github.com/bynanci/storytelling.git
cd storytelling
cargo run --quiet --locked -p storytelling-cli -- render-narrative tests/fixtures/narrative-v1/n03-authoring-v1.json
```

這份「鄰居修門」範例會輸出一份口述大綱與兩份插畫構圖。每份草稿保留來源識別碼、修訂版號與來源類別。紅傘只出現在寬景構圖。工具依輸入中宣告的文字輸出草稿，無須讀取媒體檔案。[敘事文件格式（英文）](https://github.com/bynanci/storytelling/blob/main/docs/narrative/NARRATIVE_DOCUMENT_V1.md) · [完整輸出與測試紀錄（英文）](https://github.com/bynanci/storytelling/blob/main/docs/validation/n03-narrative-cli.md)

### 查出修改影響哪些草稿

修訂紀錄採用獨立格式 `NarrativeRevisionLedgerV1`。紀錄從一份敘事文件開始，依序追加事件。用第一個命令查閱來源舊版、草稿與作者決定。後兩個命令查詢指定的來源修訂事件。

```sh
cargo run --quiet --locked -p storytelling-cli -- track-narrative-revisions tests/fixtures/narrative-v1/n04-revision-history-v1.json
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json b-only-detail-change
```

更正共用觀察後，口述與兩份插畫都要複查。只改構圖 B 的細節時，僅 `illustration-close` 需要複查。工具依每版草稿直接引用的來源計算影響。經其他來源間接引用時，由作者另行核對。紀錄可保存作者決定，無法驗證輸入者身分。[修訂紀錄格式（英文）](https://github.com/bynanci/storytelling/blob/main/docs/narrative/NARRATIVE_REVISION_LEDGER_V1.md) · [預期輸出與驗證紀錄（英文）](https://github.com/bynanci/storytelling/blob/main/docs/validation/n04-narrative-revisions.md)

<a name="video-integrations"></a>

### 影片整合

```sh
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

第一個範例回傳 `ok: true`，列出一份素材與兩個鏡頭。第二個範例依作者指定的順序，把 `detail` 排在 `opening` 前方。輸出為 JSON。[規劃輸入範例](https://github.com/bynanci/storytelling/blob/main/tests/fixtures/planner-v1/valid.json) · [CLI 契約](https://github.com/bynanci/storytelling/blob/main/docs/CONTRACTS.zh-TW.md)

| 工作 | 工具支援範圍 |
| --- | --- |
| 檢查鏡頭計畫 | 檢查素材引用、剪輯限制與鏡頭順序。 |
| 比對檔案內容 | 對指定檔案或標準輸入計算 SHA-256。輸入上限為 16 MiB。 |
| 交換剪輯資料 | 接受指定格式的單一連續畫面軌。支援靜態圖片與影片硬切。影片匯出須明確選擇只保留畫面。可交付 OpenTimelineIO（OTIO）檔案。 |
| 審閱剪輯提案 | 儲存審閱紀錄。以目前輸入比對該紀錄。可將選定剪點匯出為 OTIO。 |

剪輯器整合測試涵蓋 Linux 上的 25 fps 專案。驗證項目包括儲存後重開、重新匯入與 45 格 CPU 預覽。另一項測試在匯入回應遺失後讀回專案狀態。正式環境的寫入仍在規劃中。音訊交接與全解析度輸出仍待驗證。[CLI 指南與輸入格式](https://github.com/bynanci/storytelling/blob/main/docs/CLI_GUIDE.zh-TW.md) · [剪輯器與版本紀錄（英文）](https://github.com/bynanci/storytelling/blob/main/docs/migration/records/M1-03.md) · [復原測試（英文）](https://github.com/bynanci/storytelling/blob/main/docs/validation/m3-03-filmcraft-execution.md)

<a name="architecture-and-contributions"></a>

## 架構與參與開發

Skill 位於 `skills/storytelling/`。Rust 工作區分為四個套件。

| 套件 | 職責 |
| --- | --- |
| `storytelling-core` | 定義資料交換型別與精確時間。 |
| `storytelling-engine` | 在記憶體內驗證輸入與規劃順序。 |
| `storytelling-cli` | 讀取輸入，執行命令，寫出結果。 |
| `storytelling-journal` | 使用 SQLite 保存操作紀錄。 |

提交新樣板時，請附上合成素材、預期輸出與檢查方式。回報平台相容性時，請列出平台版本、安裝位置與實際結果。[貢獻指南](https://github.com/bynanci/storytelling/blob/main/CONTRIBUTING.zh-TW.md) · [架構](https://github.com/bynanci/storytelling/blob/main/docs/ARCHITECTURE.zh-TW.md) · [工作清單](https://github.com/bynanci/storytelling/blob/main/TASKS.zh-TW.md) · [開發路線圖](https://github.com/bynanci/storytelling/blob/main/ROADMAP.zh-TW.md) · [需求追溯](https://github.com/bynanci/storytelling/blob/main/docs/REQUIREMENTS.zh-TW.md) · [開發流程](https://github.com/bynanci/storytelling/blob/main/docs/SDLC.zh-TW.md)

[回報問題](https://github.com/bynanci/storytelling/issues/new/choose) · [安全政策](https://github.com/bynanci/storytelling/blob/main/SECURITY.zh-TW.md)

<a name="license"></a>

## 授權

專案的原創程式碼、文件及 Skill 採用 MIT 授權。逐檔清單另列採用 Apache-2.0 的元件。清單保留各元件的來源與上游聲明。[MIT 授權](LICENSE) · [Skill 授權](skills/storytelling/LICENSE) · [Apache-2.0 授權](https://github.com/bynanci/storytelling/blob/main/LICENSES/Apache-2.0.txt) · [逐檔來源與聲明（英文）](https://github.com/bynanci/storytelling/blob/main/PROVENANCE.md)
