<!-- SPDX-License-Identifier: MIT -->
# NarrativeDocumentV1 fixtures

These synthetic fixtures accompany Issue #49 / N-R02 G2 and Issue #50 / N-R03.
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

The input-size positive boundary is 1,048,576 UTF-8 bytes and the first invalid
size is 1,048,577 bytes. The #50 engine tests generate valid inert padding to a
valid JSON document; the repository does not store a 1 MiB fixture. They also
cover container depth 32 and 33.

The JSON Schema validates field shape and local numeric/string limits. Each
fixture's expected semantic result is checked by
[`storytelling-engine/tests/narrative.rs`](../../../crates/storytelling-engine/tests/narrative.rs).
The authoring CLI behavior is checked from the actual `storytelling` binary by
[`storytelling-cli/tests/narrative.rs`](../../../crates/storytelling-cli/tests/narrative.rs).
