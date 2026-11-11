# Rust boundaries and engineering gaps

<a name="rust-原生化邊界與補強任務"></a>
<a name="先統一原生的意思"></a>
<a name="原有需求與補強的差別"></a>

[繁體中文](NATIVE_BOUNDARIES_AND_GAPS.zh-TW.md) · [Machine-readable inventory](openfilm-port-index.json)

These gaps complement the [48 transfer items](OPENFILM_RUST_PORT_MAP.md). Each entry has a priority, milestone, prerequisite and proposed regression. The [task list](../../TASKS.md) and the corresponding test records define what the team actually accepted.

A pure core or engine can run the same input without Node, Python, SQLite, FFmpeg or the network. A native Rust runtime may use an OS API, a system library or SQLite. An adapter can call FFmpeg, a model or a host. The editor retains mixing, grading, effects, rendering and editing. List production, test and external dependencies separately.

The source guard `provenance_manifest_covers_every_imported_file` covers a subset of GAP-01. The finite source comparisons cover parts of GAP-02 and GAP-03. GAP-04 has a supplied-buffer test. GAP-05 has an OTIO loss contract. GAP-09 has a 25 fps Linux CPU proof. Each has a separate scope. [OTIO](records/M1-02.md) · [Host test](records/M1-03.md) · [Brief audit](records/M2-01.md) · [Planner](records/M2-02.md)


<a name="gap-01授權provenance與原生化邊界"></a>

<a name="真正需要補的16項"></a>

### GAP-01 · Licenses and boundaries

Record each source path, revision, license and change. Retain applicable Apache notices. Record maintainer approval of component and SPDX policy. Missing rights evidence blocks admission. Distinguish pure Rust from runtime and tool dependencies.

- Priority and milestone: `P0`, `M1-01`. Recorded status: `proposed`.
- Prerequisites: None.
- Regression name: `provenance_manifest_covers_every_imported_file`.

<a name="gap-02跨語言conformance與fixture凍結"></a>

### GAP-02 · Cross-language fixtures

Select synthetic cases from upstream tests. Freeze inputs, expected outputs, errors and tool versions. Keep TypeScript in the test environment. Classify a difference as a port defect, upstream defect, unsupported input or intentional change.

- Priority and milestone: `P0`, `M1-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-01`.
- Regression name: `differential_fixture_is_pinned_and_classified`.

<a name="gap-03公共wire與精確時間數字契約"></a>

### GAP-03 · Wire format and time

Preserve M0 v1 or version the change. Separate wire and checked types. Define decimal strings, reduced rationals, Unicode and canonical digests. Adapt each host format. Report losses from rounding.

- Priority and milestone: `P0`, `M1-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-01`.
- Regression name: `cross_language_large_integer_and_unicode_vectors`.

<a name="gap-04declaredverified-evidence信任升級"></a>

### GAP-04 · Declared and verified evidence

Use separate types for declarations and verified data. Record actual hashes, producer and model versions, time and scope. Changes to data or algorithms invalidate evidence. Keep human and model inferences distinct from facts.

- Priority and milestone: `P0`, `M1-01`. Recorded status: `proposed`.
- Prerequisites: `OF-29`.
- Regression name: `declared_hash_never_becomes_verified_by_deserialization`.

<a name="gap-05宿主capability與編譯損失契約"></a>

### GAP-05 · Host capabilities and loss

Identify the host, version, profile, required capabilities and time or ID mapping. Distinguish native, baked, metadata-only and unsupported results. Admit only specific cuts, stills or basic audio to a profile with evidence.

- Priority and milestone: `P0`, `M1-02`. Recorded status: `proposed`.
- Prerequisites: `GAP-03`.
- Regression name: `unsupported_effect_blocks_without_explicit_loss_choice`.

<a name="gap-06人類批准與agent權限分離"></a>

### GAP-06 · Human approval

Bind approval to plan and artifact digests, host snapshot, source versions, capabilities and loss choices. A trusted interface handles expiry and withdrawal. An agent flag or read-only hint cannot grant authority. Keep tokens out of portable projects.

- Priority and milestone: `P0`, `M3-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-03`, `GAP-05`.
- Regression name: `agent_cannot_self_approve_or_reuse_stale_approval`.

<a name="gap-07宿主資料權威id-mapping與drift"></a>

### GAP-07 · Host identity and changes

Prefer a new isolated project. The editor owns the timeline after handoff. Map assets, shots and occurrences to host IDs. A read followed by a write cannot establish atomicity. Require atomic preconditions for in-place changes.

- Priority and milestone: `P0`, `M3-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-05`, `GAP-06`.
- Regression name: `host_manual_edit_invalidates_incremental_plan`.

<a name="gap-08外部副作用的unknown-result恢復"></a>

### GAP-08 · Unknown external results

An SQLite commit cannot include editor side effects. Reconcile the operation, intent, attempt and receipt. After a timeout, read the host state before retry. Preserve unknown or partial results. Do not assume exactly-once execution.

- Priority and milestone: `P0`, `M3-02`. Recorded status: `proposed`.
- Prerequisites: `GAP-06`, `GAP-07`.
- Regression name: `timeout_after_host_commit_does_not_duplicate_import`.

<a name="gap-09結構與媒體結果的獨立驗證"></a>

### GAP-09 · Artifact and media checks

Check source availability, warnings, offline media, host order and ranges, presentation times, duration and artifact hashes. Record structural, rendering, encoding and human review results separately.

- Priority and milestone: `P0`, `M1-03`. Recorded status: `proposed`.
- Prerequisites: `GAP-05`.
- Regression name: `successful_export_with_missing_media_fails_verification`.

<a name="gap-10路徑程序provider信任邊界與預算"></a>

### GAP-10 · Trust and budgets

Limit roots, symlinks, reparse points, size, node count, time, process output, shell and network access. Treat model results, subtitles and filenames as untrusted input. Document actual OS isolation and permissions.

- Priority and milestone: `P0`, `M1-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-01`.
- Regression name: `untrusted_source_text_cannot_expand_capabilities`.

<a name="gap-11跨平台故障與資料庫可回復性"></a>

### GAP-11 · Recovery across platforms

Test Windows paths, case and locks, Linux and macOS fsync and rename, and SQLite busy, WAL and crashes. Address PID reuse, unresolved ownership, queue recovery and data limits.

- Priority and milestone: `P1`, `M3-02`. Recorded status: `proposed`.
- Prerequisites: `GAP-02`.
- Regression name: `crash_lock_and_write_failure_recover_without_loss`.

<a name="gap-12比對並切換至-rust-實作"></a>

### GAP-12 · Switch implementations

Assign one authority per service. Compare both implementations on read-only snapshots before switching with a flag. Test compatibility and rollback. Prevent competing writers. Preserve the live source catalog.

- Priority and milestone: `P1`, `M4-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-02`.
- Regression name: `feature_flag_has_one_writer_and_safe_rollback`.

<a name="gap-13原生cli供應鏈與平台分發"></a>

### GAP-13 · CLI distribution

Pin the toolchain and dependencies. List Rust, C, system, tool and model dependencies per feature in the software bill of materials. Test clean installs on three operating systems. Review the candidate before publication.

- Priority and milestone: `P1`, `M5-02`. Recorded status: `proposed`.
- Prerequisites: `GAP-01`, `GAP-11`.
- Regression name: `native_core_runs_without_node_python_or_media_tools`.

<a name="gap-14公開api與consumertransport一致性"></a>

### GAP-14 · Public interfaces

Share error and data contracts across Rust, CLI JSON and TypeScript. Version the SDK, schemas, planner and adapters independently. Bound responses. Define stable IDs, progress and cancellation. Keep MCP thin. Defer extra language bindings.

- Priority and milestone: `P1`, `M4-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-03`, `GAP-06`.
- Regression name: `external_consumer_and_cli_share_errors_and_results`.

<a name="gap-15實際故事價值與人工工作站驗收"></a>

### GAP-15 · Author and editor trials

Use authorized material to compare integrated and direct host workflows. Record time, preserved constraints, rework and repeat use. Missing media, hardware or human review blocks those observations. Public CI uses synthetic or authorized material.

- Priority and milestone: `P1`, `M5-03`. Recorded status: `proposed`.
- Prerequisites: `GAP-09`, `GAP-12`.
- Regression name: `manual_gate_cannot_be_closed_by_synthetic_metrics`.

<a name="gap-16來源漂移任務證據與差距稽核"></a>

### GAP-16 · Source changes and traceability

Inspect both repositories for each port. Record source revisions, blobs, tests, targets, differences and dependencies. Link task, requirement, test and evidence. Check PR state against GitHub metadata and code. Identify self-review explicitly.

- Priority and milestone: `P0`, `M1-01`. Recorded status: `proposed`.
- Prerequisites: `GAP-01`.
- Regression name: `transfer_manifest_has_no_missing_dependencies_or_stale_claims`.

<a name="授權與來源決策"></a>
<a name="放行與停止條件"></a>

## Source and release decisions

The upstream already has hash checks, owner fencing, file protection and consent mechanisms. Check each mechanism again in the independent SDK, across languages and at host boundaries.

Original Storytelling code uses MIT unless a file specifies a different license. Record each ported component, test, document and derivative separately. For applicable Apache components, retain their SPDX identifiers, licenses and notices. A Rust rewrite preserves the relevant source obligations. Record repository, commit, source path, blob, changes and destination. Assess media, models, fonts and codecs separately. Mark a transfer blocked if rights evidence is missing. [Per-file record](../../PROVENANCE.md) · [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.html) · [Notice guidance](https://infra.apache.org/licensing-howto.html)

1. Narrow the interchange profile if order, time or source ranges change unexpectedly.
2. Keep a small reproducer for an upstream defect. Record any intentional correction.
3. Recheck affected results after a change to source, host or material.
4. Name unavailable tools and human observations as blocked. Continue independent work.
5. Accept only the slice each PR proves.
