# Use Storytelling in your project

<a name="use-storytelling-in-your-project--跨平台使用"></a>
<a name="在專案中使用-storytelling"></a>

[繁體中文](PLATFORMS.zh-TW.md) · [Project guide](../README.md)

The [skill folder](../skills/storytelling) contains instructions, examples, a license, and optional Python scripts. An agent is an application that can load these instructions and call tools. You can also follow the instructions in a document or use them in a conversation.

<a name="用-npx-安裝"></a>
<a name="install-from-github--直接安裝到-agent"></a>

## Install with npx

The installer requires Node.js 22.20 or later. Run these commands from the root of your project. The list should include `storytelling`. [Installer reference](https://github.com/vercel-labs/skills/blob/main/docs/commands.md)

```sh
npx skills add bynanci/storytelling --skill storytelling -a codex claude-code
npx skills list
```

| Install target | Option |
| --- | --- |
| One agent | Use `-a codex` or `-a claude-code`. |
| Select agents interactively | Omit `-a` and the agent names. |
| Install for your account | Add `-g`. |
| Use a copy where symbolic links are unavailable | Add `--copy` to the `add` command. |

<a name="用-npm-固定版本"></a>
<a name="install-with-npm--以-npm-管理版本"></a>

## Pin a version with npm

Use a project that contains a `package.json` file. The first command installs the package from GitHub. The second command copies the skill into the agent directories.

```sh
npm install -D github:bynanci/storytelling
npx skills add ./node_modules/@bynanci/storytelling/skills/storytelling --skill storytelling -a codex claude-code --copy -y
```

1. Run `npx skills list`. Check that the list contains `storytelling`.
2. Commit `package.json` and `package-lock.json`. Other contributors can install the same revision from the lockfile.
3. After an update, repeat the second command to replace the copy.

The first command is sufficient to read the guides or run the Python scripts. Build the optional Rust command-line interface (CLI) separately from source. The install example uses GitHub. Installation by package name requires publication to the npm registry.

<a name="從-npm-依賴同步選用"></a>
<a name="optional-dependency-sync--選用依賴同步"></a>

### Sync the skill from an npm dependency

Use this optional preview command in a project without a previous Storytelling installation. Install the npm dependency from the previous section first. Then run the command below to create links. An existing `skills add` installation takes precedence. [Sync command reference](https://github.com/vercel-labs/skills/blob/main/docs/commands.md#skills-experimental_sync)

```sh
npx --yes skills@1.8.1 experimental_sync --include @bynanci/storytelling --no-remote -a codex claude-code -y
```

This command specifies the installer version, package, and agents. The `--no-remote` option skips remote skill declarations. Commit `skills-lock.json` with the npm lockfile. For a clean continuous integration (CI) run:

1. Run `npm ci`.
2. Run the same sync command with `--frozen-lockfile`.

<a name="平台路徑與手動安裝"></a>
<a name="agent-paths-and-manual-installation--平台路徑與手動安裝"></a>

## Agent paths and manual installation

Copy the complete `skills/storytelling/` folder to the destination. Project paths start at the root of the receiving project. The tilde (`~`) represents your home directory. If the destination already contains this skill, compare the folders before you change them.

| Agent | Project path | Account path | How to use the skill |
| --- | --- | --- | --- |
| Codex | `.agents/skills/storytelling/` | `~/.agents/skills/storytelling/` | Select the skill in `/skills`, or enter `$storytelling` with your request. [Codex documentation](https://developers.openai.com/codex/skills/) |
| Claude Code | `.claude/skills/storytelling/` | `~/.claude/skills/storytelling/` | Check the list with `/skills`. Enter `/storytelling` with your request. [Claude Code documentation](https://code.claude.com/docs/en/skills) |
| Gemini CLI | `.gemini/skills/storytelling/` or `.agents/skills/storytelling/` | `~/.gemini/skills/storytelling/` or `~/.agents/skills/storytelling/` | Run `/skills list` in a trusted workspace. After a manual copy, run `/skills reload`. Name Storytelling in your request. [Gemini CLI documentation](https://geminicli.com/docs/cli/using-agent-skills/) |
| Claude web | Account settings | Account settings | Select **Customize → Skills → + → Create skill → Upload a skill**. Upload the ZIP file. Enable the skill. The account requires access to skills and code execution. Organization settings may restrict access. [Claude documentation](https://support.claude.com/en/articles/12512180-use-skills-in-claude) |

In the source repository, `.agents/skills/storytelling` links to `skills/storytelling`. On Windows, Git can create a text file in place of that link. If this happens, use the copy option for installation.

Claude web accepts a ZIP file. Put only the `storytelling/` folder at the top of the archive. Check that the folder contains `SKILL.md`, `references/`, and `scripts/`. [ZIP format](https://support.claude.com/en/articles/12512198-how-to-create-custom-skills)

Run this POSIX shell command from the source repository to package the committed version. It refuses to replace an existing ZIP file.

```sh
(set -C; git archive --format=zip --prefix=storytelling/ HEAD:skills/storytelling > storytelling-skill.zip)
```

<a name="試用一次"></a>
<a name="try-one-request--試用"></a>

## Try the skill

> Two neighbors repaired a gate together. Write a short spoken account and two poster briefs. The second poster may include an imaginary red umbrella.

Expect one account and two briefs. Only the second poster may include the red umbrella. The wording can differ across agents. [More examples](../skills/storytelling/references/examples.md)

<a name="獨立使用"></a>
<a name="standalone-use--獨立入口"></a>

## Use the guides and tools independently

| Task | Procedure |
| --- | --- |
| Write by hand or with a chat assistant | Read the instructions for your medium. Draft from your material or supply it with the instructions in a conversation. [Skill instructions](../skills/storytelling/SKILL.md) · [Workflow guide](../skills/storytelling/references/graph-loops.md) |
| Check work order | Run the planner with Python 3. Supply JSON on standard input. Use the trace script to compare the recorded work with the plan. Both scripts work offline. [Planner](../skills/storytelling/scripts/graph_plan.py) · [Trace script](../skills/storytelling/scripts/check_trace.py) |
| Check a video plan | Run `cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json` from the source repository. This example uses the native plan format. Snapshot import and timeline export use different inputs. [CLI guide](CLI_GUIDE.md) |

<a name="驗證範圍"></a>
<a name="compatibility-evidence--驗證範圍"></a>

## What the tests cover

| Area | Evidence |
| --- | --- |
| Package and installation | Tests compare the package with the source files. They cover npx, npm, and the preview sync command with a lockfile. The installation tests use Codex and Claude Code directories. |
| Agent discovery | A test checks whether Codex finds the copied skill in a clean project. Instructions for Claude Code, Gemini CLI, and Claude web follow their product documentation. Use the example request to check the result in your installation. |
| Local tools | Project CI runs the Python scripts and Rust CLI tests. The model in the agent influences wording and quality. |

[Codex discovery test](https://github.com/bynanci/storytelling/pull/58) · [CI workflow](https://github.com/bynanci/storytelling/actions/workflows/ci.yml) · [Agent Skills specification](https://agentskills.io/specification)
