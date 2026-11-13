<!-- SPDX-License-Identifier: MIT -->
# M5-02 rough-cut beta v1 · facilitator kit

[繁體中文操作版](README.zh-TW.md) · [Protocol and scoring rules](../../../docs/validation/m5-02-rough-cut-beta-v1.md)

**Prepared only. This kit does not recruit participants or claim a beta has started.** Issue #112 is still blocked on #109, #111 and accepted M3 execution/recovery. Do not use the forms for measured sessions until the maintainer accepts the protocol and every pre-session gate passes.

## Shortest path to a valid round

1. Read the full protocol and record the maintainer's acceptance of m5-02-rough-cut-beta-v1 and the proposed thresholds in #112.
2. Complete the app/host/M3 preflight. Freeze exact build, editor profile, OS, SDK, fixtures and fault-injection path.
3. Prepare two new licensed 36-item packs plus matching 75-second starter projects and answer keys. Complete the private [pack-freeze manifest](forms/pack-freeze.csv) and [pack receipts](forms/freeze-receipts.csv); record both pack and answer-key hashes. The repository contains no participant media or completed manifest.
4. Run a task-neutral dry run to check both packs, the expected answers, timing and source references. Do not count the dry run as a participant.
5. After protocol acceptance, run python3 examples/evaluation/m5-02-rough-cut-beta-v1/allocate.py --output /private/path/allocation.json once from the repository root; create the private directory first. Copy the blank forms to private storage. Keep consent and participant identity/contact data separately.
6. Recruit five eligible, fresh creators. Exclude PR #102 and prior N-01/N-07 participants, implementers, maintainers and anyone who has seen the packs or keys.
7. Run both arms for each participant in the assigned order, with a fresh isolated project copy per arm. Limit each arm to 45 minutes; include all user-visible waiting. Inject the same applied-but-lost-ack event in each arm and require readback before retry.
8. Save each submitted artifact before revealing the key. Score with two condition-blind scorers if available. Report missing data as unknown, not zero.
9. Publish only anonymized counts, sanitized reproducible failures and the maintainer's actual continue/narrow/stop decision. Leave G5 publication separate.

## Frozen comparison at a glance

- **S:** accepted Storytelling app/workflow with review, explicit current approval, host apply, receipt/readback and recovery.
- **D:** direct operation in the same editor and version, with the same source cards and a plain manual handoff sheet.
- No generative agent in either arm. A different agent setup requires a new protocol revision before recruitment.
- Each participant uses both conditions and both matched packs. Randomize three S→D and two D→S; balance Pack A across each condition at 2/3.
- Each pack has 18 photos, 12 video clips and 6 narration/audio items; five beats; one 75-second starter cut; two protected shots; two outputs dependent on one revised fact; one unsupported claim; and one log-only optional insert that stays unresolved.
- The primary #112 completion gate is **safe completion**, not merely a saved file: a reopened 60–90 second project, correct source update, no invented evidence, protected shots intact, current human review, and readback before retry.
- Five participants provide formative counts and paired descriptions only. No significance test or time-saving claim.

## Files

forms/ contains header-only CSV templates plus the detailed failure record. Copy them outside the repository before filling them. Never commit filled forms, raw media, voice/video, private paths, consent files, participant identities or unredacted logs.

The PR #102 observations are prior design evidence only. Their different handoff tasks and 72/55/68/52-second timings are not a control condition and are not included in this kit's sample.

