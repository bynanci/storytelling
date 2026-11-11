# Architecture

<a id="架構"></a>

English · [繁體中文](ARCHITECTURE.zh-TW.md)

<a id="選擇入口"></a>

## How projects use Storytelling

A host is an application that loads the Skill or runs an integration. An AI assistant can host a writing session. A video editor can receive a timeline.

| Use case | Input and result | Execution |
| --- | --- | --- |
| Write with an agent | The author supplies notes. The agent drafts a spoken outline, scene, illustration brief, copy or video plan. | The host generates language and supplies enabled tools. The [Skill](../skills/storytelling/SKILL.md) contains examples and workflow instructions. See the [platform guide](PLATFORMS.md). |
| Automate a repeatable check | An application supplies a supported JSON document or source bytes. The CLI returns a result and exit code. | The local CLI checks plans and briefs, orders fixed windows, renders narrative drafts and queries revision impact. See the [CLI guide](CLI_GUIDE.md) and [contracts](CONTRACTS.md). |

Video import, OTIO export and review each require a supported composition snapshot. Any project can produce the format when it has the required source facts. Host compatibility depends on the tested version, platform and case.

<a id="影片功能驗收範圍"></a>

## Video implementation status

| Milestone | Accepted scope |
| --- | --- |
| M0, M1-01, M1-02 | Declared metadata checks, finite snapshots and picture-only timelines. See the [M0 contract](CONTRACTS.md) and [snapshot contract](migration/FINITE_V1.md). |
| M1-03 | Supplied-byte checks and one isolated host test. See the [test record](migration/records/M1-03.md). |
| M2-01, M2-02 | Bounded brief audit and fixed-window planning. |
| M3-01/P2, M3-02 | Read-only OTIO review and the SQLite journal from PR #56. The journal cannot execute host operations. |

PR #28 merged. Its candidate still needs an independent G2 decision. The P1 acceptance covers a separate contract.

<a id="模組與依賴方向"></a>

## Implemented dependency direction

| Module | Responsibility | Dependency boundary |
| --- | --- | --- |
| CLI | Reads local data and writes results and exit codes. | Calls the engine. |
| Engine | Checks inputs, maps snapshots, produces narrative output and serializes OTIO. | Uses core and `serde_json`. Has no filesystem, clock, random state, database, network or editor dependency. |
| Core | Defines strict wire types, rational time and typed errors. | Uses Serde and the reviewed SHA-2 software implementation. Hashes only caller-supplied buffers. |
| `storytelling-journal` | Stores operation state in bundled SQLite and controls which owner can write. | Keeps database I/O outside core and engine. The CLI does not yet call this adapter. |

`scripts/check_repo.py` checks direct dependencies. Review transitive dependencies and actual behavior separately. First-party Rust code must not use `unsafe`.

<a id="跨媒介敘事與後續擴充"></a>
<a id="future-boundaries-not-empty-crates"></a>

## Cross-media authoring and extensions

<a id="素材呈現方式與修訂"></a>
<a id="proposed-cross-media-authoring-model"></a>

### Source material, variants and revisions

The narrative model separates source material from its presentation. A spoken outline has an order. An illustration has a focal point, foreground and background. Both can reference the same observation.

| Design question | Contract |
| --- | --- |
| What did the author supply? | The brief records audience and purpose. Each unit has an ID, origin and revision. The model distinguishes observations, interpretations and proposed details. |
| What connects units? | Relations express derivation, association and mandatory order. Associations may form cycles. Mandatory order must hold within its declared variant. |
| How can the author change the presentation? | Named variants reference units and revisions. Spoken beats have an order. Illustration elements have spatial roles. A future brainstorm profile could retain alternative branches. |
| What can the software establish? | The engine checks references, constraints and revisions. A citation records a declared basis. Authors judge accuracy and fitness for publication. AI proposals use the same input checks. |

The maintainer accepted the independently versioned narrative contract for N-03 in issue #50 on 2026-11-11. Core defines the types. The engine checks and projects data. The CLI handles local I/O. `render-narrative` emits a spoken outline and illustration briefs from declared text. The N-01 study packet is ready. The human trial and decision to continue, narrow or stop remain pending. See [ADR 0005](adr/0005-independent-narrative-document-v1.md) and [N-03 evidence](validation/n03-narrative-cli.md).

N-04 starts with a `NarrativeDocumentV1` snapshot. A separate `NarrativeRevisionLedgerV1` stores append-only events. `track-narrative-revisions` replays source and variant history, declared author choices and pending reviews. `query-narrative-impact` accepts a source revision event ID. The engine reports variants with direct references. An author must inspect indirect effects through `based_on`. See the [ledger contract](narrative/NARRATIVE_REVISION_LEDGER_V1.md) and [N-04 evidence](validation/n04-narrative-revisions.md).

N-09 keeps `NarrativeDocumentV1` closed and adds a separate `NarrativeCueSheetV1` for a recording-ready sequence. It binds narration to a spoken beat and source revision, labels production directions, distinguishes known, estimated and unknown time, and checks resolved intervals on exclusive tracks. `render-cue-sheet` emits deterministic text; it does not record or mix audio. See [ADR 0007](adr/0007-versioned-narrative-cue-sheet.md), the [profile](narrative/NARRATIVE_CUE_SHEET_V1.md) and [N-09 evidence](validation/n09-cue-sheet.md).

N-11 adds a separate `NarrativeHandoffManifestV1` around the unchanged ledger. Core owns its wire types; the pure engine binds current source descriptors and complete artifact-component maps to actual variant snapshots. It reuses N-04 direct-reference impact, checks proposal scope across the whole bound snapshot, and keeps historical impact separate from current review status. The CLI reads one explicit manifest and writes reproducible text. File paths and hashes are declarations; no media is opened. See [ADR 0009](adr/0009-read-only-mixed-media-handoff.md), the [profile](narrative/NARRATIVE_HANDOFF_MANIFEST_V1.md) and [evidence](validation/n11-handoff.md).

Narrative inputs need no asset hashes, source ranges or frame rates. A future video adapter could bind selected units to explicit assets and call the planner through its public contract. Keep video V1 inputs, CLI behavior, digest vectors and OTIO fixtures as regression checks.

For origin and derivation, consult [W3C PROV-DM](https://www.w3.org/TR/prov-dm/). For spatial grouping, consult [JSON Canvas 1.0](https://jsoncanvas.org/spec/1.0/). Use synthetic cases to assess implementation cost. Record the decision in the ADR specified by the [narrative backlog](../TASKS.md#cross-media-narrative-backlog).

<a id="工作流程與宿主轉接器"></a>

### Workflow and editor adapters

The planned dependency direction has three parts.

- The workflow uses core and engine to define use cases and ports.
- Adapters use the workflow ports and core. They cover databases, snapshots, OTIO, editors and optional services.
- The CLI composes the workflow and adapters. Application and engine layers retain planning and approval policy.

The SQLite journal can operate independently. The full workflow, production executor and application adapters remain planned. Add a crate when it delivers executable functionality. A TypeScript application can run the CLI as a child process and use its JSON protocol. A packaged SDK and other transports belong to later milestones.

<a id="資料責任"></a>
<a id="authority"></a>

## Data ownership

| Data | Owner |
| --- | --- |
| Story intent, constraints, selected material, proposals and decisions | Storytelling |
| Source bytes, file access and media analysis | Authorized source tool or adapter |
| Assets, compositions, catalogs and history in a source project | Source project |
| Timeline and finishing after handoff | Target editor |
| Mapping, approval, execution and verification records | Storytelling workflow |

Plans are immutable proposal inputs. M0 rejects unsupported fields. Its model excludes images, tracks, speed, audio automation and host effects. M1-01 maps the documented composition profile into M0. It retains source facts and structured limitations.

M1-02 serializes supported picture content as native OTIO. It uses one bounded integer clock for content time. The caller explicitly accepts picture-only output or locks in metadata where required. Source facts remain available. Project frame rate stays separate. Only tests use the official Python parser.

M1-03 separately checks supplied bytes and tests one synthetic Linux FilmCraft profile at 25 fps with CPU preview. A production host executor remains future work. See the [M1-03 evidence](migration/records/M1-03.md).

M2-01 checks mandatory membership and order. M2-02 selects fixed windows. A general solver remains planned.

<a id="時間與身分"></a>

## Time and identity

M0 represents time as a reduced, nonnegative fraction with numerator and denominator encoded as `u64` strings. Valid `RationalTime` values compare through `u128` cross-products. This avoids floating-point rounding. Intervals use half-open ranges, `[start,end)`. The system does not infer a denominator or snap to frames or samples.

Keep source, timeline, frame and sample time types separate. Report any quantization by the host. Keep decimal 29.97 distinct from 30000/1001.

Assets and shot occurrences have distinct IDs. Two shots can reference one asset. A SHA string declares a value. M1-03 checks supplied bytes and retains a private immutable buffer. Later filesystem reads require separate identity checks in the adapter. The portable M0 document rejects paths, URLs, commands, model credentials and arbitrary extension fields.

<a id="執行安全與復原"></a>
<a id="future-execution-safety"></a>

## Execution safety and recovery

The compiled plan must bind the proposal, actual sources, compiler, host capabilities and accepted losses. Human approval requires a trusted interface. A model's `confirmed:true` cannot grant execution authority. Start in a new isolated host project.

Treat an external timeout as an unknown outcome. Read back host state before attempting recovery. Never retry blindly or repeatedly undo changes. SQLite cannot make external host effects transactional. Execution receipts distinguish success, failure, partial completion and unknown results. Record planning, job state and freshness separately.

The journal stores review binding, exact artifact digests, attempt receipts, executor ID and increasing fence epochs. `AppliedReported` records an executor report. `AppliedObserved` records an observation. An expired attempt in flight recovers as unknown. Fencing protects journal updates only.

M3-03 must establish that the prior request stopped before readback can support a retry. The pinned Linux test reproduces a lost acknowledgement after FilmCraft exits successfully. The runner reads back the isolated project. A hung host and production execution still need tests. See the [M3-03 record](validation/m3-03-filmcraft-execution.md).

<a id="架構決策紀錄"></a>
<a id="decisions"></a>

## Architecture decisions

- [ADR 0001](adr/0001-scope-and-boundaries.md) defines the Rust extraction strategy.
- [ADR 0002](adr/0002-exact-time-and-wire.md) defines time and wire constraints.
- [ADR 0003](adr/0003-license-and-provenance.md) records repository licensing and source attribution.
- [ADR 0007](adr/0007-versioned-narrative-cue-sheet.md) records the separate closed audio cue-sheet contract.
- [ADR 0008](adr/0008-local-png-host-feasibility.md) proposes the N-12 local PNG delivery boundary with separate host and readback evidence.
- [ADR 0009](adr/0009-read-only-mixed-media-handoff.md) proposes a read-only handoff manifest using the existing revision ledger.

<a id="固定時間窗規劃器"></a>

## Accepted fixed-window planner

M2-02 reuses the strict brief audit. The planner admits required candidates first. It ranks optional candidates deterministically and applies topological ordering to exact source windows. Core defines declared types. The engine's planner and clock modules emit an M0 plan with reason paths. This feature adds no production dependencies or execution authority. Full multi-beat solving remains outside the profile.

<a id="pr-28-的核准身分候選方案"></a>
<a id="merged-pr-28-approval-identity-candidate-not-accepted"></a>

## PR 28 approval identity candidate

PR #28 merged and still needs independent G2 review. The accepted M3-01/P1 profile binds review to finite OTIO. The two contracts have separate acceptance records. See the [P1 scope](migration/M3-01-NEXT.md).

`storytelling-core::approval` provides a pure contract and verifier. Source identity comes only from an existing private `VerifiedBufferV1`. It hashes exact proposal and compiled artifact bytes separately. It records declared host profile, version, capabilities and accepted losses. Parsed input remains untrusted until structural checks pass and its complete identity matches the locally built token.

The external signer and trusted key registry belong outside core. The caller supplies the public key binding. Strict Ed25519 verification establishes control of that key over the exact domain-separated payload. Human identity and authorization require separate evidence. `HostBindingV1.observation` binds a caller's claim. It cannot establish an independent host observation. The opaque `VerifiedApprovalV1` cannot become a serialized bearer token.

The candidate excludes key storage, replay journals, expiry, revocation, an approval database, CLI signing and host execution. Future execution needs durable replay protection, owner fencing and actual host evidence. See the [approval identity contract](migration/APPROVAL_IDENTITY_V1.md) for field order, canonical encoding and size limits.
