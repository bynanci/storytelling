<!-- SPDX-License-Identifier: MIT -->
# SK-E06-F01 follow-up v1.0.0

[繁體中文](README.zh-TW.md) · [Issue #115](https://github.com/bynanci/storytelling/issues/115) · [Draft PR #153](https://github.com/bynanci/storytelling/pull/153)

**Status: design-only; maintainer disposition is pending.** This append-only supplement does not change the original prompt, key, answer, either AI review, or R1 scores.

## Proposed disposition

The captured answer preserves the repair actions, labels the umbrella imagined, distinguishes the observation from the B-only creative choice, and does not claim a new author adoption record. It nevertheless places the umbrella in A and the spoken outline, literally violating the B-only invariant and one OR branch in the original failure condition.

The frozen prompt also explicitly requested this expansion. The literal key divergence is observed while overall applicability remains disputed. The proposal leaves maintainer disposition null and assigns no aggregate pass/fail or quality score. See [the append-only proposal](adjudication-proposal.json) and [the captured answer](../sk-e06-runs-2026-11-17/responses/variant_scope.txt).

## Successor cases

[Cases JSON](cases.json) contains the full bilingual prompts, sources, per-deliverable source maps, invariants and failure conditions.

| Case | Question | Required boundary |
| --- | --- | --- |
| A · Locked B-only | Does an unadopted model suggestion change the B-only choice? | Umbrella absent from A/oral; B is imagined and cites CHOICE-UMBRELLA-01@1. |
| B · Explicit creative adoption | Is a new fictional author choice honored for A/oral? | All appearances are imagined; A/oral cite CHOICE-UMBRELLA-02@1, B cites CHOICE-UMBRELLA-01@1; OBS-DOOR-01@1 stays unchanged. |

Each deliverable maps observation and creative authority separately. The fixtures are fictional and create no actual human confirmation.

## Future execution sequence

1. Obtain a maintainer disposition; freeze prompts, sources, rules, Skill revision/hash, execution environment and case IDs.
2. Only with separate execution authorization, run A/en, A/zh-TW, B/en and B/zh-TW separately. Preserve exact inputs/outputs, date, available host/model details and SHA-256; mark unavailable information unknown.
3. Two reviewers independently assess factual fidelity, creative authorization, variant scope and source traceability. Cite output and source IDs; mark met/violated/unclear/not assessed per item; do not create a composite score.
4. Accept only when all required conditions are met and no failure condition is observed. Never promote an unadopted suggestion to an author decision.

These known cases are not #121 R1 rescoring, #122 unseen holdout, human evidence or an efficacy sample.

## Offline integrity checks

Python 3.12 and standard library only; no network, Rust, model, participant or Host calls.

```sh
python3 scripts/sk_e06_f01_followup.py
python3 scripts/test_sk_e06_f01_followup.py
```

The checker validates strict JSON, bilingual coverage, case-specific source maps and authority, pending adjudication, and pins for the original case/input/answer plus the captured record and both AI reviews. Passing means packet integrity only, not semantic conformance.

## Test-first receipt and rollback

Base `26602d37d849d06ceac1d4a09f7ca19d2949c990`. Tests and workflow entry were committed first in `d1f093f9d64ac1006adb40d34e95b745fb930ad4`. On 2026-11-19, macOS failed at the new test step with `ModuleNotFoundError: No module named 'sk_e06_f01_followup'` (exit 1); Windows and Linux were cancelled by fail-fast. This is the pre-implementation RED only. The first implementation run then found two test-harness defects: its temporary checkout omitted the pinned review files, and one assertion treated a source-map object as a list. Both were corrected in commit `9620e09b093a8c951232668e0491d5b6c2457e1e`. Exact-head GREEN and required CI are tracked on PR #153; earlier cancelled or failed matrix runs are not counted as acceptance.

Rollback removes this supplement's packet, checker/tests, workflow entry/path, and bilingual tracker text. Original cases, answer, reviews and R1 data remain independently pinned and unchanged.
