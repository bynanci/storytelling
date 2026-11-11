<!-- SPDX-License-Identifier: MIT -->
# N-07 / N-R07 · Native CLI pilot preparation

[繁體中文](n07-pilot.zh-TW.md) · Issue [#54](https://github.com/bynanci/storytelling/issues/54) · SDLC G4–G6.

**Product decision: 待真人證據 (awaiting human evidence).** Available participant records: 0; analyzable paired records: 0. Human completion, revision time, attribution, repeated input, rework and feedback remain unobserved. The [header-only forms and status](../../examples/evaluation/n07-pilot/observations.json) do not encode blank fields as zero success or zero errors. No release candidate is recommended.

## Baseline and version audit

The engineering baseline is main `ce73836caa86502933f538821fdbf5dac92c1224`. It contains merged N-05 PR #83 / ReportV1 and the separately recorded requester gate-sample assessment. Those untimed judgments are not participants or a paired trial. PR #87 ReportV2 is a verified but unmerged candidate at preparation time; it is excluded from the primary pilot. N-06 is optional and its defer-all decision does not block N-07.

The [N-01 protocol at this baseline](https://github.com/bynanci/storytelling/blob/ce73836caa86502933f538821fdbf5dac92c1224/docs/narrative/N-01-AUTHOR-STUDY.md) proposed five new authors, randomized three AB/two BA, eight minutes per arm and the continue/narrow/stop rules before parser work. Agreement and the original Skill pilot are still unrecorded. This is a separately labeled **notes versus native CLI** extension; do not pool Skill, notes/mapped exercises, requester sample reviews or process benchmarks.

The [matched participant snapshot](../../examples/evaluation/n07-pilot/participant-kit/cli/base-document.json) uses a new `n07-door-repair` namespace and exactly the original source-unit objects. Its brief wording is adapted to N-01 without the old N-03 unsupported indoor setting or new C02. Both arms see identical rendered starting words. The correction changes S01/S02 names to Ju/Zhe; C01 stays A-only. The existing N-04 final ledger changes roles and umbrella scope, so it is an engineering control rather than the participant baseline. This native adaptation and threshold transfer need pre-trial agreement.

## Deliverable the facilitator can use

[Packet and exact build/run commands](../../examples/evaluation/n07-pilot/README.md) · [主持人腳本](../../examples/evaluation/n07-pilot/protocol.zh-TW.md) · [Participant kit](../../examples/evaluation/n07-pilot/participant-kit/) · [Facilitator-only key](../../examples/evaluation/n07-pilot/facilitator/key.md) · [Private CSV templates](../../examples/evaluation/n07-pilot/forms/).

The notes arm edits source cards, three text drafts and copy by hand. The native arm edits a new document snapshot (including exact source/revision references and prose), renders three outputs, audits the same copy and submits actual artifacts. It receives no completed after-JSON. The source before-file stays frozen. I01/C01 text, origin and scope remain protected while their changed lineage/revision metadata is explicit. CLI validators do not rewrite prose or judge factual support.

Start the continuous author timer after reading the change card; stop on saved submission or at 480 seconds. Include JSON/metadata work, tool waiting and recovery in author time. Record process wall time separately, without subtracting it. Full commercial claim/proposal subcounts and metadata/rework are secondary diagnostics; do not silently tighten the original N-01 packet-completion rubric. The forms record version, source/material hashes, anonymous ID/order/period, actual denominators, timeout/withdrawal, coded clauses, repetitions, rework and verbatim feedback. Filled records stay private.

The shortest remaining requester/facilitator actions are in the [script](../../examples/evaluation/n07-pilot/protocol.zh-TW.md#你需要做的最短步驟): record agreement and definitions; recruit five new authors and prepare fresh copies; collect both arms and actual artifacts; then apply the frozen rules and record a maintainer decision.

## Original decision rules and current application

| Rule from N-01 | Apply only to valid paired human records |
| --- | --- |
| Continue | B at least 4/5 complete, zero major attribution errors, attribution no worse than A, and at least one median gain: time down 20%, repeated input down one, or attribution errors down one per person. |
| Stop | B fewer than 3/5 complete, at least two participants with major errors, or no observable improvement in quality, time, repetition or attribution. |
| Narrow/repeat | Neither condition applies; preserve a demonstrated benefit and preregister the next scope. A small improvement below 20% is not automatically “no improvement.” |
| Missing records or agreement | **待真人證據**. A valid pair of records may contain incomplete/timeout attempts; it does not require both tasks to succeed. A timeout is incomplete and contributes 480 seconds to the original median. Do not discard it or use finished-only medians. |

“No worse attribution,” withdrawal handling and transfer from Skill to native require agreement before recruitment; proposed operational definitions are explicit in the script. CLI success and n=5 process repetitions cannot satisfy n=5 authors.

## Executed engineering evidence, separate from people

The [build provenance](n07-pilot/build-provenance.json) records Rust 1.89, source/lock identities, the actual development version and binary SHA-256. The actual build used this worktree's unchanged Rust sources at `ce73836`; [the helper results](n07-pilot/engineering/results.json) honestly record a dirty documentation checkout and do not cryptographically authenticate binary-to-source binding. The supplied binary and all declared input hashes stayed unchanged. [Participant material hashes](n07-pilot/material-sha256.json) bind the proposed packet; record the actual delivery commit and protocol hash before a trial.

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

[Next task draft N07-UX01](../../examples/evaluation/n07-pilot/facilitator/N07-UX01.md): define a safe multi-unit correction/review workflow while retaining source history and separate human decisions. This is the strongest current engineering issue to investigate, not a proven highest-value UX fix. Participant reports of metadata/repeated-input burden and misreading `current` remain hypotheses. There are no participant failures to convert into new regressions; the script specifies how to anonymize and reproduce them when supplied.

## N-R07 traceability and verification

| Need | Artifact / evidence |
| --- | --- |
| Ordinary-notes/native matched materials, timing and outcome rules | Participant kit, original source protocol, native facilitator script and header-only forms |
| Explicit zero human evidence and pending product decision | observations.json, this report; no filled participant rows |
| Separate actual CLI timing and input identity | run_cli.py, cases.json, per-case stdout/stderr and all timings/hashes |
| Protect observations, output directories and timing interpretation | helper unit tests: existing directory refusal, malformed manifest, unexpected exits, source/manifest mutation, timeout/replay behavior and warmup exclusion |
| Protect matched sources, after-snapshot revisions, blank forms and guard semantics | test_packet.py |
| RED/GREEN for the new helper | [initial RED](n07-pilot/helper-red.log), [manifest-mutation RED](n07-pilot/helper-manifest-red.log), [initial helper GREEN](n07-pilot/helper-green.log), [deep-manifest RED](n07-pilot/engineering/helper-depth-red.log), [final GREEN: eight helper/four packet tests](n07-pilot/engineering/helper-depth-green.log) |
| Old SDK flows and current submitted head | Workspace/fmt/Clippy/rustdoc/repository checks plus the PR's exact-head hosted CI; no Rust contract or algorithm changes |

The [technical review](n07-pilot/technical-review.md) separates independent protocol review from independent helper-code review and records the resolved error-handling finding. All [local checks](n07-pilot/local-checks.json) passed. Human protocol agreement and participant assessment are separate. CI for the delivery head is recorded on the PR.

## Release disposition

No RC is recommended with missing paired evidence and agreement. SDK/CLI/wire/digest/OTIO contracts and locked dependencies are unchanged. This PR adds evaluation materials and a bounded Python measurement helper only; it writes a fresh engineering output directory and does not edit media, catalogs or projects. Roll back by ceasing to use the helper/packet; there is no user-data migration. Remaining risks include same-case learning, metadata overhead, semantic/source attribution judgments, absence of participant observations and the accepted multi-source pending-review boundary. G5 compatibility/SBOM/migration/readback and explicit maintainer publication approval remain separate if an RC is later proposed.
