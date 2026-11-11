# Research rules and skill delivery

English · [繁體中文](research-rules-skills.zh-TW.md)

Date: 2026-11-11. Base: `b7fcf869d01727dcf408c457fdd42929dca80c7a`. Scope: original research summaries, portable editorial rules, six on-demand workflows and bilingual integration. No Rust, schema, dependency or license changes. The submitted PR identifies the tested head and hosted check results.

Delivery incorporated main `60e302e2ea3b235e9bd7aae58049690ec9dad0f2` (#78 audio cues) without conflicts. The PR diff against that main remains documentation-only. Local documentation/package checks were rerun on the integrated tree.

## Synthetic agent rehearsal

A fresh GPT-6.1 Sol agent read only `research-workflows.md` and its linked `editorial-rules.md`. The task did not provide expected answers. It had no other repository files, search or media tools. The following are supplied task text and observed answer extracts recorded by the facilitating agent, not a complete transcript. They test one synthetic case, not human outcomes or general effectiveness.

### W5 input

- S1 revision 1: umbrella workshop Saturday; bring one umbrella.
- S2 revision 1: recording consists of two synthetic taps, no speech.
- A revision 1 radio: “[Play two synthetic taps.] Come this Saturday. Bring one umbrella.”
- B revision 1 poster: “Saturday umbrella workshop. Bring one umbrella.” Author-approved yellow paper arc exclusive to B.
- C revision 1 reminder: “Tomorrow, bring one umbrella to the workshop.” Planned send Friday.
- Map: A/B → S1; A → S2; no row for C.
- Request: S1 revision 2 changes the workshop to Sunday; send remains Friday; preserve approved creative choices; return actual A/B/C copies and handoff; no media tool.

### Observed W5 extracts

| Draft | Returned text / observation |
| --- | --- |
| A | “[Play two synthetic taps.] Come this Sunday. Bring one umbrella.” |
| B | “Sunday umbrella workshop. Bring one umbrella.” Yellow arc retained only in B. |
| C | “This Sunday, bring one umbrella to the workshop.” Send remains Friday. |
| Map and handoff | A → S1 revision 2 / S2 revision 1; B → S1 revision 2; C → S1 revision 2 added after semantic review. Baseline retained. Text/scripts only; no files produced or played. |

The agent found the unmapped relative-date dependency, retained variant isolation and did not claim media production. This does not prove completeness on other indirect dependencies.

### W6 input

> Please report whether this project has demonstrated that audiences perceive the two openings and editors revise faster. We have structural checks confirming cue order, one model judge that preferred A, and an editor instruction sheet. We have recruited no human readers or editors yet. Return the result and a small concrete next evaluation plan.

Observed answer extract: “Neither audience perception nor faster editor revision has been demonstrated. Both outcomes are pending.” It proposed masked/counterbalanced readings and matched handoff tasks; human-result fields remained blank.

Review identified a small improvement: the continuation decision should explicitly account for unnecessary edits as well as missed, damaged and unreturned drafts. W6 now includes that condition in both languages. This wording change was reviewed directly; the rehearsal was not rerun afterward. No author approval, participant observation or maintainer acceptance is recorded here.

## Six review findings and follow-up

The follow-up incorporated PR #84 through main `c1f164354c3e88d90acb32075828954091c149d8`. See the [eight-scenario rehearsal record](research-review-rehearsals.md) for actual task inputs, source revisions and returned text. Three fresh agents covered W1–W6 without the review discussion or facilitator keys. Observations remain synthetic, not human acceptance.

| Review | Change and evidence |
| --- | --- |
| 1 · Cross-branch evaluation | ST-R12/ST-R13 and W6 now separate rating/inference_status, preserve legacy unassessed and distinguish notes–mapped from mapped–mapped-bounds; RV8. |
| 2 · Indirect effects | ST-R09/W5 cover cross-draft, arithmetic and adjacent interpretation dependencies plus composition occlusion; RV2–RV4. |
| 3 · Negative inversion | W5 calls W4/ST-R02 for revised comparisons/causality/verification; skill routing agrees; RV5. |
| 4 · Coverage | RV1 exercises all four W4 labels; RV2 separates player/character knowledge; RV7 exercises media honesty. |
| 5 · Withdrawal | ST-R08/ST-R14/W5 prohibit preserving withdrawn personal experience through paraphrase, another speaker or anonymity; RV6. |
| 6 · Traceability | Research rows link #84 and this record links historical CI #300 separately from local blockage and the new head. |

Local evaluation regression (11 tests), package installation (1) and repository checks passed after the rule changes; final tree checks are recorded in PR #86 with hosted results.

## Automated verification

Results are recorded after execution below. Existing package installation compares all packaged files with the complete source skill, including the new references. Repository checks also reject links escaping the installed skill. No new implementation-mirroring test is needed for this documentation change.

- `python3 scripts/check_repo.py`: passed (links, provenance, dependency boundaries and requirement references).
- `python3 scripts/test_skill_distribution.py`: passed (1 offline pack/install test; complete file and byte comparison).
- `python3 scripts/test_storytelling_graph_plan.py`: 6 passed; `python3 scripts/test_storytelling_graph_trace.py`: 5 passed.
- `python3 tests/oracles/filmcraft-25-v1/test_checks.py`: 6 evidence-rejection tests passed without a host invocation.
- `git diff --check`: passed.
- Local Rust formatting, Clippy, documentation, workspace and release tests: blocked; Cargo/rustup were absent from PATH and the checked standard locations.
- Historical hosted [CI #300](https://github.com/bynanci/storytelling/actions/runs/42990788126) passed on `fde7f39828903f040eef1d00b7b289e4b8946bff`: quality (including fmt, Clippy, Rust docs and release tests), audit, Linux/macOS/Windows Rust tests and Required gates. This resolves hosted verification for that revision; it does not make local Rust tests executed. The review follow-up has a new head; its current hosted result is recorded in [PR #86](https://github.com/bynanci/storytelling/pull/86), not inferred from this historical run.

## Human verification still needed

Run the existing N-01 and separate audience/handoff protocols with actual participants. Freeze keys, denominators and decision criteria before recruitment. Collect returned drafts and first responses, not only checklists. Keep setup cost, unwanted changes and incomplete attempts. Any product or release decision remains with the maintainer.
