<!-- SPDX-License-Identifier: MIT -->
# Private native-trial recorder V1

English · [主持人操作（繁體中文）](README.zh-TW.md)

This standard-library Python 3.10+ tool implements **N07-REC01 / N-R07** instrumentation for the [prospective v3 protocol](../../../docs/validation/n07-native-v3.md). It records a declared agreement, allocation, screening, starts, raw CLI calls, submissions and private attachments. It does not score prose, authenticate a maintainer, recruit authors or calculate a product decision. #54 remains stopped; preparing or merging this tool does not adopt a new trial. G5 is separate.

Use a local disk with SQLite locking and reliable flushes. One facilitator runs one session at a time. Keep the round and exported bundle outside the repository, cloud-sync folders and shared network filesystems. `rehearsal` and `trial` are immutable modes: never relabel a rehearsal as human evidence. The facilitator owns the private screen and allocation; the author receives only the relevant working material, never the full repo or key.

## Prepare and rehearse

First run the [pinned clean-build recipe](../../../docs/validation/n07-native-v3.md#frozen-inputs-and-command-preflight). It defines `N07_RUN`, containing the ce73836 binary and build receipt. From the repository root, in that same shell:

```sh
N07_RECORDER="$PWD/examples/evaluation/n07-recorder/record.py"
N07_PRIVATE="$(mktemp -d "${TMPDIR:-/tmp}/n07-rehearsal.XXXXXX")"
python3 "$N07_RECORDER" init --round "$N07_PRIVATE/round" \
  --cli "$N07_RUN/target/release/storytelling" \
  --receipt "$N07_RUN/build-receipt.json" --mode rehearsal
```

On Windows use Python 3 and private absolute paths, including `storytelling.exe`. The recorder itself uses no shell, Bash or Git subprocesses. The Bash setup above is a convenience, not a Windows setup test.

Initialization creates an exclusive private directory and SQLite database, records the executable/receipt/protocol/recorder hashes and a once-only 3 AB / 2 BA draw with seed and Python version. Repeating initialization refuses to overwrite it. Planned slots are not enrolled authors. A matching executable hash is a comparison against a declared receipt, not proof of its build source; `source_binding_verified` and `agreement_authenticated` remain false.

Create a private screening JSON with exactly these fields, replacing the values with actual screening records for a trial. The following is **synthetic rehearsal input only**:

```json
{"schema_version":1,"screened_at":"2026-11-12T00:00:00+00:00","prior_exposure":false,"cli_json_experience":"synthetic rehearsal","facilitator_code":"F01"}
```

Save it as `$N07_PRIVATE/screening.json`, then:

```sh
python3 "$N07_RECORDER" enroll --round "$N07_PRIVATE/round" \
  --participant DEMO01 --slot 1 --screening "$N07_PRIVATE/screening.json"
python3 "$N07_RECORDER" workspace --round "$N07_PRIVATE/round" \
  --participant DEMO01 --condition A --out "$N07_PRIVATE/notes"
python3 "$N07_RECORDER" workspace --round "$N07_PRIVATE/round" \
  --participant DEMO01 --condition B --out "$N07_PRIVATE/native"
python3 "$N07_RECORDER" status --round "$N07_PRIVATE/round"
```

The workspaces copy hash-pinned files from the original material receipt. Both receive identical shared starting words; B also receives the structured CLI files. No scoring key or corrected draft is copied. Existing destinations are refused. Prepare both arms before the task, keep their original snapshots, and give the author only the current arm. Do not edit sources in the repository. Workspace events retain starting bytes and hashes even after authors change their private copies.

## Run one timed arm

Read the change card, then start the assigned first condition from `config.allocation`. A means notes, B means native CLI. Change `A` to `B` if slot 1 drew BA:

```sh
python3 "$N07_RECORDER" session --round "$N07_PRIVATE/round" \
  --participant DEMO01 --condition A
```

The live process first persists a session-start event. Its continuous timer includes recorder overhead, editing, JSON metadata changes, CLI waiting, error recovery and saving. **Keep this process running**, including while the author edits in another window. It prints the session ID; retain it for attachments. Do not type Bash commands into its prompt. Its commands use JSON arrays of file paths; environment variables are not expanded there. Use literal absolute paths or paths relative to the directory from which the recorder was started:

```text
render ["/private/native/cli/working-document.json"]
audit ["/private/native/cli/copy-template.json","/private/native/cli/evidence-ledger.json"]
submit ["/private/native/cli/working-document.json","/private/native/shared/answers.md","/private/native/cli/copy-template.json"]
```

On Windows JSON paths use `/` or escaped `\\`. `render`/`audit` are only available during B. They directly invoke the receipt-bound native executable with immutable input copies. The call receipt maps the original author paths to exact invoked snapshot paths and bytes. The raw output files are printed as `stdout_path`/`stderr_path`; open those to inspect the actual outline/briefs or ReportV1. The recorder never transforms their contents or fabricates references. Output receipts and captured raw bytes are retained automatically, so submitting the final working JSON, copy and answers does not require retyping the generated drafts. If the author invokes several versions, identify the final call IDs in the answers or scoring record.

In A, use `submit` with the edited source cards, narrative drafts, commercial draft and answers. The array can contain 1–16 files; it is a capture mechanism, not a check that all required deliverables are present. A `submitted` session is **not** a scored completion: `completed` stays null until the reviewer assesses the private forms. Missing files, rejected input and failed native calls consume author time. Failed submissions can leave earlier captured files in the log; a later submission never deletes them.

At 480 seconds the session ends as `timeout`, `completed:false`, analysis time 480; the measured elapsed time is preserved. Each native call has at most 60 seconds or the remaining author time, whichever is shorter. A call timeout by itself does not equal an author timeout; it preserves raw partial output and allows the author to continue within the remaining time. There is no automatic resend.

The timer cannot stop an external text editor. At timeout the facilitator must stop editing and preserve partial work immediately with `attach --kind submission`. Such attachments have their own timestamps and **are not certified as a deadline snapshot**. Already captured calls and submissions keep their exact bytes. Human review must flag late or missing artifact preservation. EOF, Ctrl-C or clock divergence is an interruption/unknown-time case, not a completed task. The tool does not infer a wall-clock duration after a crash.

Use `withdraw reason` in the prompt to record an actual withdrawal. Before the second arm, prepare a new author copy and follow the recorded order without disclosing the key. Starts in the wrong order, duplicate arms, concurrent sessions, exposed authors and continuation by a replaced participant are rejected. Record any real-world deviation separately rather than changing the original allocation.

## Preserve observations and recover

Copy the existing [header-only forms](../n07-pilot/forms/sessions.csv) and sibling errors/feedback/cli-calls files into private storage. Record completion/started denominators, attribution clauses and major counts, repeated manual entries, rework, original feedback, setup and environment. Do not equate CLI exit 0 with success or retries with repeated facts. Blank means unknown. Attach the original files without rewriting their contents:

```sh
python3 "$N07_RECORDER" attach --round "$N07_PRIVATE/round" --session SESSION_ID \
  --kind feedback --file "$N07_PRIVATE/feedback.csv"
python3 "$N07_RECORDER" attach --round "$N07_PRIVATE/round" --session SESSION_ID \
  --kind scoring --file "$N07_PRIVATE/sessions.csv"
python3 "$N07_RECORDER" export --round "$N07_PRIVATE/round" --out "$N07_PRIVATE/export-01"
```

Kinds are `submission`, `feedback`, `scoring`, `deviation`, `decision`. An attachment is unassessed raw evidence; naming it `decision` cannot authenticate or select a maintainer decision. Exports contain the event log and content-addressed byte blobs. They include private paths and answers: **do not publish the bundle**. Prepare a separate consented anonymous summary. `comparable_pairs`, completion assessment and maintainer decision are deliberately not inferred from file presence; the recommendation remains 待真人證據 pending the protocol's human review.

If a recorder dies, inspect the existing round. A committed start remains active and a started call without a result remains `unknown`. After checking the old process is stopped, explicitly close that session:

```sh
python3 "$N07_RECORDER" recover --round "$N07_PRIVATE/round" --session SESSION_ID \
  --reason "Facilitator observed recorder termination; original timing unavailable"
```

Recovery appends `interrupted`, with unknown analysis time. It does not rerun the CLI, reset a timer or overwrite the original event. Partial stdout/stderr files survive ordinary recorder termination and are exported with `complete:false`; filesystem failures may still lose uncommitted bytes. A late call result remains an observation marked late, and cannot change the ended session. `recover` does not kill a surviving process: stop/check it first. Preserve unknown outcomes and never promote them to a valid pair automatically.

A new author may use `enroll ... --replacement-for PREVIOUS_CODE` only in the same slot after withdrawal/interruption, up to five replacements. Timeouts or unsuccessful but recorded pairs cannot be replaced to improve scores. The all-started dataset remains intact. Before any replacement, apply the protocol's comparable-pair and five-pair stopping rule manually; this recorder cannot infer comparability from unassessed attachments.

## Real trial activation and limits

After prospective maintainer adoption, create a **new** directory with `--mode trial --agreement /private/agreement.json`. The agreement must have exactly these fields:

| Field | Value |
| --- | --- |
| schema_version | Integer 1 |
| protocol_sha256 | `a9b3038272107ee6986c22879c354e73cbbd55d40d498ad3d3c12aef87d0bb1d` |
| protocol_commit | `fe2b145250b42564bf5b4d781ced5a95e8a8f9bf` |
| recorder_sha256 | SHA-256 of the actual `record.py` used; record before recruitment |
| adopted | Actual maintainer declaration `true` |
| recorded_at | Actual ISO timestamp with timezone, before the trial |
| maintainer_label | Actual decision-maker's label; stored privately |
| round_id | New anonymous bounded round identifier |

Do not fill this from a hypothetical approval or copy the synthetic screening example as human evidence. Record the tool commit as well in the private trial notes. This new recorder is a proposed instrumentation choice; accept its measured overhead and interruption handling prospectively along with the v3 protocol. A new tool hash requires a new accepted setup rather than updating an active round.

Strict JSON rejects duplicates, unknown fields, unknown versions and non-finite constants. JSON control files are limited to 1 MiB; artifact inputs and each output stream to 16 MiB. On output overflow the direct child is stopped, captured bytes are explicitly partial, and the call cannot establish complete native output. The executable is checked before/after each call; source files are read into snapshots, and original/snapshot hashes are compared afterward. These checks are not a sandbox or protection against an adversary who controls the executable, recorder or database. SQLite transactions provide one writer and durable committed events on a supported local filesystem; content hashes detect artifact corruption, not forged human identity or a maliciously rewritten database.

For validation, run `cargo build --workspace --locked` then `python3 -m unittest discover -s examples/evaluation/n07-recorder -p 'test_*.py'`. The native integration test requires the debug binary. [RED/GREEN, traceability and evidence](../../../docs/validation/n07-recorder.md).
