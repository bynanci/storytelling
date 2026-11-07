# M0 contracts

`PlanDocumentV1` is an experimental, finite structural subset, not the complete
future Story SDK or an OpenFilm import format. Its reference is
[the minimal fixture](../examples/minimal-plan.json) and
[strict wire DTOs](../crates/storytelling-core/src/wire.rs).

## Document

Required root fields: `schema_version` (integer 1), `plan_id`, `assets`, `shots`.
Each asset has `id`, `content_hash`, `duration`. Each ordered shot has `id`,
`asset_id`, `source_range: {start,end}`. No optional extension objects are accepted.
Unknown/duplicate JSON fields and malformed shapes are rejected.

Identifiers use `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`. Asset IDs and shot IDs are unique
within their own collections. Multiple shots may reference the same asset.
Hashes are 64 lowercase hexadecimal characters; this checks spelling only.

Time is `{numerator: string, denominator: string}` in seconds. Each string is a
canonical nonnegative decimal u64; denominator > 0 and gcd(n,d) = 1. Thus zero is
`0/1`, `2/2` and `00/1` are refused, and large integers survive JS transport as
strings. Source ranges are half-open with `0 <= start < end <= duration`.
Duration must be positive. No source PTS, FPS or audio-rate conversion is inferred.

Input limit 1,048,576 bytes; 1–1024 declared assets; 1–4096 shots. Serde_json's
normal recursion limit remains enabled. No arbitrary file/URL/script/provider
fields. M0 does not open declared source media or promise timelines/media formats.

## CLI

```text
storytelling validate <FILE|->
storytelling --help
storytelling --version
```

The caller explicitly supplies a local JSON document, or `-` for stdin. File
symlinks may resolve to a regular file; this local read-only CLI is not a sandbox
or server path broker. stdin intentionally waits for EOF. Output is one JSON line
for validate, with no logs interleaved. Help/version are plain text.

Success: `{"ok":true,"validation":{"schema_version":1,"plan_id":"...","assets":1,"shots":2,"scope":"declared-metadata-only"}}`.
Failure: `{"ok":false,"error":{"code":"plan.source_out_of_bounds","path":"$.shots[0].source_range.end"}}`.
Errors never echo source content or local paths. If stdout itself fails no JSON
response is promised; the process exits 3.

| Exit | Meaning |
| --- | --- |
| 0 | Declared document validated, or help/version printed |
| 1 | Invalid JSON/version/metadata/time/reference/bounds/size |
| 2 | Invalid CLI command or arguments |
| 3 | Input/output failure |

Stable codes: `input.invalid_json`, `input.too_large`, `input.io`, `cli.usage`,
`plan.unsupported_version`, `plan.invalid_id`, `plan.invalid_hash`,
`plan.invalid_time`, `plan.invalid_range`, `plan.source_out_of_bounds`,
`plan.duplicate_id`, `plan.unknown_asset`, `plan.empty_collection`,
`plan.too_many_items`. Diagnostic paths may be refined; callers should branch on
codes, not prose or line/column messages.

## Future public workflow (design only)

`observe -> propose -> compile -> preview -> approve -> executeApproved -> verify`.
M0 has no approve/apply command, token, database or host adapter. Future contracts
must distinguish StoryBrief, evidence, immutable proposal, compiled losses,
approval and execution receipt. Approval must bind actual source identities,
proposal/compiled-plan identity, host snapshot and accepted degradation choices.
