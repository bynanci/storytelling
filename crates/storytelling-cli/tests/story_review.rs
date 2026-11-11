// SPDX-License-Identifier: Apache-2.0
// Independently authored planner-to-OTIO regressions; provenance-m3-planner-review.json.
use serde_json::{Value, json};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

struct Sandbox(PathBuf);

impl Sandbox {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "storytelling-story-review-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn write(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, bytes).unwrap();
        path
    }
    fn run(&self, command: &str, args: &[&str], input: Option<&[u8]>) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .arg(command)
            .args(args)
            .current_dir(&self.0)
            .env("PATH", &self.0)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        if let Some(bytes) = input
            && let Err(error) = child.stdin.take().unwrap().write_all(bytes)
        {
            assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
        }
        child.wait_with_output().unwrap()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn decode(output: &Output, exit: i32) -> Value {
    assert_eq!(
        output.status.code(),
        Some(exit),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout.iter().filter(|b| **b == b'\n').count(), 1);
    serde_json::from_slice(&output.stdout).unwrap()
}

fn snapshot() -> Value {
    serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/openfilm/finite-v1/video-trim.json"
    ))
    .unwrap()
}

fn request(end: &str) -> Value {
    json!({"schema_version":1,"brief":{
        "schema_version":1,"brief_id":"proposal-A","intent":"Synthetic fixed-window proposal",
        "assets":[{"id":"synthetic-video"}],"manual_selected_asset_ids":["synthetic-video"],
        "hard_constraints":[],"preferences":{"preferred_asset_ids":[]}
    },"candidates":[{"asset_id":"synthetic-video","content_hash":"b".repeat(64),
        "duration":{"numerator":"10","denominator":"1"},
        "source_range":{"start":{"numerator":"1","denominator":"3"},"end":{"numerator":end,"denominator":"3"}},"score":10}],
        "max_duration":{"numerator":"10","denominator":"1"},"max_shots":1})
}

fn context() -> Value {
    json!({"schema_version":1,"project_id":"synthetic-project","target_revision":"read-only",
        "host":{"id":"filmcraft","version":"0.18.0","profile_id":"declared-only",
        "binary_sha256":"a".repeat(64),"declared_capabilities":["arbitrary-content-tick-rate","external-reference","single-video-track","still-hold"]},
        "loss_choices":["picture_only"]})
}

fn setup(s: &Sandbox) -> Vec<u8> {
    s.write("snapshot.json", &serde_json::to_vec(&snapshot()).unwrap());
    s.write("context.json", &serde_json::to_vec(&context()).unwrap());
    let proposal = s.run(
        "plan-story",
        &["-"],
        Some(&serde_json::to_vec(&request("4")).unwrap()),
    );
    decode(&proposal, 0);
    s.write("proposal.json", &proposal.stdout);
    proposal.stdout
}

#[test]
fn story_review_connects_actual_planner_output_to_exact_otio_without_source_or_host_writes() {
    let s = Sandbox::new();
    let proposal = setup(&s);
    s.write("OpenFilm-Catalog.sqlite", b"synthetic catalog sentinel");
    s.write("FilmCraft-project.json", b"synthetic host sentinel");
    let before: Vec<_> = [
        "proposal.json",
        "snapshot.json",
        "context.json",
        "OpenFilm-Catalog.sqlite",
        "FilmCraft-project.json",
    ]
    .map(|name| (name, std::fs::read(s.0.join(name)).unwrap()))
    .into_iter()
    .collect();
    let output = s.run(
        "prepare-story-review",
        &["proposal.json", "snapshot.json", "context.json"],
        None,
    );
    let bundle = decode(&output, 0);
    let again = s.run(
        "prepare-story-review",
        &["-", "snapshot.json", "context.json"],
        Some(&proposal),
    );
    assert_eq!(output.stdout, again.stdout);
    assert_eq!(bundle["otio"]["name"], "proposal-A");
    let clip = &bundle["otio"]["tracks"]["children"][0]["children"][0];
    assert_eq!(
        clip["metadata"]["storytelling"]["occurrence_id"],
        "shot-000001"
    );
    assert_eq!(clip["source_range"]["start_time"]["value"], 1);
    assert_eq!(clip["source_range"]["duration"]["value"], 3);
    assert_eq!(clip["source_range"]["duration"]["rate"], 3);
    let report = &bundle["otio"]["metadata"]["storytelling"]["report"];
    assert!(
        report["feature_matrix"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["feature"] == "audio.picture_only" && r["accepted"] == true)
    );
    for flag in [
        "executable",
        "approval_granted",
        "source_verified",
        "host_verified",
    ] {
        assert_eq!(bundle["review"][flag], false);
    }
    s.write("review.json", &output.stdout);
    let check = s.run(
        "check-story-review",
        &[
            "review.json",
            "proposal.json",
            "snapshot.json",
            "context.json",
        ],
        None,
    );
    assert_eq!(decode(&check, 0)["freshness"]["current"], true);
    let otio = s.run(
        "export-story-otio",
        &["proposal.json", "snapshot.json", "context.json"],
        None,
    );
    assert_eq!(decode(&otio, 0), bundle["otio"]);
    for (name, bytes) in before {
        assert_eq!(std::fs::read(s.0.join(name)).unwrap(), bytes);
    }
    assert_eq!(std::fs::read_dir(&s.0).unwrap().count(), 6);
}

#[test]
fn story_review_invalidates_asset_cut_loss_and_detects_displayed_artifact_tampering() {
    let s = Sandbox::new();
    let original_proposal = setup(&s);
    let prepared = s.run(
        "prepare-story-review",
        &["proposal.json", "snapshot.json", "context.json"],
        None,
    );
    let bundle = decode(&prepared, 0);
    s.write("review.json", &prepared.stdout);
    let changed = s.run(
        "plan-story",
        &["-"],
        Some(&serde_json::to_vec(&request("5")).unwrap()),
    );
    decode(&changed, 0);
    s.write("proposal.json", &changed.stdout);
    let stale = s.run(
        "check-story-review",
        &[
            "review.json",
            "proposal.json",
            "snapshot.json",
            "context.json",
        ],
        None,
    );
    assert_eq!(decode(&stale, 1)["freshness"]["current"], false);
    s.write("proposal.json", &original_proposal);
    for (field, value) in [
        ("uri", json!("synthetic://replacement.mov")),
        ("contentHash", json!("c".repeat(64))),
    ] {
        let mut changed = snapshot();
        changed["assets"][0][field] = value;
        s.write("snapshot.json", &serde_json::to_vec(&changed).unwrap());
        let result = decode(
            &s.run(
                "check-story-review",
                &[
                    "review.json",
                    "proposal.json",
                    "snapshot.json",
                    "context.json",
                ],
                None,
            ),
            1,
        );
        assert!(result["freshness"]["current"] != true);
    }
    s.write("snapshot.json", &serde_json::to_vec(&snapshot()).unwrap());
    let mut changed_context = context();
    changed_context["loss_choices"] = json!(["metadata_only_locks", "picture_only"]);
    s.write(
        "context.json",
        &serde_json::to_vec(&changed_context).unwrap(),
    );
    let stale = decode(
        &s.run(
            "check-story-review",
            &[
                "review.json",
                "proposal.json",
                "snapshot.json",
                "context.json",
            ],
            None,
        ),
        1,
    );
    assert!(
        stale["freshness"]["stale_fields"]
            .as_array()
            .unwrap()
            .contains(&json!("context.loss_choices"))
    );
    s.write("context.json", &serde_json::to_vec(&context()).unwrap());
    let mut tampered = bundle;
    tampered["otio"]["tracks"]["children"][0]["children"][0]["source_range"]["duration"]["value"] =
        json!(2);
    s.write("review.json", &serde_json::to_vec(&tampered).unwrap());
    let failed = decode(
        &s.run(
            "check-story-review",
            &[
                "review.json",
                "proposal.json",
                "snapshot.json",
                "context.json",
            ],
            None,
        ),
        1,
    );
    assert_eq!(
        failed["error"]["code"],
        "story_review.artifact_digest_mismatch"
    );
}

#[test]
fn story_review_rejects_unsupported_audio_effects_missing_loss_choices_and_bad_cli_usage() {
    let s = Sandbox::new();
    setup(&s);
    for audio in [false, true] {
        let mut changed = snapshot();
        if audio {
            changed["composition"]["tracks"][0]["type"] = json!("audio");
        } else {
            changed["composition"]["tracks"][0]["clips"][0]["effects"] = json!([{"type":"blur"}]);
        }
        s.write("snapshot.json", &serde_json::to_vec(&changed).unwrap());
        let result = decode(
            &s.run(
                "prepare-story-review",
                &["proposal.json", "snapshot.json", "context.json"],
                None,
            ),
            1,
        );
        assert_eq!(result["error"]["code"], "import.unsupported_feature");
        assert!(result.get("otio").is_none());
        assert_eq!(result["error"]["losses"][0]["disposition"], "unsupported");
    }
    s.write("snapshot.json", &serde_json::to_vec(&snapshot()).unwrap());
    let mut no_loss = context();
    no_loss["loss_choices"] = json!([]);
    s.write("context.json", &serde_json::to_vec(&no_loss).unwrap());
    let missing = decode(
        &s.run(
            "prepare-story-review",
            &["proposal.json", "snapshot.json", "context.json"],
            None,
        ),
        1,
    );
    assert_eq!(missing["error"]["code"], "otio.loss_choice_required");
    for command in [
        "prepare-story-review",
        "check-story-review",
        "export-story-otio",
    ] {
        decode(&s.run(command, &[], None), 2);
        decode(&s.run(command, &["--unknown"], None), 2);
    }
    decode(
        &s.run(
            "prepare-story-review",
            &["proposal.json", "-", "context.json"],
            None,
        ),
        2,
    );
    decode(
        &s.run(
            "check-story-review",
            &["-", "proposal.json", "snapshot.json", "context.json"],
            None,
        ),
        2,
    );
    let io = decode(
        &s.run(
            "prepare-story-review",
            &["missing.json", "snapshot.json", "context.json"],
            None,
        ),
        3,
    );
    assert_eq!(io["error"], json!({"code":"input.io","path":"$"}));
}
