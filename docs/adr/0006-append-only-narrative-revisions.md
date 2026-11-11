<!-- SPDX-License-Identifier: MIT -->
# ADR 0006: Append-only narrative revisions and variant impact

- **Status:** Proposed for N-04 G3–G4 review
- **Date:** 2026-11-11
- **Task / requirement:** Issue #51 · N-04 / N-R04
- **Base:** main `27d6985d569c0e8882c4218b19115afc382e5aad` (merged N-03 CLI)
- **Depends on:** accepted ADR 0005 / `NarrativeDocumentV1`

## Context

N-02 identifies sources and variant references but freezes a single document
snapshot. N-03 can render a shared outline and alternative illustrations, yet
cannot retain the prior source when an author changes a fact or say which
variants need another look. Mutating the N-02 document would break its accepted
wire contract and erase its snapshot identity.

## Decision

Keep `NarrativeDocumentV1` frozen. Add a separate versioned
`NarrativeRevisionLedgerV1` that contains the N-02 base plus bounded ordered
events. Replay is deterministic and pure in `storytelling-engine`; DTOs remain
in core and the CLI owns file/stdin/stdout. Source versions and output snapshots
are retained. A source edit requires an exact expected revision, marks variants
that directly cite the old revision as needing review, and cannot be followed
by another source edit until those stale variants receive explicit review
events. Review refreshes only the named variant and preserves its ordered
source ID set. A query resolves a source-change event ID and computes the
affected set itself.

An AI suggestion and an author choice are separate closed event types. A
suggestion cannot carry a decision field. An accepted or declined choice is
recorded against one creative proposal and one illustration, preserving an
immutable variant snapshot. This is a declared author decision only: the CLI
does not authenticate the event producer or prove that a human clicked a
control.

Impact in this first slice follows direct variant source references. The ledger
does not propagate invalidation through `based_on` lineage; this is an explicit
bounded limitation, not a claim that indirect dependents are current. A future
profile may add a transitive impact graph under a new compatibility decision.

## Alternatives

| Option | Cost | Decision |
| --- | --- | --- |
| Add revision/event fields to NarrativeDocumentV1 | Mutates the accepted N-02 contract and makes N-03 readers version-dependent. | Reject; keep V1 frozen. |
| Store only current source/variant state | Cannot answer which historical source a prior output referenced or preserve author choice. | Reject. |
| Trust caller-supplied affected IDs or stale flags | A malformed or model-authored list can omit an affected variant. | Reject; derive status and impact during replay. |
| Persist to SQLite now | Adds stateful storage and recovery scope before the bounded authoring requirement needs it. | Defer; the ledger is a portable read-only input. |

## Consequences and evidence

The exact N-04 fixture proves a shared S02 revision affects all three outputs;
after all three are reviewed, a C01 revision affects only illustration B. Old
S02/C01 revisions and variant snapshots remain in the report. Accepting C01 in
B does not add it to S01 or oral. A forged decision field on an assistant
suggestion is refused. See the [profile](../narrative/NARRATIVE_REVISION_LEDGER_V1.md),
[fixture](../../tests/fixtures/narrative-v1/n04-revision-history-v1.json) and
[validation record](../validation/n04-narrative-revisions.md).
