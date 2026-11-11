# Storytelling

[繁體中文](README.zh-TW.md) · [Platform guide](https://github.com/bynanci/storytelling/blob/main/docs/PLATFORMS.md) · [Examples](skills/storytelling/references/examples.md) · [Contributing](CONTRIBUTING.md) · [CI](https://github.com/bynanci/storytelling/actions/workflows/ci.yml)

Storytelling is an open-source [Agent Skill](skills/storytelling/SKILL.md) for turning source material into stories, screenplay scenes, illustration briefs, business copy, brainstorms and shot plans. Give it a page of interview notes, for example, and ask for a spoken introduction and two poster concepts. Each version can use a different angle while keeping the speaker's account and the artist's additions distinguishable.

The skill travels as one folder. Add it to a writing project, a client workspace or a software repository. Drafting uses the agent and material you choose. A separate Rust CLI offers local, repeatable checks for structured video plans.

## Start with the skill

### Install for an agent

From the project where you want to use Storytelling, run the following with Node.js 22.20 or later. Keep just the agent name you use, or omit `-a codex claude-code` to choose interactively.

```sh
npx skills add bynanci/storytelling --skill storytelling -a codex claude-code
npx skills list
```

### Install as an npm dependency

Use this route when you want your project’s npm lockfile to record the skill revision. In a project with a `package.json`, install the package from GitHub, then copy the bundled skill into the agent directories.

```sh
npm install -D github:bynanci/storytelling
npx skills add ./node_modules/@bynanci/storytelling/skills/storytelling --skill storytelling -a codex claude-code --copy -y
```

Commit `package.json` and `package-lock.json`. Rerun the second command after changing the dependency revision. The npm package contains the skill instructions, references and Python helpers. Build the optional Rust CLI separately using the source quick start below.

### Use the installed skill

The table gives the project locations and invocation syntax. For a manual install, copy the complete [skill folder](skills/storytelling), including its license, to the listed destination.

| Where you work | How to start |
| --- | --- |
| Codex in a project | Place the folder at `.agents/skills/storytelling/` in the project. Invoke `$storytelling`. |
| Claude Code in a local project | Place the same folder at `.claude/skills/storytelling/` in the project. Invoke `/storytelling`. |
| Another Agent Skills client | Use the client's documented skill loader. Discovery paths, invocation syntax and available tools belong to the client. |
| Plain Markdown | Open [SKILL.md](skills/storytelling/SKILL.md) and the references it links. A writer or team can follow the workflow by hand. |
| A script, CI job or application | Use the [standalone CLI](#quick-start) with structured input, JSON output and exit codes. |

After invoking the skill, try this request in either agent:

```text
Two neighbors fixed a gate together. Write a two-sentence account for the
neighborhood newsletter and two poster concepts. Add an imaginary red umbrella
to the second poster only.
```

The newsletter should preserve the observation. The two posters should make different visual choices, and only the second should introduce the umbrella. This gives you a small, checkable first use in your own project. For personal installs, other clients and verification status, see the [platform guide](https://github.com/bynanci/storytelling/blob/main/docs/PLATFORMS.md).

## What works today

- **Draft and adapt.** Organize notes, explore alternatives, preserve source attribution and revise the versions that use a changed detail. [Worked examples](skills/storytelling/references/examples.md)
- **Coordinate complex work.** Use the graph-loop playbook for dependencies, independent branches, review and bounded retries. The skill includes optional Python helpers for planning a dependency graph and checking a recorded execution order. [Playbook](skills/storytelling/references/graph-loops.md)
- **Find supporting material.** When a question spans documents and a suitable Graphify index is available, locate candidate passages and check them against the originals. [Graphify guide](skills/storytelling/references/graphify-tools.md)

## Quick start

### Optional Rust CLI

The development version is `0.1.0-dev.0`, available from source with Rust 1.89.0. No AI subscription, model API key or video editor is required for these examples. With [rustup](https://rustup.rs/) installed, run these commands from your terminal.

```sh
git clone https://github.com/bynanci/storytelling.git
cd storytelling
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

`validate` reports `ok: true` for one asset and two shots. `plan-story` prints JSON with `detail` before `opening`, preserving the author's explicit order even though `opening` has a higher score. To integrate your own application, start with these input shapes and the [CLI contracts](docs/CONTRACTS.md). [Planner input](tests/fixtures/planner-v1/valid.json) · [CLI guide](docs/CLI_GUIDE.md)

### Video integrations

| Input | Commands | Result |
| --- | --- | --- |
| A plan, brief or fixed-window planning request | `validate`, `audit-brief`, `plan-story` | Check declared references and constraints, or propose a shot order. |
| Local bytes and an expected SHA-256 digest | `verify-source` | Verify up to 16 MiB from an explicit file or stdin. |
| A supported OpenFilm composition snapshot | `import-openfilm`, `export-otio` | Convert a single contiguous picture track into a plan or OpenTimelineIO (OTIO). |
| A snapshot and its review context | `prepare-review`, `check-review` | Save a review record, then compare that record with current inputs. |
| A saved proposal, matching snapshot and review context | `prepare-story-review`, `export-story-otio`, `check-story-review` | Export selected cuts as OTIO. Compare a saved review bundle with the current inputs to detect changes. |

The snapshot adapter reads OpenFilm composition exports. It accepts stills and video cuts on the supported picture track. Video export requires an explicit picture-only choice. A FilmCraft v0.18.0 test on Linux at 25 fps covered save/reopen, OTIO reimport and a 45-frame CPU preview. A separate isolated test recovered from a lost import reply by reading the resulting project state. Production editor writes are planned. Audio handoff and full-resolution output await verification. [FilmCraft test record](docs/migration/records/M1-03.md) · [Recovery test](docs/validation/m3-03-filmcraft-execution.md) · [Review contract](docs/migration/PLANNER_OTIO_REVIEW_V1.md)

## Cross-media direction

Agent-assisted drafting is available today. The roadmap covers a versioned cross-media document format, repeatable source-revision checks, commercial claim tracing and author trials. [Roadmap](ROADMAP.md) · [Narrative backlog](TASKS.md#cross-media-narrative-backlog)

## Architecture and contributions

The portable skill lives in `skills/storytelling/`. The Rust workspace contains four crates. `storytelling-core` owns wire types and exact time, `storytelling-engine` owns in-memory planning and validation, `storytelling-cli` handles process I/O, and `storytelling-journal` provides a durable SQLite operation log. [Architecture](docs/ARCHITECTURE.md)

Contributions from other domains and agent clients are welcome. For a new workflow, include a small synthetic request, the expected output and how a contributor can check it. For a compatibility report, include the client, version, installation location and what you observed. Consult the [contribution guide](CONTRIBUTING.md), [open tasks](TASKS.md), [requirements](docs/REQUIREMENTS.md) and [development process](docs/SDLC.md). [Report an issue](https://github.com/bynanci/storytelling/issues/new/choose) · [Security policy](SECURITY.md)

## License

The portable skill includes its [MIT license](skills/storytelling/LICENSE). Original project code and documentation use the [MIT License](LICENSE). Identified OpenFilm-informed components use [Apache-2.0](LICENSES/Apache-2.0.txt) with a retained [upstream notice](NOTICE.openfilm). [Per-file provenance](PROVENANCE.md)
