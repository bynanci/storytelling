// SPDX-License-Identifier: Apache-2.0
// Explicit provided-byte consumer regression; see provenance-m1-03.json.
use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

const ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

fn verify_stdin(bytes: &[u8], expected: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .args(["verify-source", "-", expected])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn accepts_explicit_source_bytes_with_metadata_only_receipt() {
    let output = verify_stdin(b"abc", ABC);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let receipt: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        receipt,
        serde_json::json!({"ok": true, "verification": {
            "schema_version": 1, "algorithm": "sha256", "scope": "provided-buffer-bytes",
            "byte_length": 3, "sha256": ABC
        }})
    );
}

#[test]
fn rejects_bad_source_evidence_without_leaking_input() {
    for (bytes, expected, code) in [
        (
            b"private media bytes".as_slice(),
            ABC,
            "source.hash_mismatch",
        ),
        (b"abc".as_slice(), "BAD", "source.invalid_expected_hash"),
    ] {
        let output = verify_stdin(bytes, expected);
        assert_eq!(output.status.code(), Some(1));
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["error"]["code"], code);
        assert!(!String::from_utf8_lossy(&output.stdout).contains("private media"));
        assert!(output.stderr.is_empty());
    }
    let root = std::env::temp_dir().join(format!("storytelling-source-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let file = root.join("input.bin");
    std::fs::write(&file, b"abc").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("verify-source")
        .arg(&file)
        .arg(ABC)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(std::fs::read(&file).unwrap(), b"abc");
    std::fs::write(&file, vec![0; 16_777_217]).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .arg("verify-source")
        .arg(&file)
        .arg("invalid")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["error"]["code"], "source.too_large");
    assert_eq!(std::fs::metadata(&file).unwrap().len(), 16_777_217);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn distinguishes_source_usage_and_io_errors() {
    for args in [
        vec!["verify-source"],
        vec!["verify-source", "-"],
        vec!["verify-source", "-", ABC, "extra"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
    }
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .args(["verify-source", "/does-not-exist/private-source", ABC])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("private-source"));
}
