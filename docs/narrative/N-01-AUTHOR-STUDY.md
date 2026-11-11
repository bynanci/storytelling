# Author study: two neighbors repair a gate

English · [繁體中文](N-01-AUTHOR-STUDY.zh-TW.md)

Issue #48 · N-01 / N-R01 · G0–G1. The original packet used main commit `260485b2ff5b8d7b46215004e21a5bb1a37e6dfb`. This is a fictional exercise for a community newsletter. The packet contains no human trial results. Its source IDs describe this exercise. The [accepted data contract](NARRATIVE_DOCUMENT_V1.md) has its own specification.

## Source cards

| ID | Type | Material | Boundary |
| --- | --- | --- | --- |
| S01 | Observation | On Saturday morning a toolbox stood beside the shared wooden gate. Mei and An stood by the gate. | No address, date or ownership information. |
| S02 | Observation | Mei held the gate. An tightened a screw in the upper hinge. | The notes do not identify who discovered the problem. |
| S03 | Observation | They closed the gate once. This time it cleared the ground. | One test gives no evidence of long-term repair or safety certification. |
| S04 | Observation | They returned the tools to the toolbox. | No time or cost data. |
| I01 | Interpretation | Working together on a small problem can illustrate mutual help. | This is the author's reading of the scene. |
| C01 | Creative proposal | An imaginary red umbrella rests against the fence. | Illustration A only. Keep it out of the spoken account and illustration B. |
| U01 | Unsupported claim | “Our program repairs every gate in ten minutes on average and cuts repair costs by 30%.” | No sample, time, costs or comparison supports the claim. |

## Participant materials

**Spoken outline, about 30 seconds.** Start with Mei holding the gate while An tightens the upper hinge screw (S01, S02). Describe the single successful closing test (S03). End with the author's interpretation about neighbors helping each other (I01).

**Illustration A.** Draw a wide, eye-level three-quarter view. Center the gate. Include both neighbors, the toolbox and their different roles. Mei holds the gate while An works on the hinge. Put the imaginary umbrella against the fence to one side (S01, S02, S04, I01, C01).

**Illustration B.** Use a low, close view of the upper hinge, the frame and two pairs of hands. Label Mei as the person holding the gate and An as the person tightening the screw. Keep only the details needed for the action (S02, S03, I01).

**Commercial copy.** The example describes two neighbors adjusting one hinge screw. It gives no repair time or cost. Keep U01 out of factual copy and headlines.

### Correction card

> The two repairers in S01 and S02 were Ju and Zhe. Ju held the gate. Zhe tightened the screw. Revise the affected outputs and keep the unaffected details. Which outputs changed? Where did the red umbrella come from? What evidence supports U01?

## Run the trial

1. Recruit five authors who have not seen the packet. Use anonymous IDs. Do not collect names or video.
2. Freeze the measures and thresholds below. Prepare two fresh copies of the participant materials. Keep the scoring key with the facilitator.
3. Assign three participants to order AB and two to BA. Randomize assignments. Allow eight minutes per condition.
4. In A, use ordinary text notes to update all affected outputs by hand. In B, use the current Storytelling skill with the same materials. Keep the host, model and settings consistent when possible. Record the actual prompt, versions and any differences.
5. Read the correction card without hints. Start the timer when the participant finishes reading. Stop when they save the revised packet. Save their output before asking the scoring questions.
6. Record native-tool trials in a separate round. Keep the rubric, sources and time limit the same. Record the commit and configuration. Do not pool native-tool and skill results.

A complete packet updates S01, S02 and the three drafts with the new names and actions. S03 and S04 stay intact. The participant must identify C01 as a proposal exclusive to A. They must distinguish I01 from observation and U01 from supported fact.

## Measures and decisions

| Measure | Procedure |
| --- | --- |
| Completion | Report the number who meet every condition within eight minutes and the total number in each group. |
| Revision time | Record seconds from the end of reading to submission. Use 480 seconds for a timeout and mark the task incomplete. Report the median. |
| Attribution errors | Count cases of C01 as observation, I01 as fact, U01 as supported, and C01 appearing in an unauthorized output. Record leakage separately. Treat a false source for the umbrella or evidence for U01 as a major error. |
| Repeated input | Count manual repetitions of the changed fact after the first central correction. Exclude generated output. |

Apply the original thresholds to the five paired results. Evaluate the skill and native-tool rounds separately.

- **Continue.** B requires at least four completions out of five, no major attribution errors and no worse attribution than A. At least one median must improve: 20% less revision time, one fewer repeated entry, or one fewer attribution error per person.
- **Stop.** B has fewer than three completions, at least two participants with major errors, or no observable improvement in quality, time, repetition or attribution.
- **Narrow and repeat.** Use this outcome when neither decision applies. Keep the scope where participants showed a benefit. Define new acceptance examples before repeating the trial.

| Anonymous ID | AB or BA | Condition | Complete | Seconds | Errors and type | Repeated input | Changed outputs / umbrella source | Observations |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |

For each condition, summarize the completion count and denominator, median time, major-error participants, total attribution errors and median repeated input. Record the facilitator's deviations. Leave unobserved fields blank. The maintainer records the decision, date, rationale and unresolved questions in #48.

## Facilitator key

Reveal this section after the trial. Update S01, S02 and the name labels in all three drafts. Keep S03, S04, I01 and U01 unchanged. Keep C01 only in illustration A. Replacing the names does not establish repair durability, costs or program results.

**Deliberately incorrect sample.** “Mei and An repaired the gate for good. The red umbrella witnessed their success. Our program repairs gates in ten minutes and saves 30%.”

This sample retains the old names, turns one test into a permanent result, moves C01 outside its scope and states U01 as fact. Score only the participant's own output.

## Historical engineering baseline

These timings cover the in-memory Python graph planner from PR #61. Each synthetic node connects to at most four later nodes. The change starts at the penultimate node. Each case affects two nodes. The historical run used CPython 3.12.14 and five batches of ten runs at the commit above.

| Nodes | Edges | Affected nodes | Batch median |
| ---: | ---: | ---: | ---: |
| 256 | 1,014 | 2 | 0.64 ms |
| 1,024 | 4,086 | 2 | 2.48 ms |
| 4,096 | 16,374 | 2 | 9.95 ms |

The original run passed six planner tests and five trace tests. These numbers exclude human revision time, model calls and file or network operations. See the [planner tests](../../scripts/test_storytelling_graph_plan.py) and [trace tests](../../scripts/test_storytelling_graph_trace.py).
