# Architecture

Status: accepted M0, finite M1-01/M1-02, M2-01 bounded audit, M2-02 fixed-window
planner and M1-03 supplied-byte verification/isolated host proof; M3-01 approval
identity implementation is in progress and remains unaccepted pending independent review.
See [M0 contract](CONTRACTS.md) and [finite-v1 contract](migration/FINITE_V1.md).

## Implemented dependency direction

```text
CLI: local bytes / stdout / exit codes
  -> engine: bounded validation + finite snapshot mapping + pure OTIO serialization
    -> core: strict DTOs + rational source time + typed errors
```

Core uses Serde and the task-reviewed SHA2 software implementation for bounded
caller-supplied buffers. Engine adds serde_json but has no filesystem, clocks, random
state, database, networking or editor dependency. CLI owns I/O. `scripts/check_repo.py`
checks direct dependency boundaries; review must also examine transitive dependencies
and behavior. First-party unsafe is forbidden, not a claim about every dependency.

## Future boundaries, not empty crates

`workflow -> engine + core`: application use cases and ports.
`adapters -> workflow ports + core`: SQLite, OpenFilm reader, OTIO, FilmCraft and
optional provider implementations. `cli -> workflow + adapters` composes them.
Add these crates only with an executable slice, not for speculative architecture.
TypeScript initially wraps the CLI/JSON protocol. MCP, WASM and HTTP are transports;
no transport owns planning or approval policy.

## Authority

| Data | Authority |
| --- | --- |
| Story intent, constraints, curated membership, proposals and decisions | storytelling |
| Source bytes, actual filesystem access and media analysis | authorized producer/adapter |
| OpenFilm Composition, Catalog and history | OpenFilm application services |
| Host timeline after handoff and user finishing | target editor |
| Mapping/approval/execution/verification records | storytelling workflow |

Plan documents are immutable proposal inputs, not a second editable timeline.
The M0 subset does not model images, tracks, speed, audio automation or host effects;
unsupported fields are refused. M1-01 separately maps a strict composition-export
profile into this subset, retaining complete source facts and structured limitations.
It is not general OpenFilm-project, media or host compatibility.

M1-02 serializes the strict visual profile to native OTIO with one bounded integer
content clock, explicit picture-only/metadata-only-lock choices, and retained
source facts. Official parser Python is test-only. Project FPS remains separate;
the accepted [M1-03 proof](migration/records/M1-03.md) separately verifies one
synthetic FilmCraft Linux 25 fps CPU profile and supplied source bytes. No production
host executor is added. M2-01 audits declared hard membership/order conflicts;
M2-02 now supplies fixed-window selection; general full-solver feasibility remains future work.

## Time and identity

M0 time uses reduced nonnegative u64 numerator/denominator strings. Private valid
`RationalTime` values compare through u128 cross-products, without float rounding.
Ranges are half-open `[start,end)`. No FPS/sample snapping or denominator guessing.
Future source/timeline/frame/sample wrappers must remain distinct; host quantization
must produce an explicit report. Decimal 29.97 is not silently 30000/1001.

Asset identity and shot occurrence identity are distinct. Reusing one asset twice is
valid. A SHA string is a declaration only. M1-03 separately verifies explicitly
supplied bytes into a private owned immutable buffer, without proving later
filesystem reads. Strong filesystem identity remains future adapter work.
M0 accepts no paths, URLs, commands, model credentials or arbitrary extension
fields inside the portable document.

## Future execution safety

A compiled plan must bind the proposal, actual sources, compiler, host capabilities
and explicit loss choices. Human approval comes from a trusted surface, not an LLM
`confirmed:true`. Start in a new isolated host project. External timeout becomes
unknown -> readback/reconcile, never blind retry or Undo N times. SQLite cannot make
host effects transactional. Execution receipts distinguish succeeded, failed,
partial and unknown; planning, job lifecycle and freshness stay separate.

## Decisions

[ADR 0001](adr/0001-scope-and-boundaries.md) records the Rust extraction strategy;
[ADR 0002](adr/0002-exact-time-and-wire.md) records time and wire constraints;
[ADR 0003](adr/0003-license-and-provenance.md) preserves the actual repository license.

## Accepted fixed-window planner

M2-02 reuses the strict brief audit, admits every required candidate first, applies
deterministic optional ranks, and topologically orders exact fixed windows. Core
owns declared types; pure engine planner/clock modules emit an actual M0-valid plan
with reason paths. It adds no production dependencies or execution authority.
Full multi-beat solving remains outside this profile.

## M3-01 approval trust boundary (implementation in progress)

`storytelling-core::approval` is a pure contract/verifier. It derives source
identity only from the existing private `VerifiedBufferV1`; it hashes the exact
proposal and compiled-artifact byte sequences and snapshots declared Host
profile/version/capabilities plus accepted losses. A parsed wire identity is
untrusted until structurally validated and compared in full against the locally
built identity token.

The external signer and trusted principal/key registry are deliberately outside
core. The caller supplies a public-key binding; Ed25519 strict verification proves
control of that key over a domain-separated payload, not human identity or
authorization. `HostBindingV1.observation` is a bound caller claim, not an
independent Host observation. The opaque `VerifiedApprovalV1` is not a serialized
bearer token. There is no key storage, replay ledger, expiry/revocation service,
approval database, CLI signing path, or Host execution in M3-01. M3-02 must add
durable replay/owner fencing; M3-03 must obtain actual Host evidence before it
can execute. Bounds and canonical field/order rules are in the
[contract](migration/APPROVAL_IDENTITY_V1.md).
