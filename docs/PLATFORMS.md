# 在專案中使用 Storytelling

Storytelling 可在代理程式、一般對話或本機使用。代理程式是能載入指令並呼叫工具的應用程式。Skill 是一份含指引、範例與選用腳本的資料夾。[Skill 原始檔](../skills/storytelling)

## 用 npx 安裝

在要使用 Skill 的專案根目錄執行。安裝工具需要 Node.js 22.20 以上版本。[安裝工具說明](https://github.com/vercel-labs/skills/blob/main/docs/commands.md)

```sh
npx skills add bynanci/storytelling --skill storytelling -a codex claude-code
npx skills list
```

清單應顯示 `storytelling`。請依安裝目標選用參數。

| 目標 | 參數 |
| --- | --- |
| 只安裝到 Codex 或 Claude Code | 使用 `-a codex` 或 `-a claude-code`。 |
| 互動選擇代理程式 | 省略 `-a` 與代理程式名稱。 |
| 安裝到使用者目錄 | 加上 `-g`。 |
| 專案無法使用符號連結 | 在 `add` 指令加上 `--copy`。 |

## 用 npm 固定版本

在有 `package.json` 的專案執行下列指令。第一行從 GitHub 安裝套件。第二行將 Skill 複製到指定的代理程式目錄。

```sh
npm install -D github:bynanci/storytelling
npx skills add ./node_modules/@bynanci/storytelling/skills/storytelling --skill storytelling -a codex claude-code --copy -y
```

1. 執行 `npx skills list`。確認清單列出 `storytelling`。
2. 提交 `package.json` 與 `package-lock.json`。團隊成員可依鎖定檔還原同一版。
3. 更新套件後，再執行第二行以更新副本。

只需閱讀指引或執行 Python 腳本時，第一行即可。Rust 命令列工具須從原始碼另行建置。現行安裝範例使用 GitHub 來源。維護者發布到 npm 登錄檔後，才可用套件名稱直接安裝。

### 從 npm 依賴同步（選用）

若專案尚未安裝 Storytelling，可改用預覽版同步指令。先執行上節的 `npm install`。接著執行下列指令，以連結取代副本。舊有的 `skills add` 安裝會優先載入。[同步指令說明](https://github.com/vercel-labs/skills/blob/main/docs/commands.md#skills-experimental_sync)

```sh
npx --yes skills@1.8.1 experimental_sync --include @bynanci/storytelling --no-remote -a codex claude-code -y
```

這個範例指定工具版本、套件與代理程式。`--no-remote` 會略過遠端 Skill 宣告。請同時提交 `skills-lock.json` 與 npm 鎖定檔。在乾淨的持續整合（CI）環境先執行 `npm ci`。再執行同一條同步指令，並加上 `--frozen-lockfile`。

## 平台路徑與手動安裝

手動安裝時，請複製整個 `skills/storytelling/` 資料夾。專案路徑從使用專案的根目錄起算。`~` 代表使用者的家目錄。目的地已有同名資料夾時，請先比對內容。

| 平台 | 專案路徑 | 使用者路徑 | 啟用方式 |
| --- | --- | --- | --- |
| Codex | `.agents/skills/storytelling/` | `~/.agents/skills/storytelling/` | 在 `/skills` 選取 Skill。也可輸入 `$storytelling` 與需求。[官方說明](https://developers.openai.com/codex/skills/) |
| Claude Code | `.claude/skills/storytelling/` | `~/.claude/skills/storytelling/` | 用 `/skills` 檢查清單。輸入 `/storytelling` 與需求。[官方說明](https://code.claude.com/docs/en/skills) |
| Gemini CLI | `.gemini/skills/storytelling/` 或 `.agents/skills/storytelling/` | `~/.gemini/skills/storytelling/` 或 `~/.agents/skills/storytelling/` | 在受信任的工作區執行 `/skills list`。手動複製後執行 `/skills reload`。用自然語言指定 Storytelling。[官方說明](https://geminicli.com/docs/cli/using-agent-skills/) |
| Claude 網頁版 | 使用帳號設定 | 使用帳號設定 | 開啟 **Customize → Skills → + → Create skill → Upload a skill**。上傳 ZIP 後啟用。帳號需能使用 Skills 與程式碼執行。組織設定可能限制存取。[官方說明](https://support.claude.com/en/articles/12512180-use-skills-in-claude) |

在 Storytelling 原始碼目錄，Codex 使用 Git 符號連結。`.agents/skills/storytelling` 指向 `skills/storytelling`。若 Windows 將連結簽出為文字檔，請改用前述複製安裝。

Claude 網頁版需要 ZIP 壓縮檔。請只壓縮 `storytelling` 資料夾。開啟 ZIP，確認第一層為 `storytelling/`。其中應包含 `SKILL.md`、`references/` 與 `scripts/`。[ZIP 格式說明](https://support.claude.com/en/articles/12512198-how-to-create-custom-skills)

在原始碼根目錄，可用下列 POSIX shell 指令封裝已提交的版本。若 ZIP 已存在，指令會拒絕覆寫。

```sh
(set -C; git archive --format=zip --prefix=storytelling/ HEAD:skills/storytelling > storytelling-skill.zip)
```

## 試用一次

> 兩個鄰居一起修好了門。請寫一段適合分享的口述，再提供兩種海報構圖。第二種構圖可以加上一把想像的紅傘。

應取得一份口述與兩種構圖。只有第二種構圖可以出現紅傘。不同平台的措辭可能不同。[更多案例](../skills/storytelling/references/examples.md)

## 獨立使用

| 用途 | 操作方式 |
| --- | --- |
| 人工創作或一般對話 | 閱讀與媒介相符的段落。把素材和指引交給對話工具，或自行起草。[Skill 指引](../skills/storytelling/SKILL.md) · [多步驟流程](../skills/storytelling/references/graph-loops.md) |
| 檢查工作順序 | 用 Python 3 執行規劃腳本。從標準輸入提供 JSON。追蹤檢查腳本可比對執行順序與記錄的結果。兩支腳本都可離線執行。[規劃腳本](../skills/storytelling/scripts/graph_plan.py) · [追蹤檢查腳本](../skills/storytelling/scripts/check_trace.py) |
| 檢查影片計畫 | 從原始碼根目錄執行 `cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json`。這份範例使用原生計畫格式。快照匯入與時間軸交換使用不同的輸入。[命令列指南](CLI_GUIDE.md) |

## 驗證範圍

| 項目 | 目前的證據 |
| --- | --- |
| 套件與安裝 | 測試會比對套件內容與原始檔。測試涵蓋 npx 與 npm 兩種安裝方式，以及依鎖定檔重播的預覽同步。目標目錄包括 Codex 與 Claude Code。 |
| 代理程式啟用 | 已在乾淨專案中測試 Codex 能否找到複製的 Skill。Claude Code、Gemini CLI 與 Claude 網頁版的使用方式依官方文件整理。安裝後請使用上方請求實測。 |
| 本機工具 | 專案 CI 會執行 Python 腳本與 Rust 命令列檢查。作品的措辭與品質仍受平台模型影響。 |

[Codex 探索測試](https://github.com/bynanci/storytelling/pull/58) · [CI 工作流程](https://github.com/bynanci/storytelling/actions/workflows/ci.yml) · [Agent Skills 規格](https://agentskills.io/specification)
