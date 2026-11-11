# Architecture

## How projects use Storytelling

Storytelling has two entry points that can be adopted separately.

- **Authoring in an agent.** A host reads the portable [Skill package](../skills/storytelling/SKILL.md) and uses the author's notes to draft a talk, a scene, an illustration brief, copy or a video outline. The package carries its own examples and workflow rules. The host provides language generation and any enabled tools. See [installation by platform](PLATFORMS.md) for project and personal use.
- **Local automation.** An application or script can invoke the Rust CLI directly, pass a supported JSON document or source bytes and inspect stdout and the exit code. Any producer can supply the declared plan to `validate`, the brief to `audit-brief` or the candidate windows and constraints to `plan-story`. These operations run independently of an AI service and editor. See the [CLI guide](CLI_GUIDE.md) and [contracts](CONTRACTS.md).

OpenFilm is one producer of video composition snapshots. Import, OTIO export and the associated review commands consume its finite snapshot format. Another project can construct the same format when it has those source facts. FilmCraft is the editor used in the bounded interoperability tests. Each additional editing host needs its own compatibility evidence.

## Video implementation status

Status: accepted M0, finite M1-01/M1-02, M2-01 bounded audit, M2-02 fixed-window
planner, M1-03 supplied-byte verification/isolated host proof, M3-01/P2
read-only OTIO bridge and the accepted bounded M3-02 SQLite journal (PR #56).
The journal has no host execution path. PR #28 is merged but not accepted;
independent G2 review remains pending.
See [M0 contract](CONTRACTS.md) and [finite-v1 contract](migration/FINITE_V1.md).

## Implemented dependency direction

```text
CLI: local bytes / stdout / exit codes
  -> engine: bounded validation + finite snapshot mapping + pure OTIO serialization
    -> core: strict DTOs + rational source time + typed errors

SQLite journal adapter: durable operation state + owner fencing (no host client)
```

Core uses Serde and the task-reviewed SHA2 software implementation for bounded
caller-supplied buffers. Engine adds serde_json but has no filesystem, clocks, random
state, database, networking or editor dependency. CLI owns I/O. `scripts/check_repo.py`
checks direct dependency boundaries; review must also examine transitive dependencies
and behavior. First-party unsafe is forbidden, not a claim about every dependency.
The separate `storytelling-journal` crate is the first native-runtime adapter slice:
it adds bundled SQLite for durable state and keeps database I/O out of core and
engine. CLI does not yet compose or invoke the journal.

## Future boundaries, not empty crates

### Proposed cross-media authoring model

An oral outline has a speaking order. A single illustration has a focal point,
background and foreground. Both can refer to the same observation. The N track
separates source material from the way an author chooses to present it. The N-01
paper packet is ready, but no human author trial or continue/narrow/stop decision
is recorded. The standalone NarrativeDocumentV1 boundary is accepted for N-03 by
maintainer direction in the 2026-11-11 Issue #50 instruction; see [ADR 0005](adr/0005-independent-narrative-document-v1.md).
The human study remains pending and no N-01 outcome is claimed.

| Design question | Proposed responsibility |
| --- | --- |
| What did the author supply? | A brief describes audience and purpose. Each content unit has an ID, origin and revision. A recollection, an interpretation and a suggested detail remain distinguishable. |
| What connects the units? | Relations express declared support, association or required order. A pair of ideas can refer back to each other. Cycle checks apply to hard precedence within an ordered output. |
| How could this be expressed? | A named variant references the units it uses. Spoken beats have an order, drawings have spatial roles, and brainstorms can retain alternative groups. Authors choose the variant and can trace later edits to its referenced revisions. |
| What can software establish? | The engine can validate references, constraints and revision matches. A citation records the supplied basis for a claim. Its accuracy and suitability for publication require human judgment. Optional AI proposals enter through the same untrusted input boundary. |

Start inside the existing crates. Core would carry the separately versioned
narrative types, engine would validate and project them, and CLI would read and
write local documents. Keep narrative inputs independent of asset hashes, source
ranges and frame rates. A later video adapter could bind selected units to
explicit assets and call the existing planner through its public contract.
Keep the V1 video input shapes, CLI behavior, digest vectors and OTIO fixtures as
regressions. The N track starts with discovery and contract design.

For N-02, compare origin and derivation with [W3C PROV-DM](https://www.w3.org/TR/prov-dm/)
and spatial grouping with [JSON Canvas 1.0](https://jsoncanvas.org/spec/1.0/).
Use the synthetic cases to decide which distinctions earn their implementation cost.
Record the decision in the ADR specified by the [narrative backlog](../TASKS.md#cross-media-narrative-backlog).

### Workflow and editor adapters

`workflow -> engine + core`: application use cases and ports.
`adapters -> workflow ports + core`: SQLite, OpenFilm reader, OTIO, FilmCraft and
optional provider implementations. `cli -> workflow + adapters` composes them.
The SQLite journal now has an executable adapter slice; the broader workflow,
FilmCraft executor and OpenFilm adapter remain future work. Add other crates only
with executable slices, not for speculative architecture.
A TypeScript consumer can invoke the current CLI/JSON protocol as a child process.
A packaged SDK and additional transports belong to future milestones. Planning
and approval policy stay in the application and engine layers.

## Authority

| Data | Authority |
| --- | --- |
| Story intent, constraints, curated membership, proposals and decisions | storytelling |
| Source bytes, actual filesystem access and media analysis | authorized producer/adapter |
| A project’s source material and history | The originating project. In the OpenFilm integration, its application services own the Composition, Catalog and history. |
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
The journal persists review-binding and exact artifact digests, attempt receipts,
executor ID and monotonically increasing fence epochs. `AppliedReported` remains
distinct from `AppliedObserved`; expired in-flight attempts recover as unknown.
Fencing protects journal updates only. M3-03 must establish old-request quiescence
before treating a host readback as permission to retry.

A pinned Linux test harness now covers one lost acknowledgement after FilmCraft
has exited successfully, then reconciles the isolated native project. It does
not establish quiescence for a hung host process or make the test-only harness a
product executor; see the [M3-03 validation record](validation/m3-03-filmcraft-execution.md).

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

## Merged PR #28 approval-identity candidate (not accepted)

This candidate is distinct from the accepted bounded M3-01/P1 finite-OTIO
review-binding profile in [M3-01-NEXT](migration/M3-01-NEXT.md). It does not
satisfy that profile, and PR #28 remains unaccepted pending its own G2 review.

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
approval database, CLI signing path, or Host execution in this candidate.
Future durability and Host-execution tasks must add durable replay/owner fencing
and obtain actual Host evidence before execution. Bounds and canonical field/order rules are in the
[contract](migration/APPROVAL_IDENTITY_V1.md).
