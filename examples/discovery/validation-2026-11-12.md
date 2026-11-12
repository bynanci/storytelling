# PR #102 participant trial record · 2026-11-12

[繁體中文](validation-2026-11-12.zh-TW.md) · [Discovery scenarios](README.md)

This page records anonymous human participant results for the two complete PR #102 trial packets on 2026-11-12. Participant status was confirmed by the maintainer. Some earlier records retain `syn-*` anonymous IDs for audit compatibility; that prefix no longer denotes a synthetic participant. The scenarios and drafts themselves remain synthetic test materials.

Only outcomes supported by verbatim responses, complete returns, or maintainer-provided session summaries are included here. AI calibration rehearsals are excluded from human denominators. No names, email addresses, or other personal identifiers are stored.

## Booking service notice

### First readings

Three first-exposure transcripts are available for review:

| First version | Readers | Expected first focus | Correct new-booking channel |
| --- | ---: | ---: | ---: |
| A · action first | 2 | 2/2 | 2/2 |
| B · system status first | 1 | 1/1 | 1/1 |

All three readers cited wording in the notice and chose the website for a new booking while treating phone service as existing-booking enquiries only. After seeing both versions, readers described A as more task-oriented and B as more like a system-status notice. The sessions did not measure actual task completion time, so comments such as “faster” or “more intuitive” remain participant impressions.

The A/B first-exposure sample is not yet balanced. A slot without a verbatim first-reading record is not added to the denominator.

### Takeover revisions

The maintainer supplied session summaries for two anonymous handoff editors:

- R1 · website recovery time corrected to 10:35: 72 seconds; update A and B, retain C.
- R2 · remove the review field: 55 seconds; retain A and B, edit C only.

Both editors reported using the direct-source list to identify affected drafts. The tasks differ, so the 72-second and 55-second results do not estimate time saved by the source map.

## Kiln handoff

### First reading

One anonymous participant with editing/production experience read B first (finished bowl first) and A second (kiln door closing first).

- First read of B: the first imagined image was the bowl on the worktable, with the opening sentence cited as the reason.
- The participant listed only K2 bowl footage and K1 door-closing footage as available to edit, and treated the logged kiln opening as missing footage to verify with production.
- Second read of A: the first imagined image became the closing kiln door, again tied to the opening sentence.
- In comparing versions, the participant noticed both the narrative order and B’s extra proposed “out of the kiln” title, so impressions such as a stronger gap cannot be attributed to ordering alone.

This session supports two observations: the opening changes the first image a reader forms, and a logged event can be distinguished from footage actually delivered.

### Takeover revisions

Two anonymous handoff editors each handled one change card and returned complete A/B/C drafts:

| Change card | Reported time | Must change | Must retain | Result |
| --- | ---: | --- | --- | --- |
| R1 · opening time 09:10 → 09:20 | 68 s | A, B | C | exact |
| R2 · withdraw title card, keep result-first opening | 52 s | B | A, C | exact |

Per-draft review:

- Missed changes: 0/3 drafts that required a change.
- Extra changes: 0/3 drafts that should have been retained.
- Collateral damage: 0/6 returned drafts.
- Missing returns: 0/6.

Both editors described using the source map first and then checking the affected wording in the draft.

## Product and engineering decision

This round supports continuing to test two product hypotheses:

1. **Readers can notice an intentionally different opening.** Both the booking and kiln cases produced first-focus responses aligned with their opening sentence.
2. **Explicit source mapping can help a handoff editor narrow revision scope.** None of the four handoff tasks in this round produced a missed change, cross-draft extra edit, or collateral damage.

The data **do not** establish that source mapping saves time because there was no matched control condition for the same task. One kiln reader is also insufficient to generalize to a broader editing population.

No additional guardrail or schema change is proposed from this round. A tool change should wait until at least two independent operators reproduce the same class of failure at the same step, with the original input, complete failed return, and expected result preserved.
