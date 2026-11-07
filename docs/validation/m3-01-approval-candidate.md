<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# PR #28 approval-identity candidate validation

Status: implemented candidate, not accepted. Canonical M3-01/P1 is a separate
proposed finite-OTIO review-binding task. After dependency PRs #8, #21, #10 and #9
merged, current main is `fefb038ec1699727da2c0f436a097bbdedcfd08a`. Synchronized
head `9670be947535a3e15b70280fd7500a83b373d55f` passed both same-head runs `42377965601` and `42377961928`, including
Quality, Required, Linux, Windows, macOS and hosted Cargo Audit. Cargo Audit 0.22.1
loaded 1,422 advisories and scanned 40 dependencies without findings; the scanned
lockfile SHA-256 is `f3a69dd97731666785474156249bd1c27d2d1c3048162edfeb111e98024d7e66`. Independent G2 security review remains pending.
The evidence reconciliation in this commit requires fresh exact-head checks before
merge; passing checks or merging will not grant G2 or product acceptance.

The initial PR head `d321049add005363618f58451b913301eb6d1e96` passed hosted
CI run `42347452265` on all five jobs. Main advanced via PR #26 before review;
the candidate branch now includes that main, so the old run is not evidence for
the refreshed head. The latest post-reconciliation CI status is tracked live by
[PR #28](https://github.com/bynanci/storytelling/pull/28); the historical runs
below describe their stated earlier heads only.

## Test-first evidence

- Baseline before this candidate: `cargo test --workspace --locked` passed.
- Identity RED: after creating `tests/approval.rs`, Cargo failed to compile the
  integration target because `storytelling_core::approval` did not exist.
- Identity GREEN: with typed error stubs, 4 contract tests compiled and failed on
  `approval.not_implemented`; after implementation all 4 passed.
- Approval RED: 6 signature/canonical/opacity cases failed to compile because the
  approval API was absent. After DTO stubs, 5 behavior tests failed on
  `approval.not_implemented`. The RFC vector first failed to compile because its
  verifier helper was absent.
- Principal-ID RED/GREEN: the review found the initial ID grammar rejected
  email-style IDs (`by.nanci.tw@gmail.com`); the new test failed with
  `approval.invalid_id`, then passed after allowing `@` without normalization.
- Bounded-allocation RED/GREEN: final self-review found the builder cloned
  caller-owned Host/loss metadata before validating string limits. The new
  preflight test first failed to compile because the preflight function was
  absent, then failed against a no-op stub; after wiring validation before any
  metadata copies, it passed with `approval.invalid_id` at `$.host.host_id`.
- Focused GREEN: `cargo test -p storytelling-core --test approval --locked` passed
  11 integration tests; `cargo test -p storytelling-core --lib --locked` passed
  the RFC 8032 vector; `cargo test -p storytelling-core --doc --locked` passed
  6 compile-fail opacity examples.

## Required local gates

The following local results are historical verification on candidate code before
the dependency-upkeep synchronization. Current hosted exact-head evidence is above.

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed; exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed; exit 0 |
| `cargo test --workspace --locked` | Passed: 184 tests, 0 failed |
| `cargo test --workspace --release --locked` | Passed: 184 tests, 0 failed |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked` | Passed; exit 0 |
| `cargo audit --file Cargo.lock` | Passed locally; 39 locked packages scanned against 1,422 advisories, no findings; hosted Cargo Audit job added after this snapshot; same-head result pending |
| `python3 scripts/check_repo.py` | Passed; links, provenance, dependency boundaries and requirement references valid |
| `git diff --check` | Passed; exit 0 |

Heads `010b7405a1e9eceb397c461ef676d132bc13fbdf` and
`5bb99ad446800a69109869dca2732f395ce93933` passed the earlier five-job hosted
sets in runs `42350601091`, `42350603918`, `42350992888` and duplicate run
`42350998048`. Those historical checks neither accept this candidate nor canonical
M3-01/P1. The subsequent synchronized-head evidence is recorded at the top of this
file; every later commit requires its own exact-head checks.

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed; exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed; exit 0 |
| `cargo test --workspace --locked` | Passed: 184 tests, 0 failed |
| `cargo test --workspace --release --locked` | Passed: 184 tests, 0 failed |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked` | Passed; exit 0 |
| `cargo audit --file Cargo.lock` | Passed; 39 locked packages scanned against 1,422 advisories; no findings emitted |
| `python3 scripts/check_repo.py` | Passed; repository links, provenance, dependency boundaries and test references valid |
| Same-head Linux/Windows/macOS CI | Earlier candidate head `f571d68b69d5835ddc5cb038ea34d4f07348985c` passed all five jobs in runs `42348570713` and `42348574434`; superseded by PR #29 main update |
| Refreshed candidate head `010b7405a1e9eceb397c461ef676d132bc13fbdf` | Both runs `42350601091` and `42350603918` passed Quality, Required, Linux, Windows and macOS; subsequent evidence-document commit requires its own checks |
| Independent G2 security review | Pending |

## Claims not made

A valid signature demonstrates key control over exact payload bytes only. This
record does not claim human authorization, trusted registry provenance, Host
observation, approval expiry/revocation/replay protection, filesystem freshness,
operation persistence, Host execution, editor compatibility or release readiness.
M3-02/M3-03 and exact-head hosted/manual gates remain separate.

## Independent review remediation revision

The follow-up candidate revision narrows public names to `ApprovalIdentityV1`,
`BoundApprovalIdentityV1`, and `ExpectedApproverKeyV1`. Their docs state that
the API hashes caller-supplied bytes, accepts empty slices, and does not prove
compiler provenance, same-snapshot linkage, key-registry trust, or signer
authorization. Regression coverage now makes empty-slice behavior and exact
proposal/artifact byte ceilings explicit. Key binding mismatches use the stable
`approval.approver_key_mismatch` error.

The CI workflow adds a checksum-pinned Cargo Audit 0.22.1 dependency job to the
Required gate. Historical candidate head `f9057f24ecf25b9594e276bd115317c7463dd9f1`
passed both runs `42375033929` and `42375029946` with 39 locked dependencies.
The current 40-package lockfile result is recorded at the top of this file.
Independent G2 review remains pending; the candidate remains unaccepted.
