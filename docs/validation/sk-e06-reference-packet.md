<!-- SPDX-License-Identifier: MIT -->
# SK-E06 · Reference-packet integrity evidence

Task SK-E06 / [#115](https://github.com/bynanci/storytelling/issues/115). Base and pinned skill revision: `52dc6c315f7b3d20693f3c51e052509997ab791b`. This focused addition contains [five bilingual synthetic reference/manual cases](../research/evaluations/sk-e06/README.md), a blank form, an offline standard-library checker and path-scoped CI on Linux/Windows/macOS. It changes no Skill rule, Rust/CLI behavior, accepted runtime schema or R1 file.

## Evidence represented here

The case responses are **authored reference examples**, not captured model/human outputs. No frozen case prompt was executed as an evaluation, and no instruction-conformance, narrative-quality, human-benefit or holdout outcome is supplied. The manual execution form remains blank, including host, skill identity, model availability, date, exact raw output and review fields. Independent actual runs must record those fields before using a reference key.

Machine checker execution is separate from case execution. The checker verifies declared identity syntax, local exact-byte artifact snapshots, source/revision references, both languages, reference labels, blank-form completeness and literal examples only. Skill commit/hash entries are declared preparation metadata; the checker does not compare actual pinned Git blobs or authenticate their origin. It never uploads, renders, exports, invokes a model/editor, scores a candidate response or authenticates an author decision. Passing it cannot prove semantic compliance.

## Observed mechanical RED / GREEN

The [receipt](sk-e06-packet/red-green-receipts.json) preserves the exact no-op validator source and source hash used for the initial RED, test hash, command exit and raw-output SHA-256. RED came from an actual working-tree run of ten tests: **2 passed, 8 failed**, exit 1, because malformed packets were accepted by the no-op validator. [Selected unchanged failure lines](sk-e06-packet/mechanical-red-excerpt.txt) retain the observed assertions. No immutable commit was retained for this supplemental working-tree RED; the full log and original test/stub copies were kept locally for the integrating maintainer. No missing historical failure was reconstructed.

The initial implementation [GREEN output](sk-e06-packet/mechanical-green.txt) has **11 passed, 0 failed**, exit 0. Initial assertions are preserved. The additional strict-JSON regression rejects duplicate fields and NaN. Negative-test file writes explicitly use LF so Windows tests exercise the intended semantic/integrity rejection instead of accidentally failing on CRLF bytes. The packet's exact fixture hashes are protected by `.gitattributes`.

## Review-fix RED / GREEN

Read-only review reproduced two mechanical defects: an artifact was read once for its hash and again for JSON validation, allowing different bytes to be validated after the hash; and a symlink-resolution cycle escaped as a traceback. Three regressions were committed first at `83b03a7ad7229e0ca2093b48b76fb5cc105f05e0`. The [review-fix receipt](sk-e06-packet/review-fix-receipts.json) records the actual RED: **11 passed, 2 failed, 1 error**, exit 1, with immutable source identity and [selected diagnostic lines](sk-e06-packet/review-fix-red-excerpt.txt).

Fix commit `7fc5dd19b129cda2b6d3e03b2dc50eed8abb443b` hashes and decodes each artifact from the same immutable byte buffer, checks symlink components before resolution, and converts resolution failures to `PacketError`. The actual [review-fix GREEN](sk-e06-packet/review-fix-green.txt) is **14 passed, 0 failed/errors/skipped**, exit 0 on Linux. On systems that cannot create symlinks, the real-symlink CLI test explicitly skips; the mocked resolution-cycle regression still runs. The original ten-test RED/eleven-test GREEN receipt above remains unchanged.

This fixes packet consistency and error handling. It does not validate a candidate model response, authenticate filesystem state or Skill provenance, or supply semantic/human/efficacy evidence. The final PR still needs current-head CI on all supported platforms.

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

The original RED/GREEN source states are labeled working trees; the separate review-fix receipts identify their immutable test/fix commits. The integrating PR must record its actual final head and read the ordinary repository CI plus the three-platform SK-E06 workflow. Rust/Cargo files are untouched; Rust/host verification is not claimed from this local Python packet work.

## Continuing and stopping

The material is ready for a manual drafting or explicitly selected host-agent exercise. The facilitator gives only prompt/sources and frozen Skill first, freezes the verbatim output, then reveals reference/invariants and records per-invariant evidence, disagreements and unexpected answers. Reference examples may use different correct wording. A missing actual result stays unknown, not zero or pass.

#115's observed-run requirement remains unfulfilled. #121 still needs authorized original drafts/packets/prompts/rubric/model records, and #122 still needs its accepted dependencies and a justified instruction revision. No candidate set is relabeled as holdout; no R1 number, grouping or flag changes. #112's participants/protocol/product dependencies and every G5 publication decision remain separate.
