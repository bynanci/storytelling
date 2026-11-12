<!-- SPDX-License-Identifier: MIT -->
# ADR 0010: NarrativeVideoCoverageV1

- **Status:** Proposed G2 contract for maintainer review; mode and version boundary selected.
- **Date:** 2026-11-12
- **Issue:** [#67 / N-10](https://github.com/bynanci/storytelling/issues/67)
- **Selection:** [#53 comment](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432)
- **Depends on:** accepted NarrativeDocumentV1 / ADR 0005 and N-04 revisions / #51.

## Context and author task

The requester selected this task: choose two story units, declare video footage
for one and no asset for the other, then verify the handoff preserves the
uncovered unit as unresolved and blocks ready-for-planning/export status.
N-06's synthetic S02/S03 case shows a contract gap: a Planner can accept an
S02-only plan without knowing S03 was selected upstream. This does not identify
a Planner defect or establish a human failure. No real footage, catalog or
author trial is supplied.

## Proposed decision

Define a separate, closed, versioned `NarrativeVideoCoverageV1` declaration and
read-only validator. It binds a complete NarrativeDocumentV1 snapshot, one
named variant, the author's ordered selection of current source-unit
revisions, a declared asset-catalog snapshot, and one mapping row for every
selected unit in the same order. A row is either:

- `mapped`: an explicit asset ID/revision from the declared catalog and a
  valid half-open rational source range in seconds; or
- `unresolved`: a bounded reason explaining why no asset is assigned.

An `unresolved` row is valid input, remains visible in the deterministic result,
and blocks the `ready_for_planning` status. A complete mapping can enter
planning only; this profile never emits or authorizes an export.

The exact candidate input schema is
[`narrative-video-coverage-v1.schema.json`](../../contracts/narrative-video-coverage-v1.schema.json);
the semantic rules, diagnostic order and result contract are in the
[N-10 profile](../validation/n10-resume-gate.md) and
[paired fixtures](../../tests/fixtures/narrative-video-coverage-v1/README.md).
Those field-level rules still require G2 acceptance. The maintainer's selection
chooses the profile boundary, not automatic acceptance of this detailed schema.

## Binding and validation rules

1. The embedded narrative snapshot is validated by the unchanged
   NarrativeDocumentV1 rules. `variant_id` must resolve to one
   `spoken_outline` or `single_illustration` variant.
2. `selected_unit_refs` is a nonempty ordered subsequence of the exact current
   unit references used by that variant. This expresses the author's selected
   cut; it does not authenticate intent or prove the author selected every
   unit they meant to include.
3. The asset catalog is a declared snapshot identified by `catalog_id` and
   positive `revision`. Asset IDs are unique within it. Each asset declares a
   positive revision and a positive exact duration in seconds.
4. `mappings` has exactly one entry per selected reference, in the same order.
   Missing, duplicate, extra, stale or out-of-variant references reject. A
   mapped asset must resolve to the exact declared asset revision. Reusing one
   asset for multiple units is allowed; no mapping is inferred from matching
   IDs or filenames.
5. Rational values use canonical, reduced, nonnegative u64 decimal strings.
   Zero is `0/1`; denominators are positive. A range is half-open and requires
   `0 <= start < end <= declared asset duration`. No float conversion or
   quantization occurs.
6. `mapped` rows require `asset_ref` and `source_range` and forbid `reason`.
   `unresolved` rows require a nonempty bounded `reason` and forbid asset/range
   fields. Unknown fields and duplicate JSON keys reject.
7. The validator emits a stable result in selected-unit order. Any unresolved
   row sets `ready_for_planning=false` and adds
   `video_coverage.unresolved_required` for that row. A fully mapped valid
   input sets `ready_for_planning=true`. Invalid documents return one stable
   error code and safe JSON path. The same bytes always produce the same bytes.

### Result and command contract

The proposed `render-video-coverage <FILE|->` success result is one compact
UTF-8 JSON line with fixed key order:

```json
{"schema_version":1,"profile":"storytelling.narrative-video-coverage.v1","coverage_id":"two-unit-cut","narrative_ref":{"document_id":"n10-synthetic-story","document_revision":1,"variant_id":"rough-cut"},"asset_catalog_ref":{"catalog_id":"synthetic-catalog","revision":1},"selected_unit_refs":[{"unit_id":"S02","revision":1},{"unit_id":"S03","revision":1}],"mappings":[{"unit_ref":{"unit_id":"S02","revision":1},"status":"mapped","asset_ref":{"asset_id":"CLIP-01","revision":1},"source_range":{"start":{"numerator":"5","denominator":"2"},"end":{"numerator":"9","denominator":"2"}}},{"unit_ref":{"unit_id":"S03","revision":1},"status":"unresolved","reason":"No video asset was selected for this unit."}],"ready_for_planning":false,"diagnostics":[{"code":"video_coverage.unresolved_required","path":"$.mappings[1]"}]}
```

`mappings` preserve selection order. `ready_for_planning` is false when at
least one valid unresolved row exists and true when all selected units have a
valid declared mapping. No `export_ready` field exists. Invalid input returns
one compact JSON line `{"error":{"code":"...","path":"..."}}` (`path` is
`null` if syntax or budget failure prevents a safe JSONPath) and exit
1; valid results exit 0 even when blocked. Usage errors exit 2 and I/O errors
exit 3, following the existing CLI convention. The output carries only the
selected mapping and declared identities; it does not copy or mutate the
narrative, catalog, project or media.

### Bounds and diagnostic policy

Input is limited to 1,048,576 UTF-8 bytes, depth 33 (the embedded narrative
profile allows depth 32 before adding the coverage wrapper), 1–512 selected
references and mappings, 0–1,024 declared assets, IDs of 1–128 ASCII bytes, a positive
u32 document/catalog/asset revision, and unresolved reasons of 1–512 Unicode
scalar values. The embedded NarrativeDocumentV1 keeps its existing limits.
Rational numerator and denominator are canonical ASCII decimal strings no
longer than 20 digits and must fit u64; semantic validation enforces reduction.
Asset duration must be greater than zero.

Parsing/validation order is: raw byte bound; UTF-8/JSON syntax, duplicate-key
and depth preflight; top-level version; closed shape and primitive bounds;
embedded NarrativeDocumentV1; variant and selected-unit binding; catalog
identity and unique assets; mapping coverage/order; asset references and
ranges; then deterministic result/blocker projection. Report the first
contract error in this order, scanning arrays in input order. Safe error paths
never echo source text, unknown keys or parser internals.

Stable profile codes proposed for G2 acceptance:

| Code | Meaning |
| --- | --- |
| `video_coverage.unsupported_version` | Unsupported top-level version |
| `video_coverage.duplicate_field` | Repeated raw JSON property |
| `video_coverage.unknown_field` | Field outside the closed profile |
| `video_coverage.invalid_shape` | Missing, null or wrong-typed field |
| `video_coverage.depth_exceeded` | More than 33 nested containers |
| `video_coverage.too_many_items` | Collection or text limit exceeded |
| `video_coverage.dangling_variant` | Variant ID does not exist |
| `video_coverage.dangling_unit` | Selected/mapped unit ID does not exist |
| `video_coverage.stale_unit_revision` | Unit reference revision is not current |
| `video_coverage.duplicate_selection` | A selected unit ID occurs more than once |
| `video_coverage.unit_outside_variant` | Selected unit is not used by the named variant |
| `video_coverage.selection_order_mismatch` | Selection is not an ordered subsequence |
| `video_coverage.missing_selected_unit` | A selected unit has no mapping row |
| `video_coverage.duplicate_mapping` | A selected unit has multiple mapping rows |
| `video_coverage.mapping_outside_selection` | Mapping row names a unit that was not selected |
| `video_coverage.mapping_order_mismatch` | Mapping rows do not follow selection order |
| `video_coverage.duplicate_asset` | Catalog contains a repeated asset ID |
| `video_coverage.dangling_asset` | Asset ID is absent from the declared catalog |
| `video_coverage.stale_asset_revision` | Asset revision differs from catalog snapshot |
| `video_coverage.invalid_range` | Rational or range violates canonical/bounds rules |
| `video_coverage.unresolved_required` | Valid unresolved row blocks planning; result diagnostic, not parse failure |

Generic input byte/JSON errors and nested narrative errors retain their existing
profile codes. Duplicate catalog asset IDs use
`video_coverage.duplicate_asset`.

## Responsibilities and compatibility

- **Core:** owns closed DTO/enums and checked exact-time values. No parser I/O,
  filesystem, catalog lookup, clock, digest, media, host or inference.
- **Engine:** pure bounded decoding/validation and deterministic result
  projection. It checks declarations only and has no filesystem, model,
  network, database, media, renderer or editor calls.
- **CLI:** one explicit opt-in `render-video-coverage <FILE|->` entry point,
  deterministic JSON result on stdout, stable JSON error on invalid input,
  and existing CLI exit-code conventions. It reads only the supplied manifest
  and writes no files or project state.

The accepted NarrativeDocumentV1, N-04 ledger, N-08 profile, N-11 manifest,
StoryBriefV1, PlannedStoryV1, existing video V1 and OTIO contracts remain
unchanged. The new command is additive and opt-in. Removing it is the rollback;
no existing input migration or catalog mutation is needed.

## Alternatives

| Alternative | Reason not selected |
| --- | --- |
| Add assets and ranges to NarrativeDocumentV1 | Breaks the accepted closed text-only contract and couples writing to media availability. |
| Pass a partial M0 plan directly to Planner | The planner cannot detect a selected unit omitted before its input. |
| Extend N-11 handoff | N-11 groups media artifacts and revision impacts; it does not define a selected video cut's per-unit source ranges. |
| Infer asset IDs or auto-match clips | Names do not prove identity or editorial intent; matching would require separate evidence and authority. |
| Treat unresolved as successful planning | Silently drops a selected story unit and defeats the author task. |

## G2 / G3 gates

G2 remains open until a maintainer accepts or revises this exact field schema,
diagnostic taxonomy, result status and fixture expectations. G3 then requires a
separate implementation PR with observed RED before pure Rust GREEN, CLI
positive/negative replay, input-byte/read-only and deterministic-output tests,
and existing narrative, video V1 and OTIO regressions. G4 requires same-head
hosted CI. Actual asset identity, author adequacy and any release remain
separate evidence and G5 decisions.
