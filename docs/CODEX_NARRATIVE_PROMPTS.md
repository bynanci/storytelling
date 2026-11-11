# Seven Codex tasks for narrative authoring

English · [繁體中文](CODEX_NARRATIVE_PROMPTS.zh-TW.md)

Use one prompt per task in `bynanci/storytelling`. Read the current issue, main branch, open PRs and [agent instructions](../AGENTS.md) before starting. Reuse completed work. PR #71 accepted N-02. The current CLI includes N-03 and the N-04 revision queries. For newer tasks, use the [five follow-up prompts](CODEX_MULTIMODAL_PROMPTS.md).

## 1. Author baseline

```text
Work on #48 / N-R01. Open the N-01 author study packet. Use the same gate-repair notes for a spoken outline and two illustrations. Restrict the imaginary red umbrella to its assigned illustration. Identify the unsupported commercial claim. Prepare the matched correction trial with ordinary notes and the skill. Record completion, time, attribution errors and repeated input. Keep unobserved results blank. Attach the runnable packet and actual evidence.
```

## 2. Shared contract

```text
For #49 / N-R02, verify the accepted ADR 0005 and NarrativeDocumentV1 contract before proposing any change. Trace source units, revisions, variants, interpretations and creative proposals to the author's output. Distinguish association cycles from conflicting precedence. Check valid input, missing references, unsupported versions, duplicate fields and size limits. Preserve compatibility with video V1 and OTIO. Link existing acceptance evidence. Do not recreate the delivered contract.
```

## 3. A local authoring workflow

```text
For #50 / N-R03, replay storytelling render-narrative using the accepted N-02 fixture. Start with the author's request to $storytelling. Render one spoken outline and two illustration briefs from declared text. Compare file and stdin output. Check repeatability, unchanged input, malformed or stale references and operation without media. Run the video regressions. Record the commands, output and scope of the result.
```

## 4. Revisions and impact

```text
For #51 / N-R04, use the merged revision commands. Change a shared fact, then a detail unique to illustration B. Write the expected affected IDs before each query. Report any missed variant or unrelated one. Preserve old revisions and an author's edited or declined suggestion. Keep an approved prop local to its variant. Include the request, the changes, query results and regression evidence.
```

## 5. Claims in commercial copy

```text
Work on #52 / N-R05 after N-04. Start with the audience, claim, evidence, wording and next action. Ask which source supports one sentence. Test an explicit citation, a missing citation, a dangling reference and a revised source. Use the ten-to-eight-minute pilot example. Keep its result within the observed population. Provide a repeatable check and space for an author to assess the claim.
```

## 6. Select an additional use

```text
Work on #53 / N-R06 after N-04. Compare three requests: preserve competing brainstorm branches, distinguish screenplay events from reveal order, and link video beats to available media. Keep an unmapped beat visible. For each candidate, identify a gap in the template, the smallest useful implementation, its validation cost and author feedback. Record the maintainer's selection of at most one candidate or a decision to defer. Keep a recommendation separate from approval.
```

## 7. Author evaluation and delivery decision

```text
Work on #54 / N-R07 after #52. Use the same ordinary-notes baseline and tasks as #48. Run the spoken, illustration-revision and copy checks with actual authors. Report versions, completion counts and denominators, correction times, attribution errors and feedback. Record CLI timings separately. Leave missing human observations open. Give the maintainer evidence to continue, narrow or stop. Include compatibility, migration and rollback evidence for any release candidate. Follow G5 for publication.
```
