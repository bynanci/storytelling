# SK-E02 · Evidence plans and packet errata

English · [繁體中文](README.zh-TW.md) · [Parent issue #115](https://github.com/bynanci/storytelling/issues/115) · [Issue #119](https://github.com/bynanci/storytelling/issues/119)

This package freezes evaluation evidence boundaries before draft generation. It is evaluation documentation and offline tooling; it adds no Rust, CLI or product schema and creates no model or participant result.

## Contract

Every run selects one `evidence_mode` string:

- `packet_only`: judges may use only the frozen packet bytes. A draft can violate packet scope even when a detail is true in the world. Do not convert that into a fabrication finding.
- `packet_plus_archived_sources`: judges may use the frozen packet and the identified archived passages. A source that cannot be accessed is `unavailable`; the failure does not show that a claim is false.

Before either variant is generated, freeze the packet ID, revision and exact-byte SHA-256; the evidence mode; claim IDs and neutral proposition summaries; each claim's required/optional status; source IDs and revisions; and packet/source support locations. Record the UTC freeze time. The first generation timestamp, when known, must be later. Never fill an unknown historical value after seeing a draft or score.

The claim register is a plan, not a verdict. During adjudication, each checkable claim uses the W4 support states `supported`, `contradicted`, `insufficient` and `unavailable`. Two independent annotation arrays record `packet_scope_violations` and `packet_source_mismatches`. Both may apply to a claim. These are evaluation record fields, not new Rust enums.

## Versioned files

| File | Purpose |
| --- | --- |
| `schemas/evidence-plan-v1.schema.json` | One evidence mode, packet/source identity, frozen claim register and generation ordering. |
| `schemas/adjudication-v1.schema.json` | W4 claim support states plus distinct scope and packet/source error annotations. |
| `schemas/packet-errata-v1.schema.json` | Versioned correction fragments with fidelity and source-access provenance. |
| `fixtures/prospective-evidence-plan.json` | Synthetic plan with one archived source passage and one unavailable source. |
| `fixtures/synthetic-adjudication.json` | Synthetic examples of all four W4 states and the two separate annotation axes. |
| `fixtures/r1-packet-errata.json` | C13/C28 historical corrections without editing R1 inputs or results. |
| `fixtures/negative/*.json` | Paired invalid examples for multi-mode selection, generation before freeze, unavailable-as-supported/contradicted and R1 score rewriting. |

The schemas are evaluation artifacts only. They do not change the Rust wire contract or existing CLI commands.

## C13 and C28 errata

C13's cited Sports Administration page was fetched on 2026-11-13. The exact HTTP response bytes had SHA-256 `de297b15ff73640af20142d8223576c5320d0d08fbc4b1aeb2e61c0a372b99c7` and were captured transiently, not retained in the repository or a durable archive. The page supports the corrected name `鄭博隅`; the public record contains only the short correction, citation, response digest and capture metadata. The R1 source revision and exact original packet remain unknown.

C28's public R1 audit says to preserve “at least 12 new state legislative chambers.” A fresh request to the cited syndicated-source URL returned HTTP 403 at 2026-11-13T07:00:36Z. No source digest or archived passage is claimed for that response. The public R1 audit supplies the correction wording, but the exact R1 packet bytes are unavailable. Mark direct source support `unavailable`; do not infer contradiction or fabrication from the 403.

The errata are a proposed next packet revision. They do not rewrite R1 packets, drafts, ratings, assignments, error flags or conclusions. The historical claim register and exact generation-time freeze were not recorded, so this package does not backfill them or present the historical errata as a new evaluation.

## Source snapshots, hashes and redistribution

For a future `packet_plus_archived_sources` run:

1. Capture the exact response bytes before decoding or newline conversion. Record the citation URL, retrieval time in UTC, HTTP status, content type, source revision identifier (publisher version, date, DOI or equivalent), and SHA-256 of those exact bytes.
2. Retain a private or otherwise authorized immutable archive. Record a non-sensitive archive reference and precise passage location. Do not put private paths, signed URLs or credentials into the public record.
3. Freeze the packet bytes and claim register before either variant is drafted. Bind the run to the plan by its exact-byte hash.
4. If retrieval fails, record the attempt and status, leave passage/hash/revision unknown as appropriate, and return `unavailable`. Never hash an access-denial page as though it were the cited article.
5. Publish citations, brief authored summaries and the minimum correction text needed to identify an erratum. Do not publish full copyrighted articles or scans unless their license permits it. A hash verifies bytes only when those bytes are retained somewhere a reviewer is authorized to inspect.

For `packet_only`, store no archived source passages in the allowed evidence set. Citations printed inside the packet do not silently expand the judging mode.

## Validation

```sh
python3 scripts/test_sk_e02_contract.py
python3 scripts/sk_e02_contract.py check
python3 scripts/check_repo.py
git diff --check
```

The standard-library regression checks strict fields, one mode, freeze-before-generation ordering, claim/source/revision/location references, unavailable-source behavior, separate annotation axes, versioned C13/C28 errata, valid and invalid fixture bytes, and protection against editing R1 results. `.gitattributes` pins the exact-byte SK-E02 package files to LF; a clean-checkout regression exercises `core.autocrlf=false`, `input` and `true`. The current local environment also validated all three Draft 2020-12 schemas and valid/invalid shape examples with `jsonschema` 4.26.0; ordering and cross-record constraints are checked by the dedicated validator because JSON Schema cannot compare timestamp values or resolve IDs across records. CI does not install the optional library and runs the independent standard-library contract tests instead.

See [the verification record](../../../validation/sk-e02-evidence-contract.md) for base/head, RED/GREEN, tests and direct source-access limits.
