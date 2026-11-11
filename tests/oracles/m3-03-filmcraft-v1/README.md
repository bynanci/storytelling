# M3-03 isolated FilmCraft execution proof

This first executable M3-03 slice exercises a single synthetic video cut with
the hash-pinned FilmCraft v0.18.0 Linux CLI. It builds a fresh source and review,
creates a new project beneath an explicit new run root, persists project
creation and import intents in the SQLite journal before each host call, and
inspects the native FilmCraft project after import.

The import wrapper waits until FilmCraft has exited successfully, then withholds
its acknowledgement. The caller records `outcome_unknown`, reads the native
project and sequence, and records `applied_observed`. The proof asserts that the
journal contains one attempt and that no retry or Undo occurred. If the host
does not exit within the harness bound, the wrapper receives a stop signal and
reaps its child; the proof stops with a durable unknown result and fails for
operator inspection. It never retries that import. Existing journal regressions cover
ambiguous readback, stale owner fencing, restart recovery and rejection of late
responses on Linux, Windows and macOS CI.

Run it with absolute paths and an empty, unique run-root path:

```sh
cargo build --release --locked -p storytelling-cli
python3 tests/oracles/m3-03-filmcraft-v1/run.py \
  --storytelling "$PWD/target/release/storytelling" \
  --filmcraft /absolute/path/to/filmcraft-cli \
  --ffmpeg /absolute/path/to/ffmpeg \
  --cargo /absolute/path/to/cargo \
  --run-root /absolute/path/to/a-new-m3-03-run
```

The runner refuses an existing root or a different FilmCraft binary. The host
data directory, empty project, imported project, journal, synthetic source and
readback evidence all stay under the new root. The original empty project,
source and review inputs are hash-checked unchanged after import. No FilmCraft
Catalog or existing FilmCraft project is opened. Review freshness verifies
proposal, snapshot, context, bundle and artifact consistency; it reports
`approval_granted=false` and `executable=false`. This is synthetic integration evidence, not trusted human
authorization, general host support, or a product executor. It adds no editor
write command to the Storytelling CLI.

The result is `evidence.json`; `operations.sqlite`, `journal-result.json`, host
readback JSON and command logs are retained for review. If the proof fails after
an attempt marker is committed, inspect the journal and project in that run root
before taking any action. Never reuse that root or resend the import.
