<!-- SPDX-License-Identifier: MIT -->
# N-04 narrative revision tracking validation

**Task / requirement:** Issue #51 · N-04 / N-R04 · SDLC G3–G4  
**Base:** main `27d6985d569c0e8882c4218b19115afc382e5aad` (merged N-03 CLI).  
**Decision record:** [ADR 0006](../adr/0006-append-only-narrative-revisions.md).  
**Author inspection:** pending; these are synthetic fixtures and declared choices, not an observed human trial.

## Expected impact written before the implementation check

The ledger in [`n04-revision-history-v1.json`](../../tests/fixtures/narrative-v1/n04-revision-history-v1.json)
starts from the accepted N-02 document. S02 is directly cited by oral, wide A
and close B, so changing S02@1 must report all three. After the three explicit
review events update their S02 reference, C01 is cited only by close B because
the fixture records a decline in A and oral has no C01 reference. Changing
C01@1 must therefore report only `illustration-close`. The expected sets are
part of the engine and real-CLI tests below; the CLI never accepts a caller
supplied impact list.

## RED then GREEN

Before the CLI implementation, the new actual-binary test
`revision_tracking_queries_shared_and_variant_local_impacts` invoked
`query-narrative-impact`. The executable returned `cli.usage`, exit 2, because
the command did not exist; the test expected success for the valid fixture and
failed with the observed status mismatch. This was application-level RED.

After implementation, the narrative integration binaries passed:

```text
storytelling-engine/tests/narrative.rs: 8 passed; 0 failed
storytelling-cli/tests/narrative.rs:    4 passed; 0 failed
```

The tests exercised `track-narrative-revisions` and both `query-narrative-impact`
commands through the real CLI process. They checked exact event IDs/revisions,
ordered affected IDs, old S02/C01 content, old variant snapshots, A's decline,
B's acceptance, the B-only stale state, unchanged oral/ A content and the
negative suggestion self-confirmation case. A subsequent cleanup changed no
CLI behavior: it removed a redundant variant-reference cap (N-02 already
validates per-kind limits) and made author-decision event results leave source
revision fields empty. The final hosted checks must compile and test that exact
PR head.

## Before and after output

The base rendering is produced by the normal N-03 entry point:

```sh
storytelling render-narrative tests/fixtures/narrative-v1/n03-authoring-v1.json
```

Before edits, actual output includes these three source-tagged details:

```text
2. [beat-02] [S02@1 | supplied_observation] "阿梅扶住門，阿安收緊上方鉸鏈的一顆螺絲。"
## Variant illustration-wide — "插畫 A｜兩人與共用門（寬景）" [single_illustration]
Sources: [S01@1 | supplied_observation], [S02@1 | supplied_observation], [S04@1 | supplied_observation], [I01@1 | author_interpretation], [C01@1 | creative_proposal]
Brief: "平視寬景：...把 C01 的想像紅傘放在畫面後方的圍欄旁..."
## Variant illustration-close — "插畫 B｜鉸鏈與雙手（近景）" [single_illustration]
Sources: [S02@1 | supplied_observation], [S03@1 | supplied_observation], [I01@1 | author_interpretation]
Brief: "近距離特寫：視覺焦點放在鉸鏈、螺絲與扶門的雙手..."
```

This N-03 starting output identifies S02@1 in oral, A and B. C01 is only in
wide A at that point. The revision ledger then records the declared choice to
decline C01 in A and accept the same creative proposal only in B.

The two copyable queries are:

```sh
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json b-only-detail-change
```

Actual stdout from the CLI for those fixture events:

```text
Change shared-observation-change: S02@1 -> S02@2
Affected variants: oral, illustration-wide, illustration-close

Change b-only-detail-change: C01@1 -> C01@2
Affected variants: illustration-close
```

To inspect all source and output versions, including which exact version each
old output cited, run:

```sh
cargo run --release --quiet --locked -p storytelling-cli -- track-narrative-revisions tests/fixtures/narrative-v1/n04-revision-history-v1.json
```

The actual final-state excerpt is:

```text
Source S02@1 [supplied_observation]: "阿梅扶住門；阿安收緊上方鉸鏈的一顆螺絲。"
Source S02@2 [supplied_observation]: "阿安扶住門；阿梅收緊上方鉸鏈的一顆螺絲。"
Source C01@1 [creative_proposal]; based on S01@1: "一把想像的紅傘靠在木門旁的圍欄上。"
Source C01@2 [creative_proposal]; based on S01@1: "一把想像的紅傘移到構圖 B 的門框左後方。"

Choice decline-red-umbrella-in-a: illustration-wide C01@1 declined
Choice accept-red-umbrella-in-b: illustration-close C01@1 accepted

Current oral@2: current; sources [S01@1, S02@2, S03@1, I01@1]; "口述：...阿安扶門、阿梅鎖緊上方鉸鏈..."
Current illustration-wide@3: current; sources [S01@1, S02@2, S04@1, I01@1]; "構圖 A...門面保持簡潔。"
Current illustration-close@3: needs_review; stale [C01@1]; sources [S02@2, S03@1, I01@1, C01@1]; "構圖 B...想像紅傘 C01 位於右後方。"
```

The track output retains the original `S02@1`, `C01@1`, and base variant
snapshots. Accepting the imagined red umbrella does not edit observation S01
or oral: current oral still has no C01 reference or red-umbrella text. A stays
current after C01 changes; B keeps the prior C01@1 reference and is the only
variant marked `needs_review`.

## Negative case and exact tests

`n04-invalid-self-confirm-v1.json` adds `"decision":"accepted"` to an
`assistant_suggestion`. Running `track-narrative-revisions` returns exit 1 and
one JSON error line:

```json
{"error":{"code":"revision.untrusted_decision","path":"$.events[0].decision"},"ok":false}
```

Exact tests:

| Test | N-R04 evidence |
| --- | --- |
| `storytelling-engine/tests/narrative.rs::revision_ledger_reports_exact_impacts_and_preserves_snapshots_and_choices` | Shared S02 affects exactly oral/A/B; C01 affects exactly B; source history, old variant snapshot and author decisions remain present; observation/oral stay free of C01. |
| `...::assistant_suggestion_cannot_self_record_author_confirmation` | Rejects self-confirmation with `revision.untrusted_decision`. |
| `...::revision_ledger_requires_current_revisions_and_review_before_next_source_change` | Rejects stale expected revision and a second change before pending review is resolved. |
| `storytelling-cli/tests/narrative.rs::revision_tracking_queries_shared_and_variant_local_impacts` | Runs both queries and the history command through the actual CLI and verifies output and no input/temp-file mutation. |
| `...::untrusted_suggestion_cannot_claim_author_acceptance_from_the_cli` | Runs the forged suggestion fixture through CLI; checks one-line diagnostic and exit 1. |

The author-decision event is a separate, closed input event, but the local CLI
does not authenticate who authored it. It records the declared choice; it does
not establish a real person's identity or approval. The schema and semantic
profile also explicitly scope impact to direct variant references rather than
transitive `based_on` dependencies.

## Existing-flow regression and local verification

`python3 scripts/check_repo.py` passed after the N-04 docs/schema/fixture edits.
The JSON parser accepted the revision schema and both N-04 fixtures. The
Draft 2020-12 validator accepted the valid ledger and rejected the
self-confirmation fixture. The official differentials also passed: six
OpenFilm finite cases, eight OTIO parse/write/read cases, two story-review /
official-OTIO cases, ten StoryBrief cases and twelve fixed-window planner cases.
The available prebuilt debug harnesses passed all 25 test binaries (including M0
validation/CLI, brief/planner, OpenFilm finite import, OTIO, StoryReview,
review-binding, journal, N-02 narrative and N-04 CLI/engine); two host-only
tests remain explicitly ignored. The compiled `storytelling` application was
not a test harness and was excluded from that count.

At final documentation time the sandbox did not expose `cargo` or `rustc` on
PATH, so the just-edited Rust source could not be rebuilt locally. The 8/4
narrative debug test results above came from the previously compiled workspace
artifacts before the small result-field cleanup. The PR's hosted required,
quality, dependency and three-platform checks must pass on the final head and
are reported separately in the PR. A green result on an earlier commit is not
evidence for the final head.

No OpenFilm, OTIO, Catalog or FilmCraft files are inputs to N-04. Existing
video/OpenFilm/OTIO regression suites are among the passing prebuilt debug
harnesses. No editor or network service is called by either N-04 command.

## Author inspection

No author result is claimed. After the PR head is available, inspect the
`track-narrative-revisions` output and run both impact queries above. Check that
the old S02/C01 source lines and prior A/B/oral snapshots are retrievable, the
two expected variant sets are exact, A remains current after the C01 change,
and the umbrella never appears as an observation or oral source. Record any
correction or choice in the author-study channel separately from automated
tests and CI.

| Reviewer code | Date | Outputs inspected | Observed response / requested change | Decision |
| --- | --- | --- | --- | --- |
|  |  |  |  |  |
