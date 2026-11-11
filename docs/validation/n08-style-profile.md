<!-- SPDX-License-Identifier: MIT -->
# N-08 / N-R08 · Same source, two openings

**Issue:** [#65](https://github.com/bynanci/storytelling/issues/65)

**Base:** main at `27d6985d569c0e8882c4218b19115afc382e5aad` (after #74 merged)

**Inputs:** [NarrativeDocument fixture](../../tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json) and [StyleProfileV1 fixture](../../tests/fixtures/narrative-v1/n08-style-profile-v1.json)
**Contract:** [NarrativeDocumentV1](../narrative/NARRATIVE_DOCUMENT_V1.md) remains closed and unchanged. The independent [NarrativeStyleProfileV1](../narrative/EDITORIAL_STYLE_PROFILE_V1.md) names order rules by variant and beat ID.

## Author request and fixed material

Starting request from the Storytelling skill: **「讓鄰居知道週六修傘。給我紀實、意象兩個開場。」**

The document keeps the same audience in both variants: nearby residents hearing
about the workshop for the first time. S1 supplies the Saturday umbrella repair
event and the request to bring one umbrella. S3 supplies two synthesized taps
with no speech. The sound's cause is unknown.

| Unit | Source or role | How the drafts use it |
| --- | --- | --- |
| S1@1 | Supplied observation: Saturday repair workshop; bring one umbrella. | Same event line in both variants. |
| S3@1 | Supplied observation: two synthesized taps, no speech. | Documentary version places the cue after the event; imagery version opens with it. |
| P1@1 | Creative proposal derived from S3@1: “一下敲門，一下回應。” | Imagery version only. The metaphor is a proposed reading of the sound, not a claim about its cause. |

The audience, event day and item to bring stay fixed. The editable choices are
reveal order and the imagery-only metaphor.

## Rendered drafts

Reproduce these with the local renderer command below. Source tags stay outside
the spoken words so the author can trace each line.

### Direct-information opening

1. [D01_EVENT] [S1@1 | supplied_observation] 「週六有修傘工作坊，請帶一把傘。」
2. [D02_TAPS] [S3@1 | supplied_observation] （收尾播放 S3 的兩聲敲擊音效。）

### Imagery opening

1. [I01_TAPS] [S3@1 | supplied_observation] （先播放 S3 的兩聲敲擊；合成音效，沒有說話聲。）
2. [I02_METAPHOR] [P1@1 | creative_proposal] 「一下敲門，一下回應。」
3. [I03_EVENT] [S1@1 | supplied_observation] 「週六有修傘工作坊，請帶一把傘。」

The observable difference is when the audience learns about the event. The
first version names the event before the sound cue; the second starts with S3
and reveals the same event after the sound and metaphor.

## Rule check

The profile is bound to document n08-umbrella-openings revision 1 and checks:

- documentary-opening: D01_EVENT must precede D02_TAPS;
- imagery-opening: I01_TAPS must precede I03_EVENT.

The real local command returned exit 0:

    {"ok":true,"style_check":{"checked_rules":2,"document_id":"n08-umbrella-openings","document_revision":1,"profile_id":"n08-workshop-opening-order","scope":"declared-beat-order-only"}}

This confirms only declared beat order. It does not judge whether the imagery is
effective, whether listeners understand it, or whether the sound feels like a
knock.

## Negative cases

The contradictory profile requires I01_TAPS before I03_EVENT and
I03_EVENT before I01_TAPS within imagery-opening. The CLI returned exit 1:

    {"error":{"code":"narrative.style.order_conflict","path":"$.rules","rule_ids":["imagery-taps-before-event","imagery-event-before-taps"]},"ok":false}

The unsupported-version fixture requests profile schema version 2. It returned
exit 1 before rule validation:

    {"error":{"code":"narrative.style.unsupported_version","path":"$.schema_version"},"ok":false}

An acyclic but reversed documentary rule returns
**narrative.style.order_violation** at **$.rules[0]** with rule ID
documentary-taps-before-event. A profile bound to document revision 2 returns
**narrative.style.stale_document**; an unknown profile field returns
**narrative.style.unknown_field**. These all have CLI exit 1.

## Replay

Run from the repository root. The first command prints both readable drafts
with source IDs and origin labels. The second checks both rules. Repeating the
second command produces identical stdout bytes.

    cargo build --release --locked -p storytelling-cli
    ./target/release/storytelling render-narrative tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json
    ./target/release/storytelling check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-v1.json > /tmp/n08-check-first.json
    ./target/release/storytelling check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-v1.json > /tmp/n08-check-second.json
    cmp /tmp/n08-check-first.json /tmp/n08-check-second.json

The integration test repeats the style check twice through the actual CLI
binary and compares stdout bytes.

## Blind reader prompt

For a reader trial, show the two passages below as A and B, randomly swap their
order per reader, and hide the treatment names and source IDs. Do not explain
the intended difference before the reader responds.

**A**

週六有修傘工作坊，請帶一把傘。

（收尾播放兩聲合成敲擊音效。）

**B**

（先播放兩聲合成敲擊，沒有說話聲。）

一下敲門，一下回應。

週六有修傘工作坊，請帶一把傘。

Ask:

1. What is happening, and what are you being asked to bring?
2. Which passage tells you about the event sooner? What words or sounds led you to that answer?
3. What do you think the two sounds are? Which details came from the supplied material, and which might be an author's choice?
4. Was anything confusing or unsupported?

**Human feedback: manual-pending.** No reader trial was run for this change, so
there are no participant quotes or usability claims to report. Record actual
answers only after a person completes the blind read.

## Automated evidence

Local targeted result: cargo test --locked -p storytelling-cli --test style_profile — 7 passed.
Full workspace debug and release tests, formatting, Clippy with warnings denied,
rustdoc with warnings denied, repository traceability checks, skill distribution,
and the OpenFilm, official OTIO, StoryReview, StoryBrief and fixed-window planner
differentials passed locally.

Hosted CI must pass on this change's PR head; earlier checks on #74 do not verify
this commit. The GitHub PR checks provide the exact head and current results.
Human editorial acceptance remains a separate manual gate.