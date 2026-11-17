<!-- SPDX-License-Identifier: MIT -->
# Read-only Rust consumer SDK V1

[繁體中文](SDK_CONSUMER_V1.zh-TW.md) · [ADR 0013](adr/0013-read-only-consumer-facade-v1.md) · [G2 decision](validation/m4-01-sdk-g2-decision.md) · [Verification](validation/m4-01-readonly-sdk.md)

The adopted candidate is `storytelling_engine::consumer_v1` in the existing engine crate. This is a Draft implementation of the eight-operation read-only surface, not a published stable SDK or accepted execution capability. Package version stays `0.1.0-dev.0`. The facade versions its Rust API independently of JSON document, planner, compiler/digest and Host-profile versions.

## Run the independent example

From the repository root with Rust 1.89.0:

```sh
cargo run --manifest-path examples/consumer-sdk-v1/Cargo.toml --locked -- \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

This standalone consumer imports only the facade for domain work. Its own `main` explicitly reads the supplied file and writes the existing coverage result. The library receives buffers; it opens no path. Expected result includes the unresolved second row, `video_coverage.unresolved_required`, and `ready_for_planning:false`. This means declared coverage blocks planning, not that a media file or editor was inspected. The example is excluded from the four-member runtime workspace and publication is disabled.

## Typed operations

All return `Result<the named result, SdkErrorV1>` and delegate once to the existing backend with unchanged buffers.

| Function | Inputs | Existing result through the facade |
| --- | --- | --- |
| `validate_plan` | plan bytes | `PlanValidationV1` |
| `render_narrative` | narrative bytes | `NarrativeProjectionV1` |
| `track_revisions` | ledger bytes | `NarrativeRevisionReportV1` |
| `query_impact` | ledger bytes, change ID | `NarrativeRevisionEventResultV1` |
| `video_coverage` | coverage bytes | `NarrativeVideoCoverageResultV1` |
| `plan_story` | planner-request bytes | `PlannedStoryV1` |
| `prepare_story_review` | proposal-response, snapshot, context bytes | opaque `PreparedStoryReviewV1` |
| `check_story_review` | bundle, proposal-response, snapshot, context bytes | `ReviewFreshnessReportV1` |

The two aliases retain existing `ValidationReport` and compiler-owned `BoundStoryReviewV1`. Curated nested result types let a consumer name and match returned fields without importing private modules or adding a core dependency. They add no new parser or trust constructor. Existing wire DTOs reachable from a result remain ordinary untrusted declarations, even when they already support deserialization.

Narrative projection is intentionally typed, without a new JSON serializer. Keep variant-only proposals scoped; revision queries track direct references, not all semantic dependencies. Copying a declared author decision does not authenticate its author.

Review preparation expects the existing response object containing `ok` and `planning`, not a bare request or bare `PlannedStoryV1`. Freeze that exact response byte sequence once and reuse it for preparation/checking; [the example fixture](../tests/fixtures/consumer-sdk-v1/README.md) demonstrates this. A caller creating the existing response object in Rust serializes it once and retains those bytes. It must not parse/re-encode an already selected response on each call. The facade creates no new wire wrapper or transport.

`PreparedStoryReviewV1` exposes only immutable `record()`, `compiled_otio()` and `bundle_json()`. It cannot deserialize from supplied JSON or be constructed from a portable record plus caller-supplied artifact. These accessors return backend payloads; CLI output adds exactly one LF. Do not trim, normalize or reserialize artifact/proposal bytes to compare identities.

## Handle domain outcomes and errors

Unresolved coverage is `Ok` with planning blocked. A well-formed stale review is `Ok` with `current:false` and ordered stale fields; the existing CLI exits 1 for that result. A malformed bundle or incompatible source hash can instead be `Err`. None implies approval or execution authority.

```rust
use storytelling_engine::consumer_v1;

fn main() -> Result<(), consumer_v1::SdkEncodingErrorV1> {
    let error = consumer_v1::video_coverage(b"{").unwrap_err();
    assert_eq!(error.operation(), "video_coverage");
    assert_eq!(error.code(), "input.invalid_json");
    assert_eq!(error.path(), None);
    let details = error.source_details_json()?;
    assert_eq!(details, br#"{"code":"input.invalid_json","path":null}"#);
    Ok(())
}
```

Match identifiers, not `Display` or CLI prose. `source_details_json()` directly serializes the original error, without a CLI `ok/error` envelope or `Value` round trip. It retains null/absent paths, omitted empty narrative `relation_ids`, planner `conflicts` and review/OTIO `losses`, including empty arrays. Encoding failure returns opaque `SdkEncodingErrorV1` with `sdk.error_details_encoding_failed`; the original operation/code/path remain available. The new Rust errors have no public constructor or JSON deserializer. The fixed encoding-failure branch is tested with a test-only failing serializer; no production fault hook exists.

## Limits, concurrency and compatibility

Backend limits/order remain authoritative: ordinary buffers/snapshots 1,048,576 bytes; selected proposal 4,194,336; review context 65,536; selected bundle 4,292,672. Change IDs retain the existing 128-byte grammar/bound. Collection/depth/time limits remain profile-specific. Query validates its ID first; review size/parse checks keep argument order. No common pre-parser, default insertion or time normalization is added.

Calls are synchronous and pure. Results own their data and supported result/error/opaque types are Send/Sync; caller-thread replay is tested. Bound concurrent work and memory at the caller. There is no cache, background job or cancellation token; dropping a result does not undo an editor action or attest Host quiescence.

No execute, approve, dispatch, retry, recover, journal-write, arbitrary path/shell or Host callback is exposed under default or all features. #103–#106 still gate any future mutation capability. Signature mathematics, signer authorization, compilation binding, source-byte identity, content approval and Host readback remain distinct guarantees.

Existing root imports, CLI stdout/exits, wire schemas, rational clocks, OTIO and accepted P1/P2 digest vectors are unchanged. Migrate imports one operation at a time after checking the documented types. A breaking signature/type/ownership/error change requires a new facade major, migration review and documented compatibility interval before deprecation/removal. No stable-release support window is invented. Rollback removes this module/example and consumer imports; it requires no Catalog, project or media migration. #108 chooses its own transport; G5 controls publication.
