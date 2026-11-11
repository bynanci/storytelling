# Storytelling

[繁體中文](README.zh-TW.md) · [CLI guide](docs/CLI_GUIDE.md) · [Roadmap](ROADMAP.md) · [Contributing](CONTRIBUTING.md) · [CI runs](https://github.com/bynanci/storytelling/actions/workflows/ci.yml)

Storytelling helps you shape material into a story, a scene, a single-image composition, commercial copy or a video plan. Start in your agent with the **Storytelling skill**. For video work, the optional local Rust CLI validates structured inputs and exports a reviewable OpenTimelineIO (OTIO) timeline.

## Start with the skill

Open this checkout in Codex and enter a request:

```text
$storytelling Two neighbors fixed a gate together. Draft a short spoken story and two illustration concepts. Keep anything invented for the image out of the spoken account.
```

For another project, copy the entire [storytelling skill folder](.agents/skills/storytelling) into that project's `.agents/skills/`. For all your local Codex projects, copy the folder into `~/.agents/skills/`. In another Agent Skills client, use its skill installation location. The skill includes its references. Spoken drafts, screenwriting, illustration briefs, copy and brainstorming work with the agent alone. The agent can also draft a shot list from ordinary notes. With an appropriate local CLI and structured video inputs, it can additionally run the supported machine checks. [Examples](.agents/skills/storytelling/references/examples.md) · [Codex skills](https://developers.openai.com/codex/skills/) · [Agent Skills format](https://agentskills.io/specification)

## Quick start

### Optional Rust CLI

The source build is `0.1.0-dev.0`. A packaged release is pending. Install Rust with [rustup](https://rustup.rs/). Clone the repository and run the bundled synthetic examples. The repository selects Rust 1.89.0.

```sh
git clone https://github.com/bynanci/storytelling.git
cd storytelling
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

1. `validate` reports `ok: true` for `travel-recap` with one asset and two shots. The result covers the declared IDs and source ranges. To check actual file contents against a digest, use `verify-source`.
2. `plan-story` returns JSON on stdout. In the [example brief](tests/fixtures/planner-v1/valid.json), the author explicitly places `detail` before `opening`. The two shots in `planning.plan` preserve that order, even though `opening` has the higher score.

To try the handoff, follow the [CLI guide](docs/CLI_GUIDE.md) from a saved planner proposal to a review bundle and an `.otio` file. The guide includes the matching snapshot and review context.

## What works today

| Goal | Commands | Scope |
| --- | --- | --- |
| Check and plan | `validate`, `audit-brief`, `plan-story` | Check references, required and excluded assets, and ordering constraints. Select from supplied time ranges within shot and duration limits. |
| Exchange a visual cut | `import-openfilm`, `export-otio` | Read an OpenFilm snapshot with one contiguous picture track of stills or video cuts. Exporting video requires an explicit picture-only choice. Separate audio, effects and animation are unsupported. |
| Review a proposal | `prepare-story-review`, `export-story-otio`, `check-story-review` | Bind the selected proposal and snapshot to the actual OTIO. Check whether the declared inputs, conversion choices or artifact have changed. |
| Review an existing composition | `prepare-review`, `check-review` | Bind a finite composition and its OTIO to a declared review context. A match establishes consistency. Human approval and editor execution remain separate steps. |
| Check supplied bytes | `verify-source` | Compare an explicit file or stdin against a SHA-256 digest, up to 16 MiB. |

OpenFilm supplies the composition snapshot. The target editor owns the working timeline. An isolated test exercised FilmCraft v0.18.0 on Linux at 25 fps, including save/reopen, OTIO reimport and a 45-frame CPU preview. A separate test-only M3-03 proof journals a new isolated project and one proposal import, then resolves a lost reply from exact native readback; it adds no product editor-writing command. Trusted approval and a production executor remain open. Audio handoff and full-resolution output are outside the tested profile. [FilmCraft test record](docs/migration/records/M1-03.md) · [M3-03 proof](docs/validation/m3-03-filmcraft-execution.md) · [Review contract](docs/migration/PLANNER_OTIO_REVIEW_V1.md)

## Cross-media direction

The proposed first use case starts with one account of an event and produces a spoken outline and alternative compositions for a single illustration. An invented visual detail can stay with the illustration while the spoken account keeps its link to the source. A second use case tests commercial copy by tracing each claim to supplied evidence. The agent can draft these forms today. The deterministic cross-media contract, revision checks and pilot are proposed Rust work. The [narrative backlog](TASKS.md#cross-media-narrative-backlog) lists their dependencies and acceptance criteria.

## Architecture and contributions

The workspace contains four crates. `storytelling-core` defines wire types and exact rational time. `storytelling-engine` validates, plans and exports in memory. `storytelling-cli` owns file and stdin access, stdout and exit codes. `storytelling-journal` provides a durable SQLite operation log and owner fencing; its bounded M3-02 storage slice is merged in PR #56. A test-only integration harness exercises FilmCraft against a fresh project, but the journal crate and product CLI still grant no editor execution authority. [Architecture](docs/ARCHITECTURE.md) · [Wire and CLI contracts](docs/CONTRACTS.md)

Pick a [task](TASKS.md), check [open pull requests](https://github.com/bynanci/storytelling/pulls) and connect the change to its [requirements](docs/REQUIREMENTS.md). The [contribution guide](CONTRIBUTING.md) lists the local checks, and the [SDLC](docs/SDLC.md) explains when a change is ready for implementation, review or release. Use a synthetic example when reporting a [bug](https://github.com/bynanci/storytelling/issues/new/choose). For sensitive reports, follow the [security policy](SECURITY.md).

## License

Original project code and documentation use the [MIT License](LICENSE). OpenFilm-informed components identified in the [per-file provenance records](PROVENANCE.md) use [Apache-2.0](LICENSES/Apache-2.0.txt) with the retained [upstream notice](NOTICE.openfilm).
