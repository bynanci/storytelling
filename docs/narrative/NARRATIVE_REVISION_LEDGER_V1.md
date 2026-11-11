<!-- SPDX-License-Identifier: MIT -->
# NarrativeRevisionLedgerV1 profile

**Status:** N-04 implementation candidate for G3–G4 review.  
**Schema:** [JSON Schema Draft 2020-12](../../contracts/narrative-revision-ledger-v1.schema.json).  
**Base:** the accepted, frozen [NarrativeDocumentV1](NARRATIVE_DOCUMENT_V1.md); this profile does not change N-02.

## Purpose and limits

The ledger makes edits replayable from a declared N-02 base document. It retains
old source revisions and variant snapshots, records explicit author choices,
computes which current variants directly cite a revised unit, and labels those
variants as needing review until a matching review event arrives. It is an
append-only input history: the commands read one JSON document and write only
to stdout. They do not rewrite the base file or mutate an editor project.

The engine treats `based_on` as declared lineage, not proof. N-04 impact is
computed from direct `source_refs` in each active variant; this bounded slice
does not transitively invalidate a variant that references only a derived unit
whose own `based_on` points to the changed source. Such a variant must be
identified and revised by its author. The review record is a declaration and
does not establish factual truth, authenticated user identity or approval.

## Root and replay

The root has exactly `schema_version`, `base_document` and `events`. Version is
exactly `1`; `base_document` is validated by the frozen N-02 validator; events
must contain 1–128 entries. Input is at most 1 MiB, JSON nesting at most 32,
and objects are closed with duplicate raw keys rejected. Event IDs are unique
and use the N-02 ID syntax. Revisions are positive u32; source/variant text is
1–8,192 Unicode scalar values. All references resolve to the current source
revision at that event's position in the replay.

The report retains each base and revised source with its stable `unit_id`,
revision, origin, text and original `based_on` references. Every base and
post-decision/review variant is retained as a numbered snapshot with its exact
source references and content. Current stale status is derived from replay;
callers cannot set it in input.

| Event | Required fields | Effect |
| --- | --- | --- |
| `assistant_suggestion` | `id`, `variant_id`, `proposal_ref`, `text` | Records a suggestion for a single illustration. It creates no variant snapshot and contains no decision field. |
| `author_decision` | `id`, `variant_id`, `proposal_ref`, `decision`, `content` | Records `accepted`, `edited` or `declined` against a creative proposal and one illustration. Accepted/edited decisions add that exact proposal revision to that variant only; declined removes it from that active variant. A new variant snapshot preserves the choice. |
| `source_revision` | `id`, `unit_id`, `expected_revision`, `text` | Requires the expected current revision, appends exactly revision +1 with the same origin and `based_on`, retains the previous source, and computes directly affected variants from their prior exact references. Each affected variant becomes `needs_review`. |
| `variant_review` | `id`, `variant_id`, `source_refs`, `content` | Refreshes one stale variant to exact current revisions. The ordered unit ID list must match its prior snapshot; references are not added, removed or reordered. Appends a new current snapshot. |

A new source revision is rejected while any variant is pending review, so edits
cannot silently stack on top of unreviewed state. A review is rejected for a
current variant or if its reference IDs/revisions are wrong. Variant review
does not alter any other variant. Oral references retain the N-02 limit of 512
beats; illustration references retain the N-02 limit of 64 units.

### Suggestion and author decision boundary

The JSON type for `assistant_suggestion` is closed: fields such as `decision`,
`accepted`, `author_confirmed` or `accepted_by_author` are refused with
`revision.untrusted_decision`. The author choice is a separate event and a
separate snapshot. This prevents a suggestion record from representing itself
as accepted. However, the local CLI does not authenticate who supplied an
`author_decision` event; it is a declared choice in an untrusted input, not a
verified identity or cryptographic approval. A future interactive client must
collect that event through a distinct author action before it is appended.

## CLI

```text
storytelling track-narrative-revisions <FILE|->
storytelling query-narrative-impact <FILE|-> <CHANGE_ID>
```

Both commands read at most 1 MiB from a regular file or stdin and write
deterministic UTF-8 text on success. `track` prints event impacts, author
choices and suggestions, complete source/variant history, and current variant
status. `query` accepts only a source-revision event ID and computes its impact
from replay; callers cannot provide an affected-variant list. Errors are one
JSON line with code and safe path; exits are 1 for invalid input, 2 for usage,
and 3 for I/O. Neither command writes files or calls a model, database, editor,
or network.

## N-04 example result

For [`n04-revision-history-v1.json`](../../tests/fixtures/narrative-v1/n04-revision-history-v1.json):

```text
shared-observation-change: S02@1 -> S02@2
Affected variants: oral, illustration-wide, illustration-close

b-only-detail-change: C01@1 -> C01@2
Affected variants: illustration-close

Current oral@2: current
Current illustration-wide@3: current
Current illustration-close@3: needs_review; stale C01@1
```

The first change is reviewed separately in all three outputs. A later C01 edit
affects only the close illustration because A's author decision declined C01
and the oral outline never referenced it. Source C01@1, the old base variant
snapshots and the accepted B@2 choice remain inspectable. The current oral
content contains no C01 or red umbrella, and the source observation S01 remains
unchanged. The accepted red umbrella is still typed `creative_proposal`, not
`supplied_observation`.

## Stable diagnostics

| Code | Meaning |
| --- | --- |
| `revision.untrusted_decision` | Suggestion event contains a decision/confirmation field |
| `revision.too_many_events` | More than 128 replay events |
| `revision.duplicate_event` | Repeated event ID |
| `revision.pending_review` | Another source change/choice is attempted while a variant is stale |
| `revision.variant_not_pending` | Review targets a current variant |
| `revision.reference_set_changed` | Review alters the prior ordered unit ID set |
| `revision.decision_not_illustration` | Suggestion or choice targets a non-illustration |
| `revision.decision_not_creative_proposal` | Choice does not reference a creative proposal |
| `revision.too_many_versions` | Revision counter would overflow u32 |
| `revision.change_not_found` | Query ID is absent |
| `revision.not_source_change` | Query ID is not a `source_revision` event |

The N-02 validator's `input.*` and `narrative.*` errors also apply. Paths point
to the input field and do not echo source text.
