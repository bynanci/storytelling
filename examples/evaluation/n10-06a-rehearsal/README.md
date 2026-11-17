# N10-06A synthetic rehearsal and future author handoff

[繁體中文](README.zh-TW.md) · [Eight-case acceptance](ACCEPTANCE.md) · [Roundtable report](../../../docs/validation/n10-06a-roundtable-2026-11-17.md)

The requester chose a simulated roundtable on 2026-11-17. This package captures actual native CLI outputs for eight synthetic cases and supplies a separate **proposed** observation handoff. Human participant count is zero. The four roundtable perspectives are authored by one AI agent; they are not independent people, recorded author responses, G2 approvals or effectiveness data. It does not replace #121's missing original study inputs or create a #122 holdout.

Build the accepted feature-enabled CLI from a frozen source in its own new target, using Rust 1.89.0. From the repository root (choose an appropriate private directory on your platform):

```sh
cargo build --release --locked -p storytelling-cli --features draft-video-input --target-dir /tmp/n10-a-unique-target
python3 examples/evaluation/n10-06a-rehearsal/test_replay.py /tmp/n10-a-unique-target/release/storytelling
python3 examples/evaluation/n10-06a-rehearsal/replay.py --cli /tmp/n10-a-unique-target/release/storytelling --source-commit 06164f19a8b3614f438dd2f97ef810c94e64c3d3 --output /tmp/n10-machine-receipt-new.json
```

Windows uses the actual `storytelling.exe` path and Python command. The Python checks exercise the same normal CLI entry on each CI OS; the shell setup above is not an observed Windows author session. Set `--source-commit` to the actual build source; the tool records this declaration but does not authenticate it. A separate build receipt must connect source, toolchain, features and executable hash. Never reuse a Cargo target between source worktrees.

The replay uses only the three existing public draft fixtures. It generates eight inputs in temporary directories, captures exact input text, raw stdout/stderr as base64, hashes, exit, arguments and process elapsed time. Each case checks file/stdin/repeat equivalence; five valid cases emit V1 and replay `render-video-coverage`; three invalid cases also reject emit. All 37 calls are read-only. CLI failure/timeout remains failed machine evidence. Existing receipt paths are refused. Fixture/binary hashes are checked before/after; these checks are not a hostile-executable sandbox. Process times are not author times or a new benchmark. Repeated tool calls are not additional participants.

Four tests cover actual entry/output provenance, refusal to overwrite, retention of a failed wrong-executable call, and invalid source-metadata rejection. A test-first missing-entry failure and passing receipts are preserved in the report. Without an explicit feature-enabled binary, discovery skips the native integration method; CI invokes the file with the binary explicitly and must pass all four without skips.

## Future human observation: proposed, not executed

Use this only after the facilitator and maintainer agree to the actual observation, screening, privacy and materials. It is a noncomparative qualitative feasibility check, distinct from N07 and the blocked #112 rough-cut beta. The participant is told the tool is read-only; comprehension is assessed after that briefing, not as unprompted discovery. Fixed seconds→frames→range-correction order does not measure comparative speed or isolate learning effects.

1. Recruit one actual author. Record prior exposure, relationship to the project, CLI/JSON experience, date and separate permission for private capture, anonymous summary and publication of quotations. A maintainer self-session remains a self-session. Do not infer independence from an anonymous code.
2. Freeze exact code/binary/material hashes before starting. Copy only [the author sheet](author-sheet.zh-TW.md), [seconds starter](seconds-start.draft.json) and [frames starter](frames-start.draft.json) into private storage outside Git; keep this README, the roundtable and reviewer key with the facilitator. Starter time fields are blank, not pre-solved. Record installation/setup time separately. Prepare the actual native CLI before author timing.
3. Start a continuous timer when the author receives the task. Include reading, editing, retries, help, CLI waiting and answers. Keep every input revision, command, exit and raw output under a new private filename. Record process time separately when available. At 600 seconds stop and preserve partial work as incomplete; record withdrawals, breaks or interrupted/unknown timing without inventing a duration.
4. Save all four answers verbatim before discussing the facilitator key. Preserve unexpected answers, confusion and assistance. Use [the blank private record](observation.blank.json) to retain original task, starting data, author's exact words, observed behavior and limitations/failure log separately. Blank means unknown, never success or zero errors.
5. Review artifacts against the key, then obtain author confirmation of an anonymous summary. Permission to participate does not automatically permit public quotations. Do not publish private paths/media/identities or a completed private record. Publish only consented, reviewed synthetic reproductions or an anonymous summary.

Facilitator key (never supply before answers): S02@1 is mapped and S03@1 remains unresolved; 2.5–4.5 seconds and [75,135) at 30/1 both normalize to [5/2,9/2); 6 seconds exceeds declared duration 5 with `video_coverage.invalid_range` at `$.mappings[0].source_range.end`; readiness is about declarations and proves no file bytes, visual suitability or host action.

Predeclared observation decisions: continue to another qualitative observation only if the author can complete the three tasks and explain the unresolved/declaration boundary with recorded assistance. Narrow the next design if conversion succeeds but the author cannot identify the missing unit or infers media/host verification; retain every counterexample. Stop the session at withdrawal, unknown timing, sensitive-data exposure or 600 seconds, and preserve partial evidence. One observation can identify feasibility or a failure, not time savings or a population success rate. A maintainer decides any actual continue/narrow/stop outcome after reviewing real evidence. #129 remains Open for independent author evidence and B/C/D; technical G2 and G5 gates are separate.

A failed capture retains only actually attempted rows in `case_count`, with the fixed protocol size in `planned_case_count`. Neither counter is a completion or participant count. The test for a wrong executable starts one case and must report one attempted case rather than eight. The report preserves the original capture and this instrument-count correction separately.
