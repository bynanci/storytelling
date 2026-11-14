<!-- SPDX-License-Identifier: MIT -->
# ADR 0013 · Read-only consumer facade V1 (proposal)

- **Status:** proposed G2 design; no public API freeze, implementation or acceptance is supplied.
- **Task / requirement:** M4-01 / proposed M4-R01, [Issue #107](https://github.com/bynanci/storytelling/issues/107).
- **Design baseline:** `52dc6c315f7b3d20693f3c51e052509997ab791b`, checked 2026-11-14 UTC.
- **Dependency:** #107 explicitly permits read-only design before #106. Execution/recovery remains unavailable until the separate #103–#106 gates are accepted.

## Problem and independently reviewable result

An independent Rust consumer needs a documented entry point for validation,
revision impact, video coverage, planning and selected-proposal review. Current
engine functions already implement these accepted bounded contracts. Consumers
should not discover private modules, parse CLI prose or reproduce validation,
time arithmetic, planner selection or review hashing.

This decision proposes one curated, synchronous public module in the existing
`storytelling-engine` crate: `storytelling_engine::consumer_v1`. Keep the four
workspace crates and their dependency sets. No SDK crate, transport, wire
schema, host adapter or runtime is added by this design. The module is a future
implementation target; importing it is not supported at this baseline.

The [candidate API and usage guide](../proposals/m4-01-readonly-sdk/README.md),
[Traditional Chinese guide](../proposals/m4-01-readonly-sdk/README.zh-TW.md) and
[golden test plan](../proposals/m4-01-readonly-sdk/test-plan.md) specify the
G2 questions. The [machine-readable case inventory](../proposals/m4-01-readonly-sdk/golden-plan.json)
is a test plan, not an SDK result or a new runtime contract.

## Proposed surface and exact implementation reuse

All names below are **proposed**, not existing symbols. Functions accept the
same raw bytes as the named existing backend. They delegate once, without
pre-parsing, input serialization, default insertion, time normalization or
altering the accepted diagnostic order. Result names are curated aliases or
re-exports of existing accepted result types; no existing DTO is modified.

| Proposed `consumer_v1` function | Existing backend to call | Public result through the facade |
| --- | --- | --- |
| `validate_plan(bytes: &[u8])` | `validate_plan_json` | `PlanValidationV1`, alias of existing `ValidationReport` |
| `render_narrative(bytes: &[u8])` | `render_narrative` | existing `NarrativeProjectionV1` |
| `track_revisions(bytes: &[u8])` | `track_narrative_revisions` | existing `NarrativeRevisionReportV1` |
| `query_impact(bytes: &[u8], change_id: &str)` | `query_narrative_impact` | existing `NarrativeRevisionEventResultV1` |
| `video_coverage(bytes: &[u8])` | `render_narrative_video_coverage` | existing `NarrativeVideoCoverageResultV1` |
| `plan_story(bytes: &[u8])` | `plan_story` | existing `PlannedStoryV1` |
| `prepare_story_review(proposal: &[u8], snapshot: &[u8], context: &[u8])` | `prepare_story_review` | `PreparedStoryReviewV1`, alias of existing opaque `BoundStoryReviewV1` |
| `check_story_review(bundle: &[u8], proposal: &[u8], snapshot: &[u8], context: &[u8])` | `check_story_review` | existing `ReviewFreshnessReportV1` |

Each proposed signature returns `Result<the_named_result, SdkErrorV1>`.
The stable import path would expose only these operations, their returned
types and the immutable review record/reference types needed by their public
accessors. The implementation must not add an unchecked constructor or derive
`Deserialize` for `PreparedStoryReviewV1`. Its existing `record()`,
`compiled_otio()` and `bundle_json()` accessors remain immutable. A portable
`ReviewRecordV1` remains untrusted data when submitted again; holding or cloning
it does not confer authority.

Existing crate-root exports stay available for compatibility. A consumer of the
new surface imports the documented `consumer_v1` path only. No deprecation or
removal of root functions is part of this proposal. Composition-based legacy
review, direct OTIO export, cue sheets, mixed-media handoff, style, copy audits,
supplied-buffer verification and the N10-06 draft input helper retain their
existing APIs/CLI contracts; adding them to the facade needs a separately
reviewed scope. Selected-proposal review is the minimum reviewed path here.

## Responsibilities

| Layer | Responsibility |
| --- | --- |
| Core | Existing versioned domain contracts, exact clocks and private-field identity types. No facade, parser I/O or new authority constructor. |
| Engine facade | Curated public names, one delegation to an existing pure backend, an opaque error adapter and supported-version documentation. No second validator, planner, compiler or digest calculation. |
| CLI | Explicit file/stdin/output handling and existing command/exit/text/JSON formatting. It may adopt facade calls after equivalence tests; command bytes and exit semantics stay frozen. |
| Future TypeScript/client | #108 chooses and reviews its binding/transport and compatibility handshake; it does not import Rust-private DTOs or infer protocol from English prose. |
| Future reference app | #109 decides presentation and one-writer integration. A read-only result cannot enable execution by itself. |

## Error adapter and meaningful domain results

Propose an **opaque Rust-only** `SdkErrorV1` with these public read accessors:

```text
operation() -> &str       // documented operation ID, such as "video_coverage"
code() -> &str            // exact existing source diagnostic identifier
path() -> Option<&str>   // preserve null/absent paths; never replace with "$"
source_details_json() -> Result<Vec<u8>, SdkEncodingErrorV1>
                         // existing domain error object, fallible encoding
```

No public constructor, mutable bytes or generic `from_json` is proposed. The
adapter keeps the original backend error in a private representation and uses
its existing serializer when the caller requests detail bytes. Preserve all
source fields, including planner `conflicts`, narrative `relation_ids`, OTIO
`losses`, and null coverage paths. These bytes exclude CLI `{ok:..., error:...}`
envelopes and never enter an identity hash. Propose an opaque Rust-only
`SdkEncodingErrorV1` with fixed code `sdk.error_details_encoding_failed` for a
serialization failure; do not expose parser prose or return empty details. The
original `SdkErrorV1` still retains its source `code()` and `path()` so the
encoding failure cannot replace or erase the domain rejection. G2 must accept
this fallible accessor and adapter-only code before freezing the API.

The facade does not collapse results into a Boolean:

- A valid unresolved coverage document is `Ok` with
  `ready_for_planning=false` and its existing result diagnostics.
- A well-formed stale review is `Ok` with `current=false` and all stale fields.
  The CLI's exit 1 for stale review is a presentation convention, not a new SDK
  parse error.
- A rejected/conflicting planner request remains `Err` with its full typed
  conflict evidence; no partial plan is invented.
- Input/version/shape rejection keeps the original source code/path and first
  error. Caller code matches identifiers, never `Display` or English text.

The facade has no filesystem or stdout error: it receives buffers and returns
owned values. CLI/binding I/O errors remain at their actual I/O layer. The
facade does not promise that every currently accepted backend has identical
strictness; it preserves each profile's existing validation semantics. A
backend hardening change needs its own compatibility review and regression.

## Bounds and version selection

All limits are existing backend limits, not a larger combined SDK allowance:

| Input | Existing bound to retain |
| --- | --- |
| M0 plan, narrative, ledger, coverage, planner request, composition snapshot | `MAX_INPUT_BYTES = 1,048,576` per buffer |
| Source-change ID | Existing narrative ID grammar and 128-byte maximum |
| Selected planner proposal response | `MAX_STORY_PROPOSAL_BYTES = 4,194,336` |
| Review context | `MAX_REVIEW_CONTEXT_BYTES = 65,536` |
| Selected review bundle | `MAX_STORY_REVIEW_BYTES = 4,292,672` |

Collection, string, nesting, source-clock and arithmetic limits remain in each
accepted profile. For example coverage keeps depth 33 and 21 codes; narrative
keeps depth 32. Multi-buffer review checks retain backend argument-order size
rejections. No generic JSON preflight may run first and reorder those errors.

Facade major V1, document `schema_version`, planner/OTIO profile, compiler/digest
version and crate/package version are distinct. The candidate proposes a
`consumer_v1` namespace; the package remains `0.1.0-dev.0` and unpublished. A
future schema/profile must be rejected with the existing operation-specific
version/profile diagnostic. There is no auto-upgrade, unknown-field stripping,
"latest" selector or fallback to a looser operation. An absent
`consumer_v2`/execute method is a compile-time failure; #108 owns any dynamic
transport's unknown-operation negotiation.

## Concurrency, ownership and authority

Calls are synchronous, bounded and pure. The caller supplies immutable buffers;
results own their data and retain no borrow into the input. Independent calls
may run on separate caller-controlled threads. The facade has no cache,
global mutable state, background job, cancellation token or implicit parallel
work. Send/Sync assertions are planned for supported result/opaque-handle
types; no concurrency guarantee is claimed as verified before those tests run.
Memory budgets are per call; a consumer must bound concurrent calls. Dropping a
result performs no editor rollback and reports no host cancellation.

There is **no execute, approve, dispatch, retry, recover or journal-write
method**, optional privileged callback, shell/path parameter or host handle in
this surface. Compile-fail tests must preserve their absence. Future mutation
APIs require #103's accepted authority contract, #104/#105 implementation and
#106's accepted production host profile. They belong in a separately versioned
capability and cannot appear through a default feature change. A future writer
must honor project ownership/fencing and unknown-outcome recovery; immutable
read-only handles grant no writer lease.

All review flags and scopes retain their existing declaration-only values.
Mathematical signature validity, signer identity/current authorization,
compilation binding, source-byte identity, host qualification/readback and
human content approval remain separate evidence. PR #28 and the proposed
#103/PR #117 authority work are not adopted by this facade design.

## Compatibility and alternatives

Do not serialize facade metadata into a plan, review record, artifact or bundle.
Do not change a compiler/profile string, input byte slice, key order used by
existing serializers, loss choice, digest framing or P1/P2 golden hash. Reuse
compiler-owned `BoundStoryReviewV1`; never recreate its object from a portable
record plus caller-supplied OTIO. The CLI's current output formatter remains
the byte-compatibility authority.

| Alternative | Cost / proposed decision |
| --- | --- |
| Document all root exports as the stable SDK | Broadens the stability promise to unrelated profiles and private implementation choices. Curate eight read-only operations instead. |
| Add a fifth runtime facade crate now | Requires workspace/dependency policy and packaging changes without proving consumer need. Propose a public module in the existing engine crate first. |
| One generic operation/JSON dispatcher | Adds a second wire contract and transport/error negotiation; typed functions avoid it. #108 can propose a transport later. |
| Return CLI text from Rust | Forces prose parsing and couples presentation to domain decisions; return existing typed results and preserve CLI formatting independently. |
| Rebuild review objects from DTOs | Loses compiler-owned artifact binding and can imply unchecked trust; reuse the existing opaque object. |

A breaking change to these signatures, type aliases/visible fields, ownership,
error accessors or delegated output semantics requires a new facade major and
migration review. Adding an unrelated operation is outside initial acceptance,
not an excuse to reinterpret V1 input. Before any deprecation, document the
replacement and migration path, preserve a supported compatibility interval,
and test both APIs. No public stable-release support window is invented here.
Rollback removes only the new facade module and any CLI import refactor; the
existing root APIs, wire documents and CLI commands remain usable. No user
project, catalog or media migration is needed.

## G2 decisions and next implementation package

This proposal supplies no G2 acceptance and closes none of #107's boxes. A
reviewer/maintainer must decide the eight-operation scope, namespace/result
aliases, opaque error/fallible detail accessors and adapter-only encoding diagnostic,
version/deprecation policy and golden parity rules. Record changes and actual
reviewer identity with the tested candidate commit. A merge of this proposal is
only a design delivery.

After that decision, one focused read-only implementation PR should add the
module/wrappers and observed test-first RED/GREEN, compile-fail opacity and
unavailable-execution examples, pure/concurrent replay, exact old CLI/digest
regressions and same-head Windows/Linux/macOS CI. No wrapper symbol is runnable
in this design-only branch. Actual consumer usability, stable SDK acceptance,
mutation, external host behavior and G5 remain separately unverified.
