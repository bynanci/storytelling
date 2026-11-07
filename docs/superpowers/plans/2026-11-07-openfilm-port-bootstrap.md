# OpenFilm finite snapshot import: implementation plan

> For agentic workers: use task-by-task execution and self/independent review as available; never invent a subagent review. These checkboxes are future implementation steps.

**Goal:** Add one read-only, native Rust mapping from a frozen OpenFilm snapshot to a bounded Storytelling handoff document, with explicit losses and no media/host mutation.

**Architecture:** Preserve the M0 strict validator and pure core/engine boundary. A pure import function consumes a versioned DTO supplied by the CLI; a later filesystem adapter verifies bytes. No new planner, database or renderer is needed for this slice.

**Tech stack:** Existing pinned Rust 1.85.0 / edition 2024 / resolver 3. Use existing serde/serde_json boundary; any new dependency needs a task-specific review.

**Spec:** [Migration](../../MIGRATION.md), [port map](../../migration/OPENFILM_RUST_PORT_MAP.md), [SDLC](../../SDLC.md).

## Global constraints

Source baseline is OpenFilm `7add4762fe8174d76f43b863375ea39b73121e76`; target foundation candidate is `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`. Refresh actual branch metadata first. M0 PR #1 is not assumed merged. Preserve source IDs, original clocks, user choices and existing MIT license. Do not copy Apache implementation/tests without GAP-01 provenance decision. This importer validates declarations; it never claims actual source SHA verification or host compatibility.

## Review focus

- Repeated source occurrences must not collapse into one shot.
- Decimal 29.97 must not silently turn into 30000/1001.
- Unsupported audio/effects must not disappear behind a success response.
- Unknown future schemas and oversized nested metadata must fail before expensive traversal.
- Reading input must not migrate/write the original project or dereference a supplied path/URL.

## Task P1: freeze a finite import profile (M1-01 / OF-01–04 / GAP-01–03/16)

**Read:** actual `packages/core/src/models.ts`, `validation.ts`, `migration.ts`, `packages/exporters/src/index.ts` and matching tests on pinned OpenFilm; M0 `wire.rs`, `time.rs`, engine `lib.rs`, `tests/validation.rs`.

**Create:** `docs/migration/records/M1-01.md`, `tests/fixtures/openfilm/finite-v1/*.json` only when legally admitted synthetic inputs are prepared.

- [ ] Record actual upstream schema and whether the supported input is project export, composition export or a deliberately defined intermediate snapshot. Do not guess the live project/Catalog shape.
- [ ] Specify exact allowed fields and bounds, immutable input/output meaning, decimal conversion policy, explicit unsupported result, schema version and error codes; reuse existing M0 limits unless an approved requirement changes them.
- [ ] Use minimal fixtures for one still, one video trim, repeat-source occurrences, future schema, fractional rate, unknown effect and oversize input. Record expected results and source/license provenance.
- [ ] Review/freeze the profile before implementation. If unresolved source rights or wire shape remains, stop the import implementation and mark that dependency blocked.

## Task P2: implement the pure mapper

**Proposed files:** `crates/storytelling-core/src/compatibility.rs`, `crates/storytelling-engine/src/openfilm_snapshot.rs`, `crates/storytelling-engine/tests/openfilm_snapshot.rs`.

**Interface proposal to freeze in P1:** `import_openfilm_snapshot(bytes: &[u8], profile: &ImportProfileV1) -> Result<ImportedPlanV1, ImportError>`. The result contains a validated finite plan, preserved source-clock facts, and a structured loss report. No source bytes are opened. If the function instead returns a rejected profile, it must not return an apparently accepted executable plan.

- [ ] Write RED tests: repeated occurrences retain unique IDs; a trim is within declared bounds; input bytes unchanged; unknown schema rejected; unsupported effect surfaced; over-budget JSON rejected; rates remain distinct.
- [ ] Run `cargo test --locked -p storytelling-engine --test openfilm_snapshot`; record actual failure, not a missing toolchain.
- [ ] Implement bounded DTO validation and pure mapping; no filesystem/network/host/database dependencies. Wire new modules through actual crate exports.
- [ ] Run the selected tests and full workspace tests in debug/release. Compare to the frozen source fixture semantics, explicitly triage deviations.

## Task P3: expose one usable CLI slice and verify

**Modify:** actual CLI `src/main.rs`, CLI integration tests, contracts/requirements, TASKS and the comparison record. Exact command name and exit codes are frozen in P1; preserve existing `validate` behavior.

- [ ] Write RED for the new importer via file/stdin, JSON-only stdout, stable error/exit, no input changes and no attempt to open external media paths.
- [ ] Implement only the composition-root I/O; delegate all mapping and validation to the pure engine.
- [ ] Run format/clippy/tests/release/doc/repository guards and the CLI in an environment without Node/Python/FFmpeg.
- [ ] Commit one coherent draft PR with actual source/target SHA, requirements-to-tests, evidence, unsupported cases and rollback. Do not claim OTIO/FilmCraft support yet.

## Following slices, not part of M1-01

M1-02 adds OF-35/36: native OTIO serialization and explicit loss profile, official parser oracle and content clocks. M1-03 adds OF-29/32/33/42 as needed for authorized bytes, isolated host roundtrip and actual output verification. Full workflow/journal approval comes in M3; the M1 host proof is a bounded explicitly invoked test, not a general write-capable Agent API.
