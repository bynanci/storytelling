<!-- SPDX-License-Identifier: MIT -->
# N-07 / N-R07 · Pilot disposition and frozen native evidence

[繁體中文](n07-pilot.zh-TW.md) · Issue [#54](https://github.com/bynanci/storytelling/issues/54) · SDLC G4–G6.

**Maintainer decision: Stop the submitted external-wrapper evaluation; no release.** On 2026-11-12 (Asia/Taipei), the maintainer [closed #54 as not planned / stopped](https://github.com/bynanci/storytelling/issues/54#issuecomment-6809828575), accepting **0/5 primary completions and 5/5 authors with at least one major U01 attribution error**. These are scores of the submitted work products. They do not establish a defect in the native Rust executable or a passing native trial. The maintainer elected not to ask those authors to repeat their work.

The [agreed native protocol](https://github.com/bynanci/storytelling/issues/54#issuecomment-6796082010) pinned `n07-door-repair@1` and ReportV1 at `ce73836caa86502933f538821fdbf5dac92c1224`. **Verified eligible pairs for that frozen build: 0; native acceptance: 待真人證據.** This evidence gap is separate from the actual Stop decision for the submitted external workflow. The [N-01 Continue decision](https://github.com/bynanci/storytelling/issues/48#issuecomment-6798756961) concerns the separate Skill trial and is not pooled here.

## Submitted workflow: scores, timing and provenance

| Item | Available record and limit |
| --- | --- |
| Submitted denominator | Five authors / ten reported timed attempts, three AB and two BA. Earlier account-review pairs and different maintenance scenarios remain separate supplementary submissions. Do not add their counts. |
| CLI-labeled primary completion | **0/5**, superseding the provisional intake label of five completions. Each submitted author attributed unsupported U01 to an external source; correct U01 handling is required for completion. |
| Major attribution | **5/5 authors; at least five events**. Count the same judgment in a log and claim once per author. Preserve the original erroneous answers. |
| Reported author timing | Notes median 390 s; CLI-labeled median 190 s; all ten reported times below 480 s. The descriptive difference is 51.3%; it is not evidence of benefit from the frozen native build. |
| Unknown or unverified measures | Ordinary-notes scoring, withdrawals, manual repeated entries, rework, feedback and round-specific process timing. Retry counts do not establish repeated factual entries. Keep unknown values blank. |
| Native comparability | The later manifest identifies `489273b`, not frozen `ce73836`; their Rust source trees differ. Compatible ReportV1 output and the shared development version label do not identify the executable used. |
| Command / transformation trace | The supplied `storytelling process --scenario ...` command is unsupported by the native dispatcher. Native `audit-copy-claims` returns `ok` and `commercial_copy`, with U01's reference missing. The supplied jq filter copies existing strings from an alternate envelope and fails on archived native ReportV1 stdout. The producing external process remains unidentified. |
| Actual disposition | **Stop / no release**, supplied by the maintainer for this external workflow. #54 is closed as not planned; the frozen native acceptance criteria are unverified. |

Evidence: [scoring](https://github.com/bynanci/storytelling/issues/54#issuecomment-6800753461), [build provenance review](https://github.com/bynanci/storytelling/issues/54#issuecomment-6800945586), [dispatch/jq trace](https://github.com/bynanci/storytelling/issues/54#issuecomment-6809399303), and [actual maintainer disposition](https://github.com/bynanci/storytelling/issues/54#issuecomment-6809828575). Public comments summarize private submissions; this report does not republish participant identities or invent unavailable verbatim records.

## Baseline and version audit

The engineering baseline is main `ce73836caa86502933f538821fdbf5dac92c1224`. It contains merged N-05 PR #83 / ReportV1 and the separately recorded requester gate-sample assessment. Those untimed judgments are not participants or a paired trial. PR #87 subsequently merged the opt-in ReportV2. This pilot uses ReportV1. N-06 is optional and its defer-all decision does not block N-07.

The [N-01 protocol at this baseline](https://github.com/bynanci/storytelling/blob/ce73836caa86502933f538821fdbf5dac92c1224/docs/narrative/N-01-AUTHOR-STUDY.md) proposed five new authors, randomized three AB/two BA, eight minutes per arm and the continue/narrow/stop rules before parser work. This is a separately labeled **notes versus native CLI** extension; do not pool Skill, notes/mapped exercises, requester sample reviews or process benchmarks.

The [matched participant snapshot](../../examples/evaluation/n07-pilot/participant-kit/cli/base-document.json) uses a new `n07-door-repair` namespace and exactly the original source-unit objects. Its brief wording is adapted to N-01 without the old N-03 unsupported indoor setting or new C02. Both arms see identical rendered starting words. The correction changes S01/S02 names to Ju/Zhe; C01 stays A-only. The existing N-04 final ledger changes roles and umbrella scope, so it is an engineering control rather than the participant baseline. The maintainer accepted this adaptation and the threshold transfer in the linked native protocol decision.

## Preserved facilitator handoff

[Packet and exact build/run commands](../../examples/evaluation/n07-pilot/README.md) · [主持人腳本](../../examples/evaluation/n07-pilot/protocol.zh-TW.md) · [Participant kit](../../examples/evaluation/n07-pilot/participant-kit/) · [Facilitator-only key](../../examples/evaluation/n07-pilot/facilitator/key.md) · [Private CSV templates](../../examples/evaluation/n07-pilot/forms/).

The notes arm edits source cards, three text drafts and copy by hand. The native arm edits a new document snapshot (including exact source/revision references and prose), renders three outputs, audits the same copy and submits actual artifacts. It receives no completed after-JSON. The source before-file stays frozen. I01/C01 text, origin and scope remain protected while their changed lineage/revision metadata is explicit. CLI validators do not rewrite prose or judge factual support.

Start the continuous author timer after reading the change card; stop on saved submission or at 480 seconds. Include JSON/metadata work, tool waiting and recovery in author time. Record process wall time separately, without subtracting it. Full commercial claim/proposal subcounts and metadata/rework are secondary diagnostics; do not silently tighten the original N-01 packet-completion rubric. The forms record version, source/material hashes, anonymous ID/order/period, actual denominators, timeout/withdrawal, coded clauses, repetitions, rework and verbatim feedback. Filled records stay private.

The packet and its original allocation are preserved records, not an active recruitment list. Any future native evaluation uses a separately versioned, prospectively agreed protocol and fresh authors, as required by the maintainer disposition. That follow-up keeps matched starting drafts, three AB/two BA assignments and actual-order logs, all starts and withdrawals, original outputs and private feedback. Apply the frozen rules only to five comparable pairs in that round, including incomplete/timeout attempts with the required observations. The published procedure retains author waiting time and incomplete timeouts at 480 seconds; process milliseconds remain separate. Record the future maintainer's actual decision independently from a threshold recommendation.

## Original decision rules and current application

| Rule from N-01 | Apply only to valid paired human records |
| --- | --- |
| Continue | B at least 4/5 complete, zero major attribution errors, attribution no worse than A, and at least one median gain: time down 20%, repeated input down one, or attribution errors down one per person. |
| Stop | B fewer than 3/5 complete, at least two participants with major errors, or no observable improvement in quality, time, repetition or attribution. |
| Narrow/repeat | Neither condition applies; preserve a demonstrated benefit and preregister the next scope. A small improvement below 20% is not automatically “no improvement.” |
| Missing records or agreement | **待真人證據**. A valid pair of records may contain incomplete/timeout attempts; it does not require both tasks to succeed. A timeout is incomplete and contributes 480 seconds to the original median. Do not discard it or use finished-only medians. |

The accepted native protocol defines the attribution comparison and withdrawal handling. Five CLI process repetitions measure engineering performance.

## Executed engineering evidence, separate from people

The [build provenance](n07-pilot/build-provenance.json) records Rust 1.89, source/lock identities, the actual development version and binary SHA-256. The actual build used the preparation worktree's unchanged Rust sources at `ce73836`; [the helper results](n07-pilot/engineering/results.json) honestly record a dirty documentation checkout and do not cryptographically authenticate binary-to-source binding. The supplied binary and all declared input hashes stayed unchanged. [Participant material hashes](n07-pilot/material-sha256.json) are the preparation receipt at [packet delivery `8e2c080`](https://github.com/bynanci/storytelling/tree/8e2c08094f6bce70a7507e11ed84265bd98e3820/examples/evaluation/n07-pilot); later README status updates do not rewrite that receipt. Use that delivery checkout to verify all seventeen historical hashes. Any future trial records its own delivery commit and protocol hash.

Eleven cases ran one excluded warmup plus five measured process runs each. Every expected exit matched, repeated stdout/stderr bytes matched and input/manifest/binary hashes were unchanged. Rejections and missing/revised citation reports are expected controls, not completed human tasks. [Semantic checks](n07-pilot/semantic-checks.json) verify the expected two/three independent impact sets, correct snapshot revisions, the same historical copy status, and the batch guard.

| Engineering case | Expected / observed | Median process wall time |
| --- | --- | ---: |
| Matched baseline render | Exit 0 | 1.088 ms |
| Corrected versioned snapshot render | Exit 0 | 1.147 ms |
| Current/missing copy references | Exit 0, reference statuses only | 1.284 ms |
| Revised reference control | Exit 0 / revised | 1.448 ms |
| Dangling reference control | Exit 0 / dangling | 1.569 ms |
| Unknown copy version control | Exit 1 / copy.unsupported_version | 1.080 ms |
| Assistant self-confirmation control | Exit 1 / revision.untrusted_decision | 1.251 ms |
| Narrative missing-reference control | Exit 1 / narrative.dangling_reference | 1.027 ms |
| Independent S01 / S02 impact probes | Exit 0; separate histories, not one batch | 1.310 / 1.270 ms |
| Two-source batch guard | Exit 1 / revision.pending_review | 1.155 ms |

These are one shared Linux sandbox's five warm process observations, including startup, file I/O and serialization. Build time is excluded; CPU pinning/cache/scheduler were uncontrolled. Min/max/all samples are retained in the results. These values make no speedup, human-effort or cross-platform benchmark claim.

## Reproduced limitation and next issue draft

The initial preflight expected two consecutive source revisions to work; [the actual failure](n07-pilot/preflight-finding.json) returned `revision.pending_review` on the second change. The unchanged [synthetic reproducer](../../examples/evaluation/n07-pilot/facilitator/ledger-batch-pending.json) retains that guard as an expected rejection. The existing engine test `revision_ledger_requires_current_revisions_and_review_before_next_source_change` already protects this accepted policy. No runtime defect or user failure is inferred.

The participant workflow now uses supported snapshot editing/rendering; it does not prefill or fabricate intermediate `variant_review` events. The original before-file is preserved, but these two snapshots are not an atomic append-only N-04 multi-source history. This boundary is an explicit cost/risk to observe.

[Task draft N07-UX01](../../examples/evaluation/n07-pilot/facilitator/N07-UX01.md) retains the named engineering question: safe multi-unit correction/review history without invented human decisions. It is not a proven UX priority. The submitted major U01 errors are preserved as workflow observations; the producing command does not establish a reproducible native defect. The maintainer stopped further wrapper investigation. Any later reproducible failure needs an identified build, actual inputs, expected/actual output and a minimal synthetic regression before a named implementation task; do not infer a core fix from the external envelope.

## N-R07 traceability and verification

| Need | Artifact / evidence |
| --- | --- |
| Ordinary-notes/native matched materials, timing and outcome rules | Participant kit, original source protocol, native facilitator script and header-only forms |
| Baseline at packet preparation: no participant submissions | [Preparation snapshot](../../examples/evaluation/n07-pilot/observations.json) and blank participant forms |
| Current submitted outcomes and actual decision | Scoring/provenance/trace comments above and the dated Stop disposition; frozen native acceptance remains unverified |
| Separate actual CLI timing and input identity | run_cli.py, cases.json, per-case stdout/stderr and all timings/hashes |
| Protect observations, output directories and timing interpretation | helper unit tests: existing directory refusal, malformed manifest, unexpected exits, source/manifest mutation, timeout/replay behavior and warmup exclusion |
| Protect matched sources, after-snapshot revisions, blank forms and guard semantics | test_packet.py |
| RED/GREEN for the new helper | [initial RED](n07-pilot/helper-red.log), [manifest-mutation RED](n07-pilot/helper-manifest-red.log), [initial helper GREEN](n07-pilot/helper-green.log), [deep-manifest RED](n07-pilot/engineering/helper-depth-red.log), [final GREEN: eight helper/four packet tests](n07-pilot/engineering/helper-depth-green.log) |
| Old SDK flows and current submitted head | Workspace/fmt/Clippy/rustdoc/repository checks plus the PR's exact-head hosted CI; no Rust contract or algorithm changes |

The [technical review](n07-pilot/technical-review.md) separates independent protocol review from independent helper-code review and records the resolved error-handling finding. All [local checks](n07-pilot/local-checks.json) passed. Human protocol agreement and participant assessment are separate. CI for the delivery head is recorded on the PR.

## Release disposition

**No release or RC is authorized.** The actual decision is Stop for the submitted external-wrapper attempt, and native acceptance remains unverified. This documentation reconciliation changes no SDK/CLI/wire/digest/OTIO contract, dependency, participant input, source-key fixture or archived output. Reverting these status edits affects documentation only; the original submissions and maintainer disposition remain evidence. Any future evaluation and G5 compatibility/SBOM/migration/readback/publication decision are separate.
