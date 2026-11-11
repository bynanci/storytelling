<!-- SPDX-License-Identifier: MIT -->
# ADR 0005: Independent NarrativeDocumentV1 contract

- **Status:** Accepted for N-03 implementation by maintainer direction · 2026-11-11
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

At the original design base, Issue #48 said “Keep #49 blocked until the maintainer
records the N-01 decision” and had no comments recording such a decision. The
maintainer's current instruction explicitly identifies this ADR as accepted and
authorizes Issue #50. That instruction is recorded here as acceptance of the V1
contract for N-03 and a decision to start N-03 without waiting for the N-01 human
author pilot. It does **not** claim that the pilot or N-01 continue/narrow/stop
decision has happened; that evidence remains outstanding for the author-pilot
milestone and any release decision.

## Decision

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
| Add independent `NarrativeDocumentV1` | Gives cross-media authors their own version/refusal and loss boundary; variants can share source IDs without fabricated media | Adds a schema, parser profile and future adapter/mapping work; later interoperability must be explicit. | **Accepted** |

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
- **CLI (#50):** `storytelling render-narrative <FILE|->` reads at most
  1 MiB from an explicit regular file or stdin and writes deterministic
  human-readable UTF-8 text to stdout on success. Oral beats remain in declared
  order; variants remain in declaration order; every presented unit keeps its
  ID, revision and origin. On domain/usage/I/O failure, stdout is one JSON error
  line with a stable code and safe path; exits are 1/2/3 respectively. No logs
  are interleaved with output. The command does not auto-detect, mutate, read
  media, call a model, or write to an editor.

This is additive and does not change M0 plan JSON, StoryBrief or planner JSON,
OpenFilm finite-v1 import/export, OTIO output, existing command arguments,
review identities or digest domains. The compatibility inventory and named
regressions are in the profile. Disable the narrative path by not invoking (or
omitting from a distribution) the explicit renderer; existing projects need no
migration and no implicit conversion exists.

## Review log

The automated Codex review of the initial PR head `ede961b8d4d9e8ab025567be1cceff5c0e92c044`
raised one P2 finding: ordering IDs inside a chosen cycle did not define which
cycle to report when several cycles shared the first traversal path. This
revision specifies beat-order DFS, relation-array edge order, the first gray
back-edge witness, and a multiple-cycle fixture expecting R03/R04/R05. The
finding is addressed in the current proposal. This automated review is reviewer feedback; it does not replace the maintainer
acceptance recorded below or produce N-01 human author-study evidence.

## Acceptance and deferred work

The maintainer accepted this V1 boundary for N-03 on 2026-11-11 by directing
Issue #50 to implement against this accepted ADR. The separate automated Codex
review found a P2 ambiguity in choosing among multiple order cycles; commit
`2499a9633952b79071d9d50ed91060565b48fcc3` defines deterministic DFS witness
selection and adds a competing-cycle fixture. Re-review of that head reports no
unresolved automated findings. This automated review is recorded as reviewer
evidence, not as human author-study evidence.

N-03 is authorized to start without the N-01 author pilot. That is a decision
about sequencing only. The study table remains blank and the measured
continue/narrow/stop decision remains unmade; N-07 and any pilot/release decision
must use actual author results.

The accepted decisions for N-03 are: directed non-ordering associations;
acyclic `based_on` derivation; non-observation units require a basis; one use
of a source unit per oral variant; a 1 MiB input and the stated collection/text
budgets; free-text composition for one illustration; and JSON Schema Draft
2020-12 for the shape reference. Changes to these V1 semantics require a new
reviewed decision/version, not an implementation-local reinterpretation.

Deferred beyond N-03: immutable historical revision retention (N-04), structured
spatial layers beyond a single brief, and claim/evidence substantiation (N-05).
The #50 command selected by this implementation task is
`storytelling render-narrative <FILE|->`: it validates and renders the declared
variants deterministically for people to inspect. It does not generate new
story text or infer facts.
