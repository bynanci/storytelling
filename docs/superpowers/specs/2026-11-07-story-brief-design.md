# StoryBrief audit design

Provide one useful M2-01 slice: inspect a bounded declared brief before a planner
tries to construct a story. Preserve manual selections and explain hard reference,
inclusion/exclusion and ordering contradictions with typed diagnostics.

Options considered: port the complete upstream solver now (unbounded scope), add
an untyped planning stub (no useful consumer), or implement a strict constraint
audit with explicit limits and no planner/feasibility claim. The implementer
selects the audit under the user's automatic-continuation instruction.

The exact [contract](../../migration/STORY_BRIEF_V1.md) separates hard constraints
from preferences, retains origin paths, treats selected order as hard, and never
filters missing/manual references away. Source is pinned OpenFilm
7add4762fe8174d76f43b863375ea39b73121e76. The new wire and typed conflicts are
source-informed original design; they are not an existing upstream StoryBrief API.

Keep current three crates and existing serde dependencies. Core DTO/report types,
pure engine audit and CLI composition-root I/O form one usable vertical slice.
Source-informed files use the approved Apache component policy; MIT bytes stay
unchanged. M1-02 proceeds independently; reconcile its main/guard changes before
publishing this branch. Do not merge until parent confirms M1-02 landed and the
integrated branch passes same-head checks.
