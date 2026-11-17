<!-- SPDX-License-Identifier: MIT -->
# Proposed G3/G4 validation plan

**All transport/client runtime cases are planned, not run.** [Cases](cases.json)
are prospective requirements. Header-schema/example consistency is a separate G2
static check; it is neither observed RED/GREEN nor a human/Host result.

## #108 requirement mapping

| Requirement | Falsifiable acceptance | Cases |
| --- | --- | --- |
| M4-R02.1 · own types / compatibility | Independent strict TS consumer imports its own public API, rejects newer wire/facade/view and has explicit Node/package/profile table. | TS-G01, TS-G04, TS-G10 |
| M4-R02.2 · parity | Eight operations return domain-equivalent complete fields in order; source arrays and exact rational strings survive. | TS-G01, TS-G02 |
| M4-R02.3 · diagnostics | Original code/path/raw serializer evidence survives, including null/absence, relation/conflict/loss arrays and detail-encoding failure. | TS-G03 |
| M4-R02.4 · portable privacy | Only buffers cross the boundary; executable configuration/OS stderr remain local; malformed path/Host/shell fields cannot trigger I/O. | TS-G05, TS-G09 |
| M4-R02.5 · bounded failure | Lengths, checked sums, strict JSON, EOF, malformed stdout, process exit and cancellation races settle once without leaks/implicit retry. | TS-G06–TS-G09 |
| M4-R02.6 · reproducible consumer | Fresh checkout builds an opt-in bridge and independent unpublished TS package; complete read-only example needs no editor/model/service. | TS-G10 |
| M4-R02.7 · unchanged domains | Existing CLI stdout/exits, seven coverage CLI tests, fourteen SDK tests, P1/P2 vectors and all workspace flows stay green. | TS-G11 |
| M4-R02.8 · engineering cost | Separately measure process/copy/wait overhead and peak memory; do not infer author time saved. | TS-G12 |

## Meaningful RED before implementation

After G2 adoption, first add real-entry CLI tests that invoke the currently
built binary with the candidate entry and assert a complete frame/domain
outcome. Observe the actual old binary's missing-entry/prose/exit mismatch.
Capture exact source SHA, argv, exit, stdout/stderr bytes and fixture hashes.
A missing test target, unavailable toolchain or module-import failure alone is
not the required behavioral RED.

Then implement the minimal framing/dispatch/view layers and observe GREEN at
the same entry, without weakening assertions. Add fake-child tests for malformed
output/termination races before implementing the corresponding Node paths.
Keep a source-bound RED commit and raw logs; distinguish actual execution from
authored expectations. No production fault hook may simulate trusted approval.

## Golden sources and independent expectations

Use the accepted fixtures in place, listed with SHA-256 in cases.json:

- M0 minimal plan, N-03 narrative, N-04 append-only ledger and two impact changes.
- Complete/unresolved video coverage and the existing 21 diagnostic scenarios.
- Fixed-window planner with manual order/constraints and rich conflict cases.
- Frozen `consumer-sdk-v1/proposal-response.json` **including its LF**,
  finite OpenFilm snapshot and review context; old artifact/bundle stdout goldens.
- Existing SDK source-error byte literals for relation_ids/conflicts/losses.

The Rust facade is the reference computation, not the expected-value generator
for every assertion. Retain literal fields, declared order/source tags and
frozen independent bytes from SDK-G01–G10. Compare reference versus TS typed
values **and** those independent expectations. Compare raw OTIO/bundle/error
bytes separately. Only the old CLI's one explicit framing LF may be removed
when comparing its stdout to an accessor payload; never normalize source data.

The new planner response wrapper is deliberately compact and LF-free. Validate
its existing shape, retain the exact result bytes as its substring and replay
that one byte array into prepare/check. Its digest need not equal the CLI's
different selected response. Existing CLI vectors must remain unchanged.

## Lifecycle, errors and resource tests

cases.json freezes named adversarial conditions. Important paired assertions:

1. Unresolved/stale `Ok` versus malformed/rejected `domain_error`.
2. Domain-invalid change ID with oversized/bad ledger: ID-first source diagnostic
   survives as long as the request fits the transport cap.
3. Duplicate/unknown envelope fields versus opaque domain duplicate fields:
   envelope rejection is new; source rejection remains profile-specific.
4. Wrong wire/facade/view, swapped buffer roles or wrong request association versus
   a structurally valid correctly associated response.
5. Abort before spawn, while writing/reading, and after full frame but before
   zero exit; timeout; nonzero exit after a valid frame; exact-once settlement.
6. Prefix/header/body cap-1, cap, cap+1; checked overflow; truncation; extra LF
   versus a LF included inside a declared buffer; second frame; stderr overflow.
7. Unknown process termination retains a failure/locked instance until exit;
   no hidden replacement, retry, child-of-child, Host operation or temp write.

Use fake children under test-only paths and explicit OS adapters. Run the real
bridge in a sandbox directory with sentinel files and empty/minimal PATH/env.
The product bridge invokes no editor, probe, shell, network or child process.
Inspect before/after directory state and request bytes; never expose private
paths or include raw stderr in a public error.

## Future G3 commands and required exact-head evidence

The following commands/paths are **targets after G2, not runnable artifacts
delivered by this PR**:

```text
cargo test -p storytelling-cli --features readonly-transport --test readonly_transport
cargo build --locked -p storytelling-cli --features readonly-transport
npm ci --ignore-scripts --prefix examples/typescript-consumer-v1
npm test --prefix examples/typescript-consumer-v1
npm run example --prefix examples/typescript-consumer-v1 -- <explicit-local-binary>
```

An example must perform describe plus all eight read-only operations, show both
unresolved and stale outcomes, retain proposal/artifact/bundle bytes and print
stable code/path for one rejection. Prerequisites include Rust 1.89.0 and the
chosen locked TypeScript toolchain; prospectively test Node 22.20.0 and 24.22.1
on Linux, Windows and macOS. No current tested-runtime claim is made here.

Before G4 acceptance, add that TS/bridge OS matrix to required checks through
a separately reviewed workflow change. Existing six ordinary and three packet
jobs alone cannot prove a future TS implementation. Read all jobs and the exact
head identity; after a later authorized merge, read post-merge CI separately.

Existing runtime regressions (current commands) remain mandatory:

```sh
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo test --workspace --release --all-features --locked
cargo test -p storytelling-cli --test consumer_sdk --locked
cargo test -p storytelling-cli --test video_coverage --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps --locked
python3 scripts/check_repo.py
git diff --check
```

Planner, Narrative, N-11 handoff, OTIO and journal/M3 tests are part of workspace
regressions. Three historical ignored tests do not provide new FilmCraft evidence.
Frozen source/oracle regressions in current CI retain their own conditions.
A source-only regression cannot be labeled new transport/client GREEN.

## Prospective engineering performance

For valid narrative, ledger, coverage, planner and prepared-review inputs, freeze
source/build/OS/CPU/Node, buffer hashes and options before measurement. Per input
and path, record five warmups then thirty measured runs, alternate direct Rust
reference and Node/process order with a recorded seed. Verify every outcome/hash
before including its duration. Include copying, spawn, stdin wait, decode and
zero-exit wait in end-to-end Node time; record backend-only time separately.

Report n, failures/timeouts, median/p95/range and peak memory measurement method.
Unavailable child RSS is unknown; client resourceUsage is not child RSS. Cap
violations/timeouts are failures, not dropped outliers. The 30-second default
is a functional deadline, not evidence of a latency target or improvement.
Maintainer sets any performance release budget from measured hardware-specific
evidence. No human sample, author-effort comparison or speedup claim follows.
