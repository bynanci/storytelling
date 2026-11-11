# Port an external component

<a name="外部元件移植指南"></a>

English · [繁體中文](MIGRATION.zh-TW.md)

Use this guide to integrate a component or trace its origin. To install the writing skill, see [platforms](PLATFORMS.md). To process a local JSON document, see the [CLI guide](CLI_GUIDE.md).

M0 used upstream design concepts. M1-01 introduced a source-informed importer for a limited visual snapshot. The port does not copy a user's database or media. Original work uses MIT. The applicable external components keep their Apache-2.0 license and attribution. See the [snapshot contract](migration/FINITE_V1.md), [comparison](migration/records/M1-01.md) and [file inventory](migration/provenance-m1-01.json).

<a name="能力與資料責任"></a>

## Extraction map

| Capability | Current boundary | Remaining work |
| --- | --- | --- |
| Evidence and identity | Record declared references. Check the SHA-256 digest of supplied bytes within the documented limit. | File identity, changes between checks and use, and broader adapter evidence. |
| Plans and constraints | M2-01 audits a declared brief. M2-02 plans fixed windows. | Freeze a profile before extending it. Classify differences from the source. The full solver remains open. |
| Catalog and history | The source application owns its data. The importer reads a snapshot. | Keep direct writes to its SQLite database outside the adapter. |
| Review and recovery | PR #56 delivered the journal and its single-owner constraints. M3-03 tested readback after a lost reply on Linux. | Trusted authority, production execution, requests still in flight and wider host coverage. |
| Interchange and host work | The limited writer produces a picture-only OTIO document. One isolated host test used 25 fps and CPU rendering. | Real-media listening and other hosts need separate evidence. Use public protocols or files. Pin and test the editor version. The core has no editor dependency or fork. |
| Interfaces and presentation | Consumers and adapters provide FFmpeg, rendering, media analysis and user interfaces. | Use adapters for external tools. Assess each extraction separately. |

<a name="four-stages"></a>
<a name="移植步驟"></a>

## Migration sequence

1. **Freeze the baseline.** Record the upstream SHA, file origins and licenses. Save synthetic inputs, expected behavior, failure cases and limits.
2. **Compare implementations.** Give TypeScript and Rust the same snapshot. Compare references, ranges, hard constraints and errors. Classify each difference as a port defect, upstream defect, unsupported case or intentional contract change.
3. **Switch one service.** Use a feature flag. Give write authority to one component. Record the protocol version. Reject unsupported versions.
4. **Retire the old implementation.** Test the consumer, interchange and actual host. Keep a rollback path until the replacement passes acceptance.

<a name="相容性"></a>

## Compatibility policy

Document the conversion and any loss when importing legacy floating-point time. Do not infer a frame rate. Import never silently migrates a live project. Metadata remains a declaration until verification checks actual bytes.

Version the wire schema, Rust API, planner, compiler and adapter profiles separately. Use a new project for the first host operation. Version 0.x makes no promise of general two-way sync.

<a name="檢查授權"></a>

## Licensing checkpoint

1. Identify the source license and contributors.
2. Preserve applicable notices and document your changes.
3. Retain the component license for source-derived Apache-2.0 work, including a port into another language.
4. Refer changes to the licensing policy to the maintainer and legal reviewer.

<a name="對照來源與工作項目"></a>

## Source-mapped execution inventory

The [48-item map](migration/OPENFILM_RUST_PORT_MAP.md) groups work by runtime responsibility. Its upstream baseline is `7add4762fe8174d76f43b863375ea39b73121e76`. The [16 gaps](migration/NATIVE_BOUNDARIES_AND_GAPS.md) cover comparison, authority, host effects, distribution and human acceptance. Read the relevant source symbols and tests before each port. See the [machine ledger](migration/openfilm-port-index.json), [porting procedure](migration/CODEX_PORTING_RUNBOOK.md) and [original plan](superpowers/plans/2026-11-07-openfilm-port-bootstrap.md).

M0 checks declarations. M1-03 verifies a supplied buffer. The [host record](migration/records/M1-03.md) covers a synthetic Linux 25 fps FilmCraft CPU test. Filesystem identity, general execution and editorial acceptance remain separate.

<a name="pr-28-的核准契約"></a>

## Merged PR #28 approval identity candidate

PR #28 merged. Independent G2 acceptance remains open. The candidate binds supplied identities, host declarations, selected losses and existing buffer evidence. It verifies an Ed25519 signature. It does not sign, store keys, grant human authority, persist approvals or execute a host operation.

P1 has a separate, limited OTIO review contract. M3-02 supplies bounded journal recovery. M3-03 adds an isolated Linux proof. Production execution and full timeout recovery remain open. See [P1 scope](migration/M3-01-NEXT.md), [approval contract](migration/APPROVAL_IDENTITY_V1.md), [candidate record](migration/records/M3-01-approval-candidate.md) and [dependency review](migration/dependency-review-pr28-approval-candidate.md).
