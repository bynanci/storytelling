<!-- SPDX-License-Identifier: MIT -->
# Evidence-grounded choices for W01, the read-only SDK and preflight B

English · [繁體中文](g2-roundtable-2026-11-14.zh-TW.md) · [Source index](g2-roundtable-2026-11-14/evidence-index.json)

Baseline: `main@bca998784a0e9d3891cd7b28aab660d7d4ad7db0`, checked on 2026-11-14 UTC. No open PR existed at that check. This is a documentation-only selection record for [#103](https://github.com/bynanci/storytelling/issues/103), [#107](https://github.com/bynanci/storytelling/issues/107) and [#129 B](https://github.com/bynanci/storytelling/issues/129).

## Selected routes

| Choice | Selected recommendation | Boundary |
| --- | --- | --- |
| #103 / EX-W01 | Proceed with a narrowly scoped pure contract candidate: strict wire parsing, exact identity vectors, P2-owned compilation binding and opacity tests. | No `ExecutionPermitV1`, authority provider, project writer, dispatch or Host call. W01 cannot complete final #103 G2 or unblock #104 on its own. |
| #107 / ADR 0013 | Choose the eight-operation read-only facade in the existing engine crate, with direct backend delegation and full error preservation. | No execute/approve/recover method, transport, fifth crate or stable-release claim. Golden parity and actual consumer use remain unobserved. |
| #129 B | Choose a text-only preflight over A's unchanged input, after incorporating the small signature/order/escaping clarifications below. | A/V1 remains authoritative. No valid V1 payload on rejection, media probe, Reference App or execution action. |

These are selected **engineering recommendations for adoption**, not recorded human G2 acceptance. The requester asked Codex to judge the choices using a roundtable and supporting evidence:

> 根據圓桌會議判斷選擇內容，並加入真人與Host 支持研究證據 已送出的 #103 W01、#107 SDK、#129 B 三項技術選擇。

That instruction is recorded as the request to select and ground the proposals. It is not rewritten as a signature, a claim that a person reviewed the resulting exact candidate, or approval for a real project operation. The source index leaves formal maintainer acceptance and its identity/source null. The selected proposals still need actual adoption under their existing G2 rules before G3. Merging this document records the analysis only.

## How this roundtable was conducted

Four asynchronous, read-only **AI review roles in this Codex session** examined the same baseline: execution/security, SDK/API, author-facing CLI/UX, and evidence/methods. Codex synthesized their recommendations. No public prior roundtable record was found in repository text or GitHub issue searches for `圓桌` and `roundtable`; a private meeting may exist and has not been inspected.

These roles are not human participants or a live meeting. They share session context; the SDK and B reviewers had participated in the earlier designs. Their judgments are correlated and not blind, independent human review or an external security audit. No new author session, model-effectiveness experiment or Host execution occurred.

| Role | Recommendation and retained disagreement |
| --- | --- |
| Execution/security AI | Accept a pure W01 candidate. Defer production admission, trusted provider/fencing/quiescence and final #103 acceptance. A declared Host descriptor cannot become verified Host truth. |
| SDK/API AI | Accept the bounded facade basis after recording operation IDs, result imports and error accessor/trait behavior. Existing backend tests do not establish wrapper parity. |
| CLI/UX AI | Revise the few B details, then accept the bounded proposal. The current packet explicitly leaves its exact bridge and presentation rules pending; do not label it G3-ready before those decisions. |
| Evidence/methods AI | Existing observations support the problem and a limited next experiment. Include the stopped N07 wrapper trial; do not transfer Skill, reader or old Host outcomes into acceptance of these new APIs. |

There was no unconditional approval of the entire production execution contract or a vote by four people.

## Human evidence, including the contrary result

The source IDs resolve in the [index](g2-roundtable-2026-11-14/evidence-index.json). Cohorts remain separate; cross-record participant overlap is unknown and no unique-person total is inferred.

| ID | Existing observation | Permitted use and limitation |
| --- | --- | --- |
| H-N01 | Maintainer-attested five-author notes/Skill pilot: completion 3/5 versus 5/5, median revision time 310/185 seconds and median repeated entries 3/1. Reported order was 3 AB/2 BA with first-exposure screening. | Supports investigating source/variant revision work. Private logs were not independently audited; exact Skill prompt/host/model/settings were incomplete. The Continue decision applies to that Skill task, not CLI/SDK benefit. |
| H-102-READ | Three first-read booking records identified the website; one kiln reader distinguished a logged event from delivered footage. | Supports keeping source descriptions and missing footage distinct. No measured booking action or actual media inspection; a second reading is not a second first-exposure participant. |
| H-102-HAND | Booking handoffs took 72/55 seconds on different change cards. Kiln handoffs took 68/52 seconds; per-draft review reported no missed/extra edits or collateral damage. | Supports testing revision-scope visibility. Tasks lacked matched controls, source-map creation cost and known participant overlap. These times do not prove savings. The source report proposes no new guardrail/schema from this round. |
| VH-12 | One maintainer-supplied author, spreadsheet then manual structured-contract walkthrough, before the CLI existed. | Supports the problem hypothesis only. Dates/timing/prior exposure were not supplied in the original account; anticipated diagnostics were not program outputs. |
| VH-3 | Maintainer-author self-session, 2026-11-12 13:02 +08, prior S1/S2 exposure, self-reported 145 seconds. OUT `6/1` was rejected; `5/1` retained unresolved coverage. The author identified string-rational and iterative-first-error friction. | Closest B motivation. Codex did not observe the actions/timing; no randomization, independent author or B preview existed. S1/S3 times cannot establish a causal improvement. |
| H-N07STOP | Submitted external-wrapper workflow: 0/5 primary completions and 5/5 authors with a major U01 attribution error; maintainer chose Stop. | Contrary evidence must remain visible. The submitted build/argv/envelope could not be tied to frozen native `ce73836`; eligible native pairs remain 0, **待真人證據**. This neither demonstrates native failure nor supports wrapper benefit. #54's Stop and proposed, unadopted v3 protocol remain unchanged. |

H-N07STOP specifically argues for retaining exact build, argv and raw outputs, and separating a reference's existence, a claim's support and a person's approval. It does not justify rewriting R1 or treating this review as new participant evidence.

## Machine replay and actual Host evidence

| ID | Existing observation | What it cannot prove |
| --- | --- | --- |
| M-V1 | Codex replayed supplied N10 JSON on the recorded merged CLI: correct exit/JSONPath, unchanged files and file/stdin equivalence; seven CLI regressions passed. | Author actions, usability or benefit. The recorded 3.568/1.839 ms one-shot process measurements are not author time or benchmarks. |
| M-MEDIA | Codex probed a supplied **synthetic FFmpeg testsrc MP4**: H.264, 1280×720, 30/1, five seconds, 150 frames. | No proven binding from declared `CLIP-01` to those bytes, narrative-content match, actual captured footage or editor application. |
| HOST-M1 | Historical Codex harness used real FilmCraft 0.18.0 on Linux CPU with synthetic media: exact 25 fps/ticks, preview, save/reopen and OTIO reimport. | Human editorial quality, audio/GPU, another OS/profile, production authority, or general filesystem/media verification. |
| HOST-M3 | Historical isolated real FilmCraft import: Host exited successfully, reply was withheld, journal retained unknown, native/project-byte readback resolved applied; one import, zero retries/Undo, late reply rejected. | A still-running/hung Host, process-group quiescence, safe NotApplied retry, a production provider or current permission. The record explicitly has `approval_granted=false` and `executable=false`. |

Real Host means an actual application was exercised; its media and project were synthetic and isolated. These historical observations do not qualify the current production profile or constitute a new Host run in this task.

## Proposed details to adopt before implementation

### W01: compilation identity without execution permission

Use the exact [ADR 0011](../adr/0011-production-execution-contract-v1.md), [Execution V1](../migration/EXECUTION_CONTRACT_V1.md) and [case inventory/work packages](m3-04-execution-contract.md) at the baseline, with this narrower first implementation scope:

- Strict request/result **declaration** parsing, 65,536-byte/depth-8 bounds, duplicate rejection, deterministic error priority and the existing domain-separated identity/receipt vectors.
- Retain actual compiler-owned P2 bytes and binding through `BoundStoryReviewV1`; do not select new cuts/losses or accept a caller's artifact/digest as compilation evidence.
- Private-field, non-deserializable compiler-bound candidate and compile-fail checks. It is not an `ExecutionPermitV1`.
- Start with the pure portions of EX01/EX03/EX24–EX27/EX30–EX36 and independent vectors. The inventory is planned behavior, not tests that have passed.

A pure check must not emit normative execution `ready` or repurpose it as a contract-valid/compiler-bound label. Portable result states remain untrusted declarations when parsed. Any local “contract valid / compiler bound” label is distinct from current authority, verified Host state or dispatch readiness. Provider/approval consumption, project fencing, readback/quiescence and mutation cases stay with their separately reviewed runtime work. Final #103 acceptance still needs the required tested-contract evidence and the maintainer's actual decision; a subset of W01 tests does not complete every case.

### SDK: keep one typed delegation surface

Adopt the eight names/results in [ADR 0013](../adr/0013-read-only-consumer-facade-v1.md): `validate_plan`, `render_narrative`, `track_revisions`, `query_impact`, `video_coverage`, `plan_story`, `prepare_story_review`, `check_story_review`. Operation IDs equal these names. Curate the existing result aliases and `ReviewRecordV1` needed by immutable accessors; do not re-export unrelated backend constructors.

Choose the fallible `source_details_json() -> Result<Vec<u8>, SdkEncodingErrorV1>` with adapter-only `sdk.error_details_encoding_failed`. Preserve the original error even if encoding fails. Proposed `Debug`/`Display` expose only operation/code/path; `std::error::Error` must not expose private backend types or raw input. Code/path and complete opt-in details are the machine interface. Unresolved/stale remain their existing successful domain results.

The [golden plan](../proposals/m4-01-readonly-sdk/test-plan.md) must establish real external-crate compile/opacity checks and CLI/SDK parity. Narrative projections lack `Serialize`; use typed assertions and the existing formatter rather than adding a wire schema. An AI-written external-crate test is engineering evidence; it is not an independent human developer study.

### B: authority first, private projection, escaped text

Choose the existing candidate command `preflight-video-coverage-input <FILE|-> [--lang zh-TW|en]`, default zh-TW and opt-in `draft-video-input`. Retain exits 0/1/2/3. Proposed bridge: `render_draft_video_preflight(bytes: &[u8], english: bool) -> (String, bool)`, with the second value meaning declaration-valid only. Cross-crate linkage may require a feature-gated, doc-hidden public function; that is not Rust privacy or a stable SDK promise. View/row/hint types remain private; no public DTO/wire schema is added.

Run complete original bytes through A before projecting advice. Syntax/duplicate/budget failures yield no hints; helper failure explicitly says V1 was not run. Reuse A's exact arithmetic and the unchanged complete-input V1 result; no second range/reference validator. Retain all selected indices/order and unmatched mapping rows. For uninterpretable references show original index plus a fixed label; never invent unit/revision.

Propose deterministic hint ordering: mapping index, then fixed field order `unit_ref`, `status`, `asset_ref`, `source_range.start`, `source_range.end`, `reason`; missing-mapping selection hints follow in selection-index order. Nested time diagnostics retain A's ordering. Escape C0 U+0000–001F, DEL U+007F, C1 U+0080–009F, and bidi controls U+061C/U+200E–200F/U+202A–202E/U+2066–2069 as visible data without changing source bytes. Add malformed/duplicate selection, unmatched mapping, C1 CSI, CR/tab and bidi cases. Display-only conversions on rejected input are tentative; no V1 payload is emitted on rejection. The [B packet](../proposals/n10-06b/README.md) remains the proposal authority until these deltas are adopted. **No B code or author outcome exists yet.**

## Next evidence and decision handoff

Use the proposed [blank private collection aid](g2-roundtable-2026-11-14/handoff.blank.json); it does not adopt a research protocol or authenticate approval. Reuse existing B/SDK test plans and M3 work packages rather than create a duplicate study. Before any new measured session, select/freeze its protocol, build, materials, tasks, limits and scoring. A separately authorized candidate Host test can collect qualification evidence before final #106 acceptance; this document grants no such authorization. Record the original prompt, starting data, author verbatim response, observed behavior and limitations/failures, plus date/timezone, anonymous operator/observer, prior exposure, actual order, complete argv/raw output and deviations. Author time includes waiting; process time is separate. Preserve failures and unexpected answers.

| Track | Next actual evidence | Gate retained |
| --- | --- | --- |
| W01 | Observed test-first pure-contract/opacity RED/GREEN and exact-head compatibility checks; no Host calls. | Actual final #103 decision; #104–#106 production/candidate-Host authorization, implementation and qualification. |
| SDK | External-crate golden parity first; then a real developer outside API design attempts the documented workflow and records errors/help/feedback. | Actual human usability and any later transport/mutation/stable-release acceptance. |
| B | After implementation, a fresh author performs mapped/unresolved, seconds/frames, stale/range corrections and explains declaration readiness. | No completed B author trial yet. A time-saving comparison needs its own prospectively matched control. |
| Later Host | Separately authorized isolated profile tests: normal apply/readback/reopen, completed lost reply, in-flight/hung process, ambiguous readback, late reply and competing writer. | Do not execute now, infer quiescence from old lost-reply evidence, retry unknown, or test a real user project. |

Keep raw personal/media/path/token evidence private and publish only consented anonymous summaries or synthetic repros. Existing #54/#112 protocols and G5 remain separate. The current deliverable selects the three bounded routes and assembles support plus counterevidence; formal adoption, implementation and new human/Host outcomes are not filled from this AI review.
