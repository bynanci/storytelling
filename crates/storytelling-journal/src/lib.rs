//! Durable, owner-fenced operation journal for future adapters.
//!
//! This crate persists intent and recovery state. It has no host/editor client and
//! cannot verify approvals, artifact provenance, or the truth of caller-supplied
//! readback evidence. Unknown outcomes never authorize a new attempt.

use rusqlite::{
    Connection, OptionalExtension, Transaction, TransactionBehavior, params, types::Type,
};
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::Path;
use std::time::Duration;

pub const MAX_IDENTIFIER_BYTES: usize = 128;
pub const MAX_LEASE_MS: i64 = 86_400_000;
pub const MAX_ATTEMPTS_PER_OPERATION: u32 = 64;
pub const MAX_READBACKS_PER_ATTEMPT: u32 = 4;
pub const MAX_FENCE_EPOCH: u64 = i64::MAX as u64;
pub const MAX_EVENTS_PER_OPERATION: usize = 512;
const DEFAULT_BUSY_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_BUSY_TIMEOUT: Duration = Duration::from_secs(30);
const SCHEMA_VERSION: i64 = 1;
const APPLICATION_ID: i64 = 0x5354_4a31; // "STJ1"

const SCHEMA: &str = r#"
CREATE TABLE operations (
    operation_id TEXT PRIMARY KEY NOT NULL,
    idempotency_key TEXT NOT NULL,
    project_id TEXT NOT NULL,
    review_binding_sha256 TEXT NOT NULL,
    artifact_sha256 TEXT NOT NULL,
    intent_sha256 TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN (
        'ready', 'dispatching', 'outcome_unknown', 'not_applied_observed',
        'applied_reported', 'applied_observed', 'partial_reported', 'partial_observed'
    )),
    owner_id TEXT,
    fence_epoch INTEGER NOT NULL CHECK (fence_epoch >= 0),
    lease_until_ms INTEGER,
    attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0),
    created_at_ms INTEGER NOT NULL CHECK (created_at_ms >= 0),
    updated_at_ms INTEGER NOT NULL CHECK (updated_at_ms >= 0),
    UNIQUE (project_id, idempotency_key)
) STRICT;

CREATE TABLE attempts (
    operation_id TEXT NOT NULL REFERENCES operations(operation_id),
    attempt_id INTEGER NOT NULL CHECK (attempt_id > 0),
    executor_id TEXT NOT NULL,
    fence_epoch INTEGER NOT NULL CHECK (fence_epoch > 0),
    state TEXT NOT NULL CHECK (state IN (
        'dispatching', 'outcome_unknown', 'not_applied_observed',
        'applied_reported', 'applied_observed', 'partial_reported', 'partial_observed'
    )),
    started_at_ms INTEGER NOT NULL CHECK (started_at_ms >= 0),
    finished_at_ms INTEGER,
    evidence_sha256 TEXT,
    readback_count INTEGER NOT NULL DEFAULT 0 CHECK (readback_count >= 0),
    PRIMARY KEY (operation_id, attempt_id)
) STRICT;

CREATE TABLE journal_events (
    sequence INTEGER PRIMARY KEY,
    operation_id TEXT NOT NULL REFERENCES operations(operation_id),
    kind TEXT NOT NULL CHECK (kind IN (
        'prepared', 'attempt_started', 'response_recorded',
        'recovered_unknown', 'readback_recorded'
    )),
    executor_id TEXT,
    fence_epoch INTEGER NOT NULL CHECK (fence_epoch >= 0),
    attempt_id INTEGER,
    occurred_at_ms INTEGER NOT NULL CHECK (occurred_at_ms >= 0),
    evidence_sha256 TEXT
) STRICT;

CREATE INDEX journal_events_by_operation
    ON journal_events(operation_id, sequence);

CREATE TRIGGER journal_events_are_append_only_update
BEFORE UPDATE ON journal_events BEGIN
    SELECT RAISE(ABORT, 'journal events are append-only');
END;
CREATE TRIGGER journal_events_are_append_only_delete
BEFORE DELETE ON journal_events BEGIN
    SELECT RAISE(ABORT, 'journal events are append-only');
END;
"#;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationIntentV1 {
    operation_id: String,
    idempotency_key: String,
    project_id: String,
    review_binding_sha256: String,
    artifact_sha256: String,
}

impl OperationIntentV1 {
    pub fn new(
        operation_id: impl Into<String>,
        idempotency_key: impl Into<String>,
        project_id: impl Into<String>,
        review_binding_sha256: impl Into<String>,
        artifact_sha256: impl Into<String>,
    ) -> Result<Self, JournalError> {
        let intent = Self {
            operation_id: operation_id.into(),
            idempotency_key: idempotency_key.into(),
            project_id: project_id.into(),
            review_binding_sha256: review_binding_sha256.into(),
            artifact_sha256: artifact_sha256.into(),
        };
        intent.validate()?;
        Ok(intent)
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn idempotency_key(&self) -> &str {
        &self.idempotency_key
    }

    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    pub fn review_binding_sha256(&self) -> &str {
        &self.review_binding_sha256
    }

    pub fn artifact_sha256(&self) -> &str {
        &self.artifact_sha256
    }

    fn validate(&self) -> Result<(), JournalError> {
        validate_identifier(&self.operation_id)?;
        validate_identifier(&self.idempotency_key)?;
        validate_identifier(&self.project_id)?;
        validate_sha256(&self.review_binding_sha256)?;
        validate_sha256(&self.artifact_sha256)?;
        Ok(())
    }

    fn fingerprint(&self) -> String {
        let mut hash = Sha256::new();
        hash.update(b"bynanci.storytelling.operation-intent.v1\0");
        for field in [
            self.operation_id.as_str(),
            self.idempotency_key.as_str(),
            self.project_id.as_str(),
            self.review_binding_sha256.as_str(),
            self.artifact_sha256.as_str(),
        ] {
            hash.update((field.len() as u64).to_be_bytes());
            hash.update(field.as_bytes());
        }
        lower_hex(&hash.finalize())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationState {
    Ready,
    Dispatching,
    OutcomeUnknown,
    NotAppliedObserved,
    AppliedReported,
    AppliedObserved,
    PartialReported,
    PartialObserved,
}

impl OperationState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Dispatching => "dispatching",
            Self::OutcomeUnknown => "outcome_unknown",
            Self::NotAppliedObserved => "not_applied_observed",
            Self::AppliedReported => "applied_reported",
            Self::AppliedObserved => "applied_observed",
            Self::PartialReported => "partial_reported",
            Self::PartialObserved => "partial_observed",
        }
    }

    fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "ready" => Self::Ready,
            "dispatching" => Self::Dispatching,
            "outcome_unknown" => Self::OutcomeUnknown,
            "not_applied_observed" => Self::NotAppliedObserved,
            "applied_reported" => Self::AppliedReported,
            "applied_observed" => Self::AppliedObserved,
            "partial_reported" => Self::PartialReported,
            "partial_observed" => Self::PartialObserved,
            _ => return None,
        })
    }

    fn is_terminal(self) -> bool {
        matches!(self, Self::AppliedObserved | Self::PartialObserved)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispatchResponse {
    Applied,
    Unknown,
    Partial,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadbackResult {
    Applied,
    NotApplied,
    Ambiguous,
    Partial,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalEventKind {
    Prepared,
    AttemptStarted,
    ResponseRecorded,
    RecoveredUnknown,
    ReadbackRecorded,
}

impl JournalEventKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::AttemptStarted => "attempt_started",
            Self::ResponseRecorded => "response_recorded",
            Self::RecoveredUnknown => "recovered_unknown",
            Self::ReadbackRecorded => "readback_recorded",
        }
    }

    fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "prepared" => Self::Prepared,
            "attempt_started" => Self::AttemptStarted,
            "response_recorded" => Self::ResponseRecorded,
            "recovered_unknown" => Self::RecoveredUnknown,
            "readback_recorded" => Self::ReadbackRecorded,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationSnapshot {
    pub intent: OperationIntentV1,
    pub state: OperationState,
    pub owner_id: Option<String>,
    pub fence_epoch: u64,
    pub lease_until_ms: Option<i64>,
    pub attempt_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionLease {
    operation_id: String,
    executor_id: String,
    fence_epoch: u64,
}

impl ExecutionLease {
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn executor_id(&self) -> &str {
        &self.executor_id
    }

    pub fn fence_epoch(&self) -> u64 {
        self.fence_epoch
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JournalEvent {
    pub sequence: i64,
    pub kind: JournalEventKind,
    pub executor_id: Option<String>,
    pub fence_epoch: u64,
    pub attempt_id: Option<u32>,
    pub occurred_at_ms: i64,
    pub evidence_sha256: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum JournalError {
    InvalidInput,
    UnsupportedSchema,
    AlreadyClaimed,
    LeaseExpired,
    StaleFence,
    StaleAttempt,
    OutcomeRequiresReadback,
    TerminalState,
    IdempotencyConflict,
    LimitExceeded,
    NotFound,
    Busy,
    Storage,
}

impl JournalError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidInput => "journal.invalid_input",
            Self::UnsupportedSchema => "journal.unsupported_schema",
            Self::AlreadyClaimed => "journal.already_claimed",
            Self::LeaseExpired => "journal.lease_expired",
            Self::StaleFence => "journal.stale_fence",
            Self::StaleAttempt => "journal.stale_attempt",
            Self::OutcomeRequiresReadback => "journal.readback_required",
            Self::TerminalState => "journal.terminal_state",
            Self::IdempotencyConflict => "journal.idempotency_conflict",
            Self::LimitExceeded => "journal.limit_exceeded",
            Self::NotFound => "journal.not_found",
            Self::Busy => "journal.busy",
            Self::Storage => "journal.storage",
        }
    }
}

impl fmt::Display for JournalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for JournalError {}

pub struct Journal {
    connection: Connection,
}

impl Journal {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, JournalError> {
        Self::open_with_busy_timeout(path, DEFAULT_BUSY_TIMEOUT)
    }

    pub fn open_with_busy_timeout(
        path: impl AsRef<Path>,
        timeout: Duration,
    ) -> Result<Self, JournalError> {
        if timeout > MAX_BUSY_TIMEOUT {
            return Err(JournalError::InvalidInput);
        }
        let mut connection = Connection::open(path).map_err(map_sqlite)?;
        connection.busy_timeout(timeout).map_err(map_sqlite)?;

        let app_id: i64 = connection
            .pragma_query_value(None, "application_id", |row| row.get(0))
            .map_err(map_sqlite)?;
        let version: i64 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(map_sqlite)?;
        match (app_id, version) {
            (0, 0) => {
                let table_count: i64 = connection
                    .query_row(
                        "SELECT count(*) FROM sqlite_schema
                         WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
                        [],
                        |row| row.get(0),
                    )
                    .map_err(map_sqlite)?;
                if table_count != 0 {
                    return Err(JournalError::UnsupportedSchema);
                }
            }
            (APPLICATION_ID, SCHEMA_VERSION) => {
                let table_count: i64 = connection
                    .query_row(
                        "SELECT count(*) FROM sqlite_schema WHERE type = 'table'
                         AND name IN ('operations', 'attempts', 'journal_events')",
                        [],
                        |row| row.get(0),
                    )
                    .map_err(map_sqlite)?;
                if table_count != 3 {
                    return Err(JournalError::UnsupportedSchema);
                }
            }
            _ => return Err(JournalError::UnsupportedSchema),
        }

        connection
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(map_sqlite)?;

        let mode: String = connection
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .map_err(map_sqlite)?;
        if !mode.eq_ignore_ascii_case("wal") {
            return Err(JournalError::Storage);
        }
        connection
            .pragma_update(None, "synchronous", "FULL")
            .map_err(map_sqlite)?;
        connection
            .pragma_update(None, "foreign_keys", "ON")
            .map_err(map_sqlite)?;
        connection
            .pragma_update(None, "wal_autocheckpoint", 1000_i64)
            .map_err(map_sqlite)?;
        match version {
            0 => {
                let transaction = connection
                    .transaction_with_behavior(TransactionBehavior::Immediate)
                    .map_err(map_sqlite)?;
                transaction.execute_batch(SCHEMA).map_err(map_sqlite)?;
                transaction
                    .pragma_update(None, "application_id", APPLICATION_ID)
                    .map_err(map_sqlite)?;
                transaction
                    .pragma_update(None, "user_version", SCHEMA_VERSION)
                    .map_err(map_sqlite)?;
                transaction.commit().map_err(map_sqlite)?;
            }
            SCHEMA_VERSION => {}
            _ => return Err(JournalError::UnsupportedSchema),
        }

        Ok(Self { connection })
    }

    /// Persists an immutable intent before an executor may dispatch it.
    /// Repeating the same idempotency key and exact identity returns the first record.
    pub fn prepare(
        &mut self,
        intent: &OperationIntentV1,
        now_ms: i64,
    ) -> Result<OperationSnapshot, JournalError> {
        validate_time(now_ms)?;
        intent.validate()?;
        let fingerprint = intent.fingerprint();
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(map_sqlite)?;

        let by_operation = load_stored(&transaction, intent.operation_id())?;
        let by_request =
            load_by_request(&transaction, intent.project_id(), intent.idempotency_key())?;
        if let Some(existing) = by_operation.as_ref().or(by_request.as_ref()) {
            if existing.intent.fingerprint() != fingerprint
                || by_operation
                    .as_ref()
                    .is_some_and(|row| row.intent != *intent)
                || by_request.as_ref().is_some_and(|row| row.intent != *intent)
            {
                return Err(JournalError::IdempotencyConflict);
            }
            let result = snapshot_from_stored(existing)?;
            transaction.commit().map_err(map_sqlite)?;
            return Ok(result);
        }

        transaction
            .execute(
                "INSERT INTO operations (
                    operation_id, idempotency_key, project_id, review_binding_sha256,
                    artifact_sha256, intent_sha256, state, owner_id, fence_epoch,
                    lease_until_ms, attempt_count, created_at_ms, updated_at_ms
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'ready', NULL, 0, NULL, 0, ?7, ?7)",
                params![
                    intent.operation_id,
                    intent.idempotency_key,
                    intent.project_id,
                    intent.review_binding_sha256,
                    intent.artifact_sha256,
                    fingerprint,
                    now_ms
                ],
            )
            .map_err(map_sqlite)?;
        append_event(
            &transaction,
            NewEvent {
                operation_id: intent.operation_id(),
                kind: JournalEventKind::Prepared,
                executor_id: None,
                fence_epoch: 0,
                attempt_id: None,
                occurred_at_ms: now_ms,
                evidence_sha256: None,
            },
        )?;
        let result = load_stored(&transaction, intent.operation_id())?
            .as_ref()
            .map(snapshot_from_stored)
            .transpose()?
            .ok_or(JournalError::Storage)?;
        transaction.commit().map_err(map_sqlite)?;
        Ok(result)
    }

    /// Acquires a bounded lease and increments a persistent fencing epoch.
    /// An expired `dispatching` state is changed to `outcome_unknown` before return.
    pub fn claim(
        &mut self,
        operation_id: &str,
        executor_id: &str,
        now_ms: i64,
        lease_ms: i64,
    ) -> Result<ExecutionLease, JournalError> {
        validate_identifier(operation_id)?;
        validate_identifier(executor_id)?;
        validate_time(now_ms)?;
        validate_lease(lease_ms)?;
        let lease_until = now_ms
            .checked_add(lease_ms)
            .ok_or(JournalError::InvalidInput)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(map_sqlite)?;
        let mut operation =
            load_stored(&transaction, operation_id)?.ok_or(JournalError::NotFound)?;

        if operation.state.is_terminal() {
            return Err(JournalError::TerminalState);
        }
        if operation
            .lease_until_ms
            .is_some_and(|current_until| current_until > now_ms)
        {
            return Err(JournalError::AlreadyClaimed);
        }
        if operation.fence_epoch >= MAX_FENCE_EPOCH {
            return Err(JournalError::LimitExceeded);
        }

        if operation.state == OperationState::Dispatching {
            let attempt_id = operation.attempt_count;
            transaction
                .execute(
                    "UPDATE attempts SET state = 'outcome_unknown', finished_at_ms = ?3
                     WHERE operation_id = ?1 AND attempt_id = ?2 AND state = 'dispatching'",
                    params![operation_id, i64::from(attempt_id), now_ms],
                )
                .map_err(map_sqlite)?;
            transaction
                .execute(
                    "UPDATE operations SET state = 'outcome_unknown', updated_at_ms = ?2
                     WHERE operation_id = ?1",
                    params![operation_id, now_ms],
                )
                .map_err(map_sqlite)?;
            append_event(
                &transaction,
                NewEvent {
                    operation_id,
                    kind: JournalEventKind::RecoveredUnknown,
                    executor_id: operation.owner_id.as_deref(),
                    fence_epoch: operation.fence_epoch,
                    attempt_id: Some(attempt_id),
                    occurred_at_ms: now_ms,
                    evidence_sha256: None,
                },
            )?;
            operation.state = OperationState::OutcomeUnknown;
        }

        let new_epoch = operation.fence_epoch + 1;
        transaction
            .execute(
                "UPDATE operations SET owner_id = ?2, fence_epoch = ?3,
                    lease_until_ms = ?4, updated_at_ms = ?5 WHERE operation_id = ?1",
                params![
                    operation_id,
                    executor_id,
                    new_epoch as i64,
                    lease_until,
                    now_ms
                ],
            )
            .map_err(map_sqlite)?;
        transaction.commit().map_err(map_sqlite)?;
        Ok(ExecutionLease {
            operation_id: operation_id.to_owned(),
            executor_id: executor_id.to_owned(),
            fence_epoch: new_epoch,
        })
    }

    /// Extends a current lease. It never changes an attempt outcome.
    pub fn renew(
        &mut self,
        lease: &ExecutionLease,
        now_ms: i64,
        lease_ms: i64,
    ) -> Result<(), JournalError> {
        validate_time(now_ms)?;
        validate_lease(lease_ms)?;
        let lease_until = now_ms
            .checked_add(lease_ms)
            .ok_or(JournalError::InvalidInput)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(map_sqlite)?;
        let operation = require_current_lease(&transaction, lease, now_ms)?;
        if operation.state.is_terminal() {
            return Err(JournalError::TerminalState);
        }
        transaction
            .execute(
                "UPDATE operations SET lease_until_ms = ?4, updated_at_ms = ?5
                 WHERE operation_id = ?1 AND owner_id = ?2 AND fence_epoch = ?3",
                params![
                    lease.operation_id,
                    lease.executor_id,
                    lease.fence_epoch as i64,
                    lease_until,
                    now_ms
                ],
            )
            .map_err(map_sqlite)?;
        transaction.commit().map_err(map_sqlite)
    }

    /// Commits a durable dispatch marker. Call this before invoking any host API.
    pub fn begin_attempt(
        &mut self,
        lease: &ExecutionLease,
        now_ms: i64,
    ) -> Result<u32, JournalError> {
        validate_time(now_ms)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(map_sqlite)?;
        let operation = require_current_lease(&transaction, lease, now_ms)?;
        if operation.state != OperationState::Ready
            && operation.state != OperationState::NotAppliedObserved
        {
            return if operation.state.is_terminal() {
                Err(JournalError::TerminalState)
            } else {
                Err(JournalError::OutcomeRequiresReadback)
            };
        }
        if operation.attempt_count >= MAX_ATTEMPTS_PER_OPERATION {
            return Err(JournalError::LimitExceeded);
        }
        let attempt_id = operation.attempt_count + 1;
        transaction
            .execute(
                "INSERT INTO attempts (
                    operation_id, attempt_id, executor_id, fence_epoch, state,
                    started_at_ms, finished_at_ms, evidence_sha256
                 ) VALUES (?1, ?2, ?3, ?4, 'dispatching', ?5, NULL, NULL)",
                params![
                    lease.operation_id,
                    i64::from(attempt_id),
                    lease.executor_id,
                    lease.fence_epoch as i64,
                    now_ms
                ],
            )
            .map_err(map_sqlite)?;
        transaction
            .execute(
                "UPDATE operations SET state = 'dispatching', attempt_count = ?2,
                    updated_at_ms = ?3 WHERE operation_id = ?1",
                params![lease.operation_id, i64::from(attempt_id), now_ms],
            )
            .map_err(map_sqlite)?;
        append_event(
            &transaction,
            NewEvent {
                operation_id: &lease.operation_id,
                kind: JournalEventKind::AttemptStarted,
                executor_id: Some(&lease.executor_id),
                fence_epoch: lease.fence_epoch,
                attempt_id: Some(attempt_id),
                occurred_at_ms: now_ms,
                evidence_sha256: None,
            },
        )?;
        transaction.commit().map_err(map_sqlite)?;
        Ok(attempt_id)
    }

    /// Records a host response. `Applied` remains a report until readback confirms it.
    pub fn record_response(
        &mut self,
        lease: &ExecutionLease,
        attempt_id: u32,
        response: DispatchResponse,
        now_ms: i64,
        evidence_sha256: &str,
    ) -> Result<OperationSnapshot, JournalError> {
        validate_time(now_ms)?;
        validate_sha256(evidence_sha256)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(map_sqlite)?;
        let operation = require_current_lease(&transaction, lease, now_ms)?;
        let stored = load_attempt(&transaction, &lease.operation_id, attempt_id)?
            .ok_or(JournalError::StaleAttempt)?;
        if operation.attempt_count != attempt_id {
            return Err(JournalError::StaleAttempt);
        }
        let (new_state, state_text) = match response {
            DispatchResponse::Applied => (OperationState::AppliedReported, "applied_reported"),
            DispatchResponse::Unknown => (OperationState::OutcomeUnknown, "outcome_unknown"),
            DispatchResponse::Partial => (OperationState::PartialReported, "partial_reported"),
        };
        if stored.state != "dispatching" {
            if stored.state == state_text
                && stored.evidence_sha256.as_deref() == Some(evidence_sha256)
            {
                let result = snapshot_from_stored(&operation)?;
                transaction.commit().map_err(map_sqlite)?;
                return Ok(result);
            }
            return Err(JournalError::StaleAttempt);
        }
        if operation.state != OperationState::Dispatching {
            return Err(JournalError::StaleAttempt);
        }
        transaction
            .execute(
                "UPDATE attempts SET state = ?3, finished_at_ms = ?4, evidence_sha256 = ?5
                 WHERE operation_id = ?1 AND attempt_id = ?2 AND state = 'dispatching'",
                params![
                    lease.operation_id,
                    i64::from(attempt_id),
                    state_text,
                    now_ms,
                    evidence_sha256
                ],
            )
            .map_err(map_sqlite)?;
        transaction
            .execute(
                "UPDATE operations SET state = ?2, updated_at_ms = ?3 WHERE operation_id = ?1",
                params![lease.operation_id, new_state.as_str(), now_ms],
            )
            .map_err(map_sqlite)?;
        append_event(
            &transaction,
            NewEvent {
                operation_id: &lease.operation_id,
                kind: JournalEventKind::ResponseRecorded,
                executor_id: Some(&lease.executor_id),
                fence_epoch: lease.fence_epoch,
                attempt_id: Some(attempt_id),
                occurred_at_ms: now_ms,
                evidence_sha256: Some(evidence_sha256),
            },
        )?;
        let result = load_stored(&transaction, &lease.operation_id)?
            .as_ref()
            .map(snapshot_from_stored)
            .transpose()?
            .ok_or(JournalError::Storage)?;
        transaction.commit().map_err(map_sqlite)?;
        Ok(result)
    }

    /// Persists a caller's host readback. Only an explicit `NotApplied` observation
    /// permits a later attempt; the journal does not validate that observation.
    pub fn record_readback(
        &mut self,
        lease: &ExecutionLease,
        attempt_id: u32,
        result: ReadbackResult,
        now_ms: i64,
        evidence_sha256: &str,
    ) -> Result<OperationSnapshot, JournalError> {
        validate_time(now_ms)?;
        validate_sha256(evidence_sha256)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(map_sqlite)?;
        let operation = require_current_lease(&transaction, lease, now_ms)?;
        let stored = load_attempt(&transaction, &lease.operation_id, attempt_id)?
            .ok_or(JournalError::StaleAttempt)?;
        if operation.attempt_count != attempt_id {
            return Err(JournalError::StaleAttempt);
        }
        let (new_state, state_text) = match result {
            ReadbackResult::Applied => (OperationState::AppliedObserved, "applied_observed"),
            ReadbackResult::NotApplied => {
                (OperationState::NotAppliedObserved, "not_applied_observed")
            }
            ReadbackResult::Ambiguous => (OperationState::OutcomeUnknown, "outcome_unknown"),
            ReadbackResult::Partial => (OperationState::PartialObserved, "partial_observed"),
        };
        if stored.state == state_text && stored.evidence_sha256.as_deref() == Some(evidence_sha256)
        {
            let snapshot = snapshot_from_stored(&operation)?;
            transaction.commit().map_err(map_sqlite)?;
            return Ok(snapshot);
        }
        if !matches!(
            operation.state,
            OperationState::OutcomeUnknown
                | OperationState::NotAppliedObserved
                | OperationState::AppliedReported
                | OperationState::PartialReported
        ) {
            return Err(JournalError::OutcomeRequiresReadback);
        }
        if stored.state == "applied_observed" || stored.state == "partial_observed" {
            return Err(JournalError::StaleAttempt);
        }
        if stored.readback_count >= MAX_READBACKS_PER_ATTEMPT {
            return Err(JournalError::LimitExceeded);
        }
        transaction
            .execute(
                "UPDATE attempts SET state = ?3, finished_at_ms = ?4, evidence_sha256 = ?5,
                    readback_count = readback_count + 1
                 WHERE operation_id = ?1 AND attempt_id = ?2",
                params![
                    lease.operation_id,
                    i64::from(attempt_id),
                    state_text,
                    now_ms,
                    evidence_sha256
                ],
            )
            .map_err(map_sqlite)?;
        transaction
            .execute(
                "UPDATE operations SET state = ?2, updated_at_ms = ?3 WHERE operation_id = ?1",
                params![lease.operation_id, new_state.as_str(), now_ms],
            )
            .map_err(map_sqlite)?;
        append_event(
            &transaction,
            NewEvent {
                operation_id: &lease.operation_id,
                kind: JournalEventKind::ReadbackRecorded,
                executor_id: Some(&lease.executor_id),
                fence_epoch: lease.fence_epoch,
                attempt_id: Some(attempt_id),
                occurred_at_ms: now_ms,
                evidence_sha256: Some(evidence_sha256),
            },
        )?;
        let snapshot = load_stored(&transaction, &lease.operation_id)?
            .as_ref()
            .map(snapshot_from_stored)
            .transpose()?
            .ok_or(JournalError::Storage)?;
        transaction.commit().map_err(map_sqlite)?;
        Ok(snapshot)
    }

    pub fn get(&self, operation_id: &str) -> Result<Option<OperationSnapshot>, JournalError> {
        validate_identifier(operation_id)?;
        load_stored(&self.connection, operation_id)?
            .as_ref()
            .map(snapshot_from_stored)
            .transpose()
    }

    pub fn history(&self, operation_id: &str) -> Result<Vec<JournalEvent>, JournalError> {
        validate_identifier(operation_id)?;
        let exists: bool = self
            .connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM operations WHERE operation_id = ?1)",
                [operation_id],
                |row| row.get(0),
            )
            .map_err(map_sqlite)?;
        if !exists {
            return Err(JournalError::NotFound);
        }
        let mut statement = self
            .connection
            .prepare(
                "SELECT sequence, kind, executor_id, fence_epoch, attempt_id,
                    occurred_at_ms, evidence_sha256
                 FROM journal_events WHERE operation_id = ?1 ORDER BY sequence LIMIT ?2",
            )
            .map_err(map_sqlite)?;
        let rows = statement
            .query_map(
                params![operation_id, MAX_EVENTS_PER_OPERATION as i64],
                |row| {
                    let kind_text: String = row.get(1)?;
                    let kind = JournalEventKind::from_str(&kind_text).ok_or_else(|| {
                        rusqlite::Error::InvalidColumnType(1, "kind".to_owned(), Type::Text)
                    })?;
                    let fence: i64 = row.get(3)?;
                    let attempt: Option<i64> = row.get(4)?;
                    Ok(JournalEvent {
                        sequence: row.get(0)?,
                        kind,
                        executor_id: row.get(2)?,
                        fence_epoch: u64::try_from(fence).map_err(|_| {
                            rusqlite::Error::InvalidColumnType(
                                3,
                                "fence_epoch".to_owned(),
                                Type::Integer,
                            )
                        })?,
                        attempt_id: attempt.map(u32::try_from).transpose().map_err(|_| {
                            rusqlite::Error::InvalidColumnType(
                                4,
                                "attempt_id".to_owned(),
                                Type::Integer,
                            )
                        })?,
                        occurred_at_ms: row.get(5)?,
                        evidence_sha256: row.get(6)?,
                    })
                },
            )
            .map_err(map_sqlite)?;
        rows.map(|row| row.map_err(map_sqlite)).collect()
    }
}

struct StoredOperation {
    intent: OperationIntentV1,
    intent_sha256: String,
    state: OperationState,
    owner_id: Option<String>,
    fence_epoch: u64,
    lease_until_ms: Option<i64>,
    attempt_count: u32,
}

struct StoredAttempt {
    state: String,
    evidence_sha256: Option<String>,
    readback_count: u32,
}

fn load_stored(
    connection: &Connection,
    operation_id: &str,
) -> Result<Option<StoredOperation>, JournalError> {
    connection
        .query_row(
            "SELECT operation_id, idempotency_key, project_id, review_binding_sha256,
                artifact_sha256, intent_sha256, state, owner_id, fence_epoch,
                lease_until_ms, attempt_count
             FROM operations WHERE operation_id = ?1",
            [operation_id],
            row_to_stored,
        )
        .optional()
        .map_err(map_sqlite)
}

fn load_by_request(
    connection: &Connection,
    project_id: &str,
    idempotency_key: &str,
) -> Result<Option<StoredOperation>, JournalError> {
    connection
        .query_row(
            "SELECT operation_id, idempotency_key, project_id, review_binding_sha256,
                artifact_sha256, intent_sha256, state, owner_id, fence_epoch,
                lease_until_ms, attempt_count
             FROM operations WHERE project_id = ?1 AND idempotency_key = ?2",
            params![project_id, idempotency_key],
            row_to_stored,
        )
        .optional()
        .map_err(map_sqlite)
}

fn row_to_stored(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredOperation> {
    let intent = OperationIntentV1 {
        operation_id: row.get(0)?,
        idempotency_key: row.get(1)?,
        project_id: row.get(2)?,
        review_binding_sha256: row.get(3)?,
        artifact_sha256: row.get(4)?,
    };
    let state_text: String = row.get(6)?;
    let state = OperationState::from_str(&state_text)
        .ok_or_else(|| rusqlite::Error::InvalidColumnType(6, "state".to_owned(), Type::Text))?;
    let fence: i64 = row.get(8)?;
    let attempts: i64 = row.get(10)?;
    Ok(StoredOperation {
        intent,
        intent_sha256: row.get(5)?,
        state,
        owner_id: row.get(7)?,
        fence_epoch: u64::try_from(fence).map_err(|_| {
            rusqlite::Error::InvalidColumnType(8, "fence_epoch".to_owned(), Type::Integer)
        })?,
        lease_until_ms: row.get(9)?,
        attempt_count: u32::try_from(attempts).map_err(|_| {
            rusqlite::Error::InvalidColumnType(10, "attempt_count".to_owned(), Type::Integer)
        })?,
    })
}

fn snapshot_from_stored(stored: &StoredOperation) -> Result<OperationSnapshot, JournalError> {
    if stored.intent.fingerprint() != stored.intent_sha256 {
        return Err(JournalError::Storage);
    }
    Ok(OperationSnapshot {
        intent: stored.intent.clone(),
        state: stored.state,
        owner_id: stored.owner_id.clone(),
        fence_epoch: stored.fence_epoch,
        lease_until_ms: stored.lease_until_ms,
        attempt_count: stored.attempt_count,
    })
}

fn load_attempt(
    connection: &Connection,
    operation_id: &str,
    attempt_id: u32,
) -> Result<Option<StoredAttempt>, JournalError> {
    connection
        .query_row(
            "SELECT state, evidence_sha256, readback_count FROM attempts
             WHERE operation_id = ?1 AND attempt_id = ?2",
            params![operation_id, i64::from(attempt_id)],
            |row| {
                Ok(StoredAttempt {
                    state: row.get(0)?,
                    evidence_sha256: row.get(1)?,
                    readback_count: row.get(2)?,
                })
            },
        )
        .optional()
        .map_err(map_sqlite)
}

fn require_current_lease(
    connection: &Connection,
    lease: &ExecutionLease,
    now_ms: i64,
) -> Result<StoredOperation, JournalError> {
    let operation = load_stored(connection, &lease.operation_id)?.ok_or(JournalError::NotFound)?;
    if operation.owner_id.as_deref() != Some(lease.executor_id.as_str())
        || operation.fence_epoch != lease.fence_epoch
    {
        return Err(JournalError::StaleFence);
    }
    if operation.lease_until_ms.is_none_or(|until| until <= now_ms) {
        return Err(JournalError::LeaseExpired);
    }
    Ok(operation)
}

struct NewEvent<'a> {
    operation_id: &'a str,
    kind: JournalEventKind,
    executor_id: Option<&'a str>,
    fence_epoch: u64,
    attempt_id: Option<u32>,
    occurred_at_ms: i64,
    evidence_sha256: Option<&'a str>,
}

fn append_event(transaction: &Transaction<'_>, event: NewEvent<'_>) -> Result<(), JournalError> {
    let count: i64 = transaction
        .query_row(
            "SELECT count(*) FROM journal_events WHERE operation_id = ?1",
            [event.operation_id],
            |row| row.get(0),
        )
        .map_err(map_sqlite)?;
    if count >= MAX_EVENTS_PER_OPERATION as i64 {
        return Err(JournalError::LimitExceeded);
    }
    transaction
        .execute(
            "INSERT INTO journal_events (
                operation_id, kind, executor_id, fence_epoch, attempt_id,
                occurred_at_ms, evidence_sha256
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                event.operation_id,
                event.kind.as_str(),
                event.executor_id,
                event.fence_epoch as i64,
                event.attempt_id.map(i64::from),
                event.occurred_at_ms,
                event.evidence_sha256
            ],
        )
        .map_err(map_sqlite)?;
    Ok(())
}

fn validate_identifier(value: &str) -> Result<(), JournalError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > MAX_IDENTIFIER_BYTES
        || !bytes[0].is_ascii_alphanumeric()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(byte))
    {
        return Err(JournalError::InvalidInput);
    }
    Ok(())
}

fn validate_sha256(value: &str) -> Result<(), JournalError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(JournalError::InvalidInput);
    }
    Ok(())
}

fn validate_time(now_ms: i64) -> Result<(), JournalError> {
    if now_ms < 0 {
        return Err(JournalError::InvalidInput);
    }
    Ok(())
}

fn validate_lease(lease_ms: i64) -> Result<(), JournalError> {
    if !(1..=MAX_LEASE_MS).contains(&lease_ms) {
        return Err(JournalError::InvalidInput);
    }
    Ok(())
}

fn lower_hex(bytes: &[u8]) -> String {
    use fmt::Write as _;
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn map_sqlite(error: rusqlite::Error) -> JournalError {
    match error {
        rusqlite::Error::SqliteFailure(code, _) => match code.code {
            rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked => {
                JournalError::Busy
            }
            _ => JournalError::Storage,
        },
        _ => JournalError::Storage,
    }
}
