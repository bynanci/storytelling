# Development tasks

Baseline: `551ff38920e2085e1d53a7ba777c9680678e9725` (original README + MIT LICENSE).
Updated: 2026-11-07. M0 and finite M1-01 are accepted and merged on main; release
remains a separate decision. [SDLC](docs/SDLC.md) owns the process and
[requirements](docs/REQUIREMENTS.md) owns acceptance traceability.

## Status rules

`proposed -> ready -> in-progress -> review -> verified -> accepted`.
`blocked` requires a cause and unblocking condition. A checked box below means
implementation exists, not that a release or real-host gate has passed.
Exact-head test results belong in the PR and [validation record](docs/validation/m0-foundation.md).

## Accepted lifecycle

The user explicitly authorized merging completed, verified PRs and checking the
completed tasks. The GitHub merge metadata records:

| PR / scope | Accepted head | Merge commit on main | Merged at (UTC) |
| --- | --- | --- | --- |
| [#1 M0 foundation](https://github.com/bynanci/storytelling/pull/1) | `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b` | `22279814ff6b147c386186569d7f77f3a803cf8b` | 2026-11-07 02:17:16 |
| [#2 transfer planning](https://github.com/bynanci/storytelling/pull/2) | `95291d31c47601181070bf89a28b1a182ddf6ed8` | `9fb403be811643baf948acdb6d72232d4a6207bb` | 2026-11-07 02:17:48 |
| [#7 finite M1-01 importer](https://github.com/bynanci/storytelling/pull/7) | `c2fb478c278550b682efba5f898082e8bcd20e24` | `610ed49673d6f554e539008b34f5fc5479b9dbc1` | 2026-11-07 02:18:09 |

[Issues #3–#6](https://github.com/bynanci/storytelling/issues/6) are checked and closed.
The main tree after these merges is `53fbfe24bf734634d4374d2b0eb133ebba937299`,
identical to the verified M1-01 candidate tree. Historical base/source/head evidence
below and in comparison records remains unchanged.

This task-session authority also covers later completed, verified PRs after their
same-head gates pass. It does not authorize releases, tags, repository policy or
permission changes, nor replace real-media/host or human editorial acceptance.

## Accepted M0 delivery

| ID | State | Deliverable | Dependency | Acceptance |
| --- | --- | --- | --- | --- |
| M0-01 | accepted | Architecture, requirements, SDLC, ADRs and extraction boundary | Original repository | Scope/authority/license agreed; no invented host capability |
| M0-02 | accepted | core/engine/CLI Cargo workspace and read-only validator | M0-01 | M0-R01–R08 tests; valid JSON accepted, invalid ranges rejected |
| M0-03 | accepted | Pinned toolchain, Cargo.lock, three-OS CI, docs guard | M0-02 | Same-head quality, debug/release tests and Required gates green |
| M0-04 | accepted | README, contribution/security policy, templates and provenance | M0-01 | Commands reproducible, local links valid, MIT bytes unchanged |
| M0-G01 | accepted | Maintainer acceptance and merge of bootstrap PR | M0-01–04 | Explicit user authorization; PR #1 merged as recorded above |

- [x] Three real crates; no empty workflow/adapter placeholders.
- [x] Canonical reduced rational source time; no float rounding.
- [x] Bounded JSON, unique identities, source references and range validation.
- [x] Machine-readable CLI with distinct validation/usage/I/O exits.
- [x] Test-first RED scaffold and actual hosted failure evidence.
- [x] SDLC artifacts and requirement-to-test map committed.
- [x] Maintainer accepted the verified foundation; PR #1 is merged on main.
- [ ] Configure repository required checks/private vulnerability reporting with owner approval; a workflow file alone does not enable protection.

## Next actionable work

| ID | Priority/state | Scope | Depends on | Definition of done / PR boundary |
| --- | --- | --- | --- | --- |
| M1-01 | P1 / accepted | Finite composition-export visual mapper and read-only CLI | M0 and transfer planning accepted; PR #7 merged on main | M1-I01–I06; exact clocks, occurrence IDs, original facts and explicit limitations; no live Catalog writes |
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

## M1-01 delivery and work tracking

[Tracking #6](https://github.com/bynanci/storytelling/issues/6) owns the finite slice:
[P1 #3](https://github.com/bynanci/storytelling/issues/3) profile/provenance,
[P2 #4](https://github.com/bynanci/storytelling/issues/4) pure mapper,
[P3 #5](https://github.com/bynanci/storytelling/issues/5) CLI/verification.
See [work queue](docs/migration/M1-01-WORK-QUEUE.md),
[contract](docs/migration/FINITE_V1.md) and
[comparison/evidence](docs/migration/records/M1-01.md).

- [x] Source-informed Apache component policy explicitly approved; original MIT unchanged.
- [x] Direct composition JSON profile frozen after independent G2 review.
- [x] 33 pure importer and 8 CLI regressions; actual RED→GREEN observed.
- [x] Six pinned TS source export/oracle cases and actual Rust differential.
- [x] Maintainer accepted M1-01/P1, P2 and P3; PR #7 and its #1/#2 prerequisites are merged, and issues #3–#6 are closed.

Implemented output is declared visual metadata with `executable:false`, not an
OTIO/FilmCraft handoff. OF-03 canonical digest is deferred; original ledger entries
remain whole-capability plans rather than being marked complete from this subset.

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

At the original planning inspection, source was OpenFilm
`7add4762fe8174d76f43b863375ea39b73121e76` and target M0 was PR #1 head
`4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`, then unmerged. That historical
inventory is preserved. M0 and finite M1-01 are now accepted as recorded above;
whole OF capabilities and other native ports are not thereby complete. Six
retain-host entries remain exclusions, not rewrite tasks.

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
- [x] PORT-G01: transfer planning/boundary and the separate component provenance policy accepted; planning PR #2 and finite implementation PR #7 merged.
- [x] PORT-G02, M1-01 subset only: actual source symbols/tests/blob identities and differential evidence frozen for this accepted slice; every later task must repeat its own comparison/admission.
- [x] PORT-G03: finite M1-01 implemented, verified and accepted; remaining capability subsets, OTIO and actual host proof stay separate.

Every transfer PR records OF/GAP IDs together with original M task and new requirement IDs.
The full task index is a coverage inventory, not a demand to build 64 items before M1.
