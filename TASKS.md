# Development tasks

<a name="storytelling-開發任務"></a>

English · [繁體中文](TASKS.zh-TW.md)

Authors can start with the [eight templates](skills/storytelling/references/multimodal-templates.md). Contributors can use the tables below to find a task, its prerequisites and its acceptance criteria.

| Goal | Start here |
| --- | --- |
| Understand an authoring problem | [Product rationale](docs/WHY_STORYTELLING.md) and [roadmap](ROADMAP.md) |
| Build or evaluate a narrative feature | [Narrative backlog](#cross-media-narrative-backlog) and [requirements](docs/REQUIREMENTS.md#proposed-cross-media-narrative-requirements) |
| Work with style, sound or several media | [Multimodal backlog](#multimodal-rust-backlog) |
| Integrate an editor or port a component | [Video work](#next-actionable-work) and [migration guide](docs/MIGRATION.md) |

<a name="如何判讀狀態"></a>

## Status rules

Use `proposed → ready → in-progress → review → verified → accepted`. Mark blocked work with `blocked`. Name the cause, owner and condition for resuming work. A checked implementation item means the code exists. Record automated checks, human trials and release decisions separately. Attach the tested commit to the PR. See the [development process](docs/SDLC.md) and [sample record](docs/validation/m0-foundation.md).

The [2026-11-11 open-issue audit](docs/validation/open-issues-2026-11-11.md) links requester feedback, corrected drafts, CLI receipts and the remaining human/dependency gates.

The [2026-11-14 delivery/blocker reconciliation](docs/validation/continuation-2026-11-14.md) records merged artifacts, exact-head/post-merge CI and the remaining sixteen open issues without granting technical or human acceptance.

<a name="安裝與散布"></a>

## Skill distribution

- [x] **SK-D01.** Install from GitHub or add the skill as an npm development dependency. Both installations deliver the same skill files. See [installation](docs/PLATFORMS.md) and [SK-R01](docs/REQUIREMENTS.md#skill-distribution).

The maintainer decides whether to publish to a public registry.

<a name="跨媒體敘事任務"></a>

## Cross-media narrative backlog

Use the [research-to-rule/workflow map](docs/research/README.md) to select relevant evidence and checks for these existing tasks. It adds no acceptance or runtime status.

| Task and requirement | Status and prerequisites | What to deliver |
| --- | --- | --- |
| [N-01 / #48](https://github.com/bynanci/storytelling/issues/48) · N-R01 | G0–G1. Continue with the Skill for the tested cross-media revision task. | Five authors tried both conditions. Three of five completed with notes and five of five with the Skill. See the [maintainer's decision](https://github.com/bynanci/storytelling/issues/48#issuecomment-6798756961) for timing, attribution and repeated entries. |
| [N-02 / #49](https://github.com/bynanci/storytelling/issues/49) · N-R02 | G2 accepted. Uses the N-01 packet. | [ADR 0005](docs/adr/0005-independent-narrative-document-v1.md) and the [V1 profile](docs/narrative/NARRATIVE_DOCUMENT_V1.md) define sources, variants, spoken outlines and illustrations. [PR #71](https://github.com/bynanci/storytelling/pull/71) records acceptance. |
| [N-03 / #50](https://github.com/bynanci/storytelling/issues/50) · N-R03 | CLI and requester inspection are recorded. Revision 2 answers the close-framing feedback without changing the original fixture. | Render one outline and two compositions. Preserve source IDs, C01 only in the wide draft and C02 only as a close-composition proposal. Repeat bytes and unchanged-source evidence are in the [validation record](docs/validation/n03-narrative-cli.md). N-01/N-07 participant trials remain separate. |
| [N-04 / #51](https://github.com/bynanci/storytelling/issues/51) · N-R04 | Follows N-03. Revision queries on main. Human inspection remains separate. | Revise a shared fact, then one illustration. Find affected variants by direct reference. Retain earlier drafts, suggestions and declared decisions. See [revision evidence](docs/validation/n04-narrative-revisions.md). |
| [N-05 / #52](https://github.com/bynanci/storytelling/issues/52) · N-R05 | Merged PR #83 supplies the V1 baseline; [PR #87](https://github.com/bynanci/storytelling/pull/87) adds product pilot ReportV2. Gate-sample review is recorded; product author assessment remains pending. | `audit-copy-claims` displays audience, claims, CTA, cited historical text and reference revision status. The reviewed draft removes the unsupported program claim and labels proposals. N-01/N-07 participant measurements remain separate. See the [contract](docs/narrative/COMMERCIAL_COPY_V1.md) and [validation record](docs/validation/n05-commercial-copy.md). |
| [N-06 / #53](https://github.com/bynanci/storytelling/issues/53) · N-R06 | G0–G2 comparison complete; the requester deferred all three native profiles on 2026-11-11. A later linked decision selected video coverage for N-10 only. | The [format-discovery packet](docs/validation/n06-format-discovery.md) records compatibility, smallest changes and the historical [deferral decision](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714). The later [N-10 selection](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432) does not claim task timing, media verification or usability results. |
| [N-07 / #54](https://github.com/bynanci/storytelling/issues/54) · N-R07 | Closed as not planned / stopped by the maintainer for the submitted external-wrapper attempt. | [Disposition and evidence](docs/validation/n07-pilot.md): 0/5 primary completions; 5/5 with major U01 errors. Frozen ce73836 / ReportV1 native acceptance remains unverified. A future native evaluation needs a separately versioned prospective protocol; no release is authorized. |

N-R07 follow-up preparation: [native round v3](docs/validation/n07-native-v3.md) supplies a separate prospective protocol and pinned-build preflight. It is not adopted or started; #54 stays stopped. [N07-REC01](examples/evaluation/n07-recorder/README.md) adds private SQLite capture, matched workspaces and interruption-safe raw records; engineering completion is separate from human acceptance.

Use the [first seven Codex prompts](docs/CODEX_NARRATIVE_PROMPTS.md) or the [five follow-up prompts](docs/CODEX_MULTIMODAL_PROMPTS.md) for the selected issue.

<a name="執行作者試用"></a>

### Starting N-01

For a separate reading or handoff exercise, use the [scenario collection](examples/README.md). Keep its observations separate from the original N-01 trial. The [account-review calibration](examples/studies/account-review/calibration.md) includes synthetic scoring examples and a separate test of what evidence limits add to a source map.

1. In [issue #48](https://github.com/bynanci/storytelling/issues/48), record the current main commit and overlapping PRs.
2. Give the author the [study packet](docs/narrative/N-01-AUTHOR-STUDY.md). Ask them to correct the repairers' identities. Ask which outputs change and where the umbrella came from.
3. Recruit five participants. Follow the packet's thresholds for paired eight-minute trials. Report notes, skill and any subsequent native-tool trials separately.
4. Attach anonymized results to the issue. Leave unobserved results blank. The maintainer records the decision against the original thresholds.

Keep the material, audience and facts constant when comparing treatments. Change one choice, such as when a listener learns the date. Ask a reader to describe the difference. Ask the author why they selected, edited or declined the proposal. A structural test can check order. Human trials measure revision effort.

<a name="多模態任務"></a>

## Multimodal Rust backlog

The [workshop](skills/storytelling/references/workshop-demo.md) supplies a notice, a diagram and a sound effect. Use the skill or Markdown to draft. Use the following tasks for repeatable checks.

| Deliverable | Available today | Next check or task |
| --- | --- | --- |
| Spoken account or illustration | The CLI renders declared material without media files or a running time. Revision queries report direct references. | [N-04 revision inspection](https://github.com/bynanci/storytelling/issues/51) |
| Commercial copy | The skill can organize claims and supporting material. The video auditor cannot assess a commercial claim. | [N-05 claims](https://github.com/bynanci/storytelling/issues/52) |
| Style treatments | The CLI checks declared beat order; two official-sheet readings satisfy the qualitative gate on maintainer attestation. Both actual orders were A→B; randomization is unrecorded. | [N-08 / #65 validation](docs/validation/n08-style-profile.md) records qualified closure, source reasoning, unclear points and the unchanged reader-sheet handoff. No aesthetic superiority claim. |
| Audio and podcasts | `render-cue-sheet` binds a voice line, pause and supplied effect to N-02, preserves unknown timing, and checks exclusive-track conflicts. Recording and mixing remain host work. | [N-09 / #66](https://github.com/bynanci/storytelling/issues/66): [profile](docs/narrative/NARRATIVE_CUE_SHEET_V1.md) and [replay evidence](docs/validation/n09-cue-sheet.md). |
| Screenplay, brainstorm or video mapping | Text templates are available. N-06 historically deferred all three; the requester later selected video-only `NarrativeVideoCoverageV1` for N-10, with the task of mapping two story units where one lacks footage. | [N-10 / #67](https://github.com/bynanci/storytelling/issues/67) accepted G2 in [PR #93](https://github.com/bynanci/storytelling/pull/93); bounded Rust validation and the read-only CLI merged in [PR #97](https://github.com/bynanci/storytelling/pull/97). All six exact-head hosted jobs passed in [run #376](https://github.com/bynanci/storytelling/actions/runs/43148901399). N10-05 is complete within its bounded evidence scope: Codex replayed the supplied valid/invalid JSON; the maintainer supplied one post-merge author-operated self-session (145 seconds, with prior exposure, no randomization or independent observer); Codex probed the supplied synthetic MP4. The self-session is qualitative, not an independent user study; V1 reads no media bytes. See the [implementation validation record](docs/validation/n10-video-coverage-cli.md), [execution record](docs/validation/n10-05-execution-record.md), and [#67 evidence](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001). G5 publication remains separate. |
| [N10-06A / #129](https://github.com/bynanci/storytelling/issues/129) · N-R10-06A | Engineering slice merged in [PR #130](https://github.com/bynanci/storytelling/pull/130) as `125e5b1`; draft-only G2 remains tied to candidate `f148b6d`. Final head `b63552e` and post-merge main both passed all six CI jobs. | [ADR 0012](docs/adr/0012-draft-exact-video-range-input.md), [decision](docs/validation/n10-06a-g2-decision.md), [fixtures](tests/fixtures/n10-06a-draft/README.md), [guides](docs/proposals/n10-06a/README.md) and [append-only final validation record](docs/validation/n10-06a-exact-input.md) retain the exact input/unchanged-V1 boundary and observed RED/GREEN. Independent author use remains unobserved; #129 stays Open for B/C/D and separate media/host/authority evidence. |
| N10-06B/C/D / [#129](https://github.com/bynanci/storytelling/issues/129) | [B / PR #132](https://github.com/bynanci/storytelling/pull/132), [C / PR #133](https://github.com/bynanci/storytelling/pull/133) and [D / PR #131](https://github.com/bynanci/storytelling/pull/131) are merged proposal/checklist deliveries. | [B preflight design](docs/proposals/n10-06b/README.md) still needs focused G2 and implementation; [C contract/threat model](docs/proposals/n10-06c/README.md) needs independent G2/privacy scope, implementation and real-file evidence; [D dry-run checklist](docs/proposals/n10-06d/README.md) keeps declaration, media, content, host and authority states separate. No Reference App, media probe or Host action is supplied. |
| Mixed-media handoff | `render-handoff` binds file, description, transcript, proposal and missing-material declarations to N-04 revisions. It separates changed dates from reusable layout and sound. | [N-11 / #68](https://github.com/bynanci/storytelling/issues/68): [profile](docs/narrative/NARRATIVE_HANDOFF_MANIFEST_V1.md) and [replay evidence](docs/validation/n11-handoff.md). Human semantic review remains separate. |
| Actual image or audio file | N-12 used Inkscape 1.4 on Linux to export and reopen a synthetic PNG, then delivered a brief with the export entry point unavailable. [Trial and artifacts](docs/validation/n12-inkscape-png.md) | [N-12 / #69](https://github.com/bynanci/storytelling/issues/69): narrow-adoption recommendation awaits the maintainer. Schedule a production adapter separately. |

The [manual narration host guide](skills/storytelling/references/manual-narration-review.md) supplements N-09/N-R09, N-11/N-R11 and N-12/N-R12 with source, take and listening evidence from an anonymized operator account. It adds no native adapter or task acceptance; private-media observations are not a public reproducibility test.

<a name="影片工程任務"></a>

## Next actionable work

| Video work | Scope and next decision |
| --- | --- |
| M0–M2 | The foundation, limited interchange and selected planner scopes passed acceptance. Additional hosts, the full solver and editorial acceptance remain open. |
| M3 | P1, P2 and the M3-02 journal passed bounded acceptance. M3-03 has an isolated test. Production execution, in-flight timeouts and broader host tests remain open. |
| [M3-04 / #103](https://github.com/bynanci/storytelling/issues/103) | [PR #117](https://github.com/bynanci/storytelling/pull/117) merged proposal documents as `a23a290`: [ADR 0011](docs/adr/0011-production-execution-contract-v1.md), [Execution V1](docs/migration/EXECUTION_CONTRACT_V1.md) and [cases/work packages](docs/validation/m3-04-execution-contract.md). W01 authorization/implementation and final tested-contract G2 acceptance remain pending; no executor or current authority is supplied. |
| [M3-05 / #104](https://github.com/bynanci/storytelling/issues/104) | Blocked on explicit #103 acceptance; proposed single-attempt executor, followed by #105 recovery and #106 host acceptance. |
| M4 | The TypeScript client and single-writer integration require M2-03 and M3-03. |
| [M4-01 / #107](https://github.com/bynanci/storytelling/issues/107) · proposed M4-R01 | [PR #134](https://github.com/bynanci/storytelling/pull/134) merged the read-only facade [ADR 0013](docs/adr/0013-read-only-consumer-facade-v1.md), [English/繁中 guide](docs/proposals/m4-01-readonly-sdk/README.md) and [golden plan](docs/proposals/m4-01-readonly-sdk/test-plan.md) as `075d6b0`, all still G2 proposals. Eight operations reuse existing pure functions; facade implementation, public API freeze and CLI/SDK parity evidence remain pending. #107 stays Open; execution/recovery is unavailable until #103–#106 acceptance. |
| M5 and v1.0 | Test a second editor, independent users and distribution. Stable release needs migration, compatibility and recovery evidence. |
| [M5-02 / #112](https://github.com/bynanci/storytelling/issues/112) | Blocked on #109, #111 and an accepted M3 execution/recovery path. The [prospective v1 protocol](docs/validation/m5-02-rough-cut-beta-v1.md) and blank anonymous forms are design-only; no recruitment, measured session or beta result is claimed. |

The [video task history](docs/delivery/VIDEO_TASK_HISTORY.md#next-actionable-work) lists individual dependencies, priorities and acceptance criteria. A full editor, codec suite, effects engine and arbitrary plug-ins remain outside the current backlog.

<a name="多人協作"></a>

## File coordination

One PR owns a public schema or time contract change at a time. Adapters and interfaces can proceed against a frozen profile. Check source, authorization and host compatibility separately.

<a name="已驗收工作與歷史紀錄"></a>

## Accepted lifecycle

- [x] **M3-JL02/M3-JL06 maintenance.** The [first-readback receipt correction](docs/validation/m3-02-readback-receipt-collision.md) merged in [PR #137](https://github.com/bynanci/storytelling/pull/137) as `5bbe80d`, with observed RED/GREEN and all six final-head and post-merge CI jobs passed. This completes the receipt-collision maintenance scope and changes no execution authority.

The user authorized merging completed PRs after all required checks pass on the exact submitted commit. Check the resulting main commit. Releases, tags, repository policies and permissions require separate authority. Check actual media, host and human results independently. See the [merge record](docs/delivery/VIDEO_TASK_HISTORY.md#accepted-lifecycle), [origins](PROVENANCE.md) and [migration guide](docs/MIGRATION.md).

<details>
<summary>Find a record by its former task heading</summary>

- <a id="merged-candidate-pending-g2-acceptance"></a>[PR #28 candidate](docs/delivery/VIDEO_TASK_HISTORY.md#merged-candidate-pending-g2-acceptance)
- <a id="accepted-m0-delivery"></a>[M0 delivery](docs/delivery/VIDEO_TASK_HISTORY.md#accepted-m0-delivery)
- <a id="m1-01-delivery-and-work-tracking"></a>[M1-01 importer](docs/delivery/VIDEO_TASK_HISTORY.md#m1-01-delivery-and-work-tracking)
- <a id="m1-02-finite-otio-work"></a>[M1-02 OTIO](docs/delivery/VIDEO_TASK_HISTORY.md#m1-02-finite-otio-work)
- <a id="m2-01-bounded-brief-audit"></a>[M2-01 brief audit](docs/delivery/VIDEO_TASK_HISTORY.md#m2-01-bounded-brief-audit)
- <a id="m1-03-bounded-host-proof-work"></a>[M1-03 host proof](docs/delivery/VIDEO_TASK_HISTORY.md#m1-03-bounded-host-proof-work)
- <a id="m2-02-fixed-window-selection-and-planning"></a>[M2-02 planner](docs/delivery/VIDEO_TASK_HISTORY.md#m2-02-fixed-window-selection-and-planning)
- <a id="m2-03-selected-domain-live-planner-shadow"></a>[M2-03 live comparison](docs/delivery/VIDEO_TASK_HISTORY.md#m2-03-selected-domain-live-planner-shadow)
- <a id="m2-02o1-stable-ordering-optimisation--accepted"></a>[M2-02/O1 ordering](docs/delivery/VIDEO_TASK_HISTORY.md#m2-02o1-stable-ordering-optimisation--accepted)
- <a id="m2-02o2-cached-exact-time-construction--accepted"></a>[M2-02/O2 exact time](docs/delivery/VIDEO_TASK_HISTORY.md#m2-02o2-cached-exact-time-construction--accepted)
- <a id="m3-01p1-finite-otio-review-binding--accepted-bounded-profile"></a>[M3-01/P1 review binding](docs/delivery/VIDEO_TASK_HISTORY.md#m3-01p1-finite-otio-review-binding--accepted-bounded-profile)
- <a id="m3-01p2-selected-proposal-to-read-only-otio"></a>[M3-01/P2 OTIO review](docs/delivery/VIDEO_TASK_HISTORY.md#m3-01p2-selected-proposal-to-read-only-otio)
- <a id="m3-02-sqlite-operation-journal--accepted-bounded-storage-slice"></a>[M3-02 operation journal](docs/delivery/VIDEO_TASK_HISTORY.md#m3-02-sqlite-operation-journal--accepted-bounded-storage-slice)
- <a id="m3-03-filmcraft-executionreadback--bounded-proof-only"></a>[M3-03 host readback](docs/delivery/VIDEO_TASK_HISTORY.md#m3-03-filmcraft-executionreadback--bounded-proof-only)
- <a id="not-in-the-current-backlog"></a>[Deferred work](docs/delivery/VIDEO_TASK_HISTORY.md#not-in-the-current-backlog)
- <a id="openfilm-to-rust-transfer-ledger--2026-11-07"></a>[Historical source transfer](docs/delivery/VIDEO_TASK_HISTORY.md#openfilm-to-rust-transfer-ledger--2026-11-07)

</details>

## Gemini R1 skill evaluation record (SK-EVAL-R1)

The [40-case model-judged report and visual comparison](docs/research/evaluations/2026-11-13-gemini-r1/README.md) preserve numeric observations, source-packet errata and unverified declared assignments. This is W4/W6 diagnostic evidence, not a newly accepted requirement, human result or implemented skill improvement.
Use [#115](https://github.com/bynanci/storytelling/issues/115) as the umbrella. [SK-E01 / #118](https://github.com/bynanci/storytelling/issues/118) and [SK-E02 / #119](https://github.com/bynanci/storytelling/issues/119) are merged. [SK-E03 / #120](https://github.com/bynanci/storytelling/issues/120) is closed with four bilingual synthetic regressions and a non-independent development rehearsal; it is not an efficacy result. [SK-E04 / #121](https://github.com/bynanci/storytelling/issues/121) now has a frozen order-only protocol, anchored rubric and descriptive analysis tool ([preflight package](docs/research/evaluations/sk-e04/README.md)). Execution is blocked because original draft and packet bytes, exact prompts/rubric, and model settings are unavailable; unknown historical fields remain unknown. [SK-E05 / #122](https://github.com/bynanci/storytelling/issues/122) remains separate and cannot claim holdout efficacy until its dependencies are completed. Existing N/M acceptance, #54's disposition and #112's human-evidence gates retain their separate scope.

- [x] **SK-E04 / #121 input-integrity maintenance.** [PR #138](https://github.com/bynanci/storytelling/pull/138) merged strict JSON/UTF-8/discriminant and artifact ID/digest checks as `3e0f0d0`; all twelve final-head CI/matrix jobs passed, including 24 analyzer regressions with unchanged valid v1/v2 output. See the [RED/GREEN record](docs/validation/sk-e04-input-boundary.md). Matched-pair flip rates remain pair-weighted and case-score averaging is unchanged. This verifies the tool correction only; R1 rescoring and #122 remain blocked on authorized study evidence.

**SK-E06 / #115** supplies the remaining five adversarial [reference/manual cases](docs/research/evaluations/sk-e06/README.md): variant scope, indirect source removal, untrusted instructions/consent, invented human evidence and unavailable artifacts. The bilingual examples are authored references, not captured model/human results. `scripts/test_sk_e06_packet.py` checks packet integrity only; actual execution forms remain blank. #121/#122 and human/host/release gates remain open.
