<!-- SPDX-License-Identifier: MIT -->
# M2-02/O1 stable precedence ordering optimisation

Tracking [issue #33](https://github.com/bynanci/storytelling/issues/33). Target base:
`9513913e849f8cf15d9e5bbb2977fe587ce8b953`. Task requirements M2-O01–O03 preserve
accepted M2-P01/M2-P02 and [fixed-window v1](../FIXED_WINDOW_PLANNER_V1.md).

## Design and scope

The prior Kahn scheduler restarts a linear admission scan for every output vertex.
A successful 1,024-vertex order evaluates 524,800 admission positions, even when
there are no edges. The private replacement stores every ready admission position
in `BinaryHeap<Reverse<usize>>`. It chooses the minimum admission position after
each unlock, including when that position precedes all previously queued vertices.
A FIFO queue or a heap keyed by asset ID/catalog order would change the contract.

The invariant is that the heap contains every un-emitted zero-indegree vertex
exactly once. Initial zero-indegree vertices are heapified; each other vertex is
inserted only when its last incoming edge is consumed. This gives the same minimum
ready vertex as the previous scan by induction. Duplicate edges are counted and
consumed consistently. Missing endpoints and incomplete/cyclic traversal retain
`planner.brief_conflict` at `$.brief`, with no partial output.

Ready scheduling changes from O(V² + E) to O(V log V + E). The unchanged BTreeMap
ID setup/lookup makes the complete helper O((V + E) log V). Space remains O(V + E),
with up to V heap positions instead of the old emitted bitmap. Existing limits
V ≤ 1,024 and brief reference budget 8,192 remain. No dependency, public API,
schema, selection/ranking rule, clock or authority changes. Runtime remains pure
Rust; the timer, original scan and benchmark data are compiled only for tests.

## Source comparison and license

Pinned OpenFilm: `7add4762fe8174d76f43b863375ea39b73121e76`. Refreshed observed main:
`73f9df9810770d52cab0cc2978e55ef8301f884e` on 2026-11-08. Relevant blobs are identical:

| Actual source path | Blob at both commits |
| --- | --- |
| packages/solver/src/index.ts | c701b985495c8c76292880b5acbc9c062b03006c |
| packages/solver/tests/solver.test.ts | 0049a6d28c5662dadf33154b93800ae13a2aa214 |
| packages/story/src/story-planner.ts | c942438ba96e08294393a5683813f4830cb654d2 |

Inspected `compose`, `orderedAssets` and `generateStoryPlanFromGraph`; source tests
include “preserves explicit order and includes locked media without repetition”
and “obeys explicit chronology and refuses conflicting manual order”. Upstream
orders ready assets by capture time/locale ID, while this already accepted
profile uses admission priority. This optimisation preserves that documented
intentional difference. Broader upstream tests were read, not rerun as a suite.
Existing frozen and live selected-domain oracles cover the actual paired subset.

New helper/test files retain Apache-2.0 SPDX and are inventoried in
[task provenance](../provenance-m2-order-opt.json); existing planner/integration
ownership remains in provenance-m2-02.json. Original MIT LICENSE, NOTICE and
retained upstream license are unchanged. No new upstream code/tests/media copied.

## Test-first and equivalence evidence

Initial `cargo test -p storytelling-engine --lib --locked` returned 101 with
E0432: the test module referenced the not-yet-created private `ordering` module.
After extracting the unchanged scan to that module, all five new graph tests
passed. This is an extraction/characterisation RED, not a discovered behavioural
bug or proof of a speed regression. The heap replacement then passed the same
assertions without changes.

- All 4,096 directed four-vertex graphs (excluding self edges, covered separately)
  are checked against exhaustive whole-permutation enumeration, not another Kahn
  implementation. Both cycles and valid priority orders are covered.
- Non-catalog/non-ASCII admissions, newly unblocked priority, all 24 admission
  permutations for a branching graph, duplicate edges, empty inputs, missing
  endpoints, self edges and cycles retain exact results/errors.
- Supported-size independent, reverse-chain and 4,096-edge layered graphs retain
  all 1,024 vertices. The public API regression checks exact shot/catalog ordering,
  placements, shot IDs and actual nested M0 plan validation.

## Informational release benchmark

Linux x86_64, AMD EPYC 9V74 VM; rustc 1.85.0, LLVM 19.1.7, default release profile.
Each figure is the median of seven batches after ten warmups, 250 calls/batch,
including ID lookup/graph construction/output allocation. Baseline scan and heap
are timed in the same binary using identical synthetic graphs. Timing is not a CI
assertion; the ignored benchmark is explicitly invoked with:

`cargo test -p storytelling-engine --release --lib --locked benchmark_ordering -- --ignored --nocapture`

| Nodes | Graph | Original scan ns/call | Heap ns/call | Ratio |
| ---: | --- | ---: | ---: | ---: |
| 16 | independent | 670 | 785 | 0.85× |
| 16 | reverse chain | 1615 | 1425 | 1.13× |
| 16 | layered | 1073 | 1217 | 0.88× |
| 256 | independent | 21554 | 9172 | 2.35× |
| 256 | reverse chain | 56374 | 34665 | 1.63× |
| 256 | layered | 37806 | 22523 | 1.68× |
| 1024 | independent | 268945 | 40317 | 6.67× |
| 1024 | reverse chain | 497313 | 153695 | 3.24× |
| 1024 | layered | 341303 | 100900 | 3.38× |

Small independent/layered graphs incur about 0.1–0.15 µs extra cost. Separate
full `plan_story` measurements (1,024 one-second optional candidates, seven batches
of 40) were 5.693 ms before and 5.507 ms after; this single-environment observation
is not a general end-to-end speed guarantee. Other parsing/audit/serialization
costs still dominate. No adaptive small-graph path is added for that tiny cost.

## Validation and delivery status

Local format, clippy (warnings denied), workspace debug/release tests, rustdoc
(warnings denied), repository/provenance guard and diff whitespace checks passed.
Debug and release each passed 191 tests (including six compile-fail doctests);
the one informational benchmark is ignored by the default suites and was run
explicitly as recorded above. All 36 frozen comparisons passed: six importer,
eight official OTIO, ten Brief and twelve fixed-window Planner cases. Six host
checker rejection regressions passed without invoking FilmCraft. Forty live-shadow
regressions and a fresh actual 64-pair run passed (56 agreement, four intentional
differences, four unsupported, zero unclassified). Local comparison reports and
raw live-run evidence were written outside the repo; the target worktree was dirty
with this implementation at that stage, while the pinned source checkout stayed
clean. CI repeats these checks on the committed checkout.

Public API self-review confirmed unchanged ranking, exact clocks, origin paths,
limits, error precedence and false authority flags. New queue entries are bounded
by selected count, and each edge is consumed once. Original notices, Cargo.lock,
Cargo manifests and public core/CLI files were unchanged.

The final head, CI run and merge metadata are maintained in issue #33 and its
associated PR; versioned notes above record the pre-publication local evidence.
Review for this bounded refactor is self-review; no independent reviewer is claimed.

The two existing PRs #31/#32 were inspected on 2026-11-08: both are open Drafts,
with no submitted review; #32 has no review threads and its current six hosted
jobs passed. They still require independent G2 disposition of the contract and
static-source comparison. They are not eligible to merge solely because CI is
green. Issue #27 stays open; this task does not grant G2 approval or complete it.
