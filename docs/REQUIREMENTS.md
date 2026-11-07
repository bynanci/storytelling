# Requirements and verification traceability

M0 and the M1 importer subset below are implemented by this candidate. Host and
higher milestone requirements remain proposals. Tests are checked by the repository guard.

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

## M1-01 finite importer requirements

Importer IDs use M1-Ixx to preserve the existing M1-Rxx host-handoff requirements.
See the frozen [contract](migration/FINITE_V1.md),
[source/test/evidence record](migration/records/M1-01.md) and
[machine work index](migration/m1-01-work-index.json).

| ID | Requirement / OF-GAP scope | Implementation | Acceptance test |
| --- | --- | --- | --- |
| M1-I01 | Preserve source and occurrence identity, order and user facts / OF-01/04 | engine mapper + retained source snapshot | `repeat_source_preserves_occurrence_order_and_uses_maximum_still_hold`, `locks_selection_story_and_beats_remain_original_declarations` |
| M1-I02 | Exact decimal/source clocks, independent explicit timebase / OF-02 + GAP-03 | decimal.rs + clocks | `exact_decimal_and_exponent_continuity_does_not_infer_ntsc`, `seconds_above_f64_integer_precision_remain_exact_and_u64_max_is_allowed`, `explicit_timebase_is_separate_from_its_agreeing_decimal_declaration` |
| M1-I03 | Strict immutable bounded input/output; OF-03 identity subset, GAP-10 | DTO + budget + mapped-plan cap | `rejects_oversized_input_and_bounded_metadata_depth_strings_keys_and_values`, `compact_source_whose_mapped_plan_exceeds_m0_input_limit_rejects`, `optional_fields_reject_explicit_null_instead_of_erasing_declarations` |
| M1-I04 | Unsupported features explicit, no executable/verified claim / OF-04 | structured errors/losses | `unknown_effect_and_current_animation_reject_with_one_explicit_loss`, `embedded_audio_rejects_and_missing_or_empty_probe_still_blocks_audio_handoff` |
| M1-I05 | Original snapshot and number lexemes unchanged; declarations only / GAP-03/10 | raw snapshot + pure engine | `source_snapshot_preserves_large_numbers_lexemes_and_input_bytes`, `source_snapshot_minification_preserves_escaped_strings_whitespace_and_unicode` |
| M1-I06 | Usable file/stdin CLI, JSON/exit consistency / OF-04 + GAP-16 | CLI composition root | `imports_file_without_modifying_input_or_directory_and_returns_an_m0_plan`, `imports_stdin_with_the_same_result_as_file_input`, `treats_missing_video_media_and_instruction_text_as_opaque_declarations` |

GAP-01 is checked by `provenance_manifest_covers_every_imported_file` in the
repository guard. GAP-02/16 use the pinned six-case source oracle and differential
comparator under `tests/oracles/openfilm-finite-v1`; same-head CI replays frozen
oracle outputs. Synthetic validation does not close real-media/host gates.

## M1-02 finite OTIO requirements

These are format-only requirements. They do not close M1-R01–R03 host/source
gates. See [contract](migration/FINITE_OTIO_V1.md),
[comparison](migration/records/M1-02.md) and [work index](migration/m1-02-work-index.json).

| ID | Requirement / OF-GAP scope | Actual acceptance |
| --- | --- | --- |
| M1-O01 | Native reference/cut/still identity and range mapping / OF-35 subset | `still_emits_native_timeline_track_clip_and_synthetic_hold_reference`, `repeated_source_keeps_distinct_occurrences_order_and_maximum_hold_window`, `video_trim_preserves_exact_source_clock_and_declared_available_duration` |
| M1-O02 | Common exact content clock, safe native values/endpoints/whole duration / OF-02/35 | `fractional_cuts_share_integer_content_clock_without_project_fps_quantization`, `common_denominator_lcm_rejects_even_when_each_input_denominator_is_safe`, `accumulated_timeline_duration_checks_the_safe_integer_boundary_without_rounding`, `exact_safe_integer_boundary_is_accepted_and_larger_native_windows_reject` |
| M1-O03 | Explicit audio/lock choices, honest format capability/loss / OF-36 + GAP-05 subset | `audio_and_lock_losses_are_ordered_and_each_choice_only_accepts_its_own_loss`, `report_retains_import_limitations_and_never_claims_source_host_or_execution_verification`, `unsupported_source_features_keep_import_codes_paths_and_unaccepted_rows` |
| M1-O04 | Original snapshot lexemes and bounded amplified output / GAP-03/10 | `owner_snapshot_is_a_string_preserving_large_integer_decimal_and_opaque_exponent_lexemes`, `bounded_source_with_amplified_repeated_names_rejects_the_final_otio_budget` |
| M1-O05 | Usable direct OTIO stdout, strict loss flags, immutable file/stdin / GAP-14 subset | `exports_a_real_timeline_identically_from_file_and_stdin_without_writing`, `rejects_missing_extra_unknown_duplicate_and_misplaced_arguments_with_usage_two`, `preserves_numeric_lexemes_inside_the_source_snapshot_json_string` |
| M1-O06 | Actual source and official parser semantic conformance / GAP-02/16 | `tests/oracles/otio-finite-v1/verify.py`: six pinned TS cases plus two direct Rust fixtures, official 0.18.1 parse/write/read; no media URI dereference |

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
claims, not interchangeable scores. M0/M1-01 implement only the first two levels.
