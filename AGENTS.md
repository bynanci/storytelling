# Agent execution rules

1. Read README, TASKS, ROADMAP, docs/SDLC.md and the selected task's requirements before coding.
2. Fetch current main/open PRs. Work on one isolated branch and one coherent task. Do not overwrite another branch, merge, publish, tag, change permissions or add a maintainer signature without explicit authority.
3. Core/engine stay pure: no filesystem, Tokio, database, network, model or host calls. CLI is a composition root. Future adapters implement ports; hosts own their actual timelines.
4. Implement one usable vertical slice. Do not generate empty future crates, a second NLE, codec stack, plugin marketplace or distributed worker system.
5. Write a failing test before implementation and record observed RED then GREEN. A missing toolchain is blocked verification, not RED. Do not weaken assertions to obtain green checks.
6. Strict JSON at boundaries. Treat wire DTOs as untrusted. Preserve rational/source/sample clocks, source identity, locks, user choices and review versions.
7. A successful validator only checks declarations. Never call it actual SHA verification, human approval, FilmCraft compatibility or a render-quality pass.
8. No implicit source writes, remote model calls or editor commands. Future execution requires exact-plan approval, host isolation, receipts and recovery from unknown results.
9. Preserve existing MIT LICENSE. Before importing OpenFilm code/tests, record origin/revision/license and retain applicable notices. Never assume a language rewrite removes obligations.
10. Synthetic/publicly authorized fixtures only; no personal paths, GPS, private footage, tokens or signed URLs in tests, logs or PRs.
11. Run formatting, clippy, workspace tests, release-mode tests, docs and repository checks. Read same-head hosted results. Report blocked, cancelled and failed checks separately.
12. Update task IDs, traceability, ADRs and validation evidence. State what is implemented, automatically verified and still manual. No product percentage from checkbox counts.

Current scope: M0 validator and M1-01 read-only finite visual snapshot mapping candidates.
Read [finite-v1](docs/migration/FINITE_V1.md) and its
[comparison record](docs/migration/records/M1-01.md) for the admitted input and losses.
Next: M1-02 finite OTIO; actual host proof remains M1-03.
A test-first scaffold commit exists in history intentionally; it is not a release.

## OpenFilm transfer work

Read [port map](docs/migration/OPENFILM_RUST_PORT_MAP.md),
[gap tasks](docs/migration/NATIVE_BOUNDARIES_AND_GAPS.md),
[machine ledger](docs/migration/openfilm-port-index.json) and
[runbook](docs/migration/CODEX_PORTING_RUNBOOK.md) before any transfer.
Choose one OF/GAP subtask under an existing M milestone; record actual source SHA,
files/symbols/tests, target state, license decision, preserved behavior and intentional deltas.
Planned Rust paths/regression names are not existing code or passing evidence.
Native-core, native-runtime, external-adapter and retain-host must remain distinct.
Do not start full NLE rewrites or treat upstream Draft PRs as merged baselines.
