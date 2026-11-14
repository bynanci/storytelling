# SK-E04 · Judge calibration and order-only rescore

English · [繁體中文](README.zh-TW.md) · [Parent issue #115](https://github.com/bynanci/storytelling/issues/115) · [Issue #121](https://github.com/bynanci/storytelling/issues/121)

This package prepares, but does not execute, an order-only rerun of the 40-case Gemini R1 comparison. The R1 report says all 40 presentations were X-then-Y, with 27 X preferences, 10 Y preferences, 3 ties and 325/560 ratings equal to 5. Those observations do not identify an order effect.

## Preflight result: blocked

The public R1 package contains numeric ratings and summaries. It does not contain all original X/Y draft bytes, complete source packets, exact prompts, the exact rubric, judge model revision/settings, or verbatim judge requests/responses. The original X/Y assignment keys for some early cases are also only recorded in a compacted conversation summary. The [source report](../2026-11-13-gemini-r1/README.md) documents these limits. None of these missing historical fields is reconstructed here.

Because the order-only comparison must hold text, packet, prompt, rubric and model settings constant, there is no valid original-text order rerun to perform from this repository. A new synthetic or prospective study would be a different run and cannot be described as a rescore of R1. This package adds no model or external-service call.

## Frozen plan and rubric

- [`protocol-v2.json`](protocol-v2.json) is the current prospective record contract. It supersedes the prepared, not-run [`protocol-v1.json`](protocol-v1.json) and freezes missing-call outcomes, explicit pair slots, comparable-pair denominators and per-draft score summaries. No observed run exists under either protocol.
- [`rubric-v1.md`](rubric-v1.md) gives independent anchors for factual accuracy, traceability, uncertainty, narrative coherence, clarity, audience fit and required-task coverage. It is for a separate calibration run; do not use it to replace the unknown R1 rubric in an order-only run.
- The order-only run requires at least two replicate pairs per case, with one fresh context per order in each pair. The same exact X/Y texts, packet, judge prompt, rubric, model revision and settings are used; only presentation order changes. Context execution order is randomized and recorded in a private allocation key. The judge sees neutral IDs and no R1 scores, declared assignments or key.
- Preserve raw output and annotations before adjudication. The analyzer accepts complete v1 records for compatibility, but new or resumed runs must use v2. Each scheduled order slot is recorded, including `not_attempted`; an omitted slot is a malformed record. A valid pair has `valid_annotation` on both orders. Failures and missing annotations remain in planned denominators and are excluded only from comparative metrics.
- V2 outcomes are `valid_annotation`, `refusal`, `invalid_annotation`, `timeout`, `request_error` and `not_attempted`. A refusal or invalid annotation retains an output digest and opaque artifact ID. A timeout or request error records both as present or both null. The exact outputs stay in the authorized evidence store; public records contain no private response text or local paths. A retry uses fresh contexts and a new replicate ID; it never replaces the failed slot.

The analysis helper reports planned, comparable, incomplete, failed and not-attempted pair counts; call outcomes; per-case values; raw/adjudicated annotation disagreements; and unresolved W4 source checks. For every rubric dimension, it reports X's and Y's mean absolute score change separately. Score changes and order-gap shifts are averaged within each case before across-case summaries. Cases with no comparable pair remain visible with null comparative metrics. The output is descriptive only: no significance test or population claim. Repeated calls and the seven rubric dimensions do not increase the independent case count.

`matched_preference_flip_count`, `matched_preference_pair_count` and `matched_preference_flip_rate` are secondary **pair-weighted** counts/rate, not an equal-weight case mean. Cases with more comparable replicates contribute more to that rate. Inspect each case's flip rate and comparable-pair denominator before interpreting the pooled rate; the case count remains the analysis unit. This clarification changes no calculation, field or schema.

## Input and evidence boundaries

The CLI rejects duplicate JSON members at every nesting level, non-finite JSON constants, invalid UTF-8 and invalid record discriminant types with exit code 2 and no JSON report. Complete v1 and v2 records keep their existing fields and successful output. For v2, one opaque `raw_output_ref` must bind to one SHA-256 digest throughout a record, including failed calls with retained partial output; repeating the same ID and digest is allowed. These checks establish metadata consistency only. They do not retrieve evidence-store artifacts, authenticate their bytes, prove a fresh judge context or recover omitted scheduled pairs. The facilitator must compare the frozen allocation inventory and authorized evidence bytes separately.

The [input-boundary validation record](../../../validation/sk-e04-input-boundary.md) retains actual RED/GREEN evidence. No original R1 content, declared assignment, error flag, score or conclusion changes. The original rerun and #122 holdout remain blocked; passing these synthetic tool checks supplies no new model or human result.

## Synthetic tool fixtures

`fixtures/synthetic-order-pairs-v1.json` is retained to verify legacy complete-record compatibility. [`fixtures/synthetic-order-pairs-v2.json`](fixtures/synthetic-order-pairs-v2.json) is the current synthetic regression fixture. Its four invented cases include a timeout, a refusal and a not-attempted slot. Expected tool-only denominators are 5 planned pairs, 3 comparable pairs, 2 incomplete pairs, 2 failed pairs and 1 not-attempted pair. These values test accounting only; they are not model observations or estimates of judge behavior.

Run:

```sh
python3 scripts/sk_e04_order_analysis.py analyze \
  docs/research/evaluations/sk-e04/fixtures/synthetic-order-pairs-v2.json
python3 scripts/test_sk_e04_order_analysis.py
```

The fixture result must remain labeled `synthetic_fixture`. It demonstrates missing-call accounting and per-draft score separation, not the magnitude or direction of any real judge effect.

## Resumption gate

Before any original R1 order rerun, obtain authorized exact drafts, packets, original judge prompt and rubric, and contemporaneous model/settings records. If any item is unavailable, report it as missing and pause the matched comparison. Freeze both order arms and analysis before opening the allocation key. Keep source adjudication or any revised rubric in a separate run ID. No human evidence or product efficacy follows from this model-only protocol.

See [`protocol-v2.json`](protocol-v2.json) for the current machine-readable gate. Issue #121 remains open and blocked until the required original study material is authorized and the eligible rerun and case-level report exist.
