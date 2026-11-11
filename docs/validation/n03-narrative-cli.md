<!-- SPDX-License-Identifier: MIT -->
# N-03 local narrative CLI validation

**Task / requirement:** Issue #50 · N-03 / N-R03 · SDLC G3–G4  
**Implementation base:** `0c400b4d1f8308316328cf7e79e156ec5cb9806c` (latest main at implementation start; includes merged documentation PRs #72 and #73).  
**Human author inspection:** pending; no author response or usability result is claimed.

## Skill request and structured input

The [Storytelling Skill](../../skills/storytelling/SKILL.md) asks the author to
state audience, purpose, source details and desired outputs, then keep
observations, interpretation and invented material distinct. The natural-language
request for this synthetic case is:

> 請用星期六兩位鄰居一起調整共用木門的合成筆記，替社區電子報寫有先後次序的口述提綱，並交付兩份視覺焦點不同的單幅插畫 brief：一份用寬景呈現兩人、木門與工具箱，一份近拍鉸鏈和扶門的雙手。保留 S01–S04、I01、C01 的來源標籤，把想像的紅傘 C01 只放進寬景，不要把一次試關寫成永久修好。不要加入媒體檔、片長或剪點。

The CLI's structured input is the versioned
[`n03-authoring-v1.json`](../../tests/fixtures/narrative-v1/n03-authoring-v1.json).
It reuses the #48 S01–S04 observations, I01 interpretation and C01 visual
proposal. The #48 U01 commercial claim remains in the author-study packet; the
N-02 contract has no claim/evidence unit type, so this N-03 slice neither
relabels nor emits it. Claim substantiation remains N-05. The CLI is local and
deterministic: it does not invoke an Agent, model, image generator, editor or
media reader.

## Copyable run and actual output

Run from the repository root with Rust 1.89.0:

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-narrative tests/fixtures/narrative-v1/n03-authoring-v1.json
```

The same command accepts stdin with `-`:

```sh
cat tests/fixtures/narrative-v1/n03-authoring-v1.json | cargo run --release --quiet --locked -p storytelling-cli -- render-narrative -
```

For the checked fixture, the first command exited 0 and emitted these three
actual output sections on stdout:

```text
Narrative: door-repair@1 (schema v1)
Purpose: "呈現兩位鄰居一起處理共用木門的小事。"
Audience: "社區電子報讀者"

## Variant oral — "口述大綱" [spoken_outline]
1. [beat-01] [S01@1 | supplied_observation] "星期六上午，兩位鄰居在共用木門旁準備工具。"
2. [beat-02] [S02@1 | supplied_observation] "阿梅扶住門，阿安收緊上方鉸鏈的一顆螺絲。"
3. [beat-03] [S03@1 | supplied_observation] "兩人試關一次；這一次門沒有磨到地面。"
4. [beat-04] [I01@1 | author_interpretation] "一起處理一件日常小事，可以成為鄰里互助的畫面。"

## Variant illustration-wide — "插畫 A｜兩人與共用門（寬景）" [single_illustration]
Sources: [S01@1 | supplied_observation], [S02@1 | supplied_observation], [S04@1 | supplied_observation], [I01@1 | author_interpretation], [C01@1 | creative_proposal]
Brief: "平視寬景：兩位鄰居與木門同框，阿梅扶門、阿安在上方鉸鏈工作；工具箱放在門邊。把 C01 的想像紅傘放在畫面後方的圍欄旁。紅傘只屬這幅插畫，不是原始觀察。"

## Variant illustration-close — "插畫 B｜鉸鏈與雙手（近景）" [single_illustration]
Sources: [S02@1 | supplied_observation], [S03@1 | supplied_observation], [I01@1 | author_interpretation]
Brief: "近距離特寫：視覺焦點放在鉸鏈、螺絲與扶門的雙手；手和工具在前景，門框與室內背景柔和退後。只呈現明確標示的細節，不增添其他未提供內容。"

```

Success output is plain UTF-8 text in declared variant/beat order. Text values
are JSON-quoted so embedded newlines or control characters cannot forge another
heading. Domain and usage errors are one JSON line; the dangling-reference case
is `narrative.dangling_reference` at
`$.variants[0].beats[0].unit_ref.unit_id`, exit 1. No error prints a machine
path or source text.

## Automated verification

RED was observed before the renderer implementation. The first CLI integration
run compiled the tests, then both failed because the real executable returned
exit 2 with `cli.usage` for the not-yet-existing command (the tests expected
success for the valid fixture and exit 1 for the bad reference). This was an
observed application-level RED, not a missing-toolchain condition.

GREEN on the implemented command:

```text
cargo +1.89.0 test --locked -p storytelling-engine --test narrative
5 passed; 0 failed

cargo +1.89.0 test --locked -p storytelling-cli --test narrative
2 passed; 0 failed
```

The engine suite also checks all paired N-02 valid/invalid fixtures, the
deterministic competing-cycle witness (`R03`, `R04`, `R05`), exact 1 MiB versus
1 MiB + 1 byte, and 32 versus 33 nested containers. The CLI integration suite
invokes the actual `storytelling` executable and verifies:

| N-R03 condition | Automated evidence |
| --- | --- |
| No media files, paths, durations, or cut points | Fixture has none; the test recursively rejects video-only input fields and render succeeds. |
| Repeatability and stdin/file parity | Two file runs and one stdin run produce byte-identical stdout. |
| No mutation | Input file bytes and the complete temporary test-directory snapshot are identical before and after rendering. |
| Bad reference diagnostic | A missing source returns the frozen code/path JSON error, exit 1, and creates no files. |
| Variant scope and visual contrast | C01/red umbrella appears in the wide variant only; the close brief focuses on hinge/hands in foreground and subdued background. |

Exact test mapping:

| Test | N-R03 evidence |
| --- | --- |
| `storytelling-engine/tests/narrative.rs::accepted_fixtures_project_variants_and_keep_source_provenance` | Accepted V1 and #48 authoring fixture; ordered outline, variants, source ID/revision/origin projection, C01 reference. |
| `...::paired_contract_fixtures_return_their_declared_diagnostics` | Every accepted #49 positive/negative fixture, including deterministic cycle IDs. |
| `...::provenance_reference_relation_scope_and_explicit_order_rules_reject` | Observation/interpretation basis, derivation cycle, duplicate source, relation scope and beat-order refusal. |
| `...::diagnostic_precedence_checks_all_ids_and_references_before_basis_and_scope` | Contract-defined error phase priority for references, duplicate relation IDs and missing basis. |
| `...::raw_byte_and_container_depth_boundaries_are_inclusive` | Exact/over 1 MiB and exact/over depth 32. |
| `storytelling-cli/tests/narrative.rs::renders_three_declared_outputs_reproducibly_without_media_or_writes` | Actual binary, no-media success, three outputs, file/stdin equality, repeatability, unchanged input/test directory, source/variant separation. |
| `...::dangling_reference_returns_the_contract_diagnostic` | Actual CLI malformed-reference code/path, one-line JSON, exit 1 and no mutation. |

Legacy regressions were replayed through the workspace suites: M0
`storytelling-engine/tests/validation.rs` + `storytelling-cli/tests/cli.rs`;
finite OpenFilm `openfilm_snapshot.rs` + `openfilm.rs`; and OTIO `otio.rs` in
both crates. The official OTIO 0.18.1 oracle read/write/read passed all eight
cases, including the selected-story bridge oracle's two cases.

Local verification passed with toolchain 1.89.0: `cargo fmt --check`,
`cargo clippy --workspace --all-targets --locked -- -D warnings`,
`RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps --locked`,
`cargo test --locked --workspace`, `cargo test --locked --workspace --release`,
`python3 scripts/check_repo.py` and `python3 scripts/test_skill_distribution.py`.
The debug and release workspace runs include the legacy M0 video, Brief/planner,
OpenFilm finite importer, CLI and OTIO suites. Existing differentials also passed:
six OpenFilm importer cases, eight official OTIO parse/write/read cases, two
story-review/official-OTIO cases, ten StoryBrief cases, twelve fixed-window
planner cases and six FilmCraft-proof rejection checks. These local results do
not substitute for the PR's hosted checks; same-head CI status belongs to the
PR checks and will be reported after they finish.

## Author inspection and human feedback

The outputs above are ready for direct review. An author can run the command
from the repository root, then check these questions without editing the input:

1. Does the oral sequence read naturally, keep S01 before S02 before S03, and
   keep I01 visibly marked as interpretation?
2. Do the wide shot and close-up feel like different compositions, and do their
   source tags make the difference understandable?
3. Is C01 clearly an imagined proposal in the wide brief and absent from the
   oral outline and close-up? Does either brief imply more than the single door
   test recorded in S03?
4. Are the source labels and quoted text easy to review in terminal output?

Record observed reactions, corrections and the author's decision separately
from tests and CI. No author has inspected the outputs in this delivery yet.

| Reviewer code | Date | Outputs inspected | Observed response / requested change | Decision |
| --- | --- | --- | --- | --- |
|  |  |  |  |  |

