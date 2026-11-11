# M3-01/P2: selected planner proposal to read-only OTIO review

Base: `a7907d071894959487a0ae6df6948e09fef70596`. Scope requested 2026-11-11:
connect an actual `plan-story` stdout proposal to reproducible picture OTIO and
an immutable in-process review binding. This bounded technical profile merged through PR #44 under the user's completed-and-verified merge authority; it does not accept the separate PR #28
security candidate or claim independent human review.

Inputs are the complete successful planner CLI JSON, an admitted finite-v1
OpenFilm composition snapshot and the existing strict declared review context.
The snapshot supplies opaque media URIs, media kinds, available ranges and source
facts; the selected proposal supplies the actual shot order, IDs and exact cuts.
Match every selected asset ID, declared hash and available duration to that same
snapshot. Still windows must start at zero and remain within the snapshot's
synthetic hold bound. Video ranges may differ from the snapshot timeline.
All snapshot features are checked, including unselected material: unsupported
audio/effects reject. Video requires the explicit `picture_only` loss choice;
locks require `metadata_only_locks`. Choices never enable audio or effects.
The complete planner success envelope is strict, including nested audit/selection
DTOs: unknown/duplicate fields reject. Check false authority flags, profile/schema,
plan bounds, selection IDs/order/durations/placement and total duration. Informative
brief audit/ranking annotations are retained and bound, not proof of their origin
or a rerun of the original hard-constraint audit (its request is not supplied).
The authoritative compilation input is `planning.plan`. Changing any proposal
byte, even an annotation or whitespace, changes the artifact identity.

Additive CLI contracts:

```text
prepare-story-review <PROPOSAL|-> <SNAPSHOT_FILE> <CONTEXT_FILE>
export-story-otio <PROPOSAL|-> <SNAPSHOT_FILE> <CONTEXT_FILE>
check-story-review <REVIEW_BUNDLE> <CURRENT_PROPOSAL|-> <CURRENT_SNAPSHOT_FILE> <CURRENT_CONTEXT_FILE>
```

Prepare emits `{ok:true,review:ReviewRecordV1,otio:Timeline}` in one compact line.
Export emits exactly the same native Timeline on stdout, suitable for `.otio`.
Check consumes the entire prepare bundle, validates the displayed OTIO bytes
against its record, then recompiles current inputs and compares all identities.
Exit 0 means current, 1 means stale or rejected, 2 usage, 3 I/O. Only the proposal
may use stdin. No implicit paths, URI reads, Catalog/editor writes or host calls.

The Rust `export_story_otio` API uses the same bounded compile-and-bind path and
returns the OTIO bytes directly. It does not serialize a prepare bundle. The
freshness checker also recompiles directly into the bound artifact and skips
portable bundle serialization; only `prepare_story_review` builds that bundle.
These paths preserve the same artifact bytes and identity.

The new compiler profile is `storytelling.planner-otio-visual-ticks.v1`. OTIO owner
metadata retains the exact selected proposal text and its SHA-256 along with the
existing original snapshot text and explicit native/metadata-only/unsupported
feature report. The record binds snapshot bytes, selected plan, actual compact
OTIO bytes, compiler/profile, loss report and all declared context fields. The
opaque result owns artifact bytes; portable JSON is untrusted and can be forged.
No signature, approval grant, trusted source origin, media bytes, host capability
or replay prevention is established. All authority/evidence flags remain false.
Freshness observes only newly supplied declarations; a file changed outside the
supplied snapshot cannot be detected by this read-only contract.

## Bounds, errors and compatibility

Proposal: <=4,194,336 bytes (the existing 4 MiB planner report plus its CLI envelope).
Snapshot: <=1,048,576. Context: <=65,536. Native OTIO: <=4,194,304.
Prepare bundle: <=4,292,672 (`OTIO + 98,304 record allowance + 64`). Context,
proposal and bundle depth <=64; snapshot retains the finite importer depth/budgets.
Selected plan retains M0 bounds and <=1,024 shots. The renderer measures escaped
root metadata plus each clip before retaining expanded output. No partial artifact
is returned for a rejection. Raw OTIO JSON inside the bundle is byte-bound: do not
pretty-print/re-encode that member and expect the old artifact hash to match.
Outer JSON whitespace alone is tolerated within the bundle byte bound.

Prepare error order: all byte lengths -> strict context -> proposal depth/JSON/
claims/plan/selection/clock -> strict finite snapshot -> matched asset facts and
still bounds -> explicit loss choices -> render/budget -> capabilities/binding.
Check first preflights every size, then validates bundle depth/JSON, record
claims/digest/profile and displayed artifact hash, before the prepare sequence.
Inherited errors keep their stable codes and structured unsupported loss rows.
Bridge codes: `story_review.proposal_too_large`, `story_review.bundle_too_large`,
`story_review.invalid_proposal_json`, `story_review.invalid_proposal_claims`,
`story_review.selection_mismatch`, `story_review.asset_mismatch`,
`story_review.still_offset_unsupported`, `story_review.invalid_bundle_json`,
`story_review.wrong_profile`, `story_review.artifact_digest_mismatch`.
Diagnostics name schema fields and indices, never caller paths, URIs or values.

Existing planner/brief JSON output is unchanged. Their Rust report string fields
now own `String` values to support strict deserialization of saved CLI output;
this pre-release Rust API change is documented rather than a stable SDK promise.

Existing composition-only commands and their V1 digest vector remain unchanged.
Original MIT, Apache components/notices and locked dependencies are retained.

Next dependency: this bridge -> M3-02 durable owner-fenced operation journal ->
M3-03 isolated FilmCraft execution/readback -> M4 OpenFilm feature flag. If a host
apply times out, the outcome is **unknown**: persist that state and read back the
project before any retry. If readback cannot prove whether the exact operation
applied, stop for reconciliation; never blindly resend or claim rollback.
