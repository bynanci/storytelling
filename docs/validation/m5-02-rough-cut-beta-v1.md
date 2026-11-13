<!-- SPDX-License-Identifier: MIT -->
# M5-02 · Prospective rough-cut beta protocol v1

[繁體中文](m5-02-rough-cut-beta-v1.zh-TW.md) · Issue [#112](https://github.com/bynanci/storytelling/issues/112) · Related work [#109](https://github.com/bynanci/storytelling/issues/109) and [#111](https://github.com/bynanci/storytelling/issues/111).

**Status: design proposal only. Not yet accepted, frozen, recruited or started.** Issue #112 remains blocked on the reference app (#109), the selected second-editor profile (#111), and an accepted M3 execution/recovery path. This document does not report a beta result or close the issue. The maintainer must accept the protocol and record its version, hashes and start authorization before any measured session.

## 1. Study question and prior evidence

**Question.** Can five independent creators use the Storytelling workflow to revise and hand off a 60–90 second rough cut while preserving source trace, protected shots, current human approval and safe recovery? How do completion, errors, effort and perceived control compare with direct use of the same editor?

PR #102 supplies useful design evidence, but not a #112 baseline:

| PR #102 observation | What it supports | What it does not establish |
| --- | --- | --- |
| Three first readers of the court notice all identified the website as the new-booking channel. | Readers can be scored against a specific source-backed decision. | A five-person rough-cut result or comparative editing benefit. |
| Two court handoffs took 72 and 55 seconds on different edit cards. | Record task and change-card identity with each duration. | A timing comparison; the tasks were different and had no control arm. |
| One kiln reader distinguished the production log from footage actually delivered. | Test whether an event record is kept distinct from available media. | General performance across readers or editors. |
| Two kiln edit cards took 68 and 52 seconds; expected drafts were updated and protected drafts preserved. | Score missed, extra and collateral edits against an answer key. | Time savings; the edit cards were different and there was no same-task control. |

These observations support testing source-to-media traceability and handoff boundaries. They do not count toward #112's five participants, do not provide its control condition, and do not justify a speed or quality claim. Use new task packs and fresh participants.

The earlier N-01 Skill-versus-notes study and N-07 native-CLI proposal answer different questions. Keep their participants, materials, thresholds and results separate. In particular, do not transfer the N-01 eight-minute threshold or treat the stopped external-wrapper result in #54 as a native rough-cut result.

## 2. Readiness gates

Do not recruit, start a timed session, or describe the beta as underway until every item below is checked and recorded in the tracking issue:

- [ ] #109's reference workflow is usable for the tested path and exposes proposal, review freshness, human approval, execution, receipt/readback and recovery.
- [ ] #111 has selected exactly one editor/host and accepted a versioned profile. Both study conditions will use this same profile, host version, OS and project format.
- [ ] M3 execution and recovery are accepted for an isolated project copy. The test can reproduce an applied operation with its acknowledgement lost and can read the project back before a retry.
- [ ] The Storytelling build, direct-host build, app/adapter, host, OS, SDK, command/configuration and fixture commits are fixed. Record hashes and versions in a new protocol receipt.
- [ ] Two new, rights-cleared 36-item media packs, source cards, 75-second starter projects, expected-answer keys and their hashes are frozen. No participant-private media, paths, voices, GPS or credentials are included in public files.
- [ ] A dry run confirms both packs can be completed under the same task card, the scoring key is unambiguous, and no asset is missing except the deliberately unresolved insert.
- [ ] The five-slot allocation is generated and saved before recruitment. Facilitator, scorer, consent/storage process and participant instructions are ready.
- [ ] The maintainer accepts this protocol revision and the continue/narrow/stop thresholds before the first measured session.

A missing or failed gate blocks the relevant sessions. If a host cannot support the same controlled interruption in both conditions, revise and re-accept the protocol before recruitment; do not simulate different failure modes and call them equivalent.

## 3. Design

### Participants and sample

Recruit five fresh creators or editors who are not maintainers or implementers and have not participated in PR #102, the N-01/N-07 trials, or seen these task packs or answer keys. Record anonymous code, video-editing experience, familiarity with the chosen host, Storytelling exposure and screening outcome. Keep identity/contact and consent records privately, separate from the study data.

Each of five assigned participants completes both conditions on the **same exact pack and starting snapshot**, using a fresh isolated clone in each arm. Each participant is assigned Pack A or Pack B; the pack is balanced across both first-condition groups. This is a five-person, within-participant exploratory comparison, not a powered experiment or representative sample. Repeating the same task creates learning/carryover, so the randomized first-period comparison is primary and second-period paired differences are descriptive only. Keep recruitment, material exposure, starts, withdrawals and timeouts in separate counts. A pre-session cancellation may be replaced only before any task material is revealed or a timed arm starts; reuse that slot's assignment and record the cancellation. Do not replace anyone after material exposure or a measured start. Withdrawals after exposure count as incomplete in the five-slot completion denominator. If fewer than five comparable pairs remain, report missing observations and **待真人證據** for paired conclusions; do not pool another cohort or fill in missing values.

### Conditions

| Arm | What the participant uses |
| --- | --- |
| **S · Storytelling** | The accepted Storytelling reference workflow with the selected host profile. The participant reviews the source-linked proposal, checks freshness, explicitly approves the current proposal, applies it, and verifies the receipt/readback. |
| **D · Direct editor control** | The same host, version, project copy and media pack, operated directly. The participant receives the same source cards and a plain handoff sheet and records the same expected facts, clip IDs and source ranges manually. No Storytelling app or metadata tools. |

No generative agent is used in either arm. This deliberately tests the Storytelling control and provenance workflow against direct editing without adding a model/prompt variable. If the accepted reference app requires an agent for the tested path, freeze the exact same agent, model, settings and prompt for both arms in a new protocol revision before recruitment; do not mix configurations within this round.

Both arms receive the same task wording, source facts, media, protected-shot notice, fault event and time limit. Each runs in a fresh isolated project copy. No arm can write the source catalog or a live FilmCraft/host project.

### Allocation and counterbalancing

Use anonymous slots P01–P05. Randomize three slots to S→D and two to D→S. Assign each participant one pack, which they use in both arms on fresh clones. Constrain pack assignment so Pack A and Pack B both appear in each first-condition group (S-first and D-first), with two or three participants per pack overall. This keeps the exact same task and material within each participant's condition pair, while balancing pack identity across the randomized first-period comparison.

Before recruitment, run the kit's allocate.py once after protocol acceptance. It uses Python random.Random with algorithm label python-random.Random-v1: draw three of five slots for S→D, enumerate pack-A assignments of size two or three that place both packs in both first-condition groups, and draw one valid assignment. D uses the same assigned pack as S. The script uses a secure random seed unless one is supplied, records the Python version, creates its output exclusively, and prints a SHA-256 receipt. Save the allocation file, seed, planned order and hash privately; record the exact Python version. Reveal only the next assignment to the facilitator. After the round, publish the de-identified allocation and seed if disclosure cannot reveal participants. Record actual arm order and every deviation; planned order is not a substitute for actual order. Do not run the allocator as part of this design PR.

## 4. Frozen task-pack specification

Both packs contain **36 cleared items**: 18 photographs, 12 video clips and 6 narration/audio items. The 12 videos must provide 180–240 seconds of usable source footage in each pack; the two totals must be within 5% of one another. The packs use the same aspect ratio and target resolution. Each has five story beats (setting, preparation, action, verified result, next step), one 75-second starter rough cut, two protected video segments, one source fact changed during the task, one unsupported claim, and one optional shot present in the production log but absent from the supplied media. Both protected segments are video clips with exact source ranges. The participant's final cut must be 60–90 seconds.

The names below fix the story logic; actual asset IDs, exact frame/rational ranges, source-card revisions and SHA-256 values must be filled in the private freeze manifest before sessions. The checked-in forms (pack-freeze.csv and freeze-receipts.csv) are deliberately blank; this PR does not pretend the media pack already exists.

| Requirement | Pack A · community garden open day | Pack B · neighborhood book exchange |
| --- | --- | --- |
| Source correction delivered mid-task | Opening time changes from 10:00 to 10:20. It is cited in exactly two starter outputs: one title card and one narration line. | Opening time changes from 13:00 to 13:20. It is cited in exactly two starter outputs: one title card and one narration line. |
| Unsupported statement | “Every plot is pesticide-free.” The source cards do not support this for the whole garden. It must not remain as an unqualified factual claim in the final cut. | “Every donated book was checked for accuracy.” The intake record only confirms count/condition; it does not support a content check. It must not remain as an unqualified factual claim in the final cut. |
| Log-only optional insert | A soil-moisture close-up is requested in the production log, but no such image or clip is supplied. Keep the insert explicitly unresolved in the handoff; do not substitute or invent footage. | A barcode-scan close-up is requested in the production log, but no such image or clip is supplied. Keep the insert explicitly unresolved in the handoff; do not substitute or invent footage. |
| Protected shots | One opening sign/wide video shot and one closing harvest-table video shot, with exact source ranges frozen in the manifest. | One opening exchange-sign video shot and one closing handover-stack video shot, with exact source ranges frozen in the manifest. |

The pack-freeze manifest must identify, for each file and source card: neutral asset ID, type, source revision, media duration when applicable, still-image timeline duration, exact video/audio source range when applicable, rights/source note, protected or editable status, SHA-256, and answer-key reference. The separate freeze receipt records each pack's manifest, starter-project, source-card, answer-key and build hashes. Hash private files locally; only publish the manifest if rights and privacy allow. The public repository must not contain private media or paths.

## 5. Participant task and success standard

Give each participant the following task card, replacing only the pack name and IDs:

> Revise this 75-second starter rough cut into a 60–90 second handoff-ready cut. You may change unprotected shots, order and pacing. Keep both marked protected shots and their exact source ranges unchanged. At the facilitator's revision point, update every output that depends on the new opening time. Do not present the unsupported statement as a fact. The requested close-up exists only in the production log, not in the media bin; leave it unresolved in the handoff and do not invent a replacement. Provide the final cut and a handoff sheet that traces each factual statement to a source ID/revision, each selected video to an asset ID and exact source range, and each selected still to an asset ID and timeline duration. Inspect the version you intend to save. The facilitator may introduce one acknowledgement interruption after the final apply/save; verify the project state before retrying.

The facilitator reveals the updated source card after the participant first says the cut is ready for review. In S, the former review must become stale and the participant must inspect and approve a fresh proposal before apply. In D, the participant must re-check the changed card and manually confirm the current edit before saving. This measures a formal freshness gate against a direct human check without giving either arm a different fact.

After the final apply/save, inject the same scripted “operation applied, acknowledgement lost” event in the isolated copy. The participant must inspect or reopen the host project and read back the result before attempting any retry. Never instruct a blind retry or undo. Record whether the participant recovers, retries, or stops.

**Safe completion** requires all of the following:

1. A final project reopens and is 60–90 seconds long.
2. Both revised time outputs use the new value.
3. The unsupported statement is absent or explicitly qualified; no new unsupported claim is introduced.
4. The missing optional insert remains identified as unresolved and is not replaced with invented media.
5. Both protected video shots retain their exact frozen asset IDs, source ranges, timeline positions and protected properties.
6. The final handoff identifies each video asset and source range, each still and its timeline duration, and cites source revisions for factual claims.
7. The participant inspects the current version before the write and, after the source revision, does not use a stale review/approval.
8. After the lost acknowledgement, the participant reads back the isolated project before retry. No duplicate application occurs.
9. Source media and the source catalog remain byte-for-byte unchanged.

A saved file alone is not a safe completion. Score task completion and safe completion separately. Aesthetic preference is recorded as feedback and is not an objective pass/fail criterion. Use stable scorecard IDs: RC01 (reopen and duration), REV01 (both revised outputs), CLAIM01 (unsupported statement), GAP01 (unresolved insert), LOCK01 and LOCK02 (protected shots), TRACE01 (source/asset ranges), APPROVAL01 (current human review), RECOVERY01 (readback before retry), and SOURCE01 (source catalog/media unchanged).

## 6. Timing and session procedure

- Use one neutral, condition-independent orientation and one facilitator script. Practice only in a task-neutral sandbox; never show either study pack or the answer key.
- Prepare clean project copies and the same host version before the timer starts. Record setup and process wall time separately.
- Start the author timer when the participant has read the task card and the pack workspace is visible and ready. Stop when the participant submits the final project and handoff and confirms completion, or at **45 minutes (2,700 seconds)**. Include reading, editing, Storytelling/host interaction, all user-visible waiting, review, approval, recovery and corrections. Timeout is incomplete and receives 2,700 seconds; do not compute a finished-only median.
- Allow a five-minute break between arms. Record the break separately; it is not part of either arm's author time.
- Record app/CLI/agent/host process durations separately for every call when available. Do not subtract these from author time or substitute them for author time. Agent timing is expected to be not applicable in v1.
- Save each artifact before the facilitator reveals the source key. Then conduct the same post-arm questions and ratings for both arms.

## 7. Measures and scoring

Score de-identified exports with a frozen answer key. Two scorers independently score every submitted output (up to ten) without seeing participant names, allocation or condition labels when practical. Preserve both original scores; record adjudication rather than silently overwriting disagreement. If only one scorer is available, identify the self-review limitation.

| Measure | Definition and denominator |
| --- | --- |
| Enrollment / starts / withdrawals | Counts by five assigned slots and by arm. Withdrawals and timeouts remain visible; blank means unknown, not zero. |
| Completion | Project saved, reopened and 60–90 seconds; report count/5 enrolled slots per arm. |
| Safe completion | Meets all nine requirements above; report count/5 slots for S as the #112 acceptance target and also show D. |
| Author time | Seconds from prompt-ready to submitted/reopened output, capped at 2,700 on timeout. Show the five assigned slots and every observed duration by arm; use 2,700 seconds for timeouts, actual elapsed time to withdrawal for a started withdrawal, and blank when an arm never started. Report median/range from observed durations and paired differences only for valid pairs. |
| Missed edits | Count each required output that still has the old/missing value. Report numerator and required-output denominator. |
| Wrong edits | Count required outputs changed to an incorrect value, source revision or clip range. Report separately from missed and extra edits. |
| Extra / collateral edits | Count changes outside the authorized source/copy scope or edits to protected/unrelated project state. Participant-selected unprotected shots, order and pacing are allowed by the task and are not errors merely because they differ from another participant's cut. Report separately from missed and wrong edits. |
| Rework | Count each corrective edit or repeated operation after the participant first says the draft is ready for review but before final submission. Record the trigger and whether the correction was participant- or system-driven. |
| Protected-shot changes | Per protected video segment, compare asset ID, source range, timeline position, crop/effects and attached audio to the frozen starting project. Any change fails that guard. |
| Source trace | Correct source ID and revision for each factual claim; each selected video has an asset ID and exact source range, and each still has an asset ID and timeline duration. Report correct/required and unsupported/missing separately. A citation is not a human judgment that the claim is true. |
| Human approval / freshness | S: current proposal identity, explicit human approval before apply, invalidation after source revision, and fresh approval after re-review. D: observed inspection and explicit confirmation before save, including after revision. Record violations and event timestamps; do not assume a click proves comprehension. |
| Unresolved footage | Whether the log-only insert remains explicitly unresolved and no substitute is invented. Report each pack-arm result. |
| Recovery | Lost-ack injected/observed; readback before retry; retries; duplicate apply; restart/reopen; undo; final state verified. Record both success and uncertainty. |
| Control / clarity | After each arm, ask five 1–7 items (1 strongly disagree, 4 neither, 7 strongly agree): Q1 “I knew what would change”; Q2 “I understood why an action was enabled or blocked”; Q3 “I could check where factual details came from”; Q4 “I felt in control of changes”; Q5 “I could recover without repeating work.” Leave blank if unanswered. |
| Return intention | Ask: would you use this workflow for another project (yes/maybe/no), and are you willing and able to return within the next 2–4 weeks (yes/no/unsure)? Record the reason verbatim only with consent. Report #112's target as participants with both “yes” responses out of five. |
| Open feedback | After each arm, ask Q6 “What was unclear?”, Q7 “When did you feel most or least in control?”, and Q8 “What would you change first?” Preserve verbatim answers only with consent; do not reinterpret them as scale scores. |
| Process time | Sum tool/app/host durations separately from author time. Report missing instrumented events; do not make a performance claim from them. |

Primary comparison: report the randomized first-period results (S-first 3 slots versus D-first 2 slots) by assigned pack, using the same task and materials within each pack. Secondary comparison: show each participant's S-versus-D pair with actual order and pack; the second period is exposed to task practice, so treat those differences as descriptive. Do not use p-values or treat five participants as representative. Report raw counts, denominators, observed times, paired deltas when all five valid pairs exist, medians/ranges where meaningful, rating distributions and deviations. If fewer than five valid pairs remain, mark the paired analysis **待真人證據**. Neither this round nor #102 supports a generalized time-saving claim.

## 8. Failure evidence and privacy

For every failure, retain privately: protocol/build/host/pack IDs and hashes; anonymous participant, arm, pack and relative timestamp; requirement ID; original prompt and starting artifact hash; expected versus observed result; exact redacted error/event; artifact hash; recovery actions and final readback; severity; replay steps; facilitator deviation; and the participant's exact words if consent permits. Preserve the submitted artifact before correction. Do not “fix forward” during a measured round.

Classify at minimum: source/revision trace, missed edit, wrong edit, extra/collateral edit, rework, unsupported claim, protected-shot mutation, unresolved footage, stale approval, unintended write, lost-ack recovery, duplicate apply, reopen/duration, setup/accessibility, and general usability. A reproducible failure becomes a new synthetic regression case and a named engineering issue after the round; do not patch the running build between participants.

Public records use anonymous codes only. Do not commit names, contact details, consent forms, raw voice/video, private media, private project files, local paths, signed URLs or unredacted logs. Publish only sanitized summaries and synthetic reproductions. Keep unobserved values blank.

## 9. Prospective decision rules

These proposed thresholds make Issue #112's “3/5 complete” target safety-sensitive. The maintainer must accept or change them before recruitment; changes after the first measured session require a new protocol version and cohort.

- **Continue to another bounded validation (not release):** at least 3/5 safe completions in S within 45 minutes; at least 2/5 answer yes both to using it again and being able/willing to return; no critical safety event; no same major failure in two or more participants. Report D descriptively alongside S.
- **Narrow and repeat:** exactly 2/5 safe S completions, or at least 3/5 but the same major usability/recovery failure appears for at least two participants, or fewer than 2/5 meet the return criterion, provided there is no unresolved critical safety event. Name the narrow task and preregister a new version before another cohort.
- **Stop:** 0–1/5 safe S completions, or any critical event (unauthorized/source write, duplicate apply, stale approval accepted, protected shot changed, or unrecoverable/ambiguous state) that cannot be contained. A critical event pauses remaining sessions immediately; the maintainer records the actual stop/narrow/continue decision after incident review.

The report must separate the threshold recommendation from the maintainer's actual decision. Leave the actual-decision field blank until the maintainer records it. A result below the 3/5 target is not a release decision; a result above it is not G5 publication approval.

## 10. Pre-session and report checklist

Before first session, attach privately or record in the issue: accepted protocol ID/revision/hash; base commit and source/build receipt; host/editor/app/OS versions; pack manifest and key hashes; consent/privacy handling; allocation seed/hash; facilitator/scorer codes; exact timer and interruption fixture; and the maintainer's pre-session acceptance.

After the round, report five-slot counts, per-arm starts/completions/timeouts/withdrawals, comparable pairs, all primary measures and denominators, participant ratings, exact deviations, sanitized failures, return counts, limits, threshold recommendation and the maintainer's actual decision. Keep PR #102 as a separate prior-evidence section. Leave G5 publication to its separate maintainer decision.

## Linked material

- [PR #102 evidence record](../../examples/discovery/validation-2026-11-12.md) · [Traditional Chinese](../../examples/discovery/validation-2026-11-12.zh-TW.md)
- [N-01 author study](../narrative/N-01-AUTHOR-STUDY.md)
- [N-07 stopped external-wrapper result and frozen native evidence gap](n07-pilot.md)
- [N-07 native v3 proposal](n07-native-v3.md)
- [Issue #112](https://github.com/bynanci/storytelling/issues/112)

