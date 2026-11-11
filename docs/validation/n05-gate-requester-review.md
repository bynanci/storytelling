<!-- Historical gate-case record preserved from PR head 67e01469afd6bbe5b34daf47e00c01d1a6549c2d. The product pilot has a separate pending author review. -->
<!-- SPDX-License-Identifier: MIT -->
# N-05 commercial-copy CLI slice

Issue #52 / N-R05, SDLC G3–G4. Automated evidence and human semantic review are tracked separately.

## Reproduce

Run from the repository root:

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-copy-v1.json tests/fixtures/narrative-v1/n04-revision-history-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-copy-revised-ref-v1.json tests/fixtures/narrative-v1/n04-revision-history-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-copy-dangling-ref-v1.json tests/fixtures/narrative-v1/n04-revision-history-v1.json
```

Each successful command writes one JSON line and exits 0. Invalid shape, revision or ledger identity writes one JSON error line and exits 1; bad usage exits 2; file I/O errors exit 3. The command reads but does not change either input. `--help` lists `audit-copy-claims`.

## Automated acceptance

| Check | Evidence |
| --- | --- |
| Audience, factual copy, missing factual claim, creative slogan and CTA | `outlines_audience_claims_wording_and_call_to_action_with_reference_only_status` |
| Exact cited text is shown for review | Same CLI integration test |
| Old S02@1 resolves to its historical text and reports current revision 2 | `labels_a_claim_against_an_older_source_revision_as_revised` |
| Absent S99@1 is dangling | `labels_a_missing_source_id_as_dangling_without_claiming_support` |
| Unknown version/field, duplicate key and mismatched ledger are rejected | `rejects_unknown_versions_fields_duplicate_fields_and_mismatched_ledgers` |
| Repeat output is byte-identical and both inputs remain byte-identical | `repeated_file_run_is_identical_and_does_not_modify_input_files` |

Run these regressions with the existing video V1, narrative and OTIO tests. N-05 reference statuses describe identity and revision only. The engine does not decide that cited text supports the wording or that a factual claim is true.

The release CLI returned exit 0 for each valid fixture. The ordinary case
reported `gate-test-result` as `current` and included the supplied S03@1 text,
reported `unsupported-program-claim` as `missing`, and kept the slogan and CTA
as `creative_proposal` / `not_applicable`. The stale case reported S02@1 as
`revised`, supplied the old wording and set `current_revision` to 2. The absent
S99@1 case reported `dangling` with null cited text and no current revision.
These are the actual output values produced by the CLI; the raw output is one
JSON line per invocation.

## Requester semantic assessment · 2026-11-11

Reviewer code `requester-2026-11-11` read the actual report and S03@1 in the
conversation and supplied the judgments below. The submitted assessment is
[recorded on Issue #52](https://github.com/bynanci/storytelling/issues/52#issuecomment-6792555292).
This is requester review of the sample, not an independent participant study,
timed trial, release approval or a semantic judgment made by the validator.

| Claim | Human support decision | Suggested correction |
| --- | --- | --- |
| `gate-test-result` / S03@1 | Supported; the requester found the wording consistent with the single observed test. | Retain the one-test qualification. |
| `unsupported-program-claim` / no source | Unsupported; the requester recommended deletion or rewriting. | Delete from the reviewed draft. The suggested general rewrite still claimed cost savings without a source and was not used. |
| `neighbor-slogan` / creative proposal | The data tag is correct; make the reader-facing proposal framing explicit. | Retain `creative_proposal` and prefix the wording with `[Proposed Slogan]`. |
| `cta-repair-day` / creative proposal | The data tag is correct; make the reader-facing proposal framing explicit. | Retain `creative_proposal` and prefix the wording with `[Proposed CTA]`. |

### Revised draft and normal CLI replay

The [reviewed fixture](../../tests/fixtures/commercial-copy-v1/n05-copy-reviewed-v1.json)
advances the copy revision to 2, removes the unsupported program claim and
labels both proposals. The original fixture remains unchanged so the missing
evidence case continues to be testable. The evidence ledger is unchanged.

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-copy-claims tests/fixtures/commercial-copy-v1/n05-copy-reviewed-v1.json tests/fixtures/narrative-v1/n04-revision-history-v1.json
```

The actual [JSON output](n05-commercial-copy-reviewed.stdout) exits 0. A
[replay receipt](n05-commercial-copy-reviewed-replay.json) records two identical
runs, the exit statuses and unchanged input hashes. Its author-facing copy is:

> In one test, the gate closed without scraping the ground.
>
> [Proposed Slogan] A hinge, a helping hand, a neighbor.
>
> [Proposed CTA] Ask your neighbors about a repair day.

These edits answer the recorded feedback. A second reading of the revised draft
was not reported. N-01/N-07 completion, timing and attribution measurements remain
unobserved.

The requester-review continuation reran formatting, Clippy with warnings denied,
workspace debug and release tests, rustdoc with warnings denied, repository
checks and skill distribution. All passed. The five commercial-copy integration
tests passed, and the corrected fixture passed the normal CLI with identical
repeat bytes and unchanged source inputs. Hosted results for the final submitted
head are recorded on PR #83; earlier hosted results below remain historical.

## RED / GREEN / hosted CI

Test-first RED was observed on the test-only head `ec91544674697049a49f360243f392a86e4d282d`, run #276 (`42981887975`). Windows, Linux and macOS each built the workspace, then all four new integration tests failed because `audit-copy-claims` did not yet exist. The Quality job stopped at the format gate on that test-only commit. Dependency Audit passed.

The implementation was first checked locally with the new CLI tests (5/5).
Rust 1.89.0 formatting, full workspace debug and release tests, Clippy with
warnings denied, rustdoc with warnings denied, `scripts/check_repo.py` and
`scripts/test_skill_distribution.py` all passed. The full workspace suite also
reran video V1 and the existing OTIO CLI regression tests.

Hosted run #284 on `a913209be9e10f9d529b02b70b15ba317422e111` failed at the
repository-link check because the initial docs included unrelated links to
unmerged N-12 and author-study files. Those out-of-scope references were
removed. Corrected exact-head run [#285](https://github.com/bynanci/storytelling/actions/runs/42987880590)
passed all six configured jobs on `0ffca71dbf31cd3182df2d36834e641c9b3d5201`:
Quality and contracts, Dependency audit, Linux, Windows, macOS and Required
gates. The branch was reconciled with main `c1f164354c3e88d90acb32075828954091c149d8`, retaining its task-index updates. Exact-head run [#315](https://github.com/bynanci/storytelling/actions/runs/42995331966) passed all six configured jobs on `406db25514449254df1cef286846699e652352a1`.
