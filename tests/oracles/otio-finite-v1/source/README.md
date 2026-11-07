<!-- SPDX-License-Identifier: Apache-2.0 -->
# M1-02 pinned TS OTIO source oracle

This is a test-only, independently authored source oracle. It calls actual pinned
OpenFilm `exportTimeline("otio", ...)` with `preserveConfiguredFrameRate:true`, as
the source handoff publisher does. No media URI is opened and no host is launched.
It does not itself run the official OpenTimelineIO parser or Rust differential.

The source is pinned at `7add4762fe8174d76f43b863375ea39b73121e76`; the runner checks
the actual checkout with `git rev-parse HEAD` and requires unchanged tracked files
before and after execution. Detached checkouts and worktrees are supported.
Inputs are the six independent synthetic declarations already frozen by M1-01.
Their input bytes, supplied values and source checkout must remain unchanged.

## Run

Use official Node 24.22.1 and the frozen test-only tsx 4.23.15 loader. The existing
M1-01 oracle package contains the required lockfile and installed loader. For
portable use, replace the three paths below; no script contains a workspace path.

```sh
TSX_TSCONFIG_PATH="$SOURCE_CHECKOUT/tsconfig.json" \
  "$NODE" --import "$M1_ORACLE/node_modules/tsx/dist/loader.mjs" \
  run-source.mts "$SOURCE_CHECKOUT" "$OUTPUT_DIRECTORY" "$M1_ORACLE/producer-inputs"
```

The loader is resolved from this runner's package or the producer-input directory's
parent package. Its exact dependency lock hash is recorded. Node's official archive
checksum and the actual executable checksum are recorded in source-evidence.json.

Expected: `ok:true`, six cases, source unchanged. Exported `fixtures/*.otio` are
exact source output bytes; `producer-inputs/*.json` are byte-identical inputs.
`source-evidence.json` records tools, actual function options, source blobs,
input/output/runner hashes and source observations.

## Interpretation

Five plain cases preserve source identities, occurrence order, URIs and durations.
The raw decimal 29.97 remains the source OTIO numeric rate because the explicit
preservation option is enabled. The source represents native times at project rate.
The planned Rust profile instead uses a bounded common integer content tick clock,
sets global_start_time:null, and keeps exact project FPS in owner metadata. Compare
seconds/identity/URI semantics and report this clock-unit difference deliberately.

The scale case is accepted by source OTIO as metadata-only, retains scale in owner
metadata and emits a warning with empty native effects. It is intentionally
unsupported by the first finite Rust profile. Successful OTIO serialization does
not establish source media availability, preserved audio, actual NLE behavior or
execution approval. The official parser and real FilmCraft are distinct gates.

All newly authored runner, notes, synthetic input/output data and generated evidence
in this directory are Apache-2.0. JSON/OTIO files cannot contain SPDX comments without
changing the exact producer shape, so this directory notice covers them. Upstream
implementation/tests are not vendored; applicable source attribution and original
LICENSE/NOTICE remain recorded by the parent implementation's provenance inventory.
Test dependencies retain their own licenses; none becomes a production dependency.
