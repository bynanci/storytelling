<!-- SPDX-License-Identifier: MIT -->
# M4-02 / #108 · Read-only transport G2 record

[繁體中文 packet](../proposals/m4-02-readonly-transport/README.zh-TW.md) ·
[English packet](../proposals/m4-02-readonly-transport/README.md) ·
[ADR 0014](../adr/0014-readonly-process-transport-v1.md)

## Actual request and baseline

The requester said:

> #108唯讀 transport G2
>
> https://github.com/bynanci/storytelling/blob/main/docs%2Fvalidation%2Fsdk-acceptance-2026-11-17.zh-TW.md

Latest main was read through GitHub and fetched locally as
`62541d91c0c52ec6e49b5dc9c8350e92e83b409a` on 2026-11-17 UTC.
#108 is Open, has no client/transport and no initial comments; no overlapping
open PR existed. Read AGENTS/SDLC, README/TASKS/ROADMAP/requirements, accepted
ADR 0013, consumer source/tests, current DTOs and the linked acceptance record.
#107 is completed; this work does not recreate its facade.

The engineer recommended Node local process/raw-buffer framing over WASM/addon.
The requester answered the actual preference question on 2026-11-17:

> 本機程序＋原始 bytes 框架；先支援 Node.js（建議）

The question explicitly stated that choosing a proposal direction does not
approve a new contract or authorize implementation. This records the direction
selection only; exact fields, limits, error/lifecycle policy and G3 remain pending.

## G0/G1 scope and result

Task M4-02, requirement M4-R02. First independent deliverable is a G2 packet:
ADR, exact protocol, owned TypeScript API/views, three proposed schemas,
paired synthetic examples, source identities and named acceptance cases.
The issue's actual TypeScript client/parity/failure/fresh-checkout criteria stay
unchecked until implementation and G4 evidence exist.

Use case: an explicit local application sends declared story data to the
accepted eight-operation facade, inspects direct revision impact, unresolved
coverage, deterministic plan and selected-proposal review without a Host/model.
Success is complete source-equivalent structured results/errors plus identical
selected raw bytes; it is not author approval, actual-media verification or
execution readiness. Transport limits may reject an oversized otherwise valid
result as a separate resource failure.

## Candidate boundary and implementation work packages

| Package after adoption | Concrete scope / test-first exit |
| --- | --- |
| T01 · wire parsing | New opt-in CLI entry and isolated bridge module in storytelling-cli; bounded STRO framing, strict header decoder, EOF, version/role/order cases. Propose direct serde at existing workspace pin =1.0.229 and the explicit check_repo CLI-whitelist adjustment; no new graph version. Real CLI assertion RED before dispatcher. |
| T02 · read-only mapping | Exactly eight facade delegates; binding-only narrative view; direct result/error serializer; single frozen planner wrapper; same compiler-owned preparation accessors. No core/engine behavior or digest change. |
| T03 · independent TS | Unpublished own package/lockfile, owned public types/decoder, explicit executable configuration, snapshots, process lifecycle and structured failures. Fake-child RED/GREEN first. |
| T04 · parity/regression | All named cases, source/error raw bytes, no mutation, fresh checkout and existing workspace flows; CLI/SDK/TS comparisons plus independent literal goldens. |
| T05 · hosted/performance | Add TS/bridge Node×OS jobs to required gates, exact-head/post-merge checks; separately measure process time and memory without human claims. |

Builder choices: internal Rust module split, audited strict-JSON implementation,
test helper implementation and locked TS tooling, provided they satisfy the
proposed field/byte/lifecycle rules and dependency review. No fifth Rust crate,
new dependency, module or npm package is added by this G2-only change.

## Decision that remains pending

- [x] Choose the process/raw-byte transport direction with Node.js first (actual preference reply above; not complete G2 adoption).
- [ ] Accept exact framing, operations/roles/views, schemas, bounds/error order and the proposed existing-serde CLI dependency/whitelist adjustment.
- [ ] Accept once-frozen selected bytes and untrusted TS review-view boundary.
- [ ] Accept lifecycle/cancellation/cleanup, privacy and compatibility rules.
- [ ] Accept T01–T05 test-first scope and authorize independent G3 implementation.

Actual maintainer reply / date / exact candidate head: **pending**.
Record the person's exact words and accepted/narrowed scope here after a real
decision. Do not fill these boxes from AI review, CI or lack of a reply.
#108 stays Open. G3 is **blocked on this decision**; #103–#106, #109 and G5
remain separate.

## Evidence classification and present limits

Candidate schemas/examples and their hashes are authored data. Static checks
can verify structure, raw frame lengths and internal consistency; they do not
execute a transport. Existing Rust regressions, when run, verify unchanged
backends only. An AI builder/security review is technical review, not a human
study or G2 adoption. No new runtime RED/GREEN is claimed for this design.

Human observations: **0**. Media probes: **0**. Host observations: **0**.
No author quote/timing, current authority, G5 publication or Host receipt is
inferred. Future execution/recovery needs the separately accepted M3 boundary;
there is no hidden writer, Agent self-approval, unknown-outcome retry or Undo.

## Actual local checks and independent technical review

[23 immutable artifacts](m4-02-transport-g2/artifact-index.json) retain raw streams,
commands, explicit environment, schema counterexample and both AI reviews.
These are separate evidence classes:

| Actually observed | Result / limit |
| --- | --- |
| [Documented static replay](../proposals/m4-02-readonly-transport/verification.md) | 3 schemas, 21 authored example hashes, 17 structural validity labels, 13 original Git identities, 64 planned cases. Four intentionally corrupt frame vectors are hash-checked only. Runtime cases executed: 0. |
| [Existing Rust regression receipt](m4-02-transport-g2/runtime-regression.json) | Default debug 350 passed; all-feature debug 369; all-feature release 369. Each 0 failed / 3 existing ignored. fmt, all-target/all-feature Clippy and rustdoc with warnings denied passed. |
| [Packet/repository receipt](m4-02-transport-g2/packet-checks.json) | Exact documented Python snippet, repository links/provenance/dependency/traceability check and diff whitespace passed. |
| [Initial AI review](m4-02-transport-g2/builder-review-initial.txt) | Four material questions retained as the original observation; not rewritten after correction. |
| [AI follow-up](m4-02-transport-g2/builder-review-follow-up.txt) | Independently replayed static checks and found “nothing important left” for planning the proposed scope after actual adoption. This does not approve G2. |

The four design resolutions are explicit: bounded request preflight before
copying, structured early rejection versus provisional stdin EPIPE, exact ASCII
request IDs (including the observed trailing-LF schema counterexample), and
opaque compiler/package metadata versus pinned wire/views/profiles.
The [schema counterexample receipt](m4-02-transport-g2/schema-counterexample.json)
records actual before/after validity; it is a G2 schema correction, not a
product transport RED/GREEN.

The follow-up hashes identify the reviewed protocol/types/schemas/inventory
before this evidence appendix was attached. Final-head CI and any later action
belong on the PR; no not-yet-completed result is inserted here. G2 adoption still
requires an actual exact-head decision.

Local runtime checks used Rust 1.89.0 and a dedicated target. Debug symbols were
disabled via explicit Cargo profile environment overrides to bound disk use;
debug-mode tests were still debug builds. The complete overrides, argv and
stream SHA-256 are in the receipt. No crate/source/CLI/schema/dependency/Skill,
old golden or original research score changed. Ignored tests are the two
informational benchmarks and historical isolated FilmCraft case, not new Host
evidence.

Recheck archived bytes separately:

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib, json
root = Path('docs/validation/m4-02-transport-g2')
for item in json.loads((root / 'artifact-index.json').read_bytes())['files']:
    raw = (root / item['path']).read_bytes()
    assert len(raw) == item['bytes']
    assert hashlib.sha256(raw).hexdigest() == item['sha256']
print('23 archived artifact byte identities match; transport runtime/human/Host: 0')
PY
```

## Base synchronization after PR #152 · 2026-11-17

Main advanced to `26602d37d849d06ceac1d4a09f7ca19d2949c990` after the initial
G2 candidate. GitHub reported the Draft as conflicting; both branches had
appended independent evidence paths to `.gitattributes`. Preserve both rules.
The [static replay](../proposals/m4-02-readonly-transport/verification.md) now
distinguishes that scope base from the original pinned source base `62541d9`.

The [append-only synchronization receipt](m4-02-transport-g2/base-sync-checks.json)
records five passing precommit commands, exact raw UTF-8 stdout/stderr and their
byte hashes, plus the two replay sources. It rechecked three schemas, all 21
authored example hashes, 17 structural labels, 13 source identities and 64
unexecuted cases; the unchanged proposal files and original 23 transport / 33
merged layout receipts; repository checks; 41 offline packet/layout tests;
and whitespace. These checks ran on the merged working tree. Exact-head hosted
CI belongs on the updated Draft PR after submission.

The protocol, types, schemas, examples, cases and historical 23-artifact index
remain byte-identical to candidate `c477bf9`. This is conflict maintenance,
not revised contract semantics, runtime RED/GREEN or G2 adoption. Transport
runtime cases, human and Host observations remain zero. Draft remains pending;
no G3 implementation, product authority or release decision is inferred.

main 更新後，兩邊各自在 `.gitattributes` 新增原始證據路徑，造成衝突。
本次保留兩組規則，並將 PR 範圍基準與原始來源基準分開。新增收據記錄
五個成功命令及 41 項唯讀測試；原契約、範例、待實作案例與歷史收據不變。
這次是同步修復，完整 G2 仍待採納，沒有新增 transport、真人或 Host 成果。
