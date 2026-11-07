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

## OpenFilm to Rust transfer ledger — 2026-11-07

The [48-capability map](docs/migration/OPENFILM_RUST_PORT_MAP.md),
[16 gap tasks](docs/migration/NATIVE_BOUNDARIES_AND_GAPS.md),
[machine ledger](docs/migration/openfilm-port-index.json), and
[Codex runbook](docs/migration/CODEX_PORTING_RUNBOOK.md) refine the existing
milestones; they do not replace them or claim any port completed.

Source is OpenFilm `7add4762fe8174d76f43b863375ea39b73121e76`. Target M0 is PR #1
head `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`, still unmerged at inspection.
OF-01/02 have only partial M0 declaration-validation foundations. Other native
ports are proposed. Six retain-host entries are exclusions, not rewrite tasks.

| Wave / parent tasks | Transfer scope | Required outcome |
| --- | --- | --- |
| W0 / G1-G2 | GAP-01/02/03/16 | Provenance, actual source/test comparison, frozen wire/time profile and drift checks |
| W1 / M1-01 | OF-01/02/03/04; OF-07 only for admitted timestamp fields | Read-only snapshot mapping; no source/Catalog writes; see [implementation plan](docs/superpowers/plans/2026-11-07-openfilm-port-bootstrap.md) |
| W2 / M1-02 | OF-35/36; OF-18/24 only for explicit profile features | Finite OTIO and loss report; parser oracle; no silent effect loss |
| W3 / M1-03 | OF-29/32/33/34/42 as needed; GAP-04/05/09/10 | Verified authorized sources and isolated actual host import/render/readback/save/reopen |
| W4 / M2 | OF-05–23 selected by the story use case | Native evidence-to-plan algorithms, hard constraints and differential parity; optional travel/rhythm/transcript are not universal blockers |
| W5 / M3 | OF-25–31; GAP-06/07/08/11 | Approved execution, own journal, conservative recovery and explicit unknown/partial state |
| W6 / M4-M5 | OF-37–41 as needed; GAP-12–15 | Provider/consumer adapters, native distribution and real-user/second-host evidence |

- [x] PORT-PLAN-01: written source-to-Rust inventory, classifications, proposed targets/tests and gap map (documentation only).
- [ ] PORT-G01: maintainer accepts the transfer boundary and provenance policy before source-derived implementation.
- [ ] PORT-G02: freeze actual per-task source symbols/tests/blob identities and require differential evidence.
- [ ] PORT-G03: execute M1-01 after its M0/profile prerequisites; do not bulk-create all planned modules.

Every transfer PR records OF/GAP IDs together with original M task and new requirement IDs.
The full task index is a coverage inventory, not a demand to build 64 items before M1.
