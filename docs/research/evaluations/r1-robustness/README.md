# R1 existing-score robustness

English · [繁體中文](README.zh-TW.md)

This is an append-only arithmetic review of the [existing R1 numeric
transcription](../2026-11-13-gemini-r1/README.md), based on main
`87113b9a204a801847995a334ef8728b9185b449`. No draft was generated or rescored;
no judge, participant, holdout, media file or host was invoked. R1's original
scores, keys, flags, artifacts and conclusions remain unchanged.

From the repository root, using Python 3.12 and its standard library:

```sh
python scripts/r1_robustness.py
python scripts/test_r1_robustness.py
python scripts/r1_package_checksums.py verify --ref HEAD --worktree
```

The first command writes UTF-8 LF JSON to stdout and never writes an input.
[report.json](report.json) is the observed deterministic output. It records
SHA-256 for the exact normalized transcription, original score-input script and
published statistics. The latter two hashes identify context; only normalized
ratings and declared keys enter the new calculations. These hashes establish
byte identity, not the missing generation provenance. The new directory is
pinned to LF for cross-platform checkout and comparison.

| Descriptive quantity | Exact result / approximate range |
| --- | --- |
| Declared story minus control | `-51/280` ≈ −0.182143 |
| Omit one case; 39 cases remain | −0.223443 to −0.146520 |
| Omit one dimension; 40 cases and six dimensions remain | −0.195833 to −0.175000 |
| Omit one batch; 30 cases remain | −0.271429 to −0.057143 |
| Individual batches 1–4; ten cases each | −0.114286, −0.557143, −0.142857, +0.085714 |
| Vary C01–C10 keys, hold other declared keys fixed | 1,024 arrangements; `[-17/56, -1/280]` |
| Additionally preserve the declared five-X/five-Y early allocation | 252 arrangements; `[-81/280, -1/56]` |
| Every case's key unknown, no count constraint | `[-173/280, +173/280]` |

All arithmetic uses integers and `Fraction`. Fractions are authoritative;
six-place decimal strings are approximations. The assignment bounds are
arithmetic extrema, **not confidence intervals**. The 1,024 and 252 arrangements
are sensitivity scenarios over the same cases, not extra observations. Holding
C11–C40 keys fixed does not verify them. Allowing all keys to vary admits both
signs. None of these checks establishes improvement, causal harm or equivalence.

The single-batch results expose heterogeneity without diagnosing its cause.
There is no new p-value or CI. Original R1's bootstrap resamples whole cases;
its marginal dimension intervals, Holm sign tests, unverified labels, ceiling
and fixed X-first presentation are already disclosed in the original report.
No new statistical coding defect was found in those computations.

## W4, W5 and W6 evidence ledger

| W4 proposition | Support and boundary |
| --- | --- |
| Published numeric outputs can be reproduced | [Disposable-copy receipt](receipts/original-reproduction.txt): all five generated artifacts byte-identical in the recorded environment. This cannot replay the missing judge requests. |
| Declared mean remains negative under each listed omission | Exact rows and case denominators in [report.json](report.json); keys remain unverified. This supports only the stated arithmetic. |
| The skill improves or harms writing | **Insufficient**: treatment manifests, balanced presentation and independent efficacy evidence are unavailable. |
| A new human/host outcome exists | **Unavailable**: none was collected. |

W5 recalculates each omitted case, batch and dimension from retained scores;
it does not reuse the original composite after removing a dimension. Enumerated
early keys change only the declared assignment scenarios. The all-key bound
prevents the indirect interpretation that early-key robustness authenticates
the remaining keys. No unfavorable case or original flag is removed from R1.

| W6 denominator | Count and interpretation |
| --- | --- |
| Existing case units | 40 purposely selected cases; possible shared topic/source/judge dependencies |
| Drafts / rubric ratings | 80 / 560; seven correlated dimensions per draft, not 560 cases |
| Omission scenarios | 40 case, seven dimension, four batch omissions; no new sample |
| Declared-condition ceiling | 147/280 story and 178/280 control ratings equal five; ceiling limits discrimination |
| New judge runs / humans / hosts / holdout cases | 0 / 0 / 0 / 0 |

The repository Skill snapshot is the above base commit, with Git blobs
`c54fdadc0b8c54d7bbb439e23b971cd548906670` (`skills/storytelling/SKILL.md`),
`26abd10663da989829d58af7b12153d4b842da6e`
(`references/research-workflows.md`), and
`fe31c8f6c6eb69bb7cf744c0058011ddf2f1f6e6`
(`references/editorial-rules.md`). These are pinned Git identities, not hashes
of platform-converted checkout bytes. Cloud `@storytelling` guidance was also
read on 2026-11-15; no identity with that separately supplied version is asserted.

## Verification and remaining gates

[Acceptance](ACCEPTANCE.md) was written before implementation. The observed
[RED](receipts/red.txt) failed because the module did not exist. A subsequent
test caught a hand-calculated synthetic expectation error; its
[failure receipt](receipts/synthetic-expectation-red.txt) and correction remain
visible. [GREEN](receipts/green.txt) records ten passing regressions: exact
synthetic math, assignment constraints, dimension recomputation, stable ordering,
invalid inputs, empty omission denominators and the real deterministic entry.

[Repository checks](receipts/repo-check.txt) and
[original-package verification](receipts/r1-package-verification.txt) passed.
The optional original-analysis reproduction requires R1's pinned NumPy/SciPy:

```sh
python docs/research/evaluations/r1-robustness/receipts/reproduce-original.py
```

It works in a disposable copy. This run used Python 3.12.14, NumPy 2.3.5 and
SciPy 1.17.0. Other environments may change recorded version fields; retain and
inspect the difference rather than overwriting R1. Hosted checks must be read
for the submitted head separately. This record does not close #121/#122, start
a holdout, revise skill guidance, supply maintainer acceptance or authorize G5.
