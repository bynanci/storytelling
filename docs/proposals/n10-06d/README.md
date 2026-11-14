<!-- SPDX-License-Identifier: MIT -->
# N10-06D · Dry-run story-to-video handoff checklist

English · [繁體中文](README.zh-TW.md) · [Issue #129](https://github.com/bynanci/storytelling/issues/129)

**Status: proposed evidence checklist / acceptance specification, with no runtime API or editor integration.** Preparation base `52dc6c315f7b3d20693f3c51e052509997ab791b`; PR status rechecked 2026-11-14 UTC with observed main `125e5b1bade5be2d406fc33de67773873d2a07f5`. This packet lets an author and reviewer expose missing evidence before a future editor handoff. It supplies no execution permission.

## Scope and current gates

Use the existing [NarrativeVideoCoverageV1](../../adr/0010-narrative-video-coverage-v1.md) and [V1 schema](../../../contracts/narrative-video-coverage-v1.schema.json). A's exact-input helper is a separate [PR #130](https://github.com/bynanci/storytelling/pull/130); B's advisory UX and C's trusted local media adapter have separate decisions. D records their evidence; it does not implement them or define another coverage validator.

At this check, [#103](https://github.com/bynanci/storytelling/issues/103) is Open and its technical G2 decision remains pending. [PR #117](https://github.com/bynanci/storytelling/pull/117), reviewed at `aa6b4cf6bfe86295ad43ad1813e8057130d6d1d1`, merged **proposal documents and design-integrity checks only** as `a23a2909cf3734d207c66b372f29a85c5e541c86` on 2026-11-14. Neither schema validation nor document merge accepts the authority contract. W01's deterministic/no-host contract implementation and an exact-tested-commit maintainer decision remain separate.

[#104](https://github.com/bynanci/storytelling/issues/104) is blocked on #103; [#105](https://github.com/bynanci/storytelling/issues/105) execution recovery is blocked on #104; [#106](https://github.com/bynanci/storytelling/issues/106) requires verified #104/#105, actual host evidence and a maintainer accept/narrow/stop decision. There is no generally accepted production FilmCraft profile. The [bounded M3-03 proof](../../validation/m3-03-filmcraft-execution.md) demonstrates one isolated import/readback and lost reply; it does not establish production supervision, current authority or every interruption path.

The current D outcome is **a review packet with unresolved blockers**. Do not add an Execute/export-ready action, call a host, create a new operation, compile an artifact, mutate a catalog/project, retry an unknown operation or issue Undo from this checklist.

## Five independent evidence lanes

A reviewer records each lane independently. `passed`, `failed`, `not_verified`, `unknown` and `blocked` are human checklist labels, **not new SDK enums or V1 diagnostics**. An untrusted JSON claim that a lane passed must be checked against its evidence. The form cannot create an unforgeable permit.

| Lane ID | What a passing observation would require | What it does not establish |
| --- | --- | --- |
| `declared_valid` | Exact input bytes/digest, Storytelling commit, actual `render-video-coverage` command/exit/output, current ordered `unit_id@revision`, catalog revision, asset/range declarations and preserved unresolved rows. | File existence, actual duration, pictured content, host support, approval or export. Valid unresolved input can pass declaration validity while `ready_for_planning=false`. |
| `media_probed` | C's separately accepted adapter binds each selected mapped `asset_id@revision` to the explicitly selected local file, verified byte fingerprint, bounded metadata/stream/timing evidence and range compatibility. Retain change/replacement/race results. | That the footage depicts a narrative unit or that an editor used those bytes. Nominal fps alone cannot resolve VFR frame timestamps. A missing/unreliably bound file stays `not_verified`/`unknown`. |
| `content_reviewed` | An actual author views the identified bytes and chosen range, records the exact unit/revision and what they confirmed, with anonymous reviewer code, date and evidence reference. Review scope and unresolved judgments are explicit. | File identity from a filename/thumbnail, organizational execution authority, aesthetic superiority or acceptance of unseen frames/audio. Metadata/hash checks cannot fill this lane. |
| `host_qualified` | Exact accepted host/profile/build/OS capabilities and losses, tested target/project identity and current revision, plus applicable import/readback/save/reopen evidence accepted through #104–#106. | That a particular operation was applied or is presently authorized. A proposed profile or successful old isolated proof is insufficient. |
| `authority_accepted` | An accepted #103 boundary and trusted current provider/policy record binding the exact reviewed/compiled artifact, target revision, host, operator approval and operation identity; validity checked at the actual dispatch boundary by its future executor. | Human identity or current authorization from mathematical signature validity alone. A declaration or checkbox cannot supply this record, consume approval, claim ownership or dispatch. |

Also preserve the **review-to-compilation binding** independently of signature and authority: selected proposal, source snapshot, compiler/profile/version, exact artifact bytes and accepted-loss report must agree with the reviewed identity. This checklist can cite an existing verified record; it does not create that binding by accepting caller-supplied hashes. Missing compilation evidence is `not_verified` and blocks a future execution review even if declaration and signature checks passed.

A signature may be mathematically valid under a supplied key while the signer lacks current trusted authority. A trusted approval may exist for an artifact different from the actual compiled proposal. Keep these questions separate, including their source and date.

## Manual dry-run procedure

1. Copy [`checklist.blank.json`](checklist.blank.json) to the facilitator's authorized private workspace. Assign an anonymous case/reviewer code; record source commit, date, source/variant revisions and the exact selected unit order. Leave unknown fields null and keep every selected unit visible.
2. Run only the already supported declaration check on the supplied structured input. Save its actual input/output and exit privately; put opaque receipt IDs and hashes in the checklist. Copy `ready_for_planning` and unresolved references without converting them into execution status. Do not type a fabricated success receipt into the form.
3. For every mapped row, list the original IN/OUT unit, canonical exact seconds, declared asset revision/duration and separately supplied C evidence. Without accepted C and actual authorized observations, mark `media_probed` `not_verified`; no path or file access is inferred from the mapping. Preserve unresolved rows/reasons; a missing shot cannot be replaced with a guessed clip.
4. Ask the author to view only explicitly authorized identified material when it is available. Preserve the author's actual words and uncertainty; record `content_reviewed` only for the identified unit/range/bytes. This packet itself performs no viewing or recruitment.
5. Check the host/target qualification and authority/compilation records independently. With current #103–#106 gates, keep these lanes `blocked`. Do not interpret the existence of proposed schemas, a signed identity, or a compiled OTIO as authority or host acceptance.
6. Produce a blocker list with owner, missing evidence, affected unit/asset/revision and condition for resuming. A complete checklist is a **review packet**, not an execution instruction. D has no dispatch action even in a hypothetical future packet with five passing lanes.

Only explicit manifest reads and existing authorized observations belong here. Media reads, metadata probing, external commands, target inspection and protected-authority queries require their own accepted surface and local permissions. Keep private paths, filenames that identify people, media, raw sensitive evidence and signer details outside public commits; use opaque evidence IDs in public examples. The form is a proposed local manual record, not an accepted portable wire format.

## Changes invalidate evidence at its stated scope

| Change | Required response |
| --- | --- |
| Unit/revision, selection order, mapping, cut, catalog revision or declared duration changes | Rerun unchanged V1; retain the old input/result as historical. Reassess compilation/review binding and every affected media/content lane. |
| File bytes, asset revision, observed stream/timestamps, local-file identity or accepted conversion loss changes | Invalidate the affected media/content evidence and review/compilation binding. Keep the earlier evidence; require a fresh authorized observation and author decision. |
| Host/profile/build/OS, target project identity or target revision changes | Mark host qualification for that target `not_verified`; invalidate target-bound review/approval evidence until rechecked. |
| Approval expiry/revocation, policy/provider or execution identity changes | Authority is `unknown`/`blocked` until the accepted provider supplies a current record. A prior signature or approval does not refresh it. |
| Any prior operation has `unknown`/`partial` outcome, active/queued work or unresolved old owner | Preserve quarantine and block a new mutation regardless of fresh operation ID or approval. Use the accepted recovery gate; do not resend or Undo. |

Do not overwrite an earlier observation with a late reply or update its source revision in place. Append a new dated checklist revision and preserve the original receipts. D does not add state transitions to the SQLite journal.

## Acceptance cases and reuse of recovery design

[`acceptance-cases.json`](acceptance-cases.json) freezes synthetic expected checklist outcomes. [`checklist.synthetic-reference.json`](checklist.synthetic-reference.json) is a filled teaching example from the existing synthetic two-unit fixture; it is not an observed run. Its declaration expectation is valid/unresolved, every actual-observation field remains unverified, and it is blocked. The blank form's actual-result fields remain null.

| Case | Expected review outcome / evidence required |
| --- | --- |
| D01 · valid declaration with one unresolved unit | Preserve the missing unit and reason; declaration validity may pass, planning stays false, the handoff remains blocked. |
| D02 · complete declaration, no file evidence | Show declaration-only readiness, `media_probed=not_verified`, and no verified-media or export label. |
| D03 · changed bytes under the same asset ID | Previous hash/content confirmation cannot be reused; media/review evidence becomes stale and the packet stays blocked. |
| D04 · valid signature without current trusted authorization | Record mathematical validity only; authority remains blocked. |
| D05 · proposal A reviewed, artifact B supplied | Do not promote caller hashes to compiler binding; block on the mismatch even if the signature verifies. |
| D06 · changed host/profile/target revision | Host/target and target-bound review/approval evidence must be rechecked. |
| D07–D10 · lost reply, partial state, missing quiescence, late old reply | Preserve unknown/partial and quarantine, no resend, Undo or automatic retry. |
| D11 · hypothetical future complete evidence | Label only `review_packet_complete`; D still has no execution action. Actual future execution remains a separate accepted boundary. |

Reuse [PR #117's existing cases](https://github.com/bynanci/storytelling/blob/aa6b4cf6bfe86295ad43ad1813e8057130d6d1d1/tests/fixtures/execution-contract-v1/cases.json): EX14 (fresh approval cannot clear unknown), EX16 (lost reply plus complete correlated readback), EX19 (partial), EX20 (baseline plus positive quiescence), EX21 (late owner/attempt), and EX37 (changed actual source bytes). These remain proposed runtime assertions; the Python design-integrity checker supplies no host/recovery execution. D does not duplicate that engine or mark those tests implemented.

For a timeout/lost reply, list the last durable operation/attempt receipt and exact immutable identity; mark the outcome unknown. The future accepted #105 recovery path must establish correlated readback and positive prior-request quiescence before classifying applied/not_applied. Readback that merely resembles the expected effect while queued work may still run is insufficient. Verified not_applied may permit only a new explicit operator action under the accepted retry policy, fresh review/source/authority and remaining attempt budget; never automatic resend. Current D cannot perform those actions.

## Verification, decisions and stop conditions

This delivery changes documentation and synthetic forms only. JSON parsing, local-link/repository and whitespace checks validate the packet structure; no Rust behavior changed, so no behavior RED/GREEN is claimed. Record reviewer findings separately from any later author/media/host evidence. Neither this packet nor its merge closes #103–#106/#129 or supplies a G5 decision.

Accept this packet only if reviewers can identify each missing lane and affected unit, distinguish declaration validity from missing coverage, preserve the original source/range/decision, and explain why a signature or stale receipt cannot enable an operation. Stop/narrow if a lane is inferred from another, a missing row is filtered, a private path is exposed, an unknown operation can be retried, or a dry-run label implies host execution. A maintainer must separately accept the public consumer surface (#107–#109), C's contract/privacy scope, the #103/W01 tested contract, #104/#105 behavior and #106 host evidence. Independent author use remains unobserved; no usability or time-saving result is claimed.
