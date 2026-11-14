# M3-02 first-readback receipt regression

English · [繁體中文](m3-02-readback-receipt-collision.zh-TW.md)

This maintenance fix preserves the accepted M3-JL02/M3-JL06 journal API and
SQLite schema. Base: `c562ce0d06b7330813fa42cad739f24f4a49e9da`. It adds no host
calls, execution authority, W01 implementation, or #103 G2 acceptance.

## Reproduction and result

Prepare an operation, claim its lease and begin attempt 1. Record an `Unknown`
response with digest D, then record the first `Ambiguous` readback with the same
D. Both map to `outcome_unknown`, but they are different receipt kinds.

Before the fix, the readback returned success while history retained only
`Prepared`, `AttemptStarted` and `ResponseRecorded`. No `ReadbackRecorded` event
was persisted and the observation did not consume one of the four readbacks.
Four additional distinct readbacks succeeded, allowing five observations.

The duplicate-readback shortcut now also requires `readback_count > 0`. The first
observation always persists its event and consumes its budget; replaying that
same readback remains idempotent, including after reopening the database.
Unknown outcomes still block retry. The fifth distinct observation returns
`journal.limit_exceeded` without changing history.

## Test-first evidence

[Machine receipts](m3-02-readback-receipt-collision/red-green-receipts.json),
[RED output](m3-02-readback-receipt-collision/red.log) and
[GREEN output](m3-02-readback-receipt-collision/green.log) retain actual runs.
[Workspace receipts](m3-02-readback-receipt-collision/workspace-checks.json) record
the successful rerun separately from the initial infrastructure failure.

| Stage | Result |
| --- | --- |
| RED, `182822797470c812d7c1238d67d0fd443c19b953` | 11 passed, 2 failed; missing event and unconsumed budget assertions failed |
| Fix, `dd38313e96906aad835fc975e63665599bc44ce9` | Unchanged implementation bytes tested locally: 13 passed, 0 failed |
| Independent static AI review | No blocker; fence/lease/attempt checks still precede idempotency; not human acceptance |
| Workspace rerun | Debug/release: 312 passed each; all-feature debug/release: 331 passed each; 3 ignored each. Initial disk-capacity failure recovered; fmt, Clippy, rustdoc and repository checks passed |
| Hosted CI | Pending; must pass on the final PR head before merge |

```sh
cargo test -p storytelling-journal --test journal --locked -- --nocapture
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo test --workspace --release --locked
cargo test --workspace --all-features --release --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps --locked
python3 scripts/check_repo.py
```

`ambiguous_readback_sharing_response_digest_is_recorded_once_and_survives_restart`
checks the event, stored observation, reopen and replay.
`first_ambiguous_readback_sharing_response_digest_consumes_readback_budget`
checks four admitted observations and atomic rejection of the fifth.

## Compatibility and limits

No DTO, diagnostic, database schema, dependency or retry transition changes.
Existing databases need no migration. The fix uses the existing persisted count;
it cannot reconstruct receipts omitted by older code. For previously affected
attempts, a subsequent identical readback records the missing first observation
if no later readback was recorded. Do not invent historical timestamps or host
observations. Rolling back restores the bookkeeping defect; retain the database.

Tests use temporary synthetic SQLite files. They do not establish readback truth,
media identity, signer authority, an actual host effect, human approval or a G5
publication decision. #103/W01 and production-host gates remain separate.
