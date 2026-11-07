# storytelling

A Rust foundation for local-first story planning and reviewed editor workflows.

**Status: M0 validator + M1-01 finite import candidate, not a released editor integration.**
The executable slices validate a bounded story-plan JSON document and map a finite
OpenFilm composition export into a declared visual plan. They do not
read media, verify file hashes, generate a story, render a film or drive FilmCraft.
Future functionality is tracked separately in [TASKS](TASKS.md) and [ROADMAP](ROADMAP.md).

[繁體中文](README.zh-TW.md) · [Architecture](docs/ARCHITECTURE.md) · [SDLC](docs/SDLC.md) · [Contract](docs/CONTRACTS.md)

## Quick start

Install Rust through the official rustup distribution, then:

```sh
cargo test --workspace --locked
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
cargo run --locked -p storytelling-cli -- import-openfilm tests/fixtures/openfilm/finite-v1/still.json
```

The first validation exits 0 with `ok: true`. The invalid fixture exits 1 with
`plan.source_out_of_bounds`. For stdin use `storytelling validate -`.
`--help` and `--version` do not open a project. JSON goes to stdout; error codes
are documented in [CONTRACTS](docs/CONTRACTS.md). A successful result explicitly
has `scope: "declared-metadata-only"`; it is not media or editorial approval.

The importer also supports stdin: `storytelling import-openfilm -`. It accepts
one contiguous picture track of still/video cuts with strict field and resource
limits. It preserves raw source facts and exact numeric lexemes. Success has
`scope: "declared-visual-mapping-only"`, `executable: false`; video reports
`audio.not_mapped / blocks-host-handoff`. Explicit audio, effects and animation
reject without a plan. See the [finite-v1 contract](docs/migration/FINITE_V1.md),
[comparison/evidence](docs/migration/records/M1-01.md) and
[work tracking](docs/migration/M1-01-WORK-QUEUE.md).

## Current architecture

```text
storytelling-cli      local input/output and process exit status
        |
storytelling-engine   pure bounded validation and finite snapshot mapping
        |
storytelling-core     strict wire types and exact rational source time
```

Rust 2024, resolver 3, pinned toolchain 1.85.0 and a Cargo-generated lockfile.
First-party unsafe code is forbidden. Core/engine have no Tokio, SQLite, network,
GUI, FFmpeg or host dependency. Future workflow/adapters are design boundaries,
not empty crates or implemented integrations. All packages have `publish = false`.

## Product direction

OpenFilm remains a consumer and media-analysis source. FilmCraft is the first
planned external editor. Storytelling will own story intent, evidence references,
reviewed proposals and execution receipts; the host owns its edited timeline.
No second live timeline, codec stack or full nonlinear editor is planned.

Next are limited OTIO (M1-02) and an editable StoryPlan-to-FilmCraft handoff validated
by real import, playback, save/reopen and readback. A structurally valid OTIO file
alone will not close that gate. See [migration](docs/MIGRATION.md) and [risks](docs/RISKS.md).

## Develop and review

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo doc --workspace --no-deps --locked
python3 scripts/check_repo.py
```

CI covers Linux, Windows and macOS CLI behavior. Hosted CI is not a real editor,
installer, GPU or listening certification. Exact-head results belong in the PR
and [validation record](docs/validation/m0-foundation.md).

See [contributing](CONTRIBUTING.md), [agent rules](AGENTS.md), [security](SECURITY.md)
and [requirements traceability](docs/REQUIREMENTS.md). No automatic merge or release.

## License

The repository's original [MIT LICENSE](LICENSE) is preserved. The source-informed
M1-01 importer and regression component is Apache-2.0, with per-file provenance and
upstream [NOTICE](NOTICE.openfilm) / [license](LICENSES/Apache-2.0.txt). Existing
MIT integration remains separately identified; see [PROVENANCE](PROVENANCE.md).
No private media is copied. Repository naming does not reserve a
crates.io package name or establish FilmCraft affiliation.
