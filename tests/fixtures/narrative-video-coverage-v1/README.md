<!-- SPDX-License-Identifier: MIT -->
# NarrativeVideoCoverageV1 contract fixtures

These synthetic examples accompany the proposed G2 contract in
[ADR 0010](../../../docs/adr/0010-narrative-video-coverage-v1.md). They are
not real catalog entries, media verification, author research or executable
regression results. The field-level format remains subject to maintainer G2
review.

The same embedded narrative snapshot contains S02@1 and S03@1 in the
`rough-cut` spoken variant. The explicit selected list chooses both, in that
order. The declared catalog contains only `CLIP-01@1`, with a synthetic
12-second duration.

| Fixture | Contract expectation |
| --- | --- |
| [`valid-complete.json`](valid-complete.json) | Structurally and semantically valid declarations; planning status may be ready. This does not prove the clip contains either event or that export is safe. |
| [`valid-unresolved.json`](valid-unresolved.json) | Valid input. S03 remains visible as unresolved; deterministic result sets `ready_for_planning=false` and includes `video_coverage.unresolved_required`; expected CLI success exit 0 because the document is valid. |
| [`expected-valid-unresolved.stdout.json`](expected-valid-unresolved.stdout.json) | Proposed exact compact stdout for `valid-unresolved.json`; expected only, not captured from a running CLI. |
| [`invalid-missing-selected.json`](invalid-missing-selected.json) | S03 is selected but has no mapping row; reject with `video_coverage.missing_selected_unit`, exit 1. |
| [`invalid-stale-unit.json`](invalid-stale-unit.json) | Selection asks for S02@2 while the embedded snapshot has S02@1; reject with `video_coverage.stale_unit_revision`, exit 1. |
| [`invalid-dangling-asset.json`](invalid-dangling-asset.json) | A row points to an asset absent from the declared catalog; reject with `video_coverage.dangling_asset`, exit 1. |
| [`invalid-range.json`](invalid-range.json) | A range ends after the declared asset duration; reject with `video_coverage.invalid_range`, exit 1. |
| [`invalid-version.json`](invalid-version.json) | Top-level schema version is unsupported; reject before other profile validation with `video_coverage.unsupported_version`, exit 1. |
| [`invalid-duplicate-field.raw`](invalid-duplicate-field.raw) | Duplicate raw top-level property; reject before object decoding with `video_coverage.duplicate_field`, exit 1. This is intentionally `.raw`, because ordinary JSON loaders may silently retain the last value. |

The future opt-in entry point proposed for G3 is:

```sh
cargo run --locked -p storytelling-cli -- render-video-coverage tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

The proposed success result schema is [`narrative-video-coverage-result-v1.schema.json`](../../../contracts/narrative-video-coverage-result-v1.schema.json). The essential blocked-result properties are:

```json
{
  "profile": "storytelling.narrative-video-coverage.v1",
  "coverage_id": "two-unit-cut",
  "ready_for_planning": false,
  "diagnostics": [
    { "code": "video_coverage.unresolved_required", "path": "$.mappings[1]" }
  ]
}
```

G3 must first observe the missing-entry regression fail, then implement the
pure validator and CLI. It must save exact stdout/exit codes, compare repeated
output bytes, prove input bytes are unchanged, and rerun N-02/N-04/N-08/N-11,
Planner V1, video V1 and OTIO tests. This fixture README does not claim those
checks have run.
