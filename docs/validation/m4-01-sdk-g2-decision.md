<!-- SPDX-License-Identifier: MIT -->
# M4-01 / #107 · G2 candidate adoption, 2026-11-17

[Issue #107](https://github.com/bynanci/storytelling/issues/107) · [ADR 0013](../adr/0013-read-only-consumer-facade-v1.md) · [Frozen test plan](../proposals/m4-01-readonly-sdk/test-plan.md)

The requesting maintainer explicitly replied in this conversation:

> G2 採納 並開始實作

This responds to the preceding specific #107 choice: adopt ADR 0013's eight-operation read-only candidate, authorize an independent Draft PR with observed RED/GREEN, retain existing CLI/digests, and leave execution, final SDK acceptance, merge and publication separate. The preceding choice linked candidate `ae389128abc2ce10ada4291b4ec1e9ce7b53e0ab`; its ADR bytes match implementation base `0b39c00354f343d7a61821a029777d88a8373254`. The adopted ADR SHA-256 before this implementation is `f16a7dd625ecb106c5f6a312bae8ba1e1bec14113c629c035df15d45bf849109`.

This is the actual user's scoped technical decision, transcribed by Codex. It is not a signature, DCO, independently authenticated identity or an AI-generated approval. No human participant/Host observation or PR #28 security acceptance is supplied.

## Accepted for this implementation

- `storytelling_engine::consumer_v1` in the existing engine crate; no fifth runtime crate or dependency change.
- Eight operations: `validate_plan`, `render_narrative`, `track_revisions`, `query_impact`, `video_coverage`, `plan_story`, `prepare_story_review`, `check_story_review`.
- Existing owned domain results, curated nested result types, `PlanValidationV1` and opaque `PreparedStoryReviewV1` aliases. Preserve compiler-owned review identity and immutable accessors.
- Opaque `SdkErrorV1` with operation/code/optional-path accessors and fallible original-error JSON serialization; opaque `SdkEncodingErrorV1` with `sdk.error_details_encoding_failed`. No unchecked public constructors or JSON reconstruction of trusted handles.
- Direct delegation of original byte buffers, original error order/fields, unresolved/stale domain results, per-input bounds, version/deprecation and golden parity rules in ADR 0013. Existing CLI, wire profiles, OTIO bytes and accepted digest vectors remain authoritative.
- Pure synchronous calls, owned Send/Sync results and caller-controlled concurrency; execution/approval/recovery remains absent under default and all features.

## Frozen G3–G4 checks

SDK-G01–G10 retain the existing test plan. The first observation must be the actual missing-facade **compile-time** RED; it must not be mislabeled a runtime assertion failure. Then test real CLI/SDK results with independent literals/text goldens and one frozen baseline planner-response byte sequence. Preserve rich error details, null paths, ordering, budgets, future-version rejection, replay/input identity and opaque/absent-capability compile failures. No mock Host, text matching of source code or invented manual success substitutes for those checks.

Use a separate Cargo target directory for this worktree. Run workspace default/all-feature debug/release, fmt, Clippy with warnings denied, rustdoc, repository checks, independent-consumer checks and final exact-head Linux/Windows/macOS CI. The external test/example consumer is outside workspace membership. New glue is independently written first-party MIT code; existing source-informed fixtures/components are referenced in place with their licenses/notices unchanged, without new OpenFilm code imports.

The PR stays **Draft**, #107 stays **Open**, and final implementation review/consumer acceptance remains separate. #103–#106 authority/Host/recovery, #108 transport, #109 application, actual author trials and G5 retain their existing dependencies. This decision does not authorize any real project mutation, package release or merge of this Draft.

## Subsequent acceptance (separate from this original decision)

The later requester instruction 「幫我後續驗收 並收斂issues」 advanced the pending acceptance stage. Codex performed final engineering acceptance, marked #149 Ready, merged tested head `5bf480c` as `9d15382` under the existing verified-merge authority, checked all nine post-merge jobs and completed #107 for its read-only scope. See the [actual acceptance/action record](sdk-acceptance-2026-11-17.md). The original Draft-only G2 scope and reply above are preserved; they are not retroactively presented as a merge approval, human usability result, signature or G5 decision.
