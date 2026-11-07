# Finite OTIO implementation plan

> For agentic workers: execute task by task with actual RED/GREEN and independent
> review. User authorized automatic continuation and merging verified completed PRs.

**Goal:** Export one finite composition as usable, exact-content-clock OTIO JSON.
**Architecture:** Existing strict import → pure OTIO projection/loss policy → CLI stdout.
**Tech stack:** pinned Rust 1.85.0; current serde/serde_json; test-only OTIO 0.18.1.
**Spec:** [design](../specs/2026-11-07-finite-otio-design.md),
[frozen contract](../../migration/FINITE_OTIO_V1.md).

## Global constraints and review focus

- No source/Catalog/media/host write or source URI dereference; core/engine pure.
- Native doubles must not silently round values/rates, common-clock sums or ranges.
- Repeated sources remain occurrences; still available duration remains synthetic.
- Picture-only and metadata-only locks need explicit choices; never approval claims.
- Original metadata number lexemes survive as snapshot text; output amplification bounded.
- Official parser schema upgrades are semantic roundtrips, not raw JSON equality.
- MIT bytes/Cargo.lock unchanged; source-derived Apache notices/provenance retained.

## P1: freeze source/profile and oracle

Create comparison `docs/migration/records/M1-02.md`, contract/design above,
`docs/migration/provenance-m1-02.json`, and a bounded test-only oracle directory.

- [ ] Record actual symbols/tests/blobs, current source/target and host blocker.
- [ ] Review exact types/clock/loss/errors/budgets; correct review findings and freeze G2.
- [ ] Run official 0.18.1 exploration and pinned TS producer on legal synthetic inputs.

## P2: native writer and loss profile

Create `crates/storytelling-core/src/compiled_plan.rs`,
`crates/storytelling-engine/src/otio.rs`,
`crates/storytelling-engine/tests/otio.rs`; wire actual exports in existing crates.

Produces `export_openfilm_otio(bytes, profile, choices) -> Result<CompiledOtioV1,
OtioCompileError>` exactly as defined by the contract. No new production dependency.

- [ ] Write regressions with a compilable stub; run `cargo test --locked -p storytelling-engine --test otio`; observe functional RED.
- [ ] Implement bounded common clock, native schema, owner metadata and explicit choices.
- [ ] Run engine tests and workspace debug/release tests; record actual GREEN.
- [ ] Compare source outputs and official parser read/write/read by semantic order/ranges.

## P3: CLI, evidence and completion

Modify CLI main and add real CLI tests; update requirements/TASKS/roadmap/guards,
work index and CI-only oracle requirements/action identity. Preserve existing commands.

- [ ] Write real process RED for export-otio file/stdin/flags/JSON/exit/immutability.
- [ ] Implement composition-root I/O only; stdout is directly readable OTIO JSON.
- [ ] Run fmt/clippy/workspace debug+release/rustdoc/repository guards/diff check and PATH-isolated smoke.
- [ ] Run independent whole-branch review and converge correctness findings.
- [ ] Publish one task PR with actual full SHAs, evidence, limitations and rollback; verify same-head hosted CI.
- [ ] Under explicit user authorization, merge verified completed PR and check/close completed tasks. Retain unavailable host/manual gates as blocked.
