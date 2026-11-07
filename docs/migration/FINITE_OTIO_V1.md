# M1-02 finite OTIO JSON contract

G2 frozen after independent review and actual source/official oracle exploration. This is a format
profile, not a claim about FilmCraft or another editor. It extends the frozen
[finite import](FINITE_V1.md) without expanding admitted source features.

## Scope and entry points

`export_openfilm_otio(bytes: &[u8], profile: &OtioProfileV1,
choices: &OtioLossChoicesV1) -> Result<CompiledOtioV1, OtioCompileError>`.
The input is the same actual OpenFilm composition JSON export as M1-01; call the
existing strict importer first. Unknown/future/over-budget input and every
unsupported source feature keep their existing errors. No source migration,
filesystem/network/media/host call, baking, approval or verification occurs.

`OtioProfileV1` is a fixed unit struct. `OtioLossChoicesV1` has booleans
`picture_only` and `metadata_only_locks`, both false by Default. These are format
loss choices for this invocation, never trusted human execution approval.

The engine returns `CompiledOtioV1 { document: serde_json::Value,
report: OtioCompileReportV1 }`, Debug + Serialize, with public fields. Core owns
the profile/choices/report/error contracts in `compiled_plan.rs`; engine owns the
pure serializer. The CLI consumes only the engine. No new crate or Cargo package
is needed; existing serde/serde_json and Cargo.lock remain sufficient.

## Exact bounded content clock

OTIO native RationalTime fields are doubles. Use a **common integer ticks-per-second**
clock: LCM of the canonical rational denominators of every M0 mapped asset
duration and every shot source start/end. All native `value` and `rate` fields
are numeric integers in `[0, 9_007_199_254_740_991]`; rate is positive. Require
the LCM, each scaled start/end/duration/available duration and accumulated whole
timeline duration to fit that limit. Use checked u128 arithmetic. Never pass a
time through f64, encode decimal-string integers in OTIO numeric fields, round,
quantize, clamp or change a range to make it fit. Reject unrepresentable input
with no document.

All native ranges use this same rate. Source range duration is endTicks-startTicks;
available ranges start at 0. Timeline duration is the exact sum of admitted cuts.
One common clock prevents native rate-rescaling addition from changing endpoints.
This is an intentional representation delta from upstream project-rate OTIO.
It is **not** project FPS. `global_start_time` is null. Original rate lexeme,
exact rational rate and explicit timebase stay separately authoritative metadata.
Consumers need an arbitrary-content-tick-rate capability; no host is certified.

## Native document and identity mapping

Emit Timeline.1 → Stack.1 → one Track.1 (`kind:"Video"`) → Clip.1 occurrences,
each with singular `media_reference: ExternalReference.1`. Emit TimeRange.1 and
RationalTime.1 numeric fields. All effects/markers are empty arrays; source_range
on Stack/Track is null. No Gap is admitted by the existing contiguous input
profile; basic audio, retiming, effects, captions and graphics remain unsupported.

Timeline name is composition/plan ID; Stack name is `storytelling`; Track name is
the original track ID; Clip and ExternalReference names are original asset names.
Track/clip arrays retain original order; repeated sources remain distinct clips.
Native OTIO has no shared asset array; original asset order remains in snapshot text.
ExternalReference target_url is the exact opaque original URI, never
resolved, canonicalized, encoded again or opened. available_range uses the mapped
matched M0 asset duration: declared video duration or the maximum admitted hold
across repeated still occurrences, explicitly a synthetic still hold window.

Namespace owner metadata as `storytelling`:

- Root: `report: OtioCompileReportV1` and `source_snapshot_json:String`. The latter
  is the importer-retained compact JSON text as a **string**, not an OTIO metadata
  object. Its number lexemes and source facts must survive official read/write.
- Clip: `occurrence_id`, `asset_id`, `source_range` (M0 rational strings).
- ExternalReference: `asset_id`, `declared_content_hash`, `media_type`,
  `available_range_kind` = `synthetic-still-hold` or `declared-video-duration`.

No digest, approval token, actual media identity or runtime parser pass is minted.

## Loss and capability report

All report fields are concrete:

```text
schema_version: u32 = 1
profile: &'static str = "storytelling.otio-visual-ticks.v1"
scope: &'static str = "declared-otio-visual-interchange"
executable: bool = false
source_verified: bool = false
host_verified: bool = false
host_name: Option<String> = None
host_version: Option<String> = None
required_capabilities: Vec<String>
content_clock_rate: String             # canonical integer ticks/second
timeline_duration: TimeValue           # canonical reduced rational seconds
project_clocks: ClockFacts             # unchanged M1-01 clock facts
choices: OtioLossChoicesV1
feature_matrix: Vec<OtioFeatureV1>
import_limitations: Vec<ImportLoss>     # all original importer limitations, unchanged
```

Required capabilities, in order: `single-video-track`, `external-reference`,
`still-hold`, `arbitrary-content-tick-rate`. `OtioFeatureV1` has `feature:String`,
`path:String`, `disposition:FeatureDisposition`, `accepted:bool`. Wire dispositions
are `native`, `baked`, `metadata-only`, `unsupported`; no feature is called baked.

Feature rows have deterministic order:

1. `timeline.cuts` / `$.composition` / native / true.
2. `media.references` / `$.assets` / native / true.
3. `image.holds` / `$.assets` / native / true, iff any images.
4. `clock.content_ticks` / `$.settings` / native / true.
5. `project_rate.metadata_only` / `$.settings` / metadata-only / true; explicitly
   part of this fixed format profile, without a host FPS inference.
6. `source_facts.metadata_only` / `$` / metadata-only / true; original facts remain
   in the owner snapshot string, not claimed as native editor behavior.
7. `audio.picture_only` / `$.assets` / metadata-only, iff any videos; accepted only
   if choices.picture_only. Never infer that a video is silent.
8. `editing.locks_metadata_only` / `$` / metadata-only, iff any admitted track.locked,
   clip.locked or asset.state.locked is true; accepted only if
   choices.metadata_only_locks. Foreign editor lock enforcement is unverified.

Any unaccepted row rejects compilation with `otio.loss_choice_required`, path
`$.choices`, and those rows in `losses`; no partial/accepted document is returned.
Existing importer `audio.not_mapped / blocks-host-handoff` and `media.not_verified`
limitations remain unchanged even when format choices are accepted. Choices only
permit this limited serialization, not audiovisual/host or source verification.

`OtioCompileError` has `code:String`, `path:String`, `losses:Vec<OtioFeatureV1>`.
Import error codes/paths are preserved; any existing unsupported import loss maps
to an unaccepted unsupported feature row with the same code/path. Other parse
errors have empty losses. Diagnostic paths never expose URIs or arbitrary keys.

## Budgets and stable failures

Input/imported plan keep all M1-01 caps. Final native OTIO JSON <=4,194,304 bytes.
Build the root with an empty clip array and measure its compact JSON size, then
measure/add each bounded clip plus its comma before retaining it. Reject before
retaining amplified repeated names/URIs that exceed the result budget. Perform
a final serialized-size check as a guard. No unbounded output or traversal.

Additional codes: `otio.loss_choice_required`, `otio.unrepresentable_time`,
`otio.output_too_large`, `otio.invalid_time` (defensive invalid internal wire).
Unrepresentable clock uses path `$.content_clock`; scaled range/duration failures
use `$.assets[index].duration`, `$.shots[index].source_range`, or
`$.timeline_duration`, with an unaccepted unsupported feature
`clock.unrepresentable`. Output-size failures use `$` and no losses.
Precedence: existing input validation → loss choices → clock projection → output budget.

## CLI

`storytelling export-otio <FILE|-> [--picture-only] [--metadata-only-locks]`.
Flags may occur in either order after input; each at most once. Unknown/duplicate
flags, missing input or other extra args return usage exit 2.
Input arguments beginning `--` are options, not filenames; use an explicit `./`
or absolute path for such a literal filename. Preserve existing
validate/import-openfilm argument behavior. File/stdin reading uses existing bounds.

Success exits 0 with the actual OTIO Timeline JSON document, one stdout line,
so stdout may be redirected directly to `.otio`. The report is embedded in owner
metadata. Failure exits 1 with `{ok:false,error:OtioCompileError}` JSON; usage/I/O
remain exit 2/3 with existing error shape. No interleaved logs. Only the explicit
input JSON is opened; stdout failure exits 3. Help lists the command and flags.

## Acceptance and remaining gates

Meaningful RED→GREEN tests cover still holds, trims/repeated occurrence IDs, URI
and exact source clocks, common-rate addition, safe-integer/LCM/accumulation
boundaries, metadata lexemes, explicit audio/lock choices, unsupported features,
amplified output budgets, file/stdin/exit/immutability and old command behavior.

Official OpenTimelineIO **0.18.1** is test-only. Parse/write/read via `otio_json`
without opening media; compare semantic IDs, order, URIs, exact native value/rate
ratios, placement/duration and owner metadata. Clip.1 legitimately upgrades to
Clip.2 on official write. Do not compare raw schema labels/property order or
sort semantic arrays away. Run actual pinned TS exporter on synthetic inputs and
classify the content-clock representation delta and narrower unsupported profile.

Retain Rust 1.85.0, MIT bytes and source-informed Apache component provenance.
CI-only Python/action/package additions require pinned versions/identities and
notices appropriate to their actual use. Core/engine/CLI production remain native
with no Python/Node/FFmpeg dependency. M1-03 actual bytes and FilmCraft proof are
separate. Official FilmCraft v0.18.0 Linux CLI has now been provisioned and hashed;
the actual bounded host profile/roundtrip remains to be exercised in M1-03.
