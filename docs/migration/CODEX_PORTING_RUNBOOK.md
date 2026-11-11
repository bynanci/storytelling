# Port one feature to Rust

<a name="rust-單項移植指南"></a>

[繁體中文](CODEX_PORTING_RUNBOOK.zh-TW.md) · [Port inventory](OPENFILM_RUST_PORT_MAP.md) · [Source ledger](openfilm-port-index.json) · [Development process](../SDLC.md)

Select one task. Record the source behavior, changes in the Rust version and the evidence for the result.

<a name="1-確認來源與任務狀態"></a>
<a name="1-開始前必須核實"></a>

## Check the base revisions

1. Fetch current branches and open pull requests in both repositories.
2. Check the task and its prerequisites in the [task list](../../TASKS.md).
3. Record the full source and target commit IDs. Use the original pinned revision when you repeat a historical test.
4. Check each source path with `git cat-file -e <sha>:<path>`.
5. Read the implementation, its callers and the tests. Record observations and missing evidence.

| Historical revision | Scope |
| --- | --- |
| Upstream `7add4762fe8174d76f43b863375ea39b73121e76` | Includes coverage PR #121 and frame-time PR #125. |
| Later upstream `5f737a6c8f35dce48e213370ff27900962c9304c` | Animation PR #126 merged feature commit `b4aba4c4bb70b8cb03edd54342f1b94cec649245`. The previous main remains the comparison baseline. The finite importer and OTIO writer reject animation. [Source comparison](records/M1-01.md) · [OTIO contract](FINITE_OTIO_V1.md) |
| Initial target `551ff38920e2085e1d53a7ba777c9680678e9725` | PR #1 was a draft at `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`. |
| M1-02 `9a5232e97435f543238b90a2006460f319f4e651` | Includes PRs #1, #2, #7 and #16. |
| M2-01 and M1-03 `c03d4c8ede33563451b8ad590a6df982d0d8b82a` | PRs #17 and #18 cover declared constraints, supplied bytes and one synthetic 25 fps CPU preview on Linux. More hosts, file-replacement protection and human review have separate requirements. [CLI guide](../CLI_GUIDE.md) |
| M2-02 `895257b1bb9993d70cae3f1267f5f6fa3de91905` | PR #22 covers fixed windows, hard limits and declared ranks. See the [planner contract](FIXED_WINDOW_PLANNER_V1.md), [M2-02 report](records/M2-02.md) and [M2-03 comparison](records/M2-03.md). |

<a name="2-填寫移植比對紀錄"></a>
<a name="2-一個任務的comparison-record"></a>

## Prepare the comparison record

Start `docs/migration/records/<task-id>.md` when implementation starts. Include these fields:

- Task, parent milestone and requirement IDs.
- Source repository, commit, path, blob, symbol and test.
- Target base, head and existing behavior.
- Boundary: `native-core`, `native-runtime`, `external-adapter` or `retain-host`.
- Current status: `absent`, `partial`, `implemented` or `verified`.
- Per-file source, license decision, changes and required notices.
- Input, output, checked types, clocks, units, side effects and data ownership.
- Preserved behavior, intentional changes and unsupported inputs.
- Production dependencies, test tools, external services and host requirements.
- Failure before implementation, passing tests, comparison results and CI for the reviewed commit.
- Host and human observations, remaining limits, rollback and the next task.

<a name="3-實作一項可驗收的功能"></a>
<a name="3-標準實作流程"></a>

## Implement and test the selected behavior

1. Keep one task and one outcome on an isolated branch.
2. Pin upstream code, tests and synthetic material. Record each path and its license.
3. Read valid and invalid test cases. Record the reference version or the absence of tests.
4. Write a failing test. Missing tools or dependencies block a run and cannot establish a functional failure.
5. Implement the behavior. Keep Node, Python, SQLite and host calls outside the pure engine. Declare any external dependencies.
6. Run the Rust and upstream tests. Compare outputs without changing meaningful order. Classify each difference.
7. Exercise the relevant failure paths. Consider input limits, file replacement, stale revisions, cancellation, interrupted processes and unknown results.
8. Update the requirements and evidence. Check CI for the submitted commit. Follow the applicable merge authority. Releases, tags and administrative changes require separate authority.

<a name="4-執行驗證"></a>
<a name="4-驗證指令依實際新增範圍"></a>

## Run the checks

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo doc --workspace --no-deps --locked
python3 scripts/check_repo.py
git diff --check
```

The repository check covers crates, direct dependencies, sources and named tests. M1-03 records its own SHA2 dependency review. Add requirements and checks for an adapter in the same PR. Add crates only when a current caller needs them.

<a name="5-可直接貼給-codex-的指令"></a>
<a name="5-可直接複製的總控-prompt"></a>

## Codex task prompt

```text
In bynanci/storytelling, implement one OF or GAP task under its existing milestone.
Read AGENTS.md, TASKS.md, ROADMAP.md, docs/SDLC.md, the port inventory and the source ledger.
Fetch both main branches and open PRs. Record full source and target revisions.
Check the upstream symbols and tests. Record source blob identities and applicable licenses.
State the contract, errors, clocks and side effects. Keep pure Rust separate from external tools and the host.
Write a failing regression before implementation. Compare Rust results with pinned source behavior.
Report exact commands, observed results, intentional changes, remaining limits and human checks.
Check CI on the submitted commit. Follow the repository's documented authority for merges.
Return the task and requirement IDs, evidence links, remaining work and the next task.
```

<a name="6-選擇基準與回退方式"></a>
<a name="6-pr策略與生命週期"></a>

## Choose a base and a rollback

Planning PR #2 originally depended on M0 PR #1. Both later merged. Use current main and accepted prerequisites for the next task. Name each PR for one result, such as M1-01 import, M1-02 timeline export or M1-03 host testing.

Evaluate a native alternative separately from replacing a working adapter. Check quality and compatibility before replacing FFmpeg. Rollback can disable the new feature, restore the previous entry point, or isolate the new artifacts. Preserve user projects and main history. Read the host state before recovery after an unknown result.
