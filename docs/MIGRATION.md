# OpenFilm extraction and migration

This bootstrap imports design intent, not implementation. No OpenFilm code, tests,
project database or media is copied. The original MIT LICENSE is preserved.

## Extraction map

| Capability | M0 decision | Later migration condition |
| --- | --- | --- |
| Evidence/source identity concepts | Minimal declared hash and asset reference only | Actual byte verification and provenance supplied by an authorized adapter |
| StoryPlan/constraints/planner | Define finite new wire subset; no full port yet | Pin upstream SHA, map behavior and add differential fixtures |
| OpenFilm Catalog/Composition/history | Stay owned by OpenFilm | Read-only snapshot importer; never write its SQLite directly |
| Review/Jobs/owner recovery | Document requirements, no empty implementation | Add workflow/storage only with fault-tested vertical slice |
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
