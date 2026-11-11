<!-- SPDX-License-Identifier: MIT -->
# NarrativeDocumentV1 fixtures

These synthetic fixtures accompany Issue #49 / N-R02 G2. They are proposed
examples, not evidence that a parser or CLI has been implemented.

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

The input-size positive boundary is 1,048,576 UTF-8 bytes and the first invalid
size is 1,048,577 bytes. The oversized case is generated in #50 tests by adding
valid inert padding to a valid JSON document; the repository does not store a
1 MiB fixture. #50 should also generate maximum/depth boundary cases and test
exact limit and limit-plus-one behavior.

The JSON Schema validates field shape and local numeric/string limits. Each
fixture's expected semantic result still requires the profile validator. There
is no runtime implementation or passing test claim in this G2 proposal.
