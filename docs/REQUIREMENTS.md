# Requirements and verification traceability

M0, finite M1-01/M1-02, bounded M2-01 audit, M2-02 fixed-window planning and
M1-H01–H04's M1-03 source/host proof are accepted on main. Broader host, full-planner and human acceptance remain
separate. Tests are checked by the repository guard.

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

Rust integration tests are under `crates/*/tests`; source-level API compile-fail
doctests also pin the M1-03 private/no-deserialize trust boundary. The source test fixtures are synthetic,
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

## M2-01 bounded brief audit requirements

M2-Bxx identifies this declared-constraint audit subset separately from the
M2-R01 full-planner goal. The bounded M2-B01–B06 scope is accepted through
merged PR #17; no release, host or full-planner status follows from this audit. See the frozen
[contract](migration/STORY_BRIEF_V1.md),
[source comparison record](migration/records/M2-01.md),
[per-file provenance](migration/provenance-m2-01.json) and
[complete requirement/test mapping](migration/m2-01-work-index.json).
[Tracking issue #14](https://github.com/bynanci/storytelling/issues/14) owns this
bounded slice; verification and acceptance completed in merged PR #17; issue #14 is checked and closed.

| ID | Implemented bounded requirement | Implementation | Representative actual tests |
| --- | --- | --- | --- |
| M2-B01 | Manual choices stay hard required membership/order; include/order members are required; keep declaration order and every edge origin | core/brief.rs + engine/brief.rs | `missing_manual_choices_remain_required_and_ordered_instead_of_being_filtered`, `include_and_order_members_promote_required_assets_with_all_ordered_origins`, `duplicate_edges_retain_each_declaration_origin_without_reordering` |
| M2-B02 | Typed missing-reference, required/excluded and actual-cycle conflicts retain stable paths/order and conflicting choices | engine audit + brief/cycle.rs | `unknown_exclusions_and_repeated_reference_declarations_get_typed_conflicts`, `conflicting_fixture_has_stable_hard_diagnostic_order_and_required_origin_paths`, `cycle_witness_is_actual_and_excludes_acyclic_prefix_and_downstream_nodes` |
| M2-B03 | Preferences stay soft; exact positive rational target and Unicode intent remain declarations, with no duration analysis or text execution | core preferences + engine audit | `preference_warnings_preserve_values_and_never_relax_hard_requirements`, `positive_canonical_target_duration_is_preserved_without_feasibility_analysis`, `unicode_and_instruction_shaped_intent_remain_verbatim_inert_text` |
| M2-B04 | Strict supported version/shape/ID boundary; refuse unknown, duplicate, null or unsupported declarations | core DTOs + engine validation | `unknown_fields_and_unsupported_constraint_families_reject_strictly`, `duplicate_json_fields_reject_at_root_and_each_nested_object`, `future_version_is_probed_before_domain_validation_and_required_fields_are_strict` |
| M2-B05 | Bounded input, intent, collections, aggregate references, diagnostics and output; iterative traversal, deterministic results and immutable bytes | engine validation/audit + iterative cycle helper | `aggregate_reference_budget_accepts_8192_and_refuses_8193`, `combined_diagnostic_budget_accepts_256_and_errors_instead_of_truncating_at_257`, `full_depth_acyclic_order_and_large_origin_report_stay_within_output_budget`, `input_bytes_and_semantic_array_order_are_unchanged_and_reports_are_deterministic` |
| M2-B06 | Read-only explicit file/stdin CLI, one-line typed JSON and stable exits; scope is audit only; selected source policies have a finite differential comparator | CLI composition root + policy oracle | `fixture_reports_only_declared_consistency_without_a_planned_selection`, `audits_file_and_stdin_identically_without_changing_the_input_or_directory`, `reports_typed_hard_conflicts_with_exit_one_and_keeps_manual_choices`, `redacts_missing_input_file_with_io_exit_three` |

The machine index maps all 22 engine and 7 CLI tests. Actual pinned source
outcomes and the differential comparator live under
[the ten-case policy oracle](../tests/oracles/story-brief-v1/README.md). Only hard
membership/order/reference/conflict policy is compared. The brief schema,
preferences and typed diagnostics are downstream design. A consistent audit
checks declared rules; it does not prove composition feasibility or choose shots.


## M1-03 bounded source and FilmCraft proof

See [25 fps contract](migration/FILMCRAFT_25_V1.md),
[evidence record](migration/records/M1-03.md) and
[work index](migration/m1-03-work-index.json).

| ID | Requirement / boundary | Actual acceptance |
| --- | --- | --- |
| M1-H01 | Native SHA-256 of ≤16 MiB explicit bytes / GAP-04 and OF-29 subset | `accepts_independent_sha256_vectors_and_owns_verified_bytes`, `rejects_mismatched_and_noncanonical_expected_hashes`, `accepts_exact_buffer_budget_and_rejects_larger_before_hash_syntax` |
| M1-H02 | Explicit file/stdin verification; metadata-only receipts; no project URI access | `accepts_explicit_source_bytes_with_metadata_only_receipt`, `rejects_bad_source_evidence_without_leaking_input`, `distinguishes_source_usage_and_io_errors`; compile-fail private/no-deserialize API examples |
| M1-H03 | Actual pinned host 25 fps import, exact native ranges, default intrinsic effects and external ID mapping / M1-R01 subset | `tests/oracles/filmcraft-25-v1/run.py`, complete checked native baselines, `test_checks.py` semantic rejection tests |
| M1-H04 | Actual CPU H.264 192×108 preview, 45 exact PTS, 20/5/20 colors, save/reopen/OTIO reimport and unchanged inputs | Same harness and committed `tests/oracles/filmcraft-25-v1/evidence.json`; source/hash/host identities are observed evidence, not approval |

## M2-02 fixed-window planning requirements

The bounded M2-P01–P06 scope is accepted via merged PR #22 after all 10 same-head
checks; issue #20 is fully checked and closed completed. See [profile](migration/FIXED_WINDOW_PLANNER_V1.md),
[record](migration/records/M2-02.md), [work index](migration/m2-02-work-index.json)
and [component inventory](migration/provenance-m2-02.json).

| ID | Requirement | Actual verification |
| --- | --- | --- |
| M2-P01 | Required/manual membership and precedence are never relaxed; typed conflicts/missing candidates and hard budgets remain explicit | `emits_an_actual_m0_plan_with_deterministic_ids_and_immutable_source_facts`, `preserves_manual_and_explicit_order_with_global_reason_paths_and_catalog_order`, `rejects_hard_brief_conflicts_without_filtering_unknown_or_excluded_choices` |
| M2-P02 | Optional preference/score/ASCII selection is deterministic, skips actual capacity failures and keeps target soft | `optional_ranking_uses_preference_then_signed_score_then_ascii_id`, `too_long_optional_is_skipped_while_a_later_fitting_asset_is_selected`, `skipped_reasons_prioritize_exclusion_then_met_target_over_other_limits` |
| M2-P03 | Exact common integer clock, immutable windows/placements and bounded u64 arithmetic without rounding | `fractional_windows_preserve_source_offsets_and_exact_accumulated_placements`, `exact_u64_maximum_total_is_admitted_without_float_conversion`, `required_overflowing_total_rejects_and_oversized_optional_skips_without_addition` |
| M2-P04 | Strict schemas/declarations/input/candidate/output budgets and nonempty real M0 plan | `future_schema_is_probed_and_nested_unknown_missing_null_fields_reject`, `duplicate_json_fields_are_rejected_at_root_candidate_and_time_boundaries`, `invalid_hash_times_ranges_and_hard_bounds_keep_precise_domain_errors` |
| M2-P05 | Read-only real file/stdin CLI, typed JSON/exits, existing validator consumer, option-input grammar and no runtime tools | `plans_file_and_stdin_identically_without_mutating_input_or_directory`, `emitted_nested_plan_is_accepted_by_the_actual_m0_cli`, `hard_conflicts_emit_typed_global_paths_without_a_partial_plan` |
| M2-P06 | Actual selected source invariants, hash-pinned synthetic oracle, explicit differences and complete declaration evidence | `tests/oracles/fixed-window-planner-v1/compare.py`:12 actual pinned cases, exact timings and classified policy deltas |

Complete test mapping is in the work index; M2-R01 full solver feasibility remains broader.

## M2-03 selected-domain live shadow requirements

The scoped live shadow is accepted via merged PR #26. It invokes
actual pinned source and native binaries; it does not add a product runtime.
See [profile](migration/PLANNER_SHADOW_V1.md), [record](migration/records/M2-03.md),
[work index](migration/m2-03-work-index.json) and [per-file inventory](migration/provenance-m2-03.json).
Python test symbols are individually recorded and AST-checked; the repository guard
checks their requirement IDs and verification artifact paths. All ten same-head
hosted checks passed; the comparison record preserves the accepted identities.

| ID | Requirement | Actual verification artifact |
| --- | --- | --- |
| M2-S01 | Seeded exact shared-domain admission, ordering/capture constraints and immutable input cases | `cases.py`, `test_shadow.py` |
| M2-S02 | Actual pinned source producer, strict schemas/error families/metadata and no source mutation | `producer.mts`, `test_producer.py` |
| M2-S03 | Exact source/native semantics, canonical literal delta/refusal probes and reject unregistered pairings | `compare.py`, `test_shadow.py`, `mutation-baselines.json` |
| M2-S04 | Bounded full-duplex process I/O, global/per-child budgets and owned-group reaping | `process.py`, `test_process.py` |
| M2-S05 | Actual live run/provenance/validator/failed evidence and CI test-only closure | `run.py`, `test_orchestration.py` |
| M2-S06 | Actual completed PR classification/register/lifecycle metadata with preserved history | `PR_REGISTER.md`, `pr-register.json` |

## Merged PR #28 approval-identity candidate (not accepted)

The following M3-A requirements describe a separate merged candidate, not the
canonical proposed M3-01/P1 finite-OTIO review-binding task tracked in
[`M3-01-NEXT`](migration/M3-01-NEXT.md). They do not establish G2 acceptance,
trusted human authorization, replay protection or host execution. Independent
G2 review is pending. Implementation and verification evidence are in the
[candidate work index](migration/m3-01-approval-candidate-work-index.json),
[record](migration/records/M3-01-approval-candidate.md) and
[validation](validation/m3-01-approval-candidate.md).

| ID | Candidate requirement | Verification | State |
| --- | --- | --- | --- |
| M3-A01 | Bind exact proposal/artifact bytes, verified source identities, project, declared host claims/capabilities and accepted losses | Candidate work index and approval integration tests | Implemented on PR #28; not accepted |
| M3-A02 | Strict bounded Ed25519 verification returns an opaque result and rejects tampering, malformed data and untrusted signer bindings | Candidate work index and approval integration tests | Implemented on PR #28; independent G2 pending |
| M3-A03 | A valid signature does not establish human authorization, durable replay safety or host execution | Candidate API-boundary review, compile-fail examples and work index | Candidate scope; broader requirements remain open |

## M3-01/P1 provisional finite-OTIO review binding

This implementation candidate follows the separately prepared [M3-01 contract and source map](migration/M3-01-NEXT.md)
and is tracked by [issue #27](https://github.com/bynanci/storytelling/issues/27). M3-RB01–RB07
are provisional identifiers pending independent G2 review and acceptance. The current candidate
only binds a caller-provided composition byte slice to an importer plan, its OTIO serialization,
the complete OTIO report and declared context; it does not implement trusted approval.

| ID | Provisional requirement | Implementation | Actual tests / evidence |
| --- | --- | --- | --- |
| M3-RB01 | Deterministic versioned SHA-256 identity commits to typed plan, OTIO artifact, full loss report, profile/compiler and context | `core/source_evidence.rs`, `engine/review_binding.rs` | `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact`, `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB02 | Import and compile from one exact composition input; artifact is regenerated during freshness checks, never accepted as caller-supplied digest evidence | `engine/review_binding.rs` | `changed_composition_and_each_declared_context_field_are_reported_stale`, `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB03 | Strict bounded context/record schemas, exact identifiers/digests, sorted unique choices/capabilities and required compiler capabilities | `core/review.rs`, `engine/review_binding.rs` | `context_parser_rejects_unknown_duplicate_unsorted_and_over_budget_values`, `json_depth_scanner_accepts_exact_limit_and_ignores_string_contents`, `json_depth_and_record_byte_budgets_are_enforced_before_deserialization`, `exact_context_and_record_byte_limits_are_accepted`, `capability_collection_accepts_exact_limit_and_rejects_limit_plus_one`, `context_must_declare_compiler_required_capabilities_and_record_digest_is_checked` |
| M3-RB04 | In-process result has private fields and cannot be constructed, mutated or deserialized through public API | `engine/review_binding.rs` | Four compile-fail examples on `BoundReviewV1`; `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` |
| M3-RB05 | No self-approval: portable records are untrusted and authority/evidence flags stay false; forged true claims reject | `core/review.rs`, `engine/review_binding.rs` | `review_record_rejects_unknown_duplicate_forged_claims_and_invalid_digests`, `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` |
| M3-RB06 | Compare a supplied current composition/context and identify changed fields; do not claim authoritative time, revocation, replay or request consumption | `engine/review_binding.rs` | `changed_composition_and_each_declared_context_field_are_reported_stale`, `every_declared_context_field_has_a_stable_stale_identifier`, `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB07 | Read-only file/stdin CLI with path-free errors, stable usage/I/O/domain exits and unchanged explicit inputs | CLI composition root | `prepare_and_check_are_read_only_and_stale_is_a_successful_report`, `prepare_review_reads_composition_from_stdin_but_context_remains_an_explicit_file`, `review_commands_keep_usage_and_io_errors_distinct_and_path_free` |

The portable record digest detects accidental/tampered field changes but is not a MAC or signature;
the current caller can create a self-consistent record. A supplied context's host fingerprint and
capabilities are declarations, not observations. Exact artifact linkage applies only to the one
composition slice passed to importer+compiler and does not establish verified source provenance or
connect the fixed-window planner to OTIO. Hosted dependency audit, three-platform same-head CI,
repository regression/oracle lanes, independent G2 review and maintainer acceptance remain required
before this provisional slice can be checked complete. See the [M3-01 work index](migration/m3-01-p1-work-index.json).

## Remaining breadth and future requirements

## Evidence levels

Structural parsing < declared semantic validation < actual source verification <
host readback/render verification < human editorial acceptance. These are distinct
claims, not interchangeable scores. M0/M1-01 and the bounded M2-01 audit implement
only the first two levels.
