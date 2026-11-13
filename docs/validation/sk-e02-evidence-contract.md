# SK-E02 validation · evidence plan and R1 packet errata

English · [繁體中文](sk-e02-evidence-contract.zh-TW.md) · [Issue #119](https://github.com/bynanci/storytelling/issues/119)

## Scope and limits

Base: `d7b41310480379c946fae97f43c6560d3aa9077f` (PR #124 merge to `main`, 2026-11-13). This change freezes an evaluation-only evidence plan and reports two R1 packet corrections. It changes no Rust/CLI schema, R1 input, output, score, group assignment, error flag or conclusion. It produces no new model or human evaluation result.

R1's original packets, full drafts, claim register, source snapshots and pre-generation freeze were not retained. The errata are explicitly retrospective reports, not repaired R1 records. The synthetic contract fixtures are not evaluation data.

## RED / GREEN

- **RED:** after writing the initial 14-test regression first, `python3 scripts/test_sk_e02_contract.py` failed with `ModuleNotFoundError: No module named 'sk_e02_contract'` (exit 1).
- **First hosted RED:** on head `7ed781a8e1c8e3f8285357ee21758d744b67f6b7`, [CI run 43385781285](https://github.com/bynanci/storytelling/actions/runs/43385781285) failed the Windows SK-E02 test because Git checked out the two SHA-bound text fixtures with CRLF. Linux, macOS and Dependency Audit passed in that run.
- **Review finding:** Codex review of the first head found that the stdlib adjudicator rejected `contradicted` after failed source access but still allowed `supported`, unlike the JSON Schema. The validator now permits only `insufficient` or `unavailable`; positive and negative regressions cover both forbidden labels.
- **GREEN candidate:** `.gitattributes` now pins the exact-byte package to LF. A clean-checkout regression exercises `core.autocrlf=false`, `input` and `true`, then verifies packet, archived-passage and plan-to-adjudication hashes. The expanded suite now has 22 tests; hosted rerun on the new PR head is required.

## Direct source-access observations

| Case | Observation | What is retained | Limit |
| --- | --- | --- | --- |
| C13 | The cited Sports Administration page returned HTTP 200 on 2026-11-13T06:54:33.868047Z. The reviewed result row used 鄭博隅. SHA-256 of the exact transient HTTP response bytes: `de297b15ff73640af20142d8223576c5320d0d08fbc4b1aeb2e61c0a372b99c7`. | Citation, short corrected name, retrieval time, content type, and exact-byte response digest. | The response was temporary in the workspace and was not retained in the repository/private archive. Historical source revision and exact R1 packet are unknown. The digest cannot replace an accessible byte archive. |
| C28 | Direct request to the cited syndicated source returned HTTP 403 at 2026-11-13T07:00:36Z. | Citation and failure status/time. No response hash is presented as an article hash. | The source is `unavailable`; no claim of direct source confirmation or contradiction. Corrected wording is reported from the published R1 audit. |

No complete copyrighted article or private source path is included. Future archived-source runs must preserve exact bytes in an authorized archive and publish only citations, hashes, locations and minimal authored summaries unless redistribution is licensed.

## Acceptance traceability

| #119 criterion | Artifact and check |
| --- | --- |
| Exactly one evidence mode | `evidence-plan-v1.schema.json`; `test_evidence_mode_is_one_frozen_string_not_a_combination`, `test_unsupported_evidence_mode_is_rejected`, and committed `invalid-multiple-evidence-modes.json`. |
| Claim/source/revision/location and requiredness frozen before variants | `prospective-evidence-plan.json`; `test_prospective_fixture_is_frozen_before_variant_generation`, `test_variant_generation_must_follow_claim_freeze`, `test_committed_invalid_freeze_example_is_rejected`, `test_claim_source_ids_and_revisions_must_resolve`. |
| Reuse W4 states; separate scope violation and packet/source mismatch | `adjudication-v1.schema.json`; `test_w4_support_states_and_scope_annotations_are_separate`, `test_failed_source_access_cannot_be_relabelled_contradicted`. |
| Versioned C13/C28 errata; preserve R1 | `r1-packet-errata.json`; `test_c13_c28_are_versioned_without_rewriting_r1_outcomes`, `test_errata_cannot_include_replacement_scores_assignments_or_flags`, `test_exact_original_packet_text_cannot_be_invented`. |
| Access failure cannot prove support or contradiction | C28 fixture, `test_failed_source_access_cannot_be_relabelled_supported_or_contradicted`, `invalid-unavailable-as-supported.json` and `invalid-unavailable-as-contradicted.json`. |
| Snapshot/hash provenance and public redistribution boundary | SK-E02 README; C13 transient response metadata; C28 403 record; no source article bytes in Git. |

The validator is standard-library only and has no dependency on product runtime code. The local environment additionally used `jsonschema` 4.26.0 to run Draft 2020-12 meta-schema and fixture validation; the hosted CI relies on the new stdlib contract regressions. No clean-room human-source archive review was performed.

## Commands

```sh
python3 scripts/test_sk_e02_contract.py
python3 scripts/sk_e02_contract.py check
python3 scripts/check_repo.py
git diff --check
```

Human acceptance and any new R2 evaluation remain separate maintainer decisions. SK-E03–SK-E05 remain independent tasks.
