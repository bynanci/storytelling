<!-- SPDX-License-Identifier: MIT -->
# N-07 paired native-CLI pilot packet

[主持人操作腳本（繁體中文）](protocol.zh-TW.md) · [Pilot report](../../../docs/validation/n07-pilot.md) · [Original N-01 protocol](../../../docs/narrative/N-01-AUTHOR-STUDY.md)

**Current disposition (2026-11-12, Asia/Taipei):** the maintainer [stopped the submitted external-wrapper evaluation and closed #54 as not planned](https://github.com/bynanci/storytelling/issues/54#issuecomment-6809828575): 0/5 primary completions, 5/5 authors with a major U01 attribution error, no release. Those human submissions do not establish five eligible pairs on the frozen native executable. **Frozen native acceptance remains 待真人證據.** See the [report](../../../docs/validation/n07-pilot.md) for the scored outcomes and provenance limits.

This is the preserved notes-versus-native-CLI packet, separate from #48/Skill, requester sample judgments and process benchmarks. The instructions and planned allocation are historical handoff materials. The maintainer elected not to ask the prior authors to repeat their work. A future native evaluation needs its own versioned prospective protocol and fresh authors; no follow-up recruitment or session is claimed here. The preparation observations.json, blank forms, original protocol and captured output are unchanged. The historical material-hash receipt binds [delivery 8e2c080](https://github.com/bynanci/storytelling/tree/8e2c08094f6bce70a7507e11ed84265bd98e3820/examples/evaluation/n07-pilot), before this README status update.

Use the original N-01 facts and N-03 source fixture. C01 stays in illustration A; the correction changes S01 and S02 names to Ju/Zhe. Do not substitute the existing N-04 history (different role change and umbrella scope) or the reviewed N-03 C02 draft. Both arms start from the same shared text. Only the CLI arm receives the extra structured templates and tool instructions. It must enter its own corrections and manually revise actual drafts.

The frozen engineering CLI is built from main `ce73836caa86502933f538821fdbf5dac92c1224`, with accepted N-05 ReportV1 from merged PR #83. PR #87 subsequently merged opt-in ReportV2; it does not change this fixed ReportV1 baseline. N-06 is optional and remains a separate decision.

## Rerun engineering checks and CLI timing

Build once before measuring. To reproduce the recorded baseline independently of later SDK changes, build in a clean worktree at the fixed source commit, then invoke this packet's helper with that binary:

```sh
git worktree add --detach /tmp/n07-cli-source ce73836caa86502933f538821fdbf5dac92c1224
cargo build --release --locked --manifest-path /tmp/n07-cli-source/Cargo.toml -p storytelling-cli
python3 examples/evaluation/n07-pilot/run_cli.py --cli /tmp/n07-cli-source/target/release/storytelling --manifest examples/evaluation/n07-pilot/cases.json --out-dir /tmp/n07-engineering-run --runs 5
python3 -m unittest discover -s examples/evaluation/n07-pilot -p 'test_*.py'
```

Use a fresh output directory each time; the helper refuses an existing one. On Windows, choose new writable paths and the `.exe` binary. Build/setup is outside the process measurement. One warmup per case is excluded from the five measured process-wall-clock runs. Startup, file I/O and serialization are included; no author completion or editing time is inferred.

The primary CLI arm edits a new versioned document snapshot and renders its own draft text. An initial engineering attempt at two consecutive ledger source changes was rejected by the accepted pending-review guard; that synthetic case is retained as a negative control, not a participant failure. The manifest includes expected rejection/status controls, so a successful audit may intentionally report missing/revised/dangling citations. The source and input hashes must remain unchanged. Inspect actual output semantics against [the facilitator key](facilitator/key.md). Keep engineering outputs and the key away from participants before submission.

## Participant and private record setup

Copy `participant-kit/shared` into a fresh private folder for each arm. Give the CLI arm `participant-kit/cli` too. The facilitator keeps the frozen originals and scoring key. Do not distribute `facilitator/`, captured expected output, or the public repository's answer material as an author workspace. Edit only working copies.

Copy header-only forms into private storage. `sessions.csv` records author time and counts; `errors.csv` keeps coded clauses; `feedback.csv` keeps anonymous words; `cli-calls.csv` keeps process times separately; `agreement.csv` records the pre-trial agreement. Blank is unknown, not zero. Do not commit completed participant records or private paths. The checked-in files contain headers only.

The full [protocol](protocol.zh-TW.md) defines the timer, primary/secondary scoring, proposed threshold transfer, learning effects, privacy, failure-to-regression workflow and the requester's shortest steps. The original continue/narrow/stop thresholds require five valid paired records and a recorded agreement; engineering-only success cannot select a product decision or release candidate.

## Matched snapshot adaptation / 同材料版本說明

The study baseline is `n07-door-repair@1`, derived from the original N-01/N-03 source units. Units S01–S04/I01/C01 are identical source-unit objects (text, origin, revision and based_on) to the original. The two brief texts are adapted to the N-01 framing without the old N-03 unsupported indoor background; no C02 is introduced. Both arms receive exactly the same rendered starting words. This is an explicitly prepared native-round adaptation, not a claim of byte-identical original N-01 draft wording or prior protocol agreement. Metadata edits are timed, the original corpus stays unchanged, and original N-01 thresholds are not revised after observations.

主試用使用相同的衍生快照，而非默默替換為已審查 C02 稿。原來源單元保留、提議仍只在 A。請在收案前確認這項 native 條件格式轉換。

## Separate timing for an actual participant CLI invocation

In the private CLI working folder, this example preserves stdout and writes a separate process timing line to a private log. It does not start/stop the continuous author timer. Use the same built binary as the session record; keep the log private.

```sh
python3 -c 'import json,subprocess,sys,time; t=time.perf_counter_ns(); r=subprocess.run(sys.argv[1:]); print(json.dumps({"args":sys.argv[1:],"exit_code":r.returncode,"cli_wallclock_ms":(time.perf_counter_ns()-t)/1000000}),file=sys.stderr); sys.exit(r.returncode)' /tmp/n07-cli-source/target/release/storytelling render-narrative cli/working-document.json > narrative-final.txt 2>> cli-wallclock.jsonl
```

Copy each actual elapsed value/exit to the private cli-calls.csv or retain the JSONL as its evidence. Record failed invocations and cold/warm/unknown cache state too. Do not replace author elapsed seconds with the sum of process milliseconds.
