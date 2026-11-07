# M2-01 bounded StoryBrief audit v1

G2 frozen after independent review. This is a new source-informed brief schema, not an OpenFilm project
import or a complete solver. User requested automatic continuation; routine design
choices are settled here and independently reviewed before implementation.

## Input and authority

`audit_story_brief(bytes: &[u8]) -> Result<BriefAuditV1, ValidationError>` is pure.
No filesystem, media, provider, host, clock, database, random or network access.
No planner selection, playable plan, render, approval or feasibility proof is made.

Strict root fields, all required:

```json
{
  "schema_version": 1,
  "brief_id": "synthetic-brief",
  "intent": "A short synthetic story",
  "assets": [{"id":"a"},{"id":"b"}],
  "manual_selected_asset_ids": ["b"],
  "hard_constraints": [{"type":"must-include","asset_ids":["a"]}],
  "preferences": {"preferred_asset_ids":["a"]}
}
```

Asset objects have only required `id`. Each hard constraint has exactly `type`
and `asset_ids`; types are `must-include`, `must-exclude`, `asset-order`. Other
constraint families are unsupported in this profile, including chronology,
coverage, durations, source capacity, pins, cross-beat ownership and media balance.
Preferences has required `preferred_asset_ids` and optional `target_duration`
using the existing canonical nonnegative u64 rational-seconds object wire, e.g.
`{"numerator":"4","denominator":"1"}`. `BriefPreferencesV1` has public
`preferred_asset_ids:Vec<String>` and `target_duration:Option<TimeValue>`; an
absent target remains omitted in serialized report JSON. A
present target must be positive; absence is allowed, explicit null is refused.
The target is a preference and is never a verified achievable duration.

Identifiers use existing M0 ASCII syntax (1–128 bytes). Intent is arbitrary UTF-8
text up to 4096 bytes, retained verbatim and never interpreted as instructions.
Unknown/duplicate JSON fields and unknown constraint tags reject. Repeated
constraint types in separate declarations are allowed. Assets and every ID list
are unique within that collection. Empty manual/preferred/hard-constraint lists
are allowed; a must-include/exclude list has >=1 ID, asset-order >=2 IDs.

## Hard semantics

- Manual selections are hard required membership **and order**. Never demote or
  discard them to satisfy another input. Each must-include and asset-order member
  is also required. Candidate catalog membership does not erase a manual choice:
  missing references are conflicts, not filtering.
- Excluded IDs are the conjunction of all must-exclude lists. Missing excluded
  IDs are also conflicts. Every required/excluded overlap is a conflict, including
  membership introduced by manual order or an explicit asset-order.
- Adjacent pairs from manual selection and every asset-order list form directed
  precedence edges. Preserve declaration order. Duplicate identical edges may
  normalize into one edge only while retaining every origin path in order.
  Find one deterministic actual cycle witness using iterative DFS, never recursion.
  A cycle conflict lists the distinct witness IDs in traversal order (the final
  edge closes back to the first); its path is the closing edge's first origin.
  DFS roots are first endpoint appearance in normalized edge declaration order
  (before then after); outgoing neighbors are first edge appearance. Include
  unknown endpoints rather than filtering them from the audit graph.
- Required/excluded bindings retain first appearance order and all origin paths:
  manual IDs first, then hard constraints in source order. Do not sort selections
  or semantic arrays to hide differences. Unknown references remain in normalized
  facts with conflicts; no accepted plan is returned at all by this API.

Preferences never create hard requirements or edges. Unknown preferred IDs produce
warnings; preferred IDs explicitly excluded by hard constraints produce warnings.
Both remain in original preferences. These warnings do not relax any hard rule.

## Concrete report

Core `brief.rs` owns strict input DTOs and Serialize report types. Engine `brief.rs`
owns bounded audit logic. CLI depends only on engine's public function.

```text
BriefAuditV1:
 schema_version:u32 = 1
 brief_id:String
 intent:String
 scope:&'static str = "declared-constraint-audit-only"
 executable:bool = false
 selection_planned:bool = false
 duration_analysis:&'static str = "not-performed"
 consistent:bool = conflicts.is_empty()
 manual_selected_asset_ids:Vec<String>  # unchanged input order
 required_assets:Vec<AssetBindingV1>
 excluded_assets:Vec<AssetBindingV1>
 precedence_edges:Vec<PrecedenceEdgeV1>
 preferences:BriefPreferencesV1       # unchanged input values/order
 conflicts:Vec<BriefConflictV1>
 warnings:Vec<BriefWarningV1>

AssetBindingV1: asset_id:String, origins:Vec<String>
PrecedenceEdgeV1: before:String, after:String, origins:Vec<String>
BriefConflictV1: kind:BriefConflictKindV1, path:String, asset_ids:Vec<String>
BriefWarningV1: kind:BriefWarningKindV1, path:String, asset_ids:Vec<String>
```

Conflict kinds: `unknown-asset`, `required-excluded`, `order-cycle`.
Warning kinds: `unknown-preferred-asset`, `excluded-preference`.
Kinds serialize as these strings. Unknown/overlap/preference diagnostics have
singleton asset_ids; only order-cycle carries multiple witness IDs.

Each reference origin points to its exact known array slot, e.g.
`$.manual_selected_asset_ids[0]` or `$.hard_constraints[2].asset_ids[1]`.
An edge origin points to the second/after item. Required/excluded overlap path is
the required binding's first origin; unknown-asset conflicts are emitted once per
reference declaration, including duplicate declarations across distinct lists.
Diagnostic order: unknown hard references (manual then hard-constraint order),
required/excluded overlaps (required order), first cycle witness; preference
warnings in preference order. An unknown preferred ID gets only unknown warning.
An excluded known preferred ID gets only excluded-preference warning.

Consistent means only that implemented reference/membership/order checks found no
hard contradiction. It does not mean selected sources are usable, that all timing
or preferences can be satisfied, or that a composition exists.

## Budgets and errors

Input <=1,048,576 bytes; assets 1–1024; manual/preferred lists <=1024; hard
constraints <=256; each constraint list <=1024; aggregate reference entries
(manual + preferred + all constraint lists) <=8192. At most 256 diagnostics total
(conflicts + warnings); exceedance returns an error, not a truncated clean report.
Output compact report JSON <=4,194,304 bytes. Serde recursion guard stays enabled.
Schema version is probed before strict DTO/domain traversal. No arbitrary metadata.

Errors use existing ValidationError `{code:String,path:String}`. Stable codes:
`input.too_large`, `input.invalid_json`, `brief.unsupported_version`,
`brief.invalid_id`, `brief.invalid_time`, `brief.empty_collection`,
`brief.duplicate_id`, `brief.budget_exceeded`, `brief.too_many_diagnostics`,
`brief.output_too_large`. Paths contain only known field names/indices, never
intent text or parser prose. Input mutation is forbidden.

Representative mappings: too-short/empty constraint list uses
`brief.empty_collection` at `$.hard_constraints[index].asset_ids`; oversized
intent uses `brief.budget_exceeded` at `$.intent`; oversized collections use that
code at their collection path. Aggregate-reference overflow and diagnostic
overflow use `$` with `brief.budget_exceeded` / `brief.too_many_diagnostics`.
Invalid target rational values use `brief.invalid_time` at
`$.preferences.target_duration`; non-string rational fields/null target are
strict DTO shape failures (`input.invalid_json`, `$`). Output overflow uses `$`.

## CLI and tests

`storytelling audit-brief <FILE|->` opens only explicit JSON input and emits one
JSON line. Consistent audit: `{ok:true,audit:...}` exit0. Hard conflicts:
`{ok:false,audit:...}` exit1, with typed conflicts and preserved declarations.
Malformed/over-budget input: `{ok:false,error:...}` exit1. Usage/I/O keep exits2/3.
No logs mixed into stdout; existing commands keep their contracts.

Tests must observe functional RED then GREEN for manual hard membership/order,
include/exclude and missing references, exact cycle witness, soft preferences,
strict version/shape/time/ID/collection bounds, diagnostic budgets, deterministic
output and immutable file/stdin behavior. Actual pinned source oracle checks only
the selected membership/order/conflict policy; the new schema and preferences
are intentional downstream design. Full planner M2-02 and shadow M2-03 remain
separate tasks; no human/host/verified-source gate is closed here.
