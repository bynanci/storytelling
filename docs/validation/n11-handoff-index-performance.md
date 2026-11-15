# N-11 handoff reference index: performance protocol

Protocol frozen before the engine change: 2026-11-15 07:55 UTC.
Base: `87113b9a204a801847995a334ef8728b9185b449`.

This is a synthetic machine-performance experiment on the existing read-only
`render-handoff` command. It provides no participant, media, editor readback,
authorization, G2 acceptance or release evidence. The public contracts, limits,
diagnostics and declared-reference semantics must remain unchanged.

## Hypothesis and workloads

The existing component lookup scans the bound variant's source references for
every component unit. Index those references once per artifact without changing
the order in which components and their declared unit IDs are rendered.

Freeze four workloads with 1, 8, 32 and 63 artifacts. Each has 256 source units,
one 256-beat oral variant and one source revision (`u000@1` to `u000@2`). Each
artifact has 32 components with 64 IDs each. The first four components partition
the 256 units; the remaining 28 repeat `u192` through `u255`. Repetition across
components is permitted. Each artifact therefore requires 2,048 reference
lookups and 435,200 comparisons in the original linear search.

| Artifacts | Input bytes | Component lookups | Original search comparisons |
| --- | ---: | ---: | ---: |
| 1 | 93,273 | 2,048 | 435,200 |
| 8 | 199,918 | 16,384 | 3,481,600 |
| 32 | 565,580 | 65,536 | 13,926,400 |
| 63 | 1,037,896 | 129,024 | 27,417,600 |

These comparison totals follow from the generated IDs' positions; they are
static work counts, not sampled CPU counters. Every workload remains within the
existing 1 MiB input, 64-artifact, 64-component and 64-ID bounds.

## Prospective release comparison

Build base and candidate in separate clean target directories using the same
Rust version, release profile, locked dependencies and features. Record source
and binary SHA-256 identities. Do not reuse a Cargo target across worktrees.
The harness performs no build, model call, media access or host action.

Use seed `910206`, one warmup per binary and workload, then 21 paired trials per
workload. Randomize workload-pair order and the two binaries' order within each
pair. Record actual order, wall time, child process time when available, exit
status, stderr and stdout hash. Drain stdout in both arms. Wall time includes
process launch, input reading and output collection; it is not isolated engine
CPU time. Preserve all observations, including slow trials.

Before timing, and on every recorded run, require successful exit, empty stderr
and byte-identical stdout for the same workload. Characterize malformed and
outside-variant references through the real CLI as separate correctness cases.
Any output or error difference blocks this optimization regardless of timing.

The predeclared machine-performance target is a candidate/base median wall-time
ratio at most `0.75` for **both** the 32- and 63-artifact workloads. Report the
1- and 8-artifact results, empirical distributions and limitations even when
they show no improvement. Do not put wall-time thresholds in CI. Paired bootstrap
intervals, if reported, describe this machine's repeated process runs rather
than independent users or general hardware performance.

If correctness fails, restore the original lookup. If either large workload
misses the target, retain the evidence and reconsider the change before merge;
do not change the workloads or threshold after seeing candidate results.

## Test evidence and current status

This is a behavior-preserving performance refactor. New characterization and
metamorphic cases describe behavior already supported by the base. They are
expected to pass on both versions; there is **no claimed functional RED**.
The prospective performance comparison supplies the hypothesis test. Pending
candidate measurements must not be reported as a measured regression or GREEN.

An exploratory debug baseline used the previously verified binary from source
`ed5bfa6a7d66cce9b66faf6ac8bef653b5e43485` (its `Cargo.toml`, `Cargo.lock` and
`crates/` contents match the base). Binary SHA-256:
`952971ea905a93d1f6170a9b5fb3eae39d55e0e4d67346ebc7d45c1e380008a4`.
It used nine randomized repetitions per workload and one warmup each.

| Artifacts | Debug median wall ms | Debug observed min–max ms |
| --- | ---: | ---: |
| 1 | 67.85 | 64.80–73.14 |
| 8 | 216.11 | 207.99–227.28 |
| 32 | 721.36 | 698.27–749.12 |
| 63 | 1,402.96 | 1,344.40–1,449.44 |

The debug observations motivated the experiment. They are not a release
baseline, a before/after improvement estimate or evidence of author time saved.
Release comparison, workspace regressions, independent review and exact-head
hosted CI remain pending.

## Observed release result · 2026-11-15

[Raw primary receipt](n11-handoff-index-performance/paired-release.json) contains
84 measured pairs (21 per workload), eight warmups, actual globally interleaved
workload/arm orders, raw wall/CPU times and hashes. All 176 CLI calls returned
zero, empty stderr and identical output bytes for their workload. Inputs and
binaries remained unchanged. These are four synthetic workload shapes on one
Linux machine; repetitions are not independent people, projects or hardware.
No bootstrap interval, significance or population claim is added.

| Artifacts | Base median ms | Candidate median ms | Ratio of medians | Base / candidate empirical p95 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 8.836 | 7.314 | 0.8278 | 9.419 / 9.874 |
| 8 | 26.962 | 16.617 | 0.6163 | 30.070 / 20.125 |
| 32 | 89.083 | 48.761 | 0.5474 | 99.608 / 55.676 |
| 63 | 173.874 | 95.283 | 0.5480 | 213.593 / 104.286 |

Both large-workload ratios meet the prospectively frozen ≤0.75 target. The
63-artifact median wall reduction is about 45.2%; it concerns this exact
synthetic process workload, including roughly 2.81 MB stdout capture. It is not
a measurement of author effort or Skill effectiveness. Each workload has 256
references; even the one-artifact case is not a tiny reference set. Tiny-set
allocation overhead and other hardware/OS performance remain unmeasured.

[Build bindings](n11-handoff-index-performance/build-bindings.json) record
separate initially absent Cargo targets, Rust 1.89.0, locked dependencies,
all features, source commits and binary hashes. The harness verifies binary
bytes, not the supplied source/build relationship. Root observed the builds
from the isolated source worktrees; this is local provenance, not attestation.
An initial run had an incorrect operator-supplied candidate source argument.
Its [unaltered receipt](n11-handoff-index-performance/exploratory-mislabeled-source.json)
is retained and excluded from the primary results. The correctly identified
repeat used the same frozen protocol, workloads, seed and acceptance threshold.

## Correctness, review and replay

The index borrows references from the validated variant, with at most 256 entries
per artifact. It does not use the inventory as a replacement for bound draft
references. It leaves iteration order, historical/current revisions, complete
coverage, proposal scope and the first error unchanged. It adds no DTO, schema,
diagnostic, dependency, I/O or execution authority. Rollback restores the private
linear lookup; there is no data migration.

Three added tests in `crates/storytelling-cli/tests/handoff.rs` exercise the real
CLI: bound/current revisions with repeated component refs, reversible declared
component/unit order, and an inventory-only reference error before a later
invalid component. [Baseline Python characterization](n11-handoff-index-performance/baseline-characterization.json)
passed before the engine edit; it is not claimed as a compiled Rust test.
The first full build caught an unsupported reverse iterator in a new test.
The [compile failure](n11-handoff-index-performance/characterization-compile-failure.log)
is retained; collecting then reversing fixed the test without changing engine
behavior. This is no claimed functional defect in the original implementation.

[Local checks](n11-handoff-index-performance/checks.json) identify the engine
commit and exact Rust/Cargo/test bytes: debug and release workspace tests pass
315 with default features and 334 with all features, each with three existing
ignored manual/benchmark tests. Formatting, Clippy `-D warnings`, rustdoc
`-D warnings`, repository checks, nine harness regressions and four CLI file-input
regressions pass. The harness has separate actual
[missing-entry RED](n11-handoff-index-performance/harness-red.txt) and
[GREEN](n11-handoff-index-performance/harness-green.txt) calibration receipts.

An independent AI reviewer checked duplicate-reference invariants, validation
order, variant scope, borrowed index lifetime and the tests, finding no blocker.
The reviewer did not build, run a host or grant human G2 acceptance. Hosted
Windows/Linux/macOS tests and all other required gates must pass on the final
PR head before merge; local timing does not replace those checks. #103, #107,
#129 B/C/D and release decisions retain their separate gates.

Rebuild two isolated source worktrees/targets, then run (replace paths with the
actual binaries; never share a target between worktrees):

```sh
python scripts/test_benchmark_n11_handoff.py
python scripts/benchmark_n11_handoff.py --baseline /path/to/base/storytelling --candidate /path/to/candidate/storytelling --baseline-source 87113b9a204a801847995a334ef8728b9185b449 --candidate-source c5b4c61e8546d758441144359b6e5d86994deae5 --output /tmp/n11-paired-release.json
```
