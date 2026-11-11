<!-- SPDX-License-Identifier: MIT -->
# NarrativeDocumentV1 fixtures

These synthetic fixtures accompany Issue #49 / N-R02 G2, Issue #50 / N-R03,
Issue #51 / N-R04, Issue #65 / N-R08, and Issue #66 / N-R09.
The engine fixture tests exercise the accepted and rejected N-02 examples; the
CLI tests exercise the three-output authoring case from #48.

| File | Expected result | What it demonstrates |
| --- | --- | --- |
| `valid-v1.json` | Accept | One source set supports a spoken outline and one illustration. S01→S02 and S02→S01 association links both remain valid; C01's imagined red umbrella is referenced only by the illustration. |
| `invalid-order-cycle.json` | Reject `narrative.order_conflict` | Adds hard oral order S01 before S02 and S02 before S01. |
| `invalid-multiple-order-cycles.json` | Reject `narrative.order_conflict`; witness IDs in declaration order are R03, R04, R05 | Contains three-edge and two-edge cycles; pins the declared DFS witness-selection rule. |
| `invalid-dangling-reference.json` | Reject `narrative.dangling_reference` | Oral beat references missing unit S99. |
| `invalid-stale-revision.json` | Reject `narrative.stale_revision` | I01 basis names S02 revision 2 while only revision 1 exists. |
| `invalid-unknown-field.json` | Reject `narrative.unknown_field` | Adds an unversioned root extension. |
| `invalid-unknown-version.json` | Reject `narrative.unsupported_version` | Changes schema version to 2. Version probing precedes other V1 validation. |
| `invalid-duplicate-field.json` | Reject `narrative.duplicate_field` | Keeps two root `schema_version` keys in raw JSON; a last-key-wins parser is nonconforming. |
| `n03-authoring-v1.json` | Accept and render three variants | Reuses the #48 S01–S04 observations, I01 interpretation and C01 creative proposal. C01 appears only in the wide illustration. No media path, duration or cut point is present. U01 remains in the author-study packet; claim/evidence semantics are deferred to N-05. |
| `n04-revision-history-v1.json` | Accept and report exact impacts | Suggests C01 for B, records a decline for A and acceptance for B, changes shared S02 and reviews all three outputs, then changes B-only C01. Retains old source and variant snapshots; final B alone needs review. |
| `n04-invalid-self-confirm-v1.json` | Reject `revision.untrusted_decision` | Adds `decision: accepted` to an assistant suggestion. A suggestion record cannot confirm itself. |
| `n08-umbrella-openings-v1.json` | Accept and render two spoken variants | Keeps the neighborhood audience, Saturday event and bring-one-umbrella instruction fixed. S1 supplies the event, S3 the synthesized taps, and P1 is a metaphor proposal based only on S3. |
| `n08-style-profile-v1.json` | Accept two style-order rules | Binds to document revision 1; the documentary variant puts event before taps and the imagery variant puts taps before event. |
| `n08-style-profile-conflicting-v1.json` | Reject `narrative.style.order_conflict` | The imagery variant requires each of the same two beats to precede the other; both rule IDs are reported. |
| `n08-style-profile-order-violation-v1.json` | Reject `narrative.style.order_violation` | An acyclic documentary rule says taps precede the event, contrary to its declared beat array. |
| `n08-style-profile-unknown-version.json` | Reject `narrative.style.unsupported_version` | Requests style profile version 2. |
| `n08-style-profile-stale-document-v1.json` | Reject `narrative.style.stale_document` | Binds the style profile to document revision 2 while the input is revision 1. |
| `n08-style-profile-unknown-field.json` | Reject `narrative.style.unknown_field` | Adds an unversioned profile-root field. |
| `n09-umbrella-source-v1.json` | Accept as N-09 source | Keeps the repair workshop notice in a closed `NarrativeDocumentV1`. |
| `n09-cue-sheet-v1.json` | Accept and render three cues | Binds pending narration to the Saturday invitation, labels a pause as estimated, and references the supplied synthetic two-tap WAV with its declared duration. Unknown voice length remains unresolved. |
| `n09-cue-sheet-cross-track-overlap-v1.json` | Accept | Two resolved cues overlap on different exclusive tracks. |
| `n09-cue-sheet-adjacent-v1.json` | Accept | Half-open intervals touch on one exclusive track without conflict. |
| `n09-cue-sheet-conflict-v1.json` | Reject `narrative.cue.track_conflict` | Two resolved narration cues overlap on the same exclusive voice track. |
| `n09-cue-sheet-reversed-v1.json` | Reject `narrative.cue.invalid_range` | The interval end precedes its start. |
| `n09-cue-sheet-duplicate-id-v1.json` | Reject `narrative.cue.duplicate_id` | Two cues reuse one cue ID. |

The input-size positive boundary is 1,048,576 UTF-8 bytes and the first invalid
size is 1,048,577 bytes. The #50 engine tests generate valid inert padding to a
valid JSON document; the repository does not store a 1 MiB fixture. They also
cover container depth 32 and 33.

The N-04 ledger is shaped by [`narrative-revision-ledger-v1.schema.json`](../../../contracts/narrative-revision-ledger-v1.schema.json)
and semantically replayed by the engine; the base document still uses the
frozen N-02 validator. The JSON Schemas validate field shape and local numeric/string limits. Each
fixture's expected semantic result is checked by
[`storytelling-engine/tests/narrative.rs`](../../../crates/storytelling-engine/tests/narrative.rs).
The authoring CLI behavior is checked from the actual `storytelling` binary by
[`storytelling-cli/tests/narrative.rs`](../../../crates/storytelling-cli/tests/narrative.rs). N-08 order checks run through [`storytelling-cli/tests/style_profile.rs`](../../../crates/storytelling-cli/tests/style_profile.rs) and the independent [profile contract](../../../docs/narrative/EDITORIAL_STYLE_PROFILE_V1.md).
N-09 cases run through [`storytelling-cli/tests/cue_sheet.rs`](../../../crates/storytelling-cli/tests/cue_sheet.rs) and the separate [`NarrativeCueSheetV1` schema](../../../contracts/narrative-cue-sheet-v1.schema.json); the narrative document remains closed and unchanged.
