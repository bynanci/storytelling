<!-- SPDX-License-Identifier: MIT -->
# N-12 · One local SVG→PNG feasibility trial

This is a test-only reproduction of Issue #69 / N-R12 G0–G2, not a production
host adapter. The only creation host is Inkscape; Pillow only verifies its output.
The original MIT-licensed S2 drawing stays in the Skill assets directory.

- [Request](request.zh-TW.txt): identical bytes for both runs.
- [Real PNG](evidence/available/bench.png), [receipt](evidence/available/receipt.json),
  [fresh decode](evidence/available/readback.json), [visual readback](evidence/visual-readback.md).
- [Unavailable-path brief](evidence/unavailable/composition.zh-TW.md) and
  [receipt](evidence/unavailable/receipt.json). This deliberately hides the renderer
  from PATH, rather than claiming that Inkscape 1.4 lacks PNG support.
- [Experiment, commands and recommendation](../../../docs/validation/n12-inkscape-png.md).

`replay.py` requires Python 3.11+. The available path additionally needs Inkscape
and Pillow. Recorded versions are Inkscape 1.4 and Pillow 13.0.0 on Linux x86_64.
The fallback path uses only Python's standard library. The runner refuses an
existing output directory; use a fresh directory on every replay. It creates a
temporary Inkscape profile/cache without modifying the user's normal profile.
Source/request digests are pinned; changes require a new trial and brief review.

From the repository root, with Inkscape on PATH:

```sh
python3 tests/experiments/n12-inkscape-png/replay.py /tmp/n12-png-replay --expect available
```

To reproduce the controlled missing-capability path with the same request and source:

```sh
python_bin=$(command -v python3)
empty_path=$(mktemp -d)
env PATH="$empty_path" "$python_bin" tests/experiments/n12-inkscape-png/replay.py /tmp/n12-brief-replay --expect unavailable
rmdir "$empty_path"
```

Exit 0 means the selected deliverable succeeded. For the fallback this is only
the brief: `brief_delivered_png_pending`, with no renderer invocation or PNG.
Actual historical process stdout/stderr/exit codes are kept in `evidence/`.
PNG readback checks the two source-specific interior colors and their centroids;
it is not a generic semantic image checker. Agent visual reopening is separately
recorded and does not count as a human author trial.
