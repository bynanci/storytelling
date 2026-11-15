# R1 existing-score robustness: frozen acceptance

This evaluation-only maintenance slice starts from main
`87113b9a204a801847995a334ef8728b9185b449`. It adds exact, descriptive
diagnostics over the existing R1 numeric transcription. It changes no original
scores, assignment keys, flags, conclusions, Rust API, CLI contract or schema.

Before implementation, require:

1. Four synthetic cases with seven identical dimensions remain **four** analysis
   units. Their declared difference is `1/4`; case omission ranges from `-1/3`
   to `1`, batch omission from `-1` to `3/2`, and dimension omission stays `1/4`.
2. Conditional enumeration of two uncertain early keys has four arrangements
   with bounds `[-5/4, 1/4]`; preserving the declared one-X/one-Y allocation has
   two arrangements with the same bounds for this fixture. Swapping S02's X/Y
   scores preserves the unrestricted bounds but makes the constrained bounds
   `[-3/4, -1/4]`. All-key uncertainty has symmetric
   bounds `[-5/4, 5/4]`. These are arithmetic extrema, **not confidence intervals**.
3. Existing R1 has 40 cases, 80 drafts and 560 rubric ratings. Leave-one-case,
   dimension and batch diagnostics expose their remaining-case denominators.
   Report each batch, both early-key assumption sets, and all-key uncertainty.
4. All means, bounds and comparisons use integers and `Fraction`; any decimal
   is a labeled six-place approximation. No resampling, p-values or new CIs.
5. Reject duplicate JSON members, invalid UTF-8, non-finite constants, malformed
   dimensions, booleans/non-integer/out-of-range scores, duplicate case IDs,
   malformed batches/assignments and oversized or deeply nested inputs.
6. The real evaluation-script entry point produces deterministic JSON with
   exact-byte input SHA-256 receipts. It writes no source files. A checked-in
   output must match a fresh run. Synthetic fixtures are never human/model runs.
7. Retain observed RED and GREEN output. Verify the original R1 package bytes
   and repository checks. Record scientific-package reproduction separately;
   the new script and synthetic regressions use Python's standard library only.

No new judge calls, rescoring, holdout construction, human sessions, host
operations or acceptance are authorized by this maintenance. #121's missing
original artifacts and #122's prospective gate remain unchanged. A stable
negative declared descriptive mean does not establish causal harm, and missing
treatment provenance remains missing.

The first implemented test run exposed a mistaken preimplementation hand
calculation: `[-3/4, -1/4]` had been assigned to the original synthetic fixture's
constrained enumeration. Its actual X-minus-Y differences are `+1, -2`, so a
one-X/one-Y allocation can attain both `+3` and `-3` before adding the held-fixed
sum `-2`. The first failure is retained in
[synthetic-expectation-red.txt](receipts/synthetic-expectation-red.txt). The
expectation above was corrected, and the swapped-S02 counterexample was added.
No R1 scores or analysis were changed to make the test pass.
