# storytelling

A Rust foundation for local-first story planning and reviewed editor workflows.

**Status: M0, finite M1-01/M1-02, bounded M2-01 audit and M1-03 host proof are accepted on main. No release is published.**
The executable slices validate a bounded story-plan JSON document and map a finite
OpenFilm composition export into a declared visual plan. They do not
generate a story or drive an editor. `verify-source` separately hashes explicitly
supplied bytes; the isolated test harness has proved one real FilmCraft profile.
Future functionality is tracked separately in [TASKS](TASKS.md) and [ROADMAP](ROADMAP.md).

[繁體中文](README.zh-TW.md) · [Architecture](docs/ARCHITECTURE.md) · [SDLC](docs/SDLC.md) · [Contract](docs/CONTRACTS.md)

## Quick start

Install Rust through the official rustup distribution, then:

```sh
cargo test --workspace --locked
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
cargo run --locked -p storytelling-cli -- import-openfilm tests/fixtures/openfilm/finite-v1/still.json
cargo run --locked -p storytelling-cli -- export-otio tests/fixtures/openfilm/finite-v1/still.json --metadata-only-locks > finite.otio

cargo run --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/valid.json
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

`export-otio` writes actual OTIO Timeline JSON to stdout. Videos require
`--picture-only`; true source locks require `--metadata-only-locks`. These choices
permit limited serialization, not editor execution approval. Exact common content
ticks remain distinct from project FPS; unsafe native values reject. Original
snapshot text and structured limitations travel in owner metadata. See the
[finite OTIO contract](docs/migration/FINITE_OTIO_V1.md). Official parser support
does not certify an editor or actual media.

`audit-brief FILE|-` checks a versioned StoryBrief's declared hard references,
required/excluded membership and partial ordering. It preserves manual choices
and returns typed conflicts; preferences remain soft and unchanged. A clean
report has `scope: "declared-constraint-audit-only"`, `selection_planned: false`
and `duration_analysis: "not-performed"`. It does not produce a shot selection
or establish achievable timing. See the [brief contract](docs/migration/STORY_BRIEF_V1.md)
and [M2-01 evidence](docs/migration/records/M2-01.md).

`verify-source FILE|- SHA256` verifies at most 16 MiB of explicitly supplied bytes
against a lowercase SHA-256 and returns metadata only. Its immutable owned buffer
cannot be reconstructed from a serialized receipt. This does not verify later
filesystem reads. The [M1-03 proof](docs/migration/records/M1-03.md) exercised
FilmCraft v0.18.0 Linux at 25 fps: exact native 1920×1080 ranges, save/reopen and OTIO
reimport, plus a 45-frame 192×108 CPU H.264 review export. Audio, GUI/GPU,
full-resolution export and human editorial acceptance remain open.

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

Finite OTIO and the bounded synthetic FilmCraft proof are accepted. Next is
M2-02 deterministic selection/shot planning; reviewed production execution and
broader host/media acceptance remain later gates. See [migration](docs/MIGRATION.md)
and [risks](docs/RISKS.md).

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
and [requirements traceability](docs/REQUIREMENTS.md). Completed, verified PRs may
be merged under the user's explicit task authorization; releases and repository
policy changes remain separate decisions. See [accepted lifecycle](TASKS.md#accepted-lifecycle).

## License

The repository's original [MIT LICENSE](LICENSE) is preserved. The source-informed
M1-01 importer and regression component is Apache-2.0, with per-file provenance and
upstream [NOTICE](NOTICE.openfilm) / [license](LICENSES/Apache-2.0.txt). Existing
MIT integration remains separately identified; see [PROVENANCE](PROVENANCE.md).
No private media is copied. Repository naming does not reserve a
crates.io package name or establish FilmCraft affiliation.
