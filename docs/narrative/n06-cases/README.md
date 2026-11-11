<!-- SPDX-License-Identifier: MIT -->
# N-06 author comparison materials

Issue #53 / N-R06 · SDLC G0–G2. Base: main `ce73836caa86502933f538821fdbf5dac92c1224`, after the accepted N-04 source/variant contract and merged N-06 deferral record. These independently authored materials deepen the existing comparison; they adopt no native profile or wire extension.

Give an author these three small cases, then record which gap they actually encounter. All material is fictional. Each case has a concrete first failure, a minimal alternative, a validation plan, planning-cost assumptions and blank task-specific feedback.

| Author material | Question to compare |
| --- | --- |
| [構思：保留兩條互斥路徑](brainstorm.zh-TW.md) | Do named drafts suffice, or must unresolved questions and mutually exclusive choices survive edits as machine-readable state? |
| [編劇：同一事件的兩種揭露](screenplay.zh-TW.md) | Do named chronology/reveal variants suffice, or must roles and same-event consistency be checked across views? |
| [影片銜接：有宣告片段與缺素材](video-handoff.zh-TW.md) | Can the author provide an actual clip/range for every selected unit, and what should remain unresolved? |

## Shared source boundary

| ID | Supplied observation, revision 1 | Boundary |
| --- | --- | --- |
| S02 | 阿梅扶住門；阿安收緊上方鉸鏈的一顆螺絲。 | This is a supplied fictional action, not an authorization to perform another repair. |
| S03 | 兩人試關一次；這一次門沒有磨到地面。 | One test does not establish durability or future results. |
| S04 | 兩人把工具收回工具箱。 | No timing, cost or video file is supplied. |

The brainstorm's P_KEEP/P_REPLACE are creative proposals. The screenplay changes disclosure only. Video asset/hash/range metadata is a hypothetical declaration; no media bytes were supplied. The narrative source revision and an asset identity are different identifiers.

## Compact comparison

| Candidate | Current fit | First concrete failure | Smallest change | Compatibility |
| --- | --- | --- | --- | --- |
| Brainstorm | Two named V1 variants can retain text, stable IDs, origin and direct source refs. | `option_state: unresolved` is refused as an unknown variant field. Without it, a consumer cannot distinguish pending choice from selected alternative by contract. | Keep an external option/question table first; add a separate branch-state profile only if authors need persistent queries. | Keep NarrativeDocumentV1 closed. N-04 author decisions are illustration-only and cannot be relabeled as brainstorm selection. |
| Screenplay | V1 can hold a named chronology variant and reveal variants, each with its own order; N-08 checks declared beat rules. | Typed `occurrence_order` is refused; names alone do not type occurrence/reveal roles or require the same event/revision set across views. Valid order does not verify truthful prose. | Use a chronology table or named variant beside existing reveal profiles; consider a separate role/consistency profile only for a demonstrated gap. | Preserve V1 relations, N-04 identities and N-08 rule/digest semantics. Do not turn association into temporal precedence. |
| Video handoff | Planner V1 validates declared asset IDs and exact fixed ranges; N-11 describes read-only sources/components. | The resolved subset can plan after the missing selected unit has been omitted; no current contract joins selected narrative coverage to assets. A proposed join field is refused. | Keep a non-executable coverage map with missing rows; consider a separate pure mapping validator before any compiler/host path. | No narrative ID substitutes for `asset_id`. Do not extend closed N-11 or change existing Planner/OTIO/P1 identities. |

## Validation, cost and feedback

A planning day below means about six focused engineering hours. Ranges are unmeasured estimates for one engineer familiar with the repository; exclude author recruitment, visual polish, production host integration and release work. They express uncertainty, not performance results or committed schedules.

| Candidate | Current probe / future validation | Cost assumption | Author evidence |
| --- | --- | --- | --- |
| Brainstorm | Normal render plus unknown-field rejection; future persistence must retain both alternatives/questions and allow zero or one choice per exclusive group, recorded separately if the author converges. | Text template 0.5–1 day; bounded native branch-state profile 3–5 days; low confidence until a real edit/query task exists. | Only the recorded defer-all decision. No branch task or answer to the case questions yet. |
| Screenplay | Both reveals render and N-08 rules pass; typed chronology-field rejection. A future profile needs role and cross-view event/revision checks while preserving existing per-variant order regressions. | Existing-profile material 0.5–1 day; typed role/cross-view consistency validation 2–4 days; low confidence on scene/dialogue requirements, which are excluded. | Only the recorded defer-all decision. No scene task, listener response or preference ranking. |
| Video handoff | Declared subset plans; join field rejects. A future mapping validator must reject unresolved required units, stale revisions, dangling assets and out-of-bounds ranges. | Coverage table 0.5–1 day; bounded mapping validator 4–7 days; low confidence. Actual-media/host proof is additional work, not included. | Only the recorded defer-all decision. No clip bytes or real handoff task were supplied. |

## Decision and use

The [decision record](decision.md) retains **“暫緩三者，等實際需求”** and recommends no native feature now. The cases are ready for an author to compare; task-specific answers remain blank. N-07 may proceed independently and must not treat this comparison as participant evidence.

The accepted [NarrativeDocumentV1](../NARRATIVE_DOCUMENT_V1.md), [N-04 ledger](../NARRATIVE_REVISION_LEDGER_V1.md), [N-08 profile](../EDITORIAL_STYLE_PROFILE_V1.md), [N-11 handoff](../NARRATIVE_HANDOFF_MANIFEST_V1.md) and [fixed-window planner](../../migration/FIXED_WINDOW_PLANNER_V1.md) are unchanged. JSON files with `unsupported` in the name deliberately add refused fields. `video-mapping-sketch.json` explicitly identifies itself as a discovery sketch, not an accepted V1 payload.

[Actual CLI probes](evidence/probes.json) distinguish existing representation from the unsupported additions. They establish declared-contract behavior only, not author benefit or media validity. Reproduce the commands in each case from the repository root after `cargo build --release --locked -p storytelling-cli`.
