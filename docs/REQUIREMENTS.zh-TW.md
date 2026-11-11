# 需求與驗收對照

[English](REQUIREMENTS.md) · 繁體中文

作者工作流程請查[跨媒體敘事](#proposed-cross-media-narrative-requirements)或[多模態需求](#multimodal-authoring-and-follow-up-requirements)。下列影片工程紀錄保留各項需求的測試名稱、驗證工具與版本。

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

## M2-02/O1 ordering optimisation

The private scheduler preserves the accepted fixed-window v1 profile. It reduces
repeated readiness scans while retaining admission-priority order and exact errors.
See [record](migration/records/M2-ORDER-OPT.md),
[work index](migration/m2-order-opt-work-index.json) and
[provenance](migration/provenance-m2-order-opt.json). Final delivery status is in
[issue #33](https://github.com/bynanci/storytelling/issues/33).

| ID | Requirement | Actual test / evidence |
| --- | --- | --- |
| M2-O01 | Preserve minimum-ready admission priority and no-partial-plan errors | `compares_every_four_vertex_graph_with_brute_force_priority_order`, `newly_ready_earlier_admission_precedes_already_queued_vertex`, `rejects_unselected_endpoints_self_edges_and_cycles_without_partial_order`, `newly_unblocked_required_asset_keeps_admission_priority_in_the_full_report` |
| M2-O02 | Heap-based scheduling within existing graph limits, with descriptive performance evidence | `maximum_candidate_graphs_keep_every_vertex_and_precedence`, `benchmark_ordering` (manual, informational, no timing assertion) |
| M2-O03 | Keep frozen/live source conformance, pure dependency boundary and cross-platform checks | Existing planner comparison, live shadow, per-file provenance and CI; full mapping in the work index |

## M2-02/O2 exact-clock construction optimisation

The private constructor preserves the existing exact-time and failure contract.
See [record](migration/records/M2-CLOCK-OPT.md),
[work index](migration/m2-clock-opt-work-index.json) and
[provenance](migration/provenance-m2-clock-opt.json). Final delivery is tracked by
[issue #35](https://github.com/bynanci/storytelling/issues/35).

| ID | Requirement | Actual test / evidence |
| --- | --- | --- |
| M2-CT01 | Error-only diagnostic formatting, exact reduced times and unchanged LCM/field error precedence | `successful_cached_scaling_never_formats_an_error_path`, `cached_scaling_formats_the_precise_failure_path_once`, `fractional_clock_preserves_exact_windows_and_reduced_placements`, `clock_error_priority_keeps_the_full_lcm_pass_before_scaling` |
| M2-CT02 | Generated/supported-size compatibility with the previous factory and descriptive performance measurements | `generated_canonical_clock_inputs_match_the_accepted_implementation`, `full_candidate_budget_cache_and_empty_private_clock_keep_exact_results`, `benchmark_clock_cache` (manual, no timing assertion) |
| M2-CT03 | Retain frozen/live source conformance, provenance and cross-platform gates | Existing fixed-window comparison/live shadow; complete mappings in the work index |

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

## M3-01/P1 accepted bounded finite-OTIO review binding

The bounded first profile follows the [M3-01 contract and source map](migration/M3-01-NEXT.md)
and [issue #27](https://github.com/bynanci/storytelling/issues/27). M3-RB01–RB07 are accepted
for this scope by the signed [G2 acceptance record](migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md).
This does not accept broader M3-01 or PR #28. The implementation binds a caller-provided
composition byte slice to an importer plan, its OTIO serialization, the complete OTIO report
and declared context; it does not implement trusted approval.

| ID | Accepted bounded requirement | Implementation | Actual tests / evidence |
| --- | --- | --- | --- |
| M3-RB01 | Deterministic versioned SHA-256 identity commits to typed plan, OTIO artifact, full loss report, profile/compiler and context | `core/source_evidence.rs`, `engine/review_binding.rs` | `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` asserts the fixed cross-platform golden vector; `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB02 | Import and compile from one exact composition input; artifact is regenerated during freshness checks, never accepted as caller-supplied digest evidence | `engine/review_binding.rs` | `changed_composition_and_each_declared_context_field_are_reported_stale`, `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB03 | Strict bounded context/record schemas, exact identifiers/digests, sorted unique choices/capabilities and required compiler capabilities | `core/review.rs`, `engine/review_binding.rs` | `context_parser_rejects_unknown_duplicate_unsorted_and_over_budget_values`, `json_depth_scanner_accepts_exact_limit_and_ignores_string_contents`, `json_depth_and_record_byte_budgets_are_enforced_before_deserialization`, `exact_context_and_record_byte_limits_are_accepted`, `capability_collection_accepts_exact_limit_and_rejects_limit_plus_one`, `context_must_declare_compiler_required_capabilities_and_record_digest_is_checked` |
| M3-RB04 | In-process result has private fields and cannot be constructed, mutated or deserialized through public API | `engine/review_binding.rs` | Four compile-fail examples on `BoundReviewV1`; `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` |
| M3-RB05 | No self-approval: portable records are untrusted and authority/evidence flags stay false; forged true claims reject | `core/review.rs`, `engine/review_binding.rs` | `review_record_rejects_unknown_duplicate_forged_claims_and_invalid_digests`, `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` |
| M3-RB06 | Compare a supplied current composition/context and identify changed fields; do not claim authoritative time, revocation, replay or request consumption | `engine/review_binding.rs` | `changed_composition_and_each_declared_context_field_are_reported_stale`, `every_declared_context_field_has_a_stable_stale_identifier`, `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB07 | Read-only file/stdin CLI with path-free errors, stable usage/I/O/domain exits and unchanged explicit inputs | CLI composition root | `prepare_and_check_are_read_only_and_stale_is_a_successful_report`, `prepare_review_reads_composition_from_stdin_but_context_remains_an_explicit_file`, `review_commands_keep_usage_and_io_errors_distinct_and_path_free`, `review_cli_reads_arguments_before_domain_validation_and_keeps_error_order`, `direct_review_api_error_precedence_is_size_record_context_then_compile` |

The portable record digest detects accidental/tampered field changes but is not a MAC or signature;
the current caller can create a self-consistent record. A supplied context's host fingerprint and
capabilities are declarations, not observations. Exact artifact linkage applies only to the one
composition slice passed to importer+compiler and does not establish verified source provenance or
connect the fixed-window planner to OTIO. Hosted dependency audit, three-platform same-head CI,
all existing repository regression/oracle lanes and the six exact-head hosted gates passed for
PR #32 head `2e7662405a49698be1ebee2136f54ab539ef7c2e` (run 42425697537). A bounded paired
OpenFilm `NleHandoffService.review/publish` and Rust CLI runtime oracle now runs on the pinned
synthetic finite composition; its outcomes and limits are in the [runtime record](migration/records/M3-01-RUNTIME-ORACLE-2026-11-08.md).
The encoding intentionally differs and no full source parity is claimed. The historical
functional RED was unproven and is explicitly waived as a blocker in the signed acceptance
record; it is not represented as a test that passed. See the [M3-01 work index](migration/m3-01-p1-work-index.json).

## M3-01/P2 selected proposal to actual read-only OTIO (merged bounded delivery)

The [contract](migration/PLANNER_OTIO_REVIEW_V1.md) and
[work index](migration/m3-planner-review-work-index.json) implement the requested
planner bridge independently of PR #28 acceptance. All guarantees refer to supplied
declarations and the exact generated artifact, not source media or human approval.

| ID | Acceptance | Automated evidence |
| --- | --- | --- |
| M3-PB01 | Same snapshot and chosen fixed-window proposal produce reproducible actual OTIO; native/metadata-only/rejected semantics are explicit | `story_review_connects_actual_planner_output_to_exact_otio_without_source_or_host_writes`, `story_review_compiles_selected_subset_and_order_and_binds_the_exact_owned_artifact`; official oracle |
| M3-PB02 | Any supplied asset/URI/hash, selected cut/order, proposal byte or accepted-loss/context change makes the old bundle stale or rejects it; displayed-artifact tampering rejects | `story_review_invalidates_asset_cut_loss_and_detects_displayed_artifact_tampering`, `story_review_is_stale_when_proposal_bytes_change_even_if_the_selected_plan_does_not` |
| M3-PB03 | Unsupported audio/effects reject even with picture-only choice; stills retain zero-origin bounded holds | `story_review_rejects_unsupported_audio_effects_missing_loss_choices_and_bad_cli_usage`, `story_review_preserves_still_hold_and_rejects_still_source_offsets` |
| M3-PB04 | Strict whole planner output and nested DTOs; matched asset declarations; opaque immutable result; forged authority and composition-only record reject | `story_review_rejects_unknown_duplicate_forged_or_inconsistent_proposals`, `story_review_requires_snapshot_asset_identity_duration_and_declared_capabilities`, `story_review_rejects_forged_bundle_claims_and_never_accepts_a_composition_only_record`; compile-fail doctests |
| M3-PB05 | Bound all input/output sizes/depth; refuse malformed/unknown/duplicate data with stable field-only diagnostics | `story_review_preflights_every_size_and_accepts_exact_snapshot_and_bundle_limits`; existing OTIO expansion tests |
| M3-PB06 | Actual CLI file/stdin flow works with empty external-tool PATH and unchanged Catalog/project/input sentinels; no host calls | `story_review_connects_actual_planner_output_to_exact_otio_without_source_or_host_writes`; official oracle |
| M3-PB07 | Export and freshness paths share compile/bind semantics while avoiding unused portable-bundle serialization; artifact bytes and freshness identity remain unchanged | `story_review_compiles_selected_subset_and_order_and_binds_the_exact_owned_artifact`, `story_review_invalidates_asset_cut_loss_and_detects_displayed_artifact_tampering`; same-head CI |

The bounded M3-02 journal implementation is traced separately below. It does not
call a host or make a `NotApplied` observation true. The M3-03 proof covers an
already-applied import whose response is lost after the host exits. The production
executor still must handle requests that remain in flight, establish process
quiescence before any retry, and stop on unresolved or ambiguous readback. M4's
reference application feature flag follows accepted journal/executor readback, with one writer.

## SQLite operation journal

The [M3-02 contract](migration/M3-02-OPERATION-JOURNAL.md) defines the accepted,
bounded non-executable storage adapter delivered in PR #56. It does not complete
FilmCraft execution/readback or PR #28 G2 acceptance.

| ID | Acceptance | Implementation | Regression evidence |
| --- | --- | --- | --- |
| M3-JL01 | Persist exact operation, request key, project, review-binding digest and artifact digest; exact replay is idempotent and changed identity rejects | `crates/storytelling-journal/src/lib.rs` | `exact_review_and_artifact_identity_survive_restart_and_idempotent_prepare`, `reused_idempotency_key_with_changed_review_or_artifact_is_rejected` |
| M3-JL02 | Persist attempt identity and dispatching state before any host call; restart after a committed marker recovers as unknown | `crates/storytelling-journal/src/lib.rs` | `crash_after_dispatch_marker_recovers_unknown_and_never_retries_without_readback` |
| M3-JL03 | Monotonic fence epochs and attempt numbers reject stale owners and late old responses/readbacks | `crates/storytelling-journal/src/lib.rs` | `crash_after_dispatch_marker_recovers_unknown_and_never_retries_without_readback`, `stale_owner_and_late_attempt_response_cannot_overwrite_newer_state` |
| M3-JL04 | Unknown/reported/partial outcomes do not authorize another attempt; only explicit current-attempt NotApplied readback permits retry, and newer conflicting readback can block it before retry | `crates/storytelling-journal/src/lib.rs` | `unknown_response_and_applied_response_never_enable_a_retry`, `crash_after_dispatch_marker_recovers_unknown_and_never_retries_without_readback`, `a_fresh_conflicting_readback_before_retry_keeps_the_attempt_blocked` |
| M3-JL05 | Busy write is atomic/retryable after lock release; unrelated SQLite schemas reject before journal-mode/schema mutation | `crates/storytelling-journal/src/lib.rs` | `busy_write_leaves_no_partial_record_and_succeeds_after_lock_release`, `foreign_database_is_rejected_without_changing_its_schema_or_journal_mode` |
| M3-JL06 | Validate identifiers, lowercase digests, timestamps and lease duration; enforce per-operation attempt/readback/event bounds while leaving other operations available | `crates/storytelling-journal/src/lib.rs` | `invalid_identity_hash_and_time_limits_are_rejected_at_the_boundary`, `attempt_count_limit_is_per_operation_and_does_not_block_other_operations`, `repeated_ambiguous_readbacks_have_a_per_attempt_bound` |

The observed initial RED was three executed tests panicking at the unimplemented
journal operation; after the storage implementation they passed. A later
late-readback, ambiguous-readback-bound, conflicting-readback and unrelated-db
regressions each failed before their respective safety checks were added, then
passed. PR #56 exact-head CI and post-merge main CI passed all six jobs; see the
[validation record](validation/m3-02-operation-journal.md). No host/editor was invoked.

Known limits: executor IDs and timestamps are caller supplied; journal fencing
cannot stop a previously dispatched host call; a `NotApplied` observation is
trusted input and is safe for retry only after a production host adapter proves
the old request is no longer in flight. That quiescence gate remains open after
the bounded M3-03 proof. No exactly-once effect, trusted approval or key
authorization is claimed.

<a id="m3-03-isolated-filmcraft-execution"></a>

## M3-03 isolated FilmCraft execution proof

The [M3-03 profile](migration/M3-03-ISOLATED-FILMCRAFT.md) and
[work index](migration/m3-03-filmcraft-work-index.json) track an initial
test-only execution/readback slice. It does not add a product executor command,
trusted approval, retry policy or cross-platform FilmCraft support.

| ID | Acceptance | Implementation | Regression evidence |
| --- | --- | --- | --- |
| M3-XE01 | Regenerate and freshness-check the selected proposal, snapshot, context and exact OTIO bytes before a synthetic host operation | Pinned integration oracle | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`; machine evidence |
| M3-XE02 | Persist project-creation and import identity, owner/fence and attempt marker before each host mutation; recover the final states after reopening SQLite | Test-only adapter calls M3-02 journal API | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`, `exact_review_and_artifact_identity_survive_restart_and_idempotent_prepare` |
| M3-XE03 | Lost successful import response becomes durable unknown; native readback matches exact 25 fps clip/source ranges before recording applied-observed | FilmCraft v0.18.0 Linux oracle | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`; [evidence](../tests/oracles/m3-03-filmcraft-v1/evidence.json) |
| M3-XE04 | No duplicate apply or Undo; terminal readback cannot be overwritten by a late response | Journal fence plus host proof | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`, `stale_owner_and_late_attempt_response_cannot_overwrite_newer_state` |
| M3-XE05 | All write paths are new files under a fresh run root; no existing project or source catalog is opened | Oracle preflight and isolated FilmCraft data directory | [oracle README](../tests/oracles/m3-03-filmcraft-v1/README.md), machine evidence |
| M3-XE06 | Ambiguous outcome cannot begin another attempt; existing journal failure regressions remain portable across workspace CI | Accepted M3-02 state machine | `unknown_response_and_applied_response_never_enable_a_retry`, `repeated_ambiguous_readbacks_have_a_per_attempt_bound` |

The lost-response fault is injected only after the FilmCraft subprocess has
exited successfully. It does not demonstrate a hung host or safe retry following
NotApplied. The actual host proof is Linux-only; Windows/macOS run the journal
regressions and compile the ignored integration test. Review freshness remains
distinct from human approval: the proof explicitly records approval_granted=false
and executable=false. The production executor and PR #28 G2 gate remain open;
see the [validation record](validation/m3-03-filmcraft-execution.md).

<a id="remaining-breadth-and-future-requirements"></a>

## 敘事功能與後續需求

<a id="proposed-cross-media-narrative-requirements"></a>

### 跨媒體敘事需求

[設計依據](WHY_STORYTELLING.zh-TW.md)說明作者要做哪些決定。[敘事任務](../TASKS.zh-TW.md#cross-media-narrative-backlog)列出實作順序。每項 PR 須附使用者給 [Skill](../skills/storytelling/SKILL.zh-TW.md) 的請求及實際產物。驗證自動化行為時，請附輸入、命令、輸出及退出碼。評估作者的使用結果時，請附真人觀察。

| 需求／任務 | 使用者要完成的事 | 驗收依據 |
| --- | --- | --- |
| N-R01／N-01 | 說出對象、目的與自己提供的細節。 | [作者試用包](narrative/N-01-AUTHOR-STUDY.zh-TW.md)以提交 `260485b2ff5b8d7b46215004e21a5bb1a37e6dfb` 為基準。使用 S01 至 S04 製作口述及兩款構圖。C01 只屬於 A。U01 是尚無佐證的商業主張。試用包提供相同的身分修訂題、四項指標與決策門檻。真人觀察與維護者的 G0–G1 決定待補。 另有[盲讀與改稿流程](../examples/evaluation/README.zh-TW.md)及空白紀錄表。 |
| N-R02／N-02 | 用獨立版本的敘事文件描述素材與關係。作者無須提供媒體檔或時間資訊。 | [ADR 0005](adr/0005-independent-narrative-document-v1.md)、[V1 規格](narrative/NARRATIVE_DOCUMENT_V1.md)、[JSON Schema](../contracts/narrative-document-v1.schema.json)與[成對測試案例](../tests/fixtures/narrative-v1/README.md)定義支援範圍。口述與構圖可共用來源。允許雙向聯想。強制順序成環、引用遺失或過期、重複欄位、未知版本或超量都要回報。相容性清單涵蓋影片 JSON、CLI、既有匯入格式、OTIO 與雜湊。PR #71 的送審版本通過 CI。修正 P2 循環診斷後，自動複查沒有待處理項目。維護者在 #50 採用此契約。 |
| N-R03／N-03 | 用同一來源產出一份口述與兩款構圖。 | `render-narrative` 讀取 [V1 範例](../tests/fixtures/narrative-v1/n03-authoring-v1.json)。[驗證紀錄](validation/n03-narrative-cli.md)列出三份輸出。全景與近景各自保留來源 ID，C01 只出現在全景。測試涵蓋無媒體輸入、檔案與標準輸入重播、原檔與暫存目錄不變。引用不存在時回傳 JSON 錯誤及退出碼 1。引擎測試涵蓋 N-02、順序、來源、位元組數與深度限制。開發者在實作前記錄失敗測試。影片 V1 與官方 OTIO 回歸另有紀錄。自動化結果與作者試讀分開記錄。 已在同一紀錄附上請求者檢視、修訂文件 revision 2 及僅供近景的 C02 提案；比較性真人結果仍未觀察。 |
| N-R04／N-04 | 保留多款草稿，找出一項修訂影響的版本。 | [修訂契約](narrative/NARRATIVE_REVISION_LEDGER_V1.md)、[格式](../contracts/narrative-revision-ledger-v1.schema.json)、[案例](../tests/fixtures/narrative-v1/n04-revision-history-v1.json)與[驗證紀錄](validation/n04-narrative-revisions.md)可供重播。修訂 S02 影響口述與兩款構圖。審閱後再修訂 C01，只影響構圖 B。歷史保留來源、變體及宣告的作者選擇。提案事件不可夾帶決定。CLI 只計算直接引用，無法驗證決定者身分。PR #76 於 `9eb12eefe41ef965acee1f5e78e373e3b782372c` 合併；同一提交與合併後 CI run #218 的已設定檢查均通過。真人檢視仍待完成。 |
| N-R05／N-05 | 寫出文案大綱，辨識還缺依據的主張。 | `audit-copy-claims` 唯讀輸出受眾、主張、提議措辭、引用及 CTA。測試目前、缺漏、懸空、舊版、未知版本／欄位、重複鍵及文件不匹配。報告附上被引用的歷史文字；`reference_status` 只判斷 ID 與 revision。真人仍須判斷語意佐證和發布適宜性。[契約](narrative/COMMERCIAL_COPY_V1.md) · [驗證紀錄](validation/n05-commercial-copy.md) |
| N-R06／N-06 | 選出值得開發的新模式。 | [格式探索紀錄](validation/n06-format-discovery.md)比較三個候選方向、現有契約相容性與最小可行變更。請求者已[明確暫緩三者](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714)；不宣稱任務試用或耗時結果。 |
| N-R07／N-07 | 依實際 native 工作流證據決定繼續、縮小或停止。 | [N-07 腳本與報告](validation/n07-pilot.zh-TW.md)固定 N-01 的明示 native 對照，記錄版本、分母、作者時間、歸因、重複輸入與重工；CLI 毫秒另列，真人欄留白。run_cli 及 test_run_cli.py／test_packet.py 保護新輸出、識別與試用材料。產品決策待真人證據，不推薦 RC。 |

N-R02 的資料契約與 N-01 的試用條件已固定。新增觀察可以補充證據。變更契約或試用條件時，須另行記錄決定。

研究、規則與技能流程的對照請見[研究設計](research/README.zh-TW.md)。P01–P11/S01、ST-R01–ST-R14、W1–W6 連結到既有 N 需求；這些是編輯指引與評估問題，不增加 V1 欄位或已驗收需求。

<a id="skill-distribution"></a>

## Skill 散布

**SK-R01／SK-D01**　使用者可從 GitHub 安裝 Skill，也可用 npm 安裝後啟用。兩種方式須提供相同的指引、參考文件、輔助程式與授權。[散布測試](../scripts/test_skill_distribution.py)會打包實際檔案，在全新專案安裝後逐檔比對。[安裝與更新](PLATFORMS.zh-TW.md)

<a id="evidence-levels"></a>

## 驗證能證明什麼

| 驗證方式 | 可支持的結論 |
| --- | --- |
| 解析資料 | 工具能讀取輸入結構。 |
| 驗證宣告 | 資料內的引用、順序或限制符合契約。 |
| 驗證來源 | 實際提供的位元組符合預期值。 |
| 讀回或播放 | 指定平台確實產出可讀或可播放的檔案。 |
| 作者驗收 | 實際參與的作者確認作品符合其用途。 |

M0、M1-01 與限定範圍的 M2-01 稽核只支援前兩項。

<a id="multimodal-authoring-and-follow-up-requirements"></a>

## 多模態需求

[樣板](../skills/storytelling/references/multimodal-templates.zh-TW.md)與[工作坊範例](../skills/storytelling/references/workshop-demo.zh-TW.md)可直接試用。下列工作各有獨立的驗收結果。

| 需求／任務 | 可觀察的結果 |
| --- | --- |
| N-R08／[N-08](https://github.com/bynanci/storytelling/issues/65) | 以同一份合成素材比較兩種開場。綁定敘事文件 revision 的 `NarrativeStyleProfileV1` CLI 檢查口述變體 beat 順序。有效規則可通過；同一變體的循環會列出規則 ID；順序違反、舊 revision 和未知版本均回傳穩定診斷。[N-08 驗證紀錄](validation/n08-style-profile.md)列出重跑測試及成對案例。真人盲讀仍待執行；檢查器不宣稱評價美感或代表作者核准。 |
| N-R09／[N-09](https://github.com/bynanci/storytelling/issues/66) | `render-cue-sheet` 輸出綁定口述 `NarrativeDocumentV1` revision 的獨立封閉提示表。保留工作坊受眾、週六活動與帶一把傘的素材；旁白標為待錄、停頓標示估時，`taps.wav` 標為已宣告音效。允許跨軌疊音與半開區間首尾相接；拒絕互斥軌衝突、反向區間、重複 ID 與未知版本。保存輸出與結束碼，證明逐位元一致重播，並維持 `NarrativeDocumentV1` 不變。[規格](narrative/NARRATIVE_CUE_SHEET_V1.zh-TW.md) · [N-09 驗證紀錄](validation/n09-cue-sheet.zh-TW.md)。 |
| N-R10／[N-10](https://github.com/bynanci/storytelling/issues/67) | 依 N-06 請求者決策暫緩。後續決策須明選一案並列作者需求與版本範圍，才啟動正常 CLI 正反例及既有影片回歸。 |
| N-R11／[N-11](https://github.com/bynanci/storytelling/issues/68) | `render-handoff` 透過原有 N-04 修訂簿對照文字、圖像與聲音部分。[規格](narrative/NARRATIVE_HANDOFF_MANIFEST_V1.zh-TW.md)、[案例](../tests/fixtures/narrative-handoff-v1/README.md)及[證據](validation/n11-handoff.md)涵蓋事先固定的日期影響集合、遺失與過期引用、專屬提案外溢、完整對照、描述來源、來源未變及一致重播。語意另由作者檢視。 |
| N-R12／[N-12](https://github.com/bynanci/storytelling/issues/69) | [Inkscape 可行性試驗](validation/n12-inkscape-png.zh-TW.md)以同一請求與 S2 交付真正 PNG、完整解碼及視覺讀回，再受控隱藏匯出入口交付構圖說明。保存來源前後身分與命令／退出碼。正式 adapter 的縮小採用建議待維護者決定。 |

