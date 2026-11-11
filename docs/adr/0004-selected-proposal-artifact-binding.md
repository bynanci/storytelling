# ADR 0004: compile the selected proposal and review the actual artifact

Date: 2026-11-11. Status: bounded technical delivery accepted via PR #44 under verified-merge authority; no independent or editorial acceptance asserted.
Task: M3-01/P2, requirements M3-PB01–PB06.

The composition-only review profile proves consistency of an imported timeline;
the fixed-window planner emits a different selected plan. Exporting the original
composition after selecting proposal A cannot establish that the artifact follows A.

Admit the full strict planner stdout envelope and the same explicit finite snapshot.
Match selected asset IDs/hashes/available durations, then replace only the imported
timeline plan inside a crate-private renderer. Reuse exact content ticks, explicit
loss choices and incremental output budgets. Retain original snapshot and selected
proposal text with their distinct identities. No decimal composition synthesis or
float roundtrip is introduced. Still offsets reject under the existing hold profile.

Produce one opaque owned result and a portable `{ok,review,otio}` bundle. Verify the
actual member bytes before recompiling current inputs. Separate additive CLI commands
avoid changing the accepted composition-only schema/digest vector. Portable records
are unkeyed, untrusted declarations; all execution/approval/source/host flags stay false.
The original planner request is not supplied, so this verifies selected-plan/artifact
consistency and internal selection timing, not planner authenticity or the full audit.

The next stateful PR must add a durable owner-fenced operation journal before any
host executor. A timeout after apply is unknown, survives restart, and requires
project readback before retry; ambiguous readback blocks. The M4 consumer feature
flag follows executor/readback acceptance and preserves a single writer/authority.
No journal, host executor or feature flag placeholder is added in this PR.

Rollback: continue using existing commands; discard only new synthetic/output files.
No source Catalog or FilmCraft project is migrated or edited.
