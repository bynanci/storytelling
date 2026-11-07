<!-- SPDX-License-Identifier: Apache-2.0 -->
# Fixed-window planner source oracle

Independently authored synthetic M2-02 cases informed by OpenFilm
`7add4762fe8174d76f43b863375ea39b73121e76`. All new files in this directory,
including JSON inputs/evidence, belong to the source-informed Apache-2.0 component.
No upstream implementation, tests, media or runtime is copied. Test dependencies
retain their own licenses; see root LICENSES/Apache-2.0.txt, NOTICE.openfilm and
the task provenance inventory.

The runner executes the actual pinned `compose` implementation. Ten of twelve
cases cover manual order, hard exclusions, required membership above a soft target,
hard maximum failure, short source bounds, target shortfall, empty selection,
paired declared-score/rating selection and a lowercase ASCII tie. The remaining two
cases explicitly record different selected assets caused by fixed-window
non-shortening and the new preferred-ID policy. These differences are deliberate;
this is selected-policy evidence, not full solver or score-factor parity.

Native JSON inputs follow the frozen
[fixed-window contract](../../../docs/migration/FIXED_WINDOW_PLANNER_V1.md).
Hand-authored `expectedNative` rows freeze selected IDs/order, exact total,
target status or typed rejection for later actual Rust comparison. They are not
observations of unexecuted Rust code.

Use official Node24.22.1 and the existing M1 oracle's installed tsx4.23.15 loader:

```sh
TSX_TSCONFIG_PATH=/path/to/open-film/tsconfig.json node \
  --import /path/to/storytelling/tests/oracles/openfilm-finite-v1/node_modules/tsx/dist/loader.mjs \
  tests/oracles/fixed-window-planner-v1/run-source.mts /path/to/open-film \
  /absolute/fresh/output-directory
```

The loader's actual package version and lock hash must match the frozen M1
dependency lock; no installation or download is performed. Source status is
checked clean before/after execution; outputs must be outside that checkout.
Evidence binds full source SHA,
inspected path blobs, actual tool versions and executable/loader/package hashes,
runner/native-input/source-input hashes,
normalized complete source output hashes and immutable input checks.

Normalization deletes **only** the random generated `id` fields of composition,
track and clip objects. It preserves story/beat/asset IDs, times, facts and every
semantic array order. Thus regeneration is byte-reproducible despite upstream
random UUIDs. No media path is opened and no production Node dependency is added.
Source case inputs contain only inert synthetic URIs.

Run the actual native comparison after building the Rust CLI:

```sh
python3 tests/oracles/fixed-window-planner-v1/compare.py target/release/storytelling \
  --output /tmp/fixed-window-planner-comparison.json
```

The comparator uses the literal frozen native selections; it does not reimplement
the planner's ranking/admission search. It checks file/stdin/repeated-call byte
equality, exact rational timing and clock bounds, unchanged candidate hashes/source
ranges, stable shot IDs, ordered reasons/skipped decisions, the unchanged actual
brief-auditor report, and the actual M0 `validate` consumer accepting the nested
plan. It preserves all semantic array order. Evidence must be written outside the
checkout and records actual target HEAD/dirty state, CLI/comparator hashes,
source/runner/lock hashes, commands, each classified case and observed results.
These runs do not establish host/source verification or general solver feasibility.
