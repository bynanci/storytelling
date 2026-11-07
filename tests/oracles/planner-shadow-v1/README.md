<!-- SPDX-License-Identifier: Apache-2.0 -->
# Selected-domain live planner shadow

M2-03 executes the actual pinned OpenFilm `compose` and actual native `plan-story`
consumer in one bounded run. It extends the frozen 12-case M2-02 comparison with
seeded paired execution, targeted error pairs and literal difference probes.
All files here, including synthetic JSON mutation baselines, are independently
authored source-informed Apache-2.0 test components; no upstream implementation,
SDK, tests or media are copied. Dependencies retain their own licenses. See
`NOTICE.openfilm`, `LICENSES/Apache-2.0.txt` and the M2-03 provenance inventory.

## Scope

The [frozen profile](../../../docs/migration/PLANNER_SHADOW_V1.md) owns all exact
schemas, domains, budgets and error/disposition pairs. Default seed 20261107 and
24 cases/domain yield 64 actual pairs:

- 24 fully required fractional cases with complete ordering and dyadic source
  capacities ≤1 s; no source resizing can change those windows.
- 24 one-second optional ranking cases with integer target=max K and explicitly
  aligned rating/capture/ASCII ordering. This is a domain precondition, not general
  score-factor, capture-time or locale parity.
- 8 targeted shared successes/errors: hard membership/order, exclusion, missing ID,
  required/excluded contradiction, cycle, hard maximum, shortfall and inclusion.
- 4 literal intentional changes: fixed-window non-shortening, preferred-ID policy,
  native empty-plan refusal and native soft target above the hard maximum.
- 4 unsupported multi-beat/media-balance probes. They are separately counted and do
  not establish implementation of these features in native Rust.

Source outcomes retain semantic array order and all non-generated facts. Only
random composition/track/clip object IDs are removed. Success comparison uses
exact Fraction clocks, source windows, placements and totals with zero tolerance.
Source numeric decoding bounds finite values, exponent and significant digits
before allocating integer fractions; out-of-domain numeric encodings fail closed.
Every actual native nested plan also goes through the actual `validate` command.
The four mapped source error families have exact synthetic messages and native
code/conflict pairs. An arbitrary pair of errors, changed policy label, wrong
window/order/authority or missing result cannot become agreement.

No production API/dependency changes, broad solver parity, source/media or host
verification, execution approval, human editorial acceptance or release claim.
Existing immutable host evidence is historical, not a fresh planner authority.

## Run regressions

Use Linux x86_64, official Node 24.22.1 and the existing integrity-locked
`tests/oracles/openfilm-finite-v1` package/lock (tsx 4.23.15, esbuild 0.28.2).
Prepare an unmodified source checkout pinned to
`7add4762fe8174d76f43b863375ea39b73121e76` and a temp tool prefix using the existing
package/lock with `npm ci --ignore-scripts --no-audit --no-fund`; no source-workspace
or global dependency installation is needed. The harness never downloads tools.

All three paths below are mandatory for the live producer regression tests;
missing setup fails rather than silently skipping source coverage:

```sh
PLANNER_SHADOW_SOURCE=/absolute/open-film \
PLANNER_SHADOW_NODE=/absolute/node \
PLANNER_SHADOW_TSX=/absolute/oracle-tools/node_modules/tsx/dist/loader.mjs \
python3 -m unittest discover -s tests/oracles/planner-shadow-v1 -p 'test_*.py'
```

`test_shadow.py` checks complete successful metadata/semantic baselines and
single-field mutations, including all eight literal probes.
`mutation-baselines.json` contains compact actual source/native observations from
an executed run, solely for mutation tests; it is never read by `run.py` as live
expected output. Generated shared cases compare the two actual executions.
`test_process.py` exercises real bounded children, exact stdout/stderr/input caps,
full-duplex capture, timeouts/global deadline and owned descendant cleanup.
`test_producer.py` executes the actual source producer and checks strict UTF-8,
duplicate decoded keys, exact report shape, actual versions/hashes, domain-error
serialization and unexpected-exception failure.
`test_orchestration.py` executes failed preflight and incorrect-native runs,
requiring failed evidence, unexecuted identities to stay unknown and actual
source postchecks even on failure.

## Execute a live shadow

Build native release first. Supply absolute executable paths and a new,
nonexistent run directory outside both source and target checkouts:

```sh
cargo build --release --locked -p storytelling-cli
python3 tests/oracles/planner-shadow-v1/run.py \
  --source /absolute/open-film \
  --node /absolute/node \
  --tsx /absolute/oracle-tools/node_modules/tsx/dist/loader.mjs \
  --rust-cli /absolute/storytelling/target/release/storytelling \
  --run-root /absolute/new-shadow-run
```

Use `--seed U32` and `--cases-per-domain N` (1..64) to reproduce or vary the seeded
matrix. Maximum 144 cases. Case input/order and generator hashes are recorded.
CLI success exits 0; mismatch/execution failure exits 1; invalid usage exits 2.
The one-line summary points to `evidence.json`; raw generated input, producer,
native/validator stdout/stderr and their hashes remain only in the run directory.
Do not commit bulky runtime artifacts.

All child execution is argv-only with live bounded capture: 1 MiB stdin, 4 MiB stdout,
64 KiB stderr,30 s child timeout and180 s total. The final 5 s is reserved inside that
total for actual source postchecks. Overflow/timeout kills and reaps the owned
Linux process group, including tsx/esbuild descendants. Child `NODE_OPTIONS` and
`ESBUILD_BINARY_PATH` overrides are cleared, source tsconfig is explicit, and
inherited proxy/CA settings are preserved. No media URI or user project is opened.

A failed preflight, tool, comparison, validation or source postcheck leaves
`allPassed:false` and a structured failed stage. Unexecuted identities remain
unknown. Reports bind actual source blobs/clean state, target HEAD/tree/dirty
state, tool/binary/runner hashes, commands, observed cases and artifact bytes.
Dirty candidate reports remain explicitly dirty until rerun on the final clean
published candidate. Passing selected cases never marks the whole solver ready.
