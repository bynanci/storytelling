<!-- SPDX-License-Identifier: MIT -->
# N-08 / N-R08 · Same source, two openings

**Issue:** [#65](https://github.com/bynanci/storytelling/issues/65)

**Initial implementation base:** main at `27d6985d569c0e8882c4218b19115afc382e5aad` (after #74 merged)

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

The [standalone Traditional Chinese reader sheet](open-issues-2026-11-11/reader-sheet.zh-TW.md)
can be handed to a reader without the surrounding treatment labels or source
IDs in this validation record. The checked-in sheet remains a blank template;
the later attested sessions are recorded below, not filled into that template.
It contains text and sound cues and plays no audio.

For any additional reading, give the reader only that sheet. Before presenting
it, draw a fair coin for AB or BA and actually display, fold or cover its sections
in that sequence while keeping the original A/B labels. Privately record the
anonymous reader code, date/time/timezone, prior-exposure answer, draw and actual
order, including any deviation. A new blind reader must not have seen the drafts
or related explanations. Hide the treatment names, this validation record and
the source key; do not explain the intended difference before the response.

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

Save all four answers verbatim, including uncertainty and unexpected answers,
before revealing the source key. Keep a frozen original and its save time;
record the disclosure time and later discussion separately. Afterwards identify
differences, evidence used and unclear points by referring to those original
answers, with facilitator interpretation labeled separately. This procedure is
a handoff for any additional session, not a new reading or an automatic reason
to reopen the completed issue. The [original facilitator handoff](https://github.com/bynanci/storytelling/issues/65#issuecomment-6795973320)
provides a blank private log. Do not give that log's source context to the reader.

## Maintainer-attested closure · 2026-11-12

Issue #65 is **closed/completed on maintainer attestation with disclosed study
limitations**, as recorded in the [qualified closure](https://github.com/bynanci/storytelling/issues/65#issuecomment-6809829263).
The maintainer attests two readings of the original sheet at `5201c488`, with
the unchanged A/B passages and all four full questions. Prior exposure was
denied, no sound file was played, and all four original answers were saved
before the source key was revealed. These are facilitator accounts accepted by
the maintainer; they are not independently audited device telemetry. The private
verbatim answers and reader identifiers are not republished here. The following
table summarizes that public attestation, not newly collected answers or quotes.

| Attested session | Differences and evidence reported | Unclear or unsupported points reported | Answer save → key disclosure; actual order |
| --- | --- | --- | --- |
| First | Identifies the Saturday umbrella workshop and one umbrella; uses the explicit event sentence in A versus the opening cue in B to identify the earlier reveal. Distinguishes supplied synthesized taps from the author's placement and metaphor. | Unknown venue, time and sound origin. | 2026-11-11 21:18:42 +08 → 21:25:10 +08; A→B. |
| Second | Recognizes the same source-versus-editorial distinction according to the closure summary; no separate Q2 wording is published there. | Treating the metaphor as a literal event lacks support. | 2026-11-12 04:30:15 +08 → 04:36:00 +08; A→B. |

Both actual orders were A→B. The randomization method and draw were not supplied,
so this record does **not** establish a randomized presentation or control for
order effects. The second session's +08 time is a newly asserted correction to
an earlier export's `Z` suffix; the original export and its timestamps remain
unchanged. Neither device timing nor this summary supplies independent audit.

Earlier alternate-questionnaire responses remain separate supplementary
observations. The [first intake](https://github.com/bynanci/storytelling/issues/65#issuecomment-6798144382)
used a different fourth question. A [later intake](https://github.com/bynanci/storytelling/issues/65#issuecomment-6800952835)
preserves an unexpected B-earlier interpretation and an unclear link between the
opening cue and repair; that response must not be corrected into the intended
interpretation. The [subsequent clarification](https://github.com/bynanci/storytelling/issues/65#issuecomment-6809409205)
says exported question labels had been rewritten, so those labels alone do not
establish what was displayed. Preserve all submitted wording, answer sets and
timing discrepancies without silently merging files or inflating participant
counts. This document reconciliation does not overwrite any original response.

The two attested official-sheet accounts support the issue's bounded qualitative
observation criterion. They do not establish aesthetic superiority, preference,
general reader comprehension or broad efficacy. This update records the existing
closure and performs no new participant session.

For additional sessions, use the [facilitator checklist and blank private record](open-issues-2026-11-11/additional-reading-handoff.zh-TW.md). The reader receives only the unchanged sheet; new observations cannot retroactively supply the old randomization record.

## Merge reconciliation and N-08 acceptance scope

PR #75 was reconciled with the intervening main changes from #76, #77 and #79.
The English-first and Traditional Chinese documentation updates were retained
alongside the profile contract. The final PR head was
`99765e0eae28e0338d78fa465bf4c44edaf792d1`; exact-head CI run #265 passed all
configured jobs, and PR #75 was squash-merged as
`003191c834bfc46903f1a436fd73c5df6edd9a77`. At that original merge the blind-reader
criterion was still pending. The later qualified closure above records its
acceptance on maintainer attestation. Deterministic replay is covered by the CLI
integration test; it does not substitute for a human observation.

## Automated evidence

Local targeted result: cargo test --locked -p storytelling-cli --test style_profile — 7 passed.
Full workspace debug and release tests, formatting, Clippy with warnings denied,
rustdoc with warnings denied, repository traceability checks, skill distribution,
and the OpenFilm, official OTIO, StoryReview, StoryBrief and fixed-window planner
differentials passed locally.

PR #75 exact-head CI run #265 on `99765e0eae28e0338d78fa465bf4c44edaf792d1`
passed all configured jobs. PR #75 was subsequently squash-merged as
`003191c834bfc46903f1a436fd73c5df6edd9a77`. Reader observation was later accepted
on the qualified attestation above. Aesthetic and creative-intent judgments
remain human decisions; the automated checks do not grant them.
