# CLI and document contracts

The commands below consume different documents. [CLI guide](CLI_GUIDE.md) gives
runnable examples; each linked profile owns its strict wire fields and limits.

| Command | Explicit input / maximum bytes | Successful stdout | Contract |
| --- | --- | --- | --- |
| `validate` | M0 plan JSON / 1,048,576 | `{"ok":true,"validation":...}` | M0 contract below |
| `import-openfilm` | Finite composition-export JSON / 1,048,576 | `{"ok":true,"import":...}` | [Finite importer](migration/FINITE_V1.md) |
| `export-otio` | Same finite composition export / 1,048,576 | Native OTIO `Timeline.1` JSON directly | [Finite OTIO](migration/FINITE_OTIO_V1.md) |
| `audit-brief` | StoryBrief v1 JSON / 1,048,576 | `{"ok":true,"audit":...}` | [Bounded brief audit](migration/STORY_BRIEF_V1.md) |
| `plan-story` | Fixed-window planner envelope JSON / 1,048,576 | `{"ok":true,"planning":...}` | [Fixed-window planner](migration/FIXED_WINDOW_PLANNER_V1.md) |
| `render-narrative` | NarrativeDocumentV1 JSON / 1,048,576 | Deterministic human-readable UTF-8 text; errors are one JSON line | [NarrativeDocumentV1 profile](narrative/NARRATIVE_DOCUMENT_V1.md) |
| `verify-source` | Supplied bytes / 16,777,216, plus expected SHA-256 | `{"ok":true,"verification":...}` | [Supplied-buffer contract](migration/FILMCRAFT_25_V1.md#native-supplied-buffer-contract) |
| `prepare-story-review` | Planner stdout / 4,194,336 + finite snapshot file / 1,048,576 + context file / 65,536 | `{ok:true,review:...,otio:Timeline}`; actual artifact included | [Bounded selected proposal](migration/PLANNER_OTIO_REVIEW_V1.md) |
| `export-story-otio` | Same three inputs | The same native OTIO directly | [Bounded selected proposal](migration/PLANNER_OTIO_REVIEW_V1.md) |
| `check-story-review` | Prepare bundle / 4,292,672 + current proposal/snapshot/context | `{ok:true,freshness:...}`; stale exits 1 | [Bounded selected proposal](migration/PLANNER_OTIO_REVIEW_V1.md) |

The original single-input commands read the named regular file or `-` for stdin;
`verify-source` also requires an expected digest argument. The new story-review
commands accept stdin only for the proposal; snapshot/context/bundle remain explicit
regular files. `export-otio` accepts its explicit loss
flags after the input. Successful OTIO output has no `ok` envelope. A valid brief
with hard conflicts exits 1 and returns `{"ok":false,"audit":...}`; malformed
input instead returns an `error` object. Consult the selected profile's typed
errors, rather than treating every exit 1 as the same result shape.

`plan-story` embeds a strict Brief plus declared candidate windows/scores and hard
duration/shot bounds. It returns a proposal report containing an M0 `plan`;
planner rejection returns an `error` with typed `conflicts` and no plan. Declared
ranking and exact fixed-window timing are distinct from general feasibility,
verified media quality or host execution. The whole `planning` envelope is neither
a bare M0 plan nor a composition export accepted by `export-otio`.

Common exits are 0 for successful evaluation, 1 for domain rejection/conflict,
2 for usage and 3 for I/O. JSON responses are one line; help/version and
`render-narrative` success output are plain text. If stdout fails, no complete
response is promised. JSON collection/depth,
diagnostic and output budgets are profile-specific; a larger source-byte budget
does not enlarge JSON limits. No command executes an editor or grants approval.

## M0 plan contract

The M1-02 `export-otio` command emits a native OTIO document under its own
[finite OTIO contract](migration/FINITE_OTIO_V1.md), with explicit loss choices.

The M1-01 `import-openfilm <FILE|->` command uses the separate
[finite-v1 contract](migration/FINITE_V1.md). Its returned `plan` still obeys
this M0 contract and input size cap; its visual mapping is not executable.

`PlanDocumentV1` is an experimental, finite structural subset, not the complete
future Story SDK or an OpenFilm import format. Its reference is
[the minimal fixture](../examples/minimal-plan.json) and
[strict wire DTOs](../crates/storytelling-core/src/wire.rs).

### Document

Required root fields: `schema_version` (integer 1), `plan_id`, `assets`, `shots`.
Each asset has `id`, `content_hash`, `duration`. Each ordered shot has `id`,
`asset_id`, `source_range: {start,end}`. No optional extension objects are accepted.
Unknown/duplicate JSON fields and malformed shapes are rejected.

Identifiers use `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`. Asset IDs and shot IDs are unique
within their own collections. Multiple shots may reference the same asset.
Hashes are 64 lowercase hexadecimal characters; this checks spelling only.

Time is `{numerator: string, denominator: string}` in seconds. Each string is a
canonical nonnegative decimal u64; denominator > 0 and gcd(n,d) = 1. Thus zero is
`0/1`, `2/2` and `00/1` are refused, and large integers survive JS transport as
strings. Source ranges are half-open with `0 <= start < end <= duration`.
Duration must be positive. No source PTS, FPS or audio-rate conversion is inferred.

Input limit 1,048,576 bytes; 1–1024 declared assets; 1–4096 shots. Serde_json's
normal recursion limit remains enabled. No arbitrary file/URL/script/provider
fields. M0 does not open declared source media or promise timelines/media formats.

### Validate CLI

```text
storytelling validate <FILE|->
storytelling --help
storytelling --version
```

The caller explicitly supplies a local JSON document, or `-` for stdin. File
symlinks may resolve to a regular file; this local read-only CLI is not a sandbox
or server path broker. stdin intentionally waits for EOF. Output is one JSON line
for validate, with no logs interleaved. Help/version are plain text.

Success: `{"ok":true,"validation":{"schema_version":1,"plan_id":"...","assets":1,"shots":2,"scope":"declared-metadata-only"}}`.
Failure: `{"ok":false,"error":{"code":"plan.source_out_of_bounds","path":"$.shots[0].source_range.end"}}`.
Errors never echo source content or local paths. If stdout itself fails no JSON
response is promised; the process exits 3.

| Exit | Meaning |
| --- | --- |
| 0 | Declared document validated, or help/version printed |
| 1 | Invalid JSON/version/metadata/time/reference/bounds/size |
| 2 | Invalid CLI command or arguments |
| 3 | Input/output failure |

Stable codes: `input.invalid_json`, `input.too_large`, `input.io`, `cli.usage`,
`plan.unsupported_version`, `plan.invalid_id`, `plan.invalid_hash`,
`plan.invalid_time`, `plan.invalid_range`, `plan.source_out_of_bounds`,
`plan.duplicate_id`, `plan.unknown_asset`, `plan.empty_collection`,
`plan.too_many_items`. Diagnostic paths may be refined; callers should branch on
codes, not prose or line/column messages.

## M3-02 SQLite operation journal

The Rust storage-adapter API is documented in the
[M3-02 journal contract](migration/M3-02-OPERATION-JOURNAL.md). It is not exposed
as a CLI command and has no host client. Callers persist the immutable operation
identity and numbered `dispatching` attempt before any future host API call.
Unknown outcomes remain blocked until a current owner records readback; only an
explicit NotApplied observation permits a later attempt. The journal records the
caller's observation and evidence digest, but does not verify host truth, signer
authority, approval, or artifact origin.

## M3-01/P1 accepted bounded review binding

The bounded first profile for [issue #27](https://github.com/bynanci/storytelling/issues/27)
is accepted under the signed [M3-01/P1 G2 acceptance record](migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md).
M3-RB01–RB07 are frozen for this profile only; they do not accept the whole M3-01 milestone,
the separate PR #28 candidate, or broader M3 requirements.

| Command / API | Inputs | Behavior |
| --- | --- | --- |
| `prepare-review <COMPOSITION|-> <CONTEXT_FILE>` / `prepare_review` | finite composition bytes ≤1 MiB, strict context ≤65,536 bytes | Imports and compiles OTIO from the same exact composition byte slice and declared losses; returns an untrusted portable review record |
| `check-review <REVIEW_RECORD> <COMPOSITION|-> <CONTEXT_FILE>` / `check_review` | record ≤98,304 bytes, current composition ≤1 MiB, context ≤65,536 bytes | Validates the untrusted record, reimports/recompiles current bytes, reports sorted stale fields; matching means only declaration consistency |
| `BoundReviewV1` | constructed only by `prepare_review` | In-process private result retaining the portable DTO and exact compiled OTIO bytes; no Serialize/Deserialize/Default or public unchecked constructor |

Context v1 has exact fields `schema_version`, `project_id`, `target_revision`, `host`,
`loss_choices`; host contains `id`, `version`, `profile_id`, `binary_sha256`, and
`declared_capabilities`. Identifiers are 1–128 ASCII bytes matching
`[A-Za-z0-9][A-Za-z0-9._-]*`; hashes are 64 lowercase hex characters. Capabilities are
unique ASCII-sorted identifiers, maximum 64. Loss choices are the unique sorted subset of
`metadata_only_locks` and `picture_only`. Unknown, duplicate, missing, null or malformed fields
reject. Review JSON nesting is limited to 64; existing OTIO output cap is 4 MiB.

Identity v1 contains exact raw `composition_sha256`; SHA-256 of serialized typed M0 plan,
serialized OTIO document and complete OTIO loss report under separate versioned domains; engine
crate/compiler version; selected OTIO profile; full context digest; and independently domain-
separated context-field digests. The final binding digest commits to schema version plus the full
identity under `storytelling.finite-otio-review-binding.v1`, with a big-endian u64 payload length.
Composition digest covers exact caller-supplied bytes, so a trailing newline stales composition
even when parsing produces the same artifact. It is not a canonicalized source identity. Context
arrays must use canonical order; other semantic arrays retain their source/compiler order. JSON
serialization here is this Rust versioned DTO encoding, not general JCS.

Error order is surface-specific and covered by multi-invalid-input tests. The direct
`prepare_review` API validates the typed context before importing the composition.
`check_review` checks record, composition and context byte limits in that order before parsing;
then it validates the record, parses/validates context, and reimports/recompiles the composition.
The CLI first validates argument shape, then reads and size-checks each explicit argument in its
command order (`composition, context` for prepare; `record, composition, context` for check)
before domain parsing. Thus a later file I/O error can precede an earlier input's JSON error.
After all inputs are read, the engine's API precedence applies.

The portable record is an untrusted DTO and is never authority. `scope` must be
`declared-review-binding-only`; `executable`, `approval_granted`, `source_verified`, and
`host_verified` must all be false. `check_review` recomputes the binding digest before comparing
the current inputs. A stale record is a successful report (`ok:true`, exit 0, `current:false`);
malformed or forged claims are domain errors (exit 1). Stable stale identifiers include
`composition`, `plan`, `artifact`, `report`, `compiler_version`, `profile_id`, `context`,
`context.project_id`, `context.target_revision`, `context.host.id`, `context.host.version`,
`context.host.profile_id`, `context.host.binary_sha256`,
`context.host.declared_capabilities`, and `context.loss_choices`.

This design separates three guarantees:

1. A mathematically valid signature only proves that a signature verifies under a supplied public
   key for the exact signed message. A portable key/signature does not establish that the key is
   trusted or currently authorized.
2. Trusted signer authorization requires a separately governed trust-root/key registry and
   authorization/revocation policy. This implementation has none and performs no signature check.
3. Binding review to the actual compiled artifact requires rebuilding from the same immutable
   input snapshot and comparing exact compiler output. This candidate does that for one supplied
   finite composition byte slice and its OTIO export. It does not prove that those bytes came from a
   verified build, host snapshot, trusted producer or planner proposal.

There is no approval replay protection, request consumption, authoritative time, expiry/revocation state,
filesystem identity/TOCTOU protection, host observation, execution, or publication. The host
binary fingerprint and capability list are declarations only. M3-02 now persists bounded operation
idempotency and fences late journal updates; it does not deduplicate or stop external host calls.
Trusted grant/authorization and host execution remain broader M3 work.
See [the CLI contract](CLI_GUIDE.md#7-建立與檢查有限審核識別) and
[M3-01/P1 acceptance record](migration/records/M3-01-P1.md).

## Future public workflow (design only)

## Merged PR #28 Approval Identity v1 candidate (not accepted)

This is a separate bounded cryptographic identity/verifier candidate. The
bounded M3-01/P1 profile is documented in
[M3-01-NEXT](migration/M3-01-NEXT.md); this candidate remains separate and
unaccepted pending its own G2 decision.

The bounded core contract is documented in
[Approval Identity v1](migration/APPROVAL_IDENTITY_V1.md). It is not a CLI
command and does not execute or mutate an editor project. The API builds a
private `BoundApprovalIdentityV1` from caller-supplied exact proposal/artifact
byte slices and existing `VerifiedBufferV1` source tokens. It binds those inputs
but does not prove compilation or same-snapshot linkage. A signed payload is
accepted only when every identity field equals that token and an explicit
caller-supplied `ExpectedApproverKeyV1` binding matches; the caller must obtain
that binding from its own trusted source.

The signer receives `storytelling.approval.v1\0` followed by the frozen compact
`ApprovalPayloadV1` JSON bytes. Payload fields are serialized in Rust struct
declaration order by the existing pinned `serde_json`; maps and floating-point
fields are absent. Source IDs, Host capabilities, and accepted-loss rows must
be strictly sorted and unique. `SignedApprovalV1` JSON rejects unknown/duplicate
fields and is capped at 1 MiB. The detached signature and public key are
lowercase hex (64-byte signature / 32-byte key). Core exposes verification only.

`VerifiedApprovalV1` has private fields, no Serde traits, and read-only accessors.
It proves that the supplied key signed this exact payload. It does not prove
that the caller's key registry is trustworthy, that the named principal is
authorized, that an `observed` Host claim is true, that approval has not been
replayed/revoked/expired, or that a Host operation ran. Those are later trusted
surface, M3-02, and M3-03 responsibilities. See the
[PR #28 candidate evidence record](migration/records/M3-01-approval-candidate.md).
