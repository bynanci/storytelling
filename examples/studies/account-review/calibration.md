# Facilitator calibration for the account review

[繁體中文](calibration.zh-TW.md) · [Source and change cards](README.md) · [Facilitator key](../../evaluation/keys.md#account-review)

Use these invented responses and edits to align raters before a session. They contain answers. Give participants only the materials assigned by the [study protocol](../../evaluation/README.md). Keep the public observation sheets blank. There are no participants, measured times or condition outcomes in this example.

## Read the opening and check the inference separately

Give the operations lead the body of [A](A.md) or [B](B.md) under a neutral name. Ask what they noticed first and what words led them there. Then ask them to retell the opening. Save the first response before showing the other version.

| First exposure | Illustrative first response and cue | Example retelling | Rater's decision |
| --- | --- | --- | --- |
| A | “I noticed the size of the drop. They said ‘8,800 minutes’ and ‘down 12%.’” | “Reported minutes fell. They want to check which sites the bill counts before interpreting the change.” | First focus **match**. The retelling preserves the measured drop and the open billing question. |
| B | “The first thing I heard was my old question: ‘did both still appear on this bill?’” | “They remember my question, and they suggest marking the move dates on the chart to compare them.” | First focus **match**. The reader identifies plotting as a proposal. Dates, coverage and cause are still open. |

Keep the opening judgment in `rating`. Code the retelling separately in `inference_status`, using the reader’s own words for any unexpected inference. Both sample retellings above simply report the text (`none`). Saying “I connected the question with the figure” or repeating the proposal to plot dates is insufficient evidence that a reader believes the moves caused the decline. If the reader's meaning is unclear, ask what supports the connection and preserve the initial wording. Use `unassessed` if the response still cannot be coded.

| Illustrative retelling | How to record it |
| --- | --- |
| “The two periods differ by 1,200 minutes.” | `grounded`. Subtracting the two supplied totals supports the statement; it makes no claim about the cause. |
| “Maybe a site dropped off the bill. I'd ask which entries were counted.” | `hypothesis`. Record the possibility in the reader's own words. The records supplied do not establish an omission. |
| “One site was left off the bill when it moved. That explains the entire drop.” | `unsupported`. The reader asserts both an omission and its causal effect. Preserve the quotation in `unexpected_inference`. |
| “Once we verify the two billing entries, we'll know why usage fell.” | `unsupported`. The reader overstates what the check can establish. It could clarify which sites were counted; an explanation of the decline would need further evidence. |

After collecting the independent readings, a sample `comparison_after_both` is “A made me look at the number first. B brought back the question I asked last time. A asks to check coverage; B suggests a chart.” Enter this on the second exposure only. The two versions also differ in their closing action. To test just the opening order, give both versions the same closing action and freeze a new key before gathering observations.

## Return three drafts for each change card

Start R1 and R2 separately from the original drafts. The links below identify exact copies to return when a draft stays intact. In an actual trial, collect each copy so the rater can inspect it.

### R1 · Correct the period total

S1 changes from 8,800 to 9,200 minutes. The baseline stays at 10,000. The decrease is 800 minutes, or 8% of 10,000.

**Return A**

> We logged 9,200 minutes of Service A in the last four weeks, down 8% from 10,000 in the previous four. You asked whether the bill covers both sites that moved. Can we establish which sites the bill counts before we interpret the change?

**Return B**

> When the two sites moved, did both still appear on this bill? You raised that question last time. The four-week totals went from 10,000 to 9,200 minutes, an 8% decrease. Could we mark the move dates on the usage chart?

**Return C** using the original [analyst note](README.md#draft-c). Its task is to identify the billing entries and request the dates. It cites S2.

### R2 · Change the proposed next step

**Return A** using the [original opening](A.md). It already asks to check coverage. Use the original usage figures for this independent card.

**Return B**

> When the two sites moved, did both still appear on this bill? You raised that question last time. The four-week totals went from 10,000 to 8,800 minutes, a 12% decrease. Could we compare the billing entries for each site with you?

**Return C** using the original [analyst note](README.md#draft-c), including its request to confirm dates. R2 withdraws the proposal to plot the dates. The analyst can still request them.

## Score what the editor actually returned

Each row below describes a separate synthetic return. Assume that the editor supplies all three identifiable drafts and marks the task finished. The six rows correspond in order to syn-1 through syn-6 in [the scoring input](calibration-revisions.csv). The zero-second entries are placeholders for a CLI check. A dash means an empty set. A rater first compares the actual sentences with the key and codes the substantive edits. The scorer calculates the output columns from those codes.

From the repository root, replay the six coded returns:

```sh
python3 examples/evaluation/score_revisions.py examples/studies/account-review/calibration-revisions.csv
```

| Card and return | changed | corrected | collateral | missed | unnecessary | damaged | exact |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R1, the reference return above | A B | A B | — | — | — | — | yes |
| R1, both A and B use 9,200 but leave 12%. C adds “Usage totaled 9,200 minutes in the second period.” | A B C | — | — | A B | C | — | no |
| R1, A and B correctly say 9,200 and 8%. B also says “The moves caused that drop.” C is intact. | A B | A B | B | — | — | B | no |
| R2, the reference return above | B | B | — | — | — | — | yes |
| R2, B is correct. A merely rephrases “which sites the bill counts” as “which sites the bill covers.” C is intact. | B | B | — | — | — | — | yes |
| R2, B is correct. A also asks the customer to choose weekly or monthly rows for a future comparison. C is intact. | A B | B | — | — | A | — | no |

For the second row, record the stale percentage as a missed correction in both A and B and the new, out-of-scope figure as an unnecessary edit in C. The figure itself is correct. The third row shows how an otherwise correct B can also sustain collateral damage. In the last row, the additional formatting question changes the scope of A. It makes no claim about why usage fell and leaves the analyst’s verification task intact. The condition names in the scoring input exercise the file format. These intentionally constructed returns do not measure a difference between conditions; use actual paired returns for that comparison.
