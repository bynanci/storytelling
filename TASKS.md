# Development tasks

Baseline: `551ff38920e2085e1d53a7ba777c9680678e9725` (original README + MIT LICENSE).
Updated: 2026-11-07. This branch delivers an M0 implementation candidate; merge and
release are separate decisions. [SDLC](docs/SDLC.md) owns the process and
[requirements](docs/REQUIREMENTS.md) owns acceptance traceability.

## Status rules

`proposed -> ready -> in-progress -> review -> verified -> accepted`.
`blocked` requires a cause and unblocking condition. A checked box below means
implementation exists, not that a release or real-host gate has passed.
Exact-head test results belong in the PR and [validation record](docs/validation/m0-foundation.md).

## Current M0 delivery

| ID | State | Deliverable | Dependency | Acceptance |
| --- | --- | --- | --- | --- |
| M0-01 | review | Architecture, requirements, SDLC, ADRs and extraction boundary | Original repository | Scope/authority/license agreed; no invented host capability |
| M0-02 | review | core/engine/CLI Cargo workspace and read-only validator | M0-01 | M0-R01–R08 tests; valid JSON accepted, invalid ranges rejected |
| M0-03 | review | Pinned toolchain, Cargo.lock, three-OS CI, docs guard | M0-02 | Same-head quality, debug/release tests and Required gates green |
| M0-04 | review | README, contribution/security policy, templates and provenance | M0-01 | Commands reproducible, local links valid, MIT bytes unchanged |
| M0-G01 | pending | Maintainer review and acceptance of bootstrap PR | M0-01–04 | Read exact-head evidence, approve scope, decide merge |

- [x] Three real crates; no empty workflow/adapter placeholders.
- [x] Canonical reduced rational source time; no float rounding.
- [x] Bounded JSON, unique identities, source references and range validation.
- [x] Machine-readable CLI with distinct validation/usage/I/O exits.
- [x] Test-first RED scaffold and actual hosted failure evidence.
- [x] SDLC artifacts and requirement-to-test map committed.
- [ ] Maintainer accepts the verified PR; branch remains unmerged until approved.
- [ ] Configure repository required checks/private vulnerability reporting with owner approval; a workflow file alone does not enable protection.

## Next actionable work

| ID | Priority/state | Scope | Depends on | Definition of done / PR boundary |
| --- | --- | --- | --- | --- |
| M1-01 | P1 / ready after M0 | Freeze finite editable handoff profile; add read-only OpenFilm fixture importer | M0-G01 | Source/shot IDs, time conversion policy, lineage and expected unsupported cases; no live Catalog writes |
| M1-02 | P1 / proposed | Limited OTIO writer and loss report | M1-01 | Official parser oracle, deterministic source mapping, unknown effects fail closed |
| M1-03 | P1 / proposed | FilmCraft isolated-project roundtrip proof | M1-02 | Pinned host import/render/readback/save/reopen; original project unchanged |
| M2-01 | P1 / proposed | StoryBrief, hard constraints vs preferences and typed conflicts | M1 profile | Infeasible constraints explained; manual choices never silently dropped |
| M2-02 | P1 / proposed | Deterministic bounded selection and shot planner | M2-01 | Same frozen evidence/version produces same plan; semantic reasons cite evidence |
| M2-03 | P1 / proposed | Shadow compare selected TS planner behavior with Rust | M2-02 | Differences triaged as port bug, prior bug, unsupported or intentional change |
| M3-01 | P1 / proposed | Approval and compiled-plan identity contracts | M1-03, M2-02 | Approval binds source, plan, host version/capabilities and accepted losses |
| M3-02 | P1 / proposed | SQLite operation journal and owner-fenced recovery | M3-01 | Fault injection: crash/retry/late callback/ack failure; no duplicate apply |
| M3-03 | P1 / proposed | FilmCraft executor with unknown-result reconciliation | M3-02 | New isolated project only; timeout readback before retry; no blind Undo rollback |
| M4-01 | P2 / proposed | Thin TypeScript client and one OpenFilm feature flag | M2-03, M3-03 | One writer/authority; external sample consumes stable CLI protocol |
| M4-02 | P2 / proposed | Story Companion review-to-host user journey | M4-01 | Propose/compare/approve/execute/verify visible and accessible |
| M4-03 | P2 / proposed | Optional MCP transport over the same services | M4-01 | Agent cannot self-approve; capability/token separation; no new scheduler |
| M5-01 | P2 / proposed | Second real editor compatibility profile | M1-03, M4-02 | Actual target import/play/save/reopen, not merely a second exporter |
| M5-02 | P2 / proposed | Cross-platform CLI distribution candidate | M3-03 | Dependencies/advisories/SBOM/provenance, installation and rollback evidence |
| M5-03 | P2 / proposed | External user and SDK evaluation | M4-02 | 3–5 user trials, independent integration, measured workflow comparison |
| V1-01 | deferred | Stable public SDK and schema release policy | M5-01–03 | Migration/recovery contract, supported host matrix, reviewed release approval |

## Not in the current backlog

Full NLE parity, codecs, GPU effects engine, DAW, multicam, a marketplace, arbitrary
plugins, cloud collaboration, general bidirectional sync, universal Agent framework
and simultaneous N-API/Python/WASM bindings. Add only after an accepted requirement
shows a blocker in the reference story workflow.

## File coordination

One PR owns a public schema/time-contract change at a time. Adapter and UI work
may proceed against a frozen version. Documentation reconciliation must not mark
an open PR implemented. Security work can run independently; no task may silently
relax source, approval, license or host-compatibility gates.
