# Use Storytelling in your project · 跨平台使用

The distributable source is [`skills/storytelling/`](../skills/storytelling). It includes the instructions, license, references and optional Python scripts. Start from your own project and choose one installation method per skill. The `skills` installer requires Node.js 22.20 or later. [Installer reference](https://github.com/vercel-labs/skills/blob/main/docs/commands.md)

使用自己的筆記、場景或產品資料即可開始。要讓 Agent 載入 Skill，可依下列方式擇一安裝。若要人工創作，可直接閱讀套件內的 Markdown 指引。

## Install from GitHub · 直接安裝到 Agent

Run these commands in the receiving project. `skills list` should show `storytelling`. Use `-a codex` or `-a claude-code` for one host, omit the agent option for interactive selection, or add `-g` for a user-wide install.

在要使用 Skill 的專案執行。在結果中確認有 `storytelling`，再依下方平台表的方式叫用。若專案環境不支援符號連結，可在 `add` 指令加上 `--copy`。

```sh
npx skills add bynanci/storytelling --skill storytelling -a codex claude-code
npx skills list
```

## Install with npm · 以 npm 管理版本

From a project with a `package.json`, install the GitHub dependency. Then activate the skill for your agents from the installed package. This route ships the skill assets. The optional Rust program is built separately from a source checkout.

在有 `package.json` 的專案中執行。第一行把 Skill 裝進 `node_modules`，第二行把它複製到所選 Agent 的目錄。若只想手動讀取指引或執行 Python 腳本，第一行即可。

```sh
npm install -D github:bynanci/storytelling
npx skills add ./node_modules/@bynanci/storytelling/skills/storytelling --skill storytelling -a codex claude-code --copy -y
```

Run `npx skills list` to check the result. Commit the npm manifest and lockfile. After changing the dependency revision, rerun the local `skills add` command to refresh the copies. The GitHub package spec works before a registry release. The registry spec `@bynanci/storytelling` becomes usable when a maintainer publishes it under that scope.

用 `npx skills list` 確認安裝結果。提交 `package.json` 與 `package-lock.json` 可讓團隊還原同一版。之後更新依賴版本時，再執行第二行。現階段使用上面的 GitHub 套件來源，待正式發布至 npm registry 後才可直接使用套件名稱安裝。

### Optional dependency sync · 選用依賴同步

For a project that wants links to the npm dependency, run the npm install line above, then use the following preview command as the activation step. A linked skill follows the installed dependency revision. Choose this method in a project without an existing Storytelling installation. The upstream CLI gives a previous `skills add` installation priority. [Experimental sync reference](https://github.com/vercel-labs/skills/blob/main/docs/commands.md#skills-experimental_sync)

想直接連到 `node_modules` 中的版本時，在尚未安裝 Storytelling 的專案先執行上面的 `npm install`，再以下列命令完成啟用。此為上游預覽功能，範例固定 CLI 版本、指定套件與 Agent，並略過遠端 Skill 宣告。

```sh
npx --yes skills@1.8.1 experimental_sync --include @bynanci/storytelling --no-remote -a codex claude-code -y
```

With the preview workflow, create and commit `skills-lock.json` alongside the npm lockfile. On a clean CI checkout, run `npm ci` before the same sync command with `--frozen-lockfile`.

使用預覽同步時，請連同 npm 的鎖檔一起提交 `skills-lock.json`。CI 先以 `npm ci` 還原依賴，再用同一條同步命令加上 `--frozen-lockfile` 檢查版本與內容。

## Agent paths and manual installation · 平台路徑與手動安裝

Project paths below are relative to the receiving project. User paths begin with `~`. For a manual install, copy the entire source skill folder into one of the matching directories. In a Storytelling source checkout, Git links `.agents/skills/storytelling` to `skills/storytelling` for Codex on systems that check out symbolic links.

手動安裝時，從來源複製整份 `skills/storytelling/` 到下表對應位置。專案路徑從接收專案根目錄算起，`~` 是使用者家目錄。若已經有同名資料夾，先比對內容再更新。在 Windows 上若 Git 以文字檔簽出符號連結，可將完整 Skill 複製到另一個專案，或在該專案以 `--copy` 安裝。

| Platform | This project / 專案 | All projects / 個人 | Discover and use / 確認與使用 |
| --- | --- | --- | --- |
| Codex | `.agents/skills/storytelling/` | `~/.agents/skills/storytelling/` | Select `storytelling` using `/skills`, or invoke `$storytelling` followed by your request. [Codex skills](https://developers.openai.com/codex/skills/) |
| Claude Code, local | `.claude/skills/storytelling/` | `~/.claude/skills/storytelling/` | Check `/skills` and enter `/storytelling` followed by your request. [Claude Code skills](https://code.claude.com/docs/en/skills) |
| Gemini CLI | `.gemini/skills/storytelling/` or `.agents/skills/storytelling/` | `~/.gemini/skills/storytelling/` or `~/.agents/skills/storytelling/` | In a trusted workspace, use `/skills list` (`/skills reload` after copying). Ask in natural language to use Storytelling. [Gemini CLI skills](https://geminicli.com/docs/cli/using-agent-skills/) |
| Claude on the web | — | Upload and enable for your account / 帳號上傳並啟用 | Open **Customize → Skills → + → Create skill → Upload a skill**, upload the ZIP and enable it. Ask for a draft in natural language. Your account needs access to Skills and code execution. Organization settings may control availability. [Claude Help](https://support.claude.com/en/articles/12512180-use-skills-in-claude) |

To package for Claude web, compress **only the `storytelling` folder**. Open the archive and check for `storytelling/SKILL.md`, `storytelling/references/` and `storytelling/scripts/`. On a local Git checkout, the following command archives the committed skill; the POSIX shell will refuse to replace an existing ZIP. [Claude packaging format](https://support.claude.com/en/articles/12512198-how-to-create-custom-skills)

Claude 網頁版只需壓縮 `storytelling` 資料夾。ZIP 內第一層應為 `storytelling/`，再往下才是 `SKILL.md` 與其餘資源。可用系統壓縮工具，或在來源專案根目錄執行：

```sh
(set -C; git archive --format=zip --prefix=storytelling/ HEAD:skills/storytelling > storytelling-skill.zip)
```

## Try one request · 試用

> Two neighbors repaired a gate. Give me a short spoken story and two different poster concepts. Add a red umbrella only to the second poster.
>
> 兩個鄰居一起修好了門。請寫一段適合分享的口述，再提供兩種海報構圖。第二種構圖可以加上一把想像的紅傘。

Check that you receive one spoken draft and two visual concepts. The umbrella belongs only to the second poster. This exercises the same skill in each host; wording may vary by model.

驗收時要看到一份口述、兩種不同的構圖，且只有第二種構圖出現紅傘。[Worked examples / 更多案例](../skills/storytelling/references/examples.md)

## Standalone use · 獨立入口

| Goal / 用途 | Entry point / 入口 |
| --- | --- |
| Follow the workflow by hand or in a chat with no skill installer / 人工使用或一般對話 | Open [SKILL.md](../skills/storytelling/SKILL.md). Use the section for your medium, or give the relevant instructions and notes to your assistant. You can follow the [graph loop playbook](../skills/storytelling/references/graph-loops.md) by hand. |
| Check a dependency plan locally / 用本機 Python 核對工作順序 | Run the packaged [graph planner](../skills/storytelling/scripts/graph_plan.py) with Python 3 and a JSON document on stdin. The [trace checker](../skills/storytelling/scripts/check_trace.py) checks the recorded order and outcome. Both run independently of AI services. |
| Validate a structured video plan / 獨立驗證結構化影片計畫 | From a Storytelling source checkout, run `cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json`. The example uses the native plan format. The [CLI guide](CLI_GUIDE.md) also documents the optional OpenFilm snapshot and OTIO workflows with their specific inputs. |

## Compatibility evidence · 驗證範圍

The distribution checks compare the packed files with the source and exercise both stable install routes for the Codex and Claude Code directories. They also replay the optional npm sync with its lockfile frozen. Codex CLI discovery was exercised in a clean project with a copied skill. The project’s CI exercises the Python helpers and Rust CLI. The Claude Code and Gemini CLI paths and the Claude web upload procedure follow their respective vendors’ documentation; the ZIP layout can be checked locally. Validate activation in your target client with the sample request above. Authoring results depend on the host model. [Codex discovery record](https://github.com/bynanci/storytelling/pull/58) · [Workflow checks](https://github.com/bynanci/storytelling/actions/workflows/ci.yml) · [Agent Skills specification](https://agentskills.io/specification)

套件測試會比對安裝後的檔案，確認兩種安裝路徑都能寫入 Codex 與 Claude Code 的專案目錄，並檢查選用的 npm 同步能依鎖檔重跑。Codex 的專案探索已有實測。Python 與 Rust 的檢查由 CI 執行。Claude Code、Gemini CLI 和 Claude 網頁版的安裝方式依各平台官方文件整理，尚未在這三個用戶端逐一實測。可用上方同一組請求檢查自己的客戶端是否啟用，以及輸出是否符合要求。
