# M1-01 finite-v1 import contract

G2 frozen after independent profile review on 2026-11-07. Source baseline:
`bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76`.
The maintainer approved the separate Apache-2.0 component/provenance policy in
the 2026-11-07 task conversation; original MIT LICENSE remains unchanged.

## Input and authority

Accept the existing `exportTimeline("json", composition, assets, settings)`
composition envelope, not an OpenFilmProject or live Catalog. Root fields are
exactly `schemaVersion:"1.0.0"`, `composition`, `assets`, `settings`.
Unknown/duplicate fields and explicit null optional declarations reject (missing
or null contentHash specifically returns plan.invalid_hash). No schema migration, media read,
network request, hash verification, host command or write occurs.

`import_openfilm_snapshot(bytes: &[u8], profile: &ImportProfileV1)
-> Result<ImportedPlanV1, ImportError>` uses the single fixed finite-v1 profile.
Core `compatibility.rs` owns profile and loss/error contracts; engine owns raw
snapshot decoding, exact decimal arithmetic, validation and serialized result.
The engine returns a validated M0 `PlanDocumentV1`, complete original JSON snapshot,
original frame-rate lexeme, separate optional explicit timebase, and loss report.
Source JSON values and number lexemes are retained without f64 rewriting;
insignificant whitespace is removed outside strings for single-line CLI output.
Original input bytes remain unchanged. No digest/evidence identity is minted.

Every success has `scope:"declared-visual-mapping-only"`, `executable:false`.
Every source hash is declared only. A plan can be fed to existing M0 `validate`;
it has no host capability/approval status. Unsupported input returns an error and
structured loss, with no plan. Error paths use fixed field names/indices, never
media URIs, arbitrary metadata keys, JSON excerpts or parser error messages.

## Exact allowlist and mapping

- Composition required: `id`, `storyId`, positive `duration`, `tracks`. Exactly
  one track; required `id`, `type:"video"`, nonempty `clips`. Optional track
  `locked`, `target`, `visible`, `muted`, `solo` are booleans; visible=false,
  muted=true or solo=true reject. All other composition/track fields reject.
- Clip required: `id`, `assetId`, `timelineStart`, positive `timelineDuration`.
  Optional `beatId`, `sourceIn`, `sourceOut`, `locked`. Transform or any creative
  extension rejects on presence, including identity/empty transform, audioTiming,
  coverage, userOverrides, transitions, effects and current-main animation.
  Locks and user selection facts are preserved in the original source snapshot;
  no host lock enforcement is claimed.
- Preserve array order and occurrence IDs. Require starts contiguous from zero,
  final end equal composition duration, and unique clip/asset IDs. No gaps,
  reordering, overlap, flattening, unused assets or missing source references.
  M0 ID syntax is reused for composition/story/track/clip/asset/beat IDs.
- SourceIn defaults to 0; sourceOut defaults to sourceIn+timelineDuration. Speed
  is 1. Require sourceOut-sourceIn = timelineDuration exactly. Video duration
  must be positive, trim within it. Images require sourceIn=0; sourceOut is a
  presentation hold. Mapped image asset duration is the maximum declared hold
  for that asset, explicitly a synthetic hold window, never recording duration.
  Any supplied positive image duration is retained in the snapshot, not used to
  limit still holds.
- Asset required: `id`, `uri`, `mediaType` (image/video), `name`, `tags` (strings),
  `state` (only favorite/rejected/locked booleans), `metadata` (opaque JSON object),
  `contentHash` (M0 64 lowercase hex; intentionally narrower than source).
  Optional `duration` only. Other informational fields reject rather than
  disappear. URI is opaque, nonempty, never dereferenced. Metadata is retained
  as declarations, subject to generic limits. Known `openfilm.ffprobe.streams`
  audio declarations reject; malformed probe/stream shape also rejects.
- Accepted videos always carry `audio.not_mapped` with disposition
  `blocks-host-handoff`: no audio track does not establish silence. The result
  is a visual metadata mapping, not an audiovisual handoff. Every result has
  `media.not_verified` with disposition `not-verified`; image holds have
  `image.hold_window` with disposition `intentional-delta`. These are bounded,
  structured limitations. Populated/empty audio tracks alike reject in v1.
- Settings: positive integer `width`/`height` <=16384, positive `frameRate` <=240;
  optional timebase exactly `version:1`, positive u32 `rateNumerator` and
  `rateDenominator`, reduced, `dropFrame` boolean. Drop frame only at 30000/1001
  or 60000/1001. Require declared frameRate and explicit rate to agree within
  relative 1e-9 using exact cross-products; preserve both independently.

For frame-rate `a/b` and explicit rate `n/d`, require
`abs(a*d - n*b) * 1_000_000_000 <= n*b`; equality is accepted. All products
use checked u128; overflow rejects. Neither stored clock is replaced by the other.

All times are seconds and become reduced, nonnegative u64 rationals represented
as decimal strings. Parse JSON number lexemes directly with checked u128
intermediates; never through f64. Number spelling is at most 64 bytes, absolute
exponent <=18; overflow or unrepresentable reduced result rejects. Negative zero
rejects. 29.97 becomes 2997/100, distinct from explicit 30000/1001. This differs
intentionally from upstream legacy NTSC inference. Strict exact continuity also
differs from upstream floating tolerances. Original lexemes remain in snapshot.

## Resource/error contract

Input <=1,048,576 bytes, assets 1–1024, clips 1–4096; one track. Before domain
mapping, recursive preflight rejects depth >32 containers, >65,536 total values
plus object keys, decoded strings >4096 UTF-8 bytes, keys >128 bytes, and duplicate
keys anywhere including opaque metadata. Root envelope version is checked before
recursive preflight/domain traversal; scanning bounded raw JSON is unavoidable.
Serde recursion limit remains enabled. Result serialized JSON <=4,194,304 bytes;
the compact serialized mapped plan must separately fit M0's 1,048,576-byte input
limit before success, otherwise `import.output_too_large`.
Enable only serde_json `raw_value`; no new package/runtime dependency. This
feature preserves number spelling and is covered by the locked dependency review.

Stable errors: `input.too_large`, `input.invalid_json`, `import.unsupported_version`,
`import.invalid_snapshot`, `import.unsupported_feature`, `import.budget_exceeded`,
`import.invalid_time`, `import.invalid_range`, `import.inconsistent_timeline`,
`import.output_too_large`, plus existing `plan.*` validation codes. Shape errors
do not expose parser prose. Unsupported feature errors include a single loss
entry with disposition `unsupported` and no accepted plan.

CLI: `storytelling import-openfilm <FILE|->`. One JSON line stdout:
`{"ok":true,"import":...}` or `{"ok":false,"error":...}`. Exit 0 successful
declared mapping, 1 invalid/unsupported/over-budget, 2 usage, 3 I/O including
stdout failure. Existing validate/help/version keep their behavior, except help
lists the additional command. Only the explicit input JSON file may be opened.

## Acceptance and deferred scope

Fixtures/tests cover still hold, video trim, repeated source IDs, future envelope,
new animation, audio tracks/embedded audio, unknown effect, duplicate keys, deeply
nested/oversize metadata, exact decimals and fractional rates, explicit timebase,
source bounds, gaps, locks, declared hashes and file/stdin immutability.
Run a pinned TS export oracle on independently authored synthetic declarations;
classify equality and intentional deltas. Production smoke runs with no
Node/Python/FFmpeg in PATH. Core/engine retain no filesystem/runtime dependencies.

OF-03 is only addressed for immutable input/order/number identity; canonical
cross-language digest and Unicode normalization remain deferred, with no digest
claim. Broader metadata fields, audio, effects, optional tracks, OTIO (M1-02),
actual source bytes and host proof (M1-03), planning and journal remain future work.

## Concrete public types and deterministic reports

`ImportProfileV1` is a unit struct selecting this single profile, not a deserializable
permission or caller-customizable budget. `ImportError` has `code:String`,
`path:String`, `losses:Vec<ImportLoss>`. `ImportLoss` has `code:String`, `path:String`,
`disposition:LossDisposition`; enum wire values are `not-verified`,
`blocks-host-handoff`, `intentional-delta`, `unsupported`.

`ImportedPlanV1` (engine) is Serialize and has public fields:
`profile:&'static str` = `openfilm-finite-v1`, `scope:&'static str` =
`declared-visual-mapping-only`, `executable:bool` = false,
`plan:PlanDocumentV1`, `clocks:ClockFacts`, `losses:Vec<ImportLoss>`,
`source_snapshot:Box<serde_json::value::RawValue>`.
`ClockFacts` (core) has `declared_frame_rate:String` (original number lexeme),
`exact_frame_rate:TimeValue` (rational rate, frames/second),
`explicit_timebase:Option<DeclaredTimebaseV1>`. Timebase fields use original
camelCase wire names `version:u32`, `rateNumerator:u32`, `rateDenominator:u32`,
`dropFrame:bool`. Original clip clocks/IDs/locks and all accepted asset facts remain
in `source_snapshot`; plan shot IDs equal clip IDs, plan asset IDs equal source IDs.

Reports contain `media.not_verified` at `$.assets` first, then exactly one
`image.hold_window` (intentional-delta) or `audio.not_mapped` (blocks-host-handoff)
per asset, in source asset order. Each asset's report path is `$.assets[index]`.
There is no per-occurrence duplication. Unsupported errors carry exactly one
`import.unsupported_feature` loss at the error path; other errors have no losses.
Generic unknown typed fields/shape errors use `import.invalid_snapshot`; known
creative fields (including `effects`) use `import.unsupported_feature`. Generic
preflight failures use path `$`; domain failures use fixed typed paths/indices.
Non-null non-string contentHash is a DTO shape error (`import.invalid_snapshot`);
missing/null or invalid string hashes use `plan.invalid_hash`.
Duplicate JSON keys use `input.invalid_json`; trim failures use
`import.invalid_range`; missing/invalid hashes use existing `plan.invalid_hash`.

When `metadata["openfilm.ffprobe"]` exists it must be an object with a `streams`
array. Each stream must be an object with string `codec_type`; other fields remain
opaque metadata. Any `codec_type:"audio"` rejects as unsupported. Malformed probe
shape uses `import.invalid_snapshot`. Absent/empty probes never prove media silence.
