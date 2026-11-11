# Development tasks

Baseline: `551ff38920e2085e1d53a7ba777c9680678e9725` (original README + MIT LICENSE).
Updated: 2026-11-11. M0, finite M1-01/M1-02, bounded M2-01/M2-02 audit/planning, M1-03
supplied-byte/host proof, selected-domain M2-03 live shadow, the bounded M3-02
SQLite journal and the M3-03 test-only lost-reply proof are merged on main;
production FilmCraft execution and release remain separate decisions.
[SDLC](docs/SDLC.md) owns the process and
[requirements](docs/REQUIREMENTS.md) owns acceptance traceability.

## Status rules

`proposed -> ready -> in-progress -> review -> verified -> accepted`.
`blocked` requires a cause and unblocking condition. A checked box below means
implementation exists, not that a release or real-host gate has passed.
Exact-head test results belong in the PR and [validation record](docs/validation/m0-foundation.md).

## Cross-media narrative backlog

The [cross-media roadmap](ROADMAP.md#cross-media-narrative-roadmap) proposes
using one collection of sources and ideas for different forms of expression.
These N tasks are planned work. Each maps to the identically numbered N-R
requirement in the [traceability table](docs/REQUIREMENTS.md#proposed-cross-media-narrative-requirements). The [video delivery
queue](#next-actionable-work) continues independently.

| Task / requirement | Status and SDLC gate | Depends on | Deliverable and acceptance example |
| --- | --- | --- | --- |
| [N-01](https://github.com/bynanci/storytelling/issues/48) / N-R01 · Authoring examples | Proposed · G0–G1 | Can start from current main | Produce a synthetic source note, one spoken outline, two sketches described in words and a plain-notes comparison. Annotate supplied details, interpretations and invented details. Include a second example of a commercial claim with missing evidence. Record a measurable revision task and pilot decision criteria. |
| [N-02](https://github.com/bynanci/storytelling/issues/49) / N-R02 · Additive contract | Proposed · G2 | N-01 | Review an ADR and versioned contract for source units, relations, constraints and named variants. Show strict parsing/size limits, version refusal and a compatible path alongside video V1. A free association cycle is valid. A cycle of mandatory precedence edges is a conflict. A dangling unit reference has a diagnostic. |
| [N-03](https://github.com/bynanci/storytelling/issues/50) / N-R03 · Oral and visual planning | Proposed · G3–G4 | N-02 | Deliver one local CLI workflow with structured input and human-readable outlines. One source yields ordered speaking points and two illustration briefs with focal subject, foreground and background. Re-running the same input and options reproduces the result. A fixture with no media path, duration or cut points must succeed. |
| [N-04](https://github.com/bynanci/storytelling/issues/51) / N-R04 · Source and variant revisions | Proposed · G3–G4 | N-03 | Preserve source and revision IDs and author selections. Accept an invented visual detail in only one variant, revise a source, then report which variants cite the superseded revision. Show an unrelated variant stays current and prior revisions remain inspectable. A supplied model suggestion cannot confirm itself as fact. |
| [N-05](https://github.com/bynanci/storytelling/issues/52) / N-R05 · Evidence in commercial copy | Proposed · G3–G4 | N-04 | From a synthetic product note, build a copy outline pairing audience, claim, evidence and next action. A cited claim points back to its source. Removing the citation flags that claim for substantiation. The output labels a proposed slogan separately. Record an author's assessment of the claim and evidence. |
| [N-06](https://github.com/bynanci/storytelling/issues/53) / N-R06 · Evaluate additional profiles | Proposed · G0–G2 | N-04 | Compare three small examples: a brainstorm with two alternative branches, a script whose reveal order differs from event order, and an optional mapping from selected units to real video references. Record author feedback and recommend at most one follow-up slice. Missing video references must remain unresolved. |
| [N-07](https://github.com/bynanci/storytelling/issues/54) / N-R07 · Pilot and release decision | Proposed · G4–G6 | N-05 | Run the N-01 comparison on the oral/visual and copy workflows. Report observed times, attribution errors and completion counts with denominators. Record a continue, narrow or stop decision and convert reproducible failures into synthetic regression cases. Document compatibility and rollback for any proposed release. |

**Author-facing entry point.** Start each cross-media acceptance case with a
natural-language request to the [Storytelling skill](.agents/skills/storytelling/SKILL.md).
Keep the request, the output shown to the author, and any separately observed
engine checks. Discovery and draft examples can run with the skill alone. For
N-03 onward, the CLI provides repeatable checks where a contract has shipped.
The pilot measures what an author can complete from the skill. Use the
[seven Codex handoffs](docs/CODEX_NARRATIVE_PROMPTS.zh-TW.md) when picking up a task.

### Starting N-01

1. Start with [issue #48](https://github.com/bynanci/storytelling/issues/48). Record N-01 /
   N-R01, the current main SHA and any overlapping PR. The outcome is an author
   revising one source in two presentations and identifying what changed.
2. Write a fictional observation, for example two neighbors fixing a gate. Give
   every detail a stable label. Introduce an imagined red umbrella in one
   illustration only. Ask a reader to reconstruct the original observations
   from the spoken outline and explain where the umbrella came from.
3. Record the same revision using plain notes, including time, errors and any
   need to re-enter the source. Include the claim scenario, planned measures
   and criteria for continuing in the pilot. Publish synthetic examples and
   anonymized findings. Record outstanding research questions and evidence gaps.
4. Submit the examples and results in one documentation PR. The maintainer
   records whether there is a shared authoring problem worth taking to G2.
   If the examples work as separate templates and revision tracking offers no
   observed benefit, narrow or stop the shared-contract work.

The maintainer owns scope and the pilot decision. An implementer owns each
contract or runnable slice, a reviewer checks its evidence, and authors assess
the usefulness of the outputs. At G3–G4 use the existing core/engine/CLI
boundary, synthetic fixtures, RED/GREEN evidence, same-head CI and existing
video V1 regressions. Keep any optional model suggestions outside the pure
engine. At G6 feed author findings back into a named task and requirement.
Record blocked work with its owner and the observation needed to resume.

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
| [#26 M2-03 selected-domain live shadow](https://github.com/bynanci/storytelling/pull/26) | `4bf888949107a21b560b09222175ff1f25118a3b` | `5533fcb5d0f857ad1066a76505071653b66d1f89` | 2026-11-07 06:27:14 |
| [#56 M3-02 bounded SQLite operation journal](https://github.com/bynanci/storytelling/pull/56) | `3cd078e56ed4f2d3dd7accf655446851322cd94d` | `73196930e13d8ce53ba792c574ca4de156883a73` | 2026-11-11 02:07:53 |
| [#59 M3-03 isolated FilmCraft lost-reply proof](https://github.com/bynanci/storytelling/pull/59) | `40748487342b89332088a18a158dbd5f93597abe` | `f653952d05b8259d0a4f5e0be850748e3b2dc180` | 2026-11-11 03:07:21 |


Routine dependency upkeep merged on 2026-11-07 (these rows do not claim product
acceptance):

| PR / change | PR head | Merge commit on main | Merged at (UTC) |
| --- | --- | --- | --- |
| [#8 actions/checkout 7.0.1](https://github.com/bynanci/storytelling/pull/8) | `f3bf985acbd6abb67b3ce5f781535387dc602373` | `3905135b3fd811e7639dffd2a98116aa6543b53f` | 2026-11-07 13:19:26 |
| [#21 actions/setup-python 7.1.0](https://github.com/bynanci/storytelling/pull/21) | `ad4e33f0406a78336279fe7916d3372660169d02` | `23a8c60051c8ab81310bb4e67e7c8b2fbcf44810` | 2026-11-07 13:20:47 |
| [#10 serde 1.0.229](https://github.com/bynanci/storytelling/pull/10) | `6a2456a2008d937b1658bbb77866bee4f2b694b0` | `a4c07d5cf29a4845757ada0cb6eea664cfeb98a0` | 2026-11-07 13:22:15 |
| [#9 serde_json 1.0.154](https://github.com/bynanci/storytelling/pull/9) | `14992f18dc36804b8fa752ea2383194a9993bd38` | `fefb038ec1699727da2c0f436a097bbdedcfd08a` | 2026-11-07 13:25:36 |

All four dependency PR heads and resulting main commits passed their checks.
PR #28 was synchronized to the updated action pins and dependency lockfile, then
merged after two exact-head runs passed all six jobs. Its source candidate remains
unaccepted pending independent G2 review.

## Merged candidate pending G2 acceptance

[PR #28](https://github.com/bynanci/storytelling/pull/28) merged head `2d23b21dcfbfc49c3d76087f054cc76724c23053` to
main as `5a5702ba4c1392d763929f89da3cf7d16d5f13f4` at 2026-11-07 13:47:15 UTC. The two exact-head runs
[42378356866](https://github.com/bynanci/storytelling/actions/runs/42378356866)
and [42378352463](https://github.com/bynanci/storytelling/actions/runs/42378352463)
passed Quality, Required, Linux, Windows, macOS and Dependency Audit. The main
merge run [42378550750](https://github.com/bynanci/storytelling/actions/runs/42378550750)
also passed all six gates. The 40-package lockfile audit loaded 1,422 advisories
and reported no findings.

- [x] Bounded M3-A01/A02 implementation and tests are integrated on main.
- [x] Current dependency pins, provenance/status documents and hosted audit are integrated.
- [x] PR #28 merged under the user's instruction after exact-head gates passed.
- [ ] Independent G2 security review and explicit candidate acceptance remain pending for PR #28.
- [x] Read-only technical review packet added at [PR #28 security review](docs/validation/pr28-technical-security-review-2026-11-11.md); it is AI analysis and does not satisfy human G2.
- [x] The separate bounded M3-01/P1 finite-OTIO profile is signed and accepted; see [Issue #27](https://github.com/bynanci/storytelling/issues/27) and its acceptance record.

This merge does not establish trusted signer authorization, same-snapshot
compiler provenance, replay prevention, host execution, product acceptance or release.

[Issues #3–#6](https://github.com/bynanci/storytelling/issues/6),
[#12](https://github.com/bynanci/storytelling/issues/12),
[#14](https://github.com/bynanci/storytelling/issues/14) and
[#15](https://github.com/bynanci/storytelling/issues/15) and
[#20](https://github.com/bynanci/storytelling/issues/20) and
[#24](https://github.com/bynanci/storytelling/issues/24) are checked and closed completed.
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
| M2-03 | P1 / accepted | Actual selected-domain TS/native live shadow and completed PR register | Accepted M2-02; tracking #24 | M2-S01–S06; exact 64-pair evidence, fail-closed regressions, source/tool provenance and same-head acceptance |
| M3-01 | P1 / bounded subprofile accepted; broader milestone proposed | Approval and compiled-plan identity contracts | M1-03, M2-02 | Accepted subprofile binds same-input finite OTIO and declared context; planner bridge is delivered in P2; trusted approval and execution remain open |
| M3-01/P2 | P1 / accepted bounded technical delivery | Selected fixed-window proposal → actual read-only OTIO review bundle | Accepted M2-02 and M3-01/P1 | M3-PB01–PB07; reproducible chosen cuts/loss report; changed asset/cut/loss invalidates review; non-bundle paths skip bundle serialization; no Catalog/host writes |
| M3-02 | P1 / accepted bounded journal slice | SQLite operation journal and owner-fenced recovery | Accepted M3-01/P2; journal does not itself grant execution authority | PR #56 merged; exact-head Quality, Required, Linux, Windows, macOS and Dependency Audit all pass; post-merge main run passes; host execution remains M3-03 |
| M3-03 | P1 / in progress; bounded test-only proof merged, production executor open | FilmCraft executor with unknown-result reconciliation | Accepted M3-02 | PR #59 exact-head and post-merge main CI pass; pinned Linux proof journals a new isolated project and cut import, resolves a lost reply from exact native readback without retry/Undo; product executor, in-flight host timeout, safe NotApplied retry and broader host matrix remain open |
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
M2-02 supplies accepted fixed-window planning; M2-03 selected-domain live shadow is accepted via PR #26.

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
scores/preferences, not verified semantic evidence. Multi-beat solver, full shadow parity, production execution and editorial acceptance remain separate.

## M2-03 selected-domain live planner shadow

[Tracking #24](https://github.com/bynanci/storytelling/issues/24),
[profile](docs/migration/PLANNER_SHADOW_V1.md),
[comparison record](docs/migration/records/M2-03.md),
[work index](docs/migration/m2-03-work-index.json) and
[completed PR register](docs/delivery/PR_REGISTER.md).

- [x] Independent domain/schema/error/budget review and actual source exploration.
- [x] Functional RED→GREEN; 40 Python regressions including actual producer and failed-run evidence.
- [x] Two actual 64-pair runs: 56 agreements, four literal intentional differences, four unsupported probes, zero unclassified.
- [x] Pinned test-tool closure and Linux CI integration; original three-OS Rust and 36 frozen comparisons retained.
- [x] PR #25 merged the completed/superseded/pending PR register and preserved original PR bodies.
- [x] Final clean candidate and independent raw audit; all10same-head checks passed, PR #26 merged and #24 checked/closed completed.

This is synthetic selected-domain differential evidence. Fractional optional
budgets, full solver behavior, live Catalog authority, host execution and human
editorial acceptance remain separate. The runner opens no user project.

## M2-02/O1 stable ordering optimisation — accepted

2026-11-08 follow-up under the user's optimise/upgrade instruction; tracked in
[issue #33](https://github.com/bynanci/storytelling/issues/33). The accepted M2-02
wire, priority policy, timing and input limits remain the contract.

- [x] Internal min-priority ready queue replaces repeated candidate scans.
- [x] Exhaustive four-vertex graph comparison, supported-size graph tests and public report regression added.
- [x] Manual release benchmark recorded, including small-graph overhead and limited end-to-end gains.
- [x] Requirements, source refresh, component provenance and validation record added.
- [x] PR #34 merged as `f07ab6940484ef8b8e4f2bdb37b96893e9f92270`; both head runs (12 jobs) and main run (6 jobs) passed. Issue #33 is fully checked and closed completed.

The [validation record](docs/migration/records/M2-ORDER-OPT.md) is a versioned
implementation snapshot. Final hosted checks, review and merge/checklist status
are maintained in issue #33 and its associated PR. This optimisation does not
complete the broader OpenFilm map or M3-01. PRs #31/#32 were subsequently merged;
the bounded M3-01/P1 profile now has its own signed acceptance. PR #28 remains a
separate unaccepted candidate.

## M2-02/O2 cached exact-time construction — accepted

Follow-up [issue #35](https://github.com/bynanci/storytelling/issues/35) preserves
the accepted planner contract while caching denominators/scaled ticks and
formatting diagnostic paths only on failures.

- [x] Observed resource RED then GREEN for error-only path formatting.
- [x] Golden fractions, error-priority, 2,048 generated compatibility cases and 1,024-candidate coverage added.
- [x] Informational constructor/full-plan release measurements recorded.
- [x] Local debug/release/docs and frozen/live conformance verification passed.
- [x] PR #36 merged as `00ef8decbf1fecc7258bbc683973d41d656c8097`; both head CI runs (12 jobs) and main run (6 jobs) passed. Issue #35 is fully checked and closed completed.

The [validation record](docs/migration/records/M2-CLOCK-OPT.md),
[work index](docs/migration/m2-clock-opt-work-index.json) and provenance capture
the local and accepted delivery evidence. Head `bdd502a8f8cd36c7b822c91fba4ff87215e8beb1`
passed runs 42446786881/42446759799; merge `00ef8de` passed main run 42446933790.
Delivery was accepted under standing user authority and explicit self-review of
the unchanged planner profile; M3 G2 and release remain separate.

## M3-01/P1 finite-OTIO review binding — accepted bounded profile

[Issue #27](https://github.com/bynanci/storytelling/issues/27) tracks the bounded
same-input review binding. The signed [G2 acceptance record](docs/migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md)
accepts M3-RB01–RB07 with explicit boundaries and waives the unprovable historical
functional RED as a blocker. The waiver does not claim a historical test passed.

- [x] Pre-implementation compile failure recorded (unresolved imports; no test body ran).
- [x] Historical functional RED remains unproven and is explicitly waived as a blocker; later mutation experiments remain separate current evidence.
- [x] Same-byte importer/compiler binding, private immutable BoundReviewV1 and strict declared context implemented.
- [x] Read-only prepare-review/check-review CLI, tests, provenance, static source map and bounded paired OpenFilm runtime oracle supplied.
- [x] Current-main Planner optimisations and their accepted completion records retained during integration.
- [x] Refreshed debug/release each passed 217 tests at integration; follow-up golden-vector and error-order tests are tracked separately.
- [x] PR #40 adds the fixed cross-platform digest vector, API/CLI error-precedence regressions and pinned source-runtime evidence. Full workspace debug/release runs each pass 219 tests; both exact-head runs and the post-merge main run pass all six jobs.
- [x] PR #31 merged as `baa97903e3d3c02e2eb2747140d62f1ddca3c60c`; PR #32 merged as `3d121398e2c14c4a43d101a5b9a55a4616a81174` under the user review-complete/merge instruction.
- [x] Issue #27 entries 1/3/4/5/6 have scoped acceptance, implementation, oracle, automation, merge and reconciliation evidence; item 2 is completed by an explicit waiver, not by a retroactive test claim.
- [x] [Evidence audit](docs/migration/records/M3-01-EVIDENCE-AUDIT-2026-11-08.md) corrects provenance and historical RED claims; [runtime record](docs/migration/records/M3-01-RUNTIME-ORACLE-2026-11-08.md) adds a bounded actual source/runtime comparison.
- [x] Independent human G2 acceptance is signed. Issue #27 closed completed after PR #42 head run 42484654708 and post-merge main run 42484845782 passed all six jobs.

Matching supplied composition/context establishes declaration and recomputed
artifact consistency only. It does not authenticate a signer, verify source
origin or a host, bridge planner output to OTIO, prevent replay or execute edits.
PR #28 remains the separate supplied-payload signature candidate. Full OF-25 /
GAP-06, trusted approval and production M3-03 execution remain broader.
The [merge review](docs/migration/records/M3-01-MERGE-REVIEW-2026-11-08.md)
records the technical review and exact evidence boundaries.

## M3-01/P2 selected proposal to read-only OTIO

The [bounded contract](docs/migration/PLANNER_OTIO_REVIEW_V1.md),
[ADR](docs/adr/0004-selected-proposal-artifact-binding.md),
[work index](docs/migration/m3-planner-review-work-index.json) and
[comparison/validation record](docs/migration/records/M3-01-P2.md) own this PR.

- [x] Actual pre-implementation CLI RED: three tests executed and failed with `cli.usage`, not a compile/toolchain failure.
- [x] Same-snapshot asset matching, exact selected cuts/order and immutable actual-artifact bundle implemented.
- [x] Read-only prepare/export/check CLI and native regression tests supplied.
- [x] Official parser roundtrip oracle supplied for the selected video/still profiles; actual results are recorded separately.
- [x] Both head runs and post-merge main run passed all six jobs; technical self-review complete, without an independent-review claim.
- [x] Bounded technical bridge integrated via PR #44 under standing verified-merge authority; M3-02 is next, with execution authority still separate.
- [x] Export and freshness compile paths avoid constructing unused portable bundles while preserving OTIO bytes and review identity; M3-PB07 passed local and same-head/post-merge verification.

Follow the dependency order P2 -> operation journal -> FilmCraft execution/readback ->
OpenFilm feature flag. The critical future fault is apply-success/response-timeout:
persist `unknown`, read back the project, and only retry when non-application is
proven. Ambiguous/missing readback must never trigger a second apply. PR #28's
independent G2 security review/explicit candidate acceptance stays separate.

## M3-02 SQLite operation journal — accepted bounded storage slice

The [v1 contract](docs/migration/M3-02-OPERATION-JOURNAL.md) and
[requirement map](docs/REQUIREMENTS.md#sqlite-operation-journal) define the
bounded storage slice. `storytelling-journal` is a native SQLite adapter; it does
not call FilmCraft, modify OpenFilm Catalog or grant execution authority.

- [x] Test-first RED observed: three executed regressions fail at the incomplete journal API.
- [x] Domain-separated operation fingerprint binds operation/request/project, review-binding digest and exact artifact digest; conflicting idempotency reuse rejects.
- [x] Write-ahead attempt marker, WAL/FULL transactions, append-only event history and owner/fence epoch persisted.
- [x] Crash/unknown recovery, no automatic retry, stale owner fencing and late response/readback rejection covered by tests.
- [x] SQLite busy lock returns a stable error without partial insertion; retry after lock release succeeds.
- [x] Local Cargo Audit found RUSTSEC-2026-0459 in two SQLite trial pins; rejected both, upgraded to the fixed `libsqlite3-sys` line and aligned workspace MSRV/toolchain at 1.89. See the [dependency review](docs/migration/DEPENDENCY_REVIEW_M3_02.md).
- [x] Bounds, state distinctions, limitations and M3-03 retry/readback requirements documented.
- [x] Exact-head PR #56 CI run [42943528675](https://github.com/bynanci/storytelling/actions/runs/42943528675): Quality, Required, Linux, Windows, macOS and Dependency Audit all passed.
- [x] PR #56 merged as `73196930e13d8ce53ba792c574ca4de156883a73` after all exact-head gates passed.
- [x] Post-merge main run [42943853913](https://github.com/bynanci/storytelling/actions/runs/42943853913) passed the same six jobs.

The accepted scope is the non-executable journal only; it does not establish
host idempotency/quiescence, actual FilmCraft readback, trusted approval or
exactly-once effects. Validation evidence is maintained in
[the M3-02 record](docs/validation/m3-02-operation-journal.md). PR #28's
independent G2 review and explicit candidate acceptance remain pending and do
not block this journal slice.

## M3-03 FilmCraft execution/readback — bounded proof only

The [profile](docs/migration/M3-03-ISOLATED-FILMCRAFT.md),
[requirement map](docs/REQUIREMENTS.md#m3-03-isolated-filmcraft-execution),
[work index](docs/migration/m3-03-filmcraft-work-index.json) and
[validation record](docs/validation/m3-03-filmcraft-execution.md) document the
first actual host slice. The task remains in progress; this harness adds no
production editor-writing command.

- [x] Fresh synthetic proposal, snapshot, context, bound OTIO and source-byte identity checked before the host call.
- [x] Create-project and import operations each persist project/review/artifact identity, executor and attempt marker before FilmCraft changes the isolated run root.
- [x] Official FilmCraft v0.18.0 Linux host imported one 25-frame proposal cut; native project readback confirmed 25/1 fps, exact 1/25..26/25 source range and online synthetic media.
- [x] Synthetic source, proposal/snapshot/context/bundle/OTIO inputs and pre-import empty project remained byte-identical.
- [x] Lost acknowledgement after successful host exit was recorded as unknown; native readback resolved applied-observed; late response rejected; one attempt, zero retry, zero Undo.
- [x] Existing ambiguous-readback, crash-recovery and stale-owner journal regressions remain in the three-OS workspace suite.
- [x] PR #59 exact-head run [42950068731](https://github.com/bynanci/storytelling/actions/runs/42950068731) and post-merge main run [42950625212](https://github.com/bynanci/storytelling/actions/runs/42950625212) passed Quality and contracts, Required gates, Linux, Windows, macOS and Dependency Audit.
- [x] PR #59 merged as `f653952d05b8259d0a4f5e0be850748e3b2dc180`; this accepts only the test-only proof, not the M3-03 production executor.
- [ ] Production executor with bounded host-process supervision and a trusted, accepted authorization gate.
- [ ] Demonstrate safe stop when FilmCraft hangs or readback is ambiguous/partial; establish host-request quiescence before any NotApplied retry.
- [ ] Actual FilmCraft runs on Windows/macOS and human editor review.
- [ ] PR #28 independent G2 review and explicit candidate acceptance remain separately pending.

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
