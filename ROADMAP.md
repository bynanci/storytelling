# Storytelling roadmap

<a name="storytelling-開發路線圖"></a>

English · [繁體中文](ROADMAP.zh-TW.md)

When a workshop changes its date, an author needs to find every draft that mentions it. The poster layout can stay the same. This example guides the first cross-media milestones. See [Why Storytelling?](docs/WHY_STORYTELLING.md) for the research and author scenarios.

<a name="現有功能"></a>

## Product objective

| Entry point | What you can do | Evidence |
| --- | --- | --- |
| Storytelling skill | Draft from a purpose, an audience and source material. Use an agent or follow the Markdown instructions by hand. | [Eight templates](skills/storytelling/references/multimodal-templates.md) and [workshop](skills/storytelling/references/workshop-demo.md) |
| Local narrative CLI | Render a spoken outline and two illustration briefs from declared content. | [CLI guide](docs/CLI_GUIDE.md) and [N-03 record](docs/validation/n03-narrative-cli.md) include requester inspection and a corrected close-composition draft. Comparative participant trials remain open. |
| Local revision and style checks | Read the append-only history and find directly affected variants. `check-narrative-style` validates declared spoken-beat order against a revision-bound profile; it does not judge style. | [N-04 revision contract](docs/narrative/NARRATIVE_REVISION_LEDGER_V1.md), [N-04 record](docs/validation/n04-narrative-revisions.md) and [N-08 record](docs/validation/n08-style-profile.md). N-08 has a qualified qualitative closure on maintainer attestation; randomization remains unrecorded. |
| Local video CLI | Plan shots from declared source windows and constraints. Export a picture-only timeline within the supported profile. | [Video milestones](#milestones) |

The skill installs independently from Rust. The package version is `0.1.0-dev.0`. Install it from GitHub. Public registry publication requires a separate release decision. See [installation](docs/PLATFORMS.md).

<a name="跨媒體敘事的開發順序"></a>

## Cross-media narrative roadmap

The [research design map](docs/research/README.md) links evidence, rules and workflow priorities. Claims of benefit still require human observations.

| Outcome | Order of work | Acceptance question |
| --- | --- | --- |
| 1. Revise a story | N-02 defines the contract. N-03 renders drafts. N-04 records revisions and declared author decisions. The maintainer chose to continue after the [five-author N-01 Skill trial](https://github.com/bynanci/storytelling/issues/48#issuecomment-6798756961). | Does each direct reference appear in the impact report? Does an edit to one illustration leave the other intact? Use the [study packet](docs/narrative/N-01-AUTHOR-STUDY.md). |
| 2. Trace commercial claims | N-05 / #52 has the merged V1 baseline in PR #83; [PR #87](https://github.com/bynanci/storytelling/pull/87) adds opt-in product ReportV2. The gate sample has requester review; product author review remains pending. The CLI reports cited revisions and historical text; the requester judges support. N-07 follows N-05. | Keep the supported one-test result, remove the unsupported performance claim and label proposals. The maintainer stopped the submitted external-wrapper attempt (0/5 primary completions; 5/5 major U01 errors). Frozen ce73836 native acceptance is unverified; a future evaluation needs its own prospective version. |
| 3. Select another form | N-06 compared brainstorm branches, screenplay reveals and video mapping. The requester deferred all three on 2026-11-11, then selected video-only `NarrativeVideoCoverageV1` for N-10 on 2026-11-12. N-08 checks declared opening order and N-09 delivers separate audio cues. | Preserve the historical [N-06 decision](docs/validation/n06-format-discovery.md) and linked [N-10 selection](https://github.com/bynanci/storytelling/issues/53#issuecomment-6812213432). G2 is accepted in PR #93; the bounded G3 CLI and automated G4 checks merged in [PR #97](https://github.com/bynanci/storytelling/pull/97), with all six exact-head jobs passing in [run #376](https://github.com/bynanci/storytelling/actions/runs/43148901399). N10-05 is complete within its bounded evidence scope: Codex replayed the JSON inputs, the maintainer supplied one post-merge author self-session (145 seconds, prior exposure, no randomization or independent observer), and Codex probed the supplied synthetic MP4. Treat the human record as qualitative self-session evidence, not an independent user study; V1 does not read media bytes. See the [implementation record](docs/validation/n10-video-coverage-cli.md) and [post-merge execution record](docs/validation/n10-05-execution-record.md). N-08 records two maintainer-attested official-sheet readings with disclosed order and evidence limits. G5 publication remains separate. |
| 4. Hand work to another tool | N-11 uses the N-04 history. N-12 can test one host capability independently. | Can another person identify missing or superseded sources? Open the actual image or play the audio. Deliver the usable brief when the host lacks the capability. |

See [authoring tasks](TASKS.md#cross-media-narrative-backlog), [multimodal tasks](TASKS.md#multimodal-rust-backlog) and [architecture](docs/ARCHITECTURE.md). The author decides whether a draft serves its purpose.

<a name="影片-sdk-里程碑"></a>

## Milestones

The video work targets an editable 60–90 second rough cut from 30–50 selected photos, clips and narration. A handoff to the video planner needs actual asset IDs and source ranges.

| Milestone | State | Acceptance |
| --- | --- | --- |
| M0 foundation | The local CLI checks a declared StoryPlan. The maintainer accepted this scope. | Strict contract, source bounds, unchanged input and same-commit CI on three operating systems. |
| M1 interchange | The limited importer, OTIO writer and isolated Linux 25 fps proof passed acceptance. | FilmCraft imported, previewed and reopened the picture-only project. Audio, other profiles and human acceptance remain open. |
| M2 planning | The bounded audit, fixed-window planner and selected-domain comparison passed acceptance. The full solver remains open. | Repeat the result from the same inputs. Report hard conflicts. |
| M3 review and execution | P1, P2 and the M3-02 journal passed bounded acceptance. M3-03 has an isolated test. A production executor remains open. | Bind the proposal and artifact to the review. Check authority and operation ownership. Recover partial or unknown outcomes from receipts and host observations. |
| M4 consumers | Planned. A reference application and an independent TypeScript client will use the SDK. | One writer per service, visible review and approval, and an optional MCP interface. |
| M5 beta | Planned. M5-02 / #112 remains blocked on #109, #111 and accepted M3 execution/recovery. The [prospective v1 protocol](docs/validation/m5-02-rough-cut-beta-v1.md) is design-only; no beta has started. | Measured outcomes, tested host profiles and distribution evidence. |
| v1.0 | No release date. | Accepted migration, recovery, compatibility and security evidence for the public contract. |

<a name="實作依賴"></a>

## Dependency path

| Work | Prerequisite |
| --- | --- |
| M1-01 → M1-02 → M1-03 | M0. Prove the limited interchange before a real host test. |
| M2-01 → M2-02 → M2-03 | M2-01 follows M1-01. Requirements and fixtures can proceed in parallel. |
| M3-01/P1 → P2 → M3-02 → M3-03 | P1 requires M1-03 and M2-02. P2 connects the selected proposal to read-only OTIO review. |
| M4 → M5 → v1.0 | M4 requires M2-03 and acceptance of production M3-03 execution and readback. |

Read the [P2 contract](docs/migration/PLANNER_OTIO_REVIEW_V1.md), [journal evidence](https://github.com/bynanci/storytelling/pull/56) and [M3-03 test](docs/validation/m3-03-filmcraft-execution.md). The test reproduces a successful import with a lost reply. After a timeout, read back the project. Stop if the operation might still run or the result remains ambiguous.

<a name="如何決定是否繼續"></a>

## Investment decisions

- **M1.** Narrow the supported profile if basic clips or source ranges change during interchange.
- **M2.** Retain the reference planner if Rust produces different results. Use Rust for validation and compilation until the team explains the difference.
- **M4.** Compare the SDK with direct use of FilmCraft and an agent. Measure author effort, creative control and output quality.
- **M5.** Issue #112 retains the targets of three of five testers completing a rough cut and two returning for another project. The proposed protocol defines completion as safe completion and requires maintainer acceptance before recruitment; it does not establish results or release readiness.

<a name="發布與維運"></a>

## Release and operation

Record each candidate's source commit, locked dependencies, schema versions and artifacts. Record the compiler, adapter and host versions. The maintainer approves publication. Incompatible schema changes need migration steps and explicit rejection of newer versions. Preserve partial and unknown receipts. Real-media listening, native signing and target-editor acceptance require distinct evidence.

<a name="將既有功能移至-rust"></a>

## Native migration strategy

1. Freeze precise clocks, read-only import and OTIO interchange. Prove a handoff to one editor.
2. Select algorithms for a specific story task. Compare old and new results with fixed inputs. Travel, rhythm and transcription remain optional.
3. Move stateful review and recovery behind an accepted execution contract. SQLite cannot reverse an external editor operation.
4. Keep interfaces, codecs, GPU effects and full editing in their host applications. State each adapter's external dependencies.
5. Check provenance and concurrent access before extracting path or manifest helpers. Keep Tauri, GTK and Node launchers outside the portable SDK.
6. Compare both implementations before switching the feature flag. Keep one writer. Retire the previous implementation after acceptance. Preserve source data throughout read-only import.

See the [migration guide](docs/MIGRATION.md). Check behavior, runtime dependencies and regression results for each port.

## Gemini R1 skill evaluation record (SK-EVAL-R1)

The [40-case model-judged report and visual comparison](docs/research/evaluations/2026-11-13-gemini-r1/README.md) preserve numeric observations, source-packet errata and unverified declared assignments. This is W4/W6 diagnostic evidence, not a newly accepted requirement, human result or implemented skill improvement.
Use [#115](https://github.com/bynanci/storytelling/issues/115) as the umbrella. [SK-E01 / #118](https://github.com/bynanci/storytelling/issues/118) is merged with the versioned [offline record and blind-packet workflow](docs/research/evaluations/sk-e01/README.md). [SK-E02 / #119](https://github.com/bynanci/storytelling/issues/119) now has an evaluation-only [evidence-plan, adjudication and packet-errata contract](docs/research/evaluations/sk-e02/README.md); synthetic validation does not supply R1 provenance or a new result. [SK-E03 / #120](https://github.com/bynanci/storytelling/issues/120), [SK-E04 / #121](https://github.com/bynanci/storytelling/issues/121) and [SK-E05 / #122](https://github.com/bynanci/storytelling/issues/122) stay separate follow-ups. Existing N/M acceptance, #54's disposition and #112's human-evidence gates retain their separate scope.
