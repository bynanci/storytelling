# Storytelling

[繁體中文](README.zh-TW.md) · [CLI guide](docs/CLI_GUIDE.md) · [Roadmap](ROADMAP.md) · [Contributing](CONTRIBUTING.md) · [CI runs](https://github.com/bynanci/storytelling/actions/workflows/ci.yml)

Storytelling is a local-first Rust toolkit for planning a story from material you have already selected. Its first working use case is a video rough cut. Give the planner candidate clips, exact source ranges and editorial constraints. It proposes a shot order, explains which clips it skipped and can turn the selected cuts into an OpenTimelineIO (OTIO) timeline for review.

**Development version `0.1.0-dev.0` is available from source.** The first release is still ahead. The [cross-media roadmap](ROADMAP.md#cross-media-narrative-roadmap) proposes ways to use the same material for scripts, illustrations, copy, brainstorming and everyday storytelling.

## Quick start

Install Rust with [rustup](https://rustup.rs/). Clone the repository and run the bundled synthetic examples. The repository selects Rust 1.85.0.

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

OpenFilm supplies the composition snapshot. The target editor owns the working timeline. An isolated test exercised FilmCraft v0.18.0 on Linux at 25 fps, including save/reopen, OTIO reimport and a 45-frame CPU preview. Production editor execution is planned. Audio handoff and full-resolution output are outside the tested profile. [FilmCraft test record](docs/migration/records/M1-03.md) · [Review contract](docs/migration/PLANNER_OTIO_REVIEW_V1.md)

## Cross-media direction

The proposed first use case starts with one account of an event and produces a spoken outline and alternative compositions for a single illustration. An invented visual detail can stay with the illustration while the spoken account keeps its link to the source. A second use case tests commercial copy by tracing each claim to supplied evidence. Both are planning work. The [narrative backlog](TASKS.md#cross-media-narrative-backlog) lists their dependencies and acceptance criteria.

## Architecture and contributions

The workspace contains three crates. `storytelling-core` defines wire types and exact rational time. `storytelling-engine` validates, plans and exports in memory. `storytelling-cli` owns file and stdin access, stdout and exit codes. [Architecture](docs/ARCHITECTURE.md) · [Wire and CLI contracts](docs/CONTRACTS.md)

Pick a [task](TASKS.md), check [open pull requests](https://github.com/bynanci/storytelling/pulls) and connect the change to its [requirements](docs/REQUIREMENTS.md). The [contribution guide](CONTRIBUTING.md) lists the local checks, and the [SDLC](docs/SDLC.md) explains when a change is ready for implementation, review or release. Use a synthetic example when reporting a [bug](https://github.com/bynanci/storytelling/issues/new/choose). For sensitive reports, follow the [security policy](SECURITY.md).

## License

Original project code and documentation use the [MIT License](LICENSE). OpenFilm-informed components identified in the [per-file provenance records](PROVENANCE.md) use [Apache-2.0](LICENSES/Apache-2.0.txt) with the retained [upstream notice](NOTICE.openfilm).
