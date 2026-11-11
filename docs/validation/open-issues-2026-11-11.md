<!-- SPDX-License-Identifier: MIT -->
# Open-issue audit and requester revisions · 2026-11-11

English · [繁體中文](open-issues-2026-11-11.zh-TW.md)

The initial audit used main `c1f164354c3e88d90acb32075828954091c149d8` and
found seven open issues: #48, #50, #52, #53, #54, #65 and #67. None was fully
complete merely because its implementation had merged. During this audit the
requester supplied N-03 inspection, N-05 semantic judgments and an explicit
N-06 deferral. Those responses unlock three task completions. Participant
measurements and the N-08 blind reading remain unobserved.

## Requirement disposition

| Issue | Verified result | Completion or remaining condition |
| --- | --- | --- |
| [#50 / N-03](https://github.com/bynanci/storytelling/issues/50) | The requester inspected the three replayed drafts. Document revision 2 answers the close-framing feedback and preserves the original source units, oral and wide sections. | Close when this correction and its receipts merge. The correction is a response to the inspection; a second reading is not claimed. |
| [#52 / N-05](https://github.com/bynanci/storytelling/issues/52) | Requester source-support judgments are recorded. The corrected draft deletes the unsupported program claim and labels proposals. | Completed by [PR #83](https://github.com/bynanci/storytelling/pull/83), merge `1d7086da59f64ca456dc23864fb11cfe1253c389`. |
| [#53 / N-06](https://github.com/bynanci/storytelling/issues/53) | Compatibility and smallest-change comparisons cover all three candidates. The requester explicitly deferred all three. | Completed by [PR #85](https://github.com/bynanci/storytelling/pull/85), merge `ad798e2a888850b4ac15e99ad65633cb67e51fe6`. No mode is adopted. |
| [#48 / N-01](https://github.com/bynanci/storytelling/issues/48) | Synthetic source cards, baseline drafts, measures and proposed thresholds exist. Its stale statement that #49 was blocked has been corrected against the accepted #49 waiver. | Keep open for the protocol agreement, actual paired observations and maintainer study decision. #49/#50 already have separate bounded authorization. |
| [#54 / N-07](https://github.com/bynanci/storytelling/issues/54) | N-05 is now delivered; the existing N-01 and supplemental evaluation packets can supply the pilot materials. | Keep open for real ordinary-notes/skill/native observations, denominators, revision times, attribution errors and a continue/narrow/stop decision. |
| [#65 / N-08](https://github.com/bynanci/storytelling/issues/65) | Valid order, contradiction, unknown version and same-input replay pass. A standalone label-free reader sheet is ready. | Keep open for an actual reader's answers. Split the completed deterministic-replay item from this missing human gate. |
| [#67 / N-10](https://github.com/bynanci/storytelling/issues/67) | The [explicit N-06 deferral](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714) is now available to cite. | Keep open/deferred. Resume only after an actual author need and a new decision adopt exactly one mode and version scope. |

## Requester feedback and editorial changes

The [N-03 inspection](https://github.com/bynanci/storytelling/issues/50#issuecomment-6792812347)
found the oral draft, composition contrast and provenance labels usable, but
requested clearer close framing and tool placement. The alleged hinge spelling
errors were absent from the actual bytes. The [N-03 before/after record](n03-narrative-cli.md#requester-inspection-and-corrective-draft--2026-11-11)
documents that discrepancy and the corrective C02 proposal. The original fixture
is unchanged. C02 is an assistant-authored composition proposal, not a recorded
author approval or an added observation.

The [N-05 assessment](https://github.com/bynanci/storytelling/issues/52#issuecomment-6792555292)
supports the qualified one-test statement and rejects the unsupported ten-minute
average/30% claim. The delivered edit uses deletion. A suggested generic rewrite
still asserted cost savings without a source and was not used. The slogan and
CTA retain their proposal kinds and gain explicit reader-facing prefixes. See
the [reviewed draft and normal CLI output](n05-commercial-copy.md#revised-draft-and-normal-cli-replay).

The requester selected “暫緩三者，等實際需求” for N-06. The
[comparison/decision packet](n06-format-discovery.md) records that choice separately
from the earlier recommendation and leaves unobserved task sessions blank.

These are requester-submitted editorial judgments. They provide no independent
participant count, timing, randomized comparison, blind-reading result or release
approval.

## Executed CLI evidence

The final replay uses CLI code from main
`ad798e2a888850b4ac15e99ad65633cb67e51fe6`, the existing corpus and this change's
new N-03 revision. [results.json](open-issues-2026-11-11/results.json) records all
14 normal CLI cases, actual exits, repeat-byte equality and before/after hashes
for 12 input files. Every expected exit matched; repeated outputs matched;
all inputs stayed unchanged. Successful missing/revised/dangling copy reports
exit 0 because they report reference status rather than certify the copy.

| Cases | Expected / observed result | Saved output |
| --- | --- | --- |
| N-03 original and corrected document | Exit 0; same-input bytes repeat. Oral/wide sections are unchanged in revision 2. | [original](open-issues-2026-11-11/n03-render.stdout), [corrected](open-issues-2026-11-11/n03-reviewed.stdout), [correction receipt](open-issues-2026-11-11/n03-reviewed-replay.json) |
| N-03 missing reference | Exit 1, `narrative.dangling_reference` | [diagnostic](open-issues-2026-11-11/n03-dangling.stdout) |
| N-04 shared and local corrections | Exit 0; shared change lists all three variants, local change lists close only. | [shared](open-issues-2026-11-11/n04-shared-impact.stdout), [local](open-issues-2026-11-11/n04-local-impact.stdout) |
| N-08 rendered drafts and valid rule | Exit 0 | [drafts](open-issues-2026-11-11/n08-render.stdout), [check](open-issues-2026-11-11/n08-valid.stdout) |
| N-08 contradiction / unknown version | Exit 1 with the declared stable diagnostics | [conflict](open-issues-2026-11-11/n08-conflict.stdout), [version](open-issues-2026-11-11/n08-unknown-version.stdout) |
| N-05 current/missing, revised and dangling references | Exit 0; reference-only statuses retain exact historical text. | [ordinary](open-issues-2026-11-11/n05-current.stdout), [revised](open-issues-2026-11-11/n05-revised.stdout), [dangling](open-issues-2026-11-11/n05-dangling.stdout) |
| N-05 corrected copy | Exit 0; two claims plus a proposed CTA, no unsupported program claim. | [corrected](open-issues-2026-11-11/n05-reviewed.stdout) |
| N-05 unknown version via stdin | Exit 1, `copy.unsupported_version` | [diagnostic](open-issues-2026-11-11/n05-unknown-version.stdout) |

The [15 supplemental N-05 boundary probes](open-issues-2026-11-11/n05-boundaries.json)
ran against implementation head `41d6d4b5a10cbc3ff26678d0e8484f075f65fb4c` before
the requester-record changes, which changed no Rust source. They cover empty,
128/129 claims, 64/65 references per claim, 2048/2049 aggregate references,
duplicate CTA/reference IDs, unknown revision, 512/513-character purpose,
exact/over-1-MiB input and output expansion beyond 1 MiB. All 15 passed. A Git
comparison confirms that the Rust and contract source trees at that probe head
are identical to the final replay's main code; this equivalence is in the receipt.

Reproduce the main author-facing results from the repository root:

```sh
cargo build --release --locked -p storytelling-cli
target/release/storytelling render-narrative tests/fixtures/narrative-v1/n03-author-reviewed-v1.json
target/release/storytelling audit-copy-claims tests/fixtures/commercial-copy-v1/n05-copy-reviewed-v1.json tests/fixtures/narrative-v1/n04-revision-history-v1.json
target/release/storytelling query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
target/release/storytelling check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-v1.json
target/release/storytelling check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-conflicting-v1.json
```

The last command intentionally exits 1. Each manifest entry supplies the remaining
exact arguments and any stdin. No original input, accepted narrative contract,
ledger history or runtime behavior is changed by this audit.

## Human work still required

For N-08, give a new reader only the [standalone reader sheet](open-issues-2026-11-11/reader-sheet.zh-TW.md).
The facilitator should swap A/B order across readers and record the actual order
and prior exposure. Give the source key after the initial answers. The filled
sheet must contain the reader's real words; an already exposed reader cannot be
counted as a fresh blind read. The prepared sheet has no answers.

For N-01/N-07, use the [five-author paired protocol](../narrative/N-01-AUTHOR-STUDY.md)
and [supplemental evaluation materials](../../examples/evaluation/README.md).
Record the actual prompt, host/model/commit, condition order, completion count
and denominator, seconds, attribution errors and repeated input. Keep skill and
native trials separate. The requester/maintainer owns protocol agreement and the
continue/narrow/stop decision; a facilitator owns recruitment and observations.
No participant data was supplied in this audit.

The [hosted-check snapshots](open-issues-2026-11-11/hosted-checks.json) retain the
exact reviewed heads and passing runs for PRs #83 and #85. The PR carrying this
audit supplies its own current-head CI and review record.

Local continuation checks passed: formatting, Clippy with warnings denied,
workspace debug/release tests, rustdoc with warnings denied, repository checks,
skill distribution, graph planner/trace checks and host-evidence rejection tests.
The existing pinned-FilmCraft integration test remains explicitly ignored by the
ordinary suites; no new host invocation is claimed. A directory-local Git
attribute preserves raw `.stdout` bytes, including the renderer's final blank
line, so the captured transcripts remain byte-faithful.
