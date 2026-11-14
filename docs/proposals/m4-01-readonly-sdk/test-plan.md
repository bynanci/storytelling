<!-- SPDX-License-Identifier: MIT -->
# M4-01 · Read-only SDK golden-equivalence plan

**Status:** proposed G2 assertions; no facade implementation, new RED/GREEN,
consumer session or SDK/CLI parity run has occurred.
**Base:** `52dc6c315f7b3d20693f3c51e052509997ab791b`.
[ADR 0013](../../adr/0013-read-only-consumer-facade-v1.md) defines the candidate;
[golden-plan.json](golden-plan.json) names existing fixture/test anchors.

## Test-first implementation sequence after G2

1. Freeze exact supported operations, types, error fields and profile versions.
   Preserve the current fixture bytes and existing CLI test assertions. Save
   source/blob identities in the implementation PR; do not copy private inputs.
2. Add a test-only independent consumer using only
   `storytelling_engine::consumer_v1` and its curated result types. It must not
   import core/private engine modules. Keep it outside workspace runtime
   membership; the repository continues to contain four crates.
3. Observe and retain the actual missing-facade compile failure as RED, then
   retain runtime parity failures if wrappers compile but behavior differs.
   Do not call a planned failure an observation. Preserve all assertions.
4. Delegate to the existing backend and produce GREEN. Do not fix an unrelated
   backend or change CLI output/digests to make the wrapper test pass.
5. Run workspace debug/release, fmt, Clippy warnings denied, rustdoc/compile-fail,
   repository checks and all required final-head CI. Record cancelled/blocked
   jobs and manual evidence separately. No host test is required by this pure
   facade implementation; an ignored host test establishes no host result.

## Golden comparisons

| ID | Required assertion from the same supplied input bytes |
| --- | --- |
| SDK-G01 | `validate_plan` and `validate` agree with existing M0 result fields; CLI's envelope/exit/stdout bytes remain unchanged. |
| SDK-G02 | Narrative projection retains purpose/audience, three variants, source IDs/revisions/origins and variant-only proposals. Compare the typed projection directly to expected fields, and feed it to the unchanged private CLI formatter in CLI unit tests; integration stdout remains the existing text golden. Never reverse-parse that prose or invent a narrative JSON wire format. |
| SDK-G03 | Track/query retain historical source revisions, authored choices and exact direct affected-variant IDs. CLI text and SDK typed fields have their own predeclared expected values. The facade cannot claim indirect dependency propagation. |
| SDK-G04 | Complete and unresolved coverage return domain-equivalent V1 objects; unresolved is `Ok`/exit 0 with `ready_for_planning=false`. Preserve row order, reason, exact rational strings, 21 codes, null paths and first error. |
| SDK-G05 | Planner returns the same selected order, source ranges, exact duration, hard conflicts and non-executable/source/host flags. Compare SDK result with the CLI's `planning` field; preserve `conflicts` on errors. |
| SDK-G06 | From the existing request, freeze one planner-response byte sequence and use those exact bytes for both preparation paths. SDK `bundle_json()`/`compiled_otio()` equal CLI bundle/artifact bytes; all accepted P1/P2 digests stay fixed. Bound handles must not deserialize from or expose mutable storage for a supplied DTO. |
| SDK-G07 | Fresh/stale checks preserve `current`, ordered stale fields and false authority/source/host flags. A stale `Ok` result remains distinguishable from malformed input `Err`; CLI stale exit 1 stays unchanged. Replace one artifact byte, source hash, cut or accepted loss and retain existing rejection/freshness semantics. |
| SDK-G08 | Future versions/profiles, malformed/duplicate/unknown data and input byte/depth limits produce the exact existing backend diagnostic and full details. Compound errors retain each operation's established priority; do not normalize all operations to one policy. |
| SDK-G09 | Result/opaque types satisfy documented ownership and Send/Sync requirements; repeated sequential and caller-threaded calls are deterministic, input buffers unchanged. No filesystem/network/model/clock/database/host callback is linked into the facade. |
| SDK-G10 | `execute`, `approve`, `recover`, arbitrary shell/path calls and unchecked prepared-review constructors fail to compile from an external consumer. Mutable or `Deserialize` construction of the opaque bound object also fails. No feature enables these before #106. |

For JSON-producing CLI commands, compare the documented domain object after
removing only its existing CLI wrapper; independently preserve exact CLI
stdout goldens. For narrative/history/impact text, compare facade fields against
fixed expected domain values and the CLI against its existing text assertions.
The narrative projection at this base does not implement `Serialize`; adding a
JSON serializer solely for testing would broaden the contract and is outside
this proposal.

SDK errors retain the original source error and expose its unchanged JSON
object through a fallible detail accessor. An encoding failure must preserve
the original code/path and return the fixed adapter-only encoding diagnostic;
it must not silently emit empty details.
Verify `.code()` and `.path()` against those bytes, including optional/null
paths; preserve `conflicts`, `relation_ids` and `losses`. Do not require the new
Rust-only error wrapper to equal a CLI `ok/error` envelope or turn CLI I/O/usage
failures into engine domain failures.

## Compatibility anchors and evidence limits

Existing tests remain anchors, not new facade evidence:

- [CLI M0 tests](../../../crates/storytelling-cli/tests/cli.rs),
  [narrative/revisions](../../../crates/storytelling-cli/tests/narrative.rs),
  [coverage](../../../crates/storytelling-cli/tests/video_coverage.rs),
  [planner](../../../crates/storytelling-cli/tests/planner.rs) and
  [selected review](../../../crates/storytelling-cli/tests/story_review.rs).
- [Engine narrative tests](../../../crates/storytelling-engine/tests/narrative.rs),
  [review binding](../../../crates/storytelling-engine/tests/review_binding.rs),
  [selected review](../../../crates/storytelling-engine/tests/story_review.rs)
  and existing doc compile-fail opacity examples.
- Existing OTIO parser/source oracles and fixed digest regressions must remain
  green; a facade never updates those expected values as routine normalization.

| Evidence | Current state / owner |
| --- | --- |
| G2 facade design acceptance | pending; reviewer/maintainer |
| New facade compile/runtime RED and GREEN | not run; future implementer |
| CLI/SDK golden equivalence | not run; future reviewer |
| Same-head supported OS matrix | not run for a facade implementation |
| Independent Rust/TypeScript consumer usability | not observed; #107/#108 |
| Current authority / recovery / host profile | unavailable here; #103–#106 |
| Reference app / second host / five-user beta | blocked on their own dependencies; #109/#111/#112 |
| G5 distribution/publication | not requested or supplied |

Narrow the facade if a consumer needs fewer operations. Stop a proposal that
requires duplicating accepted validators, rewriting identity bytes, parsing CLI
prose, exposing unchecked trust constructors or permitting mutation without
#106. Record actual review findings, source commit, commands and missing data;
never convert this test plan or synthetic fixture inventory into a passed run.
