# Hand a video proposal to the local CLI

<a name="用本機-storytelling-cli-交接影片提案"></a>
<a name="用本機命令列工具交接影片提案"></a>

[繁體中文](video-cli.zh-TW.md)

With notes or photo descriptions, deliver a written shot plan or narration. Identify shots with missing material. For reproducible shot selection or timeline export, first check the input. The command-line interface (CLI) reads caller-supplied files and writes its result to standard output.

<a name="選擇已具備的執行方式"></a>

## Find the command

| Environment | Action |
| --- | --- |
| Repository with a release build | Run `./target/release/storytelling --help`. On Windows, use `target/release/storytelling.exe`. |
| Source checkout without a build | Use the toolchain in `rust-toolchain.toml`. With cached dependencies, run `cargo build --offline --release --locked -p storytelling-cli`. |
| Installed skill only | Use the author's executable or run `storytelling --help` from PATH. Check that the subcommand exists. Supply your own files. When the command or files are missing, give the written draft and identify the gap. |

<a name="選擇輸入"></a>

## Match the command to the input

OpenTimelineIO (OTIO) is a timeline exchange format.

| Input | Command | Output |
| --- | --- | --- |
| Native M0 asset and shot plan | `validate <PLAN>` | Checks declared IDs and source ranges. |
| Brief, candidate windows, declared scores and limits on duration and shot count | `plan-story <REQUEST>` | Returns `planning.plan` and reasons for selection or omission. |
| Complete proposal response, matching finite-v1 snapshot and review context | `prepare-story-review <PROPOSAL> <SNAPSHOT> <CONTEXT>` | A JSON bundle containing the actual OTIO artifact and review record. |
| The same three inputs | `export-story-otio <PROPOSAL> <SNAPSHOT> <CONTEXT>` | A native OTIO timeline. Save it when requested. |
| Previous complete review bundle plus the three current inputs | `check-story-review <BUNDLE> <PROPOSAL> <SNAPSHOT> <CONTEXT>` | Checks declarations and artifact consistency. Stale or invalid input exits 1. |

Pass the complete successful `plan-story` response to review commands. Use `planning.plan` alone for `validate`. Asset IDs, declared hashes and available durations must match the snapshot.

The video profile supports stills and hard cuts on one contiguous picture track. Explicitly accept `picture_only` for video. Handle audio, effects and animation separately. Author approval and editor writes require separate steps.

<a name="用隨附合成資料重播"></a>

## Replay the synthetic example

Create a release build. Run this Bash example from the repository root. The pipeline writes only to the terminal. Its synthetic context accepts picture-only output.

```bash
set -o pipefail
./target/release/storytelling plan-story tests/fixtures/story-review-v1/planner-request.json |
  ./target/release/storytelling prepare-story-review - \
    tests/fixtures/openfilm/finite-v1/video-trim.json \
    tests/fixtures/story-review-v1/context.json
```

A successful result contains `ok`, `review` and `otio`. Its source range starts at 1/3 second and ends at 4/3 seconds. For other material, use the author's cuts, source IDs and conversion choices. Report the actual exit code and diagnostic on failure. [CLI guide](https://github.com/bynanci/storytelling/blob/main/docs/CLI_GUIDE.md)
