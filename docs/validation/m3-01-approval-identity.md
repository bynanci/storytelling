<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# M3-01 approval identity validation

Status: implementation in progress. Current base: `073b5a415324d588019e2155dcf24ddddd860adf`.
Local gates below ran on the M3-01 candidate worktree based on that exact main SHA.
Final author self-review is recorded in the task ledger; independent G2 security
review and same-head Linux/Windows/macOS CI remain pending.

## Test-first evidence

- Baseline before M3-01: `cargo test --workspace --locked` passed.
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

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed; exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed; exit 0 |
| `cargo test --workspace --locked` | Passed: 184 tests, 0 failed |
| `cargo test --workspace --release --locked` | Passed: 184 tests, 0 failed |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked` | Passed; exit 0 |
| `cargo audit --file Cargo.lock` | Passed; 39 locked packages scanned against 1,422 advisories; no findings emitted |
| `python3 scripts/check_repo.py` | Passed; repository links, provenance, dependency boundaries and test references valid |
| Same-head Linux/Windows/macOS CI | Pending; requires Draft PR head |
| Independent G2 security review | Pending |

## Claims not made

A valid signature demonstrates key control over exact payload bytes only. This
record does not claim human authorization, trusted registry provenance, Host
observation, approval expiry/revocation/replay protection, filesystem freshness,
operation persistence, Host execution, editor compatibility or release readiness.
M3-02/M3-03 and exact-head hosted/manual gates remain separate.
