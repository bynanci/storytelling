<!-- SPDX-License-Identifier: MIT -->
# ADR 0010: Opt-in commercial-copy report V2

- Status: Proposed for N-05 continuation review
- Requirement: N-R05 / Issue #52
- Base: main `ad798e2a888850b4ac15e99ad65633cb67e51fe6`, after PR #83 and the N-06 documentation merge

The accepted commercial-copy report V1 is closed and its default CLI output has existing replay evidence. The product trial needs historical/current source text side by side, explicit reference-attention IDs and a marker that every factual claim still requires a human support judgment. Adding these fields directly to V1 would break closed-schema consumers.

Add independent ReportV2 DTOs and a V2 JSON Schema. An explicit trailing `--report-v2` option selects V2; default `audit-copy-claims` retains V1 shape and bytes. Input CommercialCopyDocumentV1 and NarrativeRevisionLedgerV1 remain unchanged. Core defines both reports; the pure engine builds reference data and projects the selected output; CLI selects the version and handles bounded I/O. No remote model or host is introduced.

V2 adds `current_source_text`, `reference_attention_claim_ids` and `support_status`. A current citation never establishes semantic support. Factual claims always require human review. Creative proposals do not acquire factual or author-approved status from their references.

Each output version checks its own serialized 1 MiB limit. V1 cannot inherit V2's larger serialized size and reject a formerly valid V1 report. Real CLI tests cover exact default V1 bytes and an input whose V1 fits while V2 exceeds the limit, alongside four product citation cases. Invalid options retain `cli.usage`; existing input errors retain their codes.

Alternatives: mutate V1 (breaks its closed schema), change the default to V2 (breaks existing callers), or omit the added information (requires separate ledger inspection). Opt-in V2 preserves existing callers while exposing the comparison requested by the product example. Rollback: stop selecting `--report-v2`; no input migration is needed.

This is a technical versioning decision for review. Product-case author assessment remains blank and independent of reference validation.
