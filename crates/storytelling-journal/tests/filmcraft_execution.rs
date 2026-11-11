//! Manual Linux integration proof for the bounded FilmCraft M3-03 profile.
//!
//! Run only through `tests/oracles/m3-03-filmcraft-v1/run.py`, which creates a
//! unique synthetic run root and supplies a pinned host and fresh story review.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use storytelling_journal::{
    DispatchResponse, Journal, JournalEventKind, OperationIntentV1, OperationSnapshot,
    OperationState, ReadbackResult,
};

const OWNER: &str = "m3-03-proof-executor";
const LEASE_MS: i64 = 300_000;
const FILMCRAFT_TICKS_PER_SECOND: i64 = 254_016_000_000;

fn env_path(name: &str) -> PathBuf {
    let path = PathBuf::from(std::env::var_os(name).unwrap_or_else(|| panic!("missing {name}")));
    assert!(path.is_absolute(), "{name} must be absolute");
    path
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before Unix epoch")
        .as_millis()
        .try_into()
        .expect("millisecond time fits i64")
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn run_host(host: &Path, data_dir: &Path, project: Option<&Path>, args: &[&str]) -> Vec<u8> {
    let mut command = Command::new(host);
    command.arg("--data-dir").arg(data_dir);
    if let Some(project) = project {
        command.arg("--project").arg(project);
    }
    let output = command
        .args(args)
        .env("XDG_CONFIG_HOME", data_dir.join("xdg-config"))
        .env("XDG_DATA_HOME", data_dir.join("xdg-data"))
        .env("XDG_CACHE_HOME", data_dir.join("xdg-cache"))
        .env("XDG_STATE_HOME", data_dir.join("xdg-state"))
        .env("FILMCRAFT_DEV_DESKTOP_ENTRY", "0")
        .output()
        .expect("FilmCraft command starts");
    assert!(
        output.status.success(),
        "FilmCraft command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty(), "unexpected FilmCraft stderr");
    output.stdout
}

fn apply_intent(
    journal: &mut Journal,
    operation_id: &str,
    request_id: &str,
    project_id: &str,
    binding: &str,
    artifact: &str,
) -> (OperationIntentV1, storytelling_journal::ExecutionLease, u32) {
    let intent = OperationIntentV1::new(operation_id, request_id, project_id, binding, artifact)
        .expect("valid immutable operation identity");
    journal
        .prepare(&intent, now_ms())
        .expect("intent persisted before dispatch");
    let lease = journal
        .claim(operation_id, OWNER, now_ms(), LEASE_MS)
        .expect("single proof executor claims operation");
    let attempt = journal
        .begin_attempt(&lease, now_ms())
        .expect("write-ahead attempt marker persisted before host call");
    (intent, lease, attempt)
}

fn state_name(state: OperationState) -> &'static str {
    match state {
        OperationState::Ready => "ready",
        OperationState::Dispatching => "dispatching",
        OperationState::OutcomeUnknown => "outcome_unknown",
        OperationState::NotAppliedObserved => "not_applied_observed",
        OperationState::AppliedReported => "applied_reported",
        OperationState::AppliedObserved => "applied_observed",
        OperationState::PartialReported => "partial_reported",
        OperationState::PartialObserved => "partial_observed",
    }
}

fn event_name(kind: JournalEventKind) -> &'static str {
    match kind {
        JournalEventKind::Prepared => "prepared",
        JournalEventKind::AttemptStarted => "attempt_started",
        JournalEventKind::ResponseRecorded => "response_recorded",
        JournalEventKind::RecoveredUnknown => "recovered_unknown",
        JournalEventKind::ReadbackRecorded => "readback_recorded",
    }
}

fn summary(snapshot: &OperationSnapshot, journal: &Journal) -> Value {
    let events = journal
        .history(snapshot.intent.operation_id())
        .expect("journal history remains readable")
        .into_iter()
        .map(|event| {
            json!({
                "kind": event_name(event.kind),
                "executor_id": event.executor_id,
                "fence_epoch": event.fence_epoch,
                "attempt_id": event.attempt_id,
                "evidence_sha256": event.evidence_sha256,
            })
        })
        .collect::<Vec<_>>();
    json!({
        "operation_id": snapshot.intent.operation_id(),
        "project_id": snapshot.intent.project_id(),
        "review_binding_sha256": snapshot.intent.review_binding_sha256(),
        "artifact_sha256": snapshot.intent.artifact_sha256(),
        "state": state_name(snapshot.state),
        "owner_id": snapshot.owner_id,
        "fence_epoch": snapshot.fence_epoch,
        "attempt_count": snapshot.attempt_count,
        "events": events,
    })
}

fn write_readback(
    host: &Path,
    data_dir: &Path,
    project: &Path,
    root: &Path,
    stem: &str,
) -> (Value, Vec<u8>, Vec<u8>) {
    let project_json = run_host(host, data_dir, Some(project), &["inspect", "project"]);
    let project_value: Value =
        serde_json::from_slice(&project_json).expect("project JSON readback");
    let native_json = fs::read(project).expect("native project file readback");
    let sequence_json = run_host(host, data_dir, Some(project), &["inspect", "sequence"]);
    let _: Value = serde_json::from_slice(&sequence_json).expect("sequence JSON readback");
    fs::write(
        root.join(format!("{stem}-project-readback.json")),
        &project_json,
    )
    .expect("save project readback evidence");
    fs::write(
        root.join(format!("{stem}-native-project-readback.json")),
        &native_json,
    )
    .expect("save native project readback evidence");
    fs::write(
        root.join(format!("{stem}-sequence-readback.json")),
        &sequence_json,
    )
    .expect("save sequence readback evidence");
    (project_value, native_json, sequence_json)
}

fn write_empty_project_readback(
    host: &Path,
    data_dir: &Path,
    project: &Path,
    root: &Path,
) -> (Value, Vec<u8>) {
    let project_json = run_host(host, data_dir, Some(project), &["inspect", "project"]);
    let project_value: Value =
        serde_json::from_slice(&project_json).expect("project JSON readback");
    let native_json = fs::read(project).expect("native empty-project file readback");
    fs::write(root.join("empty-project-readback.json"), &project_json)
        .expect("save empty project readback evidence");
    fs::write(
        root.join("empty-native-project-readback.json"),
        &native_json,
    )
    .expect("save empty native project evidence");
    (project_value, native_json)
}

#[test]
#[ignore = "requires the pinned FilmCraft host and a fresh isolated run root"]
fn isolated_import_with_lost_reply_is_journaled_unknown_then_resolved_by_native_readback() {
    let root = env_path("M3_03_RUN_ROOT");
    let host = env_path("M3_03_FILMCRAFT");
    let story = env_path("M3_03_STORYTELLING");
    let python = env_path("M3_03_PYTHON");
    let wrapper = env_path("M3_03_DELAYED_REPLY_WRAPPER");
    let marker = env_path("M3_03_HOST_EXIT_MARKER");
    let stop_signal = env_path("M3_03_STOP_SIGNAL");
    let result_path = env_path("M3_03_RESULT_JSON");
    let proposal_path = root.join("proposal.json");
    let snapshot_path = root.join("snapshot.json");
    let context_path = root.join("context.json");
    let bundle_path = root.join("review-bundle.json");
    let otio_path = root.join("review.otio");
    let database_path = root.join("operations.sqlite");
    let data_dir = root.join("filmcraft-data");
    let empty_project = root.join("empty.fcproj");
    let imported_project = root.join("imported.fcproj");

    assert!(root.is_dir(), "run harness must create a fresh root");
    assert!(!database_path.exists(), "journal database must be new");
    assert!(!empty_project.exists() && !imported_project.exists());
    assert!(host.is_file() && story.is_file() && python.is_file() && wrapper.is_file());
    assert!(marker.is_absolute() && stop_signal.is_absolute() && result_path.is_absolute());

    let proposal = fs::read(&proposal_path).unwrap();
    let snapshot = fs::read(&snapshot_path).unwrap();
    let context: Value = serde_json::from_slice(&fs::read(&context_path).unwrap()).unwrap();
    let bundle: Value = serde_json::from_slice(&fs::read(&bundle_path).unwrap()).unwrap();
    let artifact = fs::read(&otio_path).unwrap();
    let binding = bundle["review"]["binding_sha256"].as_str().unwrap();
    let artifact_sha = bundle["review"]["identity"]["artifact_sha256"]
        .as_str()
        .unwrap();
    let project_id = context["project_id"].as_str().unwrap();
    assert_eq!(sha256(&artifact), artifact_sha);
    assert_eq!(
        sha256(&proposal),
        bundle["otio"]["metadata"]["storytelling"]["proposal_sha256"]
    );

    let freshness = Command::new(&story)
        .args([
            "check-story-review",
            bundle_path.to_str().unwrap(),
            proposal_path.to_str().unwrap(),
            snapshot_path.to_str().unwrap(),
            context_path.to_str().unwrap(),
        ])
        .output()
        .expect("freshness checker starts");
    assert!(freshness.status.success(), "review inputs must be current");
    let freshness_json: Value = serde_json::from_slice(&freshness.stdout).unwrap();
    assert!(freshness_json["freshness"]["current"].as_bool().unwrap());
    assert!(!freshness_json["freshness"]["executable"].as_bool().unwrap());
    assert!(
        !freshness_json["freshness"]["approval_granted"]
            .as_bool()
            .unwrap()
    );

    let original_inputs = [
        sha256(&proposal),
        sha256(&snapshot),
        sha256(&fs::read(&context_path).unwrap()),
        sha256(&fs::read(&bundle_path).unwrap()),
        artifact_sha.to_owned(),
    ];
    let source_path = root.join("source.mp4");
    let source_sha_before_host = sha256(&fs::read(&source_path).unwrap());
    let mut journal = Journal::open(&database_path).expect("fresh SQLite journal opens");

    // Persist the isolated blank-project creation as its own operation.
    let (_, create_lease, create_attempt) = apply_intent(
        &mut journal,
        "m3-03-create-project",
        "m3-03-create-request",
        project_id,
        binding,
        artifact_sha,
    );
    let create_stdout = run_host(
        &host,
        &data_dir,
        None,
        &[
            "--save-as",
            empty_project.to_str().unwrap(),
            "exec",
            "file.newProject",
            "name=Review",
        ],
    );
    assert!(
        empty_project.is_file(),
        "empty project is isolated under run root"
    );
    journal
        .record_response(
            &create_lease,
            create_attempt,
            DispatchResponse::Applied,
            now_ms(),
            &sha256(&create_stdout),
        )
        .unwrap();
    let (empty_state, empty_bytes) =
        write_empty_project_readback(&host, &data_dir, &empty_project, &root);
    assert!(empty_state["activeSequence"].is_null());
    let empty_evidence = sha256(&empty_bytes);
    let created = journal
        .record_readback(
            &create_lease,
            create_attempt,
            ReadbackResult::Applied,
            now_ms(),
            &empty_evidence,
        )
        .unwrap();
    assert_eq!(created.state, OperationState::AppliedObserved);

    // The write-ahead marker commits before the host import command is spawned.
    let (_, import_lease, import_attempt) = apply_intent(
        &mut journal,
        "m3-03-import-proposal-a",
        "m3-03-import-request-a",
        project_id,
        binding,
        artifact_sha,
    );
    assert_eq!(import_attempt, 1);
    assert!(!imported_project.exists());
    let empty_project_sha_before_import = sha256(&fs::read(&empty_project).unwrap());
    let mut child = Command::new(&python)
        .arg(&wrapper)
        .arg(&host)
        .arg(&data_dir)
        .arg(&empty_project)
        .arg(&imported_project)
        .arg(&otio_path)
        .arg(&marker)
        .arg(&stop_signal)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("delayed-reply fault injector starts");
    let deadline = Instant::now() + Duration::from_secs(120);
    while !marker.exists() {
        if let Some(status) = child.try_wait().expect("fault injector wait") {
            panic!("fault injector returned before host completion marker: {status}");
        }
        if Instant::now() >= deadline {
            // Ask the wrapper to kill and reap its host child before returning.
            // No second import is issued, even if this later readback is unclear.
            fs::write(&stop_signal, b"stop").expect("request host-child cleanup");
            let cleanup_deadline = Instant::now() + Duration::from_secs(10);
            while !marker.exists() && Instant::now() < cleanup_deadline {
                thread::sleep(Duration::from_millis(20));
            }
            if !marker.exists() {
                let _ = child.kill();
                let _ = child.wait();
                panic!("fault injector did not reap the timed-out host child");
            }
            break;
        }
        thread::sleep(Duration::from_millis(20));
    }
    let marker_status = fs::read_to_string(&marker).expect("host completion marker readable");

    // The host finished but its acknowledgement is deliberately withheld. Kill
    // the waiting wrapper, persist Unknown, and reconcile from the native file.
    child
        .kill()
        .expect("stop response wrapper after known child exit");
    let _ = child.wait().expect("response wrapper is reaped");
    journal
        .record_response(
            &import_lease,
            import_attempt,
            DispatchResponse::Unknown,
            now_ms(),
            &sha256(b"m3-03-lost-reply-after-host-exit"),
        )
        .unwrap();
    assert_eq!(
        journal
            .get("m3-03-import-proposal-a")
            .unwrap()
            .unwrap()
            .state,
        OperationState::OutcomeUnknown,
        "lost acknowledgement is durable before readback"
    );
    assert_eq!(marker_status, "0", "FilmCraft import itself must succeed");

    let (project_state, project_bytes, sequence_bytes) =
        write_readback(&host, &data_dir, &imported_project, &root, "imported");
    assert!(project_state["activeSequence"].is_number());
    let sequence: Value = serde_json::from_slice(&sequence_bytes).unwrap();
    assert_eq!(sequence["durationFrames"], 25);
    assert_eq!(sequence["settings"]["frame_rate"]["num"], 25);
    assert_eq!(sequence["settings"]["frame_rate"]["den"], 1);
    assert_eq!(sequence["video"].as_array().unwrap().len(), 1);
    let clips = sequence["video"][0]["items"].as_array().unwrap();
    assert_eq!(
        clips.len(),
        1,
        "one selected proposal yields one native clip"
    );
    assert_eq!(clips[0]["startFrame"], 0);
    assert_eq!(clips[0]["endFrame"], 25);
    assert_eq!(clips[0]["durationFrames"], 25);
    assert_eq!(clips[0]["sourceIn"], FILMCRAFT_TICKS_PER_SECOND / 25);
    assert_eq!(clips[0]["sourceOut"], FILMCRAFT_TICKS_PER_SECOND * 26 / 25);
    let native: Value = serde_json::from_slice(&project_bytes).unwrap();
    let media_id = clips[0]["item"].as_i64().unwrap().to_string();
    let media = &native["project"]["items"][media_id]["kind"]["Media"];
    assert_eq!(
        media["media"]["File"]["path"],
        root.join("source.mp4").to_str().unwrap()
    );
    assert_eq!(media["offline"], false);
    assert_eq!(media["info"]["kind"], "Movie");
    assert_eq!(
        sha256(&fs::read(&empty_project).unwrap()),
        empty_project_sha_before_import,
        "the source empty project remains unchanged after save-as import"
    );
    let current_inputs = [
        sha256(&fs::read(&proposal_path).unwrap()),
        sha256(&fs::read(&snapshot_path).unwrap()),
        sha256(&fs::read(&context_path).unwrap()),
        sha256(&fs::read(&bundle_path).unwrap()),
        sha256(&fs::read(&otio_path).unwrap()),
    ];
    assert_eq!(
        current_inputs, original_inputs,
        "review inputs stay immutable"
    );
    assert_eq!(
        sha256(&fs::read(&source_path).unwrap()),
        source_sha_before_host,
        "source media stays immutable"
    );
    let import_evidence = sha256(&[project_bytes, sequence_bytes].concat());
    let imported = journal
        .record_readback(
            &import_lease,
            import_attempt,
            ReadbackResult::Applied,
            now_ms(),
            &import_evidence,
        )
        .expect("readback resolves the unknown outcome");
    assert_eq!(imported.state, OperationState::AppliedObserved);
    assert_eq!(
        imported.attempt_count, 1,
        "no duplicate host apply occurred"
    );
    assert_eq!(
        journal
            .begin_attempt(&import_lease, now_ms())
            .unwrap_err()
            .code(),
        "journal.terminal_state",
        "resolved operation cannot be applied again"
    );
    let late_ack = journal.record_response(
        &import_lease,
        import_attempt,
        DispatchResponse::Applied,
        now_ms(),
        &sha256(b"m3-03-late-ack"),
    );
    assert!(
        late_ack.is_err(),
        "late response cannot change readback state"
    );

    let import_snapshot = journal.get("m3-03-import-proposal-a").unwrap().unwrap();
    let create_snapshot = journal.get("m3-03-create-project").unwrap().unwrap();
    assert_eq!(import_snapshot.intent.artifact_sha256(), artifact_sha);
    assert_eq!(import_snapshot.intent.review_binding_sha256(), binding);
    assert_eq!(
        journal
            .history("m3-03-import-proposal-a")
            .unwrap()
            .iter()
            .filter(|event| event.kind == JournalEventKind::AttemptStarted)
            .count(),
        1
    );
    drop(journal);

    let journal = Journal::open(&database_path).expect("SQLite journal survives reopen");
    let durable_import = journal.get("m3-03-import-proposal-a").unwrap().unwrap();
    assert_eq!(durable_import.state, OperationState::AppliedObserved);
    assert_eq!(durable_import.attempt_count, 1);
    let durable_create = journal.get("m3-03-create-project").unwrap().unwrap();
    assert_eq!(durable_create.state, OperationState::AppliedObserved);
    let result = json!({
        "schema_version": 1,
        "status": "passed",
        "profile": "filmcraft-25-picture-only-v1",
        "scope": "synthetic isolated-project lost-ack/readback proof; no product executor or trusted approval",
        "project_id": project_id,
        "review_binding_sha256": binding,
        "artifact_sha256": artifact_sha,
        "freshness_current": freshness_json["freshness"]["current"],
        "freshness_approval_granted": freshness_json["freshness"]["approval_granted"],
        "freshness_executable": freshness_json["freshness"]["executable"],
        "host_readback": {
            "active_sequence": project_state["activeSequence"],
            "duration_frames": sequence["durationFrames"],
            "frame_rate": sequence["settings"]["frame_rate"],
            "clip_count": clips.len(),
            "native_media_online": media["offline"] == false,
        },
        "create_operation": summary(&create_snapshot, &journal),
        "import_operation": summary(&durable_import, &journal),
        "source_and_review_input_sha256": original_inputs,
        "source_sha256": source_sha_before_host,
        "source_unchanged": true,
        "review_inputs_unchanged": true,
        "host_project_sha256": sha256(&fs::read(&imported_project).unwrap()),
        "empty_project_sha256": empty_project_sha_before_import,
        "empty_project_unchanged": true,
        "readback_evidence_sha256": import_evidence,
        "lost_ack_reconciliation": {
            "host_process_exit": "success",
            "caller_response": "unknown",
            "state_before_readback": "outcome_unknown",
            "readback": "applied",
            "final_state": state_name(durable_import.state),
            "late_response_rejected": late_ack.is_err(),
        },
        "attempts": 1,
        "retry_count": 0,
        "undo_count": 0,
    });
    fs::write(
        result_path,
        serde_json::to_vec_pretty(&result).expect("serialize bounded proof result"),
    )
    .expect("write proof result");
}
