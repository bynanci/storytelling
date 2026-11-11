<!-- SPDX-License-Identifier: MIT -->
# N-06 / N-R06 · Format discovery

**Issue:** [#53](https://github.com/bynanci/storytelling/issues/53)  
**Stage:** SDLC G0–G2 comparison complete; requester decision is to defer all three native profiles.
**Shared case:** the synthetic two-neighbor shared-door repair in [N-01](../narrative/N-01-AUTHOR-STUDY.md) and the N-04 revision ledger.  
**Evidence boundary:** this packet is a design comparison with a recorded requester deferral. No task session or timed trial has been run; the task-observation fields below remain blank.

## Shared material and comparison prompts

Keep the supplied observations fixed: S01 places Mei and An beside the shared wooden door with a toolbox; S02 says Mei steadies the door while An tightens one upper-hinge screw; S03 records one test close without scraping; S04 records both returning the tools. Do not turn a hypothetical branch into an observation.

| Candidate | Matched authoring task | Current fit and compatibility | Smallest reversible change |
| --- | --- | --- | --- |
| Branching brainstorm | Keep two possible next steps open: “if the adjustment holds” and “if rubbing returns.” Ask whether the author needs both alternatives visible before choosing. | NarrativeDocumentV1 already permits named variants, source references and creative proposals, but it has no explicit unresolved-branch state or branch-selection lifecycle. A pair of named drafts can preserve text but cannot reliably distinguish “not decided” from “selected.” | Start with a skill template using stable branch IDs, source refs, a what-if label, unresolved/selected status and an explicit decision question. No Rust type is justified until an author needs the status to survive edits or be queried. |
| Screenplay reveal | Keep event order S02 then S03, but try revealing the result S03 before showing the hinge action S02. Do not change what happened. | NarrativeDocumentV1 supports ordered spoken beats and N-08 NarrativeStyleProfileV1 checks declared must_precede relations within a spoken variant. It can represent this small reveal-order question. It does not model scene headings, dialogue turns or occurrence time separately from reveal order, so it is not a screenplay contract. | Reuse the current narrative variant plus N-08 order profile for a text-only probe. Add no screenplay schema unless an author needs scene/dialogue semantics beyond beat order. |
| Video handoff with absent asset | Select S03@1 as the intended moment, then identify which supplied video clip should carry it. If none exists, leave the mapping unresolved. | N-11 NarrativeHandoffManifestV1 can bind narrative unit references to declared file, description or transcript sources and artifact components, but it does not map those references to StoryBrief/Planner asset_id values or media ranges. Its file paths and digests are declarations, not host asset identities. A narrative ID must not be treated as an asset ID; unresolved mappings must not become planned shots or executable OTIO. | Test an explicit, non-executable mapping row that reuses N-11 source IDs and adds narrative_ref, nullable asset_id, mapping_status (unresolved or resolved) and a reason. Keep missing-asset rows blocked from planning/export. Do not extend closed N-11 or another contract until an author needs this mapping. |

## Compatibility and regression boundary

No code or existing contract changes are proposed by this discovery packet.

- N-02 NarrativeDocumentV1 remains the source for units, origins, revisions, relations and named variants. Its must_precede relation is scoped to a variant.
- N-04 NarrativeRevisionLedgerV1 continues to track source and variant revisions. A future branch or handoff must retain the old source revision and must not convert a proposal into an author decision.
- N-08 already checks declared beat order with a revision-bound profile. It is sufficient for a small reveal-order probe, not a full screenplay.
- StoryBriefV1 and PlannedStoryV1 select declared media asset_id values; the StoryReview/OTIO bridge binds plans to an imported source snapshot. N-11 separately tracks declared media files and artifact-component sources, but no current path maps a narrative unit to a Planner asset ID or media range.

If one candidate is selected, preserve these regressions:

| Candidate | Regression before implementation |
| --- | --- |
| Brainstorm branches | Keep narrative V1 positive/negative fixtures, duplicate/dangling-reference checks, N-04 history snapshots and declared-decision rejection passing. Test that unresolved alternatives survive replay and never appear as author-approved. |
| Screenplay reveal | Keep N-03 narrative render and N-04 source identity stable; exercise N-08 valid order, contradiction, stale-document and unsupported-version cases through the actual CLI. Verify occurrence facts remain S02 then S03 when reveal order changes. |
| Video handoff | Keep N-11 manifest fixtures/tests plus video V1 planner and OTIO round-trip tests passing; add resolved, unresolved, missing and mismatched unit/asset cases. An unresolved or absent asset must stop planning/export with a stable diagnostic and no host/catalog write. |

## Author review packet

Ask the author to use the same door-repair material for each prompt, then choose at most one next profile.

1. Which output would you actually make next: alternate brainstorm paths, a reveal-order script, or a handoff from a source unit to video?
2. Which step currently causes repeated work or uncertainty? What would you do with ordinary notes?
3. For the brainstorm, should both next steps stay explicitly unresolved? Is a text table enough?
4. For the screenplay, is changing reveal order while preserving event order useful? Are ordered beats enough, or do you need scenes and dialogue?
5. For video handoff, N-11 can identify declared source files but not a Planner clip or range. Which clip corresponds to S03@1? If there is no clip, is an explicit unresolved row useful, and should it block export?
6. What is the smallest change that would make the chosen task easier?

| Session | Candidate | Author answer / observed task | Smallest change requested | Compatibility concern |
| --- | --- | --- | --- | --- |
|  |  |  |  |  |
|  |  |  |  |  |
|  |  |  |  |  |

No task-specific answers or time measurements were supplied. The requester
provided only the adoption decision below; do not infer a measured preference
from the templates or N-08 implementation.

## Explicit requester decision · 2026-11-11

The requester read the three V1 scopes and the comparison summary, then selected:

> 暫緩三者，等實際需求

The [decision is recorded on Issue #53](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714).
It adopts no additional native profile. This is a requester decision, not an
independent author trial or a claim that a particular workflow saves time.

| Candidate | Recorded disposition | Current path |
| --- | --- | --- |
| Branching brainstorm | Deferred with the other two candidates; no branch task was performed. | Retain named text alternatives; revisit a persistent unresolved/selected lifecycle only for an actual author need. |
| Screenplay | Deferred with the other two candidates; no scene task was performed. | Reuse N-08 for the bounded reveal-order probe. A separate occurrence/reveal contract is not adopted. |
| Story-to-video mapping | Deferred with the other two candidates; no source clip was supplied. | Keep missing mappings unresolved and use N-11 for read-only source handoff. No timeline/export authority is added. |

## Recommendation and gate

**Recommendation: defer all three native profiles until author feedback identifies an actual gap.** The existing text workflow can hold alternative ideas; N-08 covers the narrow reveal-order rule; and N-11 already provides read-only source/artifact handoff but not unit-to-video-asset timeline mapping. That mapping cannot be inferred safely. If the author chooses the screenplay probe, reuse N-08 before adding a schema. If video is selected, preserve unresolved mapping and keep execution disabled until an asset is declared.

This explicit deferral completes the N-06 comparison/decision scope. Keep #67
open and deferred. Resume N-10 only when an actual author request identifies a
workflow gap and a new linked decision adopts exactly one mode, its author need
and version scope. A recommendation or this deferral is not a mode adoption.

## Automated validation

Exact-head hosted run [#317](https://github.com/bynanci/storytelling/actions/runs/42995948102) passed all six configured jobs on `e16997305d881f235709c68d8434175fc77f7239`: quality/contracts, dependency audit, Linux, Windows, macOS and required gates. This confirms repository consistency and existing regression status for the packet. It does not establish author need, preference or acceptance.

## Reproduction and document checks

This packet changes no runtime behavior. From the repository root, run python3 scripts/check_repo.py and cargo test --locked --workspace. These checks establish repository consistency and existing regression status only; they do not establish author need or acceptance.
