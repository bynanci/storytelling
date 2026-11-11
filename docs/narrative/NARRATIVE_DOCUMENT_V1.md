<!-- SPDX-License-Identifier: MIT -->
# NarrativeDocumentV1 profile

**Status:** accepted for N-03 by maintainer direction on 2026-11-11.  
**Schema:** [JSON Schema Draft 2020-12](../../contracts/narrative-document-v1.schema.json).  
**Positive and negative fixtures:** [narrative-v1 fixtures](../../tests/fixtures/narrative-v1/README.md).

This document is the normative semantic companion to the shape schema.
“Must” and “must not” describe the accepted V1 profile for N-03. The V1 wire
shape is frozen for this slice; later changes require a versioned decision.

## Purpose and boundary

A document describes one author's declared source material and alternative
presentations. V1 supports spoken outlines and single-illustration briefs with
the same source units. It contains no media bytes, paths, URLs, asset hashes,
durations, frame rates, clip ranges, renderer commands or evidence-verification
claims. It does not decide publication truth or human approval.

A successful validation means only that the supplied declarations are
well-shaped, bounded, internally referenced and order-consistent. It is not a
fact check or an authorization decision.

## Versioned root

The root is a UTF-8 JSON object with exactly:

| Field | Type | Rule |
| --- | --- | --- |
| `schema_version` | integer | Required, exactly `1`; other versions refuse before V1 field validation. |
| `document_id` | ID | Stable identifier for this narrative document. |
| `document_revision` | positive u32 | Intended revision of this snapshot; V1 cannot verify monotonicity without external history. |
| `purpose` | string | Nonempty, at most 512 Unicode scalar values. |
| `audience` | string | Nonempty, at most 512 Unicode scalar values. |
| `units` | array | 1–512 content units. |
| `relations` | array | 0–4,096 relation declarations. |
| `variants` | array | 1–16 named presentation variants. |

Every object at every level is closed: unknown fields reject. Duplicate JSON
property names reject before a map/value representation can overwrite them.
Missing required fields, explicit null in place of a required value and wrong
JSON types reject. JSON numbers are integers where specified; floating-point
revisions and schema versions reject.

## Content units and provenance

Each unit has exactly `id`, `revision`, `origin`, `text` and `based_on`.
IDs match `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`; the 1–128 bound is ASCII bytes.
Unit IDs are unique within `units`. Revisions are positive u32. Text is
nonempty and no more than 8,192 Unicode scalar values.

| Origin | Meaning | `based_on` |
| --- | --- | --- |
| `supplied_observation` | Material the author declares was supplied as an observation or note. The validator does not authenticate it. | Required empty array. |
| `author_interpretation` | Author's reading of other material. It remains interpretation. | 1–32 exact current unit references. |
| `creative_proposal` | Invented or proposed content, such as the red umbrella. It is not silently promoted to observation. | 1–32 exact current unit references. |

A reference has exactly `unit_id` and `revision`. It must resolve to one unit
in this document and match that unit's current revision. No external references
are accepted. A missing ID returns `narrative.dangling_reference`; a revision
mismatch returns `narrative.stale_revision`. A non-observation with no basis
returns `narrative.missing_basis`; an observation with a nonempty basis returns
`narrative.observation_has_basis`. The derivation graph formed by `based_on`
must be acyclic; association links are separate and may loop.

`based_on` records author-declared derivation, not evidence. In particular, it
does not show that a cited source proves the content, that the source is true,
or that an author accepted an AI suggestion.

## Relations

A relation has a unique ID in `relations`. Relation IDs are unique among
relations; unit IDs and variant IDs are each unique in their own collections.

- `association` has exactly `id`, `kind`, `from`, `to`. It is a directed,
  conceptual, non-ordering link. It may point both ways: A→B and B→A is valid.
  It never contributes an order edge or proof/support.
- `must_precede` has exactly `id`, `kind`, `variant_id`, `before`,
  `after`. It is a strict hard order edge for a named `spoken_outline`.
  Endpoints must be distinct current units present in that variant's beats.
  An unknown `variant_id` returns `narrative.dangling_reference`; references to
  an existing illustration variant or absent beats return
  `narrative.invalid_relation_scope`.

Unknown relation kinds reject. Repeated declarations are not silently
coalesced; duplicate relation IDs reject. For each spoken variant, the directed
`must_precede` graph must be acyclic. A valid A-before-B plus B-before-A
association pair does not enter this graph. A hard-order cycle returns
`narrative.order_conflict`. To select one witness deterministically, process
variants in declaration order; for each oral variant, run depth-first search
from beats left-to-right, traversing that variant's precedence edges in relation
array order. Use the standard white/gray/black DFS rule: the first edge to a
gray ancestor closes the reported simple cycle, and traversal stops. Report
that cycle's relation IDs in document declaration order. This chooses the first
DFS witness, not necessarily the shortest cycle. The multiple-cycle fixture pins
the tie-break: it must report R03/R04/R05 before the also-valid R03/R06 cycle.
The validator never deletes an edge or chooses a replacement order.

## Named variants

Variant IDs are unique among variants and use the shared ID syntax. The
human-readable `name` is nonempty and at most 128 Unicode scalar values.
Names need not be unique; callers address variants by ID.

### `spoken_outline`

A spoken outline has exactly `id`, `name`, `kind` and `beats`. Beats are
an explicit ordered array of 1–512 entries. Every beat has a unique-per-variant
ID, one exact `unit_ref` and nonempty presentation `text` (at most 8,192
Unicode scalar values). A source unit may appear at most once in a given oral
variant, so each strict precedence edge has an unambiguous position. The array
order must satisfy every applicable `must_precede` edge. A violation returns
`narrative.order_violation`; equal-position ordering is impossible in this
profile because each beat names one unit.

### `single_illustration`

An illustration variant has exactly `id`, `name`, `kind`, `unit_refs` and
`brief`. It names 1–64 unique exact unit references and a nonempty brief of at
most 8,192 Unicode scalar values. The brief describes composition in ordinary
language. V1 does not impose screen coordinates, foreground/background roles,
visual-generation APIs, or a renderer. A proposal such as C01 belongs only to
the variant that references it.

## Limits, parsing order and diagnostics

Limits are inclusive and apply to decoded JSON values unless stated otherwise:

| Limit | Proposed bound |
| --- | ---: |
| Raw input | 1,048,576 UTF-8 bytes; reject the next byte before parsing |
| Container nesting | 32; root object has depth 1 |
| Units / relations / variants | 1–512 / 0–4,096 / 1–16 |
| Beats per oral variant | 1–512 |
| `based_on` references per unit | 1–32 for derived units; 0 for supplied observations |
| Unit references per illustration | 1–64 |
| IDs | 1–128 ASCII bytes |
| Purpose / audience / name | 1–512 / 1–512 / 1–128 Unicode scalar values |
| Unit / beat / illustration text | 1–8,192 Unicode scalar values |
| Revisions | positive u32 |

The proposed deterministic rejection sequence is: raw byte budget; UTF-8 and
JSON syntax plus duplicate-key/depth preflight; schema-version probe; strict V1
shape and field/type checks; collection/text/ID budgets and duplicate IDs;
reference existence/revision checks; origin/basis and derivation graph; relation
scope; hard-order cycle; explicit beat-order satisfaction. For otherwise
independent errors at the same phase, report the first path in input traversal
order. Paths use JSONPath-like locations and never include untrusted property
names or source text. Multiple order-cycle relation IDs preserve declaration
order. A parser must not silently accept last-key-wins duplicates.

Stable proposed codes include:

| Code | Meaning |
| --- | --- |
| `input.too_large` | Raw byte limit exceeded |
| `input.invalid_json` | Invalid UTF-8 or JSON syntax |
| `narrative.depth_exceeded` | More than 32 nested containers |
| `narrative.duplicate_field` | Duplicate property in one JSON object |
| `narrative.unsupported_version` | `schema_version` is not exactly 1 |
| `narrative.unknown_field` | Closed-object field not in V1 |
| `narrative.invalid_shape` | Missing, null, or wrong-typed field |
| `narrative.too_many_items` | Collection or text budget exceeded |
| `narrative.duplicate_id` | Duplicate ID in its typed collection |
| `narrative.dangling_reference` | Referenced unit/variant does not exist |
| `narrative.stale_revision` | Reference revision differs from current unit |
| `narrative.missing_basis` | Derived unit has no declared basis |
| `narrative.observation_has_basis` | Supplied observation declares a basis |
| `narrative.derivation_cycle` | `based_on` graph is cyclic |
| `narrative.invalid_relation_scope` | Precedence is not within one oral variant |
| `narrative.order_conflict` | Hard precedence graph has a cycle |
| `narrative.order_violation` | Beat order violates an acyclic hard edge |

Error objects expose stable code and safe path. They do not echo text, paths,
unknown keys or parser internals. JSON Schema enforces the portable shape; it
cannot detect repeated raw JSON keys, raw byte/depth budgets, cross-reference
revision equality, derivation cycles, or precedence semantics. The engine
profile remains authoritative for those checks.

## Package ownership and N-03 CLI surface

- **`storytelling-core`:** DTOs/enums for the V1 data only; no parser I/O,
  filesystem, clocks, hashes, inference, media access, or external authority.
- **`storytelling-engine`:** pure bounded decode, validation and deterministic
  projection of the declared variants; duplicate-key/depth preflight,
  references/revisions, provenance and relation/order semantics, stable
  diagnostics. No filesystem, model/network, database, clock or editor.
- **`storytelling-cli` in Issue #50:** the explicit
  `storytelling render-narrative <FILE|->` command reads at most 1 MiB from a
  regular file or stdin, calls the engine, and writes deterministic
  human-readable text. It prints oral beats in declared order and illustration
  variants in declaration order, preserving each source ID/revision and origin.
  Domain rejection exits 1, usage 2, I/O 3. It never auto-detects JSON.

The command is opt-in. Omitting or not invoking it disables the narrative path;
existing projects need no migration. It does not write a file, Catalog, project,
timeline, or editor state. No new story text is inferred: it only validates and
organizes presentation text already declared in the input.

## Video compatibility and regression inventory

N-02 adds a parallel profile; the following input/output contracts and test
oracles remain unchanged. Issue #50 must run them at the same head as the new
narrative tests:

| Surface | Existing contract / fixtures | Existing regression suites / invariant |
| --- | --- | --- |
| M0 video JSON | `examples/minimal-plan.json`, `tests/fixtures/invalid-source-range.json` | `crates/storytelling-engine/tests/validation.rs`, `crates/storytelling-cli/tests/cli.rs`; strict IDs/ranges, unknown/duplicate fields, unsupported versions, exact time and size behavior. |
| StoryBrief V1 | `tests/fixtures/brief-v1/{valid,conflicting}.json`; [profile](../migration/STORY_BRIEF_V1.md) | `crates/storytelling-engine/tests/brief.rs`, `crates/storytelling-cli/tests/brief.rs`; hard order cycle, duplicate keys, unknown fields/version, size/depth and fixture diagnostics. |
| Planner V1 / `plan-story` | `tests/fixtures/planner-v1/{valid,conflicting}.json`; [profile](../migration/FIXED_WINDOW_PLANNER_V1.md) | `crates/storytelling-engine/tests/planner.rs`, `crates/storytelling-cli/tests/planner.rs`; file/stdin parity, repeatability, bounded input, emitted M0 plan remains valid. |
| OpenFilm finite-v1 JSON | `tests/fixtures/openfilm/finite-v1/*.json`; [profile](../migration/FINITE_V1.md) | `crates/storytelling-engine/tests/openfilm_snapshot.rs`, `crates/storytelling-cli/tests/openfilm.rs`; imported snapshot bytes/number lexemes unchanged and strict unsupported-feature refusal. |
| OTIO | `tests/oracles/otio-finite-v1`; [profile](../migration/FINITE_OTIO_V1.md) | `crates/storytelling-engine/tests/otio.rs`, `crates/storytelling-cli/tests/otio.rs`; deterministic timeline, file/stdin parity, loss/error semantics, official parser oracle. |
| Review/digest | [Planner OTIO review](../migration/PLANNER_OTIO_REVIEW_V1.md), StoryReview fixtures in `tests/fixtures/story-review-v1` | `crates/storytelling-engine/tests/review_binding.rs` and `story_review.rs`; fixed cross-platform artifact/binding vectors and domain-separated identities stay byte-for-byte unchanged. N-02 defines no digest. |
| CLI surface | [CLI guide](../CLI_GUIDE.md), [contract index](../CONTRACTS.md) | `crates/storytelling-cli/tests/cli.rs`, `brief.rs`, `planner.rs`, `openfilm.rs`, `otio.rs`, `review_cli.rs`, `story_review.rs`; all current commands/flags/exit codes remain unchanged. |

Existing implementations do not consume this profile until #50 is accepted and
implemented. A future new command is opt-in. Removing or not invoking that
command disables the narrative path without editing old JSON or migrating a
project. No existing digest may be reused as a NarrativeDocument digest.

## Deferred decisions and evidence

1. N-01's human author pilot and continue/narrow/stop decision have not happened.
   The maintainer authorized N-03 without that pilot; this does not substitute
   for the pilot's actual measures.
2. N-04 must decide how old unit revisions are retained and queried. V1 carries
   only current unit revisions.
3. Future spatial profiles may need structured foreground/background fields;
   N-03 uses the accepted free-text single-image brief.
4. Claim/evidence support semantics remain N-05 work and are not implied by
   `based_on`.
