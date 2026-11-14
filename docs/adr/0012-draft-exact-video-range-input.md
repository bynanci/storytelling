<!-- SPDX-License-Identifier: MIT -->
# ADR 0012 · Exact video-range input helper (N10-06A candidate)

- **Status: draft-only G2 accepted by the requester on 2026-11-14 for candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf`; G3 implemented; automated G4 passed at implementation head 59516f2; current-head review required.** See the [exact decision and remaining gates](../validation/n10-06a-g2-decision.md). The candidate text below remains the accepted implementation basis; stable SDK, human, media, host and release gates remain separate. The [later continuation](../validation/n10-06a-g2-decision.md#later-continuation-verified-engineering-merge-authorization) permits engineering merge after current-head CI/review.
- **Issue:** [#129](https://github.com/bynanci/storytelling/issues/129), N10-06A / N-R10-06A.
- **Base:** `52dc6c315f7b3d20693f3c51e052509997ab791b`, checked 2026-11-14 UTC.
- **Scope:** one draft-only input helper; the accepted [ADR 0010](0010-narrative-video-coverage-v1.md), V1 schemas, public Rust DTOs, 21 diagnostics and existing CLI semantics are unchanged.

## G0 / G1: author outcome and falsifiable criteria

An author supplies IN/OUT as exact decimal seconds or zero-based integer frame boundaries and receives canonical reduced nonnegative u64 rational strings usable by the existing V1 validator. Missing footage remains unresolved. The helper never reads media or infers an asset from a name. The prior author self-session motivates this work; it does not measure this helper's usability. The [baseline and gate inventory](../validation/n10-06a-gates.md) records the evidence limits and #103–#114 dependencies.

Freeze the [acceptance vectors](../../tests/fixtures/n10-06a-draft/cases.json) before G3. The required examples include `2.5 = 5/2`, `0.1 = 1/10`, `0 = 0/1`, frame `[75,135)` at `30/1` fps = `[5/2,9/2)` seconds, and frame boundary `150` at `30000/1001` fps = `1001/200` seconds. Reject negative/exponent/NaN/timecode syntax, invalid rates, zero denominators, overflow and VFR/unknown frame timing. Empty/reversed/beyond-duration ranges must receive the unchanged V1 error and JSONPath after exact conversion. No rounding or clamping.

## G2 candidate accepted for draft implementation: minimum private DTO

The requester accepted the following **draft-only candidate** at `f148b6d` for focused G3 implementation; see the [decision record](../validation/n10-06a-g2-decision.md). It is not a stable public SDK format. No schema is added to `contracts/`, and no existing public core/engine DTO is expanded. A feature-gated experimental function would return JSON values rather than freeze a new SDK type.

```text
DraftCoverageInput {
  draft_version: "n10-06a.1-draft",
  coverage: JSON object
}
```

`coverage` contains the existing V1 input fields and ordering. Only mapped rows' `source_range` values use one of these closed helper shapes:

```json
{"unit":"seconds","start":"2.5","end":"4.5"}
```

```json
{"unit":"frames","start_frame":"75","end_frame":"135","fps":{"numerator":"30","denominator":"1"},"timing":"constant"}
```

The helper converts these values, then passes the generated V1 bytes to `render_narrative_video_coverage`. It does not duplicate narrative, variant, revision, catalog, mapping/order or duration validation. An unresolved row is copied intact. Missing or unknown mapping fields are left to V1 where possible; malformed helper ranges use helper diagnostics. No absent mapping is filled and no selection is filtered.

Frame indices are canonical ASCII u64 integer strings, zero based, with exclusive OUT. fps is an explicit positive reduced u64 rational; `timing:"constant"` is a caller declaration, **not a media observation**. `variable`, `unknown` or missing timing rejects frame mode. Seconds require no frame-rate assumption. `HH:MM:SS;FF` and drop-frame labels are never frame indices.

Decimal grammar: `(0|[1-9][0-9]*)(\.[0-9]+)?`, ASCII, 1–128 bytes. Trailing fractional zeros are accepted and reduced. Reject signs, whitespace, leading integer zeros, missing digits around the decimal point, commas, Unicode digits, exponents and NaN/Infinity. This bounded grammar covers exact u64-fit rationals such as the 63-place decimal of `1/2^63`; do not impose an undeclared precision cap.

The wrapper has exactly two fields. Raw input is at most 1,048,576 bytes; nesting is at most 34 containers (one more than unchanged V1), with strict UTF-8, syntax and duplicate-key checking before decoding. Do not deserialize nested JSON through an unchecked `Value` that loses duplicate keys. The generated V1 bytes must independently satisfy V1's own byte/depth limits.

## Exact arithmetic candidate

Use a separately authored MIT helper; do not reuse the exponent-accepting OpenFilm decimal grammar.

1. Decimal seconds: validate grammar; remove insignificant fractional trailing zeros; form the decimal coefficient as bounded ASCII digits. Zero becomes `0/1`.
2. Cancel coefficient factors of 2 and 5 against the decimal scale using exact bounded decimal-string division before parsing the remaining coefficient. This prevents a premature u128 overflow for a final u64-fit result such as `1/2^63`.
3. Parse the reduced coefficient with checked u128 multiply/add; form remaining powers of 2 and 5 with checked u128 arithmetic, take GCD, reduce, then check both final components against u64. Every exceeded bound returns an error.
4. Frames: parse index/fps components as canonical u64; require positive fps components and reduction. Compute `index * fps_denominator / fps_numerator` with checked u128 multiplication, GCD reduction and final u64 checks.
5. Verify each emitted string pair with unchanged `RationalTime::from_strings`. Range order and catalog duration are decided by the existing V1 validator. No float, implicit rounding, truncation or quantization is permitted.

Regression vectors must include a reducible intermediate beyond u64: `9223372036854775807.5`, `0.00000000000000000025`, and frame `u64::MAX` at `u64::MAX/2` fps. Also test true final u64 overflow and a coefficient exceeding u128 after exact cancellation.

## Proposed opt-in CLI and output

Proposed command: `draft-video-coverage-input <FILE|-> [--emit-v1]`, compiled only with CLI feature `draft-video-input`, disabled by default. This opt-in candidate is a local experiment for G2 review; removal is the rollback. Default builds retain their existing command surface. The [focused G2 decision](../validation/n10-06a-g2-decision.md) now authorizes this implementation.

Default output is one deterministic compact JSON line:

```text
{
  draft_version: "n10-06a.1-draft",
  scope: "declarations_only",
  status: "declaration_ready_for_planning" | "declaration_unresolved",
  conversions: [{input_path, output_path, original, source_range}],
  coverage: <valid existing V1 input>,
  validation: <unchanged existing V1 result>
}
```

Paths are fixed/indexed JSONPaths: input `$.coverage.mappings[i].source_range`, output `$.mappings[i].source_range`. `original` retains the supplied time object/text; `source_range` shows exact seconds. Original files stay byte-identical. `--emit-v1` prints only the validated V1 document, suitable for piping into unchanged `render-video-coverage -`. Neither mode writes files. Invalid input emits no coverage document.

Valid complete and valid unresolved inputs exit 0; invalid helper/V1 input exits 1; usage exits 2; input/output I/O failure exits 3. Helper failure shape: `{"error":{"stage":"helper","code":"draft_time.…","path":"…"}}`. A V1 failure uses `stage:"v1"` with the **unchanged** V1 code/path. A's helper reports one deterministic error; multiple-field advisory UX belongs to B. Final authority is V1, including first-error-wins across its validation phases. `ready_for_planning=true` is labeled only “declarations can enter planning / 宣告可進入規劃”.

Proposed helper diagnostics (not V1 diagnostics):

| Candidate code | Condition / safe path |
| --- | --- |
| `draft_time.input_too_large` | Raw helper byte limit; null path |
| `draft_time.invalid_json` | UTF-8/JSON syntax; null path |
| `draft_time.duplicate_field` | Repeated key anywhere; `$` without echoing key |
| `draft_time.depth_exceeded` | Raw nesting >34; `$` |
| `draft_time.unsupported_version` | Unrecognized draft version; `$.draft_version` |
| `draft_time.invalid_shape` | Wrong closed wrapper/range field or type; safe fixed/indexed path |
| `draft_time.invalid_decimal` | Unsupported seconds grammar; fixed start/end path |
| `draft_time.invalid_frame` | Noncanonical frame integer string; fixed start/end frame path |
| `draft_time.invalid_fps` | Nonpositive, noncanonical or nonreduced fps; range `.fps` |
| `draft_time.frame_timing_not_supported` | Timing missing, variable or unknown; range `.timing` |
| `draft_time.overflow` | Checked arithmetic or final u64 bound; fixed offending input path |

Helper order: byte budget → strict JSON/depth/duplicate preflight → version → closed wrapper → mapped time inputs in row order (unit/shape, start then end; frames: timing, fps, start then end) → unchanged V1 validator → result. Do not echo untrusted keys, values, paths or parser internals in errors. Unknown nested V1 fields remain available to the V1 validator; do not silently strip them.

## Responsibilities, compatibility and gates

- Core: unchanged accepted time/wire types.
- Engine: a separate, pure `draft_video_input` module behind feature `draft-video-input`; reuse existing strict JSON preflight, `RationalTime::from_strings` and final V1 validation. Candidate API: `normalize_draft_video_coverage(bytes: &[u8]) -> Result<serde_json::Value, serde_json::Value>`, behind the same feature. This experimental JSON boundary is accepted for the focused draft implementation; the stable facade requested by #107 remains separately gated. All existing APIs/DTOs keep their behavior.
- CLI: wires its disabled-by-default `draft-video-input` feature to the engine feature and owns explicitly selected file/stdin I/O and `--emit-v1`. No new crate or dependency, media tool, network, database or host call; the four existing crate dependency sets remain unchanged.
- B: richer row preview/advisory hints is a separate PR after A; SDK/UI decisions follow #107–#109.
- C: trusted local-file binding/probe adapter needs an independent G2 contract and threat model. No adapter is supplied by A.
- D: dry-run specification only; independent declaration/media/content/host/authority states. #103/PR #117 acceptance and #104–#106 host evidence remain pending.

The requester accepted exact candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` and authorized focused A G3 in the [decision record](../validation/n10-06a-g2-decision.md). Changes to this accepted draft grammar/API/diagnostics require another review. Human operation, media probe, host readback, authority, stable SDK adoption and G5 decisions remain unverified or separately gated.

After acceptance: write normal-entry CLI regression tests; retain observed RED on the test-first commit; implement; retain GREEN; run unchanged seven `video_coverage` tests plus workspace debug/release, fmt, Clippy `-D warnings`, rustdoc, repository checks and final-head CI on Windows/Linux/macOS. An independent author operation remains a separate unverified G4 observation. The full test plan and evidence fields are in the [English guide](../proposals/n10-06a/README.md) and [繁中 guide](../proposals/n10-06a/README.zh-TW.md).
