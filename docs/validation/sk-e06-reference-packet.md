<!-- SPDX-License-Identifier: MIT -->
# SK-E06 · Reference-packet integrity evidence

Task SK-E06 / [#115](https://github.com/bynanci/storytelling/issues/115). Base and pinned skill revision: `52dc6c315f7b3d20693f3c51e052509997ab791b`. This focused addition contains [five bilingual synthetic reference/manual cases](../research/evaluations/sk-e06/README.md), a blank form, an offline standard-library checker and path-scoped CI on Linux/Windows/macOS. It changes no Skill rule, Rust/CLI behavior, accepted runtime schema or R1 file.

## Evidence represented here

The case responses are **authored reference examples**, not captured model/human outputs. No frozen case prompt was executed as an evaluation, and no instruction-conformance, narrative-quality, human-benefit or holdout outcome is supplied. The manual execution form remains blank, including host, skill identity, model availability, date, exact raw output and review fields. Independent actual runs must record those fields before using a reference key.

Machine checker execution is separate from case execution. The checker verifies identities, local exact-byte hashes, source/revision references, both languages, reference labels, blank-form completeness and literal examples only. It never uploads, renders, exports, invokes a model/editor, scores a candidate response or authenticates an author decision. Passing it cannot prove semantic compliance.

## Observed mechanical RED / GREEN

The [receipt](sk-e06-packet/red-green-receipts.json) preserves the exact no-op validator source and source hash used for the initial RED, test hash, command exit and raw-output SHA-256. RED came from an actual working-tree run of ten tests: **2 passed, 8 failed**, exit 1, because malformed packets were accepted by the no-op validator. [Selected unchanged failure lines](sk-e06-packet/mechanical-red-excerpt.txt) retain the observed assertions. No immutable commit was retained for this supplemental working-tree RED; the full log and original test/stub copies were kept locally for the integrating maintainer. No missing historical failure was reconstructed.

The final [GREEN output](sk-e06-packet/mechanical-green.txt) has **11 passed, 0 failed**, exit 0. Initial assertions are preserved. The additional strict-JSON regression rejects duplicate fields and NaN. Negative-test file writes explicitly use LF so Windows tests exercise the intended semantic/integrity rejection instead of accidentally failing on CRLF bytes. The packet's exact fixture hashes are protected by `.gitattributes`.

Commands:

```sh
python3 scripts/test_sk_e06_packet.py
python3 scripts/sk_e06_packet.py
python3 scripts/test_skill_evaluation_records.py
python3 scripts/test_sk_e02_contract.py
python3 scripts/test_sk_e03_cases.py
python3 scripts/test_sk_e04_order_analysis.py
python3 scripts/check_repo.py
git diff --check
```

New checker regressions cover reference-only provenance, version/skill pinning, checksum/missing artifacts, confined artifact paths, required patterns/locales/source revisions, forbidden capture fields, broken literal examples, prefilled or incomplete manual forms, strict JSON, and CLI integrity-only labeling. They test the checker, not a model's interpretation of the adversarial source.

The RED/GREEN source states are labeled working trees. The integrating PR must record its actual final head and read the ordinary repository CI plus the three-platform SK-E06 workflow. Rust/Cargo files are untouched; Rust/host verification is not claimed from this local Python packet work.

## Continuing and stopping

The material is ready for a manual drafting or explicitly selected host-agent exercise. The facilitator gives only prompt/sources and frozen Skill first, freezes the verbatim output, then reveals reference/invariants and records per-invariant evidence, disagreements and unexpected answers. Reference examples may use different correct wording. A missing actual result stays unknown, not zero or pass.

#115's observed-run requirement remains unfulfilled. #121 still needs authorized original drafts/packets/prompts/rubric/model records, and #122 still needs its accepted dependencies and a justified instruction revision. No candidate set is relabeled as holdout; no R1 number, grouping or flag changes. #112's participants/protocol/product dependencies and every G5 publication decision remain separate.
