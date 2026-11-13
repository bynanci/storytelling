# SK-E01 · Offline generation records and blinded packets

Issue [#118](https://github.com/bynanci/storytelling/issues/118) · W6 / ST-R12–ST-R13

This package records what actually produced each draft and prepares a separate condition-free judging packet. It uses Python's standard library only. It never calls a model, accesses a remote service, or requires Rust. The included gate-repair pair is hand-authored synthetic test data, not an evaluation result.

## Record boundary

`schemas/skill-evaluation-record-v1.schema.json` defines the private source record. Each case binds the shared task and audience. Each generation binds the exact UTF-8 prompt and evidence-packet bytes; the verifier requires identical prompt and packet hashes across the matched arms. Each generation also records:

- case, run, generation and neutral text IDs, plus exact per-generation prompt, evidence-packet and draft byte references with SHA-256;
- condition provenance: `observed`, `declared`, `unknown` or `synthetic`, with `skill_on`, `skill_off` or `unknown` kept as a separate field;
- model provider, model, version, runtime and settings, using explicit `null` when unknown;
- generation timestamp, exact draft SHA-256, and skill Git commit plus package SHA-256 when those are known.

`observed` means a contemporaneous record supports the condition label; it does not authenticate the model provider or prove organizational authorization. `declared` is an unverified entry. Use `unknown` for both status and condition when evidence is missing; never infer the condition from writing style or a published group assignment. Use an opaque evidence ID rather than a local path or participant name. Do not store credentials in model settings. Synthetic records must identify themselves as fixtures and cannot be mixed with observed results.

All referenced paths are relative to the bundle. The verifier hashes the exact file bytes without newline or Unicode normalization. Keep private prompts, media, personal details and copyrighted source text in an authorized local study folder. The public fixture contains only invented text. Do not put absolute paths, credentials or private evidence in a public record.

`.gitattributes` pins this package to LF so clean checkouts with automatic EOL conversion cannot invalidate the recorded hashes; verification still reads and hashes raw file bytes.

For a loaded skill package, record the full Git commit and `sha256` from `hash-skill-tree`. The v1 tree digest is SHA-256 over the domain prefix `storytelling-skill-tree/v1\0`, followed in sorted relative POSIX path order by each path's 8-byte big-endian length, path UTF-8 bytes, file 8-byte big-endian length and exact file bytes. Symlinks are rejected. This captures local package bytes, while the commit records repository revision.

## Blind packet and allocation key

`prepare-blind` writes two files to **different directories**:

- `skill-evaluation-blind/v1` contains the common task, audience, prompt and evidence packet, plus text under `T-` IDs followed by six neutral Crockford characters. The presentation order is shuffled.
- `skill-evaluation-allocation/v1` is private. It maps text IDs to case/run/generation IDs, condition and provenance, skill identity and presentation position. It stores the random seed so the order can be checked. It contains no scores.

The judge packet excludes condition, skill identity, run/generation IDs, hashes, random seed and all current or prior scores. The generator checks the packet against a closed field set and checks allocation IDs and order against the private record. The JSON Schemas are the versioned contract; the standard-library helper enforces its operational checks without requiring a JSON Schema package. Keep the allocation key with the facilitator until judging is recorded and frozen. Send judges only the packet; do not include the record, key, terminal output containing private paths, or earlier ratings. Blinding cannot prevent a judge from guessing from the prose itself.

Scores belong in a separate post-judging file. Never place scores in the generation record or judge packet. `verify-blind` rejects condition/allocation and score/rating fields. `verify-allocation` detects missing, extra or mismatched IDs and checks the recorded presentation order against the seed.

## Manual, offline workflow

From the repository root, validate the synthetic example:

```sh
python scripts/skill_evaluation_records.py verify docs/research/evaluations/sk-e01/fixtures/synthetic-valid
python scripts/skill_evaluation_records.py hash-skill-tree skills/storytelling
```

For an actual, authorized model-only evaluation, copy the fixture to a private study directory, replace its synthetic records with one frozen task per matched pair, and save each exact prompt, packet and draft as local UTF-8 files. Use an independent clean generation context per condition. Record `null` for unknown model/runtime fields; use `observed` only when a contemporaneous run record supports that label, and record both skill commit and tree digest for skill-on runs.

Create empty output directories outside the private bundle and keep them separate. Omit `--seed` for a random seed generated by the helper; the seed is written only to the private key. Supply `--seed` only for a synthetic replay or an intentionally reproducible packet.

```sh
mkdir -p /tmp/sk-e01-judge /tmp/sk-e01-private
python scripts/skill_evaluation_records.py verify /path/to/private-bundle
python scripts/skill_evaluation_records.py prepare-blind /path/to/private-bundle \
  --judge-out /tmp/sk-e01-judge/packet.json \
  --key-out /tmp/sk-e01-private/allocation.json
python scripts/skill_evaluation_records.py verify-blind /tmp/sk-e01-judge/packet.json
python scripts/skill_evaluation_records.py verify-allocation /path/to/private-bundle \
  /tmp/sk-e01-private/allocation.json
```

Keep outputs in access-controlled storage. Before sharing, inspect the judge packet as the recipient would see it, freeze the rubric, record ratings without the key, and reveal the key only after ratings are frozen. Actual model calls, human trials and publication are separate decisions and evidence.

## R1 preservation

[`r1-provenance-status.json`](r1-provenance-status.json) records missing per-draft generation metadata as `unknown`. It does not reconstruct conditions from prose or promote published assignments into observed provenance. The 40-case scores, flags, declared groups, source audit and conclusions remain in their existing R1 files, unchanged. R1 is not a valid complete V1 generation record.

## Checks and limits

```sh
python scripts/test_skill_evaluation_records.py
python scripts/skill_evaluation_records.py verify docs/research/evaluations/sk-e01/fixtures/synthetic-valid
```

The tests cover exact-byte hashes, duplicate/mismatched IDs, unsafe paths, allocation order, condition/score leakage and explicit R1 unknowns. They establish record-tool behavior only. They do not establish model condition truth, source quality, judge blinding in operation, human efficacy or product benefit. No human or model observations are included.
