# M3-01/P1 G2 contract proposal: finite-OTIO review binding

Status: **proposed for independent G2 review**. This document is a review packet,
not a frozen contract, accepted requirement, implementation, human approval, or
source/host verification. Tracking issue: [#27](https://github.com/bynanci/storytelling/issues/27).
No Issue #27 checklist item is completed by this document.

The proposed first profile reviews one finite OpenFilm composition snapshot using
the existing importer and OTIO exporter. It does not review `plan-story` output:
the planner-to-composition compiler bridge does not exist. Source symbols, exact
source/test blob identities, and extraction limits are recorded in
[M3-01-NEXT.md](M3-01-NEXT.md). This proposal copies no OpenFilm source, tests, or
fixtures. Any future source-derived code or tests still need the approved
Apache-2.0/SPDX and per-file provenance treatment.

## Guarantees and boundaries

Three different claims must stay separate:

| Claim | What establishes it | What it does not establish |
|---|---|---|
| Signature is mathematically valid | Ed25519 verification succeeds for the exact domain-separated payload and the public key supplied to the verifier | That the key is trusted, belongs to the named person, or has authority for this project/action |
| Signer has trusted authorization | A separately governed principal/key registry and policy bind the key to a current role, scope, validity and revocation state | That the reviewed OTIO was compiled from the same source snapshot |
| Review is bound to the actual compiled artifact | A private builder validates and compiles the exact supplied composition bytes, then hashes the exact owned output/report and declared context | Host observation, filesystem freshness, human acceptance, authorization, replay prevention, or safe execution |

Merged candidate [PR #28](https://github.com/bynanci/storytelling/pull/28) implements
the first claim only within its supplied-input boundary. `bind_approval_identity_v1`
hashes caller-supplied proposal and artifact bytes separately; its own API
documentation says it does not compile them or establish their semantic
relationship. `ExpectedApproverKeyV1` is supplied by the caller; the verifier
checks the signed principal/key IDs against that input and verifies the signature.
There is no trusted key registry, revocation authority, or proof that the
caller-provided artifact came from the supplied proposal. `VerifiedApprovalV1`
being opaque prevents fabrication of that verification result, but does not add
those missing trust claims. PR #28 remains an unaccepted candidate pending its
own independent G2 review.

Issue #27 owns same-input compilation linkage, immutable finite review identity,
declared-context freshness, and a read-only CLI. It must not silently claim the
second guarantee or reuse PR #28's signature verification as proof of the third.
Signer authorization, trusted time/revocation, durable one-use/replay enforcement,
and execution belong to later separately accepted work.

## Proposed construction and wire boundary

`prepare-review <COMPOSITION|-> <CONTEXT_FILE>` would read bounded bytes and a
strict context DTO, then call the existing importer and
`export_openfilm_otio` against the same immutable composition byte slice and
explicit existing loss choices. The builder would immediately serialize the
returned `CompiledOtioV1` internally and retain only the bytes/digests it needs;
callers would never supply a prebuilt `CompiledOtioV1`, report digest, or artifact
digest as evidence of compilation. This is necessary because
`CompiledOtioV1 { document, report }` is currently public and mutable.

The context proposal has fixed fields only: schema version; ASCII `project_id`;
declared `target_revision` (including an explicit new-isolated-project value);
expected host ID, version, profile ID and lowercase binary SHA-256; a sorted,
unique set of required capability IDs; and a sorted, unique set of the existing
`picture_only` / `metadata_only_locks` choices. The values are caller declarations.
Those two loss-choice IDs are mapped directly to the existing
`OtioLossChoicesV1` fields and are supplied to compilation; no parallel caller
choice object can disagree with the identity.
The P1 builder must not inspect a host, open a project URI, resolve a source URI,
read a media file, or imply that the binary fingerprint was observed.

The proposed private `BoundReviewV1` is constructible only after bounded context
validation, composition import and successful OTIO compilation. Its fields are
private; it has no public unchecked constructor, `Default`, `Deserialize`, or
mutable accessors. Immutable accessors may expose digests and owned output bytes.
The CLI's serialized `ReviewRecordV1` is an untrusted strict DTO, not that token;
parsing a record can never recreate `BoundReviewV1`, `VerifiedBufferV1`, an
`ApprovedPlan`, or a capability.

`check-review <REVIEW_RECORD> <CURRENT_COMPOSITION|-> <CURRENT_CONTEXT_FILE>`
must re-import and recompile the explicitly supplied current composition before
comparing it. A review record and current context alone cannot prove that the
current source still compiles to the same artifact. It returns stable stale-field
codes, not an approval result. Both commands are read-only, never invoke a host,
and emit `scope: "declared-review-binding-only"`, `executable: false`,
`approval_granted: false`, `source_verified: false`, and `host_verified: false`.
The existing CLI permits at most one stdin input; these proposed forms use stdin
for the composition only.

## Proposed identity encoding

The default proposal is a typed, versioned SHA-256 identity with domain
`storytelling.finite-otio-review-binding.v1`. Encode the identity payload as a
fixed-field typed DTO, then hash a fixed domain prefix, a big-endian payload
length, and the compact serialized payload. Do not concatenate free-form fields
without framing. The payload binds:

- exact SHA-256 of the input composition bytes, so even a byte-level source change
  invalidates the review;
- SHA-256 of the importer's validated `PlanDocumentV1` serialized from its fixed
  struct fields, preserving asset and shot array order;
- exact SHA-256 of the compiled OTIO JSON bytes produced internally;
- SHA-256 of the complete typed compile report, including profile, clock, choices,
  feature dispositions and limitations;
- compiler/contract/schema/profile versions; and
- a context digest plus fixed per-field context digests, allowing stale-field
  names to be reported without echoing project values.

The raw composition digest deliberately preserves exact input bytes, not merely
semantic JSON equivalence. Typed identity payloads contain no arbitrary JSON map
or floating-point number. IDs are bounded ASCII and are not Unicode-normalized;
other strings are bound as their exact UTF-8 bytes. Semantic arrays preserve
order. Only fields explicitly defined as sets, such as required capabilities and
loss-choice IDs, are sorted by ASCII bytes and reject duplicates. Whether this
encoding and the current `serde_json::Value` output are stable enough across
supported platforms/toolchain updates is a G2 decision backed by cross-platform
golden tests, not an assumption.

The binding hash is an identity, not a secret or credential. Portable JSON can be
copied or replayed. The hash alone is not an approval, signature, grant, freshness
observation, source verification, or authorization decision.

The following requirement IDs are proposed for review and are not yet accepted:

| ID | Proposed outcome |
|---|---|
| M3-RB01 | Typed, versioned deterministic identity binds exact source bytes, normalized plan, compiled OTIO/report bytes, versions and declared context. |
| M3-RB02 | The opaque builder derives the artifact only from the same validated composition bytes; caller-provided artifacts/digests are rejected as proof. |
| M3-RB03 | Strict bounded context validation and declared-context field freshness; host data stays declared, not observed. |
| M3-RB04 | Validated review identity has an opaque construction boundary; serialized records are untrusted DTOs and cannot reconstruct it. |
| M3-RB05 | No self-approval, approval grant, signer authority, executable flag, or capability is produced by matching the identity. |
| M3-RB06 | Changed source/artifact/context is reported stale; trusted time, durable revocation and replay consumption remain out of scope. |
| M3-RB07 | Read-only CLI, stable errors/bounds, source-informed regressions, provenance, local gates and same-head three-OS CI are required for the finite slice. |

## Proposed bounds, validation order and errors

Keep current limits: 1 MiB composition JSON and 4 MiB serialized OTIO output.
Propose a 64 KiB context, 96 KiB review record, 64 capability IDs, 128-byte
ASCII IDs, and 64-level maximum JSON nesting. Context and record use fixed DTOs,
reject unknown, duplicate, null-for-required and malformed fields, and contain no
arbitrary maps or unbounded strings. These new values remain proposals until G2
confirms they cover the existing finite profile and have boundary tests.

Proposed error precedence is deterministic:

1. Argument shape and command-specific stdin rules; usage exits 2.
2. Bounded reads in argument order; I/O exits 3 and never echoes file paths.
3. Per-input byte limits before JSON parsing; oversize is a validation error.
4. Strict review/context DTO parsing and schema/version validation.
5. Import errors, then OTIO compilation/loss-choice errors.
6. Identity serialization and digest construction.
7. For `check-review`, compare fields in schema order and return sorted stale
   field IDs; a changed input is a successful check result with `current: false`,
   not a signature or authority error.

Validation errors retain stable machine codes and JSON field paths, never source
paths, URIs, raw context values, or media contents. Preserve existing CLI exits:
0 for successful report, 1 for invalid input/compilation, 2 for usage, and 3 for
I/O. G2 should confirm whether stale review is a normal `ok: true, current: false`
report or a distinct stable nonzero exit before RED tests freeze it.

## Required functional RED matrix after G2 freeze

Write these tests before implementation and record observed RED, then GREEN:

- Same exact bytes/context/choices produce the same identity on repeated runs and
  across Linux, Windows and macOS. Whitespace-only source change is stale under
  the proposed exact-byte rule.
- Mutate one semantic input at a time: asset order, shot order, repeated
  occurrence, source range, declared source hash, source-snapshot fact, loss
  choice, native/metadata-only disposition, compiled artifact byte, report field,
  compiler/schema/profile version and every context field.
- Prove both compilation linkage and immutability: API does not accept caller
  `CompiledOtioV1`/digest; compile-fail checks reject constructing, mutating,
  defaulting or deserializing `BoundReviewV1`; a serialized review DTO cannot
  recreate the private token.
- Cover each new limit at exactly the maximum and maximum-plus-one; malformed
  digest/ID, duplicate or unknown keys at each level, null required values,
  wrong types, excessive nesting, truncated input and unsupported versions.
- Cover stable error precedence when two inputs are invalid, path-free failures,
  each CLI exit, stdin and explicit-file modes, and byte-for-byte unchanged input
  files. Assert neither command creates files or invokes the host.
- Use complete positive fixtures and one-field negative fixtures. Compare a small
  source-informed oracle for the selected revision/staleness policies, document
  Rust's intentional typed SHA-256 encoding as a delta, and do not claim full TS
  runtime parity. Existing frozen/live 6/8/10/12 comparison lanes remain green.
- Run format, clippy, workspace tests, release tests, docs, provenance/repository
  guard, dependency audit, independent code review, and same-head Linux/Windows/
  macOS CI. These are implementation acceptance gates, not evidence that a host
  was observed or a person authorized the signer.

## Work split and G2 exit conditions

Issue #27 should own the above finite identity, strict context, stale diagnostics,
immutable construction, read-only CLI, source-informed tests, and implementation
evidence. PR #28 should remain responsible for its separate supplied-payload
signature-verification candidate and its own G2 decision. A future trust registry,
signer-role policy, revocation/expiry authority, nonce consumption or durable replay
prevention must have its own authority and storage design; do not fold them into
this pure P1 slice.

Before functional RED, an independent G2 reviewer must resolve or explicitly
accept the proposed identity inputs and canonical encoding, context fields and
declared-only meaning, exact limits and nesting rule, error precedence/stale exit,
opaque type/serialization boundary, source/compiler/profile version scope, and
the separation from PR #28. The review must also confirm the first-profile
importer-to-OTIO path and the unsupported planner bridge. Record reviewer identity,
reviewed commit, findings and disposition without asking Codex to sign or self-
approve. Until then, this packet remains a Draft proposal and the Issue #27 G2
checkbox stays open.
