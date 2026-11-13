# SK-E04 · Judge calibration and order-only rescore

English · [繁體中文](README.zh-TW.md) · [Parent issue #115](https://github.com/bynanci/storytelling/issues/115) · [Issue #121](https://github.com/bynanci/storytelling/issues/121)

This package prepares, but does not execute, an order-only rerun of the 40-case Gemini R1 comparison. The R1 report says all 40 presentations were X-then-Y, with 27 X preferences, 10 Y preferences, 3 ties and 325/560 ratings equal to 5. Those observations do not identify an order effect.

## Preflight result: blocked

The public R1 package contains numeric ratings and summaries. It does not contain all original X/Y draft bytes, complete source packets, exact prompts, the exact rubric, judge model revision/settings, or verbatim judge requests/responses. The original X/Y assignment keys for some early cases are also only recorded in a compacted conversation summary. The [source report](../2026-11-13-gemini-r1/README.md) documents these limits. None of these missing historical fields is reconstructed here.

Because the order-only comparison must hold text, packet, prompt, rubric and model settings constant, there is no valid original-text order rerun to perform from this repository. A new synthetic or prospective study would be a different run and cannot be described as a rescore of R1. This package adds no model or external-service call.

## Frozen plan and rubric

- [`protocol-v1.json`](protocol-v1.json) freezes the estimand, case-level analysis unit, run separation, missing-data rules and exact resumption gate.
- [`rubric-v1.md`](rubric-v1.md) gives independent anchors for factual accuracy, traceability, uncertainty, narrative coherence, clarity, audience fit and required-task coverage. It is for a separate calibration run; do not use it to replace the unknown R1 rubric in an order-only run.
- The order-only run requires one fresh context per case and order, with the same exact X/Y texts, packet, judge prompt, rubric, model revision and settings; only presentation order changes. The judge sees neutral IDs and no R1 scores, declared assignments or key.
- Preserve raw output and annotations before adjudication. Store adjudicated annotations separately with the reason. Record missing calls, timeouts and unresolved source checks.

The analysis helper computes case-level preference flips, the X-minus-Y score-gap shift across orders, per-draft/per-dimension absolute score changes, annotation disagreements and W4 source-check counts. It emits descriptive records only: no significance test or population claim. Repeated calls and the seven rubric dimensions do not increase the independent case count.

## Synthetic tool fixture

`fixtures/synthetic-order-pairs-v1.json` exists only to test the record validator and analysis helper. It has four invented pairs; its values are not copied from R1 and are not model observations. Run:

```sh
python3 scripts/sk_e04_order_analysis.py analyze \
  docs/research/evaluations/sk-e04/fixtures/synthetic-order-pairs-v1.json
python3 scripts/test_sk_e04_order_analysis.py
```

The fixture results must remain labeled `synthetic_fixture`. They demonstrate how missing annotations and order shifts are counted, not the magnitude or direction of any real judge effect.

## Resumption gate

Before any original R1 order rerun, obtain authorized exact drafts, packets, original judge prompt and rubric, and contemporaneous model/settings records. If any item is unavailable, report it as missing and pause the matched comparison. Freeze both order arms and analysis before opening the allocation key. Keep source adjudication or any revised rubric in a separate run ID. No human evidence or product efficacy follows from this model-only protocol.

See [`protocol-v1.json`](protocol-v1.json) for the full machine-readable gate. The issue remains open until an eligible model run and its case-level report exist.
