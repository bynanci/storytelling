# SK-E03 · Required facts and attribution regressions

English · [繁體中文](README.zh-TW.md) · [Parent issue #115](https://github.com/bynanci/storytelling/issues/115) · [Issue #120](https://github.com/bynanci/storytelling/issues/120)

This package turns four R1 error patterns into synthetic instruction-conformance cases. It contains bilingual prompts, inputs, expected invariants, failure conditions and exact output captures in [`cases-v1.json`](cases-v1.json). No original article text, private material, or R1 score is reproduced.

## Cases and rules

| Case | Pattern | Main checks |
| --- | --- | --- |
| `C03-SYN-01` | Financial release versus news-report attribution | Dates, place, denominators, units, qualifiers and speaker/source attribution. |
| `C04-C06-SYN-01` | Agency notice versus organizer advice | Agency scope, dates/place and the identity of the person who supplied logistics advice. |
| `C07-SYN-01` | Required fields absent from a packet | Unknown location, speaker and denominator stay unresolved; an optional omission remains distinct from an explicit contradiction. |
| `C24-SYN-01` | Juxtaposed indicators | Separate populations and units remain separate; descriptive co-movement is not rewritten as causality. |

The cases exercise W1, W4 and W5 from the existing skill at commit `e6b27d852ef259faf392645a8461c7e6dc006302` and editorial-rules revision 2. They do not change the editorial rules: this run found no demonstrated rule gap. English and Traditional Chinese are both covered.

## Output and evidence limits

The JSON captures the exact draft text produced while Codex applied the cited guidance in the authoring session on 2026-11-13. This is a development rehearsal, not a blind or independent model evaluation. The model family is recorded as GPT-6, but the serving revision was not exposed; the output cannot be reproduced as a frozen-model result. The case author and assessor were the same agent. Instruction-conformance outcomes are self-reviewed; narrative quality is not scored. The mechanical regression checks only required text presence/absence and record completeness. It cannot establish semantic truth or audience comprehension.

No failed model output was observed in this rehearsal. The failure conditions describe what would fail; they are not represented as observed failures. No human observations were collected. This package is not a holdout result and makes no claim that the skill is more accurate, useful or effective.

## Run the checks

```sh
python3 scripts/test_sk_e03_cases.py
```

The standard-library test checks the versioned record, all four required patterns, both languages, captured outputs and minimal mechanical assertions. It also verifies that the limits of the rehearsal stay explicit. Changes to the guidance require a fresh run with a separately frozen revision; do not rewrite these captures after seeing a future holdout.
