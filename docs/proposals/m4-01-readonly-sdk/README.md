<!-- SPDX-License-Identifier: MIT -->
# M4-01 · Read-only consumer SDK design

English · [繁體中文](README.zh-TW.md) · [Issue #107](https://github.com/bynanci/storytelling/issues/107) · [ADR 0013](../../adr/0013-read-only-consumer-facade-v1.md)

**State: proposed G2 design only.** No facade module, new SDK DTO, accepted
public API or golden-equivalence result is delivered here. The starting source
snapshot is `52dc6c315f7b3d20693f3c51e052509997ab791b` (2026-11-14 UTC).
Issue #107 is Open with no acceptance comment at this audit. It permits this
read-only design before #106; execution-capable APIs remain blocked. PR #117
has since been merged as proposal documents at `a23a2909cf3734d207c66b372f29a85c5e541c86`;
that delivery supplies no #103 G2/W01 acceptance or production authority.

## What a consumer would import

The candidate is one `consumer_v1` public module in the existing engine crate,
with eight synchronous operations: plan validation, narrative rendering,
revision history, impact query, video coverage, planning, selected-proposal
review preparation and review checking. Each delegates once to the existing
pure engine function and returns its existing typed domain result. There is no
fifth crate, second validator, generic RPC dispatcher or CLI prose parser.

This is a design sketch, **not a runnable example at this commit**:

```rust,ignore
use storytelling_engine::consumer_v1;

// Supplied by the caller; the facade never opens a path.
let coverage = consumer_v1::video_coverage(&coverage_bytes)?;
if !coverage.ready_for_planning {
    // Preserve unresolved rows and their diagnostics for the author.
}
let proposal = consumer_v1::plan_story(&planner_input_bytes)?;
// Review preparation receives the existing planner-response bytes, snapshot
// and context. It does not accept a caller-constructed trusted review object.
let prepared = consumer_v1::prepare_story_review(
    &proposal_response_bytes, &snapshot_bytes, &context_bytes,
)?;
let exact_bundle = prepared.bundle_json();
```

The domain result is not a new capability token. A stale review remains an
`Ok` freshness result with `current=false`; unresolved coverage remains `Ok`
with `ready_for_planning=false`. Signature validity, actual source bytes,
author content approval, current authority and host readback need their own
evidence. The facade exposes no execute/approve/recover method or host handle.

## Existing workflow to inspect now

These commands already exist and can be run without a model or editor:

```sh
cargo run --quiet --locked -p storytelling-cli -- render-narrative \
  tests/fixtures/narrative-v1/n03-author-reviewed-v1.json
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact \
  tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --quiet --locked -p storytelling-cli -- render-video-coverage \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
cargo run --quiet --locked -p storytelling-cli -- plan-story \
  tests/fixtures/planner-v1/valid.json
```

These are inspection steps, not newly observed facade results. Exact current
CLI contracts and exits remain in [CONTRACTS.md](../../CONTRACTS.md). The facade
would receive the same in-memory bytes after the consumer's explicit I/O;
CLI usage/I/O errors do not become engine validation errors.

## Decisions to review

| Question | Candidate / remaining decision |
| --- | --- |
| Import/package boundary | Existing `storytelling_engine::consumer_v1`, four existing workspace crates. No package publication or stable release yet. |
| Result ownership | Re-export existing accepted result types; opaque existing prepared review exposes immutable record/bundle/artifact bytes only. |
| Error access | Proposed opaque `SdkErrorV1` preserves operation ID, exact code, optional path and full unchanged source-error details. Propose a fallible detail accessor and fixed `sdk.error_details_encoding_failed` adapter diagnostic; preserve the original error and decide this at G2. |
| Byte/version limits | Existing operation-specific bounds, validation order and unsupported-version/profile errors pass through. No normalization of bytes that participate in hashes. |
| CLI parity | Preserve exact existing stdout/exits. Compare typed narrative/history fields and existing text goldens without adding a JSON schema or parsing prose. |
| Concurrency | Pure synchronous calls, immutable caller inputs, owned results; planned Send/Sync and deterministic caller-thread replay tests. Callers bound concurrency/memory. |
| Execution/recovery | Absent. #103–#106 acceptance and separately reviewed capability design are prerequisites for any later mutation surface. |

Review the [golden test matrix](test-plan.md) and [existing anchors](golden-plan.json).
It names expected assertions, not passed tests. Implementation requires a
focused test-first PR with observed RED/GREEN, independent-consumer compile
checks and current-head Windows/Linux/macOS CI. Existing backend tests do not
prove that a future wrapper preserves their behavior.

## Compatibility, migration and evidence

The facade wraps current functions; no input schema/profile, rational clock,
P1/P2 digest framing, OTIO bytes, reviewed loss choice or existing CLI command
changes. Existing root exports remain compatible. Consumers can later migrate
imports one operation at a time after parity is verified. Breaking public
signatures/types/error accessors require a new facade major and migration
review; document deprecation before removal. Removing the new module is the
rollback. No user project, catalog, media or input-file migration is involved.

| Evidence | Current state |
| --- | --- |
| G2 facade scope/API/version policy accepted | pending |
| Facade implementation and RED/GREEN | not implemented / not run |
| Golden CLI/SDK result equivalence | not run |
| Independent Rust/TypeScript consumer operation | not observed |
| Host/current approval/recovery | unavailable here; #103–#106 |
| G5 publication | no decision supplied |

Merging this proposal records a design deliverable. Keep #107 Open and its
implementation/acceptance boxes unchecked. #108 still needs its binding
selection; #109 retains the app and one-writer requirements; #111/#112 need
actual host/user evidence. A maintainer/reviewer can accept or revise this
candidate using the exact reviewed commit and the proposed fallible error-detail
accessor. No generic merge instruction supplies those technical decisions.
