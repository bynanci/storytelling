# Five Codex tasks for multimodal authoring

English · [繁體中文](CODEX_MULTIMODAL_PROMPTS.zh-TW.md)

Use these prompts after the [first seven tasks](CODEX_NARRATIVE_PROMPTS.md). Submit one block per Codex session. Check the current main branch, the issue and open PRs before changing files. Reuse the [workshop](../skills/storytelling/references/workshop-demo.md), [V1 contract](narrative/NARRATIVE_DOCUMENT_V1.md) and [development gates](SDLC.md).

| Suggested order | Deliverable | Prerequisite |
| --- | --- | --- |
| [N-08 / #65](#n-08) | Two treatments with one testable style rule | Accepted #49. Share IDs and revision semantics with #51. |
| [N-09 / #66](#n-09) | A recording cue sheet | Accepted #49. The N-03 outline is optional. |
| [N-12 / #69](#n-12) | A real file from one host | Use #49 or the #68 manifest. |
| [N-11 / #68](#n-11) | A read-only source manifest and impact report | Replay #51 first. |
| [N-10 / #67](#n-10) | One selected profile | #53 must record a decision to adopt exactly one candidate. |

The first three tasks can run in parallel. N-10 and N-11 have separate prerequisites. For active work, add these acceptance details to the existing task.

| Task | Add to the existing session |
| --- | --- |
| #50 | Compare file and stdin output. Test a stale reference. Check that the input remains unchanged. |
| #51 | Record expected and actual variant IDs. Record missed and unrelated variants. Retain an edited or rejected suggestion. |
| #53 | Compare a gap in each template. Record the maintainer's selection or deferral. Keep a research recommendation separate. |
| #48 and #54 | Use the original matched author study. Keep actual user observations separate from engineering checks. |

## N-08

```text
In bynanci/storytelling, work on #65, N-R08. Read AGENTS.md, TASKS.md, the issue and ADR 0005. Check main and open PRs. Use stable unit IDs, revisions and variant IDs. Keep the style profile separate from frozen V1.
Ask $storytelling for documentary and poetic openings to the same Saturday workshop invitation. Hold the audience, facts and requirement to bring one umbrella fixed. Use the notice and the two taps. Identify invented imagery.
Check one observable rule through a local entry point: the cue precedes the reveal in the poetic variant. Allow the opposite order in the other variant. Test conflicting rules in one variant and an unsupported profile version. Compare repeated output. Attach fixtures, diagnostics, exit codes, regression results and exact-head CI. Supply unlabeled drafts for blind reading. Record feedback only after a real reader responds.
```

## N-09

```text
In bynanci/storytelling, work on #66, N-R09. Read AGENTS.md, TASKS.md, the issue and accepted #49. Reuse the N-03 draft if useful. Define a versioned audio cue sheet outside frozen V1.
Ask $storytelling for two beats of narration, silence and two taps. Use the supplied synthetic taps.wav as an effect. Identify the speaker, track, source, order and time unit. Distinguish declared, estimated and unknown times.
Test permitted cross-track overlap, adjacent intervals, reversed intervals, duplicate IDs and conflicts on an exclusive track. Preserve unknown duration. Replay a valid and an invalid case through the CLI. Compare bytes and exit codes. Attach a recording-ready sheet, observed failing and passing tests, and CI for the exact change.
```

## N-12

```text
In bynanci/storytelling, evaluate one host capability for #69, N-R12, gates G0–G2. Read AGENTS.md, the issue, #49 and relevant OF-37 through OF-39 entries. Continue a previously selected case.
Otherwise, render the authorized workshop bench.svg to PNG with one local host. Request the folded blue umbrella on the left and the spool on the right. Record the version, authorization, inputs and command. Decode the actual output. Check its size and composition. A transcription trial needs authorized speech and a reference transcript. The bundled WAV contains only taps.
Run the same request without the capability. Deliver a usable brief and identify the missing tool. If only the fallback runs, record the blocker and rerun condition. Submit evidence for adopting, narrowing or deferring the integration. Distinguish a recommendation from a maintainer decision.
```

## N-11

```text
In bynanci/storytelling, work on #68, N-R11. Read AGENTS.md, #49, #51 and the current open PRs. Replay both N-04 impact queries before building on them. If unavailable, record the missing prerequisite.
Use the workshop notice, SVG and WAV in a versioned, local, read-only manifest. Distinguish actual files, descriptions, transcripts, proposals and missing material. Ask $storytelling to move the event from Saturday to Sunday.
List the expected affected outputs before querying: invitation, poster date and both spoken dates. Preserve the layout and original audio. Compare exact IDs and report missed or unrelated outputs. Test a nonexistent ID, a stale revision, a poster-only proposal outside its scope and a description-only handoff. Use digests for byte identity. Attach deterministic diagnostics, unchanged inputs, regression results and exact-head CI.
```

## N-10

```text
In bynanci/storytelling, assess #67, N-R10. Read AGENTS.md, #49, #53 and the current PRs. Link to the decision adopting one candidate. If it contains only a recommendation or defers all candidates, report the condition for resuming and stop. Otherwise implement only the selected profile.
For a screenplay, allow event order A then B with reveal order B then A. Reject a missing scene.
For a brainstorm, allow reciprocal associations. Preserve unresolved branches. Reject a cycle of hard precedence.
For story-to-video, show a selected unit mapped to declared media. Keep missing media visible as unmapped. Reject an explicit binding to an absent asset.
Start with the author's request to $storytelling. Deliver a local entry point, one passing and one failing fixture, exit codes and repeatable output. Preserve V1 compatibility. Report observed test failure, the fix, legacy video regressions and same-head CI. Explain what the new rule can check beyond the template.
```
