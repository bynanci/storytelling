<!-- SPDX-License-Identifier: MIT -->
# M2-02/O2 cached exact-time planner construction

Tracking [issue #35](https://github.com/bynanci/storytelling/issues/35), requirements
M2-CT01–CT03. Base `f07ab6940484ef8b8e4f2bdb37b96893e9f92270` includes accepted
PR #34's ready-queue optimisation. The accepted [fixed-window v1](../FIXED_WINDOW_PLANNER_V1.md)
API, wire, input limits, priorities, exact clocks and errors remain the contract.

## Behaviour and resource design

The former constructor stores an owned diagnostic path for every time field,
parses denominators during LCM and every subsequent scale, validates all scaled
values, then scales windows/maximum/target again. For N candidates and a target,
that is 8N+6 denominator parses, 5N+4 numerator parses, and 6N+2 diagnostic String
constructions. Those diagnostic strings are used only when a failure occurs.

The replacement caches each parsed denominator with a borrowed TimeValue and a
structured location. It first completes the same u128 LCM/u64 bound pass. Only
then does it scale maximum, target and each candidate's duration/start/end in the
original order. Each field is scaled once; windows reuse those ticks. Both
numerator and denominator parse counts become 3N+2 with a target. Successful
scaling never invokes the diagnostic formatter; failures invoke it once. This
property is tested with a counter, not global allocator instrumentation.

Arithmetic remains integer-only and uses the same equations and checked u64
conversion. The cached positive denominator divides the completed rate because
all public inputs were canonically validated before this private constructor.
No public caller can supply a cache or invoke the helper. Unselected candidate
facts still participate. Complete LCM failure retains `$` precedence over earlier
unscalable values; scaled failures retain maximum, target, candidate duration,
start and end priority. Zero source offsets and reduced output fractions remain.

The values vector contains exactly 1 + target-present + 3N entries. Structured
paths borrow static field names; candidate indices remain bounded by 1,024.
Time/space complexity remains O(N) apart from bounded gcd work. No dependency,
public API/schema, input/output limit, authority or host invocation is added.

## Source and provenance

Pinned OpenFilm remains `7add4762fe8174d76f43b863375ea39b73121e76`; refreshed main
is `73f9df9810770d52cab0cc2978e55ef8301f884e`. Solver/planner source and matching
solver tests retain the same blobs documented in the [previous optimisation](M2-ORDER-OPT.md).
The inspected `compose`/`orderedAssets` policies remain source context, while this
exact u64 fixed-window clock is the already accepted downstream profile, not a
new port of upstream floating-duration allocation or its full solver.

The baseline Rust clock at this task's base has blob
`313247e1028b2b177d4c15f5255305eda37d53bc`. Its test-only copy changes private type
naming/formatting; original maths and traversal are retained for paired comparison.
New test provenance is in [the task manifest](../provenance-m2-clock-opt.json);
modified existing modules remain in provenance-m2-02.json. MIT, Apache license,
NOTICE and Cargo dependency files remain unchanged. No upstream code/tests/media
are copied. No independent G2 acceptance is claimed for M3 PRs #31/#32.

## Observed RED and GREEN

Before replacement, a new eager diagnostic helper prototype correctly returned
15 ticks for 5/4 seconds at rate 12 but invoked its path callback once on success.
`successful_cached_scaling_never_formats_an_error_path` failed (expected 0 calls,
observed 1); the library run had 11 passing, one failing and two ignored tests.
This is the new error-only formatting resource contract, not a claim that the
original accepted arithmetic was incorrect. The other compatibility assertions
passed against the unchanged constructor before refactoring.

After cache/deferred-format implementation the failing assertion passed unchanged.
Coverage includes exact golden fractions and reduced placements, one callback on
each malformed-numerator/overflow failure, full-LCM precedence, maximum/target/
candidate error priority, 2,048 seeded canonical cases compared against the old
factory, empty private inputs and all 1,024 candidates. A real public plan-story
regression checks the corresponding full-envelope error precedence.

## Informational release measurements

Linux x86_64 AMD EPYC 9V74 VM, rustc 1.85.0/LLVM 19.1.7, default release profile.
Ten warmups then seven batches of 250 calls; each number is the median ns/call,
including temporary cache/window construction. Both implementations are timed
in the same binary on identical fractional synthetic candidates. The benchmark
is ignored by default and explicitly invoked without timing assertions:

`cargo test -p storytelling-engine --release --lib --locked benchmark_clock_cache -- --ignored --nocapture`

| Candidates | Original constructor ns | Cached constructor ns | Ratio |
| ---: | ---: | ---: | ---: |
| 16 | 5170 | 1070 | 4.83× |
| 256 | 78340 | 15874 | 4.94× |
| 1024 | 324023 | 65731 | 4.93× |

Separate whole-plan measurements of the same 1,024 one-second-window input (seven
batches of 40 calls) were 6.183 ms before / 5.598 ms after. This is a local VM
observation, not a general end-to-end guarantee. Parsing, audit and serialization
still dominate. The baseline benchmark had old and current factories at about
1× before code replacement, providing a calibration rather than a speed claim.

## Validation and delivery

Local format, warnings-denied clippy, debug/release workspace tests, warnings-denied
rustdoc and whitespace checks passed. Both debug and release passed 199 tests;
two informational benchmarks are ignored by default, with this task's benchmark
explicitly executed above. All 36 frozen comparisons (6 importer, 8 official OTIO,
10 Brief, 12 Planner), six host-checker regressions and 40 live-shadow regressions
passed. A fresh actual 64-pair run returned 56 agreements, four registered
intentional differences, four unsupported cases and zero unclassified. Native
plan-story with no external tools on PATH passed. Local comparison artifacts are
outside the repo; source was clean and target held this dirty implementation.
CI repeats validation on the committed checkout.

Self-review checked cache lifetimes, positive/dividing denominator assumptions,
LCM-before-scale ordering, exact u128 bounds, descriptor/field order and no
serialized authority change. This preserves the already accepted G2 profile;
no independent reviewer or M3 G2 approval is asserted. Final hosted head/run,
merge and completion checklist live in issue #35 and its associated PR.
