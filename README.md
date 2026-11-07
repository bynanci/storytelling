# storytelling

A Rust foundation for local-first story planning and reviewed editor workflows.

**Status: M0 implementation candidate, not a released editor integration.**
The executable slice validates a bounded story-plan JSON document. It does not
read media, verify file hashes, generate a story, render a film or drive FilmCraft.
Future functionality is tracked separately in [TASKS](TASKS.md) and [ROADMAP](ROADMAP.md).

[繁體中文](README.zh-TW.md) · [Architecture](docs/ARCHITECTURE.md) · [SDLC](docs/SDLC.md) · [Contract](docs/CONTRACTS.md)

## Quick start

Install Rust through the official rustup distribution, then:

```sh
cargo test --workspace --locked
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
```

The first validation exits 0 with `ok: true`. The invalid fixture exits 1 with
`plan.source_out_of_bounds`. For stdin use `storytelling validate -`.
`--help` and `--version` do not open a project. JSON goes to stdout; error codes
are documented in [CONTRACTS](docs/CONTRACTS.md). A successful result explicitly
has `scope: "declared-metadata-only"`; it is not media or editorial approval.

## Current architecture

```text
storytelling-cli      local input/output and process exit status
        |
storytelling-engine   pure bounded validation; later planning/compilation
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

The next milestone is a finite, editable StoryPlan-to-FilmCraft handoff validated
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

The repository's original [MIT LICENSE](LICENSE) is preserved. This bootstrap
introduces new code and synthetic fixtures; no OpenFilm implementation or private
media is copied. Future extraction must retain per-file origin and applicable
notices; see [PROVENANCE](PROVENANCE.md). Repository naming does not reserve a
crates.io package name or establish FilmCraft affiliation.
