<!-- SPDX-License-Identifier: MIT -->
# N-11: a read-only umbrella-workshop handoff

English · [繁體中文](n11-handoff.zh-TW.md)

**Issue / requirement:** #68 / N-R11. **Base:** `main` `003191c834bfc46903f1a436fd73c5df6edd9a77`. **Scope:** one independent manifest and read-only CLI; existing NarrativeDocumentV1, NarrativeRevisionLedgerV1 and video V1 contracts are unchanged. [Proposed ADR 0009](../adr/0009-read-only-mixed-media-handoff.md)

## Start with the author's request

> 修傘活動改成週日，請告訴接手的人哪些稿要改。
>
> The umbrella workshop moves to Sunday. Tell the next editor which drafts must change.

The [Storytelling Skill](../../skills/storytelling/SKILL.md) uses the [workshop materials](../../skills/storytelling/references/workshop-demo.md): S1 supplies the notice, S2 supplies the left blue umbrella / right ochre spool description, and S3 supplies the synthetic taps description. The date change is the requested edit. C1, placing the poster text in the gap, is a poster-only creative proposal. The handoff reports the declared source map; this run does not inspect an image, play audio, infer meaning from a digest, or authenticate an author's decision.

## Dependency gate: replay #51 first

[#51](https://github.com/bynanci/storytelling/issues/51) is closed after merged [PR #76](https://github.com/bynanci/storytelling/pull/76). Before N-11 implementation, the existing CLI returned exit 0 for both original cases:

```sh
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json b-only-detail-change
```

The [shared-source output](n11-handoff/n04-shared.stdout) lists `oral, illustration-wide, illustration-close` for S02@1→2. The [variant-only output](n11-handoff/n04-local.stdout) lists only `illustration-close` for C01@1→2. These receipts were rerun after updating to the current main. The workshop's [standalone ledger](../../tests/fixtures/narrative-handoff-v1/date-ledger-v1.json) also returns exit 0 and the exact four affected variants: [actual #51 output](n11-handoff/date-impact.stdout).

## Expected impact, then comparison

The [expected set](../../tests/fixtures/narrative-handoff-v1/expected-impact.json) was written before the new CLI was executed. The [comparison](n11-handoff/impact-comparison.json) checks every row, with **zero false negatives and zero false positives** in both classifications.

| Component | Expected / actual classification | Before draft → supplied reviewed draft |
| --- | --- | --- |
| `invite/text` | UPDATE / UPDATE | Saturday invitation → Sunday invitation |
| `poster/date` | UPDATE / UPDATE | Saturday poster date → Sunday poster date |
| `oral-info/date` | UPDATE / UPDATE | Saturday spoken date → Sunday spoken date |
| `oral-poetic/date` | UPDATE / UPDATE | Saturday spoken date → Sunday spoken date |
| `poster/layout` | REUSE / REUSE | Blue umbrella left; ochre spool right; C1 remains poster-only |
| `oral-poetic/taps` | REUSE / REUSE | Same supplied two-tap cue |
| `original-wav/audio` | REUSE / REUSE | Same S3@1 file identity |

“Before” is the handoff after S1 changes to revision 2, while the four existing drafts still cite S1@1 and need review. That historical draft reference is legitimate. A source inventory descriptor falsely claiming the current S1 is still revision 1 rejects. The after fixture appends explicit variant-review events and retains the Saturday snapshots. `UPDATE` names the historical impact of the chosen edit; `status=current` shows that supplied Sunday drafts have already been reviewed in the ledger. Neither status proves the text is semantically correct.

Each artifact's components must cover the complete set of its bound variant's source IDs. Omitting a known date reference rejects instead of silently hiding impact. Proposal scope is checked across the whole bound snapshot, so leaving C1 out of a component map cannot hide its use by another variant.

## Replay the actual CLI

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/before-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/after-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/descriptions-only-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/transcript-missing-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/missing-source-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/stale-revision-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/proposal-leak-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-handoff-v1/date-ledger-v1.json date-to-sunday
```

[Exact executed commands and exit codes](n11-handoff/commands.json) use the built release binary. All stderr streams were empty.

| Case | Actual stdout | Exit |
| --- | --- | --- |
| Saturday drafts awaiting review | [before](n11-handoff/before.stdout) | [0](n11-handoff/before.exit) |
| Reviewed Sunday drafts | [after](n11-handoff/after.stdout) | [0](n11-handoff/after.exit) |
| Only source descriptions supplied | [descriptions](n11-handoff/descriptions-only.stdout) | [0](n11-handoff/descriptions-only.exit) |
| Missing image and supplied non-speech annotation | [partial handoff](n11-handoff/transcript-missing.stdout) | [0](n11-handoff/transcript-missing.exit) |
| Unknown source ID | [handoff.dangling_reference](n11-handoff/missing-source.stdout) | [1](n11-handoff/missing-source.exit) |
| Current descriptor at old S1 revision | [handoff.stale_revision](n11-handoff/stale-revision.stdout) | [1](n11-handoff/stale-revision.exit) |
| Poster-only C1 used by oral-info | [handoff.variant_scope](n11-handoff/proposal-leak.stdout) | [1](n11-handoff/proposal-leak.exit) |

The description-only handoff retains usable invitation text and a composition brief while explicitly stating that original media was not delivered. The transcript test supplies a textual non-speech annotation; it does not claim that taps.wav contains spoken words or that this tool transcribed audio. A `missing` representation preserves the gap and the remaining brief. [Input fixture guide](../../tests/fixtures/narrative-handoff-v1/README.md)

## Replay, source identity and tests

[Replay results](n11-handoff/replay.json) record exit 0 and exact stdout byte equality for a second file run and stdin. The integration test also checks that an isolated directory gains no new files and input bytes remain unchanged. [Before/after hashes](n11-handoff/source-readonly.json) cover both notices, the original SVG/WAV, manifests, ledger and expected set; every file was unchanged by the commands.

The renderer only checks digest spelling. Separate actual `verify-source` calls checked the supplied [notice bytes](n11-handoff/verify-S1.stdout), [SVG bytes](n11-handoff/verify-S2.stdout) and [WAV bytes](n11-handoff/verify-S3.stdout), each exit 0. These byte checks establish identity, not faithful wording or media quality.

Actual test-first RED: `cargo test --locked -p storytelling-cli --test handoff` exited 101; all 10 initial cases failed because `render-handoff` was not yet recognized and returned usage exit 2. [Original log](n11-handoff/tdd-red.log) · [exit](n11-handoff/tdd-red.exit). The final GREEN has 12 CLI integration cases, including omitted source coverage and hidden proposal scope regressions. Original assertions were preserved; the scope sentence was made precise about **source** files because the CLI must read its manifest input.

[Local gate commands and exits](n11-handoff/local-checks.json) record formatting, strict Clippy, workspace debug/release tests and warnings-denied Rustdoc. Existing video V1, digest and OTIO tests remain in those suites. Hosted `Required gates`, audit, three-platform Rust tests and the complete quality/oracle workflow must be read on the submitted PR head. No human editorial acceptance, audio listening or image-quality result is claimed. Semantic grouping, faithful Sunday wording and declared choices still require the author's review.

An independent automated agent reviewed the implementation and replayed omission and scoped-proposal cases. The review found that an incomplete component map could conceal a changed source; complete coverage and whole-snapshot proposal-scope checks now reject those cases. The final review reported no further actionable finding. It also validated all four positive fixture shapes against the schema and independently recomputed every saved source hash. This automated review is separate from human acceptance. The authoring agent also reviewed the final diff itself.
