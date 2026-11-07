# M3-01 next slice: immutable review identity and freshness contracts

Proposed preparation scope; tracking [#27](https://github.com/bynanci/storytelling/issues/27).
This read-only readiness audit on 2026-11-07 performed no product edits, tests,
approval issuance or host execution. M2-03 has since been accepted through PR #26
at main `5533fcb5d0f857ad1066a76505071653b66d1f89`; the earlier inspection identities
below are retained for traceability. Proposed APIs/requirements here are not
implemented, frozen or accepted capabilities.

Merged candidate [PR #28](https://github.com/bynanci/storytelling/pull/28) proposes
a separate cryptographic identity API (M3-A01–A03); it is integrated at main
commit `5a5702ba4c1392d763929f89da3cf7d16d5f13f4` but remains unaccepted while independent G2 review is pending.
Final head `2d23b21dcfbfc49c3d76087f054cc76724c23053` passed Quality, Required, Linux, Windows, macOS and hosted
Cargo Audit in both runs `42378356866` and `42378352463`; the merge commit passed
all six main gates in run `42378550750`. Audit scanned the 40-package lockfile
against 1,422 advisories with no findings. This does not complete canonical
M3-01/P1. This readiness proposal uses distinct M3-RB01–RB07 IDs; evaluate
compilation linkage and freeze that contract only after independent review.

## 2026-11-07 implementation continuation

After the separate G2 review packet was prepared, the maintainer authorized a provisional
implementation candidate on an isolated branch. The candidate now adds same-byte finite importer /
OTIO binding, strict declared context, read-only freshness checks, CLI commands and synthetic tests;
see [M3-01/P1 validation record](records/M3-01-P1.md), [provisional contract](../CONTRACTS.md#m3-01-p1-provisional-review-binding),
and [work index](m3-01-p1-work-index.json). The implementation uses existing source SHA-256 and
adds no production dependency. The first local RED preceded implementation; its baseline was the
available local commit `1d8d0e3652aa2afe77375550876639b726928eba`, which predates the latest remote
main/merged approval candidate. Exact-base compatibility and multi-platform behavior therefore
require hosted checks on the stacked implementation PR.

The candidate preserves all four authority/evidence flags as false. The serialized record is
untrusted, has no signature/trust-key/revocation/replay semantics, and its context does not observe
the host. G2 has not frozen or accepted these provisional names, field encoding or budgets. The
planner→OTIO bridge remains absent, so binding covers only composition bytes passed directly to both
importer and compiler. No issue acceptance checkbox or completion state is claimed; the tracking
issue remains open until same-head hosted evidence, independent G2 review and maintainer acceptance.

## Observed state and readiness

## Recommended single PR boundary

Build **M3-01/P1 immutable finite-OTIO review binding**, explicitly a bounded
subset of M3-01 / OF-03 / OF-25 / OF-36 / GAP-03/05/06/07. Do not mark the whole
OF-25 durable acceptance/replay or full GAP-06 trusted approval complete.

Use the existing finite OpenFilm composition snapshot as the first input. Invoke
the actual importer and `export_openfilm_otio` internally from the *same*
validated bytes and explicit choices; own the exact compiled bytes/report in a
private, immutable result. This prevents callers constructing or mutating the
currently public `CompiledOtioV1 { document, report }` from acquiring a validated
identity. No fourth crate, Node/SQLite/host calls or new production dependency.

The [G2 review packet](M3-01-G2-REVIEW-PACKET.md) now gives a concrete proposed
identity, context, bounds, error order and RED matrix; none is frozen or accepted.
Its `check-review` proposal also requires the current composition as input so the
tool can recompile and compare the exact artifact. Proposed commands are
`prepare-review <COMPOSITION|-> <CONTEXT_FILE>` and
`check-review <REVIEW_RECORD> <CURRENT_COMPOSITION|-> <CURRENT_CONTEXT_FILE>`.
They read only explicit bounded inputs, never resolve project URIs or invoke a
host. Successful output remains `executable:false`, `approval_granted:false`,
`source_verified:false`, `host_verified:false`, with an explicit
`declared-review-binding-only` scope. A matching declared context is not a permit.

First profile is the already admitted finite OTIO composition domain. A declared
host context binds the expected FilmCraft/version/profile/binary fingerprint and
new-isolated-project policy, but cannot claim a fresh host observation. Retain
the generic format report's honest scope; do not relabel arbitrary common clocks
as supported by the actual 25 fps host proof.

## Concrete requirement/test outcomes for G2

Suggested new requirement IDs M3-RB01–RB07 preserve existing M3-R01/R02 IDs. Freeze
exact signatures, wire fields, error precedence and budgets before functional RED.

1. **M3-RB01: typed deterministic identity.** Domain-separated versioned SHA-256
   commits to normalized validated plan, source asset/occurrence identities,
   compiler/schema/profile versions, exact OTIO bytes and complete loss report.
   Preserve every semantic array order. Use existing core SHA2 via a pure helper;
   engine already owns serde_json. Fixed ASCII DTO field keys, normalized exact
   rational strings and unchanged UTF-8 string values avoid arbitrary JSON/JCS
   ambiguity. Freeze Unicode/no-NFC and byte-versus-semantic digest rules.
2. **M3-RB02: exact compilation linkage.** Recompile from the reviewed snapshot;
   never accept a caller's artifact/report digest as proof of compilation. One
   change to shot order/range, source hash, artifact bytes, profile/compiler
   version, native/baked/metadata-only disposition or explicit loss choice must
   invalidate the binding. Bind source snapshot revision separately from plan
   identity so changed retained facts are not silently ignored.
3. **M3-RB03: strict declared context.** Bind project ID/base revision, expected
   target snapshot fingerprint, exact required capabilities and accepted loss
   choices. Unknown/null/duplicate fields, malformed versions/digests and excess
   bytes/items/depth reject before hashing. Keep existing 1 MiB composition input,
   4 MiB compiled-output and 16 MiB supplied-buffer limits; give new context and
   report their own small explicit limits rather than increasing old limits.
4. **M3-RB04: immutable domain boundary.** Construct only through successful native
   validation/compilation; private fields, immutable byte access and no
   Deserialize/Default/public unchecked constructor on any validated result.
   Serialized review records remain untrusted DTOs and can never reconstruct an
   `ApprovedPlan` or `VerifiedBufferV1`.
5. **M3-RB05: no self-approval.** Reject caller `confirmed:true`, `approved:true`,
   actor labels, MCP annotations or imported receipt-shaped fields as unknown/
   unsupported input; do not expose approve/execute commands. Identity matching
   must never return a capability/token or set authority flags. This is a code
   boundary test, not evidence that a trusted human approved anything.
6. **M3-RB06: freshness and revocation semantics.** Compare the exact current
   context against the bound context, including one-field mutations of source,
   proposal, compiled artifact, target revision, capability/profile and losses.
   Expiry/revocation can be pure *declared-state* evaluation only, with explicit
   caller-supplied time/state. Do not claim trusted time, durable revocation,
   request consumption or replay safety until an authoritative runtime exists.
   A request ID reused with another fingerprint must be distinguishable; durable
   enforcement remains M3-02.
7. **M3-RB07: usable honest CLI/evidence.** Explicit file/stdin input, path-free
   errors, stable validation/usage/I/O exits and unchanged inputs. Use complete
   positive fixtures and mutate one semantic field per negative test. Functional
   RED precedes implementation; debug/release, three-platform same-head CI,
   existing 6/8/10/12 comparison lanes and M2-03 checks stay green. Freeze a small
   source-informed TS identity/staleness oracle; classify changed Rust digest
   encoding as intentional delta, not false byte parity or a full runtime port.

## Exact pinned source map

All paths below are relative to `packages/` at source SHA above. Existing source
provides review/revision/request guards, **not** a generic FilmCraft ApprovedPlan
contract. These symbols and blobs were actually inspected, not inferred from the
planning ledger.

| Source path / symbols | Git blob SHA |
|---|---|
| `application/src/silence.ts`: `recordId`, `verifyReviewedProposal` | `f94bb561f1f24936b2c2a05087e6e66746f5b99e` |
| `application/src/editor.ts`: `silenceApplyFingerprint`, `editFingerprint`, `revisionFor`, `commitEdit` | `bb5fc1d38a12fb17cdaf09a01e6bc61aeca3ebd9` |
| `application/src/silence-jobs.ts`: `apply`, `assertReviewBound`, `assertApplyReceipt` | `6d5eba7d86369c096bda584e89466e1b1426d499` |
| `application/src/story-plan.ts`: `acceptPlanInternal`, stale graph/story revision and receipt key | `23cad980b2f8ae29c794800661f6f9e6d638031d` |
| `application/src/scene-script-review.ts`: `evidenceHash`, `context`, `accept` | `11c4bcfa1b29e3094f636787a5010de29b445461` |
| `application/src/nle-handoff.ts`: `snapshot`, `review`, `publish` | `7af8931a969e4eb13abe92b3db0b6b859c494e1c` |
| `exporters/src/handoff.ts`: `createHandoffPlan`, `HandoffPlan` | `debc8308b472372bd60870a6ec9e6d7d15189c66` |
| `core/src/presets.ts`: `canonicalPresetJson` | `7b8213dca3c4773800d9d2168e81daff588149a0` |
| `core/src/preset-json.ts`: bounded strict JSON preflight | `d9d232c4255acdd9510153e15a0347c13a4b08fd` |
| `media/src/verified-source.ts`: stronger filesystem snapshot boundary | `f7b8b33445fb2a14394d34544a5b97d6d01f267c` |

Actual source regressions suitable for finite extraction:

| Test file / observed test names or relevant cases | Git blob SHA |
|---|---|
| `application/tests/story-plan.test.ts`: “rejects stale plan when underlying story or graph changes after preview was generated”; “idempotently handles lost-response retry without duplicating timelines”; rejects cycles/unresolved alternatives | `7ac3357de7aa9885f893aece8a9ec3effc2bb5c5` |
| `application/tests/silence-review.test.ts`: altered envelopes/stored payloads/cross-project copies; missing record/stale timeline; same-size source change with restored mtime | `11fbcd692e37db9ef106b8689e3e3b64fcd2cbc2` |
| `application/tests/silence-jobs.test.ts`: receipt retained after acknowledgement failure; changed request reuse; exact replay | `bd383faf764ad78f6059ce4792420865d1aa8823` |
| `application/tests/scene-script-receipt-validation.test.ts`: “recovers both distinct request IDs from the same restored project” | `314462ea008cdf83a98d370121d5cdc3ff921aab` |
| `core/tests/presets.test.ts`: “defines stable canonical content and scoped IDs” (`-0→0`, ordered arrays, undefined rejection) | `7fd11747efaa54c22b51a2a6df16606a55411153` |

These tests were read, not executed in this audit. Durable replay, filesystem
race protection and database writes are source-reference evidence for later
runtime tasks, not acceptance claims for the proposed pure slice.

## Actual gaps and risks

- **Planner/compiler bridge is absent.** Current CLI documentation explicitly
  says `planning.plan` is M0 input, not composition export, and cannot directly
  feed `export-otio`. This recommended first slice reviews importer-derived plans
  and the same snapshot's compiled OTIO. If M3-01 must review `plan-story` output,
  first implement/review an explicit source-binding + planner→composition/compiler
  bridge and test order/occurrence/ranges; hashing two unrelated artifacts is not
  proof that one compiled from the other. Do not silently claim this integration.
- **No single source canonical hash algorithm.** Story-plan's local `digest` is a
  custom UTF-16 code-unit rolling hash, not SHA-256; canonicalPresetJson sorts JS
  keys; editor uses localeCompare; other services hash plain JSON.stringify.
  Native typed/versioned/domain-separated encoding is an intentional change.
- **Trust is missing for actual grants, not for development.** No trusted
  human-issuance channel/credential, authoritative revocation/expiry/request
  store or approval consumption exists. Actor strings/signatures supplied in
  portable JSON cannot supply that authority. Do not invent a user confirmation
  gate before writing the contracts, and do not issue a fake grant to complete a
  checkbox. Keep real trusted approval issuance as an explicitly unimplemented
  follow-up, with M3-R01 still broader than this slice.
- **Host/source authority remains limited.** M1-03's proof is historical and
  synthetic; its external mapping is not a current target snapshot or atomic
  CAS. Private supplied-buffer verification does not cover later filesystem
  reads. No in-place host updates or read-before-write CAS claim. New isolated
  target projects remain the first executor policy.
- **Loss acknowledgement is exact.** Metadata-only lock/pin disposition is not
  native protection, generic feature “accepted” is not human consent, and passing
  a known 25 fps example does not certify arbitrary content clocks.
- **Durability stays M3-02.** No SQLite, journal, owner fencing, one-use-token,
  timeout/reconciliation, exactly-once or durable receipt/replay promise in P1.

## Delivery obligations and next steps

Read current AGENTS/TASKS/SDLC and runbook before implementation; obtain independent
G2 contract review, not another user permission prompt. Preserve the original MIT
LICENSE and approved Apache-2.0 component policy. Add task-specific provenance,
comparison record, requirement/work index and actual source/oracle evidence;
keep the aggregate guard and core/engine I/O restrictions. No empty workflow or
adapter crate, no new external runtime or crypto dependency is needed. Freeze
the subset honestly, implement/test/publish under existing authorization, merge
only after same-head gates, and leave full approval/execution/runtime gates open.
