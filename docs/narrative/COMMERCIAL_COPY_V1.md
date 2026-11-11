<!-- SPDX-License-Identifier: MIT -->
# CommercialCopyDocumentV1

`CommercialCopyDocumentV1` records proposed copy and the source revisions its factual claims cite. It is independent of video assets. A reference does not imply that a claim is true, that its source supports the wording, or that the copy is ready to publish.

## Input shape

The root object requires `schema_version: 1`, `copy_id`, positive `copy_revision`, `evidence_document_id`, `purpose`, `audience`, a nonempty `claims` array and one `call_to_action`. Unknown fields and duplicate JSON object keys are rejected. Each claim and call to action has a unique `id`, a `kind` (`factual_claim` or `creative_proposal`), `wording` and an `evidence_refs` array. Each reference is a source `unit_id` and positive `revision` from the named `NarrativeRevisionLedgerV1`.

The input JSON Schema describes shape and basic collection limits. Runtime validation also checks duplicate keys, IDs, claim identity, duplicate references, ledger identity and aggregate limits.

| Limit | Value |
| --- | ---: |
| Copy and evidence-ledger input | 1,048,576 bytes each |
| Claims | 1–128 |
| Evidence references per claim | 0–64 |
| Evidence references across claims and CTA | 2,048 |
| Purpose and audience | 512 Unicode scalar values each |
| Wording | 8,192 Unicode scalar values per claim |
| Serialized report | 1,048,576 bytes |

## Versioned reference-only reports

The default command returns the accepted `CommercialCopyReportV1` unchanged. Append `--report-v2` to opt into the separately versioned `CommercialCopyReportV2` (`schema_version: 2`). Both read the same V1 copy and N-04 ledger inputs. Report selection does not change input schema versions.

With `--report-v2`, `audit-copy-claims` returns the audience, purpose, original claim wording and call to action. Each reference report includes the cited ID/revision, the supplied text and origin for that exact historical revision, and the current revision and `current_source_text` if the source exists. The old text is never replaced by the current text. `reference_status` describes only whether a reference is current, revised, dangling, unknown, missing or not applicable. The report-level `reference_scope` is fixed to `reference-identity-and-revision-only`.

`reference_attention_claim_ids` lists claims whose references are `missing`, `dangling`, `unknown_revision` or `revised`, in input order with the CTA last. It identifies reference problems, not every unsupported assertion. Every factual claim carries `support_status: human_review_required`, even when its references are current. Creative proposals carry `support_status: not_applicable`; their `kind` and wording remain visible whether or not they cite a source.

A factual claim without references is `missing`. A creative proposal without references is `not_applicable`; a slogan is not silently recast as an evidence-backed fact. For claims with references, any dangling reference takes precedence, followed by `unknown_revision`, `revised`, then `current`. A current reference means only that its unit ID and revision match the supplied ledger. It is not a semantic support judgment, independent verification, author approval or publication clearance. A human reviewer must compare the cited text with the proposed wording.

The ledger is caller-supplied data. The command does not verify who authored it or where its observations came from. A different ledger `document_id` is rejected. This slice does not infer evidence through `based_on`, score claim strength or suggest replacement copy.

## Fictional product example

The [pilot fixtures](../../tests/fixtures/commercial-copy-v1/README.md) supply a synthetic trial record for a fictional organizing product. Both “試行組平均整理時間由 10 分鐘降到 8 分鐘” and “所有團隊效率都提升 20%” cite the same `PILOT_LOG@1`. Both references are `current`, and both claims remain `human_review_required`. The latter generalizes beyond the trial group and changes the measured quantity from organizing time to all-team efficiency. The author must inspect that scope; the machine does not silently approve or rewrite either sentence.

When the supplied ledger revises the measured time to 9 minutes, both old references become `revised`. The report names those two claim IDs and displays the unchanged draft wording, the old 10→8 record and the current 10→9 record. The unrelated product description, proposed slogan and CTA retain their prior values.

The ReportV1 schema and default output bytes remain unchanged after PR #83. ReportV2 adds fields through an explicit opt-in; each report version enforces its own 1 MiB serialized output limit. An input whose V1 report fits can still return V1 when its larger V2 report would be rejected. Accepted narrative, revision-ledger, video and OTIO inputs remain unchanged. See [ADR 0010](../adr/0010-opt-in-commercial-copy-report-v2.md).

| Diagnostic | Meaning |
| --- | --- |
| `copy.unsupported_version` | Copy schema version is not supported. |
| `copy.unknown_field`, `copy.duplicate_field` | Closed JSON shape or unique-key requirement violated. |
| `copy.evidence_document_mismatch` | Copy names a different ledger document. |
| `copy.duplicate_claim_id`, `copy.duplicate_evidence_reference` | Duplicate identity in the copy. |
| `copy.invalid_revision`, `copy.too_many_claims`, `copy.too_many_references` | A declared bound was violated. |
| `copy.invalid_shape`, `copy.output_too_large` | Input or bounded report cannot be represented. |
| `input.invalid_json`, `input.too_large` | JSON parsing or byte limit failed. |

See the [input schema](../../contracts/commercial-copy-v1.schema.json), [V1 report schema](../../contracts/commercial-copy-report-v1.schema.json), [V2 report schema](../../contracts/commercial-copy-report-v2.schema.json), and [synthetic examples](../../tests/fixtures/commercial-copy-v1/README.md).
