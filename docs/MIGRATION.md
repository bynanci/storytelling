# OpenFilm extraction and migration

M0 imported design intent only. M1-01 now adds a source-informed, separately licensed
finite visual importer; see its [contract](migration/FINITE_V1.md),
[comparison record](migration/records/M1-01.md) and [provenance](migration/provenance-m1-01.json).
No project database or media is copied. Original MIT LICENSE is preserved, with
Apache-2.0 component attribution and applicable notices retained.

## Extraction map

| Capability | Accepted current scope | Later migration condition |
| --- | --- | --- |
| Evidence/source identity concepts | Declared references plus explicit bounded supplied-buffer SHA-256 verification | Filesystem identity/TOCTOU and stronger adapter provenance remain future work |
| StoryPlan/constraints/planner | Accepted M2-01 audit and M2-02 fixed-window declared plans; full solver remains open | Extend only through frozen source-aware profiles and classified differential evidence |
| OpenFilm Catalog/Composition/history | Stay owned by OpenFilm | Read-only snapshot importer; never write its SQLite directly |
| Review/Jobs/owner recovery | Bounded owner-fenced SQLite journal candidate; no host executor | Add FilmCraft workflow only with fault-tested isolated-project/readback slice |
| FFmpeg/render/GUI/media analysis | Remain outside pure core | Consumer or adapter supplies results |
| FilmCraft | No crate dependency or fork | Public protocol/file handoff with a pinned tested host |

## Four stages

1. **Freeze**: record upstream SHA, per-file origin/license, exact inputs, expected
   invariants, valid/invalid fixtures and supported scope. No private footage.
2. **Shadow**: TS and Rust consume the same frozen snapshot. Compare references,
   ranges, hard constraints and errors. Classify differences as port bug, prior bug,
   unsupported case or deliberate contract change; do not copy bugs as truth.
3. **Feature flag**: switch one OpenFilm service to the Rust boundary. Keep one
   authoritative writer. Version the protocol and reject unsupported newer schemas.
4. **Retire**: deprecate the corresponding TS implementation only after consumer,
   compatibility and real-host gates. Keep rollback until the replacement is proven.

## Compatibility policy

Do not translate legacy floats by guessing a frame rate. Record conversion rules
and loss. Do not migrate projects implicitly on read. Never relabel imported
metadata as verified byte identity. New-plan schema, Rust API, planner/compiler and
adapter compatibility versions are separate. The first host execution creates a
new project; bidirectional synchronization is not a v0.x promise.

## Licensing checkpoint

Before a port, inspect the actual source license and contributor provenance, retain
required notices and document modifications. Do not relicense Apache-origin code
as MIT by translation. A separate component or an approved repository policy
change may be necessary; obtain maintainer/legal review rather than assuming.

## Source-mapped execution inventory

The [48-entry Rust transfer map](migration/OPENFILM_RUST_PORT_MAP.md) records native
candidates, external adapters and host exclusions against OpenFilm
`7add4762fe8174d76f43b863375ea39b73121e76`. The [16 gap tasks](migration/NATIVE_BOUNDARIES_AND_GAPS.md)
cover cross-language evidence, approval, host side effects, distribution and real-user gates.
Use the [machine ledger](migration/openfilm-port-index.json),
[Codex runbook](migration/CODEX_PORTING_RUNBOOK.md), and
[M1-01 implementation plan](superpowers/plans/2026-11-07-openfilm-port-bootstrap.md).

The original 48+16 inventory added planning artifacts, not implementation or a release. Source modules were
located through pinned entrypoints/tree and selected code inspection; a per-task
implementation review must still read exact symbols and tests. Preserve the original
M0 declared-metadata-only scope: later M1-03 separately verifies explicit supplied
buffers and proves one synthetic FilmCraft Linux 25 fps CPU profile. See its
[record](migration/records/M1-03.md); filesystem identity, general execution and
human acceptance remain future adapter/workflow gates.

## Merged PR #28 approval-identity candidate

This unaccepted candidate adds only bounded exact-plan identity and
Ed25519 verification in pure core. It consumes the existing supplied-buffer
verification token and binds explicit Host claims and accepted losses. It does
not sign, store keys, establish human authorization, persist/replay approvals,
or execute a host. It is distinct from the canonical proposed M3-01/P1
finite-OTIO review-binding scope in
[M3-01-NEXT](migration/M3-01-NEXT.md) and does not satisfy that task. See
[Approval Identity v1](migration/APPROVAL_IDENTITY_V1.md), the
[PR #28 candidate record](migration/records/M3-01-approval-candidate.md) and its
[dependency review](migration/dependency-review-pr28-approval-candidate.md). Independent G2 review
is pending. M3-02 and M3-03 remain
separate milestones for durable recovery and actual Host execution evidence.
