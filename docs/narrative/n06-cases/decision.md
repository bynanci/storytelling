<!-- SPDX-License-Identifier: MIT -->
# N-06 decision record: no native mode adopted

Requirement N-R06 / Issue #53 · G0–G2. This supplement provides concrete author-comparison materials after merged PR #85; it changes no Rust, accepted schema, host, media file or release state.

## Evidence considered

- Accepted N-02/N-04 source and variant contracts; current N-08 beat-order checks, N-11 read-only handoff and Planner fixed-window contract.
- The three [small cases](README.md) and their [normal CLI probes](evidence/probes.json). The probes establish current declared behavior and first refused extensions, not author benefit.
- The requester's [recorded decision](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714): **“暫緩三者，等實際需求”**.
- No task-specific author answers, independent reading, paired task observations, real video bytes or measured engineering implementation costs were supplied.

## Decision

**Continue to defer all three new native profiles. Recommend zero follow-up native features now.** The text material is sufficient to ask the next product question without extending accepted contracts. There is no evidence to rank author value among the candidates. The cost ranges are low-confidence planning assumptions and do not establish return on investment.

This preserves the already recorded deferral. It does not reopen N-10 or turn a design sketch into a wire contract. No native-feature task draft is opened because no mode is selected. N-07 is independent: its ordinary-notes/CLI pilot need not wait for N-06 and must not count these probes as participant outcomes.

## Architecture boundary if a mode is later adopted

| Owner | Responsibility to freeze at the next G2 |
| --- | --- |
| Core | A separate versioned profile and exact source/variant references; keep accepted NarrativeDocumentV1, N-04 ledger and existing video types readable. |
| Engine | Pure bounded validation/replay of declared state, chronology or mapping. Check dangling/stale references and explicit unresolved states; add no filesystem, model or host calls. |
| CLI / client | Explicit input/output and stable diagnostics. Collect an author's separate choice rather than upgrading an assistant suggestion to acceptance. |
| Author / host | Judge meaning and usefulness; provide actual media/authorization and readback evidence for any later editor execution. These are not inferred from parser success. |

No owner receives a new implementation task from this record. The current probes are [recorded separately](evidence/checks.json); they do not accept one of the proposed profiles.

## Conditions for a new decision

| Candidate | Evidence that would justify revisiting it | Bounded acceptance to define before implementation |
| --- | --- | --- |
| Brainstorm | An author demonstrates that a named text table loses pending questions or choices across an actual revision/handoff. | Both unresolved options/questions survive replay; zero or one explicit choice per mutual-exclusion group; zero retains the unresolved state; suggestions never self-accept. |
| Screenplay | An author needs typed occurrence/reveal roles or cross-view same-event/revision consistency beyond named variants and existing order checks. | One chronology/two-reveal example, cross-view event omission/addition and revision mismatches, preserved per-variant order/conflict regressions and unchanged occurrence facts; scene/dialogue scope explicitly included or excluded. |
| Video mapping | An author supplies selected units, a real declared asset catalog and ranges, including an actual missing-coverage case. | Complete positive map and explicit unresolved negative, exact rational bounds/revisions, no silent coverage drop, no host execution. Actual media/host proof remains separate. |

The requester/maintainer should then adopt **exactly one** mode, its author need and version scope in #53 or a linked successor decision before #67 implementation. The author must supply the task evidence; the engineer must estimate the selected bounded scope again. A recommendation alone is not adoption.

## Author-comparison record — no session yet

Give the author the three case sheets in alternating order where practical. Ask what they would actually make next, what they currently do in ordinary notes, the first failure they encounter and the smallest useful change. Do not lead them toward the lowest estimated engineering cost. Save their own wording and an anonymous label; do not infer a preference from the existing implementation.

| Anonymous label / date | Case order | Actual task attempted | First observed failure / no failure | Minimal change requested | Selected mode or explicit deferral |
| --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |

The only current answer is the prior global defer-all decision. The blank row is not a zero-duration trial or a successful completion. Keep any later participant timing with its own protocol and versions.
