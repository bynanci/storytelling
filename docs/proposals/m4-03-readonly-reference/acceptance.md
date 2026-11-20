<!-- SPDX-License-Identifier: MIT -->
# M4-03 read-only reference app · prospective acceptance

[Issue #109](https://github.com/bynanci/storytelling/issues/109) · Baseline `04d2ac6f5cdb43d16cc7e4c94a2ad14fad49974d`.

These cases specify observable behavior for a future app. The fixtures and
[SDK integration tests](../../../crates/storytelling-cli/tests/consumer_sdk.rs)
supply domain expectations. App tests and author observations are pending.

Run each case through the proposed app entry point in a temporary workspace.
Use the public [`consumer_v1` facade](../../SDK_CONSUMER_V1.md). Treat the
fixture groups as separate declared snapshots. Narrative, coverage and planner
fixtures may reuse IDs without establishing any cross-fixture linkage.

| Case | Input or action | Pass condition |
| --- | --- | --- |
| RO-01 · Sources and revisions | Open the [reviewed narrative](../../../tests/fixtures/narrative-v1/n03-author-reviewed-v1.json), then separately inspect the [revision ledger](../../../tests/fixtures/narrative-v1/n04-revision-history-v1.json). Select `shared-observation-change` and `b-only-detail-change`. | The narrative shows `door-repair@2` and provenance such as `S02@1` (`supplied_observation`). The shared event lists `oral`, `illustration-wide` and `illustration-close`. The B-only event lists `illustration-close` alone. Its latest status is `needs_review` with stale `C01@1`. Preserve the declared creative proposal and author-decision distinctions. |
| RO-02 · Declared coverage | Compare [complete](../../../tests/fixtures/narrative-video-coverage-v1/valid-complete.json) and [unresolved](../../../tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json) coverage. | Both show `S02@1` then `S03@1`. The complete input can show declared planning readiness. The unresolved input shows the original reason for `S03@1`, `video_coverage.unresolved_required` at `$.mappings[1]`, and preparation from that coverage remains blocked. Both views show media verification as unavailable. |
| RO-03 · Malformed and incompatible inputs | Open coverage with a [duplicate field](../../../tests/fixtures/narrative-video-coverage-v1/invalid-duplicate-field.raw) or [unsupported version](../../../tests/fixtures/narrative-video-coverage-v1/invalid-version.json). Open a [narrative with a dangling reference](../../../tests/fixtures/narrative-v1/invalid-dangling-reference.json). | Keep backend error code and path (`video_coverage.duplicate_field` at `$`, `video_coverage.unsupported_version` at `$.schema_version`, and `narrative.dangling_reference` at `$.variants[0].beats[1].unit_ref.unit_id`). Distinguish a rejected input from an accepted unresolved result. A failed replacement cannot leave the previous result active. |
| RO-04 · Proposal and conflict | Inspect the [valid planner case](../../../tests/fixtures/planner-v1/valid.json) and then the [conflicting](../../../tests/fixtures/planner-v1/conflicting.json) case. | The proposal retains `detail` then `opening`, exact windows `[1/10, 3/10)` and `[0/1, 1/10)`, and declared total `3/10` s. A conflict names `planner.brief_conflict` and required/excluded asset `opening`. The conflicting input yields no proposal for review. |
| RO-05 · Review identity and freshness | Use the [frozen proposal response](../../../tests/fixtures/consumer-sdk-v1/proposal-response.json), [composition snapshot](../../../tests/fixtures/openfilm/finite-v1/video-trim.json) and [review context](../../../tests/fixtures/story-review-v1/context.json). Prepare a review, check its bundle, then check that same bundle after each independent change: set `target_revision` to `read-only-next` in a copy of the context, or append one LF to a copy of the selected proposal. | Preserve original proposal bytes even when the parsed value agrees. Show binding, plan and artifact identities from the prepared handle alongside the declared host profile. Compare the artifacts with the [SDK baseline](../../validation/m4-01-readonly-sdk/baseline.json). The unchanged bundle checks `current:true`. The changed context produces `current:false` with `context.target_revision` stale. Show both supplied target revisions with their input roles. The changed proposal independently produces `current:false` with `artifact` stale. Review freshness cannot enable Execute. |
| RO-06 · Rejected review | In the RO-05 bundle, change the `proposal-A` title in the embedded OTIO to `Proposal-A` as in SDK-G07. In another copy change the snapshot asset's `contentHash` to 64 `c` characters. | The tampered bundle is rejected with `story_review.artifact_digest_mismatch` at `$.otio`. The incompatible snapshot is rejected with `story_review.asset_mismatch` at `$.planning.plan.assets[0]`. Neither can be shown as a current review. |
| RO-07 · Privacy and controls | Load fixtures from a private test directory with a distinctive synthetic path. Exercise successful selection and a failed local open using the distinctive path, and inspect persistent data, diagnostics and shareable output. | Preserve source bytes and the directory contents. Default UI and shareable output omit the distinctive path, raw contents and the snapshot’s asset URI. Display declared host fields with their evidence status. Execute is unavailable throughout with a visible explanation. An app restart has no applied edit, receipt or recovery state to present. |

For traceability, M4-R03.1 maps to RO-01–03, M4-R03.2 to RO-04–06, M4-R03.3 to RO-02/05/07, M4-R03.4 to RO-03/07, and M4-R03.5 to the full replay and author observation. Add a delayed-check test at G2 if the chosen UI can overlap requests.

Before implementation, choose a real app entry point and link each case to its
future test. The first implementation should record the failing assertion
before the behavior change, then verify the same assertion on the submitted
commit. Passing tests in the Rust facade remain baseline evidence for the app.

## Baseline replay · 2026-11-20

A local Linux build of the existing CLI used Rust 1.89.0 and `cargo build
--release --locked --offline -p storytelling-cli` in a fresh target directory.
The Rustup proxy exited during setup on this host. Invoking the installed pinned
Cargo and Rust binaries directly completed the same build. The replay made 17
calls against 12 committed fixture paths and temporary copies. These were
existing CLI operations, with no Reference App present.

| Calls | Observed result |
| --- | --- |
| `render-narrative` and `query-narrative-impact` | Baseline narrative and both change IDs displayed. The two events named three variants and one variant respectively. |
| `render-video-coverage`, `render-narrative`, `plan-story` | The fixture outcomes matched RO-02–04, including rejected inputs and `unresolved_required`. |
| `prepare-story-review` and `export-story-otio` | Stdout matched the existing SDK baseline bundle and OTIO bytes, including the CLI's final LF. |
| `check-story-review` | Original inputs gave `current:true`. The two changes independently reported `context.target_revision` and `artifact` as stale. The two rejected inputs returned the RO-06 codes and paths. |

The selected fixture paths and prior receipts matched Git after the replay.
`cargo test --release --locked --offline -p storytelling-cli --test consumer_sdk`
passed all 14 existing SDK tests.
Presentation, delayed results, privacy checks at the app boundary and author
observations remain prospective.

## Future author observation

After the app cases pass, ask one independent author to inspect the public
fixtures, identify the missing unit, read its source revision and reason, and
explain which review became stale and why Execute is unavailable. Give the
author task instructions and starting inputs. Keep the expected answers with
the facilitator until the author answers. Capture an anonymous participant code,
prior exposure, tested build, completion or withdrawal, elapsed time including
waiting, assistance, and exact wording before feedback. Keep consent and the
private record outside Git. Record misunderstanding of readiness, verification
or authority as an observed failure. One observation can identify friction and
does not establish population rates or replace the #112 Beta.

The original #109 criteria remain open. Single writer, authority, durable
receipts, readback, restart recovery, and the normal/lost-reply demo require the
separate [M3 qualification](https://github.com/bynanci/storytelling/issues/106).
Choosing a TypeScript client requires the [#108 binding](https://github.com/bynanci/storytelling/issues/108).
