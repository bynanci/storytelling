# Requirements and acceptance criteria

<a name="requirements-and-verification-traceability"></a>
<a name="需求與驗收對照"></a>

English · [繁體中文](REQUIREMENTS.zh-TW.md)

See the [narrative requirements](#proposed-cross-media-narrative-requirements) and [multimodal requirements](#multimodal-authoring-and-follow-up-requirements) for authoring workflows. The video tables identify test names, tools and their versions.

Main includes M0, finite M1-01/M1-02, the bounded M2-01 audit and M2-02 fixed-window planning. It also includes the M1-03 source and host evidence for M1-H01–H04. Broader host tests, full-planner tests and human acceptance have separate requirements. The repository guard checks the test names.

| ID | Requirement | Implementation owner | Acceptance test |
| --- | --- | --- | --- |
| M0-R01 | Accept valid declared metadata and reused source occurrences | engine/lib.rs | `accepts_declared_plan_and_reused_source` |
| M0-R02 | Refuse unsupported version, unknown and duplicate fields | core/wire.rs + engine | `rejects_unsupported_version`, `rejects_unknown_and_duplicate_json_fields` |
| M0-R03 | Check IDs, declared hash format, uniqueness and source references | engine | `rejects_duplicate_identities`, `rejects_missing_source_reference`, `rejects_invalid_identity_and_hash` |
| M0-R04 | Canonical bounded exact rational source time, no float rounding | core/time.rs | `rejects_noncanonical_and_zero_denominator_time`, `compares_large_rationals_without_float_rounding` |
| M0-R05 | Nonempty source ranges within positive declared duration | engine | `rejects_empty_or_reversed_ranges`, `rejects_source_range_outside_declared_duration`, `rejects_zero_duration_and_preserves_input_bytes` |
| M0-R06 | Bound input and collections | CLI + engine | `rejects_malformed_or_oversized_input`, `rejects_empty_and_over_budget_collections` |
| M0-R07 | Return deterministic, read-only results for declared metadata. Media verification requires separate evidence. | engine + CLI | `accepts_exact_rational_intervals_and_is_deterministic`, `accepts_valid_file_with_machine_readable_output` |
| M0-R08 | Machine JSON and distinct validation/usage/I/O outcomes | CLI | `rejects_bad_source_range_with_exit_one`, `accepts_stdin`, `distinguishes_usage_and_io_failures`, `reports_help_and_version_without_loading_a_project` |

Rust integration tests are under `crates/*/tests`. The compile-fail doctests cover private construction and deserialization restrictions in the M1-03 API. The synthetic fixtures use placeholder hashes. A passing test establishes consistency of the declared metadata.

## M1-01 finite importer requirements

Importer IDs use M1-Ixx to preserve the existing M1-Rxx host-handoff requirements.
See the frozen [contract](migration/FINITE_V1.md),
[source/test/evidence record](migration/records/M1-01.md) and
[machine work index](migration/m1-01-work-index.json).

| ID | Requirement / OF-GAP scope | Implementation | Acceptance test |
| --- | --- | --- | --- |
| M1-I01 | Preserve source and occurrence identity, order and user facts / OF-01/04 | engine mapper + retained source snapshot | `repeat_source_preserves_occurrence_order_and_uses_maximum_still_hold`, `locks_selection_story_and_beats_remain_original_declarations` |
| M1-I02 | Exact decimal/source clocks, independent explicit timebase / OF-02 + GAP-03 | decimal.rs + clocks | `exact_decimal_and_exponent_continuity_does_not_infer_ntsc`, `seconds_above_f64_integer_precision_remain_exact_and_u64_max_is_allowed`, `explicit_timebase_is_separate_from_its_agreeing_decimal_declaration` |
| M1-I03 | Limit input and output sizes and preserve input bytes / OF-03 identity subset and GAP-10 | DTO + budget + mapped-plan cap | `rejects_oversized_input_and_bounded_metadata_depth_strings_keys_and_values`, `compact_source_whose_mapped_plan_exceeds_m0_input_limit_rejects`, `optional_fields_reject_explicit_null_instead_of_erasing_declarations` |
| M1-I04 | Unsupported features explicit, no executable/verified claim / OF-04 | structured errors/losses | `unknown_effect_and_current_animation_reject_with_one_explicit_loss`, `embedded_audio_rejects_and_missing_or_empty_probe_still_blocks_audio_handoff` |
| M1-I05 | Preserve the original snapshot and number lexemes as declarations / GAP-03/10 | raw snapshot + pure engine | `source_snapshot_preserves_large_numbers_lexemes_and_input_bytes`, `source_snapshot_minification_preserves_escaped_strings_whitespace_and_unicode` |
| M1-I06 | Usable file/stdin CLI, JSON/exit consistency / OF-04 + GAP-16 | CLI composition root | `imports_file_without_modifying_input_or_directory_and_returns_an_m0_plan`, `imports_stdin_with_the_same_result_as_file_input`, `treats_missing_video_media_and_instruction_text_as_opaque_declarations` |

The repository guard checks GAP-01 with `provenance_manifest_covers_every_imported_file`. GAP-02/16 use six pinned source cases and a differential comparator under `tests/oracles/openfilm-finite-v1`. CI replays frozen oracle outputs on the same commit. Real media and host behavior require separate evidence.

## M1-02 finite OTIO requirements

M1-02 covers format behavior. Source and host acceptance have separate requirements under M1-R01–R03. See the [contract](migration/FINITE_OTIO_V1.md), [comparison](migration/records/M1-02.md) and [work index](migration/m1-02-work-index.json).

| ID | Requirement / OF-GAP scope | Actual acceptance |
| --- | --- | --- |
| M1-O01 | Native reference/cut/still identity and range mapping / OF-35 subset | `still_emits_native_timeline_track_clip_and_synthetic_hold_reference`, `repeated_source_keeps_distinct_occurrences_order_and_maximum_hold_window`, `video_trim_preserves_exact_source_clock_and_declared_available_duration` |
| M1-O02 | Common exact content clock, safe native values/endpoints/whole duration / OF-02/35 | `fractional_cuts_share_integer_content_clock_without_project_fps_quantization`, `common_denominator_lcm_rejects_even_when_each_input_denominator_is_safe`, `accumulated_timeline_duration_checks_the_safe_integer_boundary_without_rounding`, `exact_safe_integer_boundary_is_accepted_and_larger_native_windows_reject` |
| M1-O03 | Explicit audio/lock choices, honest format capability/loss / OF-36 + GAP-05 subset | `audio_and_lock_losses_are_ordered_and_each_choice_only_accepts_its_own_loss`, `report_retains_import_limitations_and_never_claims_source_host_or_execution_verification`, `unsupported_source_features_keep_import_codes_paths_and_unaccepted_rows` |
| M1-O04 | Original snapshot lexemes and bounded amplified output / GAP-03/10 | `owner_snapshot_is_a_string_preserving_large_integer_decimal_and_opaque_exponent_lexemes`, `bounded_source_with_amplified_repeated_names_rejects_the_final_otio_budget` |
| M1-O05 | Usable direct OTIO stdout, strict loss flags, immutable file/stdin / GAP-14 subset | `exports_a_real_timeline_identically_from_file_and_stdin_without_writing`, `rejects_missing_extra_unknown_duplicate_and_misplaced_arguments_with_usage_two`, `preserves_numeric_lexemes_inside_the_source_snapshot_json_string` |
| M1-O06 | Actual source and official parser semantic conformance / GAP-02/16 | `tests/oracles/otio-finite-v1/verify.py`: six pinned TS cases plus two direct Rust fixtures, official 0.18.1 parser, writer and reader without opening media URIs |

## M2-01 bounded brief audit requirements

M2-Bxx covers the audit of declared constraints. M2-R01 covers the full planner. Merged PR #17 accepted M2-B01–B06. Release, host and full-planner acceptance have separate requirements. See the [contract](migration/STORY_BRIEF_V1.md), [source comparison record](migration/records/M2-01.md), [per-file provenance](migration/provenance-m2-01.json) and [complete requirement/test mapping](migration/m2-01-work-index.json).
[Tracking issue #14](https://github.com/bynanci/storytelling/issues/14) covers this scope. Verification and acceptance in merged PR #17 closed the issue.

| ID | Implemented bounded requirement | Implementation | Representative actual tests |
| --- | --- | --- | --- |
| M2-B01 | Require manual membership and order. Require all members of include and order declarations. Preserve declaration order and every edge origin. | core/brief.rs + engine/brief.rs | `missing_manual_choices_remain_required_and_ordered_instead_of_being_filtered`, `include_and_order_members_promote_required_assets_with_all_ordered_origins`, `duplicate_edges_retain_each_declaration_origin_without_reordering` |
| M2-B02 | Typed missing-reference, required/excluded and actual-cycle conflicts retain stable paths/order and conflicting choices | engine audit + brief/cycle.rs | `unknown_exclusions_and_repeated_reference_declarations_get_typed_conflicts`, `conflicting_fixture_has_stable_hard_diagnostic_order_and_required_origin_paths`, `cycle_witness_is_actual_and_excludes_acyclic_prefix_and_downstream_nodes` |
| M2-B03 | Keep preferences optional. Preserve the exact positive rational target and Unicode intent as declarations. Skip duration analysis and never execute the text. | core preferences + engine audit | `preference_warnings_preserve_values_and_never_relax_hard_requirements`, `positive_canonical_target_duration_is_preserved_without_feasibility_analysis`, `unicode_and_instruction_shaped_intent_remain_verbatim_inert_text` |
| M2-B04 | Enforce the supported version, shape and ID rules. Reject unknown, duplicate, null or unsupported declarations. | core DTOs + engine validation | `unknown_fields_and_unsupported_constraint_families_reject_strictly`, `duplicate_json_fields_reject_at_root_and_each_nested_object`, `future_version_is_probed_before_domain_validation_and_required_fields_are_strict` |
| M2-B05 | Limit input, intent, collections, total references, diagnostics and output. Use iterative traversal. Return deterministic results and preserve input bytes. | engine validation/audit + iterative cycle helper | `aggregate_reference_budget_accepts_8192_and_refuses_8193`, `combined_diagnostic_budget_accepts_256_and_errors_instead_of_truncating_at_257`, `full_depth_acyclic_order_and_large_origin_report_stay_within_output_budget`, `input_bytes_and_semantic_array_order_are_unchanged_and_reports_are_deterministic` |
| M2-B06 | Audit explicit file or stdin input without writes. Return one line of typed JSON and stable exit codes. Compare selected source policies with the finite oracle. | CLI composition root + policy oracle | `fixture_reports_only_declared_consistency_without_a_planned_selection`, `audits_file_and_stdin_identically_without_changing_the_input_or_directory`, `reports_typed_hard_conflicts_with_exit_one_and_keeps_manual_choices`, `redacts_missing_input_file_with_io_exit_three` |

The machine index maps all 22 engine tests and 7 CLI tests. The [ten-case policy oracle](../tests/oracles/story-brief-v1/README.md) contains the pinned source results and the differential comparator. It compares hard requirements for membership, order, references and conflicts. This project defines the brief schema, preferences and typed diagnostics. The audit checks declared rules. Shot selection and composition feasibility require a planner.


## M1-03 bounded source and FilmCraft proof

See [25 fps contract](migration/FILMCRAFT_25_V1.md),
[evidence record](migration/records/M1-03.md) and
[work index](migration/m1-03-work-index.json).

| ID | Requirement / boundary | Actual acceptance |
| --- | --- | --- |
| M1-H01 | Native SHA-256 of ≤16 MiB explicit bytes / GAP-04 and OF-29 subset | `accepts_independent_sha256_vectors_and_owns_verified_bytes`, `rejects_mismatched_and_noncanonical_expected_hashes`, `accepts_exact_buffer_budget_and_rejects_larger_before_hash_syntax` |
| M1-H02 | Check an explicit file or stdin. Return a metadata-only receipt without opening project URIs. | `accepts_explicit_source_bytes_with_metadata_only_receipt`, `rejects_bad_source_evidence_without_leaking_input`, `distinguishes_source_usage_and_io_errors`, and compile-fail examples for private construction and deserialization restrictions |
| M1-H03 | Actual pinned host 25 fps import, exact native ranges, default intrinsic effects and external ID mapping / M1-R01 subset | `tests/oracles/filmcraft-25-v1/run.py`, complete checked native baselines, `test_checks.py` semantic rejection tests |
| M1-H04 | Actual CPU H.264 192×108 preview, 45 exact PTS, 20/5/20 colors, save/reopen/OTIO reimport and unchanged inputs | Same harness and committed `tests/oracles/filmcraft-25-v1/evidence.json`. The evidence records source, hash and host identities. Approval requires separate evidence. |

## M2-02 fixed-window planning requirements

Merged PR #22 accepted M2-P01–P06 after all ten checks passed on the same commit. Issue #20 closed as complete. See the [profile](migration/FIXED_WINDOW_PLANNER_V1.md), [record](migration/records/M2-02.md), [work index](migration/m2-02-work-index.json) and [component inventory](migration/provenance-m2-02.json).

| ID | Requirement | Actual verification |
| --- | --- | --- |
| M2-P01 | Keep all required and manual membership and precedence constraints. Report typed conflicts, missing candidates and hard limits. | `emits_an_actual_m0_plan_with_deterministic_ids_and_immutable_source_facts`, `preserves_manual_and_explicit_order_with_global_reason_paths_and_catalog_order`, `rejects_hard_brief_conflicts_without_filtering_unknown_or_excluded_choices` |
| M2-P02 | Optional preference/score/ASCII selection is deterministic, skips actual capacity failures and keeps target soft | `optional_ranking_uses_preference_then_signed_score_then_ascii_id`, `too_long_optional_is_skipped_while_a_later_fitting_asset_is_selected`, `skipped_reasons_prioritize_exclusion_then_met_target_over_other_limits` |
| M2-P03 | Exact common integer clock, immutable windows/placements and bounded u64 arithmetic without rounding | `fractional_windows_preserve_source_offsets_and_exact_accumulated_placements`, `exact_u64_maximum_total_is_admitted_without_float_conversion`, `required_overflowing_total_rejects_and_oversized_optional_skips_without_addition` |
| M2-P04 | Strict schemas/declarations/input/candidate/output budgets and nonempty real M0 plan | `future_schema_is_probed_and_nested_unknown_missing_null_fields_reject`, `duplicate_json_fields_are_rejected_at_root_candidate_and_time_boundaries`, `invalid_hash_times_ranges_and_hard_bounds_keep_precise_domain_errors` |
| M2-P05 | Read-only real file/stdin CLI, typed JSON/exits, existing validator consumer, option-input grammar and no runtime tools | `plans_file_and_stdin_identically_without_mutating_input_or_directory`, `emitted_nested_plan_is_accepted_by_the_actual_m0_cli`, `hard_conflicts_emit_typed_global_paths_without_a_partial_plan` |
| M2-P06 | Actual selected source invariants, hash-pinned synthetic oracle, explicit differences and complete declaration evidence | `tests/oracles/fixed-window-planner-v1/compare.py` covers 12 actual pinned cases, exact timings and classified policy deltas |

The work index maps every test. M2-R01 also requires evidence of full solver feasibility.

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
| M2-O03 | Keep frozen/live source conformance, pure dependency boundary and cross-platform checks | The work index maps the planner comparison, live shadow, per-file provenance and CI |

## M2-02/O2 exact-clock construction optimisation

The private constructor preserves the existing exact-time and failure contract.
See [record](migration/records/M2-CLOCK-OPT.md),
[work index](migration/m2-clock-opt-work-index.json) and
[provenance](migration/provenance-m2-clock-opt.json). [Issue #35](https://github.com/bynanci/storytelling/issues/35) tracks delivery.

| ID | Requirement | Actual test / evidence |
| --- | --- | --- |
| M2-CT01 | Error-only diagnostic formatting, exact reduced times and unchanged LCM/field error precedence | `successful_cached_scaling_never_formats_an_error_path`, `cached_scaling_formats_the_precise_failure_path_once`, `fractional_clock_preserves_exact_windows_and_reduced_placements`, `clock_error_priority_keeps_the_full_lcm_pass_before_scaling` |
| M2-CT02 | Generated/supported-size compatibility with the previous factory and descriptive performance measurements | `generated_canonical_clock_inputs_match_the_accepted_implementation`, `full_candidate_budget_cache_and_empty_private_clock_keep_exact_results`, `benchmark_clock_cache` (manual, no timing assertion) |
| M2-CT03 | Retain frozen/live source conformance, provenance and cross-platform gates | The work index maps the fixed-window comparison and live shadow |

## M2-03 selected-domain live shadow requirements

Merged PR #26 accepted the live comparison for the selected domain. The test invokes pinned source and native binaries. The product has no dependency on this test runtime. See the [profile](migration/PLANNER_SHADOW_V1.md), [record](migration/records/M2-03.md), [work index](migration/m2-03-work-index.json) and [per-file inventory](migration/provenance-m2-03.json).
The work index lists each Python test symbol. The repository guard checks these symbols against the syntax tree and checks requirement IDs and evidence paths. All ten hosted checks passed on the same commit. The comparison record identifies the accepted versions and files.

| ID | Requirement | Actual verification artifact |
| --- | --- | --- |
| M2-S01 | Seeded exact shared-domain admission, ordering/capture constraints and immutable input cases | `cases.py`, `test_shadow.py` |
| M2-S02 | Actual pinned source producer, strict schemas/error families/metadata and no source mutation | `producer.mts`, `test_producer.py` |
| M2-S03 | Exact source/native semantics, canonical literal delta/refusal probes and reject unregistered pairings | `compare.py`, `test_shadow.py`, `mutation-baselines.json` |
| M2-S04 | Bounded full-duplex process I/O, global/per-child budgets and owned-group reaping | `process.py`, `test_process.py` |
| M2-S05 | Actual live run/provenance/validator/failed evidence and CI test-only closure | `run.py`, `test_orchestration.py` |
| M2-S06 | Actual completed PR classification/register/lifecycle metadata with preserved history | `PR_REGISTER.md`, `pr-register.json` |

## Merged PR #28 approval-identity candidate (not accepted)

Merged PR #28 contains the M3-A candidate. The [M3-01/P1 task](migration/M3-01-NEXT.md) covers a separate finite-OTIO review binding. PR #28 still requires independent G2 review. Its evidence does not establish trusted human authorization, replay protection or host execution. See the [candidate work index](migration/m3-01-approval-candidate-work-index.json), [record](migration/records/M3-01-approval-candidate.md) and [validation](validation/m3-01-approval-candidate.md).

| ID | Candidate requirement | Verification | State |
| --- | --- | --- | --- |
| M3-A01 | Bind exact proposal/artifact bytes, verified source identities, project, declared host claims/capabilities and accepted losses | Candidate work index and approval integration tests | PR #28 implementation. Acceptance pending. |
| M3-A02 | Strict bounded Ed25519 verification returns an opaque result and rejects tampering, malformed data and untrusted signer bindings | Candidate work index and approval integration tests | PR #28 implementation. Independent G2 review pending. |
| M3-A03 | A valid signature does not establish human authorization, durable replay safety or host execution | Candidate API-boundary review, compile-fail examples and work index | Candidate scope only. Broader requirements remain open. |

## M3-01/P1 accepted bounded finite-OTIO review binding

The first profile follows the [M3-01 contract and source map](migration/M3-01-NEXT.md) and [issue #27](https://github.com/bynanci/storytelling/issues/27). The signed [G2 acceptance record](migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md) covers M3-RB01–RB07 for this profile. Broader M3-01 requirements and PR #28 have separate acceptance gates. The implementation binds one caller-supplied composition to its importer plan, OTIO serialization, complete OTIO report and declared context. Trusted approval remains outside this profile.

| ID | Accepted bounded requirement | Implementation | Actual tests / evidence |
| --- | --- | --- | --- |
| M3-RB01 | Deterministic versioned SHA-256 identity commits to typed plan, OTIO artifact, full loss report, profile/compiler and context | `core/source_evidence.rs`, `engine/review_binding.rs` | `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` checks the cross-platform golden vector. Also see `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB02 | Import and compile one exact composition input. Regenerate the artifact for every freshness check. Reject caller-supplied digests as artifact evidence. | `engine/review_binding.rs` | `changed_composition_and_each_declared_context_field_are_reported_stale`, `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB03 | Strict bounded context/record schemas, exact identifiers/digests, sorted unique choices/capabilities and required compiler capabilities | `core/review.rs`, `engine/review_binding.rs` | `context_parser_rejects_unknown_duplicate_unsorted_and_over_budget_values`, `json_depth_scanner_accepts_exact_limit_and_ignores_string_contents`, `json_depth_and_record_byte_budgets_are_enforced_before_deserialization`, `exact_context_and_record_byte_limits_are_accepted`, `capability_collection_accepts_exact_limit_and_rejects_limit_plus_one`, `context_must_declare_compiler_required_capabilities_and_record_digest_is_checked` |
| M3-RB04 | In-process result has private fields and cannot be constructed, mutated or deserialized through public API | `engine/review_binding.rs` | Four compile-fail examples on `BoundReviewV1` and `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` |
| M3-RB05 | Treat portable records as untrusted. Keep authority and evidence flags false. Reject forged claims of authority or evidence. | `core/review.rs`, `engine/review_binding.rs` | `review_record_rejects_unknown_duplicate_forged_claims_and_invalid_digests`, `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact` |
| M3-RB06 | Compare the supplied current composition and context. Identify changed fields. Make no claim of authoritative time, revocation, replay protection or request consumption. | `engine/review_binding.rs` | `changed_composition_and_each_declared_context_field_are_reported_stale`, `every_declared_context_field_has_a_stable_stale_identifier`, `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding` |
| M3-RB07 | Read-only file/stdin CLI with path-free errors, stable usage/I/O/domain exits and unchanged explicit inputs | CLI composition root | `prepare_and_check_are_read_only_and_stale_is_a_successful_report`, `prepare_review_reads_composition_from_stdin_but_context_remains_an_explicit_file`, `review_commands_keep_usage_and_io_errors_distinct_and_path_free`, `review_cli_reads_arguments_before_domain_validation_and_keeps_error_order`, `direct_review_api_error_precedence_is_size_record_context_then_compile` Supplementary [shared CLI file-input hardening](validation/cli-file-input.md): `scripts/test_cli_file_input.py` checks 33 explicit file positions for nonblocking FIFO/directory rejection, regular-file/stdin replay and unchanged size errors. This is not path confinement or TOCTOU protection. |

The portable digest detects changes to record fields. A caller can recalculate it after tampering. Authentication requires a MAC or signature. The caller declares the host fingerprint and capabilities. The binding covers one composition input to the importer and compiler. Source provenance and the fixed-window planner connection to OTIO require separate evidence.

For PR #32, all six hosted gates passed on commit `2e7662405a49698be1ebee2136f54ab539ef7c2e` (run 42425697537). Checks included the dependency audit, three-platform CI, repository regressions and oracles. A runtime oracle compares upstream `NleHandoffService.review/publish` with the Rust CLI on the pinned synthetic finite composition. The implementations use different encodings. The [runtime record](migration/records/M3-01-RUNTIME-ORACLE-2026-11-08.md) documents the results and limits of this comparison. The signed acceptance record waived the missing historical failing test as a blocker. It did not record a passing result for that test. See the [M3-01 work index](migration/m3-01-p1-work-index.json).

## M3-01/P2 selected proposal to actual read-only OTIO (merged bounded delivery)

The [contract](migration/PLANNER_OTIO_REVIEW_V1.md) and [work index](migration/m3-planner-review-work-index.json) document the planner bridge. Its implementation proceeds independently of PR #28 acceptance. The guarantees cover supplied declarations and the exact generated artifact. Source media and human approval require separate evidence.

| ID | Acceptance | Automated evidence |
| --- | --- | --- |
| M3-PB01 | Generate reproducible OTIO from the same snapshot and selected fixed-window proposal. Report native, metadata-only and rejected behavior. | `story_review_connects_actual_planner_output_to_exact_otio_without_source_or_host_writes`, `story_review_compiles_selected_subset_and_order_and_binds_the_exact_owned_artifact`, plus the official oracle |
| M3-PB02 | Detect any change to the supplied asset, URI, hash, selected cut or order, proposal bytes, accepted losses or context. Mark the bundle stale or reject it. Reject changes to the displayed artifact. | `story_review_invalidates_asset_cut_loss_and_detects_displayed_artifact_tampering`, `story_review_is_stale_when_proposal_bytes_change_even_if_the_selected_plan_does_not` |
| M3-PB03 | Reject unsupported audio and effects even with the picture-only choice. Preserve bounded still-image holds that start at zero. | `story_review_rejects_unsupported_audio_effects_missing_loss_choices_and_bad_cli_usage`, `story_review_preserves_still_hold_and_rejects_still_source_offsets` |
| M3-PB04 | Check the whole planner output and nested DTOs. Require matching asset declarations. Return an opaque, immutable result. Reject forged authority and composition-only records. | `story_review_rejects_unknown_duplicate_forged_or_inconsistent_proposals`, `story_review_requires_snapshot_asset_identity_duration_and_declared_capabilities`, `story_review_rejects_forged_bundle_claims_and_never_accepts_a_composition_only_record`, plus compile-fail doctests |
| M3-PB05 | Limit all input and output sizes and depth. Reject malformed, unknown or duplicate data. Return stable diagnostics that identify fields only. | `story_review_preflights_every_size_and_accepts_exact_snapshot_and_bundle_limits`, plus existing OTIO expansion tests |
| M3-PB06 | Run the file and stdin CLI with an empty external-tool PATH. Preserve Catalog, project and input sentinels. Make no host calls. | `story_review_connects_actual_planner_output_to_exact_otio_without_source_or_host_writes`, plus the official oracle |
| M3-PB07 | Use the same compilation and binding rules for export and freshness checks. Skip unused portable-bundle serialization. Preserve artifact bytes and freshness identity. | `story_review_compiles_selected_subset_and_order_and_binds_the_exact_owned_artifact`, `story_review_invalidates_asset_cut_loss_and_detects_displayed_artifact_tampering`, plus CI for the same commit |

The M3-02 journal records caller observations, including `NotApplied`. It makes no host calls. The M3-03 proof covers a successful import with a lost response after the host exits. A production executor must handle requests still in flight. Before retrying, it must check that the previous request is no longer in flight. It must stop on unresolved or ambiguous readback. The M4 reference application feature flag requires an accepted journal and executor readback with one writer.

## SQLite operation journal

The compatible first-readback receipt correction and observed RED/GREEN are [recorded separately](validation/m3-02-readback-receipt-collision.md); it adds no execution authority or #103 acceptance.

The [M3-02 contract](migration/M3-02-OPERATION-JOURNAL.md) defines the storage adapter accepted in PR #56. This profile records operations without executing them. FilmCraft execution and readback require separate evidence. The PR #28 G2 review remains open.

| ID | Acceptance | Implementation | Regression evidence |
| --- | --- | --- | --- |
| M3-JL01 | Store the exact operation, request key, project, review-binding digest and artifact digest. An exact replay has no additional effect. Reject changes to identity. | `crates/storytelling-journal/src/lib.rs` | `exact_review_and_artifact_identity_survive_restart_and_idempotent_prepare`, `reused_idempotency_key_with_changed_review_or_artifact_is_rejected` |
| M3-JL02 | Store attempt identity and dispatching state before any host call. On restart, recover a committed dispatch marker as unknown. | `crates/storytelling-journal/src/lib.rs` | `crash_after_dispatch_marker_recovers_unknown_and_never_retries_without_readback`, `ambiguous_readback_sharing_response_digest_is_recorded_once_and_survives_restart` |
| M3-JL03 | Monotonic fence epochs and attempt numbers reject stale owners and late old responses/readbacks | `crates/storytelling-journal/src/lib.rs` | `crash_after_dispatch_marker_recovers_unknown_and_never_retries_without_readback`, `stale_owner_and_late_attempt_response_cannot_overwrite_newer_state` |
| M3-JL04 | Block retry for unknown, reported or partial outcomes. Require explicit NotApplied readback for the current attempt. Block retry if newer readback conflicts. | `crates/storytelling-journal/src/lib.rs` | `unknown_response_and_applied_response_never_enable_a_retry`, `crash_after_dispatch_marker_recovers_unknown_and_never_retries_without_readback`, `a_fresh_conflicting_readback_before_retry_keeps_the_attempt_blocked` |
| M3-JL05 | Keep a busy write atomic. Permit retry after lock release. Reject unrelated SQLite schemas before changing the schema or journal mode. | `crates/storytelling-journal/src/lib.rs` | `busy_write_leaves_no_partial_record_and_succeeds_after_lock_release`, `foreign_database_is_rejected_without_changing_its_schema_or_journal_mode` |
| M3-JL06 | Check identifiers, lowercase digests, timestamps and lease duration. Limit attempts, readbacks and events per operation. Keep other operations available. | `crates/storytelling-journal/src/lib.rs` | `invalid_identity_hash_and_time_limits_are_rejected_at_the_boundary`, `attempt_count_limit_is_per_operation_and_does_not_block_other_operations`, `repeated_ambiguous_readbacks_have_a_per_attempt_bound`, `first_ambiguous_readback_sharing_response_digest_consumes_readback_budget` |

Before implementation, three tests panicked at the missing journal operation. They passed after implementation. Later regressions covered late readback, bounds on ambiguous readback, conflicting readback and unrelated databases. Each test failed before its specific safety check and passed afterward. All six jobs passed on PR #56 and on main after merge. The tests invoked no host or editor. See the [validation record](validation/m3-02-operation-journal.md).

The caller supplies executor IDs and timestamps. Journal fencing cannot stop a host call that has already started. The journal trusts a `NotApplied` observation. Before retry, a production host adapter must check that the previous request is no longer in flight. This requirement remains open after M3-03. The journal makes no guarantee of exactly-once effects, trusted approval or key authorization.

<a id="m3-03-isolated-filmcraft-execution"></a>

## M3-03 isolated FilmCraft execution proof

The [M3-03 profile](migration/M3-03-ISOLATED-FILMCRAFT.md) and [work index](migration/m3-03-filmcraft-work-index.json) cover a test of host execution and readback. The product executor command, trusted approval, retry policy and cross-platform FilmCraft support remain separate requirements.

| ID | Acceptance | Implementation | Regression evidence |
| --- | --- | --- | --- |
| M3-XE01 | Regenerate and freshness-check the selected proposal, snapshot, context and exact OTIO bytes before a synthetic host operation | Pinned integration oracle | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`, plus machine evidence |
| M3-XE02 | Store project-creation and import identity, owner, fence and attempt marker before each host mutation. Recover the final states after reopening SQLite. | Test-only adapter calls M3-02 journal API | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`, `exact_review_and_artifact_identity_survive_restart_and_idempotent_prepare` |
| M3-XE03 | Persist an unknown state after losing a successful import response. Match native clip and source ranges at exactly 25 fps before recording applied-observed. | FilmCraft v0.18.0 Linux oracle | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`, plus [evidence](../tests/oracles/m3-03-filmcraft-v1/evidence.json) |
| M3-XE04 | Prevent duplicate apply or Undo operations. Reject a late response that would overwrite terminal readback. | Journal fence plus host proof | `isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback`, `stale_owner_and_late_attempt_response_cannot_overwrite_newer_state` |
| M3-XE05 | Write new files under a fresh run root. Open no existing project or source catalog. | Oracle preflight and isolated FilmCraft data directory | [oracle README](../tests/oracles/m3-03-filmcraft-v1/README.md), machine evidence |
| M3-XE06 | Block another attempt after an ambiguous outcome. Run existing journal failure regressions across the workspace CI platforms. | Accepted M3-02 state machine | `unknown_response_and_applied_response_never_enable_a_retry`, `repeated_ambiguous_readbacks_have_a_per_attempt_bound` |

The test injects a lost response only after the FilmCraft subprocess exits successfully. A hung host and safe retry after NotApplied require separate tests. The actual host test runs on Linux. Windows and macOS run journal regressions and compile the ignored integration test. The test records approval_granted=false and executable=false. Review freshness does not grant human approval. The production executor and the PR #28 G2 gate remain open. See the [validation record](validation/m3-03-filmcraft-execution.md).

## M3-04 proposed execution contract (G2 pending)

These are proposed requirements, not implemented test results. [ADR 0011](adr/0011-production-execution-contract-v1.md) and the [design/case/work-package record](validation/m3-04-execution-contract.md) trace the [synthetic case inventory](../tests/fixtures/execution-contract-v1/cases.json). Design-vector checks do not satisfy production contract testing or human acceptance.

| ID | Proposed requirement | Designed cases (not passing runtime tests) |
| --- | --- | --- |
| M3-EX01 | Immutable P2-derived identity | EX01, EX03, EX27 |
| M3-EX02 | Current local approval, expiry/revocation and grant consumption | EX02, EX06–EX10 |
| M3-EX03 | Observed host/target/source binding; explicit prior provisioning | EX04–EX05, EX29, EX37 |
| M3-EX04 | Project and operation fences, idempotency, atomic persistence | EX11–EX13, EX21–EX22 |
| M3-EX05 | Unknown quarantine, quiescence and no automatic retry | EX14–EX16, EX20, EX23 |
| M3-EX06 | Complete correlated readback and final quiescence | EX17–EX19, EX28 |
| M3-EX07 | Strict bounded wire and deterministic diagnostics | EX24–EX26, EX30–EX36 |
| M3-EX08 | Existing-format/API/database compatibility; no implicit host calls | All cases and existing regressions |

<a name="敘事功能與後續需求"></a>

## Remaining breadth and future requirements

<a name="跨媒體敘事需求"></a>

### Proposed cross-media narrative requirements

The [product rationale and author scenarios](WHY_STORYTELLING.md) identify the decisions that authors make and the people who can assess the results.
Use the [N-track tasks](../TASKS.md#cross-media-narrative-backlog) for the implementation scope and the SDLC gate for each requirement.

Record the original [skill](../skills/storytelling/SKILL.md) request and the output that the author received.
For a deterministic result, record the local command, fixture, output and exit status.
For an author or recipient outcome, record what the person did.
Keep packaging results, example outputs and pilot observations in separate evidence records.
Each implementation PR must identify its fixtures, tests and evidence.

| ID / task | Author or maintainer requirement | Evidence at the applicable SDLC gate |
| --- | --- | --- |
| N-R01 / N-01 | The author identifies the audience, the message and the details they supplied. | Use the [N-01 author study packet](narrative/N-01-AUTHOR-STUDY.md) prepared against main commit `260485b2ff5b8d7b46215004e21a5bb1a37e6dfb`. It contains synthetic notes S01–S04, an oral outline and two contrasting compositions. Only composition A uses C01. The packet includes unsupported commercial claim U01, a matched identity revision, four measures and decision thresholds set before the study. Five authors compared notes with the Skill. See the [N-R01 results and G0–G1 Continue decision](https://github.com/bynanci/storytelling/issues/48#issuecomment-6798756961). Supplemental [audience and handoff protocol](../examples/evaluation/README.md) and blank observation forms. |
| N-R02 / N-02 | The author can describe material and its relationships without media files or clocks. The narrative document has its own version. | See [accepted ADR 0005](adr/0005-independent-narrative-document-v1.md), the [NarrativeDocumentV1 profile](narrative/NARRATIVE_DOCUMENT_V1.md), the [Draft 2020-12 schema](../contracts/narrative-document-v1.schema.json) and the [paired synthetic fixtures](../tests/fixtures/narrative-v1/README.md). The valid case shares source units between an oral outline and an illustration. The profile permits two-way associations. It defines how to handle precedence cycles, missing or stale references, duplicate keys, unknown fields or versions, and size limits. The compatibility inventory covers existing video JSON, the CLI, finite snapshot import, OTIO and digest regressions. CI passed on the reviewed commit for PR #71. The team resolved a P2 cycle-witness finding. The latest automated review had no open findings. The maintainer authorized N-03 in issue #50. |
| N-R03 / N-03 | The author can use one source for a spoken outline and two distinct spatial compositions. | Run `storytelling render-narrative <FILE\|->` with [n03-authoring-v1.json](../tests/fixtures/narrative-v1/n03-authoring-v1.json) under the accepted `NarrativeDocumentV1` profile. The [validation record](validation/n03-narrative-cli.md) contains the skill request and three outputs. The ordered oral beats, wide illustration brief and close illustration brief preserve source IDs and origins. Only the wide brief uses C01. CLI tests cover execution without media, identical bytes across repeated file and stdin runs, and unchanged input and temporary files. A dangling reference produces a JSON diagnostic and exit code 1. Engine tests cover the paired N-02 fixtures, references, order, provenance, and byte and depth limits. The record documents the failing test before implementation, local video V1 debug and release tests, and official OTIO tests. Hosted CI for the same commit and author inspection have separate evidence records. The tests supply no human outcome. Requester inspection, the corrective document revision 2 and its close-only C02 proposal are recorded in the same validation record; comparative participant outcomes remain unobserved. |
| N-R04 / N-04 | The author can keep alternative variants and see which ones a revision affects. | See [NarrativeRevisionLedgerV1](narrative/NARRATIVE_REVISION_LEDGER_V1.md), its [schema](../contracts/narrative-revision-ledger-v1.schema.json), [fixture](../tests/fixtures/narrative-v1/n04-revision-history-v1.json) and [G3–G4 validation](validation/n04-narrative-revisions.md). The CLI reports that S02@1→@2 affects the oral outline and both illustrations. After author review, C01@1→@2 affects only illustration B. The ledger retains earlier source and variant snapshots and declared choices. A suggestion event cannot carry decision fields. The CLI cannot authenticate who supplied an `author_decision`. Impact follows direct references only. Compare the expected and actual affected IDs. Record omissions and unrelated rewrites. PR #76 merged at `9eb12eefe41ef965acee1f5e78e373e3b782372c`; same-head and post-merge CI run #218 passed all configured checks. Human inspection remains pending. |
| N-R05 / N-05 | The author can assemble a commercial outline and inspect the evidence behind its wording. | `audit-copy-claims --report-v2` reads fictional product copy and an N-04 ledger. Both the scoped 10→8-minute claim and overbroad all-teams 20% claim cite PILOT_LOG@1 and report current references with human review still required. CLI golden tests cover current, missing, dangling and revised sources; revision attention identifies exactly the two dependent claims while retaining original wording, historical/current source text and unaffected proposals. Preserve strict JSON, replay/read-only checks, video and OTIO regressions. [Contract](narrative/COMMERCIAL_COPY_V1.md) · [N-R05 test map and actual output](validation/n05-commercial-copy.md). |
| N-R06 / N-06 | The contributor can identify which additional profile warrants development. | The [format-discovery packet](validation/n06-format-discovery.md) compares three candidates, current contract compatibility and smallest viable change. The requester explicitly deferred all three in the [2026-11-11 decision](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714); a later linked [2026-11-12 decision](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432) selects video coverage for N-10 only. No task trial or timing is claimed. |
| N-R07 / N-07 | The maintainer can judge whether the native workflow merits continuing, narrowing or stopping. | [N-07 disposition](validation/n07-pilot.md) records the actual Stop / no-release decision for five submitted external-workflow outputs: 0/5 primary completions and 5/5 with major U01 errors. Frozen ce73836 / ReportV1 native acceptance is unverified; its result remains 待真人證據. Preserve the separate preparation snapshot, raw-measure protections and existing helper/packet tests. Future evaluation and G5 publication are separate. |

N-R07 preparation follow-up: [native v3](validation/n07-native-v3.md) adds executable provenance, pair eligibility and private record instructions. Existing helper/packet tests validate engineering preparation only; human acceptance remains 待真人證據. [N07-REC01 validation](validation/n07-recorder.md) maps private recorder concurrency, timeout, raw-byte, recovery and native-entry tests; declared agreements and attachments do not authenticate or score humans.

These evidence requests retain the accepted N-R02 wire contract and the N-01 comparison protocol.

The [research design map](research/README.md) traces P01–P11/S01 to ST-R01–ST-R14, W1–W6 and the existing N requirements. These are editorial guidance and evaluation questions, not new V1 fields or additional accepted requirements.

<a name="skill-散布"></a>

## Skill distribution

**SK-R01 / SK-D01.** A consumer can install Storytelling from GitHub with the Skills CLI.
A consumer can also install the npm dependency and activate the bundled skill.
Both routes must deliver the same instructions, references, helpers and license.
The [distribution test](../scripts/test_skill_distribution.py) creates an npm tarball and installs it in a clean consumer project.
It compares the installed files with the source.
See the [platform guide](PLATFORMS.md) for activation, checks and updates.

<a name="驗證能證明什麼"></a>

## Evidence levels

Use the level that the evidence supports. M0, M1-01 and the bounded M2-01 audit cover only levels 1 and 2.

| Level | Evidence | Question it answers |
| --- | --- | --- |
| 1 | Structural parsing | Can the parser read the document? |
| 2 | Declared semantic validation | Do the supplied values and relationships satisfy the contract? |
| 3 | Source verification | Do the actual source bytes match the expected identity? |
| 4 | Host readback or render verification | What did the host import or render? |
| 5 | Human editorial acceptance | Did the author or reviewer accept the result? |

<a name="多模態需求"></a>

## Multimodal authoring and follow-up requirements

Use the bundled [templates](../skills/storytelling/references/multimodal-templates.md) and [hands-on demo](../skills/storytelling/references/workshop-demo.md) with the current skill.
Each proposed Rust or adapter feature requires its own acceptance evidence.

| Requirement / task | Observable result and evidence |
| --- | --- |
| N-R08 / [N-08](https://github.com/bynanci/storytelling/issues/65) | Compare two openings using one synthetic source. The revision-bound `NarrativeStyleProfileV1` CLI checks declared spoken-variant beat order. Valid rules pass; a same-variant cycle reports rule IDs; order violations, stale revisions and unknown versions return stable diagnostics. Same-input replay and paired fixtures are documented in [N-08 validation](validation/n08-style-profile.md). Two official-sheet readings satisfy the qualitative gate on maintainer attestation; both were A→B with no recorded randomization method. Private verbatim answers and devices were not independently audited. The validation record preserves unexpected supplementary responses and the source/key-disclosure limits; the checker makes no aesthetic or approval claim. |
| N-R09 / [N-09](https://github.com/bynanci/storytelling/issues/66) | `render-cue-sheet` renders a separate closed cue sheet bound to a spoken `NarrativeDocumentV1` revision. Preserve the workshop audience, Saturday event and bring-one-umbrella source; mark the narration as pending recording, the pause as estimated and `taps.wav` as a declared effect. Accept cross-track overlap and touching half-open ranges; reject exclusive-track conflicts, reversed intervals, duplicate IDs and unknown versions. Save outputs and exit states, prove byte-identical replay, and keep `NarrativeDocumentV1` unchanged. See the [profile](narrative/NARRATIVE_CUE_SHEET_V1.md) and [N-09 validation](validation/n09-cue-sheet.md). |
| N-R10 / [N-10](https://github.com/bynanci/storytelling/issues/67) | The requester selected video-only, read-only `NarrativeVideoCoverageV1`: choose two story units, declare footage for one and none for the other, and preserve the gap as unresolved while blocking ready-for-planning/export status. [ADR 0010, schemas and paired fixtures](validation/n10-resume-gate.md) were accepted at G2 in PR #93. The bounded validator and `render-video-coverage` CLI merged in PR #97; all six exact-head hosted jobs passed in [run #376](https://github.com/bynanci/storytelling/actions/runs/43148901399). N10-05 is complete within its bounded evidence scope: Codex replayed the supplied JSON; the maintainer supplied one post-merge author self-session (145 seconds, with prior exposure and no randomization or independent observer); Codex probed the supplied synthetic MP4. The session is qualitative, not an independent user study. The execution record preserves the verbatim response, usability friction and limits. V1 does not read media bytes or verify catalog claims; G5 publication remains separate. |
| N-R10-06A / [N10-06A #129](https://github.com/bynanci/storytelling/issues/129) | Authors enter exact decimal seconds or explicit constant-rate frame boundaries and obtain canonical reduced u64 rational ranges for unchanged V1 validation. Draft-only G2 for [ADR 0012](adr/0012-draft-exact-video-range-input.md) and [acceptance vectors](../tests/fixtures/n10-06a-draft/README.md) was accepted at candidate `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf`; see the [exact requester decision](validation/n10-06a-g2-decision.md). Focused A G3 and automated G4 are verified at implementation head 59516f2; current-head review/CI remains required, with [RED/GREEN and local validation receipts](validation/n10-06a-exact-input.md); final-head CI is tracked on PR #130. Existing V1/21 diagnostics retain independent accepted status. Stable SDK (#107), author/media/host acceptance, B/C/D, release remain separately gated; engineering merge is authorized after current-head checks/review. [Gate record](validation/n10-06a-gates.md). |
| N-R11 / [N-11](https://github.com/bynanci/storytelling/issues/68) | `render-handoff` maps text, image and audio components through the unchanged N-04 ledger. The [profile](narrative/NARRATIVE_HANDOFF_MANIFEST_V1.md), [fixtures](../tests/fixtures/narrative-handoff-v1/README.md) and [evidence](validation/n11-handoff.md) cover the predeclared date-impact set, missing/stale references, variant-only proposal leakage, complete mapping, supplied descriptions, unchanged sources and deterministic replay. Human semantic review is separate. |
| N-R12 / [N-12](https://github.com/bynanci/storytelling/issues/69) | The [Inkscape feasibility trial](validation/n12-inkscape-png.md) uses the same request and S2 for a real PNG with full decode/visual reopening and a controlled missing-entry-point composition brief. Keep source identities, commands and exits. Production-adapter adoption remains a maintainer decision. |

## Gemini R1 skill evaluation record (SK-EVAL-R1)

The [40-case model-judged report and visual comparison](research/evaluations/2026-11-13-gemini-r1/README.md) preserve numeric observations, source-packet errata and unverified declared assignments. This is W4/W6 diagnostic evidence, not a newly accepted requirement, human result or implemented skill improvement.
Use [#115](https://github.com/bynanci/storytelling/issues/115) as the umbrella. **SK-E01 / #118** maps W6 / ST-R12–ST-R13 to the [versioned offline record, blind/allocation schemas and fixtures](research/evaluations/sk-e01/README.md). **SK-E02 / #119** maps W1/W4/W5 and ST-R01/ST-R02/ST-R09/ST-R10 to the [evaluation-only evidence-plan, adjudication and errata contract](research/evaluations/sk-e02/README.md), checked by `scripts/test_sk_e02_contract.py`; it freezes evidence mode, pre-generation claims, W4 states, separate scope/source mismatch annotations and C13/C28 errata without changing R1. **SK-E03 / #120** maps W1/W4/W5 and ST-R01/ST-R02/ST-R05/ST-R09/ST-R10 to four bilingual synthetic instruction-conformance cases and captured development outputs; the serving revision was unavailable and the rehearsal is not an efficacy result. **SK-E04 / #121** maps W6 / ST-R12–ST-R13 to the [order-only protocol, anchored rubric and descriptive paired analysis](research/evaluations/sk-e04/README.md), checked by `scripts/test_sk_e04_order_analysis.py`; original R1 rerun remains blocked by missing exact inputs and judge settings. These evaluation tools add no Rust/CLI schema and do not rewrite historical scores. [SK-E05 / #122](https://github.com/bynanci/storytelling/issues/122) remains a separate prospective holdout task. Existing N/M acceptance, #54's disposition and #112's human-evidence gates retain their separate scope.

**SK-E04 / #121 input boundary** traces to real API/CLI regressions in `scripts/test_sk_e04_order_analysis.py` and the [validation record](validation/sk-e04-input-boundary.md): nested duplicate keys, non-finite constants, malformed UTF-8/discriminant types, and record-wide artifact ID/digest consistency. Same-ID/same-digest reuse and byte-identical successful v1/v2 reports remain supported. This validates declarations, not evidence-store bytes or study eligibility; #121/#122 retain their gates.

**SK-E06 / #115** maps W1/W3/W4/W5/W6 and ST-R01/ST-R02/ST-R03/ST-R07–ST-R14 to five [adversarial reference cases and blank manual forms](research/evaluations/sk-e06/README.md). It checks fixture identity, bilingual material, source revisions, reference labels and literal assertions through `scripts/test_sk_e06_packet.py`; these are packet checks, not semantic conformance or efficacy evidence. No new Rust/CLI requirement or accepted wire format is added. Actual case execution and #121/#122 remain unfulfilled.

## Proposed read-only consumer facade (M4-R01)

| Requirement / task | Proposed requirement and evidence state |
| --- | --- |
| M4-R01 / [M4-01 #107](https://github.com/bynanci/storytelling/issues/107) | An independent consumer can use one documented read-only Rust facade for validation, narrative/revision impact, video coverage, planning and selected-proposal review without private-module imports or CLI prose parsing. [ADR 0013](adr/0013-read-only-consumer-facade-v1.md) proposes eight delegating functions, curated existing result types, preserved source error details, exact per-operation byte/version/digest behavior and unavailable mutation/recovery. The [golden plan](proposals/m4-01-readonly-sdk/test-plan.md) names existing CLI/engine fixtures and future test-first parity/opacity/concurrency checks. Status: G2 proposed; no API freeze, facade implementation, RED/GREEN, same-head SDK parity, consumer or host acceptance. #107 remains Open; #103–#106 gate execution and #108 owns transport selection. |
