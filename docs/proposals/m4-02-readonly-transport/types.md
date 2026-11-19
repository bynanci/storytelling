<!-- SPDX-License-Identifier: MIT -->
# Proposed independent TypeScript API and owned views

**G2 specification, not a .ts implementation or published package.**
[Protocol](protocol.md) controls framing/limits and [test plan](test-plan.md)
controls verification. No Rust-private type import or CLI-text parser is allowed.

## Public API, ownership and failure model

Proposed location after G2: an independent unpublished
`examples/typescript-consumer-v1/` package with its own lockfile and public
`readonly_v1` exports. The root npm Skill package stays unchanged.

```typescript
type BytesV1 = Uint8Array;
type CallOptionsV1 = { readonly signal?: AbortSignal };
type CleanupV1 = "not_started" | "observed_exit" | "unconfirmed";
type DomainOperationV1 =
  | "validate_plan" | "render_narrative" | "track_revisions" | "query_impact"
  | "video_coverage" | "plan_story"
  | "prepare_story_review" | "check_story_review";

interface DomainFailureV1 {
  readonly layer: "domain";
  readonly operation: DomainOperationV1;
  readonly code: string;                 // exact SDK source code
  readonly path: string | null;          // never substitute "$"
  readonly sourceDetailsJson: BytesV1 | null;
  readonly detailsEncodingError: "sdk.error_details_encoding_failed" | null;
}
interface TransportFailureV1 {
  readonly layer: "transport";
  readonly operation: DomainOperationV1 | "describe";
  readonly code:                         // child-generated codes
    | "transport.invalid_frame" | "transport.header_too_large"
    | "transport.invalid_header" | "transport.unsupported_wire_version"
    | "transport.unsupported_facade_version" | "transport.unsupported_operation"
    | "transport.invalid_buffers" | "transport.input_limit_exceeded"
    | "transport.output_limit_exceeded" | "transport.encoding_failed";
  readonly path: string | null;
}
interface ClientFailureV1 {
  readonly layer: "client";
  readonly operation: DomainOperationV1 | "describe" | null;
  readonly code:
    | "client.invalid_options" | "client.invalid_input" | "client.request_limit_exceeded" | "client.busy"
    | "client.aborted" | "client.timeout" | "client.spawn_failed"
    | "client.process_failed" | "client.invalid_response"
    | "client.response_limit_exceeded";
  readonly cleanup: CleanupV1;
}
type CallResultV1<T> =
  | { readonly ok: true; readonly value: T }
  | { readonly ok: false; readonly error:
        DomainFailureV1 | TransportFailureV1 | ClientFailureV1 };

interface ClientConfigV1 {
  readonly executable: string;   // absolute trusted local path; never portable
  readonly timeoutMs?: number;   // integer 1..120000; default 30000
}
interface PlannedStoryViewV1 {
  readonly planning: PlannedStoryResultV1;
  readonly proposalResponse: BytesV1;  // chosen raw bytes, not re-encoding
}
interface PreparedStoryReviewViewV1 {
  readonly record: ReviewRecordViewV1;
  readonly recordJson: BytesV1;
  readonly compiledOtio: BytesV1;
  readonly bundleJson: BytesV1;
}
interface ReadonlyClientV1 {
  describe(options?: CallOptionsV1): Promise<CallResultV1<CapabilitiesV1>>;
  validatePlan(document: BytesV1, options?: CallOptionsV1):
    Promise<CallResultV1<PlanValidationViewV1>>;
  renderNarrative(document: BytesV1, options?: CallOptionsV1):
    Promise<CallResultV1<NarrativeProjectionViewV1>>;
  trackRevisions(ledger: BytesV1, options?: CallOptionsV1):
    Promise<CallResultV1<RevisionReportViewV1>>;
  queryImpact(ledger: BytesV1, changeId: string, options?: CallOptionsV1):
    Promise<CallResultV1<RevisionEventViewV1>>;
  videoCoverage(coverage: BytesV1, options?: CallOptionsV1):
    Promise<CallResultV1<VideoCoverageViewV1>>;
  planStory(request: BytesV1, options?: CallOptionsV1):
    Promise<CallResultV1<PlannedStoryViewV1>>;
  prepareStoryReview(proposal: BytesV1, snapshot: BytesV1, context: BytesV1,
    options?: CallOptionsV1): Promise<CallResultV1<PreparedStoryReviewViewV1>>;
  checkStoryReview(bundle: BytesV1, proposal: BytesV1, snapshot: BytesV1,
    context: BytesV1, options?: CallOptionsV1):
    Promise<CallResultV1<ReviewFreshnessViewV1>>;
}
declare function createReadonlyClientV1(config: ClientConfigV1):
  | { readonly ok: true; readonly client: ReadonlyClientV1 }
  | { readonly ok: false; readonly error: ClientFailureV1 };
```

Expected failures resolve to the discriminated union; no stderr/Display/prose
parsing or exceptions carrying a private path. Unhandled programming defects
are defects, not manufactured domain failures. Config validation is synchronous.
No process is started by factory construction; `describe` is explicit. Every
method still validates wire/facade/view compatibility per response.

Input snapshots happen before the first asynchronous yield. Output byte arrays
are owned copies; where the client retains bytes it returns a fresh copy from
each accessor. `readonly` is a TypeScript constraint, not a proof of runtime
immutability or authentic authority. A caller can mutate its own copy; feeding
it back is untrusted input. The portable prepared view has no methods to approve,
execute, retry, construct a trusted Rust handle or grant an author identity.

Preflight the transport's bounded header and aggregate frame lengths before
copying input buffers. Oversize is client.request_limit_exceeded with no child,
not a new domain code. A normal domain-invalid source within the wire budget
is still delegated unchanged. A success's opaque OTIO/bundle buffers must be
nonempty, but remain untrusted and are not independently compiled by the client.

## Common owned domain values

These are the proposed client's independently owned shape definitions, not
imports or changes to the referenced existing Rust contracts. Arrays are
readonly throughout. Decoders check exact fields, scalar types, enums, safe
integer ranges and absent-versus-null rules; they do not re-run domain validation.
They reject malformed/mismatched results rather than coerce them.

```typescript
type U32V1 = number;  // exact integer 0..4294967295
type I32V1 = number;  // exact integer -2147483648..2147483647
type Sha256V1 = string;
interface TimeV1 { readonly numerator: string; readonly denominator: string }
interface RangeV1 { readonly start: TimeV1; readonly end: TimeV1 }
interface UnitRefV1 { readonly unit_id: string; readonly revision: U32V1 }
type OriginV1 =
  | "supplied_observation" | "author_interpretation" | "creative_proposal";
interface SourceTagV1 extends UnitRefV1 { readonly origin: OriginV1 }
interface PlanDocumentViewV1 {
  readonly schema_version: 1;
  readonly plan_id: string;
  readonly assets: readonly {
    readonly id: string; readonly content_hash: string; readonly duration: TimeV1
  }[];
  readonly shots: readonly {
    readonly id: string; readonly asset_id: string; readonly source_range: RangeV1
  }[];
}
interface PlanValidationViewV1 {
  readonly schema_version: 1;
  readonly plan_id: string;
  readonly assets: number; readonly shots: number; // bounded nonnegative integers
  readonly scope: "declared-metadata-only";
}
interface NarrativeProjectionViewV1 {
  readonly schema_version: 1;
  readonly document_id: string;
  readonly document_revision: U32V1;
  readonly purpose: string;
  readonly audience: string;
  readonly variants: readonly (
    | { readonly kind: "spoken_outline"; readonly id: string; readonly name: string;
        readonly beats: readonly {
          readonly id: string; readonly source: SourceTagV1; readonly text: string
        }[] }
    | { readonly kind: "single_illustration"; readonly id: string;
        readonly name: string; readonly sources: readonly SourceTagV1[];
        readonly brief: string }
  )[];
}
interface RevisionEventViewV1 {
  readonly id: string;
  readonly kind: "assistant_suggestion" | "author_decision"
    | "source_revision" | "variant_review";
  readonly unit_id?: string;
  readonly from_revision?: U32V1; readonly to_revision?: U32V1;
  readonly variant_id?: string;
  readonly affected_variants: readonly string[];
}
interface RevisionReportViewV1 {
  readonly document_id: string;
  readonly events: readonly RevisionEventViewV1[];
  readonly source_history: readonly {
    readonly unit_id: string; readonly revision: U32V1;
    readonly origin: OriginV1; readonly text: string;
    readonly based_on: readonly UnitRefV1[]
  }[];
  readonly variant_history: readonly {
    readonly variant_id: string; readonly revision: U32V1;
    readonly kind: "spoken_outline" | "single_illustration"; readonly name: string;
    readonly source_refs: readonly UnitRefV1[]; readonly content: string
  }[];
  readonly suggestions: readonly {
    readonly id: string; readonly variant_id: string;
    readonly proposal_ref: UnitRefV1; readonly text: string
  }[];
  readonly author_decisions: readonly {
    readonly id: string; readonly variant_id: string;
    readonly proposal_ref: UnitRefV1;
    readonly decision: "accepted" | "edited" | "declined";
    readonly variant_revision: U32V1
  }[];
  readonly current_variants: readonly {
    readonly variant_id: string; readonly revision: U32V1;
    readonly status: "current" | "needs_review";
    readonly stale_source_refs: readonly UnitRefV1[];
    readonly source_refs: readonly UnitRefV1[]; readonly content: string
  }[];
}
interface AssetRefV1 { readonly asset_id: string; readonly revision: U32V1 }
interface VideoCoverageViewV1 {
  readonly schema_version: 1;
  readonly profile: "storytelling.narrative-video-coverage.v1";
  readonly coverage_id: string;
  readonly narrative_ref: {
    readonly document_id: string; readonly document_revision: U32V1;
    readonly variant_id: string
  };
  readonly asset_catalog_ref: { readonly catalog_id: string; readonly revision: U32V1 };
  readonly selected_unit_refs: readonly UnitRefV1[];
  readonly mappings: readonly (
    | { readonly unit_ref: UnitRefV1; readonly status: "mapped";
        readonly asset_ref: AssetRefV1; readonly source_range: RangeV1 }
    | { readonly unit_ref: UnitRefV1; readonly status: "unresolved";
        readonly reason: string }
  )[];
  readonly ready_for_planning: boolean;
  readonly diagnostics: readonly { readonly code: string; readonly path: string }[];
}
```

A `RevisionEventViewV1` omitted optional field stays absent; null is not an alias
for omission. The binding does not infer indirect semantic impact or authenticate
declared author choices. Narrative variants preserve proposal source tags only
where returned; the imaginary umbrella cannot become an observed event.

## Planner and review views

```typescript
interface AssetBindingViewV1 { readonly asset_id: string; readonly origins: readonly string[] }
interface ConflictViewV1 {
  readonly kind: "unknown-asset" | "required-excluded" | "order-cycle";
  readonly path: string; readonly asset_ids: readonly string[]
}
interface BriefAuditViewV1 {
  readonly schema_version: 1; readonly brief_id: string; readonly intent: string;
  readonly scope: string; readonly executable: false;
  readonly selection_planned: boolean; readonly duration_analysis: string;
  readonly consistent: boolean; readonly manual_selected_asset_ids: readonly string[];
  readonly required_assets: readonly AssetBindingViewV1[];
  readonly excluded_assets: readonly AssetBindingViewV1[];
  readonly precedence_edges: readonly {
    readonly before: string; readonly after: string; readonly origins: readonly string[]
  }[];
  readonly preferences: {
    readonly preferred_asset_ids: readonly string[]; readonly target_duration?: TimeV1
  };
  readonly conflicts: readonly ConflictViewV1[];
  readonly warnings: readonly {
    readonly kind: "unknown-preferred-asset" | "excluded-preference";
    readonly path: string; readonly asset_ids: readonly string[]
  }[];
}
interface PlannedStoryResultV1 {
  readonly schema_version: 1;
  readonly profile: "storytelling.fixed-window-planner.v1";
  readonly scope: "declared-fixed-window-plan-only";
  readonly executable: false; readonly source_verified: false; readonly host_verified: false;
  readonly selection_planned: boolean; readonly duration_analysis: string;
  readonly content_clock_rate: string; readonly total_duration: TimeV1;
  readonly target_status: "not-requested" | "met" | "below" | "above";
  readonly brief_audit: BriefAuditViewV1; readonly plan: PlanDocumentViewV1;
  readonly selections: readonly {
    readonly asset_id: string; readonly shot_id: string; readonly required: boolean;
    readonly declared_score: I32V1; readonly timeline_start: TimeV1;
    readonly duration: TimeV1; readonly reason_paths: readonly string[]
  }[];
  readonly skipped_candidates: readonly {
    readonly asset_id: string; readonly path: string;
    readonly reason: "excluded" | "not-needed" | "shot-limit" | "duration-limit"
  }[];
}
interface ReviewFieldDigestsViewV1 {
  readonly project_id: Sha256V1; readonly target_revision: Sha256V1;
  readonly host_id: Sha256V1; readonly host_version: Sha256V1;
  readonly host_profile_id: Sha256V1; readonly host_binary_sha256: Sha256V1;
  readonly host_declared_capabilities: Sha256V1; readonly loss_choices: Sha256V1;
}
interface ReviewRecordViewV1 {
  readonly schema_version: 1; readonly scope: string;
  readonly binding_sha256: Sha256V1;
  readonly identity: {
    readonly composition_sha256: Sha256V1; readonly plan_sha256: Sha256V1;
    readonly artifact_sha256: Sha256V1; readonly report_sha256: Sha256V1;
    readonly compiler_version: string; readonly profile_id: string;
    readonly context_sha256: Sha256V1;
    readonly context_fields: ReviewFieldDigestsViewV1
  };
  readonly executable: false; readonly approval_granted: false;
  readonly source_verified: false; readonly host_verified: false;
}
interface ReviewFreshnessViewV1 {
  readonly schema_version: 1;
  readonly scope: "declared-planner-otio-review-only";
  readonly current: boolean; readonly stale_fields: readonly string[];
  readonly executable: false; readonly approval_granted: false;
  readonly source_verified: false; readonly host_verified: false;
}
interface CapabilitiesV1 {
  readonly wire_version: 1; readonly facade_version: 1;
  readonly package_version: string;
  readonly operations: readonly DomainOperationV1[]; // exactly the table's eight, in order
  readonly views: readonly string[];                // exactly the table's nine, in order
  readonly limits: {
    readonly header_bytes: 16384; readonly request_frame_bytes: 16777216;
    readonly response_frame_bytes: 67108864; readonly stderr_bytes: 65536
  };
}
```

Review profile IDs are checked against the pinned output profiles. Compiler
identity and package strings are retained as opaque metadata without a client
allowlist; compiler freshness belongs to the Rust backend. A package patch alone
does not invalidate an otherwise compatible wire/facade/view. No string is rewritten.
No flag is promoted to true by the bridge. A malicious response with invented
authority flags is malformed, not an authorization event.

Full source error detail is owned raw JSON, not a lossy subset mapped into a new
domain error. Its exact bytes remain accessible alongside source code/path.
The client checks duplicate keys, UTF-8, depth, safe integer tokens and top-level
code/path agreement while retaining all remaining evidence fields; it does not
discard a conflict/loss it does not interpret.

## Structural decoding and migration

All enumerated objects are closed; do not silently drop unknown fields in
success views. Strings stay exact Unicode (no normalization); source/identity
bytes stay bytes. Count fields are safe nonnegative integers; revisions u32,
scores i32, rational/clock values strings. Reject number coercion, booleans as
integers, fraction/exponent tokens for integer fields and unsafe integers.
No semantic source-ID or range checking replaces the accepted backend.

An implementation may split these owned interfaces into files and choose an
audited parser/decoder internally, but cannot omit public fields, coerce values,
import Rust internals, create a trusted-review constructor or relax the closed
view without a reviewed change. Dependency and parser choice needs its own
G3 supply-chain evidence; none is installed or accepted here.

The minimal future example first calls describe, then the eight operations over
the existing named fixtures. It retains the raw planner response and review
bundle in memory across calls, shows unresolved/stale as meaningful results,
and inspects source errors by identifiers. It uses explicit local reads only
at the application entry. It is an engineering consumer, not a participant
study or approval workflow. No execution/recovery method exists.
