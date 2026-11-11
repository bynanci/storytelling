<!-- SPDX-License-Identifier: MIT -->
# N-07 independent technical review

Review date: 2026-11-11. Reviewers are separate automated agents, not participant authors or maintainers granting SDLC acceptance.

The protocol reviewer inspected the original N-01 thresholds, exact source objects, matched starting drafts, revision references, proposal scope, blank forms and actual CLI artifacts. That reviewer also authored the measurement helper; its protocol/evidence review is not claimed as independent review of its own code. Two wording findings were corrected: the facilitator answer is a completed document snapshot, not a completed ledger; warmup time is recorded but excluded from statistics.

A different reviewer inspected `run_cli.py`, its tests and the recorded artifacts independently of their author. Existing-output refusal, repository input containment, command allowlisting, expected rejection controls, timing separation, byte/hash replay and zero-human-evidence claims were consistent. One error-handling finding remained: a size-valid JSON manifest with about 20,000 nested arrays escaped as `RecursionError`.

That finding now has an observed [RED](engineering/helper-depth-red.log) and [GREEN](engineering/helper-depth-green.log). The loader normalizes the failure to `ValueError` before output-directory creation or subprocess invocation. The regression also checks that input bytes remain unchanged. The parent reviewer inspected the fix and reran all **12 tests: eight helper and four packet tests**. No substantive technical findings remain open from these reviews.

The final helper reran all eleven engineering cases, with one warmup and five measured attempts each. Expected exits, stdout/stderr replay and source/manifest/binary hashes passed. The final helper identity is recorded in [build provenance](build-provenance.json). [Local checks](local-checks.json) retain the full Rust/documentation validation and the final Python test run; exact-head hosted results belong on the PR.

These findings do not supply protocol agreement, participants, author usefulness evidence, a G2 maintainer signature, or a G5 publication decision. The product decision remains **待真人證據**.
