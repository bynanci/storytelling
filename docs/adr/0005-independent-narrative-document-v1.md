<!-- SPDX-License-Identifier: MIT -->
# ADR 0005: Independent NarrativeDocumentV1 contract

- **Status:** Proposed for G2 review; not accepted or frozen
- **Date:** 2026-11-11
- **Task / requirement:** Issue #49 · N-02 / N-R02
- **Design base:** `main` at `906c2c64aba820c06e4a74f4348c746b53e8b17b`
- **Depends on:** Issue #48 · N-01 / N-R01
- **Implementation:** Issue #50 is downstream and deliberately out of scope here

## Context and authority

The synthetic packet in [N-01](../narrative/N-01-AUTHOR-STUDY.zh-TW.md), merged by
[PR #64](https://github.com/bynanci/storytelling/pull/64), has one supplied source
set for a spoken outline and visual compositions. It distinguishes S01–S04
observations, I01 interpretation, C01 (the imagined red umbrella), and U01, a
commercial claim with no evidence. The red umbrella belongs to illustration A
only. The packet's human study/results table is blank: no author has performed
the matched task, and the proposed thresholds have not been accepted by a
maintainer.

At this design base, Issue #48 still says “Keep #49 blocked until the maintainer
records the N-01 decision”; it has no comments recording such a decision. The
current maintainer instruction asks for Issue #49 now. This ADR treats that
instruction as authorization to **prepare and review a G2 proposal only**. It
does not claim an N-01 continue/narrow/stop decision, waive a human trial, freeze
this schema, or authorize Issue #50. Whether #50 must wait for the N-01 author
pilot remains an explicit maintainer decision.

## Decision proposed

Add an independent, strict, versioned `NarrativeDocumentV1` format that models
declared source units and named presentations without video assets, media paths,
durations, frame rates, or cut points. Keep it alongside every existing video
contract. Store the wire shape in
[`contracts/narrative-document-v1.schema.json`](../../contracts/narrative-document-v1.schema.json)
and the authoritative semantic rules in
[`NARRATIVE_DOCUMENT_V1.md`](../narrative/NARRATIVE_DOCUMENT_V1.md).

A document declares its purpose and audience; a monotonically intended document
revision; units with stable IDs, per-unit revision, origin and declared
`based_on` references; relations; and named variants. V1 origins are
`supplied_observation`, `author_interpretation` and `creative_proposal`.
`based_on` means “the author declares this unit was derived from these
units.” It is not evidence, independent corroboration, truth, or permission to
publish. U01-style substantiated-claim semantics are deferred to N-05.

The minimum example has one ordered oral variant and one single-illustration
variant. Both refer to the same S units. C01 is referenced only by the
illustration. The JSON Schema describes shape; the profile defines duplicate
key rejection, byte/depth budgets, unique IDs, current revisions, graph
semantics, deterministic diagnostics and the fact that a valid document is
still only a set of declarations.

## Alternatives considered

| Option | Benefit | Cost / failure mode | Decision |
| --- | --- | --- | --- |
| Extend M0 `PlanDocumentV1` | Reuses strict JSON and existing validation | M0 assets, hashes, durations, shots and source ranges encode video. Narrative-only authors would need fake assets/times or a breaking M0 V1 change; this violates N-R02's no-media case. | Reject |
| Extend `StoryBriefV1` or planner V1 | Reuses IDs and hard-order audit | Brief/planner contracts mean selected video assets, candidate windows, durations, shot counts and M0 plan output. Oral beats and spatial single-image composition would be shoehorned into video planning, and existing consumers would risk changed audit/plan behavior. | Reject |
| Add independent `NarrativeDocumentV1` | Gives cross-media authors their own version/refusal and loss boundary; variants can share source IDs without fabricated media | Adds a schema, parser profile and future adapter/mapping work; later interoperability must be explicit. | **Propose** |

## Semantic boundary

An `association` is a directed conceptual link and imposes no order. Therefore
A associated-with B plus B associated-with A is valid. A `must_precede` edge is
a hard, strict order constraint scoped to a `spoken_outline` variant. A cycle
in those edges is a conflict; the engine must report it, never delete or sort
away an author decision. The oral `beats` array itself is the declared output
order, and must satisfy every applicable edge. Association edges do not enter
the precedence graph.

Every reference names both an in-document unit ID and its exact current unit
revision. Missing IDs and revision mismatches reject. Supplied observations
have an empty basis list. Interpretations and creative proposals cite one or
more current units. Derivation links must be acyclic; conceptual cross-links
belong in `association`. These checks establish internal consistency only.
They do not establish that an observation occurred, that an interpretation is
reasonable, that a proposal was accepted, or that a person is authorized.

V1 contains a current snapshot only. It does not retain old unit text or verify
that `document_revision` increased from an external prior version. Immutable
revision history and stale-variant reporting belong to N-04. V1 defines no
canonical digest or signed identity.

## Bounded strict input and failure behavior

The profile caps UTF-8 JSON at 1,048,576 raw bytes; JSON nesting at 32 containers
(root object depth 1); units at 512; relations at 4,096; variants at 16; oral
beats at 512 per variant; `based_on` at 32 per unit; illustration references
at 64; IDs at 1–128 ASCII bytes; purpose/audience at 512 Unicode scalar values;
and unit/beat/brief text at 8,192 Unicode scalar values. Revisions are positive
u32. Boundaries are inclusive; the next byte/item/scalar rejects.

The engine must reject duplicate object keys before materializing a last-key-wins
JSON value; reject unknown fields and unknown versions; check all references and
revisions; then check derivation and precedence graphs. In particular, a valid
JSON Schema shape does not override semantic errors. Oversize input fails before
JSON parsing. Stable error codes and safe-path rules are specified by the
profile. No error echoes source text, arbitrary keys or local file paths.

## Ownership and compatibility

- **core:** versioned value types and enums only; no I/O or policy inference.
- **engine:** bounded strict decode, cross-reference/revision checks, graph and
  order validation, stable diagnostics; pure and deterministic, with no file,
  media, model, network, database, clock or host access.
- **CLI (#50 only):** explicit local file/stdin entry, bounded read, one JSON
  response and documented exit codes. No auto-detection, mutation, media read,
  model call, editor write, or new command in this design PR.

The additive profile does not change M0 plan JSON, StoryBrief or planner JSON,
OpenFilm finite-v1 import/export, OTIO output, existing command arguments,
review identities or digest domains. The compatibility inventory and named
regressions are in the profile. Disable the future path by not invoking (or
omitting from a distribution) the explicit N-02 validation command; existing
projects need no migration and no implicit conversion exists.

## G2 review conditions and open decisions

Before N-03 implementation, an independent reviewer and maintainer must record
whether this exact profile is accepted, changes requested, or narrowed. The
maintainer must also decide whether the N-01 human pilot/continue decision is a
hard prerequisite to #50. Reviewers should specifically examine:

1. whether `based_on` is the right declaration name and whether non-observation
   units must require a basis;
2. whether association should be directed (as proposed) and whether V1 should
   forbid derivation cycles;
3. whether forbidding repeated use of a unit in one oral variant is acceptable;
4. the 1 MiB/32-depth and collection/text budgets;
5. the `single_illustration` profile's free-text brief versus structured spatial
   roles; the design intentionally does not grow a general canvas model;
6. whether JSON Schema Draft 2020-12 is an adequate published shape reference.

No issue author or implementer may count a self-review as independent G2
acceptance. Until the reviewer/maintainer decision is recorded, this remains a
proposal; #50 must not treat the fixtures as frozen.
