<!-- SPDX-License-Identifier: MIT -->
# ADR 0008: Local PNG host feasibility boundary

- **Status:** Proposed after N-12 G0–G2 experiment; maintainer adoption pending
- **Date:** 2026-11-11
- **Task / requirement:** #69 · N-12 / N-R12
- **Base:** `213c4963a1d72daf9d902cb0d75f737de86c1caf`
- **Integration base:** `7ef55612f660efee230623cbdbc3756386739fe1`

## Context

The author asks for an openable image preserving the left blue umbrella and
right thread spool in the original MIT-licensed S2 workbench drawing. A textual
illustration brief cannot prove a PNG exists. The accepted NarrativeDocumentV1
is closed and intentionally has no media paths or execution instructions.

## Proposal

Keep one local host and one capability for this feasibility slice: Inkscape 1.4
converts the supplied, self-contained SVG to PNG. A test-only runner records
input/request identity, host version, command, output bytes, and exit status.
Pillow separately reopens and fully decodes the saved PNG. Source-specific fill
centroids complement an agent visual reopening; neither is human author feedback.

When the same Inkscape entry point is hidden from PATH in a controlled run,
deliver the same request as a source-labeled composition brief with PNG pending.
This probes missing execution capability, not a claim that the installed
Inkscape build lacks PNG support. Discovery is not authorization: the current
user request authorizes this source and operation only.

Keep receipts and commands outside NarrativeDocumentV1. Core and engine remain
pure, and no production provider registry, renderer or Rust subprocess API is
added. Apply the OF-37 fixed-tool/actual-readback principle and OF-39 separate
registration/enablement principle; OF-38 transcription is outside this trial.

## Outcome and deferred decisions

The [experiment](../validation/n12-inkscape-png.md) produced and reopened a
640×360 PNG with the requested positions. The controlled missing-capability
run delivered a usable brief without claiming a PNG. Recommend narrowing
adoption to this manual local conversion procedure. The maintainer decides
whether to schedule a production adapter in a separate ticket.

This is one Linux/build/sample observation. Text, fonts, external resources,
arbitrary SVG, other hosts and cross-platform compatibility require new cases.
No migration is needed because no public contract changes. Rollback removes
the isolated reproduction/evidence and retains the original S2 source.
