<!-- SPDX-License-Identifier: MIT -->
# N-12 / #69: Local PNG host feasibility

English · [繁體中文](n12-inkscape-png.zh-TW.md)

This G0–G2 trial delivered a real [640×360 PNG](../../tests/experiments/n12-inkscape-png/evidence/available/bench.png)
and a usable [composition brief](../../tests/experiments/n12-inkscape-png/evidence/unavailable/composition.zh-TW.md)
when the selected export entry point was unavailable. The recommendation awaits
maintainer review. No production host adapter or public wire change is included.

## Request, input and pass conditions

The author asked `$storytelling`: “給我可開啟的圖檔，保留左邊藍傘、右邊線軸。”
(Give me an openable image; preserve the blue umbrella on the left and spool on
the right.) Both runs used the exact [request bytes](../../tests/experiments/n12-inkscape-png/request.zh-TW.txt),
SHA-256 `14014b129c6790129e28ecd6cf2d26fc591bc25a5a122bfb6bbc0bbeb945c517`.
A text template alone cannot establish that an image file exists.

The initial base was `main` `213c4963a1d72daf9d902cb0d75f737de86c1caf`.
The branch then integrated the freshly fetched English-first main
`7ef55612f660efee230623cbdbc3756386739fe1` before submission.
Issue #69 had no selected trial or comments and no same-ticket PR. The existing
FilmCraft evidence does not test SVG export. The accepted #49 / [ADR 0005](../adr/0005-independent-narrative-document-v1.md)
boundary is sufficient; #68 had no manifest to reuse at trial start.

The only source is the original MIT-licensed S2 [bench.svg](../../skills/storytelling/assets/umbrella-workshop/bench.svg)
and its [workshop description](../../skills/storytelling/references/workshop-demo.md).
It supplies a 640×360 workbench, folded blue umbrella on the left and ochre
spool on the right. No event text was added. The user's request authorizes this
local conversion and reopening of the synthetic source.

Before execution, the pass conditions were: one host and one export capability;
actual output plus version/command/exits; fresh complete PNG decoding at 640×360;
visual reopening with the requested object positions; a usable brief for the
same request with capability unavailable; unchanged source bytes throughout.

## Host execution and independent readback

The single creation host was preinstalled **Inkscape 1.4 (e7c3feb100, 2024-10-09)**,
using only local SVG→PNG export. The platform was Linux x86_64, CPython 3.12.14.
Pillow 13.0.0 was a decoder, not a second creation host. No renderer installation
or remote model call was needed. Each run used a temporary Inkscape profile/cache.

The actual export argv, also saved in the receipt:

```sh
/usr/bin/inkscape skills/storytelling/assets/umbrella-workshop/bench.svg \
  --export-type=png \
  --export-filename=tests/experiments/n12-inkscape-png/evidence/available/bench.png \
  --export-width=640 --export-height=360
```

| Observation | Evidence |
| --- | --- |
| Export exit 0; PNG is 24,216 bytes | [exit](../../tests/experiments/n12-inkscape-png/evidence/available/export.exit), [stdout](../../tests/experiments/n12-inkscape-png/evidence/available/export.stdout), [stderr](../../tests/experiments/n12-inkscape-png/evidence/available/export.stderr) |
| PNG SHA-256 `f2e11234e60fba34d6a29f6e53963a9268e503a55be16ad2a496589edb0afc38` | [receipt](../../tests/experiments/n12-inkscape-png/evidence/available/receipt.json) |
| `Image.verify()`, then fresh reopen and full `load()`: PNG, 640×360, RGBA | [readback.json](../../tests/experiments/n12-inkscape-png/evidence/available/readback.json) |
| Blue source fill `#437483`: 5,719 pixels, centroid `(184.19,201.73)` in left half | Same readback; fixture-specific color evidence, not generic object recognition |
| Ochre source fill `#ce933e`: 3,002 pixels, centroid `(476.54,186.63)` in right half | Same readback |
| Saved PNG reopened with `view_image`: umbrella/hook on left, spool/thread on right, neither mirrored nor cropped | [visual readback](../../tests/experiments/n12-inkscape-png/evidence/visual-readback.md) |

Inkscape emitted PangoFT2FontMap and GtkRecentManager initialization warnings;
their original stderr is retained. Export succeeded and this sample has no
visible text. Do not extrapolate this observation to fonts, linked resources,
text layout or arbitrary SVG support.

## Controlled unavailable-capability path

An empty PATH genuinely made the selected Inkscape entry point undiscoverable
to the same replay process. This is a **controlled injected entry-point absence**,
not a claim that Inkscape 1.4 lacks PNG support. `shutil.which('inkscape')`
returned no executable. No version or export process was invoked.

The [fallback receipt](../../tests/experiments/n12-inkscape-png/evidence/unavailable/receipt.json)
contains the same request/source hashes, `provider_invocations: []`, and
`png_created: false`. The [exit 0](../../tests/experiments/n12-inkscape-png/evidence/unavailable.exit)
means brief delivery succeeded, with `brief_delivered_png_pending`. That path's
PNG and image readback remain pending. The brief includes canvas, colors,
object placement, source labels and conditions for resuming export. This
unavailable-capability observation is not a feature TDD RED.

Both runs retained the source SHA-256
`87687584b8f4d01360feea9fbf84e510560f44c745a5973e001e55c01a35d1c9` before
and after execution. The separate [Git diff check](../../tests/experiments/n12-inkscape-png/evidence/source-git-diff.exit)
also returned 0. An independent agent recomputed the hashes, fully decoded and
visually reopened the saved PNG, and checked fallback/no-PNG evidence. No human
author trial or subjective feedback was recorded.

## Repository verification

After rebasing onto `7ef5561`, all local gates passed: format, strict workspace
Clippy, debug and release workspace tests, warnings-denied rustdoc, repository
links/provenance/requirements, graph plan/trace checks, FilmCraft evidence
rejection checks, skill distribution and skill CLI checks. Existing ignored
benchmarks and the pinned FilmCraft host test remain ignored as configured.
No assertions were relaxed. Hosted CI must be read against the submitted PR head.
The independent evidence review found no actionable gap. This trial adds only
reproduction/evidence, so no production feature RED/GREEN is claimed.

## Replay and recommendation

The [test-only runner](../../tests/experiments/n12-inkscape-png/README.md) pins the
request and source hashes and refuses an existing output directory. From the
repository root, use fresh output paths:

```sh
python3 tests/experiments/n12-inkscape-png/replay.py /tmp/n12-png-replay --expect available
python_bin=$(command -v python3)
empty_path=$(mktemp -d)
env PATH="$empty_path" "$python_bin" tests/experiments/n12-inkscape-png/replay.py /tmp/n12-brief-replay --expect unavailable
rmdir "$empty_path"
```

Available replay requires Inkscape, Pillow and a writable new output directory;
fallback needs only Python. A final replay reproduced the saved PNG and brief
byte-for-byte. If export times out, inspect the existing output and attempt
readback before any retry. Keep an uncertain result pending and use a new run
directory instead of overwriting it.

Recommend **narrow adoption: a manual local SVG→PNG procedure for synthetic,
self-contained shapes**. Both a real image and a source-labeled fallback were
demonstrated without a general provider registry. The maintainer decides whether
to accept this recommendation and schedule a production adapter separately.

[ADR 0008](../adr/0008-local-png-host-feasibility.md) keeps paths, commands and host
receipts outside the closed NarrativeDocumentV1. Core/engine remain pure.
OF-37's fixed-tool/actual-media-readback principle and OF-39's separation of
capability and enablement apply. OF-38 transcription does not apply; none of
OF-37–39 is marked implemented by this trial.

Fonts, external assets, other SVGs and cross-platform compatibility need explicit
authorized samples and new trials. A production adapter needs separate scope for
bounded inputs, process isolation and result admission. The actual author must
evaluate whether the image/brief serves their purpose. No public data migration
is needed; rollback removes only this isolated experiment and evidence.
