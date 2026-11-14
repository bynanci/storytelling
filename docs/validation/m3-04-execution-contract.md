<!-- SPDX-License-Identifier: MIT -->
# M3-04 execution contract: design, test cases and implementation handoff

Date: 2026-11-13. Base: `48c1ca005c0087a585f2028d59ebb45fcdab5a46`.
Status: **design for G2 review; no G2 acceptance or production executor**.
[中文審查摘要](../migration/EXECUTION_CONTRACT_V1.zh-TW.md) ·
[ADR 0011](../adr/0011-production-execution-contract-v1.md) ·
[normative proposal](../migration/EXECUTION_CONTRACT_V1.md).

## Baseline audit

Issue #103 is open/ready, #104 is open/blocked on #103, and there were no open
PRs at the baseline audit. This is a new coherent design branch. The two related
issues had no comments/acceptance record at the time checked. These facts do
not describe unmerged/private work that was not visible to the audit.

| Existing file / API | Reuse and evidence limit |
| --- | --- |
| `crates/storytelling-core/src/review.rs` | `ReviewContextV1` already declares project, target revision and host profile/binary/capabilities/losses. Portable records remain untrusted; all executable/approval/verification flags stay false. |
| `crates/storytelling-engine/src/review_binding.rs` | `BoundReviewV1`, `prepare_review`, `check_review` own compilation bytes and freeze P1 identity/freshness. Preserve digest vectors. |
| `crates/storytelling-engine/src/story_review.rs` | `BoundStoryReviewV1`, `prepare_story_review`, `check_story_review` bind the actual selected proposal to compiled OTIO. Reuse rather than compiling an independent execution artifact. |
| `crates/storytelling-core/src/approval.rs` | `VerifiedApprovalV1` checks Ed25519 against caller-provided expected key/identity. Does not authenticate current permission or prove compilation. PR #28's independent G2 remains pending. |
| `crates/storytelling-journal/src/lib.rs` | `OperationIntentV1`, `Journal::{prepare,claim,renew,begin_attempt,record_response,record_readback}` already provide bounded durable operation state, attempt/fence checks and idempotency. Project-level exclusion, authority consumption and verified readback are missing. |
| `crates/storytelling-journal/tests/journal.rs` | Eleven accepted journal regressions establish local synthetic persistence behavior. Caller-provided executor/time/readback are not actual authority or host truth. |
| `crates/storytelling-journal/tests/filmcraft_execution.rs` | Ignored manual proof: FilmCraft 0.18.0 Linux, one isolated synthetic import/readback. Child exits before reply is withheld; no production process supervision/safe retry/current authorization. |
| `tests/oracles/m3-03-filmcraft-v1/` | Native project/readback and exact ranges corroborate the bounded proof. Review/artifact digests survive in journal evidence; native host does not independently authenticate them. |
| `crates/storytelling-cli/src/main.rs` | Current public commands are read-only. No production executor command is being added by this PR. |

Read the [M3-02 validation](m3-02-operation-journal.md) and
[M3-03 validation](m3-03-filmcraft-execution.md) for actual source/host hashes,
observed results and platform limits. Reuse their records without enlarging them.

## Requirement and test design

The [case inventory](../../tests/fixtures/execution-contract-v1/cases.json) is a
machine-readable **expected-behavior design**. Its named fake scenarios stand in
for trusted runtime observations; they are not authority assertions accepted
from request JSON. `valid-request.json` uses fabricated, labeled synthetic
digests, not a real compilation/host/authorization receipt. The identity vector
only tests the proposed framing. The schema constrains shape, not freshness,
authority, host truth or effects. Do not count these files as passing product
contract tests or mark #103's runtime-test criterion complete.

| Proposed requirement | Cases | Required future assertion |
| --- | --- | --- |
| M3-EX01: immutable P2-derived identity | EX01, EX03, EX27 | Same bound inputs give exact digest/bytes; changed source/cut/loss/proposal/context becomes stale; tampered digest rejects. Private construction/deserialization/mutation compile-fail checks. |
| M3-EX02: current authority and replay | EX02, EX06–EX10 | Trusted local provider and current explicit approval required; expiry/revocation/policy and consumption serialize before dispatch. Neither valid nor invalid PR #28 signature grants execution. |
| M3-EX03: actual target/profile checks | EX04–EX05, EX29, EX37 | Fresh declarations alone are insufficient; compare actual host/profile/binary, sources and target revision under canonical project guard; no implicit target provisioning. |
| M3-EX04: durable one-writer/idempotency | EX11–EX13, EX21–EX22 | Exact completed replay has zero new calls; changed identity rejects; distinct operations/aliases share target exclusion; late owner/attempt cannot overwrite; storage failure has no partial commit/call. |
| M3-EX05: unknown/quiescence/retry | EX14–EX16, EX20, EX23 | Unknown quarantines target across restart; unchanged state with an active prior host stays unknown. Lost reply resolves by readback without resend. #104 never retries; #105 needs verified not_applied plus quiescence/new authority. |
| M3-EX06: actual readback/partial | EX17–EX19, EX28 | Success exit alone is reported/unknown; incomplete/wrong-target/source/range/attempt evidence or still-active work cannot prove final applied; verified partial remains terminal/manual. |
| M3-EX07: strict deterministic wire | EX24–EX26, EX30–EX36 | Boundaries/duplicate/null/unknown/version cases have exact code/path and first-error precedence; input and source/target sentinels unchanged. |
| M3-EX08: compatibility and non-mutation | All cases plus existing regressions | Preserve P1/P2/approval vectors, journal V1, narrative/video/OTIO/CLI behavior; no production host invoked by default tests. |

For every future admission failure, assert code **and path**, no partial permit,
`fake_host.mutation_calls == 0`, unchanged catalog/source/target sentinels and
no consumed approval/dispatch marker. Pure validation also performs zero readback
or process calls. Lifecycle cases use a fake host, fake clock and trusted fake
authority installed by the test composition root. Record fake calls, attempted
markers, both fences, before/after SQLite rows, process/readback ordering and
evidence digests. All timeout/cancellation/partial cases assert zero automatic
retry and zero Undo. Inject crashes at every storage/dispatch/evidence boundary.

The production journal test must run competing connections/threads on the same
temporary database and alias-resolved resource. Include old-child-still-running
after lease expiry and restart; changing operation ID must not clear quarantine.
Test expiry at the exact bound, revoked policy racing dispatch commit, disk/busy
failures, corrupt/missing private evidence and per-operation exhaustion. Separate
OS power-loss tests and actual host acceptance remain manual future evidence.

## Implementation packages after G2

All names below are **planned**, not existing APIs/tests. Keep the four existing
crates. No new runtime/editor crate or placeholder is needed for this design.
This delivery stops at design. A separately authorized, non-mutating contract
implementation/test PR (W01) must supply #103's deterministic/no-host evidence
before final issue acceptance. Endorsing this design alone does not unblock #104.

| Package / dependency | Concrete proposed files | Acceptance / commands after implementation |
| --- | --- | --- |
| EX-W01 · pure contract evidence for #103, after design review and separate authorization | New `crates/storytelling-core/src/execution.rs`; expose DTOs/diagnostics from core `lib.rs`. New engine `src/execution.rs`, exports in `lib.rs`; reuse P2 internals rather than change their digests. New engine `tests/execution_contract.rs`, core doc compile-fail examples. | RED from cases and opacity tests first; GREEN deterministic identity/strictness; `cargo test --locked -p storytelling-engine --test execution_contract` and `cargo test --locked -p storytelling-core --doc`. |
| EX-W02 · production storage under #104, after final #103 acceptance and W01 | Refactor reusable journal transaction helpers in `crates/storytelling-journal/src/lib.rs`; add `src/production.rs` for new schema and opaque project/attempt handles; `tests/production_journal.rs`. Keep V1 database/API behavior intact. | RED cross-operation race, alias/restart/quarantine and atomic grant consumption tests; `cargo test --locked -p storytelling-journal --test production_journal`. |
| EX-W03 · #104 single attempt, after final #103 acceptance, W01/W02 and G2-reviewed authority/candidate-profile interfaces | CLI `src/execution/{mod.rs,authority.rs,filmcraft.rs}` and main command composition; CLI manifest/lockfile and `scripts/check_repo.py` dependency allowlist updated only as required to connect journal/core. CLI `tests/execution_cli.rs` with fake host/provider; no public unchecked permit constructor. | One normal explicit entry, marker before fake call, exact owned artifact, trusted local approval, readback-backed result. `cargo test --locked -p storytelling-cli --test execution_cli`. Missing trusted authority/provider or profile keeps mutation disabled. Candidate-profile isolated verification needs separate authorization; production activation waits for #105/#106 and maintainer acceptance. |
| EX-W04 · #105 recovery, after #104 | CLI/runtime supervision/reconciliation modules and journal recovery tests; versioned evidence receipt/parser. | Controlled in-flight timeout, descendant/queued-work quiescence and explicit safe retry evidence. Cannot infer these from the old lost-reply proof. |
| EX-W05 · #106 host acceptance, after #104/#105 | Extend the isolated FilmCraft oracle and host-profile records under a separately authorized unique synthetic root; do not change old evidence. | Actual pinned-host profile, source checks, save/reopen/projection, no duplicate effect, platform/process matrix. No user-project test. |

Dependency order: reviewed design → separately authorized #103 pure contract
tests (W01) → final G2/#103 acceptance → #104(W02/W03) → #105(W04) → #106(W05).
A design-only approval cannot close #103 or satisfy its runtime-test criterion. #104's verified single-attempt candidate does not imply
production acceptance; #109/#111/#112 retain their own gates. PR #28 security
review is independent; V1 does not adopt that candidate as its authority mode.

## Validation commands and evidence

Current design checks (no host):

```sh
python3 tests/fixtures/execution-contract-v1/check_design.py
python3 scripts/check_repo.py
git diff --check
```

The first script checks design-file integrity, exact framing vectors, fixture
inventory and raw duplicate preservation only. It is not an execution validator.
For existing regression scope, use:

```sh
cargo test --locked -p storytelling-core --test approval
cargo test --locked -p storytelling-engine --test review_binding
cargo test --locked -p storytelling-engine --test story_review
cargo test --locked -p storytelling-journal --test journal
cargo test --locked -p storytelling-cli --test review_cli
cargo test --locked -p storytelling-cli --test story_review
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked
```

Default workspace tests leave the manual FilmCraft test ignored. The future test
commands in W01–W04 do not exist yet; do not run them and report missing files as
RED. No new runtime behavior is implemented, so no implementation RED/GREEN is
claimed by this design PR. Local results are recorded in `m3-04-local-checks.json`;
hosted results must be read for the submitted head. Future implementation work must observe
actual test failures before implementing the new behavior.

## G2 acceptance

All items remain pending. A maintainer must name the exact ADR/profile/schema/
fixture commit and record the actual decision, scope changes and reviewer identity.
Codex may prepare evidence, not sign or accept for the maintainer.

- [ ] Accept the initial isolated-project action/profile boundary and complete intent field/digest vector.
- [ ] Accept trusted local current approval and the concrete provider's session, policy, revocation, time and protected-store trust boundary. Signed/offline grants remain excluded.
- [ ] Accept transactional approval consumption, operation and project fencing, alias identity, quarantine and new-store schema/rollback behavior.
- [ ] Accept readback projection/correlation/source-byte obligations, not_applied/quiescence evidence, all outcome/diagnostic/limit rules and #104 single-attempt scope.
- [ ] Review positive/negative cases and work packages; obtain passing deterministic no-host contract tests in the separate W01 PR before final #103 acceptance/unblocking #104. The design-integrity checker does not satisfy this criterion.
- [ ] Record independent review findings separately from self-review; record remaining FilmCraft/process/platform evidence under #105/#106.

Actual G2 decision: **pending**. #103 remains open. #104 remains blocked. No
real-project write, new host invocation, merged implementation or release is
authorized by this design record.

## Design review findings

A separate Codex agent reviewed the draft (AI review, not human/security G2).
The review identified five gaps, addressed before submission: freeze protected
result/receipt fields and digest framing; require quiescence before final applied
and guard release; make target provisioning explicit before review; split wire
cases into concrete expected diagnostics/paths; constrain multi-store claims to
the one configured registry/store. EX28–EX36 and the result schema preserve the
corrections. A second pass corrected request/result nullability, required ready
identity and unsuccessful diagnostics, and explicitly required outer receipt
identity to match the protected stored operation/attempt/profile. Human G2
acceptance remains pending.

Final review also distinguished candidate-profile testing from protected production activation, kept detailed P2 causes in private evidence to preserve the closed public error shape, and identified ready/dispatching as API-only intermediate outcomes.
