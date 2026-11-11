# Use Storytelling in your project · 跨平台使用

The portable package is the [`storytelling/` folder](../.agents/skills/storytelling). Copy the whole folder, including its license, references and scripts. Any project can supply its own notes, scenes or product brief. A compatible AI host can draft from those materials immediately. Video interchange and deterministic checks are optional.

可攜套件就是 `storytelling/` 資料夾。將整個資料夾放到自己的專案，開啟慣用的 AI 工具，就能從自己的筆記、場景或產品資料開始。內容起草只需要能閱讀 Skill 的 AI。想自行照流程創作，或從終端機執行確定性檢查，可以直接使用本頁的獨立入口。

## Install and invoke · 安裝與呼叫

Project paths below are relative to the **receiving project**. User paths begin with `~` and apply across local projects. Pick one scope. If a `storytelling` folder already exists in that location, compare the files before updating it.

專案路徑從**要使用 Skill 的專案**根目錄算起；`~` 指個人家目錄。選擇一個安裝範圍，已有同名資料夾時先比對版本。

| Platform | This project / 專案 | All projects / 個人 | Discover and use / 確認與使用 |
| --- | --- | --- | --- |
| Codex | `.agents/skills/storytelling/` | `~/.agents/skills/storytelling/` | Select `storytelling` using `/skills`, or invoke `$storytelling` followed by your request. [Codex skills](https://developers.openai.com/codex/skills/) |
| Claude Code, local | `.claude/skills/storytelling/` | `~/.claude/skills/storytelling/` | Check `/skills` and enter `/storytelling` followed by your request. [Claude Code skills](https://code.claude.com/docs/en/skills) |
| Gemini CLI | `.gemini/skills/storytelling/` or `.agents/skills/storytelling/` | `~/.gemini/skills/storytelling/` or `~/.agents/skills/storytelling/` | In a trusted workspace, use `/skills list` (`/skills reload` after copying). Ask in natural language to use Storytelling. [Gemini CLI skills](https://geminicli.com/docs/cli/using-agent-skills/) |
| Claude on the web | — | Upload and enable for your account / 帳號上傳並啟用 | Open **Customize → Skills → + → Create skill → Upload a skill**, upload the ZIP and enable it. Ask for a draft in natural language. Your account needs access to Skills and code execution. Organization settings may control availability. [Claude Help](https://support.claude.com/en/articles/12512180-use-skills-in-claude) |

To package for Claude web, compress **only the `storytelling` folder**. Open the archive and check for `storytelling/SKILL.md`, `storytelling/references/` and `storytelling/scripts/`. On a local Git checkout, the following command archives the committed skill; the POSIX shell will refuse to replace an existing ZIP. [Claude packaging format](https://support.claude.com/en/articles/12512198-how-to-create-custom-skills)

Claude 網頁版只需壓縮 `storytelling` 資料夾。ZIP 內第一層應為 `storytelling/`，再往下才是 `SKILL.md` 與其餘資源。可用系統壓縮工具，或在來源專案根目錄執行：

```sh
(set -C; git archive --format=zip --prefix=storytelling/ HEAD:.agents/skills/storytelling > storytelling-skill.zip)
```

## Try one request · 試用

> Two neighbors repaired a gate. Give me a short spoken story and two different poster concepts. Add a red umbrella only to the second poster.
>
> 兩個鄰居一起修好了門。請寫一段適合分享的口述，再提供兩種海報構圖。第二種構圖可以加上一把想像的紅傘。

Check that you receive one spoken draft and two visual concepts. The umbrella belongs only to the second poster. This exercises the same skill in each host; wording may vary by model.

驗收時要看到一份口述、兩種不同的構圖，且只有第二種構圖出現紅傘。[Worked examples / 更多案例](../.agents/skills/storytelling/references/examples.md)

## Standalone use · 獨立入口

| Goal / 用途 | Entry point / 入口 |
| --- | --- |
| Follow the workflow by hand or in a chat with no skill installer / 人工使用或一般對話 | Open [SKILL.md](../.agents/skills/storytelling/SKILL.md). Use the section for your medium, or give the relevant instructions and notes to your assistant. You can follow the [graph loop playbook](../.agents/skills/storytelling/references/graph-loops.md) by hand. |
| Check a dependency plan locally / 用本機 Python 核對工作順序 | Run the packaged [graph planner](../.agents/skills/storytelling/scripts/graph_plan.py) with Python 3 and a JSON document on stdin. The [trace checker](../.agents/skills/storytelling/scripts/check_trace.py) checks the recorded order and outcome. Both run independently of AI services. |
| Validate a structured video plan / 獨立驗證結構化影片計畫 | From a Storytelling source checkout, run `cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json`. The example uses the native plan format. The [CLI guide](CLI_GUIDE.md) also documents the optional OpenFilm snapshot and OTIO workflows with their specific inputs. |

## Compatibility evidence · 驗證範圍

Codex CLI discovery was exercised in a clean project with a copied skill. The project’s CI exercises the Python helpers and Rust CLI. The Claude Code and Gemini CLI paths and the Claude web upload procedure follow their respective vendors’ documentation; the ZIP layout can be checked locally. Validate activation in your target client with the sample request above. Authoring results depend on the host model. [Codex discovery record](https://github.com/bynanci/storytelling/pull/58) · [Workflow checks](https://github.com/bynanci/storytelling/actions/workflows/ci.yml) · [Agent Skills specification](https://agentskills.io/specification)

Codex 的專案探索已有實測；Python 與 Rust 的檢查由 CI 執行。Claude Code、Gemini CLI 和 Claude 網頁版的安裝方式依各平台官方文件整理，尚未在這三個用戶端逐一實測。可用上方同一組請求檢查自己的客戶端是否啟用，以及輸出是否符合要求。
