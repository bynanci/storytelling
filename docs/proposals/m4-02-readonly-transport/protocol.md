<!-- SPDX-License-Identifier: MIT -->
# Proposed read-only process protocol V1

**G2 candidate only.** [ADR](../../adr/0014-readonly-process-transport-v1.md) ·
[Owned views](types.md) · [Cases](test-plan.md) · [Decision](../../validation/m4-02-transport-g2.md).
Normative words describe the proposed bridge, not existing executable behavior.

## 1. Wire frame and bounds

One process accepts **one** request, followed by stdin EOF, and emits **one**
response, followed by stdout EOF and exit. No banner, newline, heartbeat, batch,
compression, implicit encoding negotiation or second frame is allowed.

```text
4 bytes: ASCII STRO (53 54 52 4f)
4 bytes: H, unsigned 32-bit big-endian header byte length
H bytes: strict UTF-8 JSON header
remaining bytes: raw buffers, concatenated in descriptor order
EOF
```

| Quantity | Candidate bound |
| --- | --- |
| H | 1..16,384 bytes |
| Request frame, including 8-byte prefix and header | 16,777,216 bytes |
| Response frame, including prefix and header | 67,108,864 bytes |
| Buffer count | 0..4, exact per-operation role/order |
| Descriptor `byte_length` | Nonnegative u32; zero permitted at framing level |
| Header JSON depth | 8 container levels, root object at level 1 |
| Structured response JSON depth | 64 container levels; never applied to source input buffers |
| Stderr capture budget | 65,536 bytes; normal bridge stderr is empty |
| Active calls per client instance | 1; no implicit queue |

Lengths count **bytes**, not characters. Calculate checked sums before allocation;
reject overflow or cap excess. The response is fully encoded through bounded
writers before its first stdout byte; an encoding/cap failure emits only the
small structured error frame. These caps bound wire data, **not all backend
allocations or process RSS**. Process resource isolation is a caller deployment
responsibility; benchmark memory in G4.

Headers reject BOM, invalid UTF-8, unpaired escaped surrogates, duplicate keys at
any nesting, non-object roots, unknown fields and invalid field types. Integers
use plain decimal JSON tokens, with no fraction/exponent; do not round them.
Candidate schemas describe structural validity but cannot enforce duplicate-key,
lexical-integer, byte/depth/EOF checks or diagnostic precedence.

Use a structural probe before semantic version/operation selection: P2 accepts
u32 version values and an operation string for later P3/P4 rejection, rather
than failing early on a const/enum. Root/descriptor fields are closed at P2;
operation-specific params are closed at P5. Candidate schemas describe the
fully valid format, not the error-order implementation.

Source buffers are opaque bytes until the selected facade receives them. In
particular, invalid source UTF-8/JSON, a domain-invalid ID and a one-byte-over
backend buffer still reach that backend when the frame fits transport bounds.
Do not add a domain pre-parser or pre-enforce its narrower per-buffer limits.

## 2. Headers and operation mapping

Request fields are exactly `protocol` (constant `storytelling.readonly`),
`wire_version:1`, `facade_version:1`, `request_id`, `operation`, `params` and
`buffers`. A request ID is 1..64 ASCII characters matching
`[A-Za-z0-9][A-Za-z0-9._-]*`. Descriptor fields are exactly `name` and
`byte_length`. The operation fixes every buffer name/order. All params are `{}`
except `query_impact`, whose only param is a string `change_id`. Its domain ID
grammar/128-byte bound belong to the backend, not the header schema.

| Wire operation | Buffer roles, in exact order | Success view / buffer roles |
| --- | --- | --- |
| `describe` | none | `capabilities_v1` / `result_json` |
| `validate_plan` | `document` | `plan_validation_v1` / `result_json` |
| `render_narrative` | `document` | `narrative_projection_v1` / `result_json` |
| `track_revisions` | `ledger` | `revision_report_v1` / `result_json` |
| `query_impact` | `ledger` | `revision_event_v1` / `result_json` |
| `video_coverage` | `coverage` | `video_coverage_v1` / `result_json` |
| `plan_story` | `request` | `planned_story_v1` / `result_json`, `proposal_response` |
| `prepare_story_review` | `proposal`, `snapshot`, `context` | `prepared_story_review_v1` / `record_json`, `compiled_otio`, `bundle_json` |
| `check_story_review` | `bundle`, `proposal`, `snapshot`, `context` | `review_freshness_v1` / `result_json` |

Every domain row delegates exactly once to the same-named `consumer_v1` function.
`describe` is bridge metadata, never a domain validator or an authority check.
There is no wildcard or execution/recovery alias.

Every response has `protocol`, `wire_version`, `facade_version`, `request_id`,
`operation`, `status` and `buffers`. For a validated request, ID/operation must match
exactly. Success adds only `view`. Domain failure adds only `domain_error`.
Transport failure adds only `error`. Header schemas are closed.

For an invalid request before validation, a transport-error response uses null
ID/operation; it cannot guess a caller association from partially parsed data.
After validation, transport errors echo the exact ID/operation. Because each
process has one call, the client may associate a null-ID response only with a
request-validation transport code listed below. Every other missing/wrong
association, including wrong non-null IDs, is `client.invalid_response`.

## 3. Bytes, views and identity

Non-narrative serializable domain results use their existing Rust serializer
directly, without a JSON Value round trip or new source fields. The proposed
narrative projection view maps the typed enum explicitly to `kind`-tagged
variants, preserves every field/order/source origin and stays in the bridge.
Core projection types do not gain `Serialize` or a new domain schema.

All arrays preserve backend order, including impact/coverage rows, planner
choices, rich diagnostics and stale fields. All rational numerators,
denominators and clock-rate values remain strings. TypeScript never converts
these to Number, normalizes fractions or infers times from display text.

For `plan_story`:

1. Encode the returned `PlannedStoryV1` once as `result_json`.
2. Construct `proposal_response` once by concatenating the exact ASCII prefix
   `{"ok":true,"planning":`, those result bytes, and `}`. No LF is added.
3. Return both owned buffers. The caller selects and retains this response
   sequence for subsequent preparation/checks. Never rebuild it from a decoded
   TypeScript object or serialize it again on each review call.

That existing proposal shape is accepted by the backend; this is not a new
planner document. A bridge-created response may have a different raw digest
from the old CLI's serialized response plus LF. Domain-equivalent planning is
not raw-byte identity. If a caller selected an existing CLI/other valid response,
pass **its original bytes**, including any LF, unchanged instead.

Preparation gets record, OTIO and bundle directly from the **same**
`PreparedStoryReviewV1` value's immutable accessors. Serialize only its record;
copy accessor OTIO/bundle bytes exactly. Do not reconstruct a trusted Rust
handle from transported JSON. The Node `PreparedStoryReviewViewV1` is portable
untrusted data, with no approval/execute brand. Checking that view always calls
the backend with the retained raw bundle and original current inputs.

The transport never removes/adds a framing LF to source or artifact bytes.
Magic, lengths, request IDs, metadata and view versions do not enter existing
proposal/artifact/context/P1/P2 digests. A checksum alone authenticates nothing.

Success output buffers are nonempty. The client structurally decodes result or
record JSON, but treats compiled OTIO and bundle bytes as opaque. It does not
reimplement compilation, infer media validity or authenticate the record by
parsing a bundle. A later check delegates the retained original bytes back to
the backend; actual G4 parity proves the bridge extracted them from one value.

## 4. Domain outcomes and failures

`status:"ok"` can include `ready_for_planning:false` or `current:false`. A stale
review is not an error. It conveys no approval, actual media verification, Host
qualification or current execution authority. This new protocol's exit 0 does
not alter the existing CLI's stale-result exit 1.

A domain-error header contains:
`{code:string,path:string|null,details_encoding_error:null|"sdk.error_details_encoding_failed"}`.
Code/path are copied from the retained SDK error and are not English prose.
When details encode, exactly one `source_details_json` buffer contains the
original accessor bytes (no CLI envelope). Preserve null versus absent fields,
field order, `relation_ids` omissions, and `conflicts`/`losses` including empty
arrays. Strictly decode this detail object only to check its code/path agree
with the header; preserve its complete original bytes and opaque extra evidence.
Absent/null detail path maps to the accessor's null summary, without rewriting
the detail object.

If detail encoding fails, retain original domain code/path, set the fixed SDK
encoding code and return no detail buffer. Do not return empty detail bytes or
replace the original rejection with encoding prose. If an encoded response is
too large, return a transport cap error without a partial result.

Child-generated transport codes and precedence:

| Order | Condition | Code / header path |
| --- | --- | --- |
| P1 | Bad/truncated magic/prefix; H=0; truncated header | `transport.invalid_frame` / null |
| P1 | H exceeds 16,384 | `transport.header_too_large` / null |
| P2 | Header UTF-8/JSON/duplicate/depth/base shape or type failure | `transport.invalid_header` / null |
| P3 | Unknown wire version, then facade major | `transport.unsupported_wire_version` / `$.wire_version`; `transport.unsupported_facade_version` / `$.facade_version` |
| P4 | Unknown operation | `transport.unsupported_operation` / `$.operation` |
| P5 | Invalid operation params | `transport.invalid_header` / `$.params` |
| P5 | Wrong/missing/duplicate/extra/out-of-order buffer roles | `transport.invalid_buffers` / `$.buffers` |
| P6 | Declared frame exceeds request cap; checked length-sum failure | `transport.input_limit_exceeded` / null |
| P7 | Truncated body, surplus bytes or second frame before EOF | `transport.invalid_frame` / null |
| P8 | Original facade rejection | `status:"domain_error"`; original first error |
| P9 | Response frame exceeds cap | `transport.output_limit_exceeded` / null |
| P9 | Result/view encoding failure | `transport.encoding_failed` / null |

Validate P1 through P7 completely before delegation. Within P5 check params
before roles. Within P9 cap failure wins when the bounded writer reports cap
exhaustion; otherwise use encoding_failed. Null correlation is allowed only for
P1–P7 request-validation codes; P9 must echo the validated request. No path or
exception text from OS I/O is emitted.

A completed structured frame, including a domain/transport-error frame, exits 0.
Nonzero exit, signal, truncated stdout, unexpected stderr or inability to
complete a response is a client process/protocol failure. Do not trust a complete
frame from a child that later exits nonzero.

## 5. Node lifecycle and cancellation

The caller configures an absolute, trusted executable path outside portable
documents. Proposed fixed argv is `["sdk-readonly-v1"]`, with `shell:false`.
No request-derived argv/cwd/path, PATH search, shell interpolation, network,
media read, file write, Host invocation or child-of-child operation is allowed.

The environment is empty on POSIX; Windows may copy only the caller's local
`SystemRoot` needed by the OS loader. No inherited tokens, signing material,
model keys, Host credentials or user-supplied process environment is forwarded.
The client uses no temp files and the bridge emits no request contents to logs.
Runtime panic/OS stderr stays private, bounded and excluded from public errors.

The client snapshots each Uint8Array synchronously before asynchronous work;
SharedArrayBuffer-backed or detached inputs and unpaired-surrogate header
strings are `client.invalid_input`. Ordinary input/output arrays are owned
copies; TypeScript readonly does not make a Uint8Array intrinsically immutable.
Accessors return copies when retaining internal identity-bearing bytes.

At method entry check options, already-aborted signal, busy state, input types
and buffer metadata, then the bounded header/total-frame budget, then snapshot
inputs. Reject SharedArrayBuffer/detached input as invalid_input before copying.
Check the wire budget without first copying arbitrary buffers or materializing
unbounded header JSON; any intermediate header encoding needs a fixed bound.
A request exceeding the header or frame budget returns
`client.request_limit_exceeded` with not_started. Narrower domain limits are
still left to the facade. An active call owns one process and no queue. Unsupported versions,
malformed output and OS failure never trigger a fallback CLI command or retry.

The wall deadline includes snapshot, spawn, stdin writing, processing, stdout
reading/decoding and process close. Default 30,000 ms; explicit integer
`timeoutMs` is 1..120,000. Use a monotonic clock and check after synchronous
encoding/decoding too. JS event-loop delay means callback scheduling is not a
hard real-time guarantee.

Accept success/domain error only after a complete checked frame, stdout EOF,
empty stderr and observed exit 0, before abort/deadline. Terminal settlement is
once only. An abort/timeout observed before that commit discards even a partial
or fully received uncommitted response. Race tests freeze these boundaries.

Read stdout/stderr concurrently with stdin writing. An early P1–P7 child
rejection may close stdin and cause EPIPE before the writer finishes: keep that
write error provisional until bounded response/exit collection completes.
A valid pre-validation transport error plus EOF/empty stderr/exit 0 can be
returned despite EPIPE. Success or domain_error requires the entire request to
have been written and stdin closed. EPIPE without a valid rejection, or any
nonzero child exit, is client.process_failed; never accept partial input as
success. Abort/deadline still wins before result commit.

On cancellation, timeout, malformed/excess output or process failure, close
stdin and terminate only this child. Propose SIGTERM then SIGKILL after 250 ms
(or Node's supported Windows termination), with a total 1,000 ms cleanup budget.
Report cleanup as `not_started`, `observed_exit` or `unconfirmed`. Unconfirmed
termination locks that client instance until an actual exit is observed;
never silently start a replacement. Preserve the triggering error, rather than
relabeling cancellation as success. No Host quiescence, Undo or retry is inferred.

Client error codes are `client.invalid_options`, `client.invalid_input`,
`client.request_limit_exceeded`,
`client.busy`, `client.aborted`, `client.timeout`, `client.spawn_failed`,
`client.process_failed`, `client.invalid_response` and
`client.response_limit_exceeded`. They are client-layer codes, not domain or
the existing 21 coverage codes. A response wrong version/view/shape/association,
duplicate key, unsafe numeric token, unexpected stderr or trailing stdout is
invalid_response; response byte-cap excess is response_limit_exceeded. Never
expose private paths/stderr in the public failure object.

## 6. Compatibility and G2 exit

| Version axis | Initial proposal / authority |
| --- | --- |
| Wire / view | `storytelling.readonly`, wire 1 and named V1 views; no auto negotiation/fallback. |
| Rust facade | `consumer_v1`, independently accepted in #107. |
| Plan / narrative / ledger / coverage / planner request | Existing schema_version 1; backend-specific rejection remains authoritative. |
| Planner profile | `storytelling.fixed-window-planner.v1`. |
| Selected review profile | `storytelling.planner-otio-visual-ticks.v1`; existing compiler/digests unchanged. |
| Compiler identity | Preserve the backend string exactly, currently `storytelling-engine-0.1.0-dev.0`. No client compiler allowlist; existing backend freshness checks compare current/previous compiler identities. |
| Package / Node | Unpublished dev package; proposed client baseline Node 22.20+. Package version is not a protocol or Host attestation. |
| Production mutation / release | Unavailable; #103–#106 and G5 independent. |

`describe` returns fixed supported operations/views/limits and version strings,
not "latest", approval flags or a verified executable hash. Every response
repeats supported wire/facade versions; the client checks them per call, not
only at startup. No cached describe grants trust to a replaced executable.

Package metadata is an informational string, not a compatibility allowlist:
compatible wire/facade/views are not rejected merely for a different package
patch. Output profile IDs remain pinned as above; input profiles and compiler
freshness are checked by the original backend. Preserve every original string,
including an earlier compiler ID inside an input bundle. No upgrade is implied.

A breaking framing/view/operation/error/ownership change needs a new wire major
or explicitly new view/operation plus compatibility review. Reject unknown
versions, operations, fields and views; never strip them or choose a looser
profile. A valid backend output may exceed the transport's response budget;
that is an explicit partial-support/resource limit, not a backend error or
proof the input is invalid. Freeze this tradeoff in G2 and test cap boundaries.

G3 is blocked until the [exact-head decision](../../validation/m4-02-transport-g2.md).
G4 must demonstrate named cases, all eight Rust/TS equivalent outcomes, original
bytes, a fresh-checkout example and three-platform process failures. No such
runtime, performance, human, media, Host or release evidence is supplied by
candidate schemas/examples.
