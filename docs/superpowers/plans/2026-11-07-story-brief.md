# StoryBrief audit implementation plan

Goal: strict declared brief audit with preserved manual hard choices and typed conflicts.
Architecture: core DTOs/report → pure engine audit → local read-only CLI.
Tech: Rust1.85, current serde/serde_json; no new production dependency.
Spec: [design](../specs/2026-11-07-story-brief-design.md),
[wire/semantics](../../migration/STORY_BRIEF_V1.md).

Review focus: missing selections cannot vanish; order constraints imply required
membership; soft preferences never relax hard exclusions; cycle witnesses must
be actual cycles; budgets fail explicitly rather than truncate diagnostics.

## P1 contract/source

- [x] Record actual source SHA/blobs/symbols/tests and independent synthetic fixture rights.
- [x] Independently review/freeze precise profile before implementation.
- [x] Execute selected policy cases against pinned actual source, classify limits/deltas.

## P2 pure audit

Create core/engine `brief.rs` modules and engine/tests/brief.rs; wire real exports.
Produces `audit_story_brief(bytes:&[u8])->Result<BriefAuditV1,ValidationError>`.

- [x] Write compilable stub + meaningful tests; observe functional RED.
- [x] Implement strict bounds, origin-preserving hard sets, iterative cycle witness and soft warnings.
- [x] Run selected tests and workspace debug/release; record GREEN.

## P3 CLI/delivery

Add `audit-brief FILE|->` and real process tests; preserve existing commands.

- [x] Observe CLI RED, implement only composition-root I/O, then GREEN.
- [x] Add M2-Bxx requirement/test mapping, source comparison, per-file provenance and work index.
- [x] Incorporate fresh main/M1-02 aggregate guard; run all required quality/docs/repo/source checks.
- [ ] Independent whole-branch review, actual same-head CI, one task PR.
- [ ] Wait for parent's M1-02-landed signal before integration/merge; mark only completed task scope.
