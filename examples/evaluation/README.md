# Test a reading and a handoff

English · [繁體中文](README.zh-TW.md) · [Choose a case](../README.md)

Use a facilitator, readers from the intended audience, and an editor who did not write the drafts. You can run either exercise. All source packets are synthetic. The two CSV forms contain headers only. Keep completed forms and real edits outside this public repository. Use anonymous codes.

## 1. Did the audience perceive the intended choice?

1. Pick one case and one language. The author or facilitator reads the [scoring key](keys.md) and fixes the intended first focus, reveal and acceptable evidence for A and B before showing the work. To test newly generated drafts, replace the sample texts, source map and key with independently reviewed ones first.
2. Copy **only the body** of each reading file into a separate plain document with a neutral name. Keep source cards, role labels, author requests, C, the key, filenames and GitHub navigation out of the session. For audio scripts, use the same performer and setup for both. For composition briefs, either test how illustrators read the briefs or render both and test the resulting images. Record both the medium and a neutral artifact version (for example, `clip-04-v2`). Keep the exact image, script or recording with the private study records so another rater can check the cue.
3. Give half the readers one version first and reverse the order for the rest. Use the same audience, language and medium in both arms. Before revealing the other version, ask: “What did you notice first? At what point did you learn something that changed your understanding? Which words, sound or detail gave you that impression? How would you tell it to someone else?” Record their first answer verbatim. Avoid naming the intended contrast.
4. Show the second version. Ask the same questions, then ask what changed for them. Record each exposure separately in [readings.csv](readings.csv). In the second exposure row, record the reader’s own comparison verbatim in **comparison_after_both**. Leave this field blank for the first exposure. Code first exposures separately.
5. Compare the response with the case key. Use **match** when the reader independently names the expected focus or reveal and points to a supporting cue. Use **partial** for a plausible focus without supporting evidence or a mixed reading; **different** for another focus; **unknown** for no interpretable answer. Preserve unexpected readings. Have a second rater examine disputed responses before reporting them.

Report matches/readers **for A and B separately**, plus one anonymous clue or counterexample from each. A small pilot can use five first exposures per version, with a proposed signal of at least four matches in each group. Fix the threshold before recruiting. A weak or inconsistent result calls for revising the wording, timing or composition and trying it with fresh readers. A preference vote answers a different question.

## 2. Does the next editor miss fewer revisions or change too much?

1. Give the editor the case's source cards, A, B and C, and **one** change card. Ask for actual revised copies of all three outputs. Freeze the originals. The editor must return an unchanged copy of any draft they decide to keep. Start each card from a fresh set. R1 and R2 are independent exercises.
2. Compare two conditions. **notes** uses the same materials and ordinary notes. **mapped** adds only the three-row source map from the facilitator key; copy that table alone. Do not give either group the answer key, expected IDs or the other condition's edits. Use the same editor tools, time budget and instructions. This comparison tests the value of the provided map. For a full Skill trial, record the time and effort to produce the map and drafts and follow the separate [N-01 protocol](../../docs/narrative/N-01-AUTHOR-STUDY.md).
3. To limit learning effects, use matched cases/cards and alternate their assignment across editors. For example, editor 1 gets case X with notes and case Y with the map; editor 2 gets Y with notes and X with the map. Balance the order too. Record case, condition, language and elapsed seconds for each attempt, including unfinished attempts. Use equivalent time limits and report the number unfinished. Keep results for different media, languages and unbalanced task groups separate.
4. Enter all auditable returned draft IDs in **received**. An omitted draft has unknown content. Mark **finished=yes** only if the editor submits all three and declares the task complete. Compare the actual before/after copies. Mark a draft **changed** for a substantive edit. Mark it **corrected** when it satisfies the requested change in the key. Mark **collateral** when a protected fact, attribution, privacy choice, reveal or unaffected detail was also damaged, including inside a correctly revised draft. Let harmless rephrasing pass. Keep the offending sentence and check disputed decisions with another reviewer.
5. Copy [revisions.csv](revisions.csv). Enter space-separated IDs such as `A C`; leave an empty set blank. Use `en` or `zh-TW`, `notes` or `mapped`, and `yes` or `no` for completion. Save the private copy outside the checkout. From the repository root, run:

```sh
python3 examples/evaluation/score_revisions.py /path/to/revisions.csv
```

Use the scorer only with the supplied cases and their original [expected draft sets](expected.csv). For new outputs, freeze an independently reviewed key, source map and artifact version before the trial. Score against that private key separately.

**missed** includes expected drafts with no delivered correction, including unreturned ones. **unnecessary** counts substantive changes to received, unaffected drafts. **damaged** reports moderator-coded collateral damage in any received draft, including a correctly revised one. Keep the categories separate because they can overlap. **unreturned** identifies drafts whose contents could not be assessed. **exact** requires a finished task with no missed, unnecessary or damaged drafts.

For each condition, report distinct editors and completed/attempted tasks, exact/attempted tasks, missed/expected drafts, unnecessary/received unaffected drafts, damaged/received drafts, and unreturned/requested drafts (three requested per attempt). The score output lists the eligible draft IDs. Pool counts over comparable attempts; report a zero denominator as “not applicable.” Keep R1 and R2 separate when they differ in difficulty. Report median completion seconds for finished attempts alongside timeout and withdrawal counts. Compare within paired assignments. Only claim improvement when comparable observations support it; report a trade-off if one error type improves while another worsens. Separate setup time from handoff time. The [N-01 early decision rules](../../docs/narrative/N-01-AUTHOR-STUDY.md) belong to their original study.

## Exceptional inputs

| If this happens | What the facilitator records |
| --- | --- |
| A source disappears, two witnesses disagree, or a usage permission changes | The editor flags the affected item and requests a decision. Check indirect paraphrases manually; a direct-reference map can miss them. Keep usable material separate. |
| A plausible addition has no source | Attribute it as a proposal in the permitted version, omit it or ask the author. Check that it did not enter a factual account. |
| The request needs speech, pictures or a finished clip but the host can deliver text only | Test the script or brief and label the medium. To test actual listening/viewing, play or open the produced file with the intended audience. |
| Both versions lead readers to the same focus | Keep the answers, including contradictory evidence. Change one editorial choice, such as the first clue, and rerun with fresh readers. |
| A reader skips the second passage or a handoff times out | Retain the first exposure or partial edits; report the incomplete count. |

The examples support N-R01, N-R03 and N-R07 as additional materials. The story study, the Rust contract tests and author acceptance remain separate records. [Tasks](../../TASKS.md) · [Audience and handoff examples](../README.md)
