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

<a name="安裝與散布"></a>

## Skill distribution

- [x] **SK-D01.** Install from GitHub or add the skill as an npm development dependency. Both installations deliver the same skill files. See [installation](docs/PLATFORMS.md) and [SK-R01](docs/REQUIREMENTS.md#skill-distribution).

The maintainer decides whether to publish to a public registry.

<a name="跨媒體敘事任務"></a>

## Cross-media narrative backlog

| Task and requirement | Status and prerequisites | What to deliver |
| --- | --- | --- |
| [N-01 / #48](https://github.com/bynanci/storytelling/issues/48) · N-R01 | G0–G1. Packet ready. Can run alongside N-03. | Compare ordinary notes with the skill. Record completion, revision time, attribution errors and repeated input. Human trials and the maintainer's decision remain open. |
| [N-02 / #49](https://github.com/bynanci/storytelling/issues/49) · N-R02 | G2 accepted. Uses the N-01 packet. | [ADR 0005](docs/adr/0005-independent-narrative-document-v1.md) and the [V1 profile](docs/narrative/NARRATIVE_DOCUMENT_V1.md) define sources, variants, spoken outlines and illustrations. [PR #71](https://github.com/bynanci/storytelling/pull/71) records acceptance. |
| [N-03 / #50](https://github.com/bynanci/storytelling/issues/50) · N-R03 | Follows N-02. CLI on main. Author inspection remains open. | Render one outline and two compositions. Check provenance, failure cases, unchanged inputs, repeatable output and video regressions. See [commands and results](docs/validation/n03-narrative-cli.md). |
| [N-04 / #51](https://github.com/bynanci/storytelling/issues/51) · N-R04 | Follows N-03. Revision queries on main. Human inspection remains separate. | Revise a shared fact, then one illustration. Find affected variants by direct reference. Retain earlier drafts, suggestions and declared decisions. See [revision evidence](docs/validation/n04-narrative-revisions.md). |
| [N-05 / #52](https://github.com/bynanci/storytelling/issues/52) · N-R05 | Merged PR #83 supplies the V1 baseline; [PR #87](https://github.com/bynanci/storytelling/pull/87) adds product pilot ReportV2. Gate-sample review is recorded; product author assessment remains pending. | `audit-copy-claims` displays audience, claims, CTA, cited historical text and reference revision status. The reviewed draft removes the unsupported program claim and labels proposals. N-01/N-07 participant measurements remain separate. See the [contract](docs/narrative/COMMERCIAL_COPY_V1.md) and [validation record](docs/validation/n05-commercial-copy.md). |
| [N-06 / #53](https://github.com/bynanci/storytelling/issues/53) · N-R06 | G0–G2 comparison complete; the requester explicitly deferred all three native profiles. | The [format-discovery packet](docs/validation/n06-format-discovery.md) records compatibility, smallest changes and the [deferral decision](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714). No task timing or usability result is claimed. N-10 remains deferred until a new single-mode decision names an actual author need. |
| [N-07 / #54](https://github.com/bynanci/storytelling/issues/54) · N-R07 | G4–G6. Follows N-05. | Repeat the baseline. Report completion counts, denominators, time and attribution errors per condition. Link failures to fixtures. The maintainer decides whether to continue, narrow the scope or stop. Consider release separately. |

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
| Style treatments | The CLI checks declared spoken-beat order with a separate revision-bound profile; it does not judge aesthetics. The label-free reader trial remains manual-pending. | [N-08 / #65](https://github.com/bynanci/storytelling/issues/65) implementation and evidence: [validation record](docs/validation/n08-style-profile.md). |
| Audio and podcasts | `render-cue-sheet` binds a voice line, pause and supplied effect to N-02, preserves unknown timing, and checks exclusive-track conflicts. Recording and mixing remain host work. | [N-09 / #66](https://github.com/bynanci/storytelling/issues/66): [profile](docs/narrative/NARRATIVE_CUE_SHEET_V1.md) and [replay evidence](docs/validation/n09-cue-sheet.md). |
| Screenplay, brainstorm or video mapping | Text templates are available. The requester deferred all three new native profiles in [N-06](docs/validation/n06-format-discovery.md). | [N-10 / #67](https://github.com/bynanci/storytelling/issues/67) stays open/deferred until an actual author need and exactly one adopted mode are recorded. |
| Mixed-media handoff | `render-handoff` binds file, description, transcript, proposal and missing-material declarations to N-04 revisions. It separates changed dates from reusable layout and sound. | [N-11 / #68](https://github.com/bynanci/storytelling/issues/68): [profile](docs/narrative/NARRATIVE_HANDOFF_MANIFEST_V1.md) and [replay evidence](docs/validation/n11-handoff.md). Human semantic review remains separate. |
| Actual image or audio file | N-12 used Inkscape 1.4 on Linux to export and reopen a synthetic PNG, then delivered a brief with the export entry point unavailable. [Trial and artifacts](docs/validation/n12-inkscape-png.md) | [N-12 / #69](https://github.com/bynanci/storytelling/issues/69): narrow-adoption recommendation awaits the maintainer. Schedule a production adapter separately. |

<a name="影片工程任務"></a>

## Next actionable work

| Video work | Scope and next decision |
| --- | --- |
| M0–M2 | The foundation, limited interchange and selected planner scopes passed acceptance. Additional hosts, the full solver and editorial acceptance remain open. |
| M3 | P1, P2 and the M3-02 journal passed bounded acceptance. M3-03 has an isolated test. Production execution, in-flight timeouts and broader host tests remain open. |
| M4 | The TypeScript client and single-writer integration require M2-03 and M3-03. |
| M5 and v1.0 | Test a second editor, independent users and distribution. Stable release needs migration, compatibility and recovery evidence. |

The [video task history](docs/delivery/VIDEO_TASK_HISTORY.md#next-actionable-work) lists individual dependencies, priorities and acceptance criteria. A full editor, codec suite, effects engine and arbitrary plug-ins remain outside the current backlog.

<a name="多人協作"></a>

## File coordination

One PR owns a public schema or time contract change at a time. Adapters and interfaces can proceed against a frozen profile. Check source, authorization and host compatibility separately.

<a name="已驗收工作與歷史紀錄"></a>

## Accepted lifecycle

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

