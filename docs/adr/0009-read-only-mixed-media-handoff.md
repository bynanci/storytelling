<!-- SPDX-License-Identifier: MIT -->
# ADR 0009: Read-only mixed-media handoff manifest

- **Status:** Proposed for N-11 / N-R11 review
- **Date:** 2026-11-11
- **Issue:** [#68](https://github.com/bynanci/storytelling/issues/68)
- **Base:** `main` `003191c834bfc46903f1a436fd73c5df6edd9a77`
- **Depends on:** accepted ADR 0005 and merged N-04 / #51

## Context

N-04 can replay stable source IDs and revisions and report directly affected
variants. A handoff also needs to distinguish actual-file declarations,
descriptions, supplied transcripts, variant-scoped creative proposals and
missing material. A poster can contain a changed date and an unchanged layout.
The accepted NarrativeDocumentV1 is closed and deliberately has no media paths.

## Proposed decision

Add a separate closed NarrativeHandoffManifestV1 containing an unchanged N-04
ledger, one source-change event ID, current source descriptors and artifact
component groups. Reuse the existing replay result for impact. Derive draft
references from the bound ledger snapshot, and require the component union to
cover all direct references. Check creative-proposal scope across the entire
bound snapshot so a partial map cannot conceal an out-of-scope reference.

Keep historical impact (`UPDATE`/`REUSE`) separate from current draft review
status. An old reference in a pending draft is valid history; a source
descriptor claiming the old version is current is invalid. File paths and
hashes remain declarations. Actual bytes require a separate supplied-buffer
verification. No parser result establishes editorial truth or author identity.

Core owns wire types; the pure engine validates and projects; the CLI reads
one explicit manifest and writes stdout. No media or host operation is added.
The [profile](../narrative/NARRATIVE_HANDOFF_MANIFEST_V1.md) defines limits,
replay semantics and rollback: stop using the new command. Existing narrative,
ledger, video V1 and OTIO inputs need no migration.

## Alternatives

| Option | Reason not selected |
| --- | --- |
| Extend NarrativeDocumentV1 with paths and media fields | Changes an accepted closed contract and burdens text-only consumers. |
| Treat every poster part as affected | Rewrites the unchanged layout when only the notice changes. |
| Trust a caller-supplied affected list or incomplete source map | Can omit an affected source or conceal a variant-only proposal. |
| Infer meaning from text or compare only file hashes | Does not establish source scope or semantic revision impact. |

## Verification and limits

The [fixtures](../../tests/fixtures/narrative-handoff-v1/README.md) fix expected
impact before execution. CLI tests cover the valid file and description cases,
missing/stale references, scope leakage, incomplete mapping and deterministic,
read-only replay. The [validation record](../validation/n11-handoff.md) records
actual RED/GREEN, output, exit codes and compatibility checks. Human semantic
review and maintainer acceptance remain separate from those checks.
