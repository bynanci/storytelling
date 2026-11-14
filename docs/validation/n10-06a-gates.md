<!-- SPDX-License-Identifier: MIT -->
# N10-06A baseline, scope and gate record

**Checked:** 2026-11-14 UTC. **Task:** N10-06A / [Issue #129](https://github.com/bynanci/storytelling/issues/129).  
**Actual latest main at start:** `52dc6c315f7b3d20693f3c51e052509997ab791b`, read from GitHub's `refs/heads/main`.  
**State:** G0/G1 inventory complete; the requester accepted draft-only G2 at candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` on 2026-11-14 and authorized A's RED/GREEN implementation. **G3 is implemented; G4 review is in progress; the PR remains Draft.** See the [decision record](n10-06a-g2-decision.md). The stable public SDK facade, host profile, execution authority and release retain separate gates.

## Authorization and G2 boundary

The requester's scoped N10-06 task of 2026-11-14 authorizes one focused A PR and says to first propose the minimum draft-input DTO, CLI entry and helper diagnostics, explicitly identifying them as unaccepted. It also requires an unapproved new contract or host capability to remain `blocked` and prohibits automatic merge.

At the start-of-task audit, Issue #129 had no acceptance comment and required: “G2: maintainer decides the one draft-input grammar, helper output and diagnostics before G3.” Its A row required a “new draft-only G2 decision before freezing fields.” The requester subsequently accepted the exact candidate in the active conversation: “接受此 G2 候選，授權 A 的 RED／GREEN 實作，保留 Draft（建議）”. The [decision record](n10-06a-g2-decision.md) binds that response to `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf`. This unlocks the focused draft-only A G3 implementation. It does not freeze #107's stable facade or establish G4/host/release acceptance. The existing V1 validator remains independently accepted.

The [SDLC](../SDLC.md) requires observed RED/GREEN, evidence for the submitted commit and separate manual acceptance. [AGENTS.md](../../AGENTS.md) requires pure core/engine code, strict untrusted-JSON boundaries, exact rational times and explicit evidence for media, approval and host claims. The current task's no-auto-merge instruction controls this PR.

## Existing capability and reusable Rust modules

| Existing component | Reuse in A; boundary to preserve |
| --- | --- |
| [`storytelling-core/src/time.rs`](../../crates/storytelling-core/src/time.rs) | `RationalTime::from_strings` checks canonical reduced nonnegative u64 numerator/denominator strings. Its comparison uses u128 cross-products. It rejects ambiguous encodings; the helper must normalize independently before calling it. Do not change this public type or its behavior. |
| [`storytelling-core/src/wire.rs`](../../crates/storytelling-core/src/wire.rs) | Existing `TimeValue` is the V1 string-pair wire representation. Emit the unchanged representation after checked integer conversion and GCD reduction. |
| [`storytelling-core/src/narrative_video_coverage.rs`](../../crates/storytelling-core/src/narrative_video_coverage.rs) | Existing V1 document, asset catalog, mapping, result and error DTOs. Reuse them without adding draft fields or helper errors. |
| [`storytelling-engine/src/narrative_video_coverage.rs`](../../crates/storytelling-engine/src/narrative_video_coverage.rs) | `render_narrative_video_coverage` remains the authoritative pure validator: 1 MiB/depth-33 bounds, strict shape, narrative/selection order and revisions, catalog identity, mapped/unresolved rows and exact declared-range checks. Preserve the 21 diagnostics and first-error order. |
| [`storytelling-engine/src/narrative.rs`](../../crates/storytelling-engine/src/narrative.rs) | Existing narrative validation and bounded JSON preflight, already used by the coverage validator. Do not create a weaker second narrative validator. |
| [`storytelling-cli/src/main.rs`](../../crates/storytelling-cli/src/main.rs) | Existing bounded JSON file/stdin reading and `render-video-coverage <FILE\|->` dispatch. Preserve the command's exits: valid complete/unresolved 0, V1 contract error 1, usage 2, I/O 3. No implicit output-file writes. |
| [`storytelling-cli/tests/video_coverage.rs`](../../crates/storytelling-cli/tests/video_coverage.rs) | Seven accepted normal-entry-point regressions and the existing synthetic V1 fixtures provide compatibility checks. Historical 7/7 evidence is recorded in [N-10 CLI validation](n10-video-coverage-cli.md); a new PR must rerun the required regressions on its own head. |

A adds an independent exact-time input helper candidate. It does not reimplement `NarrativeVideoCoverageV1`, modify its schemas/public DTOs, add to its 21 diagnostics, replace its error order or change `render-video-coverage` success/failure semantics. Full row-level advisory UX belongs to B. Media probing belongs to C. Editor integration belongs to later accepted M3 work.

## Current issue and PR evidence

The linked issue bodies/comments and PR were read on the check date. No later approval is inferred from historical completed work.

| Work | Observed state and controlling dependency |
| --- | --- |
| [#67 / N-10](https://github.com/bynanci/storytelling/issues/67) | **Closed/completed.** The [G2 decision](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322) accepted the existing V1 fields, schemas, 21 diagnostics/order and fixtures. PR #93 merged as `80577b7334110abf0c0765ec1d1c3e4cf4ae3996`; implementation PR #97 merged as `169bf6baff33cd452238337237d437e15ac5b292`. This acceptance covers declarations only. |
| [#103 / M3-04](https://github.com/bynanci/storytelling/issues/103) | **Open, ready for G2 contract decision; no comments.** Exact production execution/authority scope still needs explicit maintainer acceptance before executor implementation. |
| [PR #117](https://github.com/bynanci/storytelling/pull/117) | **Open Draft**, initial head `d6728acae6afcf4039c695ad09296792a1107d4f`; refreshed without design changes to `aa6b4cf6bfe86295ad43ad1813e8057130d6d1d1` with main@52dc6c3 during this task. All six exact-head CI jobs passed in [run #479](https://github.com/bynanci/storytelling/actions/runs/43537207405). Its own record says design only; deterministic/no-host contract implementation and an explicit decision on the tested commit remain pending. Signature validity from PR #28 is separate from current authority and compilation; independent PR #28 G2 acceptance remains pending. |
| [#104 / M3-05](https://github.com/bynanci/storytelling/issues/104) | **Open, blocked** until #103 is accepted. Owns the production FilmCraft executor. |
| [#105 / M3-06](https://github.com/bynanci/storytelling/issues/105) | **Open; implementation blocked** on #104. Recovery specifications/fixtures may precede implementation. `unknown` must survive restart and block blind retry. |
| [#106 / M3-07](https://github.com/bynanci/storytelling/issues/106) | **Open, blocked** until #104/#105 are verified. Requires actual host evidence, human project verification and maintainer accept/narrow/stop for M3. |
| [#107 / M4-01](https://github.com/bynanci/storytelling/issues/107) | **Open.** Read-only public-facade design may proceed; execution-capable APIs are blocked on #106. Future helper API adoption must follow this public-interface decision. |
| [#108 / M4-02](https://github.com/bynanci/storytelling/issues/108) | **Open, blocked** on #107's public facade/transport definition; execution/recovery additionally require #106. |
| [#109 / M4-03](https://github.com/bynanci/storytelling/issues/109) | **Open, blocked** on #107; execution also depends on #106. This A PR does not build its reference app. |
| [#110 / M4-04](https://github.com/bynanci/storytelling/issues/110) | **Open, blocked** on #107/#109; mutation tools additionally require #106. An Agent cannot grant approval to its own plan. |
| [#111 / M5-01](https://github.com/bynanci/storytelling/issues/111) | **Open, blocked** on #109 and accepted #106/#107 boundaries. No second-editor support is established. |
| [#112 / M5-02](https://github.com/bynanci/storytelling/issues/112) | **Open, blocked** on #109/#111 and accepted M3 execution/recovery. Its [latest comment](https://github.com/bynanci/storytelling/issues/112#issuecomment-6842705392) records PR #116 as protocol/blank-kit delivery only, no participants, and maintainer protocol acceptance before recruitment. |
| [#113 / M5-03](https://github.com/bynanci/storytelling/issues/113) | **Open, blocked** on #112 and accepted M3/M4/host evidence. G5 distribution/publication approval is separate. |
| [#114 tracker](https://github.com/bynanci/storytelling/issues/114) | **Open.** Preserves M3 → consumers → second host → independent beta → release-candidate evidence. Read-only API design may run earlier; production mutation remains gated by #106. |
| [#129 / N10-06](https://github.com/bynanci/storytelling/issues/129) | **Open.** Initial GitHub audit found no G2 acceptance comment; the requester then accepted draft-only A G2 at `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf` in the active conversation. [G3 is authorized and in progress](n10-06a-g2-decision.md); the PR stays Draft. A/B/C are separate scopes; C has its own G2/privacy gate and D remains dry-run until M3 acceptance. |

## Unverified evidence and pending acceptance

- The [VH-01-S3 account](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001) identifies hand-written rational strings and sequential fixes as usability costs. It is a maintainer/author self-session with prior exposure, self-reported timing and no independent observer. It is motivation for A, not proof that A improves author performance.
- The historical supplied MP4 was a synthetic FFmpeg test pattern. Its machine metadata probe did not bind `asset_id`/revision to a selected file or prove narrative content, synchronization, host application or export readiness.
- A has no independent author session, real-media binding, host readback or release evidence. Record any new machine RED/GREEN and CI separately in the A execution record. Leave author/media/host/maintainer/release fields unverified until actual evidence exists.
- A valid V1 result with `ready_for_planning=true` means **declarations can enter planning**. It does not establish file existence, actual footage identity, visual suitability, editor support or authority to execute.

## Follow-up dependencies and stop conditions

| Slice | Resume conditions | Narrow/stop conditions |
| --- | --- | --- |
| **B: input preflight UX** | A's exact candidate accepted and verified. Preserve selected `unit_id@revision` order, mapped/unresolved rows, original input and exact IN/OUT conversion with JSONPath. Existing V1 validation remains the final authority. Public SDK/UI adoption follows #107–#109. | If a text preview safely meets the task, keep the scope there. Stop any design that hides V1's first error, silently drops a missing unit, clamps a range or labels declarations as verified media/export readiness. |
| **C: local media identity sidecar** | Independent G2 contract/threat model and explicit local read/privacy scope. Bind operator-selected file handle, asset ID/revision, bytes/hash and bounded probe observations. Keep paths, media and sensitive evidence private. | Symlink/TOCTOU/replacement, unreadable or out-of-scope files, unsupported/VFR timestamps or multiple-stream ambiguity must remain `unknown`/`not_verified`. Stop if a reliable binding cannot be established. No shell interpolation, downloads, cross-directory traversal or media writes. |
| **D: dry-run pre-editor handoff** | Specification/checklist only while #103/PR #117 and #104–#106 remain pending. Track declaration validity, local-media identity/metadata, author content review, host/target qualification and current authority separately. | No FilmCraft project mutation, Agent self-approval or automatic retry of `unknown`. Stop an execution handoff when any required state lacks evidence; real host/readback acceptance remains with M3. |

A cancellation/rollback removes only the proposed helper entry and its separate input layer. The accepted V1 command, schema, DTOs, diagnostics and existing fixtures remain the fallback. The [recorded A G2 decision](n10-06a-g2-decision.md) permits its focused draft-only G3 implementation. A stable public facade, complete reference app, media adapter, production execution or G5 publication requires its own decision/evidence; this Draft PR supplies none of those gates.
