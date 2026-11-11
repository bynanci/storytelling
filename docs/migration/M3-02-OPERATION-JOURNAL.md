# M3-02 SQLite operation journal v1

Status: accepted bounded adapter, merged by [PR #56](https://github.com/bynanci/storytelling/pull/56).
The crate persists
operations and recovery state; it does not call a host, execute an edit, grant an
approval, or verify a readback. See the requirement/test map in
[REQUIREMENTS](../REQUIREMENTS.md#sqlite-operation-journal).

## Identity and write-ahead rule

`storytelling_journal::OperationIntentV1` freezes an operation ID, project ID,
request idempotency key, M3-01/P2 review-binding SHA-256, and exact OTIO artifact
SHA-256. Its domain-separated intent fingerprint includes every field. A repeated
project/request key with identical identity returns the original row; changed
identity under either the operation ID or request key rejects with
`journal.idempotency_conflict`. The journal stores identifiers, digests, states
and evidence digests, never media, OTIO bytes, project files, filesystem paths,
host responses or credentials.

An adapter must commit `prepare`, acquire a lease with `claim`, then commit
`begin_attempt` before invoking any host API. `begin_attempt` inserts the exact
attempt number, current executor and fence epoch and changes the operation to
`dispatching` in one SQLite `IMMEDIATE` transaction. If the process stops after
that commit, recovery treats the operation as unknown even if the host call may
not yet have started. This deliberate false uncertainty prevents a blind replay.

## States and recovery

```text
ready -> dispatching -> applied_reported -> applied_observed (terminal)
                     -> outcome_unknown -> not_applied_observed -> dispatching
                                        -> applied_observed (terminal)
                                        -> outcome_unknown (read back again)
                     -> partial_reported -> partial_observed (terminal)
```

An expired lease on `dispatching` atomically changes the attempt and operation to
`outcome_unknown`, appends a recovery event, increments the fencing epoch and
assigns the new executor. `outcome_unknown`, host-reported applied/partial and
ambiguous observations cannot begin another attempt. Only an explicit
`NotApplied` readback creates `not_applied_observed`, from which the current
owner may begin the next numbered attempt. Applied host responses remain
`applied_reported` until a readback is recorded as `applied_observed`.
A newer readback may revise a prior `NotApplied` observation until the next
attempt is durably started; after that point, the earlier attempt is stale.

Every mutation checks the current executor ID, fence epoch and unexpired lease.
The current attempt number is also checked before accepting a response or
readback, including an otherwise identical duplicate receipt. An old owner or a
late receipt for an earlier attempt cannot overwrite a newer state. Repeating an
identical receipt for the current attempt is idempotent.

The lease and fencing token protect journal state only; FilmCraft does not yet
understand the token. Before M3-03 treats a `NotApplied` readback as retry
permission, its adapter must establish that the earlier host request is no longer
in flight, or use a host-supported idempotency key. A readback taken while an old
request can still complete is not sufficient evidence to safely retry. No
exactly-once external effect is claimed.

## SQLite and bounded storage

The adapter uses pinned `rusqlite` 0.41.1 with bundled SQLite. Each connection
requires WAL mode, `synchronous=FULL`, foreign keys and a bounded busy timeout;
mutations use `BEGIN IMMEDIATE`. Schema version and application ID are checked,
and unknown versions fail closed. Journal events have database triggers that
reject update/delete. SQLite transactions make each journal mutation atomic;
they cannot include the external host side effect.

On open, the adapter checks the SQLite application ID, schema version and
required tables before changing journal mode or creating schema. An unrelated or
unknown-version database is rejected without modification.

Limits are part of v1: identifiers are 1–128 ASCII bytes; digest fields are 64
lowercase hex bytes; executor leases are 1 ms–24 hours; each operation allows
at most 64 attempts, four readbacks per attempt, the signed SQLite integer range
of fencing epochs and 512 append-only events. The per-operation readback bound
keeps the event history finite; exact duplicate receipts do not consume that
budget. There is no aggregate operation-count cap, so one exhausted operation
does not block unrelated new operations. A database busy response is retryable
after the lock clears; other SQLite failures map to path-free stable error codes.
SQLite file size is not capped and can grow with operation count. Disk-full
failure must surface without partial transaction state and can be retried after
space is restored. There is no deletion, compaction, backup, schema-upgrade or
retention API yet; define those policies before long-lived production use.
When a per-operation hard limit is reached, that operation requires operator
handling; unrelated operations remain usable.

The first two trial pins (`rusqlite` 0.37 / `libsqlite3-sys` 0.35 and `rusqlite`
0.40.2 / `libsqlite3-sys` 0.38) were rejected after Cargo Audit identified
RUSTSEC-2026-0459. The final candidate uses the fixed `libsqlite3-sys` 0.39 line
and raises the repository MSRV to 1.89; see the
[dependency review](DEPENDENCY_REVIEW_M3_02.md).

The caller supplies nonnegative epoch-millisecond times. The journal does not
provide a trusted clock, authenticate executor IDs, prove process death, or
prevent wall-clock skew. Expiry is a liveness hint; fencing is the stale-write
check. WAL/FULL durability assumes a local filesystem and working OS/storage
sync semantics. Network filesystems, hardware power-loss behavior and each
platform's filesystem implementation require separate evidence.

## Scope boundary

`review_binding_sha256` and `artifact_sha256` are caller-supplied identities. The
journal preserves them exactly but does not recompute the review, compile the
artifact, validate the #28 signature, identify an authorized signer, consume a
nonce, or decide whether losses are approved. #28's verifier proves only that a
caller-provided expected public key validates a signature over caller-supplied
bytes; it does not prove signer authorization or compiler provenance. M3-01/P2
provides the bounded compiler-derived OTIO identity; M3-02 persists that identity
without elevating it to execution authority.

There is no CLI command or executor in this slice. Tests use only temporary
synthetic SQLite files. M3-03 remains responsible for new isolated FilmCraft
projects, actual readback, ambiguity handling and a one-writer host policy.
