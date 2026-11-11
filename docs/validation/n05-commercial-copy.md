<!-- SPDX-License-Identifier: MIT -->
# N-05 / N-R05 · Fictional product copy and evidence revisions

Issue [#52](https://github.com/bynanci/storytelling/issues/52), SDLC G3–G4, following merged PR [#83](https://github.com/bynanci/storytelling/pull/83). Delivery base main: `ad798e2a888850b4ac15e99ad65633cb67e51fe6`; prior N-05 head: `41d6d4b5a10cbc3ff26678d0e8484f075f65fb4c`. No product-case author session has occurred. The author-review fields below are intentionally blank.

## Usable outline and source

The synthetic product **理序盒** provides a work-record classification checklist. Its fictional pilot covers three volunteer teams for one week, performing the same organizing task. The ledger stores these observations as `PILOT_LOG@1`, the unchanged product feature as `PRODUCT@1`, and a proposed slogan as creative `TAGLINE@1`. A recorded assistant suggestion does not establish author acceptance.

The [input copy](../../tests/fixtures/commercial-copy-v1/n05-pilot-copy-v1.json) declares its audience and purpose and includes:

| ID / kind | Original wording | Cited source |
| --- | --- | --- |
| `pilot-average-time` / factual | 試行組平均整理時間由 10 分鐘降到 8 分鐘 | PILOT_LOG@1 |
| `all-teams-efficiency` / factual | 所有團隊效率都提升 20% | PILOT_LOG@1 |
| `product-checklist` / factual | 理序盒提供工作紀錄分類清單。 | PRODUCT@1 |
| `proposed-tagline` / creative proposal | 把紀錄理順，讓討論開始。 | TAGLINE@1 / creative_proposal |
| `cta-pilot-signup` / creative proposal | 申請理序盒的小型團隊試行，先看看是否符合你的整理流程。 | No factual citation |

**Acceptance example, not a human trial result:** the first sentence describes the measured trial group. The second expands the population to all teams and changes organizing time into overall efficiency. A 20% reduction in the measured time does not establish a 20% improvement in every team's efficiency. Both sentences deliberately cite the same current record so a valid reference cannot be mistaken for adequate wording support. The system retains both sentences for author review.

## Run through the normal CLI

From the repository root, build once and run the four cases. On Windows, use `target/release/storytelling.exe` or the equivalent `cargo run` commands in the [CLI guide](../CLI_GUIDE.md#audit-copy-claims-against-a-narrative-ledger-n-05).

```sh
cargo build --release --locked -p storytelling-cli
./target/release/storytelling audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
./target/release/storytelling audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-no-reference-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
./target/release/storytelling audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-dangling-ref-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-v1.json --report-v2
./target/release/storytelling audit-copy-claims tests/fixtures/commercial-copy-v1/n05-pilot-copy-v1.json tests/fixtures/commercial-copy-v1/n05-pilot-ledger-revised-v1.json --report-v2
```

Each command outputs one JSON line and exits 0, including missing/dangling references: the audit succeeded and reports a reference problem, not publication clearance. Invalid contracts exit 1 with a diagnostic, usage errors 2, and input/output errors 3. The CLI reads explicit files or stdin and writes stdout; core/engine remain pure. No model, media, catalog or editor is invoked.

## Expected and actual outputs

The complete expected JSON reports were written before adding the new report fields. The tests compare parsed JSON values, so pretty expected files and one-line stdout can be compared without ignoring any field. Repeated actual stdout must also be byte-identical.

| Case | Machine result | Expected | Actual stdout / exit |
| --- | --- | --- | --- |
| Current | Both pilot claims `current`; attention IDs `[]`; all factual claims `human_review_required`. | [normal](../../tests/fixtures/commercial-copy-v1/expected/n05-pilot-normal.json) | [stdout](n05-pilot/normal.stdout) / [exit](n05-pilot/normal.exit) |
| No reference | Only `pilot-average-time` is `missing` and in the attention list; wording retained. | [missing](../../tests/fixtures/commercial-copy-v1/expected/n05-pilot-missing.json) | [stdout](n05-pilot/missing.stdout) / [exit](n05-pilot/missing.exit) |
| Nonexistent reference | Only `pilot-average-time` is `dangling`; declared absent ID retained, source text/revision null. | [dangling](../../tests/fixtures/commercial-copy-v1/expected/n05-pilot-dangling.json) | [stdout](n05-pilot/dangling.stdout) / [exit](n05-pilot/dangling.exit) |
| Source revised | Exactly `pilot-average-time`, `all-teams-efficiency` are `revised` and in the attention list. | [revised](../../tests/fixtures/commercial-copy-v1/expected/n05-pilot-revised.json) | [stdout](n05-pilot/revised.stdout) / [exit](n05-pilot/revised.exit) |

The revised run uses the **same copy bytes** with an N-04 ledger that appends one source revision: `PILOT_LOG@2` corrects 10→8 to 10→9 minutes. Both original sentences and `cited_source_text` for @1 remain unchanged; `current_source_text` shows @2. The product feature, slogan and CTA remain identical to the baseline. [Input hashes and replay results](n05-pilot/replay.json) record unchanged inputs and matching expected reports for every case.

`reference_attention_claim_ids` reports reference issues only. The overbroad sentence is absent from that list in the normal case because its citation is current. Its `support_status` still requires human review. Creative kind/origin remains visible even when a proposed slogan has a current reference. No support score, author confirmation or approved replacement wording is inferred.

## N-R05 traceability

The [real CLI integration tests](../../crates/storytelling-cli/tests/commercial_copy_pilot.rs) cover the product case. The [original commercial-copy tests](../../crates/storytelling-cli/tests/commercial_copy.rs) remain intact.

| Requirement | Test / evidence |
| --- | --- |
| Audience, purpose, factual claims, source refs, slogan, CTA; current does not imply support | `current_citation_does_not_certify_scoped_or_overbroad_product_wording` and complete normal report |
| Missing reference without rewriting copy | `missing_citation_identifies_the_claim_without_rewriting_its_wording` |
| Nonexistent source preserves declared ID | `dangling_citation_identifies_the_claim_and_keeps_the_declared_reference` |
| Revised source identifies exactly two affected claims and retains old text / unaffected outputs | `source_revision_flags_both_pilot_claims_and_preserves_other_copy` |
| All four complete reports, deterministic stdout, unchanged source/copy bytes | `all_four_cases_replay_identically_and_leave_every_input_unchanged` |
| Closed JSON, unknown versions/fields, duplicate keys, mismatched ledger | Existing `rejects_unknown_versions_fields_duplicate_fields_and_mismatched_ledgers` |
| Default report V1 shape and exact accepted bytes | `default_report_v1_preserves_the_reviewed_cli_receipt_bytes` |
| Independent V1/V2 output limits | `report_v2_output_limit_does_not_shrink_accepted_report_v1_capacity` |
| Explicit trailing flag and V1 input versions | `report_selection_rejects_bad_flags_and_does_not_upgrade_input_schema` |
| Accepted N-04, narrative, video V1 and OTIO compatibility | Full workspace debug/release tests and hosted quality/oracle lanes |

Core owns report DTOs; engine reuses N-04 ledger replay and organizes deterministic references; CLI retains its existing bounded reader/output path. The opt-in ReportV2 adds current text, attention IDs and human-review markers. Default ReportV1 retains its accepted shape and exact reviewed output bytes. The input contract and accepted NarrativeDocumentV1/NarrativeRevisionLedgerV1/video/OTIO contracts are unchanged. No dependency is added. Roll back by ceasing to use this new CLI command.

## RED / GREEN and review evidence

The original command's test-first RED is preserved in hosted [run #276](https://github.com/bynanci/storytelling/actions/runs/42981887975). This continuation also observed RED before engine/core edits: all five new product tests failed on absent report fields, while every fixture parsed and the CLI exited 0. The [initial field RED log](n05-pilot/tdd-red.log) retains the assertions. The final V2 compatibility tests have their own [observed RED](n05-pilot/tdd-v2-red.log), including accepted V1 byte equality and independent output limits. After implementation, the product and existing copy tests passed; see the [actual GREEN log](n05-pilot/tdd-green.log). [Local commands and results](n05-pilot/local-checks.json) record Rust 1.89 checks. The PR records CI for its final head; prior-head results are not substituted for final checks.

An independent agent reviews code, schemas and tests separately from the implementer. This is automated technical review, not human editorial acceptance. The [review record](n05-pilot/technical-review.md) reports no correctness blockers; it does not fill the author-review table.

## Author review — pending

Shortest author exercise:

1. Run the normal command. Read the audience, both sentences and the same `PILOT_LOG@1` text beside them. Decide whether the sample and measured quantity support each sentence.
2. Run the revised command. Confirm exactly two claims are marked for reference attention; inspect both old and current source texts. Decide how to revise the two sentences, without losing the original record.
3. Inspect the proposed slogan and CTA. Confirm their labels and choose, edit or decline them. Record an anonymized author label/date and per-claim decision below or on Issue #52.

| Claim | Machine checked | Human support / choice | Wording correction |
| --- | --- | --- | --- |
| `pilot-average-time` | PILOT_LOG@1 current; @2 makes it revised |  |  |
| `all-teams-efficiency` | Same reference status; no scope judgment |  |  |
| `product-checklist` | PRODUCT@1 current, unaffected by pilot edit |  |  |
| `proposed-tagline` | Creative kind/origin; no author acceptance |  |  |
| `cta-pilot-signup` | Creative proposal; factual support not applicable |  |  |

Author label: ______  Date: ______  Draft disposition: ______

No product-case author decisions, task times or usability outcome are claimed. Keep the human acceptance item on Issue #52 open until a person records the assessment.

## Earlier gate-repair requester review

The concurrent requester assessment for the earlier gate example is preserved in the [original scoped review record](n05-gate-requester-review.md) and [Issue #52 comment](https://github.com/bynanci/storytelling/issues/52#issuecomment-6792555292). The reviewed gate fixture and its original replay receipts remain unchanged. That assessment supports the gate example only; it does not fill the product pilot review table above. The new report fields add reference information without changing the reviewed draft's original wording or source.

The [current default-V1 gate replay](n05-pilot/gate-reviewed-replay.json) confirms exact byte equality with the original reviewed receipt; [current stdout](n05-pilot/gate-reviewed.stdout) retains the accepted V1 shape. The prior receipt remains historical evidence for its original head.
