<!-- SPDX-License-Identifier: MIT -->
# N10-06A · Draft-only G2 decision

- **Decision date:** 2026-11-14 UTC; exact time not recorded here.
- **Source:** requester response in the active Codex conversation for [Issue #129](https://github.com/bynanci/storytelling/issues/129).
- **Accepted candidate commit:** `f148b6d1ad0883725c0dbe5fec601f6178f5dfdf`.
- **Base main:** `52dc6c315f7b3d20693f3c51e052509997ab791b`.
- **Decision scope:** focused N10-06A draft-only G2; G3 RED/GREEN implementation authorized; PR remains Draft.

The requester selected this response verbatim:

> 接受此 G2 候選，授權 A 的 RED／GREEN 實作，保留 Draft（建議）

This records the response given for the exact candidate above. It is not a signature, DCO, independent review or execution-authority grant. No person's name, organizational role or signing identity is inferred from the response.

## Accepted implementation basis

The decision accepts the candidate [ADR 0012](../adr/0012-draft-exact-video-range-input.md) and its G1/G2 packet at the accepted commit as the basis for a focused A implementation:

- The `n10-06a.1-draft` wrapper, bounded exact decimal grammar, explicit constant-rate frame inputs and canonical reduced u64 rational output.
- The disabled-by-default `draft-video-input` feature, candidate engine JSON API and opt-in `draft-video-coverage-input` CLI/output.
- The helper diagnostics and ordering, checked integer/GCD algorithm, strict JSON boundary, original-input preservation and unchanged final V1 validation authority.
- The frozen positive/negative synthetic acceptance vectors, followed by observed RED, implementation, GREEN and final-head regression/CI evidence.

The grammar, fields, API, output and diagnostics at that candidate remain the reviewed implementation target. A material change to them requires a revised G2 decision. G3 is now in progress; this record supplies no passing implementation or G4 result. RED/GREEN and CI receipts must identify their own tested commits.

## Gates that remain separate

The draft helper is not the stable SDK facade required by #107. B's richer UX, C's local-file/media contract and privacy review, and D's dry-run checklist retain their separate dependencies. No media access or file-identity proof is authorized by this G2 response.

#103 / Draft PR #117 and #104–#106 retain their pending production authority, executor, recovery and host-qualification gates. There is no FilmCraft mutation, Agent self-approval, automatic retry of an unknown outcome, host/readback acceptance, independent author observation or G5 release evidence in this decision. The current task prohibits automatic merge; the PR remains Draft for review.
