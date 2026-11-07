# Finite OTIO design

Goal: make the existing finite composition import usable as an OTIO JSON artifact
while preserving exact content ranges, occurrence identity and explicit limitations.
The existing Storytelling CLI supplies local I/O; pure engine serialization stays
in the current workspace. Full contract: [finite OTIO v1](../../migration/FINITE_OTIO_V1.md).

## Approaches considered

- Project-rate double projection resembles OpenFilm's exporter but requires an
  explicit quantization/error policy and may round source clocks. Defer this
  until an actual host profile needs it.
- Per-value rational clocks retain exact native components, but official OTIO
  rescaling while adding mixed rates can still change endpoints.
- One bounded common integer content clock keeps every native component and
  addition exact. It must be distinguished from project FPS and refused when
  its LCM/value range exceeds the safe-double integer domain. This is selected.

The implementer selected the third option under the user's explicit automatic
continuation instruction. It is recorded as an implementation decision, not a
user-confirmed host capability. Independent G2 review precedes production code.

## Boundaries

Only the already admitted single contiguous visual track is supported. Emit
native ExternalReference/Clip/TimeRange nodes; no audio, gaps, retiming, effects,
renderer, baking or host operation is added. Original snapshot text is a metadata
string because an OTIO AnyDictionary cannot retain every original numeric lexeme.
Video picture-only and metadata-only lock handling require explicit format loss
choices. All artifacts retain executable/source-verified/host-verified false.

The official Python parser and pinned TypeScript exporter are test-only oracles.
Official FilmCraft v0.18.0 Linux CLI is now provisioned and hashed; actual host
profile proof remains M1-03 and parser success will not close it. The user separately
authorized merging completed verified PRs and checking
completed tasks. This does not authorize release or repository-policy changes.

Source remains OpenFilm 7add4762fe8174d76f43b863375ea39b73121e76; main observed
5f737a6c8f35dce48e213370ff27900962c9304c. Base is accepted M1-01 main descendant
440209f21d2b5fd6e7b4cad0823359c802cf8c8e after accepted completion-status PR #11.
Existing MIT is unchanged; new source-informed
component files use the previously approved Apache-2.0/provenance policy.
