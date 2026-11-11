<!-- SPDX-License-Identifier: Apache-2.0 -->
# Selected proposal / official OTIO oracle

Run with the same hash-pinned OpenTimelineIO 0.18.1 wheel and CPython 3.12 as
`../otio-finite-v1/requirements-linux-py312.txt`:

```sh
python tests/oracles/story-review-v1/verify.py target/release/storytelling
```

This invokes the real native planner, bridge, exporter and freshness checker,
checks artifact/proposal/snapshot SHA-256 with Python's independent hashlib,
and parses/writes/reads both video-trim and still-hold artifacts with official
`otio_json`. Exact cut/order/source references and owner metadata must survive.
Only synthetic temporary files are written; no source media, Catalog or host is
opened. This is format evidence, not FilmCraft execution or editorial approval.
The upstream source map is static; this new oracle executes no OpenFilm runtime.
