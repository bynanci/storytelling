use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use storytelling_journal::{
    DispatchResponse, Journal, JournalError, JournalEventKind, OperationIntentV1, OperationState,
    ReadbackResult,
};

static NEXT: AtomicU64 = AtomicU64::new(1);

struct TestDb(PathBuf);

impl TestDb {
    fn new() -> Self {
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        Self(std::env::temp_dir().join(format!(
            "storytelling-journal-{}-{id}.sqlite",
            std::process::id()
        )))
    }
    fn path(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
        let mut wal = self.0.as_os_str().to_owned();
        wal.push("-wal");
        let _ = std::fs::remove_file(PathBuf::from(wal));
        let mut shm = self.0.as_os_str().to_owned();
        shm.push("-shm");
        let _ = std::fs::remove_file(PathBuf::from(shm));
    }
}

fn intent() -> OperationIntentV1 {
    OperationIntentV1::new(
        "op-01",
        "req-01",
        "project-01",
        "a".repeat(64),
        "b".repeat(64),
    )
    .unwrap()
}

#[test]
fn exact_review_and_artifact_identity_survive_restart_and_idempotent_prepare() {
    let db = TestDb::new();
    let intent = intent();
    let first = {
        let mut journal = Journal::open(db.path()).unwrap();
        journal.prepare(&intent, 100).unwrap()
    };
    let mut journal = Journal::open(db.path()).unwrap();
    let replay = journal.prepare(&intent, 101).unwrap();
    assert_eq!(first, replay);
    assert_eq!(journal.get("op-01").unwrap().unwrap().intent, intent);
}

#[test]
fn crash_after_dispatch_marker_recovers_unknown_and_never_retries_without_readback() {
    let db = TestDb::new();
    let old_lease = {
        let mut journal = Journal::open(db.path()).unwrap();
        journal.prepare(&intent(), 100).unwrap();
        let lease = journal.claim("op-01", "executor-a", 101, 50).unwrap();
        assert_eq!(journal.begin_attempt(&lease, 102).unwrap(), 1);
        lease
    };

    let mut journal = Journal::open(db.path()).unwrap();
    let lease = journal.claim("op-01", "executor-b", 200, 50).unwrap();
    let recovered = journal.get("op-01").unwrap().unwrap();
    assert_eq!(recovered.state, OperationState::OutcomeUnknown);
    assert_eq!(recovered.fence_epoch, 2);
    assert!(journal.begin_attempt(&lease, 201).is_err());
    assert!(matches!(
        journal.record_response(
            &old_lease,
            1,
            DispatchResponse::Applied,
            201,
            &"c".repeat(64)
        ),
        Err(JournalError::StaleFence)
    ));
    let reconciled = journal
        .record_readback(&lease, 1, ReadbackResult::NotApplied, 202, &"c".repeat(64))
        .unwrap();
    assert_eq!(reconciled.state, OperationState::NotAppliedObserved);
    assert_eq!(journal.begin_attempt(&lease, 203).unwrap(), 2);
    assert!(
        journal
            .history("op-01")
            .unwrap()
            .iter()
            .any(|e| e.kind == JournalEventKind::RecoveredUnknown)
    );
}

#[test]
fn stale_owner_and_late_attempt_response_cannot_overwrite_newer_state() {
    let db = TestDb::new();
    let mut journal = Journal::open(db.path()).unwrap();
    journal.prepare(&intent(), 100).unwrap();
    let old = journal.claim("op-01", "executor-a", 101, 20).unwrap();
    let first = journal.begin_attempt(&old, 102).unwrap();
    journal
        .record_response(&old, first, DispatchResponse::Unknown, 103, &"d".repeat(64))
        .unwrap();
    journal
        .record_readback(
            &old,
            first,
            ReadbackResult::NotApplied,
            104,
            &"e".repeat(64),
        )
        .unwrap();
    let second = journal.begin_attempt(&old, 105).unwrap();
    assert_eq!(second, 2);
    assert!(matches!(
        journal.record_response(&old, first, DispatchResponse::Applied, 106, &"f".repeat(64)),
        Err(JournalError::StaleAttempt)
    ));
    assert!(matches!(
        journal.record_response(&old, first, DispatchResponse::Unknown, 106, &"d".repeat(64)),
        Err(JournalError::StaleAttempt)
    ));
    assert!(matches!(
        journal.record_readback(
            &old,
            first,
            ReadbackResult::NotApplied,
            106,
            &"e".repeat(64)
        ),
        Err(JournalError::StaleAttempt)
    ));
    assert_eq!(
        journal.get("op-01").unwrap().unwrap().state,
        OperationState::Dispatching
    );
}

#[test]
fn reused_idempotency_key_with_changed_review_or_artifact_is_rejected() {
    let db = TestDb::new();
    let mut journal = Journal::open(db.path()).unwrap();
    journal.prepare(&intent(), 100).unwrap();

    let changed_artifact = OperationIntentV1::new(
        "op-01",
        "req-01",
        "project-01",
        "a".repeat(64),
        "c".repeat(64),
    )
    .unwrap();
    assert!(matches!(
        journal.prepare(&changed_artifact, 101),
        Err(JournalError::IdempotencyConflict)
    ));

    let changed_operation = OperationIntentV1::new(
        "op-02",
        "req-01",
        "project-01",
        "a".repeat(64),
        "b".repeat(64),
    )
    .unwrap();
    assert!(matches!(
        journal.prepare(&changed_operation, 102),
        Err(JournalError::IdempotencyConflict)
    ));
}

#[test]
fn unknown_response_and_applied_response_never_enable_a_retry() {
    let db = TestDb::new();
    let mut journal = Journal::open(db.path()).unwrap();
    journal.prepare(&intent(), 100).unwrap();
    let lease = journal.claim("op-01", "executor-a", 101, 100).unwrap();
    let attempt = journal.begin_attempt(&lease, 102).unwrap();
    let unknown = journal
        .record_response(
            &lease,
            attempt,
            DispatchResponse::Unknown,
            103,
            &"d".repeat(64),
        )
        .unwrap();
    assert_eq!(unknown.state, OperationState::OutcomeUnknown);
    assert!(matches!(
        journal.begin_attempt(&lease, 104),
        Err(JournalError::OutcomeRequiresReadback)
    ));

    let applied = journal
        .record_readback(
            &lease,
            attempt,
            ReadbackResult::Applied,
            105,
            &"e".repeat(64),
        )
        .unwrap();
    assert_eq!(applied.state, OperationState::AppliedObserved);
    assert!(matches!(
        journal.begin_attempt(&lease, 106),
        Err(JournalError::TerminalState)
    ));
}

#[test]
fn a_fresh_conflicting_readback_before_retry_keeps_the_attempt_blocked() {
    let db = TestDb::new();
    let mut journal = Journal::open(db.path()).unwrap();
    journal.prepare(&intent(), 100).unwrap();
    let lease = journal.claim("op-01", "executor-a", 101, 100).unwrap();
    let attempt = journal.begin_attempt(&lease, 102).unwrap();
    journal
        .record_response(
            &lease,
            attempt,
            DispatchResponse::Unknown,
            103,
            &"d".repeat(64),
        )
        .unwrap();
    let absent = journal
        .record_readback(
            &lease,
            attempt,
            ReadbackResult::NotApplied,
            104,
            &"e".repeat(64),
        )
        .unwrap();
    assert_eq!(absent.state, OperationState::NotAppliedObserved);

    let applied = journal
        .record_readback(
            &lease,
            attempt,
            ReadbackResult::Applied,
            105,
            &"f".repeat(64),
        )
        .unwrap();
    assert_eq!(applied.state, OperationState::AppliedObserved);
    assert!(matches!(
        journal.begin_attempt(&lease, 106),
        Err(JournalError::TerminalState)
    ));
}

#[test]
fn invalid_identity_hash_and_time_limits_are_rejected_at_the_boundary() {
    assert!(matches!(
        OperationIntentV1::new(
            "../op",
            "req-01",
            "project-01",
            "a".repeat(64),
            "b".repeat(64)
        ),
        Err(JournalError::InvalidInput)
    ));
    assert!(matches!(
        OperationIntentV1::new(
            "op-01",
            "req-01",
            "project-01",
            "A".repeat(64),
            "b".repeat(64)
        ),
        Err(JournalError::InvalidInput)
    ));

    let db = TestDb::new();
    let mut journal = Journal::open(db.path()).unwrap();
    assert!(matches!(
        journal.prepare(&intent(), -1),
        Err(JournalError::InvalidInput)
    ));
    journal.prepare(&intent(), 100).unwrap();
    assert!(matches!(
        journal.claim("op-01", "executor-a", 101, 86_400_001),
        Err(JournalError::InvalidInput)
    ));
}

#[test]
fn foreign_database_is_rejected_without_changing_its_schema_or_journal_mode() {
    let db = TestDb::new();
    let existing = Connection::open(db.path()).unwrap();
    existing
        .execute_batch("CREATE TABLE user_data (value TEXT NOT NULL);")
        .unwrap();
    let mode_before: String = existing
        .pragma_query_value(None, "journal_mode", |row| row.get(0))
        .unwrap();
    drop(existing);

    assert!(matches!(
        Journal::open(db.path()),
        Err(JournalError::UnsupportedSchema)
    ));

    let reopened = Connection::open(db.path()).unwrap();
    let mode_after: String = reopened
        .pragma_query_value(None, "journal_mode", |row| row.get(0))
        .unwrap();
    let table_count: i64 = reopened
        .query_row(
            "SELECT count(*) FROM sqlite_schema WHERE type = 'table' AND name = 'user_data'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let version: i64 = reopened
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap();
    let app_id: i64 = reopened
        .pragma_query_value(None, "application_id", |row| row.get(0))
        .unwrap();
    assert_eq!(mode_before, mode_after);
    assert_eq!(table_count, 1);
    assert_eq!(version, 0);
    assert_eq!(app_id, 0);
}

#[test]
fn busy_write_leaves_no_partial_record_and_succeeds_after_lock_release() {
    let db = TestDb::new();
    let mut journal =
        Journal::open_with_busy_timeout(db.path(), Duration::from_millis(30)).unwrap();
    let blocker = Connection::open(db.path()).unwrap();
    blocker.execute_batch("BEGIN IMMEDIATE").unwrap();

    assert!(matches!(
        journal.prepare(&intent(), 100),
        Err(JournalError::Busy)
    ));
    drop(blocker);
    assert_eq!(journal.get("op-01").unwrap(), None);
    journal.prepare(&intent(), 101).unwrap();
    assert_eq!(
        journal.get("op-01").unwrap().unwrap().state,
        OperationState::Ready
    );
}

#[test]
fn attempt_count_limit_is_per_operation_and_does_not_block_other_operations() {
    let db = TestDb::new();
    let mut journal = Journal::open(db.path()).unwrap();
    journal.prepare(&intent(), 100).unwrap();
    let lease = journal.claim("op-01", "executor-a", 101, 10_000).unwrap();

    let mut now_ms = 102_i64;
    for attempt_id in 1..=64_u32 {
        now_ms += 1;
        assert_eq!(journal.begin_attempt(&lease, now_ms).unwrap(), attempt_id);
        let response_hash = format!("{attempt_id:064x}");
        now_ms += 1;
        journal
            .record_response(
                &lease,
                attempt_id,
                DispatchResponse::Unknown,
                now_ms,
                &response_hash,
            )
            .unwrap();
        let readback_hash = format!("{:064x}", u64::from(attempt_id) + 1_000);
        now_ms += 1;
        journal
            .record_readback(
                &lease,
                attempt_id,
                ReadbackResult::NotApplied,
                now_ms,
                &readback_hash,
            )
            .unwrap();
    }

    assert!(matches!(
        journal.begin_attempt(&lease, now_ms + 1),
        Err(JournalError::LimitExceeded)
    ));
    let other = OperationIntentV1::new(
        "op-02",
        "req-02",
        "project-01",
        "1".repeat(64),
        "2".repeat(64),
    )
    .unwrap();
    journal.prepare(&other, 501).unwrap();
    assert_eq!(
        journal.get("op-02").unwrap().unwrap().state,
        OperationState::Ready
    );
}

#[test]
fn repeated_ambiguous_readbacks_have_a_per_attempt_bound() {
    let db = TestDb::new();
    let mut journal = Journal::open(db.path()).unwrap();
    journal.prepare(&intent(), 100).unwrap();
    let lease = journal.claim("op-01", "executor-a", 101, 100).unwrap();
    let attempt = journal.begin_attempt(&lease, 102).unwrap();
    journal
        .record_response(
            &lease,
            attempt,
            DispatchResponse::Unknown,
            103,
            &"d".repeat(64),
        )
        .unwrap();

    for index in 0..4_u64 {
        let evidence = format!("{index:064x}");
        let state = journal
            .record_readback(
                &lease,
                attempt,
                ReadbackResult::Ambiguous,
                104 + index as i64,
                &evidence,
            )
            .unwrap();
        assert_eq!(state.state, OperationState::OutcomeUnknown);
        // An acknowledgement replay for the same evidence is idempotent.
        if index == 0 {
            journal
                .record_readback(&lease, attempt, ReadbackResult::Ambiguous, 104, &evidence)
                .unwrap();
        }
    }

    assert!(matches!(
        journal.record_readback(
            &lease,
            attempt,
            ReadbackResult::Ambiguous,
            200,
            &"f".repeat(64),
        ),
        Err(JournalError::LimitExceeded)
    ));
}
