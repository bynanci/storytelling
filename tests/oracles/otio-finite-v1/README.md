<!-- SPDX-License-Identifier: Apache-2.0 -->
# Finite OTIO official and pinned-source oracle

This independently authored, test-only comparator runs the actual Rust CLI, then
uses official OpenTimelineIO **0.18.1** to read/write/read its OTIO JSON through
the `otio_json` adapter. It never reads media URIs or starts an editor. Production
Rust has no Python, OpenTimelineIO, Node, or media-runtime dependency.

On CPython 3.12, Linux x86_64, create an isolated environment outside the checkout:

```sh
python3.12 -m venv /tmp/storytelling-otio-oracle
/tmp/storytelling-otio-oracle/bin/python -m pip install --no-deps --require-hashes \
  -r tests/oracles/otio-finite-v1/requirements-linux-py312.txt
/tmp/storytelling-otio-oracle/bin/python tests/oracles/otio-finite-v1/verify.py \
  target/debug/storytelling --output /tmp/storytelling-otio-evidence.json
```

The only permitted distribution is the official CPython 3.12 Linux x86_64 wheel
whose SHA-256 is `8f0924f818b496cf771612d489082bdb83f42b6bd97ed06589dd2e2c086742c9`.
The requirements hash check verifies downloaded wheel bytes during installation.
Evidence distinguishes that installation pin from hashes of the actual installed
native extensions. The comparator requires the exact Python platform and OTIO
version; it does not claim to reconstruct a downloaded wheel from installed files.
The wheel's upstream LICENSE and NOTICE remain in its installed distribution.

Six source cases come from the actual pinned OpenFilm exporter output in
[source/](source/README.md), with input/output/runner and dependency-lock hash
chains checked against the frozen M1-01 JSON fixtures. Five cases must serialize;
scale remains a structured Rust rejection although source OTIO retains it as
metadata only. Two additional Rust cases exercise acknowledged editing locks and
exact `1/10 + 1/5` content timing, including lossless number lexemes in the owner
snapshot string. Semantic arrays retain their original order throughout checks.

Native Rust times, ranges, parent placement and total duration are compared as
exact `Fraction` values, without tolerance. Every native value/rate must be an
integer in the frozen safe range and use the same bounded content clock. Actual
source OTIO seconds may differ by at most `1e-9` for these small synthetic inputs;
each observed error is recorded separately as a source floating representation
error. The intentional Rust delta is its common integer content clock and null
global start, while declared/exact project FPS remains separate owner metadata.
Still references use the maximum admitted hold for each asset in Rust; upstream
uses each occurrence's hold. This intentional available-window representation
does not change clip duration. The frozen source repeated-source case is video;
the Rust repeated-still regression separately checks the maximum-hold rule.
Official writing may upgrade Clip.1 to Clip.2 and replace `media_reference` with a
reference map, so comparison uses parsed semantics rather than raw schema equality.

Evidence is required outside the checkout. It records actual commands, hashes,
source SHA, target HEAD and dirty state, Python/OTIO identities, every case, source
floating errors and classifications. All importer limitations must remain equal
to the actual importer report. Execution, source-media and host verification stay
false. Passing this oracle proves serialization and the stated synthetic semantics;
it does not establish audio preservation, actual media availability, FilmCraft/NLE
compatibility, rendering, or editorial acceptance.

All newly authored comparator, documentation, requirements and generated evidence
in this directory are Apache-2.0. This directory notice covers JSON/OTIO data and
requirements that cannot carry embedded SPDX text without changing their format.
The source subdirectory has its own provenance notice and exact frozen artifacts;
no upstream implementation is copied here. Test dependencies retain their own
licenses. See the migration provenance inventory and `LICENSES/third-party/NOTICE.txt` for source
attribution and applicable upstream notices.
