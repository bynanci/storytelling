# Development tasks

Baseline: `551ff38920e2085e1d53a7ba777c9680678e9725` (original README + MIT LICENSE).
Updated: 2026-11-07. M0, finite M1-01/M1-02, bounded M2-01/M2-02 audit/planning and M1-03
supplied-byte/host proof are accepted on main; release remains a separate decision.
[SDLC](docs/SDLC.md) owns the process and
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
| [#16 finite M1-02 OTIO](https://github.com/bynanci/storytelling/pull/16) | `fb65753df617c7df80ef2581396eead6b07a6434` | `9a5232e97435f543238b90a2006460f319f4e651` | 2026-11-07 03:09:33 |
| [#17 bounded M2-01 audit](https://github.com/bynanci/storytelling/pull/17) | `1c8c01da48af75b860b1ce49241c3fd509b16c59` | `422190e27bc604bd71485f78279373992f99e68c` | 2026-11-07 03:20:26 |
| [#18 bounded M1-03 proof](https://github.com/bynanci/storytelling/pull/18) | `2b859d29e22f6f2a39f73822086b59618bb8a402` | `e0a5ade7ee2c0414f87aa121bef1bdf259a7e75e` | 2026-11-07 03:26:40 |
| [#22 M2-02 fixed-window planner](https://github.com/bynanci/storytelling/pull/22) | `1f4ff082ed27dbc6223af2f0a0b1cfe71a06fe54` | `895257b1bb9993d70cae3f1267f5f6fa3de91905` | 2026-11-07 03:59:29 |

[Issues #3–#6](https://github.com/bynanci/storytelling/issues/6),
[#12](https://github.com/bynanci/storytelling/issues/12),
[#14](https://github.com/bynanci/storytelling/issues/14) and
[#15](https://github.com/bynanci/storytelling/issues/15) and
[#20](https://github.com/bynanci/storytelling/issues/20) are checked and closed completed.
The historical tree after PR #7 was `53fbfe24bf734634d4374d2b0eb133ebba937299`.
PR #18's accepted tree is `9b6b3d34b1289ab78ffc453ca094289a0de91f89`;
feature heads passed all 10 same-head push/PR checks before merge. Historical
base/source/head evidence remains unchanged. Draft #13 was closed unmerged as
superseded by #11/#16/#17, with useful source-drift corrections retained.

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
| M1-02 | P1 / accepted | Native finite OTIO writer, explicit picture-only/lock choices and direct CLI artifact | Accepted M1-01 | M1-O01–O06; official 0.18.1 parser oracle, exact content ticks, explicit loss; actual host proof remains M1-03 |
| M1-03 | P1 / accepted | Bounded supplied-byte verification and FilmCraft isolated 25 fps CPU proof | Accepted M1-02; PR #18 merged | M1-H01–H04; exact native ranges, 45-frame 192×108 preview, save/reopen/reimport, unchanged inputs; broader acceptance open |
| M2-01 | P1 / accepted | Bounded StoryBrief hard constraints vs preferences and typed conflicts | M1 profile; PR #17 merged | M2-B01–B06; manual choices preserved, declared reference/membership/order contradictions explained; full planner/feasibility separate |
| M2-02 | P1 / accepted | Finite fixed-window selection and shot planner | Accepted M2-01; PR #22 merged | M2-P01–P06; exact source windows, hard choices/order, deterministic declared ranks/reason paths; full solver remains open |
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

## M1-02 finite OTIO work

[Tracking #12](https://github.com/bynanci/storytelling/issues/12):
P1 source/profile/oracle → P2 pure serializer/loss report → P3 usable CLI and CI.
See [contract](docs/migration/FINITE_OTIO_V1.md),
[source/evidence record](docs/migration/records/M1-02.md) and
[machine index](docs/migration/m1-02-work-index.json).

- [x] Independent G2 review and actual pinned TS/official parser exploration.
- [x] Actual 25 engine RED → GREEN and 9 new CLI failures → 10 CLI GREEN.
- [x] Common integer clock, exact native bounds, repeated identity and original snapshot text.
- [x] Explicit picture-only/metadata-only-lock choices; no approval or host verification claim.
- [x] Official parse/write/read and source conformance: eight cases passed locally.
- [x] Same-head hosted gates passed; PR #16 merged and issue #12 checked/closed.
- [x] Accepted-status reconciliation records the finite format scope; host proof remains M1-03.

M1-03 has now supplied actual isolated FilmCraft v0.18.0 Linux 25 fps proof;
see its record below. The generic exporter retains its format-only report.
Audio/full-editor parity and human editorial/real-user/release gates remain separate.

## M2-01 bounded brief audit

See the [frozen profile](docs/migration/STORY_BRIEF_V1.md),
[source/comparison record](docs/migration/records/M2-01.md) and
[work index](docs/migration/m2-01-work-index.json).

- [x] Versioned strict StoryBrief schema and independently reviewed G2 profile.
- [x] Pure hard-reference/membership/order audit, manual choices preserved, typed conflicts and soft preference warnings.
- [x] Actual engine/CLI RED→GREEN, ten pinned source policy cases and Rust comparison.
- [x] Fresh-main integration, independent implementation review and same-head hosted gates.
- [x] Verified PR #17 merged and tracking issue #14 fully checked/closed under user authorization.

This audit produces no selected shots, timing feasibility or executable plan.
M2-02 now supplies accepted fixed-window planning; M2-03 planner shadow remains separate proposed work.

## M1-03 bounded host proof work

[Tracking #15](https://github.com/bynanci/storytelling/issues/15),
[contract](docs/migration/FILMCRAFT_25_V1.md),
[record](docs/migration/records/M1-03.md),
[work index](docs/migration/m1-03-work-index.json).

- [x] Independent G2 review of supplied-buffer trust boundary and exact SHA2 dependency.
- [x] Native functional RED → GREEN: real SHA-256, immutable private owned buffers, explicit bounded CLI and metadata-only receipts.
- [x] Actual FilmCraft v0.18.0 25 fps picture-only import and exact native range/identity readback.
- [x] CPU H.264 192×108 preview: 45 frames, exact 1.8 s and PTS; decoded 20 red / 5 blue / 20 green.
- [x] Native save/reopen, OTIO reimport, external source/native ID mapping and unchanged input/source hashes.
- [x] Final clean candidate receipt, independent review, all 10 same-head hosted gates; PR #18 merged and #15 fully checked/closed.

Only this synthetic Linux headless CPU profile is proved. Audio, GUI/GPU,
full-resolution export, real-user media, filesystem race closure and human
editorial acceptance remain open; no production host executor or approval API
is introduced.

## M2-02 fixed-window selection and planning

[Tracking #20](https://github.com/bynanci/storytelling/issues/20),
[profile](docs/migration/FIXED_WINDOW_PLANNER_V1.md),
[comparison](docs/migration/records/M2-02.md),
[work index](docs/migration/m2-02-work-index.json).

- [x] Frozen independent G2/clock review and actual 12-case source oracle.
- [x] Engine 26 functional RED → GREEN; hard/manual choices retained and exact fixed windows planned.
- [x] CLI 8 failures/1 existing pass → 9 GREEN; emitted nested plan accepted by actual M0 validator.
- [x] Strict bounds, deterministic reasons/skips and immutable source declarations; no new dependency/crate.
- [x] Full local quality/source checks and independent code/docs reviews passed.
- [x] All10same-head hosted checks passed; PR #22 merged as `895257b1bb9993d70cae3f1267f5f6fa3de91905`.
- [x] #20 fully checked and closed completed; work index, plans and public accepted status reconciled.

This finite planner proposes declared fixed windows; optional ranks are supplied
scores/preferences, not verified semantic evidence. Multi-beat solver and M2-03
full shadow parity, production execution and editorial acceptance remain separate.

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
