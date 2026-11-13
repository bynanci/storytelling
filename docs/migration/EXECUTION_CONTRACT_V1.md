<!-- SPDX-License-Identifier: MIT -->
# Execution Contract V1 — proposed M3-04 boundary

Status: **G2 proposal, not executable and not accepted**. [繁體中文審查摘要](EXECUTION_CONTRACT_V1.zh-TW.md).
See [ADR 0011](../adr/0011-production-execution-contract-v1.md),
[test/work package](../validation/m3-04-execution-contract.md),
[wire schema](../../contracts/execution-intent-v1.schema.json) and
[synthetic cases](../../tests/fixtures/execution-contract-v1/README.md).

## 1. Scope and trust

V1 proposes one action, `import_otio_isolated_project`, into a newly created
isolated empty project copy. It excludes shared/in-place user projects, audio or
effects outside the accepted finite profile, remote workers and offline signed
grants. A future registered host profile must be accepted separately; the
test-only FilmCraft 0.18.0 Linux picture-only proof does not register one.

A G2-reviewed **candidate profile specification** supports #104 fake tests and a
separately authorized isolated proof. A **production-enabled accepted profile**
requires #105/#106 evidence and explicit maintainer activation in the protected
runtime allowlist. Requests cannot register, promote or override profiles.
Candidate verification does not activate production; the two stages avoid making
#104 implementation depend on its later #106 production acceptance.

Provisioning the empty target is an explicit prior operator/profile workflow,
with its own durable registry/baseline receipt, completed before review and
approval. This action only imports into that existing registered target. It
cannot silently create/recreate a missing project, including during recovery.
Separate provisioning automation requires its own accepted action contract.

| Guarantee | Producer and limit |
| --- | --- |
| Mathematical signature validity | PR #28's verifier checks canonical bytes against a caller-supplied expected Ed25519 key. It does not supply current authorization. Its independent G2 remains pending. |
| Current authorization | Trusted runtime authority provider checks the authenticated session, protected policy, exact execution identity and current human approval at the dispatch commit. Authentication and policy administration belong to the integrating application. |
| Actual compiler binding | Engine constructs the intent from `BoundStoryReviewV1` and its exact owned OTIO bytes, then rechecks current proposal/snapshot/context. Portable DTOs cannot substitute for this result. |
| Applied host effect | Accepted profile reader verifies actual target state after dispatch, including native bytes and expected semantic projection. Hashes alone do not prove the host observation. |

Core/engine remain pure. CLI/runtime performs I/O, authority, storage and host
coordination. Opaque runtime permits prevent accidental construction or JSON
forgery; they are not a sandbox against arbitrary code in the trusted process.
V1 cannot prove human identity, organizational authority outside the configured
provider, host truth before readback, or absence of side effects outside the
profile's observed target/footprint. Supplied-buffer verification is not proof
of later filesystem bytes. Profile acceptance must specify source-byte checks
before dispatch and immutability through readback; an unverifiable source fails.

## 2. Immutable intent and exact bytes

The wire envelope is exactly `schema_version`, `intent`, `intent_sha256`,
`approval_ref`. It is an untrusted request. The schema closes every object.
`approval_ref` only locates a protected authority record; possession is not a
bearer capability. It is outside the execution digest so an explicitly permitted
later attempt can obtain a new approval for the same operation identity.

The `intent` fields below are required, in this **canonical serialization order**:

| Field | Meaning |
| --- | --- |
| `operation_id`, `idempotency_key` | Immutable operation and request keys; one project/key maps to one entire intent. |
| `project_resource_id` | Runtime registry key for the actual isolated target; aliases resolve to the same resource. A user-supplied path/name cannot establish identity. |
| `target_revision_sha256` | Exact baseline native-project digest under the registered profile. Also bound to P2's declared target revision. |
| `review_binding_sha256` | Existing P2 binding; preserves compiler, context, plan and accepted-loss identity. |
| `proposal_sha256`, `snapshot_sha256` | Exact selected planner stdout and finite snapshot bytes. These are additional raw-byte hashes, not replacements for P2's domain-separated digests. |
| `artifact_sha256` | SHA-256 of the exact owned compiled OTIO bytes, compared using P2's artifact identity rules. No reserialization at dispatch. |
| `report_sha256` | Existing P2 conversion-report digest, including accepted/rejected loss semantics. |
| `compiler_version`, `export_profile` | Exact compiler version and finite OTIO profile from P2. |
| `host_profile_id`, `host_profile_sha256` | Registered immutable profile plus its full definition digest (capabilities, projection, clocks, limits, supervision and source checks). |
| `host_version`, `host_binary_sha256` | Actually observed host version and bytes, equal to the declared review context. |
| `adapter_build_sha256` | Pinned executor/adapter build identity. |
| `expected_projection_sha256` | Digest of the profile's deterministic expected native projection derived from the actual compiled artifact, never supplied as trusted by the requester. |
| `action` | Exactly `import_otio_isolated_project`. |

Identity payload is compact UTF-8 JSON in the field order above:
`{"schema_version":1,"intent":{...}}`. All values are ASCII strings except the
schema integer, so escaping and number ambiguity are excluded. Digest:
`SHA256(UTF8("storytelling.execution-identity.v1") || 0x00 || U64BE(payload_length) || payload)`.
The fixture freezes an independent byte/digest vector. JSON member order on input
does not matter; duplicate members always reject before canonicalization.

Production preparation requires P2 context `project_id` to equal the canonical
resource ID and `target_revision` to equal the baseline SHA-256 string. Existing
read-only P2 contexts keep their original contract; they are not automatically
production-admissible.

After G2, engine's proposed `BoundExecutionIntentV1` must own or retain the P2
result, verified descriptor and exact bytes. No public field mutation, unchecked
constructor, `Default`, `Deserialize` or wire-to-permit conversion. A private
runtime `ExecutionPermitV1` additionally binds current authority, project/operation
fences and attempt. Neither type is a transport permission token.

## 3. Current approval and signature responsibility

V1 uses **trusted local current approval**, not an untrusted signed-grant file.
The application configures the authority provider/policy out of band; the request
cannot select a provider, principal, key, permission, policy or current time.
The provider must authenticate the session and produce an explicit human decision
for the exact intent digest, action, target and current review (including losses).
Missing provider/session/decision denies execution. The provider must expose
revocation/current policy checks and a durable single-use record containing:

- approval ID, intent digest, operation ID, principal/session and executor IDs;
- action/resource scope, policy revision/epoch, issuance and expiry;
- explicit current human-confirmation receipt digest (no private text in logs).

Approval lifetime is at most 300,000 ms. Runtime time must be nonnegative i64
epoch milliseconds, supplied by its trusted clock, with issuance <= now < expiry.
Clock rollback/uncertainty denies a new dispatch. Session identity, current policy
and revocation are checked again immediately before dispatch. The policy epoch,
approval consumption and dispatch marker must serialize in the same protected
transaction; an authority backend that cannot provide this ordering is unsupported
in V1. A revoke after that commit cannot erase an already-started operation;
record the event and reconcile its outcome. No bounded clock proves human identity.

Approval is consumed for **one attempt**, atomically with `begin_attempt` and the
production identity/project guard. Crash after consumption is unknown; a fresh
approval is required for any later permitted attempt. Another operation cannot
reuse it. Readback/reconciliation requires current read/recovery permission, not
a new permission to mutate. Returning a cached terminal receipt requires read
permission and never redispatches.

PR #28's `VerifiedApprovalV1` and portable `approval_granted` flags are not accepted
inputs to this provider. Its signing domain and 1 MiB proposal bound remain
unchanged (P2 permits 4,194,336 bytes). A future signed execution grant would need
a separate version/domain binding the full execution identity, a trusted key
registry, expiry/revocation and durable replay consumption, plus independent G2.
Invalid signatures remain covered by that candidate's own tests; this contract
rejects the signed-grant mode before treating any signature as permission.

## 4. One writer, persistence and dispatch

Reuse `OperationIntentV1`, per-operation `ExecutionLease`, epochs, attempt numbers,
append-only events and the existing state machine. Add a production descriptor
and **project-level** guard: current V1 ownership is only per operation ID.
Two operation IDs, request keys, target aliases or restart processes must not
obtain concurrent mutation authority for the same actual resource.

The resource registry resolves an actual target handle, project identity and
baseline revision before dispatch. The registered profile must enforce exclusive
access or host revision/CAS behavior throughout mutation/readback. An advisory
file lock alone does not exclude an external editor. If the profile cannot
establish the required isolation, refuse execution. V1's fresh private project
scope limits this problem; it does not claim protection from malicious external
processes outside that scope.

Dispatch sequence, with all failures before step 5 producing zero mutation calls:

1. Strictly parse and validate the request; recompute P2 and execution bindings.
   Resolve idempotency against the full descriptor. Exact terminal replay returns
   the stored receipt; changed identity rejects. Unknown replay only reconciles.
2. Store exact proposal, snapshot, context, review bundle, artifact, report,
   expected projection and profile privately in a bounded content-addressed store.
   Verify each object on read. Flush required objects before the dispatch commit;
   a missing/corrupt object blocks. Garbage collection cannot remove live evidence.
3. Acquire the canonical project guard and current operation lease. A project
   quarantined by an unknown/partial earlier operation blocks even a new ID.
4. Under that guard, recheck actual host/profile/binary, sources, target revision,
   current review and authority. Atomically persist the full descriptor reference,
   both fences, consumed approval, numbered `dispatching` attempt and its initial
   durable receipt. On storage
   failure, do not dispatch. A durable marker without a call still recovers unknown.
5. Dispatch exactly the retained artifact once; record bounded process identity,
   deadlines and acknowledgements. The adapter cannot reselect cuts or losses.
6. Read back and persist correlated evidence/receipt before claiming a result.
   Failed result persistence after a side effect is unknown; do not send again.

Production storage is a proposed new SQLite schema version 2, using the same
connection/transaction machinery and state semantics, with added descriptor,
approval-consumption, project-guard and evidence-reference tables. Keep the
existing V1 API/database contract unchanged. Production opens a new empty V2
database; no automatic upgrade/import of unresolved V1 rows. Unknown schemas and
foreign databases reject before modification. An explicit migration needs its own
backups/reconciliation plan. One local production store owns a target registry;
multiple independent stores/workers for the same target are unsupported.

## 5. Outcomes, readback and recovery

| Public outcome | Existing journal state / requirement |
| --- | --- |
| `ready` | `Ready`, admitted but not permission to dispatch from wire JSON. |
| `dispatching` | `Dispatching`, write-ahead marker committed; result unresolved. |
| `applied` | `AppliedObserved` only, with verified complete correlated readback, positive prior-request quiescence and durable evidence. |
| `not_applied` | `NotAppliedObserved` only after baseline/footprint equality **and positive prior-request quiescence**. No automatic retry. |
| `unknown` | `OutcomeUnknown`, `AppliedReported`, or an unresolved `PartialReported`; timeout, crash, lost response, failed evidence persistence or ambiguous observation. |
| `partial` | `PartialObserved`, verified nonzero incomplete effect; terminal/manual recovery. |
| `rejected` | Pre-dispatch failure, not an added journal state. No host mutation; an existing uncertain operation stays uncertain. |

Host `Applied` acknowledgement maps only to `AppliedReported`, not public applied.
A timeout/cancellation/lease expiry does not prove the old process stopped. A new
owner can take over **recovery observation**, but the project stays quarantined
until previous execution is proved quiescent and its effect classified. Preserve
the target guard across restart; process death alone does not release it.

An accepted profile's opaque `VerifiedHostReadbackV1` must bind resource, baseline,
execution digest, current attempt, both fences, profile/binary, capture order and
native evidence digests. Its reader must verify the complete expected projection:
clip occurrence IDs/count/order, asset identity and verified source association,
exact source ranges and clocks, timeline placement/duration, tracks, intrinsic
effects, locks/approved loss behavior, online status and no unexpected edits in
the declared footprint. It checks native saved bytes plus host inspection and
save/reopen where the profile requires it. A summary alone is insufficient.
The journal correlates review identity to this evidence; native FilmCraft bytes
are not assumed to carry or authenticate the review digest themselves.

Final `applied` also requires positive quiescence: the expected state appearing
while old/queued work can still write remains unknown. Do not release the
project guard or mark `AppliedObserved` until quiescence and effect are verified.

`not_applied` additionally needs complete baseline equality and positive proof
that no old host request/descendant/queued work can still modify the target.
Killing/reaping the direct child or seeing an unchanged project once is
insufficient without accepted process-group/host semantics. Unsupported evidence
or disagreement remains unknown. A verified partial effect remains partial;
do not label it not_applied. Readback cannot prove absence of every external
side effect outside the declared footprint.

#104 has one attempt. #105 may add an explicit operator retry only after verified
not_applied, quiescence, unchanged original target/identity, current source/review,
new current approval, both writer fences and a newly committed attempt number.
The original execution identity stays fixed; changed target/artifact/profile
requires a new review and operation **after** the old uncertainty is resolved.
Unknown, partial, late conflicting readback, grant expiry/revocation, ownership
loss, active host work, changed target or exhausted budgets forbids retry. No
automatic Undo, compensation, best-effort success or exactly-once claim.

## 6. Result and durable receipt

The [result schema](../../contracts/execution-result-v1.schema.json) closes these
required fields: `schema_version` (1), `operation_id` (ID or null if not parsed),
`intent_sha256` (digest or null), `outcome`, `receipt`, `error`. `outcome` is one of
ready/dispatching/applied/not_applied/unknown/partial/rejected. `error` is null or
exactly `{code,path}`; `code` is from the table below. `$` is the root sentinel;
all non-root paths are RFC 6901 pointers relative to the relevant request or
evidence object (for example `/intent/target_revision_sha256`, `/readback`).

Ready requires nonnull operation/intent IDs and `error:null`. Rejected, unknown,
partial and not_applied require a typed nonnull error; applied/dispatching use
`error:null`. Before an attempt, ready/rejected has `receipt:null`. For an existing uncertain
operation, retain its actual unknown/partial outcome and receipt even if a new
request has an error; do not emit a rejected outcome implying no effect. Every
attempt result includes a receipt envelope `{payload,sha256}`. Receipt payload
fields below are required in exactly this order:

1. `schema_version` (1), `operation_id`, `intent_sha256`, `project_resource_id`;
2. `target_revision_before_sha256`, `target_revision_after_sha256` (digest or null);
3. `attempt_id` (1–64), `operation_fence_epoch`, `project_fence_epoch` (1..i64::MAX);
4. `host_profile_id`, `host_profile_sha256`, `host_version`, `host_binary_sha256`;
5. `authority_receipt_sha256`, `readback_sha256`, `quiescence_sha256`;
6. `outcome` (dispatching/applied/not_applied/unknown/partial), `recorded_at_ms` (0..i64::MAX).

Evidence digests identify the exact bounded private metadata objects flushed
before the receipt commits; verify their raw SHA-256 when loading. They contain
no permission through a public DTO. Authority receipt is required for every
attempt. Readback/quiescence and after-revision digests are nullable only when
the corresponding evidence has not been verified. Applied/not_applied require
all three nonnull; partial requires readback and after-revision, and retains
quarantine even if quiescence is known. Dispatching/unknown preserve whatever
evidence exists without elevating it. A nullable digest is unknown, never an
empty proof. The profile freezes evidence capture/projection details separately.

Outer operation/intent IDs must equal receipt payload IDs, and the resource,
baseline, actual profile/binary, attempt and both fences must match the protected
stored operation and verified evidence. A schema cannot enforce these cross-field
equalities; runtime validation must do so before recording/classifying a result.

Receipt digest is `SHA256(UTF8("storytelling.execution-receipt.v1") || 0x00 ||
U64BE(payload_length) || compact_ordered_payload_UTF8)`. Integers use canonical
base-10 with no floats; tokens/hashes follow the request rules. A new outcome
appends a receipt/event; never overwrite old evidence. Repeating the same terminal
operation returns the original durable receipt bytes, not a newly timestamped
one. A receipt with a matching digest is still untrusted on import; only the
runtime's protected store and verified evidence can produce an in-process result.
If a post-effect storage error prevents a new receipt, return unknown with the
last durable dispatching/unknown receipt and the storage error. Do not manufacture
a persisted unknown receipt. Reopening its dispatch marker triggers recovery.

## 7. Bounds, diagnostics and compatibility

| Surface | V1 proposed bound |
| --- | --- |
| Request / result | 65,536 bytes each; JSON nesting <=8; closed fields, no duplicate/missing fields or floats. Request nulls reject; results allow only the explicitly nullable fields in section 6. |
| IDs/version tokens / hashes | Existing `[A-Za-z0-9][A-Za-z0-9._-]*`, 1–128 ASCII bytes; hashes exactly 64 lowercase hex. |
| Compilation inputs | Existing P2 limits unchanged: proposal 4,194,336; snapshot 1,048,576; context 65,536; bundle 4,292,672; artifact 4,194,304 bytes. |
| Evidence | Metadata receipt <=65,536; native readback <=16 MiB; process stdout/stderr <=1 MiB each; overflow causes unknown if dispatched. Private storage only; no public paths/credentials/raw media. |
| Supervision | Deadline 1–300,000 ms, exact value frozen in accepted profile. Expiry records unknown and initiates supervision/reconciliation, never auto-retry. |
| Journal | Retain 64 attempts/operation, four readbacks/attempt, 512 events/operation, i64 fence bound and 1 ms–24 h leases; authority freshness is independent of lease TTL. |

Version 1 accepts only schema_version 1. Parse/bounds errors report one stable
code/path, without input snippets. Later semantic stages also return the first
failure deterministically. Stage order: (1) byte bound, (2) JSON syntax/duplicates/depth,
(3) version, (4) closed shape/field bounds, (5) digest and P2 freshness,
(6) idempotency/history, (7) resource owner/quarantine, (8) actual host, target then sources,
(9) current authority/grant, (10) durable attempt then host/readback. JSON depth
above 8 returns limit_exceeded; malformed syntax/duplicate members return
invalid_json; a noninteger version returns invalid_shape. In one
stage, fields follow schema order. Never erase an existing unknown state to
report a fresh preflight failure.

| Codes under `execution.` | Meaning |
| --- | --- |
| `input_too_large`, `invalid_json`, `unsupported_version`, `invalid_shape`, `limit_exceeded` | Ordered admission failures. Duplicate fields use invalid_json. |
| `identity_mismatch`, `stale_review` | Digest disagreement or existing P2 freshness failure; retain bounded P2 detail/path in private evidence, not extra fields in the closed public error. |
| `idempotency_conflict`, `writer_conflict` | Changed identity under a key, or project owner/quarantine conflict. |
| `host_mismatch`, `target_mismatch`, `source_mismatch` | Actual registered host/profile/binary, baseline or source bytes differ from reviewed intent. |
| `unauthorized`, `unsupported_approval`, `approval_expired`, `policy_changed`, `replayed_grant` | Local provider absent/deny, signed/offline mode not admitted, stale time/policy or consumed approval. |
| `storage_failure`, `stale_owner` | Atomic persistence failed or operation/project/attempt fencing check failed. |
| `unknown_outcome`, `readback_mismatch`, `partial_outcome`, `retry_forbidden` | Preserve uncertain/partial state; never authorize another call from these codes. |

The ready/dispatching outcomes are API-only intermediate states. The synchronous
CLI returns a classified final or unknown outcome, not an intermediate permit.
Proposed CLI: separate future `execute-story-review`/`reconcile-execution` entry
points with JSON result/diagnostic, exits 0 for readback-verified applied or cached
applied; 1 for rejection, unknown, not_applied or partial; 2 usage; 3 pre-dispatch
I/O/storage failure. Post-dispatch I/O is outcome unknown (exit 1). A stdout write
failure never changes durable operation state. No command is implemented here.

P1/P2 vectors, PR #28 signing domain, narrative/video JSON and existing CLI remain
unchanged. Old clients cannot execute by opting into this schema accidentally.
Rollback disables future mutation entry points, preserves V2/evidence stores and
quarantine, and uses the untouched V1 store for old bounded workflows. Never
restore a pre-dispatch database snapshot and resend against a changed host.
