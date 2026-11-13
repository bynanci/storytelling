# SK-E01 validation record

Task: [#118](https://github.com/bynanci/storytelling/issues/118) · W6 / ST-R12–ST-R13
Base tree: `8029e403da8f4657be71fdfd93389ddcc70a0866` (PR #123's merged tree; main commit `29a6361c40dcbf9833ac1a55201293e52710121f`).
Scope: offline evaluation records and blind-packet preparation only. No model call, participant session, Rust/CLI schema or change to R1 scores, flags, declared groups or conclusions.

## RED / GREEN

Before adding the record helper, `python3 scripts/test_skill_evaluation_records.py` failed with `ModuleNotFoundError: No module named 'skill_evaluation_records'`. On the first implementation, the allocation-leak regression observed `unknown_field` instead of the required `allocation_leak`; the validator was changed to identify private fields before applying the closed packet schema.

The final local suite covers raw-byte hash mismatch, duplicate neutral IDs, mismatched allocation IDs/order, unsafe paths, private condition and score leakage, R1 unknown provenance, closed schemas, separate output locations, and symlink rejection in skill-tree hashing.

A focused regression initially failed because a symlinked directory was omitted by the file-only traversal. The hash routine now checks every traversed entry before selecting regular files; the regression passes and the documented symlink rejection includes both file and directory links.

Review-driven regressions also reproduced two additional issues before correction: `core.autocrlf=true` changed the prompt fixture bytes in a clean clone and caused `hash_mismatch`; `Path` ordering disagreed with the documented relative POSIX path order for `a/x` and `a.txt`. The fixture subtree is now pinned to LF in `.gitattributes`, the clean-clone regression checks `false`, `input`, and `true`, and tree hashing sorts the relative POSIX strings.

```text
python3 scripts/test_skill_evaluation_records.py
Ran 18 tests — OK

python3 scripts/skill_evaluation_records.py verify docs/research/evaluations/sk-e01/fixtures/synthetic-valid
OK: sk-e01-synthetic-gate-pair (2 generations)

mkdir -p /tmp/sk-e01-final-judge /tmp/sk-e01-final-private
python3 scripts/skill_evaluation_records.py prepare-blind docs/research/evaluations/sk-e01/fixtures/synthetic-valid \
  --judge-out /tmp/sk-e01-final-judge/packet.json \
  --key-out /tmp/sk-e01-final-private/allocation.json --seed 29
OK: 1 cases prepared in the requested separate output directories.
The presentation seed is stored only in the private allocation key.

python3 scripts/skill_evaluation_records.py verify-blind /tmp/sk-e01-final-judge/packet.json
OK: packet contains no condition or score metadata
python3 scripts/skill_evaluation_records.py verify-allocation docs/research/evaluations/sk-e01/fixtures/synthetic-valid /tmp/sk-e01-final-private/allocation.json
OK: 2 private mappings match the record
```

The seed above is only for this synthetic regression; real studies omit `--seed` and keep the generated seed in the private key. The example is explicitly synthetic, and model/runtime fields are `null` except for the fixture explanation. It is not a model comparison or human observation. The emitted packet and allocation key both validate against their Draft 2020-12 schemas; the fixture record also validates against the record schema (`jsonschema` 4.26.0).

## R1 integrity

The R1 package and `MANIFEST.sha256` are not edited by this work. `r1-provenance-status.json` keeps per-draft condition, prompt/packet bytes and hashes, model/runtime/settings, skill commit/hash and generation timestamp explicitly `unknown`. Published group assignments remain declared; no style-based relabeling was performed. The R1 analysis, score, flag, source-audit and conclusion files remain untouched.

Hosted CI, Codex review and final head evidence are recorded in the linked PR after those checks finish. Passing offline tests do not establish that future model calls used the recorded conditions, that a human judge remained blinded, or that the skill improves outcomes.

The fixture record, generated blind packet and generated key were also validated locally against all three Draft 2020-12 JSON Schemas with `jsonschema` 4.26.0. Existing workspace checks passed on the same source tree: Rust 1.89 formatting, Clippy, debug/release workspace tests and rustdoc. The one real FilmCraft integration test remains ignored because it requires the pinned host; this task makes no host claim. After PR #123 was merged, main commit `29a6361c40dcbf9833ac1a55201293e52710121f` passed all six hosted jobs in [run 43374785340](https://github.com/bynanci/storytelling/actions/runs/43374785340).
