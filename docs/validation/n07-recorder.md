<!-- SPDX-License-Identifier: MIT -->
# N07-REC01 / N-R07 · Private recorder engineering evidence

Base: `fe2b145250b42564bf5b4d781ced5a95e8a8f9bf`. Scope: SDLC G2 instrumentation design, G3 implementation and G4 regression; no new trial adoption or G5 release. [Facilitator guide](../../examples/evaluation/n07-recorder/README.md) · [繁體中文](../../examples/evaluation/n07-recorder/README.zh-TW.md).

## Decision and boundaries

The previous workflow could not bind the submitted output to the native executable. The v3 protocol supplies the prospective rules; this separate Python recorder supplies durable local events and byte-preserving capture. Core, engine, CLI, ReportV1 and the frozen protocol/materials remain unchanged. Python invokes only the two native read-only author commands. SQLite is a private recorder file, not a new SDK crate or a FilmCraft operation journal.

One round has an immutable rehearsal/trial mode, a fixed allocation, declared build receipt and exact recorder/protocol hashes. Trial mode requires a prospective agreement document; parsing it is not signer authentication. Material copies are checked against the historical receipt. An event commits before a process starts; a terminal session cannot be overwritten by a later call result. Recovery closes an interrupted session with unknown time and never retries. Output/answer bytes are preserved, including unexpected answers and partial output. These engineering properties do not certify human participation, source build provenance or editorial correctness.

The timer runs in one live process, includes capture overhead and native waiting, caps timeout analysis at 480 seconds, and refuses to reconstruct elapsed time after interruption. It cannot freeze an external editor at the deadline. Facilitator preservation of timeout artifacts and all scoring/feedback remain human evidence requirements. The export deliberately leaves comparable-pair count and maintainer decision unknown; attachments are unassessed. Protocol/recorder adoption must be recorded before recruiting fresh authors.

## Observed RED and GREEN

- Before record.py existed, the actual `init` CLI test expected exit 0 and observed exit 2 (script absent). Initial suite: 13 tests, one assertion failure, 11 import errors and one native-integration skip before build. This is an observed missing-entry-point RED, not a fabricated earlier runtime defect.
- After the basic recorder implementation: 13 tests passed except the explicitly skipped native integration before building the debug binary.
- Added the matched-workspace acceptance test before implementation: 20 tests, one error (`workspace` absent), no assertion weakening. Implemented receipt-checked material copying.
- After `cargo build --workspace --locked`: 20 recorder tests passed with no skips, including the actual recorder CLI → native `audit-copy-claims` entry. The synthetic rehearsal still returns U01 `missing` with empty evidence and leaves the author's input bytes unchanged.

Self-review added a strict-JSON negative case before the fix: `1e9999` was accepted as a non-finite float (one observed failing assertion). The parser now rejects overflowed numbers and non-UTF-8 control JSON; the unchanged assertions pass.

Two additional tests verify that agreement declarations bind the exact recorder hash without becoming authentication, and that the live timer expires while waiting for input. The final local suite has **22 passing tests, no skips**.

## Acceptance mapping

| Requirement | Regression evidence |
| --- | --- |
| Preserve 3 AB / 2 BA draw and refuse overwrite | `test_cli_rehearsal_init_and_nonoverwrite` |
| Trial requires agreement; strict receipt and binary match | `test_trial_requires_real_agreement_record`, `test_agreement_binds_exact_recorder_and_preserves_declaration_not_authentication`, `test_receipt_duplicate_unknown_and_mismatched_binary_rejected` |
| Exposed candidates retained but not started | `test_exposure_preserved_and_start_refused` |
| One active arm; no duplicate or competing starts | `test_concurrent_start_and_duplicate_arm_refused`, `test_two_simultaneous_starts_have_only_one_winner` |
| Timeout is incomplete with analysis time 480 | `test_deadline_cannot_be_reported_as_submission`, `test_live_timer_expires_while_waiting_for_input` |
| Interruptions stay unknown; no silent retries/late overwrite | `test_recovery_is_unknown_and_late_end_cannot_overwrite`, `test_intent_is_durable_and_late_capture_does_not_end_new_session`, `test_crash_preserves_unknown_call_and_exported_partial_without_retry`, `test_eof_records_interruption_not_completion` |
| Retain withdrawals and replacement lineage | `test_replacement_retains_prior_start` |
| Preserve exact bytes, corruption detection and input/output bounds | `test_verbatim_attachment_export_and_overwrite_refusal`, `test_recorded_hash_detects_damaged_blob`, `test_bounded_file_read_and_strict_json`, `test_capture_preserves_binary_streams_and_timeout_partial`, `test_output_limit_is_explicit_and_bounded` |
| No notes-arm CLI or unsupported process command | `test_disallowed_command_and_notes_arm_never_launch` |
| Same starting words, no scoring key, no overwritten workspace | `test_workspace_copies_matched_words_without_key_and_never_overwrites` |
| Real recorder/native entry preserves ReportV1 | `test_real_cli_entry_preserves_raw_native_report` |

The test receipt intentionally binds a local test executable with synthetic source declarations. It is marked rehearsal and cannot prove that build came from ce73836. The separate frozen-binary rehearsal is recorded below. No real author or reader is generated by the tests.

## Verification and residual work

Run the recorder tests after building the workspace. CI runs them on Linux, Windows and macOS; full workspace debug/release tests, fmt, Clippy, rustdoc, repository checks and dependency audit remain required. The PR records the exact tested head and hosted outcomes. No test interprets a submitted draft as a completed human task.

Self-review, not independent human review: keep private data outside the repository; distinguish captured output from source attestation; retain unknown interrupted outcomes; enforce pre-launch intent and single active session; preserve matched inputs and the unchanged v3 protocol digest. The maintainer still needs to adopt the instrumentation and obtain five eligible paired records before applying thresholds. Actual anonymity, consent, freshness, scoring, deviations and decision remain unverified. #54 stays stopped and #65's qualified closure is unaffected.

Rollback: stop the recorder, retain the original private directory and an exclusive export, then return to the manual v3 records in a newly agreed round. Never delete existing trial events or silently switch capture methods mid-round. No data migration or publication is performed by this change.

## Frozen-binary synthetic rehearsal

The [sanitized replay receipt](n07-recorder-rehearsal.json) records the actual recorder CLI creating a rehearsal, copying matched A/B workspaces, running render and audit against the ce73836 preflight executable, submitting both synthetic arms and exporting the bundle. Both native calls returned exit 0, binary/input/snapshot hashes stayed unchanged, and every exported blob matched its name. The raw ReportV1 unsupported claim remained missing with empty evidence. The initial uncorrected drafts and blank answers were intentionally submitted; their completion scores remain unknown. No human author, pair, elapsed author-performance claim or feedback is inferred. Private database, full argv and paths are not published.

Local workspace debug/release tests each passed 302 tests with three existing ignored tests. Existing 12 pilot helper/packet tests, fmt, Clippy, rustdoc, repository and skill-distribution checks passed. The final Python portability and strict-JSON adjustments were followed by all 22 recorder tests and a new frozen-binary rehearsal; hosted CI verifies the submitted head on all three systems.

## Windows checkout correction

The first [PR CI run](https://github.com/bynanci/storytelling/actions/runs/43129282130) passed Linux/macOS but Windows correctly rejected a different protocol digest: Git autocrlf had rewritten the frozen Markdown bytes. Added narrow `.gitattributes` rules for the hashed protocol, material receipt, participant kit and recorder Python files. A local `core.autocrlf=true` checkout replay preserved all 14 hashed files byte-for-byte. The strict hash assertions remain unchanged; final Windows CI must pass on the corrected head.

The next Windows run passed the recorder operations but caught an open SQLite connection in the corruption test cleanup (`WinError 32`). The test now explicitly closes its injected-corruption connection; no assertions or production recovery behavior were changed. The production transaction context already closes connections in `finally`.
