<!-- SPDX-License-Identifier: MIT -->
# N10-06A · Exact video-range input proposal

English · [繁體中文](README.zh-TW.md) · [Issue #129](https://github.com/bynanci/storytelling/issues/129) · [ADR 0012 candidate](../../adr/0012-draft-exact-video-range-input.md)

**State: draft-only G2 accepted; G3 RED/GREEN implementation in progress; PR remains Draft.** The requester accepted candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` on 2026-11-14. [The decision record](../../validation/n10-06a-g2-decision.md) preserves the exact response and limits. Base main is `52dc6c315f7b3d20693f3c51e052509997ab791b`. The command/API remains an opt-in draft interface; #107 owns any stable SDK facade. Runtime and final-head results must be attached after implementation. [The gate inventory](../../validation/n10-06a-gates.md) records existing capability and dependencies.

## Author task and proposed input

Choose two current story units, map one to a declared asset using seconds or frame boundaries, and leave the other unresolved. The helper must preserve selection order, exact `unit_id@revision`, the unresolved reason and the original time input. The accepted V1 validator decides whether the generated declaration is valid.

The candidate wrapper has `draft_version: "n10-06a.1-draft"` and `coverage`, which holds the existing V1 fields. A mapped row's `source_range` uses one closed helper form:

```json
{"unit":"seconds","start":"2.5","end":"4.5"}
```

```json
{"unit":"frames","start_frame":"75","end_frame":"135","fps":{"numerator":"30","denominator":"1"},"timing":"constant"}
```

Both examples must become `[5/2,9/2)` seconds, serialized as canonical `numerator`/`denominator` strings. Frame IN is zero based; frame OUT is exclusive. Explicit constant timing is a declaration, not an observation of a file. VFR, unknown timing and timecode/drop-frame labels reject frame mode. Seconds use exact ASCII decimal strings and no float conversion.

The proposed default response retains each original time object and its input/output JSONPaths alongside the exact conversion, valid V1 input and unchanged V1 result. `--emit-v1` would print only the valid V1 input. Failure must emit no coverage document. A valid unresolved mapping remains visible and keeps `ready_for_planning=false`. A true value may be labeled only **“declarations can enter planning”**, never verified footage or export readiness.

## Available baseline commands

These commands exist today and read only their explicit JSON input:

```sh
cargo run --quiet --locked -p storytelling-cli -- render-video-coverage \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
cargo test --locked -p storytelling-cli --test video_coverage
```

Their expected baseline behavior is documented in [the accepted N-10 record](../../validation/n10-video-coverage-cli.md). Run results for this PR must be recorded separately; an example command is not an execution receipt.

## Implementation commands and planned checks

G2 authorizes `draft-video-coverage-input <FILE|-> [--emit-v1]` behind the disabled-by-default `draft-video-input` feature. Implementation is in progress; the following are the agreed entry points to verify on this branch, with observed results still pending:

```sh
cargo run --quiet --locked -p storytelling-cli --features draft-video-input -- \
  draft-video-coverage-input tests/fixtures/n10-06a-draft/seconds-unresolved.draft.json
cargo run --quiet --locked -p storytelling-cli --features draft-video-input -- \
  draft-video-coverage-input tests/fixtures/n10-06a-draft/frames-30-unresolved.draft.json
cargo run --quiet --locked -p storytelling-cli --features draft-video-input -- \
  draft-video-coverage-input tests/fixtures/n10-06a-draft/frames-ntsc-unresolved.draft.json --emit-v1
cargo test --locked -p storytelling-engine --features draft-video-input
cargo test --locked -p storytelling-cli --features draft-video-input
```

After the helper is verified, its `--emit-v1` output can be supplied unchanged to the existing `render-video-coverage <FILE|->` command. The original accepted CLI remains the final validator. Expected outputs are assertions until their actual replay is recorded.

| Planned check | Frozen acceptance / failure evidence |
| --- | --- |
| Exact seconds | `2.5 → 5/2`, `0.1 → 1/10`, `0 → 0/1`; trailing zeros reduce; no rounding. |
| Exact frames | `[75,135)` at `30/1 → [5/2,9/2)`; boundary `150` at `30000/1001 → 1001/200`. |
| Wide intermediates | Reduce before final u64 checks; cover representable results whose unreduced numerator/denominator exceeds u64, the 63-place decimal of `1/2^63`, and genuine overflow. |
| Helper rejection | Negative/exponent/NaN/localized/invalid decimal or frame syntax, invalid/zero fps, VFR/unknown/missing timing, duplicate keys, unknown draft fields and byte/depth limits reject with the proposed helper code and safe path. |
| Final V1 authority | Empty/reversed/out-of-duration ranges, stale references, selection order and missing mappings retain existing V1 errors and first-error order. No clamp or dropped row. |
| Read-only replay | File/stdin repetitions produce identical bytes; original inputs and directory sentinels stay unchanged; no media, network, catalog or project access. |
| Compatibility | Existing seven `video_coverage` CLI cases, Planner, Narrative, N-11, OTIO, M3/core and workspace regressions remain green. |

[Acceptance vectors](../../../tests/fixtures/n10-06a-draft/cases.json) are specifications, not observed results. The implementer must first add normal-entry tests, save their actual failing output and tested commit as RED, then implement and retain the actual passing output as GREEN. Do not invent a historical failure or mark a planned test passed.

After behavior is implemented, run workspace debug/release tests, `cargo fmt --all -- --check`, Clippy with `-D warnings`, rustdoc with warnings denied, and `python3 scripts/check_repo.py`. Final PR-head Windows, Linux, macOS and remaining required CI gates must pass. Changes to the head require fresh relevant results.

## Evidence ledger

| Evidence | Current state | Receipt / decision |
| --- | --- | --- |
| Draft-only G2 acceptance | `accepted_for_A_G3` | [Exact response](../../validation/n10-06a-g2-decision.md), candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` |
| New helper CLI/engine RED | `receipt_pending` | null |
| New helper CLI/engine GREEN | `pending` | null |
| Final-head implementation regression / CI | `pending` | null |
| Independent author entering seconds/frames | `not_observed` | null |
| Trusted local media binding/probe | `not_verified` | null |
| Host readback / execution authority | `blocked` | null |
| G5 release decision | `not_requested` | null |

Machine checks, media observations, author operation, host readback, maintainer acceptance and release each need their own dated source/receipt. No synthetic fixture supplies a human result. Keep private paths, media and sensitive evidence outside the public repository.

## Decision and follow-up

The requester accepted the exact candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` in the active conversation:

> 接受此 G2 候選，授權 A 的 RED／GREEN 實作，保留 Draft（建議）

The [decision record](../../validation/n10-06a-g2-decision.md) authorizes the focused A implementation under ADR 0012's reviewed grammar/API/output/diagnostics. It records no signer identity or independent review. Material changes to the accepted candidate need a revised G2 decision. G4 evidence and the current task's no-auto-merge restriction remain in force.

B's richer row/advisory UX follows accepted and verified A; future SDK/UI adoption follows #107–#109. C needs its own local-file contract, threat model and privacy G2. D remains a dry-run checklist while #103/PR #117 and #104–#106 remain pending. Stop or narrow any design that hides V1's first error, silently clamps ranges, invents missing coverage, treats metadata as content proof, enables self-approval or retries an unknown host outcome. Remove the helper/feature to roll back; the accepted V1 workflow remains available.
