# M2-02 finite fixed-window planner v1

G2 frozen after independent contract and clock review. This downstream profile implements one
usable deterministic planner, not the full OpenFilm multi-beat solver. The user
requested automatic continuation; fixed-window selection and the following
routine design choices are engineering decisions, not attributed user preferences.

## Input and public API

`plan_story(bytes: &[u8]) -> Result<PlannedStoryV1, PlanningErrorV1>` is pure.
Input JSON is at most1,048,576 bytes. Exactly these required root fields:

```json
{
  "schema_version":1,
  "brief":{"schema_version":1,"brief_id":"demo","intent":"Demo",
    "assets":[{"id":"a"}],"manual_selected_asset_ids":["a"],
    "hard_constraints":[],"preferences":{"preferred_asset_ids":[]}},
  "candidates":[{"asset_id":"a","content_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "duration":{"numerator":"4","denominator":"1"},
    "source_range":{"start":{"numerator":"0","denominator":"1"},
      "end":{"numerator":"4","denominator":"1"}},"score":10}],
  "max_duration":{"numerator":"8","denominator":"1"},"max_shots":4
}
```

All objects reject duplicate/unknown/missing required fields and explicit null.
The existing brief target_duration remains optional; its absence is allowed. Candidate
score is a signed JSON i32 integer; no floating scores or semantic/model claims.
The brief is the existing strict M2-01 schema and is audited by its actual API.
Each candidate ID must occur in its catalog; candidates are unique, at most1024,
and may omit optional catalog entries. Every required asset must have a candidate.
Candidate IDs/hashes/time values follow M0 syntax. Duration is positive and the
nonempty half-open source range lies in[0,duration]. Zero duration or start>=end
returns plan.invalid_range; end>duration returns plan.source_out_of_bounds. No media URI is opened.
`max_duration` is a positive exact hard bound (zero: plan.invalid_range at
$.max_duration; malformed canonical values: plan.invalid_time); `max_shots` is an integer1..1024.
The existing brief target is a soft preference, even when above the hard maximum.
Zero candidates is syntactically allowed but cannot produce a nonempty plan.

## Selection, order and timing

1. Probe planner schema before strict envelope traversal; then call the existing
   brief auditor before planner domain validation. Invalid brief errors have
   outer `$.brief` paths. Any hard brief conflict rejects with no plan.
2. Validate hard bounds and all candidate declarations, including excluded or
   unused ones. Check required candidate availability in required first-appearance
   order, then required count against max_shots. Never discard required choices.
3. Compute one exact integer planning clock as the LCM of every candidate duration,
   start/end denominator, max_duration denominator and optional target denominator.
   LCM and each scaled value must fit u64. Overflow is explicitly unrepresentable,
   never rounding or an infeasibility claim. This conservative profile may reject
   an unused candidate whose declared clock exceeds the bound.
4. Admit all required candidates first. Their fixed windows must fit the hard
   duration bound; otherwise return a typed failure. Windows are never shortened.
5. Optional candidates rank by preferred-list position first (nonpreferred last),
   then descending declared score, then ascending ASCII asset ID. These rankings
   are explicit downstream policy; they do not port OpenFilm's score factors,
   capture-time tie breaks or locale-dependent comparison.
6. Scan ranked eligible candidates: stop admitting when a present soft target is
   met; otherwise skip a candidate that cannot fit remaining duration or shot
   budget and continue to later candidates. With no target, fill the hard budgets.
   Excluded assets never enter selection. Required assets can exceed the soft
   target. A shortfall is a successful fixed-window proposal, not a global solver
   failure. Empty selection returns planner.no_selection, an intentional downstream
   safety difference from upstream empty compositions.
7. Stable Kahn ordering respects every selected precedence edge from the brief.
   Among ready vertices use admission order (required first-appearance, then
   optional rank). All edge endpoints are required by M2-01 semantics. No cycles
   can be repaired by dropping an asset. Array order is never sorted to hide a
   semantic difference.
8. Produce an M0 PlanDocumentV1 with plan_id=brief_id, selected assets in original
   candidate catalog order, and ordered shot IDs shot-000001 onward. Source ranges
   and declared hashes are unchanged. Placements start at zero and accumulate
   exact fixed window ticks. Rational outputs are reduced canonical u64 strings.
   Validate the actual nested plan using the existing M0 validator and its1MiB
   limit; the complete compact report has a4MiB limit.

## Concrete output and diagnostics

Core planner.rs owns Serialize output/error types and the strict candidate DTO.
Engine planner.rs owns envelope admission/selection/order; planner/clock.rs owns
bounded integer planning time. No new crate or production dependency is added.

`PlannedStoryV1` has schema_version1, profile storytelling.fixed-window-planner.v1,
scope declared-fixed-window-plan-only, executable/source_verified/host_verified
false, selection_planned true, duration_analysis exact-fixed-windows,
content_clock_rate:String, total_duration:TimeValue, target_status
(not-requested/met/below/above), the unchanged `brief_audit`, `plan`, `selections`
and `skipped_candidates`. Brief-audit diagnostic/origin paths remain brief-local;
all planner errors and selection reason paths address the outer input.

Selections are in shot order. Each `SelectionDecisionV1` has asset_id, shot_id,
required:bool, declared_score:i32, timeline_start:TimeValue, duration:TimeValue,
reason_paths:Vec<String>. Required reasons are every required origin prefixed
with $.brief; optional reasons include its preferred-list slot when present and
candidate score path. Every row ends with its candidate source_range path.
These cite supplied declarations, never independently verified quality/evidence.

Skipped rows are in original candidate order, with asset_id, path to the candidate,
and reason excluded/not-needed/shot-limit/duration-limit, in that priority when
several conditions apply. The reason is the
actual admission decision; selected rows never appear here.

`PlanningErrorV1` is Serialize/Debug with code:String,path:String,
conflicts:Vec<BriefConflictV1> (always present). Brief-conflict paths are prefixed
with $.brief; their order and IDs remain unchanged. Other errors have conflicts[].
Codes: input.too_large/input.invalid_json; planner.unsupported_version,
planner.brief_conflict, planner.invalid_max_shots, planner.too_many_candidates,
planner.duplicate_candidate, planner.unknown_candidate,
planner.required_candidate_missing, planner.required_shots_exceeded,
planner.required_duration_exceeded, planner.unrepresentable_time,
planner.no_selection, planner.output_too_large; existing plan.invalid_id,
plan.invalid_hash, plan.invalid_time, plan.invalid_range and
plan.source_out_of_bounds apply to candidate facts.
Required count/duration errors point to $.max_shots/$.max_duration. Missing required
candidate points to its first required brief origin. Clock failure points to the
specific offending time, or $ for an accumulated LCM failure.

CLI `plan-story FILE|-` reads only explicit JSON and emits one line:
`{ok:true,planning:<report>}` exit0 or `{ok:false,error:<PlanningErrorV1>}` exit1;
usage2, input/output I/O3. Unknown/extra flags and an option-looking input
starting -- reject as usage; literal such filenames use ./ or an absolute path. Help lists all six
commands. Existing commands retain their wire shapes and limits.

## Source comparison and excluded scope

Pinned source7add4762 includes compose required-first membership/order, exclusions,
source bounds, preferred durations and a soft target. Compare selected synthetic
membership/order/duration invariants with actual source, classifying fixed windows,
declared ranking, deterministic IDs/ASCII ties and empty-plan refusal explicitly.
Do not call this M2-03 full planner shadow parity. No multi-beat allocation, coverage,
media-balance, rejected/rated-state inference, locks, echoes, graph/preset plans,
source fingerprinting, optimality, general feasibility, host approval/execution,
rendering or editorial acceptance is established by this planner.

Public core types: PlanningCandidateV1 {asset_id:String, content_hash:String,
duration:TimeValue, source_range:SourceRange, score:i32} derives Deserialize/Serialize
with deny_unknown_fields. TargetStatusV1 and SkippedReasonV1 are Serialize enums
using kebab-case spellings above. SkippedCandidateV1 {asset_id:String,path:String,
reason:SkippedReasonV1}. SelectionDecisionV1 fields are exactly those listed above.
PlannedStoryV1/PlanningErrorV1 derive Debug/Serialize, never trust-bearing
Deserialize. Static profile/scope/analysis strings are &'static str; schema_version
is u32 and the four booleans are bool.
