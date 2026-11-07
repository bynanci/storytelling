# Fixed-window planner design

User intent: continue the prepared task sequence and improve related documents.
Base c03d4c8; accepted M2-01 supplies strict hard membership and order audits.

Options considered: port the complete multi-beat solver (large unresolved coverage,
score/lock/graph dependencies); return another ordering-only audit (does not deliver
shots); or plan declared fixed windows with hard budgets and deterministic optional
ranking. Choose the third as an engineering decision under automatic continuation.
It yields a real M0 plan while keeping full solver/host proof claims separate.

The [frozen profile](../../migration/FIXED_WINDOW_PLANNER_V1.md) specifies exact
input/output/error/clock and resource semantics. Core/engine remain pure; CLI owns
explicit input/stdout only. No new dependency/crate. Existing schemas remain intact.

Independent G2 builder review precedes implementation. Requirements M2-P01–P06
will map hard choices, ranking, exact clocks, strict budgets, CLI and selected source
comparison to meaningful RED/GREEN tests. Original MIT remains unchanged;
source-informed component files/tests/oracle carry Apache inventory/notices.
