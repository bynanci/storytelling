# FilmCraft 25 fps isolated host proof

SPDX-License-Identifier: Apache-2.0

Independently authored synthetic integration proof informed by pinned OpenFilm
contracts and release-pinned FilmCraft CLI behavior. No upstream tests, SDK,
binary, media or implementation code is copied. See
[contract](../../../docs/migration/FILMCRAFT_25_V1.md) and
[provenance](../../../docs/migration/provenance-m1-03.json).

`run.py` requires explicit executable paths and a **new** absolute run directory.
The official FilmCraft v0.18.0 Linux x86_64 binary must already be downloaded and
verified; the harness checks its exact SHA-256 before invoking it. It never
downloads tools, modifies HOME, opens user projects or installs packages.

```sh
python tests/oracles/filmcraft-25-v1/test_checks.py
python tests/oracles/filmcraft-25-v1/run.py --storytelling /absolute/storytelling --filmcraft /absolute/filmcraft-cli --run-root /absolute/new-run
```

FFmpeg/FFprobe default to `/usr/bin/ffmpeg` and `/usr/bin/ffprobe`; explicit
overrides are supported. All bulky generated sources, logs, projects and encoded
movies stay in the run directory. `evidence.json` records actual observed
summaries and hashes. Reproductions can vary in codec/container bytes and native
allocated IDs; acceptance is determined by checked behavior, never prewritten
hashes of outputs. The only pinned executable digest is the official host.

Native sources and sequence are 1920×1080; the explicitly scaled CPU encoded
preview is 192×108. This proof does not certify full-resolution rendering.
