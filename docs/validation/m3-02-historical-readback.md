<!-- SPDX-License-Identifier: MIT -->
# M3-02 historical readback receipt regression

[繁體中文](m3-02-historical-readback.zh-TW.md) · M3-JL02/JL04/JL06 · related [#105](https://github.com/bynanci/storytelling/issues/105)

Base: `91bf8f80139f59c0406340eb3565d296bd562931`. A separately prompted AI journal auditor identified this accepted-journal defect; the root agent reproduced it without a Host. The review is synthetic technical evidence, not a human or FilmCraft observation.

The journal compared a duplicate only with the latest attempt state/evidence. For one current lease/attempt, `Unknown(D) → NotApplied(E) → Ambiguous(F) → replay NotApplied(E)` therefore replaced newer unknown evidence, appended a receipt, consumed readback budget and permitted attempt 2. The accepted contract requires exact current-attempt receipt idempotency and blocks retry when newer readback conflicts. The [RED log](m3-02-historical-readback/red.log) actually reports `older evidence reopened retry: Ok(2)`.

The correction checks for an already recorded **readback** digest under the same operation and attempt, inside the existing immediate transaction and after validating current lease/fence/attempt. A known receipt returns the current snapshot without writes, including after a newer observation, restart, terminal result or an exhausted readback budget. A response digest alone does not suppress the first readback. Fresh distinct evidence follows the original state machine; only a fresh `NotApplied` can make another attempt available. No schema, public type, error, dependency, fence or limit changes.

## Tests and source receipts

Test-first source `c55d5b21d41ea3e47ccc606a066c9a302a4143c0`: 17 tests, 14 pass /3 fail, exit 101. Implementation `4dee6e88d2adb1cf4c3147340f5aa79f674e2e69`: [GREEN](m3-02-historical-readback/green.log), 17/17 pass. The four additions exercise:

- replay of an old NotApplied receipt across reopen, keeping retry blocked;
- eight valid first/nonterminal and latest-result combinations, preserving current state/history and the latest retry classification;
- nonconsecutive duplicates not spending the four-readback budget, including response/readback same-hash distinction;
- hash reuse across operations/attempts plus stale lease/attempt rejection before idempotency.

[Workspace command receipts](m3-02-historical-readback/workspace-checks.json) retain raw logs, exits, exact source and hashes. The Cargo target is dedicated to this worktree and was absent before its first RED build. Required hosted CI and any merge results are reported on the PR, not inferred from local tests.

## Evidence identity and compatibility limits

An evidence digest represents an immutable readback receipt in an operation/attempt. Replaying a historical hash cannot create a new observation by supplying a later caller timestamp or changed result argument. **Inspect the returned current snapshot**: success means the duplicate was handled, not that the newly supplied result changed state. A new observation needs distinct evidence bytes/digest containing the relevant new observation context; unchanged project bytes alone do not establish fresh quiescence or safe retry.

Schema v1 retains historical readback hashes but not their result subtypes. The correction therefore cannot prove whether a reused historical digest carries the same subtype; it treats consumed evidence as a no-op. It does not authenticate the supplied evidence, a caller's NotApplied claim, current authority, Host truth or absence of an in-flight mutation. These remain adapter/authority acceptance work in #103–#106. No Host call, retry, Undo or real project mutation ran here.

Existing databases need no migration. Previously recorded events are not rewritten, and this fix cannot reconstruct or repair a state already changed by a historical replay under old code. Operations suspected of that history need explicit operator review before further execution. Rolling back the lookup would restore the demonstrated replay defect; keep dispatch disabled for unresolved outcomes rather than using a downgrade as recovery.

An independent AI reviewer, prompted with base and implementation commits only, checked transaction ordering, context scoping, limits and state transitions. Any findings and exact-head/post-merge checks are recorded on the PR. Formal G2, production M3 acceptance and G5 remain separate.
