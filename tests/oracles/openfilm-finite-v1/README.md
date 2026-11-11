<!-- SPDX-License-Identifier: Apache-2.0 -->
# OpenFilm finite-v1 source oracle and differential check

This is a test-only package for M1-01 / OF-01–04 / GAP-01–03/16. Node, tsx and
Python are verification tools; Storytelling production core/engine/CLI do not
depend on them. This package does not run the full upstream test suite.

`run-source.mts` executes the actual pinned OpenFilm functions against independently
authored synthetic values. The five admitted-shape cases run `validateComposition`,
`validateMediaAsset` and `exportTimeline("json", ...)`; a sixth case confirms that
upstream JSON preserves non-identity scale. Inputs remain unchanged. The runner
checks the real source checkout with `git rev-parse HEAD` and requires unchanged
tracked source files before and after execution. It uses dynamic file-URL imports
and supports a detached checkout or worktree.

The frozen source is `bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76`.
Current upstream main is a separate observation and must not replace this oracle
without explicitly refreshing the profile and evidence.

## Files and licensing

All newly authored files in this directory, including the JSON producer inputs,
source-produced JSON fixture data, generated evidence and lockfile, are licensed
under Apache-2.0. Source files carry SPDX comments; JSON data is covered by this
directory notice because adding a license property would alter the exact exporter
envelope. See repository `LICENSES/Apache-2.0.txt`, `LICENSES/third-party/NOTICE.txt` and
`docs/migration/provenance-m1-01.json`. Root Storytelling MIT licensing is unchanged.
Downloaded dependencies retain their own licenses, recorded in the lockfile.
No upstream implementation/test file is vendored in this package.

- `producer-inputs/*.json`: exact source-function input bytes, including case ID.
- `fixtures/*.json`: exact output bytes returned by actual upstream JSON export.
- `source-evidence.json`: source/tool identities, input/output hashes and source clocks.
- `run-source.mts`: independently authored source assertions and data generation.
- `compare.py`: Python stdlib comparison against a real built Rust CLI.
- `differential-evidence.json`: actual CLI commands, binary/output hashes and results.
- `package.json`, `package-lock.json`: frozen test-only tsx dependency closure.

Fixture cases are one still, video trim, repeated source, decimal frame rate,
explicit NTSC timebase and unsupported scale. All source URIs and hash declarations
are synthetic. Fixtures do not contain private media or certify any real source hash.

## Reproduce

Use Node **24.22.1** and tsx **4.23.15**. The source requires Node >=24.14.0.
The committed source record was produced on Linux x64 using the official Node
archive, verified against nodejs.org `SHASUMS256.txt` before extraction:

```text
https://nodejs.org/dist/v24.22.1/node-v24.22.1-linux-x64.tar.xz
SHA256 6c172a9ab6718f7b801fb5a7bd486af2ee5e32758086a04b84fcce2aa77095e6
```

Prepare an unmodified upstream checkout, then run from this directory. Replace
`/path/to/open-film` with that checkout. No full upstream dependency install is
required; tsx uses its actual tsconfig to resolve the source package aliases.

```sh
git clone https://github.com/bynanci/open-film.git /path/to/open-film
git -C /path/to/open-film checkout --detach 7add4762fe8174d76f43b863375ea39b73121e76
npm ci --ignore-scripts --no-audit --no-fund
TSX_TSCONFIG_PATH=/path/to/open-film/tsconfig.json \
  node --import tsx run-source.mts /path/to/open-film .
python3 compare.py ../../../target/debug/storytelling
```

Build the Rust CLI from the repository root first with `cargo build --locked -p
storytelling-cli`. A Windows executable uses the `.exe` suffix. `compare.py`
requires Python 3.10+ and no external Python packages. To keep recorded evidence
unchanged while replaying, give `run-source.mts` a separate output directory and
pass it as `compare.py --source-output /path/to/output --output /path/to/result.json`.

Expected source result: `ok:true`, exact frozen source SHA, six fixture cases.
Expected comparison result: `ok:true`, six cases. Each admitted mapping also passes
the actual M0 CLI `validate -` command. Generated evidence records target HEAD,
whether the working tree was dirty and the actual tested executable SHA-256;
dirty-tree evidence is not a claim that a committed HEAD or hosted CI passed.

## What comparison proves

The comparator parses numbers using `Decimal` and integers using `int`. It directly
compares source snapshot JSON semantics without sorting arrays. It checks occurrence
and asset identity/order, exact source ranges and durations using `Fraction`, frame
rate declarations and explicit timebase facts, `executable:false`, media evidence
limits, still hold-window disclosures and video audio-handoff loss codes. No input
fixture bytes may change. Unsupported scale must return a structured rejection
without a plan.

Decimal `29.97` stays exact `2997/100` in Storytelling. The actual source clock
resolver derives `30000/1001` while preserving the decimal export declaration.
This is an intentional interpretation difference, recorded separately from
preserved JSON/source identity and unsupported creative edits.

Successful source export or Rust import does not prove real media integrity,
audio preservation, OTIO/FilmCraft compatibility, a host roundtrip, same-head CI,
subjective quality or release readiness.
