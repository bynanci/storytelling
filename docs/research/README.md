# Research, rules and skill design

English · [繁體中文](README.zh-TW.md) · [Project rationale](../WHY_STORYTELLING.md)

Storytelling keeps **one portable skill with six on-demand workflows**, supported by 14 stable editorial rule IDs. Research motivates questions and safeguards; local evidence must establish usefulness. This delivery adds Markdown guidance, not runtime behavior, a new schema, a release or human acceptance.

## Read or contribute

| Need | Canonical document |
| --- | --- |
| Understand the evidence and access limits | [11 papers and one standard](../../skills/storytelling/references/research-foundations.md) |
| Apply or review a rule | [ST-R01–ST-R14](../../skills/storytelling/references/editorial-rules.md) |
| Draft, compare, adapt, audit, revise or evaluate | [W1–W6](../../skills/storytelling/references/research-workflows.md) |
| Load the portable package | [Skill entry point](../../skills/storytelling/SKILL.md) |
| Check delivery evidence | [Validation record](../validation/research-rules-skills.md) |

Contract, editorial and human checks have different authority. A preference can be declined. A passing declaration check cannot prove factual support, identity, permission, aesthetics or audience response. The portable references are canonical; `rules/` contains discovery pointers rather than a second ruleset.

## Research → rule → workflow → existing work

Review follow-up incorporated main `c1f164354c3e88d90acb32075828954091c149d8`, which merged [PR #84](https://github.com/bynanci/storytelling/pull/84) at source head `ec4a879ccbf2ea1b04288d2f0fbec09b94f8dcce`. W6 now follows its independent `rating`/`inference_status` axes and separate mapped–mapped-bounds handoff comparison. W5 covers arithmetic, adjacent interpretation, consent withdrawal and W4 re-audit. These are synthetic validation and protocol changes, not participant outcomes.

Snapshot: 2026-11-11, base `b7fcf869d01727dcf408c457fdd42929dca80c7a`. This table maps design work, not live issue status. N-03 rendering, N-04 direct-reference impact, N-08 declared spoken order and N-11 handoff have repository implementations; their human outcomes remain separate. At initial review PRs #78 (audio cues) and #83 (commercial evidence audit) were open. During delivery, #78 merged as `60e302e2ea3b235e9bd7aae58049690ec9dad0f2`; this branch incorporates it without changing its runtime files. N-09 now has declared cue-sheet checks; produced sound and human outcomes still need separate evidence. Recheck #83 before implementation.

| Evidence | Rule IDs | Workflow | Requirement / existing issue | Next evidence needed |
| --- | --- | --- | --- | --- |
| P01 event indexing; P02 intentional planning; P06 outline control | ST-R04–ST-R06 | W1 intent; W2 alternatives | N-R03 / [#50](https://github.com/bynanci/storytelling/issues/50); N-R06 / [#53](https://github.com/bynanci/storytelling/issues/53); N-R10 / [#67](https://github.com/bynanci/storytelling/issues/67) | Compare a reveal, open brainstorm and missing video asset before selecting another native profile. |
| P03 narrative visualization; P09 multimedia learning | ST-R06, ST-R07, ST-R12 | W2; W3 adaptation; W6 evaluation | N-R08 / [#65](https://github.com/bynanci/storytelling/issues/65); N-R09 / [#66](https://github.com/bynanci/storytelling/issues/66) | Blind first-reading observations and separate audio production checks; retain access needs. |
| P04 CoAuthor; P08 human-AI interaction; S01 PROV-DM | ST-R01, ST-R03, ST-R08, ST-R11, ST-R14 | W1; W3; W5 revision | N-R04 / [#51](https://github.com/bynanci/storytelling/issues/51); N-R11 / [#68](https://github.com/bynanci/storytelling/issues/68); N-R12 / [#69](https://github.com/bynanci/storytelling/issues/69) | Actual author choices and inspected handoff artifacts; declared provenance does not grant rights. |
| P07 atomic factual support | ST-R01, ST-R02, ST-R10 | W4 claims audit | N-R05 / [#52](https://github.com/bynanci/storytelling/issues/52), related PRs [#83](https://github.com/bynanci/storytelling/pull/83) and [#84](https://github.com/bynanci/storytelling/pull/84) | Reviewer checks each proposition against the passage, including qualifiers and missing support. |
| P05 visual story revision; P11 build-system analogy | ST-R09, ST-R10, ST-R13 | W5; W6 | N-R04 / [#51](https://github.com/bynanci/storytelling/issues/51); N-R07 / [#54](https://github.com/bynanci/storytelling/issues/54); [#84](https://github.com/bynanci/storytelling/pull/84) source limits/calibration | Compare actual edits, indirect effects and setup cost against ordinary notes. |
| P10 model-judge biases | ST-R12, ST-R13 | W6 | N-R01 / [#48](https://github.com/bynanci/storytelling/issues/48); N-R07 / [#54](https://github.com/bynanci/storytelling/issues/54) | Actual human observations, balanced order and explicit denominators; model rehearsal is synthetic. |

## Priority and stopping decisions

1. **Use the existing contracts.** Apply W1–W5 to current drafts, preserve source revisions and author choices, and keep semantic judgments separate. Reuse #52/#65/#66/#68 rather than opening duplicate implementation issues.
2. **Collect human evidence before expanding runtime.** Run N-01 and the separate reader/handoff exercises under their own frozen protocols. Record completion/attempts, A/B first-focus ratings and separately supported/unsupported/unassessed inferences, missed/expected, unnecessary/unaffected received, damaged/received and unreturned/requested drafts. Keep notes–mapped and mapped–mapped-bounds comparisons separate, with damaged/received drafts primary for the latter. Report elapsed time separately from setup/map/boundary creation. Zero denominators are not applicable.
3. **Select at most one next native form through N-06.** Advance #67 only after a candidate is selected. Keep #69's production-adapter decision separate from its narrow feasibility trial.
4. **Split packages only when warranted.** A workflow needs a distinct trigger, self-contained references, independent synthetic evaluation and a maintainer before becoming another skill. Do not add installations or paid dependencies to ordinary Markdown use.

Freeze continue/narrow/revise/stop criteria before recruitment. The example protocols' thresholds are local pilot choices, not published effect sizes. Reconsider the design when ordinary notes are sufficient, mapping cost outweighs benefit, or omissions, unnecessary edits, damage or missing outputs worsen. Do not derive completion percentages from documentation checkboxes.

## Open-source boundary

The original guidance and synthetic examples retain the repository's MIT license. Existing Apache components and notices retain their own coverage. Citations grant no right to redistribute paper PDFs, figures, datasets, model weights or code. Record origin, revision, license and notices before any future import. This delivery adds original summaries and links only; it changes no license.

The npm file patterns already include `references/*.md`, so English and Traditional Chinese references travel with the existing package. All local links in the copied skill remain inside that folder; repository-only protocols use public GitHub URLs. Verify the offline pack/install test and repository link check. No personal skill installation, registry publication or release tag is part of this design.

## Evaluation layers

- **Automated:** repository links/provenance boundaries, complete offline package installation and existing graph helpers. Rust checks remain separate even for a documentation change.
- **Synthetic agent rehearsal:** exercise indirect revision, variant isolation, artifact honesty and absence of human observations. Record the input and observed behavior; do not count a model as a participant.
- **Human:** observe the intended language, audience and finished medium; keep author and recipient roles distinct. No results means pending.
- **Maintainer:** decide scope, acceptance and release from those records. A bibliography or passing CI supplies no approval.
