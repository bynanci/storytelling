<!-- SPDX-License-Identifier: MIT -->
# Independent automated technical review

Task: Issue #52 / N-R05. Reviewed product continuation following merged PR #83 at main `1d7086da59f64ca456dc23864fb11cfe1253c389` in the isolated N-05 worktree.

Reviewer: a separate Codex agent (`n05_final_review`), with no implementation edits. This is independent automated review, not human editorial acceptance or maintainer approval.

Scope: core/engine commercial-copy types and report logic, new real-CLI integration tests, five product inputs/ledgers, four handwritten expected outputs and the report JSON Schema.

Result: no correctness blockers found.

- Historical wording and cited source text remain intact; current source text is separate.
- Attention IDs follow the declared reference statuses in deterministic claim order, with CTA last and at most 129 entries.
- Both pilot claims keep human review required even when their citation is current. No support/approval result is inferred.
- The revised ledger preserves the entire original document and appends one source-revision event. Tests assert the exact two affected IDs and unchanged product, slogan and CTA.
- All four expected reports validate against the updated JSON Schema. The ledger/copy inputs replay without mutation.
- Core/engine remain pure, limits remain enforced, and no dependency is added.

Hosted CI for the submitted head is recorded on the PR. A separate real author assessment remains pending. The author-review table is blank; this review does not accept the copy for publication.

## V1/V2 compatibility review

After PR #83 merged concurrently, the reviewer inspected the explicit V2 boundary. The accepted V1 DTO declarations and report schema match main exactly. V1 projection is bounded after removing V2-only fields; V2 is independently bounded. Only a trailing `--report-v2` selects V2, while both modes keep V1 input parsing. The reviewer found no correctness blockers. Default V1 temporarily allocates bounded V2 current-source text before discarding it; this is a nonblocking allocation cost, not an output or validation change. Exact-byte, independent-capacity and option-grammar CLI regressions cover the compatibility boundary.

## Final integration review · 2026-11-11

This follow-up is by the integrating Codex session, not a new independent reviewer or a human author. Reviewed remote head: `e988095a1fc77a4c5b4be286af33dc353e963c46`; integrated main: `99db0c03e89bec026b7e9ed4f19ffdb924888585`; merge candidate before the documentation corrections: `7fef8a95f7281afd259d363b1b6da09edc57492f`.

The ROADMAP conflicts were limited to adjacent status rows. The resolution preserves the completed gate review, the explicit N-06/N-10 deferral, the matched N-07 preparation and the research guidance from main. No runtime, schema or fixture changes were needed after the reviewed PR head.

| Finding | Disposition / priority | Before | Minimal correction / proof |
| --- | --- | --- | --- |
| F1: existing gate-review fragment links lost their target when the record moved | Accept / P2 | Both audit translations link to `n05-commercial-copy.md#revised-draft-and-normal-cli-replay`, but that heading/anchor was absent. An explicit read-back assertion failed. The repository link check passes because it does not validate this fragment. | Retain the heading at its old URL and link to the moved scoped evidence. Read-back now passes; original gate fixtures and receipts are unchanged. |
| F2: product review instructions said to keep completed Issue #52 open | Accept / P2 | The instruction conflicted with the recorded, completed gate-sample review and closed issue. | Keep the product review table pending in its own scope. Explicitly preserve the completed gate acceptance and separate #48/#54 participant evidence. Read-back confirms the stale instruction is absent. |

Fresh verification used an isolated Cargo target. An initial run against a shared target executed a stale pre-V2 CLI and returned usage errors; it is excluded as candidate evidence. Rebuilding into the isolated target passed all eight product/compatibility/option/capacity tests without any source correction. This was a local build-artifact problem, not a product failure or a weakened test.

[Final integration checks](integration-checks.json) record successful formatting, warnings-denied Clippy, workspace debug/release tests, warnings-denied rustdoc, repository checks, skill distribution/CLI checks, evaluation and N-07 packet tests, graph tests and film oracle checks. [Fresh normal CLI replays](integration-replay.json) retain commands, exits, hashes, schema checks and byte comparisons: four V2 cases match the handwritten expectations and saved stdout; default V1 matches the original accepted receipt exactly; all inputs are unchanged. Hosted CI for the final submitted head is linked from PR #87.

Verdict: ready for final-head CI and technical merge. The opt-in report is a versioned public output boundary; rollback omits `--report-v2` and needs no input migration. Product author assessment remains blank. No participant result, publication clearance, release decision or new native narrative mode is supplied.
