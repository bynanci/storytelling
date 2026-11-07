# Requirements and verification traceability

M0 requirements are implemented by this candidate. Higher milestones remain
proposals. The test names below are checked against source by the repository guard.

| ID | Requirement | Implementation owner | Acceptance test |
| --- | --- | --- | --- |
| M0-R01 | Accept valid declared metadata and reused source occurrences | engine/lib.rs | `accepts_declared_plan_and_reused_source` |
| M0-R02 | Refuse unsupported version, unknown and duplicate fields | core/wire.rs + engine | `rejects_unsupported_version`, `rejects_unknown_and_duplicate_json_fields` |
| M0-R03 | Validate IDs, declared hash shape, uniqueness and source references | engine | `rejects_duplicate_identities`, `rejects_missing_source_reference`, `rejects_invalid_identity_and_hash` |
| M0-R04 | Canonical bounded exact rational source time, no float rounding | core/time.rs | `rejects_noncanonical_and_zero_denominator_time`, `compares_large_rationals_without_float_rounding` |
| M0-R05 | Nonempty source ranges within positive declared duration | engine | `rejects_empty_or_reversed_ranges`, `rejects_source_range_outside_declared_duration`, `rejects_zero_duration_and_preserves_input_bytes` |
| M0-R06 | Bound input and collections | CLI + engine | `rejects_malformed_or_oversized_input`, `rejects_empty_and_over_budget_collections` |
| M0-R07 | Deterministic read-only behavior; do not claim media verification | engine + CLI | `accepts_exact_rational_intervals_and_is_deterministic`, `accepts_valid_file_with_machine_readable_output` |
| M0-R08 | Machine JSON and distinct validation/usage/I/O outcomes | CLI | `rejects_bad_source_range_with_exit_one`, `accepts_stdin`, `distinguishes_usage_and_io_failures`, `reports_help_and_version_without_loading_a_project` |

All Rust tests are under `crates/*/tests`. The source test fixtures are synthetic,
with placeholder hashes; success is strictly declared-metadata validation.

## Next requirements: not yet implemented

M1-R01: a finite host profile preserves supported shot order/source ranges through
real FilmCraft import/save/reopen. M1-R02: every unsupported feature or quantization
is reported before approval. M1-R03: host and source identities are verified by the
adapter, with path-free portable provenance.

M2-R01: hard constraints are satisfied or return explicit conflict, never silently
relaxed. M3-R01: an Agent cannot authorize itself. M3-R02: uncertain external effects
are reconciled before retry. M4-R01: external consumers need not depend on Vue/Tauri.
M5-R01: supported host/platform claims have actual target evidence.

## Evidence levels

Structural parsing < declared semantic validation < actual source verification <
host readback/render verification < human editorial acceptance. These are distinct
claims, not interchangeable scores. M0 implements only the first two levels.
