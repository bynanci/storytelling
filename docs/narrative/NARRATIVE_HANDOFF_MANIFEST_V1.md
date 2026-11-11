<!-- SPDX-License-Identifier: MIT -->
# NarrativeHandoffManifestV1 profile

English · [繁體中文](NARRATIVE_HANDOFF_MANIFEST_V1.zh-TW.md)

Use a handoff manifest to identify which parts of text, image and audio drafts
must change after a source revision. The [schema](../../contracts/narrative-handoff-manifest-v1.schema.json)
is a separate closed V1 document. It embeds the existing
[NarrativeRevisionLedgerV1](NARRATIVE_REVISION_LEDGER_V1.md), without extending
that ledger or the accepted, closed `NarrativeDocumentV1`.

## Run the example

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/before-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/after-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/descriptions-only-v1.json
```

The workshop date changes from Saturday to Sunday. The expected component sets
were recorded in [expected-impact.json](../../tests/fixtures/narrative-handoff-v1/expected-impact.json)
before the new command ran:

| Action | Components |
| --- | --- |
| `UPDATE` | `invite/text`, `poster/date`, `oral-info/date`, `oral-poetic/date` |
| `REUSE` | `poster/layout`, `oral-poetic/taps`, `original-wav/audio` |

`UPDATE` reports historical impact at the named source-change event. It does
not mean that a reviewed draft still needs work. `status=needs_review` marks a
pending draft; `status=current` marks a draft with a matching declared review.
The after example retains the same impact set and supplies Sunday wording.
See the [N-11 evidence](../validation/n11-handoff.md) for actual output, exit
codes, before/after drafts, replay and source-byte checks.

## Sources and artifacts

The root contains `schema_version`, `manifest_id`, `ledger`, `change_id`,
`sources` and `artifacts`. `change_id` must identify a source-revision event in
the replayed ledger. The engine reuses N-04's direct-reference impact result;
the caller does not provide an affected-variant list.

Each source descriptor names a stable `unit_id` and the **current source
revision** in the ledger. There is one descriptor per listed unit. The tagged
`representation` distinguishes these inputs:

| Kind | Fields | What the declaration means |
| --- | --- | --- |
| `file` | `media_kind`, `relative_path`, `sha256` | A declared local text, image or audio file. The digest has 64 lowercase hexadecimal characters. |
| `description` | `media_kind`, `text` | Supplied text describing material; no original file is claimed. |
| `transcript` | `text` | Supplied textual annotation; the underlying recording is not inspected. |
| `creative_proposal` | `variant_id`, `text` | A proposal whose ledger origin is `creative_proposal`, scoped to that one variant. |
| `missing` | `media_kind`, `reason` | A missing material remains explicit while usable text can still be handed over. |

File paths are at most 1,024 Unicode scalar values. Absolute paths, empty, dot
or dot-dot segments, backslashes, colons and control characters reject. The
engine does not open these paths or calculate their hashes. Use the separate
`verify-source` CLI with actual bytes when digest verification is needed.

Each artifact names its medium, one existing variant ID and its component
source IDs. The engine derives draft revisions from the actual ledger variant
snapshot; callers cannot replace those revisions. All component IDs are unique
within an artifact, and the union of component source IDs must cover **all** of
the bound variant's direct references. Repeating a source in different
components is permitted. A dropped reference rejects with
`handoff.incomplete_mapping` instead of silently suppressing impact.

The entire bound snapshot is checked for proposal scope, including references
that a component map omits. A poster-only proposal cannot enter an oral
variant. Descriptors for creative-proposal ledger units must use the proposal
representation; changing its tag cannot bypass scope.

A pending draft may legitimately retain `S1@1` while the current source is
`S1@2`. The renderer shows draft and current references separately. A **source
descriptor** falsely claiming to be current at `S1@1` rejects with
`handoff.stale_revision`. Prior snapshots remain in the unchanged N-04 ledger.

## Limits, diagnostics and evidence

Input is at most 1 MiB, with the narrative reader's 32-level JSON depth limit
and duplicate-key preflight. All objects are closed. Version must be integer
`1`. IDs use `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`. Source descriptors are limited
to 1–256; artifacts to 1–64; components per artifact and source IDs per
component to 1–64. Text is 1–8,192 Unicode scalar values. The embedded ledger
retains all its existing limits.

The command accepts one file or `-` for stdin. Success is deterministic,
human-readable UTF-8 with exit 0. Invalid data emits one JSON diagnostic with
exit 1. Usage errors use 2 and input/output errors use 3. Diagnostics include
`handoff.dangling_reference`, `handoff.stale_revision`,
`handoff.variant_scope`, `handoff.incomplete_mapping`,
`handoff.duplicate_field`, `handoff.unknown_field` and
`handoff.unsupported_version`; embedded ledger errors retain their existing
codes.

The engine checks declared references and component grouping. It does not
infer whether a date appears in prose, authenticate an author, inspect image
positions, transcribe sound, render a poster, or follow indirect `based_on`
links. The author must review those semantic differences. The CLI reads the
manifest and writes stdout only; it does not rewrite sources, drafts or media.
