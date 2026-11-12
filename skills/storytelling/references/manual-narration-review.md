# Review narration through a host

[繁體中文](manual-narration-review.zh-TW.md) · [Skill](../SKILL.md) · [Audio template](multimodal-templates.md#audio)

Use this guide when an author wants recorded or synthesized narration and needs to choose its delivery. Storytelling helps prepare the script, source map and review. An operator, who may use an assistant, explicitly calls available production tools within the author's authorization. The Storytelling CLI does not synthesize voices, operate these hosts or certify their output.

## Start with one bounded request

This example uses the fictional workshop source, not a private recording:

```text
Use Storytelling to review a short workshop narration.
Keep the supplied event facts and the author's preferred earlier take as a
comparison. Prepare one short passage and identify the actual host operation.
Record the source, text and output revisions. Check content separately from
the author's opinion of the voice and delivery. Do not replace the full track
merely because the short sample is preferred.
```

Expect a speakable passage, a host action within the existing authorization, and a review record. If no suitable host or authorized reference is available, deliver the passage and cue sheet with recording still pending. These are operator instructions, not a new CLI command or automatic adapter.

## Keep evidence with the take

Keep sensitive evidence in the author's private project. Use neutral IDs in public discussions.

| Evidence | What the operator records |
| --- | --- |
| Authorization and source | Authorized use, source identity, actual file hash, reference crop and transcript revision. A processed copy is not automatically a raw recording. |
| Generation setup | Tool and source revision, model and weight identity, dependencies, precision, seed, speed and reference profile. Record both displayed text and normalized model input. |
| Actual result | Output hash, native sample rate and sample count, measured duration, host action, exit state and readback. Script timecodes remain estimates until measured. |
| Content check | Transcriber and settings, whether a prompt was supplied, estimated word times and unresolved words. Automatic speech recognition (ASR) can flag omissions; it does not establish naturalness or the author's usual pronunciation. |
| Listening and choice | Which file the author heard, comparison loudness, the feedback's scope and the next authorized step. Separate voice similarity, correct wording and natural delivery. |

A manifest declaration is not a byte check. A byte check is not a listening result. A completed host process is not an accepted narration. When a take changes, recheck its dependent subtitles and timing; do not reuse a review tied to another output hash.

## Preserve the author's choice

1. Keep the preferred take and its setup. Change one relevant factor for a controlled comparison. If both engine and reference passage change, report the combined change; do not credit the engine alone.
2. Check actual audio for missing words and clipped endings. Separate ASR findings from confirmed pronunciation errors. Do not prompt the transcriber with the expected text and then call its result independent.
3. Compare copies at similar loudness while retaining native files. A slower total duration can mean longer gaps, not less dense speech within phrases. Pitch and loudness do not prove relaxed delivery.
4. Record preference at its actual scope. A preferred short sample does not accept a paragraph or the complete narration. Existing authorization can cover full production or a review render using the selected voice; do not request the same permission again. Keep that authorization distinct from whether the author has listened to and accepted the full artifact.
5. If the author selects the first take again, recover its source, reference and settings. Keep known pronunciation fixes, including context-sensitive text normalization. Record the difference rather than promising identical output.

Use measured audio to place subtitles and preserve intended silence in authorized production or review outputs; record editorial acceptance separately. Do not accelerate speech only to fit an old script window. Keep declined takes as declined; a new model or a successful transcription does not override the author.

## When the author chooses to record personally

Treat the change as a new production instruction. Stop the remaining synthesis jobs that the operator owns and confirm that their workers have exited. Retain trials as excluded drafts; do not silently use a preferred synthetic sample to fill a missing line.

Deliver a verbatim recording script with separate cues for pauses, suggested recording windows and words that must stay off the recording. Preserve the author's wording and distinguish suggested timing from measured speech. Continue authorized picture and music work without the excluded voice takes. Label that output as a picture-first edition with narration pending; it is not the final mixed film.

When the author's recordings arrive, identify the actual files, measure their timing and revise dependent subtitles and music levels. Keep intentional silence intact. The change of voice source does not require restarting unrelated picture work or requesting permission already given for it.

## Bound host execution

On a memory-limited computer, serialize large inference stages and rendering. Set a small trial scope first. If a local guard cannot read memory or save its receipt, it should stop its owned child and report failure instead of leaving an unmonitored job running. Do not close unrelated applications implicitly. A polling guard is not an operating-system memory reservation; direct-child cleanup does not guarantee control of an arbitrary process tree.

For an external write with an unknown outcome, inspect the host before retrying. Retain failed and partial receipts. A portable tool can include pinned setup instructions and private reference profiles without an entire environment; validate its extracted entry point before calling it reusable. Do not publish private profiles with a general guide.

## An anonymized operator observation

In one personal-narration workflow, an author preferred the voice of an early short sample but rejected later passages as too formal or too fast. A slower trial still did not meet the requested delivery. Later alternatives passed independent transcription, yet the author returned to the first voice as the comparison baseline. This did not accept the full narration. The next check retained the earlier voice setup and a known text-normalization correction.

The author later chose to record personally. The operator stopped the remaining synthesis work, retained trial files as excluded drafts, and moved to a recording-script and picture-first handoff. Alignment to new human recordings remained pending. This preserved the author's latest choice without treating previous trial feedback as authority to continue synthesis.

The operator also found that a monitoring exception could leave its child running. A later check showed that a virtual-environment launcher could leave the actual interpreter outside direct-child cleanup. The guard was changed to assign the suspended child to a private Windows Job Object before resuming it; timeout, guard termination and monitoring/storage failures were checked with actual owned descendants. These host-side checks were separate from Storytelling's validators. This reported experience is not an independently reproduced study, a model ranking, a public host-compatibility test, or evidence that the film was completed. Private media and transcripts are omitted.

This guide supports N-09/N-R09 (audio cues and estimated timing), N-11/N-R11 (source and revision handoff) and N-12/N-R12 (actual host artifacts). It changes no schema, native command, task acceptance or release status.
