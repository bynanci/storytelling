<!-- SPDX-License-Identifier: MIT -->
# Read-only SDK acceptance and issue convergence · 2026-11-17

[繁體中文](sdk-acceptance-2026-11-17.zh-TW.md) · [SDK guide](../SDK_CONSUMER_V1.md) · [Issue #107](https://github.com/bynanci/storytelling/issues/107) · [Machine snapshot](sdk-acceptance-2026-11-17/post-disposition.json)

The eight-operation read-only Rust V1 facade passed final engineering acceptance. [PR #149](https://github.com/bynanci/storytelling/pull/149) was marked Ready and merged as `9d15382045fb7453827accda772c56ddc0abee44`; its tree is identical to tested head `5bf480c017f1d27cce307ae5931470d772bef5d6`. #107 is **closed/completed with every original criterion checked**. The observed Open Issue count moved from **16 to 15**, not a product-completion percentage. The accepted namespace is distinct from a published stable package, production authority or G5.

## Actual decision and verification

The requester had adopted ADR 0013's precise G2 candidate and then asked:

> 幫我後續驗收 並收斂issues

Codex performed criterion-by-criterion engineering acceptance and used the repository's existing verified-merge authorization. The prior Draft/Open restriction described the earlier G2-only stage; the follow-up request advanced that stage. This records the actual user's request and the engineering action, not a human usability session, independent human security review, signature or DCO. The [original G2 record](m4-01-sdk-g2-decision.md) remains historical evidence; it is not rewritten to pretend it originally granted merge authority.

| #107 criterion / exit | Acceptance evidence |
| --- | --- |
| Public V1 surface and version policy | Eight named `consumer_v1` functions, curated existing types, separate facade/document/profile/compiler/package versions, explicit breaking/deprecation/migration rules. |
| Stable domain results/errors without unchecked trust construction | Private error representation, original error serializer/code/path/details, complete returned-type closure and immutable compiler-owned prepared-review alias; positive consumer plus compile-fail opacity checks. |
| Validation, impact, coverage, planning and review | All eight operations directly delegate original buffers once; all are exercised by the facade-only integration consumer. |
| Unavailable mutation/recovery until #106 | No execute/approve/dispatch/retry/recover/journal/path/shell callback under default or all features; future one-writer/authority work remains in #103–#106/#109. |
| Concurrency/ownership documentation | Owned Send/Sync results, caller-bounded synchronous replay, no implicit cache/background/cancellation; dropping a result grants no lease or rollback. |
| Shared CLI/SDK fixtures | Independent literal fields/text goldens, exact frozen proposal, artifact/bundle bytes plus one CLI LF, existing P1/P2 vectors, rich error-byte assertions. |
| Future version/profile rejection | Existing operation-specific diagnostics, limits, first-error/argument order and unsupported-version/profile failures preserved. |
| Migration/deprecation | New facade major before breaking signatures/types/semantics; existing root APIs remain; rollback needs no user-data migration. |
| Independent consumer exit | Separate standalone coverage package plus an external integration-test consumer using only the facade for all eight operations. Automated/AI-authored compatibility evidence; #107 does not require a new human study. |

[Fresh acceptance audit](sdk-acceptance-2026-11-17/sdk-acceptance-audit.txt) found no source blocker. [Fresh parent commands and raw stream hashes](sdk-acceptance-2026-11-17/fresh-verification.json) at exact `5bf480c` passed all-feature release **369/0 failed/3 existing ignored**, SDK **14/14**, the standalone consumer, fmt, Clippy `-D warnings`, rustdoc `-D warnings`, repository checks and a clean tracked diff. The three ignored tests are two informational benchmarks and the pinned isolated FilmCraft case; no Host result is inferred. Earlier source-bound default/debug/release evidence remains in [the implementation record](m4-01-readonly-sdk.md).

The original fresh receipt records argv but omits invocation environment. An [append-only environment note](sdk-acceptance-2026-11-17/invocation-environment.json) transcribes the actual controller's explicit Rust/toolchain/target overrides, including `RUSTDOCFLAGS=-D warnings` for step 5. It is a parent transcription, not an independent environment dump or rerun; original receipt/stream bytes stay unchanged.

| Tested source | Ordinary CI (6 jobs) | Packet OS matrix (3 jobs) |
| --- | --- | --- |
| Final head `5bf480c` | [44069823511](https://github.com/bynanci/storytelling/actions/runs/44069823511), all success | [44069823440](https://github.com/bynanci/storytelling/actions/runs/44069823440), all success |
| Merged main `9d15382` | [44074036241](https://github.com/bynanci/storytelling/actions/runs/44074036241), all success | [44074036178](https://github.com/bynanci/storytelling/actions/runs/44074036178), all success |

## Fifteen remaining issues

Owners below are responsible roles, not new GitHub assignments or purported human decisions. A prepared artifact is distinguished from an accepted implementation or actual observation. The [pre-decision sixteen-issue audit](sdk-acceptance-2026-11-17/issue-convergence-audit.txt) and [original snapshot](sdk-acceptance-2026-11-17/open-issues.json) remain unchanged: their conditional #107 verdict predates the actual acceptance/merge above. The post-disposition snapshot records the later closed state.

| Issue | Current scope / completed foundation | Remaining falsifiable exit and owner / resume condition |
| --- | --- | --- |
| [#103](https://github.com/bynanci/storytelling/issues/103) | Execution ADR/schema/scenarios are proposal deliveries. | Maintainer/reviewer must adopt exact W01 scope; engineer supplies deterministic no-host contract RED/GREEN; final tested authority/provider/fencing/readback contract must be accepted before #104. W01 alone is not full production acceptance. |
| [#104](https://github.com/bynanci/storytelling/issues/104) | Review/journal/isolated M3-03 proof reusable. | After #103 acceptance, engineer implements single writer, durable dispatch, current authority/artifact binding and readback; authorized isolated Host fixture must show no duplicate apply. |
| [#105](https://github.com/bynanci/storytelling/issues/105) | Journal fixes and historical lost-reply proof delivered. | After #104, recovery engineer/operator demonstrates persisted reconciliation across named timeout/crash/late-response boundaries, durable unknown and fresh explicit NotApplied before retry. Maintenance tests are not production recovery. |
| [#106](https://github.com/bynanci/storytelling/issues/106) | One historical isolated FilmCraft profile observation. | After #104/#105, actual operator/build/OS/project/review/receipt matrix and correspondence review; maintainer records accept/narrow/stop M3. |
| [#108](https://github.com/bynanci/storytelling/issues/108) | **Ready for read-only transport G2** after accepted #107. | Consumer engineer/maintainer chooses binding, operations/views, framing/version/error/raw-byte/failure rules before G3; then TS parity and fresh-checkout example. #106 is only an execution/recovery dependency. |
| [#109](https://github.com/bynanci/storytelling/issues/109) | Read-only app design can proceed against accepted SDK. | App owner scopes UI/client first; full acceptance still needs usable app, one writer, identity visibility, receipts, restart and lost-reply demo under accepted #106. TS specifically needs #108. No app is delivered by #149. |
| [#110](https://github.com/bynanci/storytelling/issues/110) | Optional read-only tool design eligible; no MCP schema accepted. | Tool engineer scopes versioned public-SDK tools; full human-review/self-approval/unknown-retry sessions depend on #109/#106. Optional MCP does not block #108. |
| [#111](https://github.com/bynanci/storytelling/issues/111) | Second-editor criteria prepared; none qualified. | Maintainer selects one bounded target after usable #109/M3; authorized operator proves import/play/save/reopen/readback/losses using the same fixtures. |
| [#112](https://github.com/bynanci/storytelling/issues/112) | Prospective beta protocol/forms prepared. | Study owner first accepts #109/#111/M3 and freezes build/material/task/scoring, then facilitator recruits actual five independent users and retains failures/metrics. Earlier readers/models are not these users. |
| [#113](https://github.com/bynanci/storytelling/issues/113) | RC evidence checklist prepared. | After dependencies, release engineer assembles candidate-bound clean-machine install, compatibility, migration/rollback/recovery/security/study evidence. Maintainer's G5 publication remains separate. |
| [#114](https://github.com/bynanci/storytelling/issues/114) | #107 child is now checked complete. | Product owner keeps parent Open until actual production, reference-flow, second-host, beta and RC exits are met or explicitly re-scoped. No percentage inferred from child checkboxes. |
| [#115](https://github.com/bynanci/storytelling/issues/115) | Packet, five captured synthetic responses and two AI reviews delivered; F02 full-layout preflight prepared but unexecuted. | Skill/evaluation owner must adjudicate/version F01's prompt/key scope before rerun/rule change; preserve original divergence/imaginary label. #121/#122 remain separately unmet; no aggregate efficacy/human claim. |
| [#121](https://github.com/bynanci/storytelling/issues/121) | Order/rubric/analyzer/input-hardening delivered. | Data custodian must supply authorized exact original drafts/packets/prompts/model settings and eligible run context before order-only paired rerun. Reconstructed/new data cannot become R1. |
| [#122](https://github.com/bynanci/storytelling/issues/122) | Prospective criteria prepared; no holdout started. | Study owner needs eligible #121 pilot variation and justified instruction revision, then freezes unseen-case metrics/sample/stopping/analysis before observing results. |
| [#129](https://github.com/bynanci/storytelling/issues/129) | A engineering done; B/C proposals and D checklist delivered. | Facilitator supplies actual independent A observation. Maintainer separately adopts B before implementation; C needs privacy/G2/qualified local read view and real-file binding evidence; D keeps five lanes independent until actual media/content/Host/authority evidence exists. |

## Next concrete decision

The immediate follow-up is **#108's read-only binding G2**, not another Rust validator. The review packet must choose transport and supported operation views, preserve exact rational strings and selected proposal/bundle/OTIO bytes, retain source diagnostic objects, reject future versions, and define bounded output/cancellation/process failure. Existing narrative projection intentionally has no core JSON serializer: a bridge must explicitly design its view rather than parse CLI prose or silently extend V1. Process/JSON versus another binding is still a choice, not an adopted contract. #103 W01 and #129 B/C have separate explicit scope decisions; SDK acceptance grants none of them.

New human observations, actual media probes and Host observations in this acceptance: **0**. No author quotes/timings, original R1 inputs, current signer authority, Host success, release or publication is synthesized. Unknown outcomes do not authorize retry or Undo. This documentation changes status/traceability only; Rust, CLI/schema, existing fixtures and evidence scores are unchanged.

## Recheck retained artifacts

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib, json
root = Path('docs/validation/sdk-acceptance-2026-11-17')
index = json.loads((root / 'artifact-index.json').read_bytes())
for item in index['files']:
    raw = (root / item['path']).read_bytes()
    assert len(raw) == item['bytes']
    assert hashlib.sha256(raw).hexdigest() == item['sha256']
print('Pre/post decision artifacts and fresh verification receipts match; human/Host observations: 0')
PY
python3 scripts/check_repo.py
git diff --check
```

Checksums establish retained byte consistency, not identity authentication or semantic correctness. This status-only change adds no runtime behavior and invents no RED/GREEN. The actual implementation RED/GREEN remains intact in #149; final-head and post-merge CI for this reconciliation are recorded on its PR.
