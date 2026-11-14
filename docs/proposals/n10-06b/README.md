<!-- SPDX-License-Identifier: MIT -->
# N10-06B · Read-only mapping preflight proposal

English · [繁體中文](README.zh-TW.md) · [Issue #129](https://github.com/bynanci/storytelling/issues/129)

**State: proposed G2 design; implementation and format acceptance are blocked on a focused maintainer decision.** This packet defines B only. It adds no command, runtime DTO, public schema or implemented behavior. Design branch base: A's PR #130 head `59516f24f4d829a3d695c115d2f94e208a78cdea`; main at the original A audit was `52dc6c315f7b3d20693f3c51e052509997ab791b`. A subsequently merged as `125e5b1bade5be2d406fc33de67773873d2a07f5`. This is B design material only; that A merge does not accept B. Read current main before any implementation.

A's [accepted draft-only G2](../../validation/n10-06a-g2-decision.md) and [ADR 0012](../../adr/0012-draft-exact-video-range-input.md) already define exact decimal/frame conversion and its helper errors. B proposes a readable preview of the same input. The accepted [V1 validator](../../adr/0010-narrative-video-coverage-v1.md) continues to decide declaration validity, preserving all 21 diagnostic codes and first-error order. #107–#109 decide future SDK, TypeScript and Reference App interfaces; no app or transport is designed here.

## G0 / G1: task and invariant

An author selects `S02@1`, then `S03@1`, maps S02 to a declared asset with IN/OUT and leaves S03 unresolved. The preview must show those two rows in that order, the original unit and numbers, exact converted seconds, declared asset identity/duration, fixed JSONPaths and the unresolved reason. When the input has several mistakes, the author may see several **advisory input hints** and exactly the authority result the existing A/V1 pipeline returns. The preview must never repair the input, omit a selected unit, clamp a range or promote declarations to evidence about a file or editor.

The [synthetic scenarios](scenarios.json) are frozen proposed assertions, not observations. They cover complete/unresolved input, multiple helper mistakes, stale revision before range rejection, missing or reordered mappings, VFR and duplicate raw JSON. A future author observation uses the [blank evidence form](evidence-form.blank.json); all actual result fields remain null.

## Proposed interaction

The smallest candidate is a text-only CLI preview:

```text
storytelling preflight-video-coverage-input <FILE|-> [--lang zh-TW|en]
```

The command does **not exist** in this packet. G2 would need to accept this command, Traditional Chinese default labels, opt-in `draft-video-input` feature, output order, advisory policy and exits before G3. It consumes the unchanged `n10-06a.1-draft` input. No second input grammar, V1 schema, stable result DTO or machine-readable text contract is introduced. Future clients must not parse this prose as an SDK API.

The proposed command prints a preview only and never writes a file. A remains the sole normalization/output command: when its final V1 check succeeds, the author can explicitly request A's existing `--emit-v1`. B adds no `--emit-v1` alias and cannot make an invalid input eligible for output. A valid unresolved document is eligible to be printed as **V1 declaration JSON** while remaining blocked from planning. It is never labeled export-ready.

Proposed exits match A's conventions: 0 for valid complete or unresolved declarations, 1 for an A helper or V1 rejection, 2 for usage, 3 for explicit input/output I/O failure. The preview prints the actual first error even when hints identify other fields. It creates no new V1 diagnostic and cannot turn a rejection into exit 0.

## Authority before advice

Run unchanged `normalize_draft_video_coverage` on the complete original bytes. Its result determines the authority block:

| A/V1 outcome | Required authority block | V1 JSON eligible through A |
| --- | --- | --- |
| Raw byte/UTF-8/syntax/duplicate/depth failure | A helper rejected; show its exact code/path. No decoded rows or hints from ambiguous JSON. V1: not run. | No |
| Helper conversion/shape failure | A helper rejected; show its exact code/path. V1: not run. Safely decoded rows and independent helper hints may be displayed after this block. | No |
| Conversion succeeds, V1 rejects | V1 rejected; show the unchanged single V1 code/path. Do not list later possible V1 errors as actual diagnostics. | No |
| V1 valid with unresolved rows | Declaration input valid; unresolved coverage blocks planning. Show unchanged result diagnostics. | Yes; declaration JSON only |
| V1 valid and fully mapped | **Declarations can enter planning / 宣告可進入規劃**. | Yes; declaration JSON only |

The collector is subordinate to that full-input result. It must reuse A's exact time grammar/arithmetic for field hints; it cannot maintain another converter or cross-reference/range validator. It may identify unsupported decimal/frame syntax, invalid fps/timing or a visibly absent mapping. It may display the declared OUT beside the declared duration for the author to compare. It must not invent `video_coverage.invalid_range`, revision, coverage or order diagnostics that V1 has not returned. An advisory about a missing row is descriptive text, not a substitute for V1's missing-mapping error.

For example, `S02@2` against a narrative at revision 1 plus OUT=6 against declared duration 5 must first show the actual stale-unit error. The exact OUT and duration remain visible. After the author fixes only the reference, the next replay can show the actual V1 range error. Hints must not combine the two runs into a fabricated multi-error V1 response.

## Row projection and text order

The presentation order is fixed:

1. Authority block with stage, state and the actual single error or valid result diagnostics.
2. Scope line: declarations only. File bytes/metadata, author content review, host support and execution authority require their own evidence.
3. One row for **every** selected reference in its input order, including invalid or duplicate references. Each row retains the supplied `unit_id@revision`, its selection index and all exact matching mapping indices. No deduplication, sort or automatic revision repair.
4. Any mapping rows not matched to a selection remain visible in a separate original-index list. Do not reorder the input before validation.
5. Advisory input hints, clearly labeled non-authoritative and sorted by input row/field order.
6. Whether A can print valid V1 declaration JSON; planning readiness remains a separate statement. No Execute action exists.

A mapped-row display shows the original `seconds` or `frames` unit, IN inclusive / OUT exclusive, original strings, explicit fps/timing when supplied, the exact rational seconds when conversion is possible, asset ID/revision and catalog-declared duration. A display-only conversion is labeled tentative whenever the full document is rejected. If the time cannot be interpreted, show no exact result. An unresolved row shows `unresolved` and its unchanged reason; IN/OUT stays absent. A selected reference with no row is displayed as missing, never synthesized into a new unresolved row.

Every time cell identifies both locations where available:

```text
Input:  $.coverage.mappings[0].source_range.start
Output: $.mappings[0].source_range.start
IN:     "2.5" seconds = {"numerator":"5","denominator":"2"} seconds
OUT:    "4.5" seconds = {"numerator":"9","denominator":"2"} seconds (exclusive)
```

`unit_id`, revision and supplied reason are displayed as escaped/quoted data. Escape control characters and terminal sequences; do not print an invalid identifier, unknown key, parser message, hidden path or arbitrary source content directly into terminal control text. Diagnostic paths are fixed/indexed JSONPaths. Do not echo unknown fields or their values. The bounded input remains unchanged.

### Illustrative preview only

This is authored design text, **not captured command output**:

```text
宣告預檢：有效；缺素材單元仍阻擋規劃
範圍：僅檢查宣告；實體素材與剪輯器尚未驗證
1. "S02"@1 · mapped · mapping[0] · asset "CLIP-01"@1
   IN  "2.5" seconds = 5/2 seconds
   OUT "4.5" seconds = 9/2 seconds [exclusive]
   $.coverage.mappings[0].source_range → $.mappings[0].source_range
2. "S03"@1 · unresolved · mapping[1]
   Reason: "No video asset was selected for this unit."
V1: video_coverage.unresolved_required @ $.mappings[1]
V1 宣告 JSON：可由 A 明確輸出；宣告不可進入規劃
```

## Implementation boundary after a G2 decision

| Layer | Proposed responsibility |
| --- | --- |
| Core | No change to V1 DTOs, exact time type, wire fields or diagnostics. |
| Engine | Pure opt-in private preflight projection; reuse existing raw JSON preflight, A's exact conversion and the final full-input A/V1 result. No I/O, media probe, network, database, model or host call. Internal factoring may support advisory reuse while preserving A's public output/error order byte-for-byte. |
| CLI | Explicit bounded manifest/stdin read, language selection, deterministic escaped text output and unchanged A command. No default file writes. |
| Future consumers | #107 decides the public facade; #108 transport and #109 app cannot treat this proposed text preview as an accepted public API. |

Do not split the input into individually valid documents and infer full validity from those results. Invalid coverage emits no V1 payload. Budgets remain A's raw 1 MiB/depth 34 and V1's independent 1 MiB/depth 33 limits; a budget/syntax failure prevents an advisory traversal. No parser or advisory may discard unknown keys before the final validator sees them.

The exact Rust projection signature and CLI text contract are pending focused G2. No new public DTO/schema or stable advisory ID is frozen by the scenario JSON; scenario IDs are test-plan identifiers only.

## G3 / G4 plan and remaining evidence

After A is reviewed/merged and B receives its own focused G2 decision, first write normal-entry tests against the frozen scenarios and retain their actual RED output/commit. Implement the smallest pure projection, then retain GREEN and exact output. Test:

- selected/mapping indices, original time strings, unit/revision order and unresolved reasons;
- multiple hints alongside the unchanged first A/V1 result; later V1 errors appear only after a new invocation;
- no partial V1 output, no clamping/synthesis/deduplication, safe quoting and no unknown-key/value echo;
- deterministic file/stdin/language replay, unchanged inputs and directory sentinels, and no media tools on PATH;
- A's original JSON/error output and all seven accepted V1 CLI regressions unchanged;
- workspace default/opt-in debug/release, fmt, Clippy, rustdoc, repository checks and final-head Windows/Linux/macOS CI.

A real independent author operation remains unobserved. Record exact task, source/candidate version, starting data, actual errors/actions, completion, elapsed time and verbatim feedback only after it occurs. Synthetic assertions and illustrative text do not supply that result or demonstrate time savings. The blank form separates machine replay, author observation, media, host, maintainer and release evidence.

G2 must decide the one preview command/feature/language/exits, exact output ordering/escaping, advisory boundary, invalid-input emission rule and private reuse mechanism before G3. If a text table solves the task, keep it text-only. Stop any approach that hides V1's first error, silently repairs coverage, labels a file verified, or enables host execution. B can be removed without migrating A/V1 input. C's local-file sidecar/threat model, D's five-state dry-run, #103/PR #117, #104–#106 and G5 retain separate decisions.

All files in this packet are newly authored MIT design material. Scenario source contexts reference the synthetic A fixtures at the stated base. No external code, private path, real media or participant record was imported.
