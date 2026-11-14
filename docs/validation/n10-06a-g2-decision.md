<!-- SPDX-License-Identifier: MIT -->
# N10-06A · Draft-only G2 decision

- **Decision date:** 2026-11-14 UTC; exact time not recorded here.
- **Source:** requester response in the active Codex conversation for [Issue #129](https://github.com/bynanci/storytelling/issues/129).
- **Accepted candidate commit:** `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf`.
- **Base main:** `52dc6c315f7b3d20693f3c51e052509997ab791b`.
- **Original decision scope:** focused N10-06A draft-only G2; G3 RED/GREEN implementation authorized; PR remained Draft at that stage. The later merge authorization below updates only the review/merge restriction.

The requester selected this response verbatim:

> 接受此 G2 候選，授權 A 的 RED／GREEN 實作，保留 Draft（建議）

This records the response given for the exact candidate above. It is not a signature, DCO, independent review or execution-authority grant. No person's name, organizational role or signing identity is inferred from the response.

## Accepted implementation basis

The decision accepts the candidate [ADR 0012](../adr/0012-draft-exact-video-range-input.md) and its G1/G2 packet at the accepted commit as the basis for a focused A implementation:

- The `n10-06a.1-draft` wrapper, bounded exact decimal grammar, explicit constant-rate frame inputs and canonical reduced u64 rational output.
- The disabled-by-default `draft-video-input` feature, candidate engine JSON API and opt-in `draft-video-coverage-input` CLI/output.
- The helper diagnostics and ordering, checked integer/GCD algorithm, strict JSON boundary, original-input preservation and unchanged final V1 validation authority.
- The frozen positive/negative synthetic acceptance vectors, followed by observed RED, implementation, GREEN and final-head regression/CI evidence.

The grammar, fields, API, output and diagnostics at that candidate remain the reviewed implementation target. A material change to them requires a revised G2 decision. The [implementation record](n10-06a-exact-input.md) supplies the subsequent RED/GREEN and automated G4 receipts. This G2 decision itself supplies no passing implementation or human result; every test/CI receipt identifies its own tested commit.

## Gates that remain separate

The draft helper is not the stable SDK facade required by #107. B's richer UX, C's local-file/media contract and privacy review, and D's dry-run checklist retain their separate dependencies. No media access or file-identity proof is authorized by this G2 response.

#103 / PR #117 design proposal and #104–#106 retain their pending production authority, executor, recovery and host-qualification gates. There is no FilmCraft mutation, Agent self-approval, automatic retry of an unknown outcome, host/readback acceptance, independent author observation or G5 release evidence in this decision. The original implementation task prohibited automatic merge; the later authorization below permits merging completed, verified engineering work after current-head review and CI.


## Later continuation: verified engineering merge authorization

After receiving the implementation report and CI results, the requester gave this instruction verbatim in the active conversation:

> 繼續任務並想辦法解決所有issues 與pr 最終驗收完merge

This authorizes progressing completed, verified PRs through review and merge. For N10-06A, the earlier requirement to keep PR #130 Draft is superseded once the current head passes required checks and review. The implementation head `59516f24f4d829a3d695c115d2f94e208a78cdea` passed all six jobs in [CI run #483](https://github.com/bynanci/storytelling/actions/runs/43538270184); later head changes need fresh checks.

This continuation does not supply a technical G2 acceptance or W01 authorization for #103 / PR #117, an independent author session, media identity or metadata evidence, host qualification/readback, current execution authority, or a G5 publication decision. Issue #129 remains open for B/C/D and its outstanding human evidence. No signature or organizational role is inferred from the request.
