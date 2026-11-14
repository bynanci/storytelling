// SPDX-License-Identifier: MIT
//! Normal CLI entry-point regressions for the explicitly enabled draft helper.
use std::process::Command;

#[cfg(not(feature = "draft-video-input"))]
#[test]
fn draft_command_is_absent_from_default_build() {
    let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
        .args(["draft-video-coverage-input", "-"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[cfg(feature = "draft-video-input")]
mod enabled {
    use super::*;
    use serde_json::{Value, json};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::process::{Output, Stdio};
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Directory(PathBuf);
    impl Directory {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let path = std::env::temp_dir().join(format!(
                "storytelling-draft-time-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Directory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/n10-06a-draft")
            .join(name)
    }
    fn base() -> Value {
        serde_json::from_slice(&std::fs::read(fixture("seconds-unresolved.draft.json")).unwrap())
            .unwrap()
    }
    fn run(command: &str, bytes: &[u8], extra: &[&str], directory: &Path) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .args([command, "-"])
            .args(extra)
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        // A malformed/budget-rejected input may close stdin early.
        let _ = child.stdin.take().unwrap().write_all(bytes);
        child.wait_with_output().unwrap()
    }
    fn draft(value: &Value) -> Output {
        let directory = Directory::new();
        run(
            "draft-video-coverage-input",
            &serde_json::to_vec(value).unwrap(),
            &[],
            &directory.0,
        )
    }
    fn line(output: &Output, exit: i32) -> Value {
        assert_eq!(
            output.status.code(),
            Some(exit),
            "stdout={}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(output.stderr.is_empty());
        assert_eq!(
            output.stdout.iter().filter(|byte| **byte == b'\n').count(),
            1
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }
    fn error(value: Value, stage: &str, code: &str, path: Option<&str>) {
        assert_eq!(
            line(&draft(&value), 1),
            json!({"error":{"stage":stage,"code":code,"path":path}})
        );
    }
    fn range(value: &mut Value) -> &mut Value {
        &mut value["coverage"]["mappings"][0]["source_range"]
    }

    #[test]
    fn converts_seconds_and_frames_to_expected_v1_and_preserves_unresolved() {
        for name in [
            "seconds-unresolved",
            "frames-30-unresolved",
            "frames-ntsc-unresolved",
        ] {
            let value: Value = serde_json::from_slice(
                &std::fs::read(fixture(&format!("{name}.draft.json"))).unwrap(),
            )
            .unwrap();
            let expected: Value = serde_json::from_slice(
                &std::fs::read(fixture(&format!("{name}.expected-v1.json"))).unwrap(),
            )
            .unwrap();
            let result = line(&draft(&value), 0);
            assert_eq!(result["coverage"], expected);
            assert_eq!(result["scope"], "declarations_only");
            assert_eq!(result["status"], "declaration_unresolved");
            assert_eq!(result["validation"]["ready_for_planning"], false);
            assert_eq!(
                result["conversions"][0]["original"],
                value["coverage"]["mappings"][0]["source_range"]
            );
            assert_eq!(
                result["conversions"][0]["input_path"],
                "$.coverage.mappings[0].source_range"
            );
            assert_eq!(
                result["conversions"][0]["output_path"],
                "$.mappings[0].source_range"
            );
            let dir = Directory::new();
            let emitted = run(
                "draft-video-coverage-input",
                &serde_json::to_vec(&value).unwrap(),
                &["--emit-v1"],
                &dir.0,
            );
            assert_eq!(line(&emitted, 0), expected);
            let authority = line(
                &run("render-video-coverage", &emitted.stdout, &[], &dir.0),
                0,
            );
            assert_eq!(authority, result["validation"]);
        }
    }
    #[test]
    fn exact_seconds_vectors_include_wide_intermediates_and_long_reduction() {
        let cases: Value =
            serde_json::from_slice(&std::fs::read(fixture("cases.json")).unwrap()).unwrap();
        for case in cases["decimal_success"].as_array().unwrap() {
            let mut value = base();
            range(&mut value)["start"] = json!("0");
            range(&mut value)["end"] = case["input"].clone();
            // Large exact values need a correspondingly declared maximum duration.
            value["coverage"]["asset_catalog"]["assets"][0]["duration"] =
                json!({"numerator":"18446744073709551615","denominator":"1"});
            if case["expected"]["numerator"] == "0" {
                range(&mut value)["start"] = case["input"].clone();
                range(&mut value)["end"] = json!("1");
                assert_eq!(
                    line(&draft(&value), 0)["coverage"]["mappings"][0]["source_range"]["start"],
                    case["expected"]
                );
            } else {
                assert_eq!(
                    line(&draft(&value), 0)["coverage"]["mappings"][0]["source_range"]["end"],
                    case["expected"]
                );
            }
        }
    }
    #[test]
    fn rejects_invalid_seconds_and_overflow_without_returning_coverage() {
        let cases: Value =
            serde_json::from_slice(&std::fs::read(fixture("cases.json")).unwrap()).unwrap();
        for group in ["decimal_reject", "overflow"] {
            for case in cases[group].as_array().unwrap() {
                let mut value = base();
                range(&mut value)["start"] = case["input"].clone();
                error(
                    value,
                    "helper",
                    case["code"].as_str().unwrap(),
                    Some("$.coverage.mappings[0].source_range.start"),
                );
            }
        }
    }
    #[test]
    fn rejects_fps_vfr_unknown_timing_and_timecode() {
        for fps in [
            json!({"numerator":"0","denominator":"1"}),
            json!({"numerator":"30","denominator":"0"}),
            json!({"numerator":"60","denominator":"2"}),
            json!({"numerator":"30.0","denominator":"1"}),
        ] {
            let mut value: Value = serde_json::from_slice(
                &std::fs::read(fixture("frames-30-unresolved.draft.json")).unwrap(),
            )
            .unwrap();
            range(&mut value)["fps"] = fps;
            error(
                value,
                "helper",
                "draft_time.invalid_fps",
                Some("$.coverage.mappings[0].source_range.fps"),
            );
        }
        for timing in [Some("variable"), Some("unknown"), None] {
            let mut value: Value = serde_json::from_slice(
                &std::fs::read(fixture("frames-30-unresolved.draft.json")).unwrap(),
            )
            .unwrap();
            if let Some(timing) = timing {
                range(&mut value)["timing"] = json!(timing);
            } else {
                range(&mut value).as_object_mut().unwrap().remove("timing");
            }
            error(
                value,
                "helper",
                "draft_time.frame_timing_not_supported",
                Some("$.coverage.mappings[0].source_range.timing"),
            );
        }
        let mut value: Value = serde_json::from_slice(
            &std::fs::read(fixture("frames-30-unresolved.draft.json")).unwrap(),
        )
        .unwrap();
        range(&mut value)["start_frame"] = json!("00:00:02;15");
        error(
            value,
            "helper",
            "draft_time.invalid_frame",
            Some("$.coverage.mappings[0].source_range.start_frame"),
        );
    }
    #[test]
    fn frame_conversion_reduces_before_u64_and_rejects_real_overflow() {
        let cases: Value =
            serde_json::from_slice(&std::fs::read(fixture("cases.json")).unwrap()).unwrap();
        for case in cases["frame_boundary_success"].as_array().unwrap() {
            let mut value = base();
            *range(&mut value) = json!({"unit":"frames","start_frame":"0","end_frame":case["frame"],"fps":case["fps"],"timing":"constant"});
            assert_eq!(
                line(&draft(&value), 0)["coverage"]["mappings"][0]["source_range"]["end"],
                case["expected"]
            );
        }
        for case in cases["frame_reject"].as_array().unwrap() {
            let mut value = base();
            *range(&mut value) = json!({"unit":"frames","start_frame":case["frame"],"end_frame":"1","fps":case["fps"],"timing":"constant"});
            error(
                value,
                "helper",
                case["code"].as_str().unwrap(),
                Some("$.coverage.mappings[0].source_range.start_frame"),
            );
        }
    }
    #[test]
    fn preserves_authoritative_v1_range_diagnostics_and_first_error_order() {
        for (start, end) in [("4.5", "2.5"), ("2.5", "2.5"), ("0", "6")] {
            let mut value = base();
            range(&mut value)["start"] = json!(start);
            range(&mut value)["end"] = json!(end);
            value["coverage"]["asset_catalog"]["assets"][0]["duration"] =
                json!({"numerator":"5","denominator":"1"});
            let dir = Directory::new();
            let plain = run(
                "draft-video-coverage-input",
                &serde_json::to_vec(&value).unwrap(),
                &[],
                &dir.0,
            );
            let v = line(&plain, 1);
            assert_eq!(v["error"]["stage"], "v1");
            assert_eq!(v["error"]["code"], "video_coverage.invalid_range");
            value["coverage"]["selected_unit_refs"][0]["revision"] = json!(2);
            error(
                value,
                "v1",
                "video_coverage.stale_unit_revision",
                Some("$.selected_unit_refs[0].revision"),
            );
        }
        let mut value = base();
        value["coverage"]["unknown"] = json!(true);
        error(value, "v1", "video_coverage.unknown_field", Some("$"));
    }
    #[test]
    fn helper_reports_start_before_end_when_both_endpoints_are_invalid() {
        let mut value = base();
        range(&mut value)["start"] = json!("-1");
        range(&mut value)["end"] = json!(5);
        error(
            value,
            "helper",
            "draft_time.invalid_decimal",
            Some("$.coverage.mappings[0].source_range.start"),
        );
        let mut value = base();
        *range(&mut value) = json!({"unit":"frames","start_frame":"-1","end_frame":5,"fps":{"numerator":"30","denominator":"1"},"timing":"constant"});
        error(
            value,
            "helper",
            "draft_time.invalid_frame",
            Some("$.coverage.mappings[0].source_range.start_frame"),
        );
    }

    #[test]
    fn complete_mapping_labels_only_declaration_readiness() {
        let mut value = base();
        let mut second = value["coverage"]["mappings"][0].clone();
        second["unit_ref"] = value["coverage"]["selected_unit_refs"][1].clone();
        value["coverage"]["mappings"][1] = second;
        let result = line(&draft(&value), 0);
        assert_eq!(result["validation"]["ready_for_planning"], true);
        assert_eq!(result["status"], "declaration_ready_for_planning");
        assert_eq!(result["scope"], "declarations_only");
    }
    #[test]
    fn rejects_duplicate_keys_before_lossy_value_decode_and_raw_budgets() {
        let dir = Directory::new();
        for bytes in [
            b"{\"draft_version\":\"a\",\"draft_version\":\"b\"}".as_slice(),
            b"{\"coverage\":{\"x\":1,\"x\":2}}",
        ] {
            assert_eq!(
                line(&run("draft-video-coverage-input", bytes, &[], &dir.0), 1),
                json!({"error":{"stage":"helper","code":"draft_time.duplicate_field","path":"$"}})
            );
        }
        for bytes in [b"{".to_vec(), vec![0xff]] {
            assert_eq!(
                line(&run("draft-video-coverage-input", &bytes, &[], &dir.0), 1)["error"]["code"],
                "draft_time.invalid_json"
            );
        }
        let over = vec![b' '; 1_048_577];
        assert_eq!(
            line(&run("draft-video-coverage-input", &over, &[], &dir.0), 1)["error"]["code"],
            "draft_time.input_too_large"
        );
        let deep = format!("{}0{}", "[".repeat(35), "]".repeat(35));
        assert_eq!(
            line(
                &run("draft-video-coverage-input", deep.as_bytes(), &[], &dir.0),
                1
            )["error"]["code"],
            "draft_time.depth_exceeded"
        );
    }
    #[test]
    fn version_and_closed_helper_shapes_fail_without_echoing_unknown_fields() {
        let mut value = base();
        value["draft_version"] = json!("future");
        error(
            value,
            "helper",
            "draft_time.unsupported_version",
            Some("$.draft_version"),
        );
        let mut value = base();
        value["private-injected-key"] = json!("secret");
        error(value, "helper", "draft_time.invalid_shape", Some("$"));
        let mut value = base();
        range(&mut value)["unexpected"] = json!(true);
        error(
            value,
            "helper",
            "draft_time.invalid_shape",
            Some("$.coverage.mappings[0].source_range"),
        );
    }
    #[test]
    fn file_stdin_replay_is_identical_and_input_directory_unchanged() {
        let dir = Directory::new();
        let bytes = std::fs::read(fixture("seconds-unresolved.draft.json")).unwrap();
        let path = dir.0.join("input.json");
        std::fs::write(&path, &bytes).unwrap();
        std::fs::write(dir.0.join("catalog-sentinel"), b"unchanged").unwrap();
        let listing = || {
            let mut v = std::fs::read_dir(&dir.0)
                .unwrap()
                .map(|e| e.unwrap().file_name())
                .collect::<Vec<_>>();
            v.sort();
            v
        };
        let before = listing();
        let from_file = || {
            Command::new(env!("CARGO_BIN_EXE_storytelling"))
                .arg("draft-video-coverage-input")
                .arg(&path)
                .current_dir(&dir.0)
                .env("PATH", "")
                .output()
                .unwrap()
        };
        let a = from_file();
        let b = from_file();
        let stdin = run("draft-video-coverage-input", &bytes, &[], &dir.0);
        line(&a, 0);
        assert_eq!(a.stdout, b.stdout);
        assert_eq!(a.stdout, stdin.stdout);
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert_eq!(listing(), before);
        assert_eq!(
            std::fs::read(dir.0.join("catalog-sentinel")).unwrap(),
            b"unchanged"
        );
    }
    #[test]
    fn usage_and_io_errors_retain_new_command_exit_conventions() {
        for args in [
            vec!["draft-video-coverage-input"],
            vec!["draft-video-coverage-input", "-", "--bad"],
        ] {
            let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
                .args(args)
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(2));
        }
        let dir = Directory::new();
        let output = Command::new(env!("CARGO_BIN_EXE_storytelling"))
            .arg("draft-video-coverage-input")
            .arg(dir.0.join("missing.json"))
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(3));
    }
}
