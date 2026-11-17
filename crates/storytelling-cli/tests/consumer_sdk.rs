// SPDX-License-Identifier: MIT
// New synthetic facade integration glue; existing fixtures remain at their original paths.
// Mutation checks deliberately target distinct adapter failures: lost fields, reordered
// errors, normalized identity bytes, stale-as-error, borrowed output, and hidden writes.

use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use storytelling_engine::consumer_v1 as sdk;

const PLAN: &[u8] = include_bytes!("../../../examples/minimal-plan.json");
const NARRATIVE: &[u8] =
    include_bytes!("../../../tests/fixtures/narrative-v1/n03-author-reviewed-v1.json");
const LEDGER: &[u8] =
    include_bytes!("../../../tests/fixtures/narrative-v1/n04-revision-history-v1.json");
const COVERAGE: &[u8] =
    include_bytes!("../../../tests/fixtures/narrative-video-coverage-v1/valid-complete.json");
const UNRESOLVED: &[u8] =
    include_bytes!("../../../tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json");
const PLANNER: &[u8] = include_bytes!("../../../tests/fixtures/planner-v1/valid.json");
const PROPOSAL: &[u8] =
    include_bytes!("../../../tests/fixtures/consumer-sdk-v1/proposal-response.json");
const SNAPSHOT: &[u8] =
    include_bytes!("../../../tests/fixtures/openfilm/finite-v1/video-trim.json");
const CONTEXT: &[u8] = include_bytes!("../../../tests/fixtures/story-review-v1/context.json");
const BUNDLE_STDOUT: &[u8] =
    include_bytes!("../../../docs/validation/m4-01-readonly-sdk/baseline-bundle.stdout");
const OTIO_STDOUT: &[u8] =
    include_bytes!("../../../docs/validation/m4-01-readonly-sdk/baseline-otio.stdout");
const COVERAGE_STDOUT: &[u8] = include_bytes!(
    "../../../tests/fixtures/narrative-video-coverage-v1/expected-valid-unresolved.stdout.json"
);

fn root(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(path)
}

fn object(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap()
}

fn line(bytes: &[u8]) -> Vec<u8> {
    let mut result = bytes.to_vec();
    result.push(b'\n');
    result
}

fn payload(stdout: &[u8]) -> &[u8] {
    stdout.strip_suffix(b"\n").expect("one frozen CLI LF")
}

// Only existing prose/coverage receipt checkouts permit CRLF normalization.
// Never call this on proposals, bundles, artifacts, snapshots, or raw diagnostics.
fn text_receipt(bytes: &[u8]) -> Vec<u8> {
    String::from_utf8(bytes.to_vec()).unwrap().replace("\r\n", "\n").into_bytes()
}

fn changed(bytes: &[u8], mutation: impl FnOnce(&mut Value)) -> Vec<u8> {
    let mut value = object(bytes);
    mutation(&mut value);
    serde_json::to_vec(&value).unwrap()
}

struct Sandbox(PathBuf);

impl Sandbox {
    fn new() -> Self {
        static SERIAL: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "storytelling-consumer-sdk-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        std::fs::create_dir(path.join("neighbor")).unwrap();
        std::fs::write(path.join("neighbor/sentinel.txt"), b"declared bytes only\n").unwrap();
        Self(path)
    }

    fn put(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, bytes).unwrap();
        path
    }

    fn run(&self, command: &str, args: &[&std::ffi::OsStr], input: Option<&[u8]>) -> Output {
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
        let mut stdin = child.stdin.take().unwrap();
        if let Some(bytes) = input {
            match stdin.write_all(bytes) {
                Ok(()) => {}
                Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe),
            }
        }
        drop(stdin);
        child.wait_with_output().unwrap()
    }

    fn snapshot(&self) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
        fn collect(at: &Path, base: &Path, files: &mut BTreeMap<PathBuf, Option<Vec<u8>>>) {
            for entry in std::fs::read_dir(at).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let kind = entry.file_type().unwrap();
                let key = path.strip_prefix(base).unwrap().to_owned();
                if kind.is_dir() {
                    files.insert(key, None);
                    collect(&path, base, files);
                } else {
                    assert!(kind.is_file());
                    files.insert(key, Some(std::fs::read(path).unwrap()));
                }
            }
        }
        let mut files = BTreeMap::new();
        collect(&self.0, &self.0, &mut files);
        files
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn exit(output: &Output, code: i32) {
    assert_eq!(output.status.code(), Some(code), "{}", String::from_utf8_lossy(&output.stdout));
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout.last(), Some(&b'\n'));
}

fn file_and_stdin(command: &str, path: &Path, input: &[u8], args: &[&str], code: i32) -> Vec<u8> {
    assert_eq!(std::fs::read(path).unwrap(), input);
    let sandbox = Sandbox::new();
    let before = sandbox.snapshot();
    let suffix: Vec<_> = args.iter().map(std::ffi::OsStr::new).collect();
    let mut file_args = vec![path.as_os_str()];
    file_args.extend(&suffix);
    let file = sandbox.run(command, &file_args, None);
    let mut stdin_args = vec![std::ffi::OsStr::new("-")];
    stdin_args.extend(&suffix);
    let stdin = sandbox.run(command, &stdin_args, Some(input));
    exit(&file, code);
    exit(&stdin, code);
    assert_eq!(file.stdout, stdin.stdout);
    assert_eq!(sandbox.snapshot(), before);
    assert_eq!(std::fs::read(path).unwrap(), input);
    file.stdout
}

fn source_error(error: sdk::SdkErrorV1, operation: &str, expected: &Value) -> Vec<u8> {
    assert_eq!(error.operation(), operation);
    assert_eq!(error.code(), expected["code"].as_str().unwrap());
    assert_eq!(error.path(), expected["path"].as_str());
    let bytes = error.source_details_json().unwrap();
    assert!(!bytes.is_empty());
    assert_eq!(object(&bytes), *expected);
    assert_eq!(error.source_details_json().unwrap(), bytes);
    bytes
}

fn single_error(command: &str, operation: &str, input: &[u8], error: sdk::SdkErrorV1, expected: Value) {
    source_error(error, operation, &expected);
    let sandbox = Sandbox::new();
    let input_path = sandbox.put("untrusted.raw", input);
    let stdout = file_and_stdin(command, &input_path, input, &[], 1);
    let envelope = if command == "render-video-coverage" {
        json!({"error":expected})
    } else {
        json!({"ok":false,"error":expected})
    };
    assert_eq!(object(&stdout), envelope);
}

fn shared_impact() -> Value {
    json!({"id":"shared-observation-change","kind":"source_revision","unit_id":"S02",
        "from_revision":1,"to_revision":2,"affected_variants":["oral","illustration-wide","illustration-close"]})
}

#[test]
fn sdk_g01_plan_validation_has_independent_fields_and_unchanged_cli_bytes() {
    let original = PLAN.to_vec();
    let report = sdk::validate_plan(&original).unwrap();
    let expected = json!({"schema_version":1,"plan_id":"travel-recap","assets":1,"shots":2,
        "scope":"declared-metadata-only"});
    assert_eq!(serde_json::to_value(&report).unwrap(), expected);
    let stdout = file_and_stdin("validate", &root("examples/minimal-plan.json"), &original, &[], 0);
    assert_eq!(stdout, b"{\"ok\":true,\"validation\":{\"assets\":1,\"plan_id\":\"travel-recap\",\"schema_version\":1,\"scope\":\"declared-metadata-only\",\"shots\":2}}\n");
    assert_eq!(object(&stdout)["validation"], serde_json::to_value(report).unwrap());
    assert_eq!(original, PLAN);
    let bad = include_bytes!("../../../tests/fixtures/invalid-source-range.json");
    single_error("validate", "validate_plan", bad, sdk::validate_plan(bad).unwrap_err(),
        json!({"code":"plan.source_out_of_bounds","path":"$.shots[0].source_range.end"}));
}

#[test]
fn sdk_g02_projection_has_literal_provenance_and_the_existing_text_goldens() {
    let original = NARRATIVE.to_vec();
    let projection = sdk::render_narrative(&original).unwrap();
    assert_eq!((projection.schema_version, projection.document_id.as_str(), projection.document_revision), (1,"door-repair",2));
    assert_eq!(projection.purpose, "呈現兩位鄰居一起處理共用木門的小事。");
    assert_eq!(projection.audience, "社區電子報讀者");
    assert_eq!(projection.variants.len(), 3);
    let sdk::NarrativeProjectedVariantV1::SpokenOutline { id, name, beats } = &projection.variants[0] else { panic!("oral variant changed kind") };
    assert_eq!((id.as_str(),name.as_str()), ("oral","口述大綱"));
    let beats: Vec<_> = beats.iter().map(|beat| (beat.id.as_str(),beat.source.unit_id.as_str(),beat.source.revision,beat.source.origin.as_str(),beat.text.as_str())).collect();
    assert_eq!(beats, [
        ("beat-01","S01",1,"supplied_observation","星期六上午，兩位鄰居在共用木門旁準備工具。"),
        ("beat-02","S02",1,"supplied_observation","阿梅扶住門，阿安收緊上方鉸鏈的一顆螺絲。"),
        ("beat-03","S03",1,"supplied_observation","兩人試關一次；這一次門沒有磨到地面。"),
        ("beat-04","I01",1,"author_interpretation","一起處理一件日常小事，可以成為鄰里互助的畫面。")
    ]);
    let sdk::NarrativeProjectedVariantV1::SingleIllustration { id, name, sources, brief } = &projection.variants[1] else { panic!("wide variant changed kind") };
    assert_eq!((id.as_str(),name.as_str()), ("illustration-wide","插畫 A｜兩人與共用門（寬景）"));
    assert_eq!(sources.iter().map(|s| (s.unit_id.as_str(),s.revision,s.origin.as_str())).collect::<Vec<_>>(), [
        ("S01",1,"supplied_observation"),("S02",1,"supplied_observation"),("S04",1,"supplied_observation"),
        ("I01",1,"author_interpretation"),("C01",1,"creative_proposal")]);
    assert_eq!(brief,"平視寬景：兩位鄰居與木門同框，阿梅扶門、阿安在上方鉸鏈工作；工具箱放在門邊。把 C01 的想像紅傘放在畫面後方的圍欄旁。紅傘只屬這幅插畫，不是原始觀察。");
    let sdk::NarrativeProjectedVariantV1::SingleIllustration { id, name, sources, brief } = &projection.variants[2] else { panic!("close variant changed kind") };
    assert_eq!((id.as_str(),name.as_str()), ("illustration-close","插畫 B｜鉸鏈與雙手（近景）"));
    assert_eq!(sources.iter().map(|s| (s.unit_id.as_str(),s.revision,s.origin.as_str())).collect::<Vec<_>>(), [
        ("S02",1,"supplied_observation"),("S03",1,"supplied_observation"),
        ("I01",1,"author_interpretation"),("C02",1,"creative_proposal")]);
    // Reviewed prose mentions excluded C01; provenance is asserted on source tags.
    assert_eq!(brief,"較寬的近景：焦點是上方鉸鏈、螺絲及兩人的手，不要求極近特寫。依 C02 安排工作者手邊的收緊工具與門邊扶門的手；工具不指定種類，地面工具箱留在畫外。門框作局部背景，不另指定室內場景。手部位置、裁切與景深是本版構圖選擇，不是原始觀察；不放 C01 紅傘。");
    let stdout = file_and_stdin("render-narrative", &root("tests/fixtures/narrative-v1/n03-author-reviewed-v1.json"), &original, &[], 0);
    assert_eq!(stdout, text_receipt(include_bytes!("../../../docs/validation/open-issues-2026-11-11/n03-reviewed.stdout")));
    let authoring = include_bytes!("../../../tests/fixtures/narrative-v1/n03-authoring-v1.json");
    assert_eq!(sdk::render_narrative(authoring).unwrap().document_revision,1);
    let stdout = file_and_stdin("render-narrative", &root("tests/fixtures/narrative-v1/n03-authoring-v1.json"),authoring,&[],0);
    assert_eq!(stdout,text_receipt(include_bytes!("../../../docs/validation/open-issues-2026-11-11/n03-render.stdout")));
    let bad = include_bytes!("../../../tests/fixtures/narrative-v1/invalid-dangling-reference.json");
    single_error("render-narrative","render_narrative",bad,sdk::render_narrative(bad).unwrap_err(),json!({"code":"narrative.dangling_reference","path":"$.variants[0].beats[1].unit_ref.unit_id"}));
    assert_eq!(original,NARRATIVE);
}

#[test]
fn sdk_g03_revision_history_keeps_authored_choices_and_exact_direct_impacts() {
    let original = LEDGER.to_vec();
    let report = sdk::track_revisions(&original).unwrap();
    assert_eq!(report.document_id,"door-repair");
    assert_eq!(report.events.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), [
        "suggest-red-umbrella-for-b","decline-red-umbrella-in-a","accept-red-umbrella-in-b",
        "shared-observation-change","review-oral-after-shared-change","review-wide-after-shared-change",
        "review-close-after-shared-change","b-only-detail-change"]);
    assert_eq!(serde_json::to_value(&report.events[3]).unwrap(),shared_impact());
    let local = json!({"id":"b-only-detail-change","kind":"source_revision","unit_id":"C01",
        "from_revision":1,"to_revision":2,"affected_variants":["illustration-close"]});
    assert_eq!(serde_json::to_value(&report.events[7]).unwrap(),local);
    assert_eq!(report.source_history.len(),8);
    for (id,revision,origin,text) in [
        ("S02",1,"supplied_observation","阿梅扶住門；阿安收緊上方鉸鏈的一顆螺絲。"),
        ("S02",2,"supplied_observation","阿安扶住門；阿梅收緊上方鉸鏈的一顆螺絲。"),
        ("C01",1,"creative_proposal","一把想像的紅傘靠在木門旁的圍欄上。"),
        ("C01",2,"creative_proposal","一把想像的紅傘移到構圖 B 的門框左後方。")
    ] {
        let source = report.source_history.iter().find(|s| s.unit_id==id && s.revision==revision).unwrap();
        assert_eq!((source.origin.as_str(),source.text.as_str()),(origin,text));
    }
    assert_eq!(serde_json::to_value(&report.author_decisions).unwrap(),json!([
        {"id":"decline-red-umbrella-in-a","variant_id":"illustration-wide","proposal_ref":{"unit_id":"C01","revision":1},"decision":"declined","variant_revision":2},
        {"id":"accept-red-umbrella-in-b","variant_id":"illustration-close","proposal_ref":{"unit_id":"C01","revision":1},"decision":"accepted","variant_revision":2}]));
    assert_eq!(report.suggestions.len(),1);
    assert_eq!(report.suggestions[0].id,"suggest-red-umbrella-for-b");
    assert_eq!(report.current_variants.iter().map(|v| (v.variant_id.as_str(),v.revision,v.status.as_str())).collect::<Vec<_>>(),[("oral",2,"current"),("illustration-wide",3,"current"),("illustration-close",3,"needs_review")]);
    assert_eq!(serde_json::to_value(&report.current_variants[2].stale_source_refs).unwrap(),json!([{"unit_id":"C01","revision":1}]));
    assert!(report.current_variants[..2].iter().all(|v| v.source_refs.iter().all(|r| r.unit_id!="C01")));
    assert!(report.variant_history.iter().any(|v| v.variant_id=="illustration-wide" && v.revision==1 && v.source_refs.iter().any(|r| r.unit_id=="C01")));
    let path = root("tests/fixtures/narrative-v1/n04-revision-history-v1.json");
    let history = file_and_stdin("track-narrative-revisions",&path,&original,&[],0);
    let history = String::from_utf8(history).unwrap();
    for literal in ["Narrative revision history: door-repair\n","Choice decline-red-umbrella-in-a: illustration-wide C01@1 declined (variant revision 2)","Choice accept-red-umbrella-in-b: illustration-close C01@1 accepted (variant revision 2)","Current oral@2: current","Current illustration-wide@3: current","Current illustration-close@3: needs_review; stale [C01@1]"] {
        assert!(history.contains(literal),"missing literal {literal}");
    }
    for (id,expected,golden) in [
        ("shared-observation-change",shared_impact(),include_bytes!("../../../docs/validation/open-issues-2026-11-11/n04-shared-impact.stdout").as_slice()),
        ("b-only-detail-change",local,include_bytes!("../../../docs/validation/open-issues-2026-11-11/n04-local-impact.stdout").as_slice())
    ] {
        assert_eq!(serde_json::to_value(sdk::query_impact(&original,id).unwrap()).unwrap(),expected);
        assert_eq!(file_and_stdin("query-narrative-impact",&path,&original,&[id],0),text_receipt(golden));
    }
    let self_confirm = include_bytes!("../../../tests/fixtures/narrative-v1/n04-invalid-self-confirm-v1.json");
    single_error("track-narrative-revisions","track_revisions",self_confirm,sdk::track_revisions(self_confirm).unwrap_err(),json!({"code":"revision.untrusted_decision","path":"$.events[0].decision"}));
    source_error(sdk::query_impact(self_confirm,"shared-observation-change").unwrap_err(),"query_impact",&json!({"code":"revision.untrusted_decision","path":"$.events[0].decision"}));
    assert_eq!(original,LEDGER);
}

#[test]
fn sdk_g04_complete_and_unresolved_coverage_preserve_the_result_contract() {
    let complete = sdk::video_coverage(COVERAGE).unwrap();
    let expected = json!({"schema_version":1,"profile":"storytelling.narrative-video-coverage.v1","coverage_id":"two-unit-cut",
        "narrative_ref":{"document_id":"n10-synthetic-story","document_revision":1,"variant_id":"rough-cut"},
        "asset_catalog_ref":{"catalog_id":"synthetic-catalog","revision":1},"selected_unit_refs":[{"unit_id":"S02","revision":1},{"unit_id":"S03","revision":1}],
        "mappings":[{"unit_ref":{"unit_id":"S02","revision":1},"status":"mapped","asset_ref":{"asset_id":"CLIP-01","revision":1},"source_range":{"start":{"numerator":"5","denominator":"2"},"end":{"numerator":"9","denominator":"2"}}},
            {"unit_ref":{"unit_id":"S03","revision":1},"status":"mapped","asset_ref":{"asset_id":"CLIP-01","revision":1},"source_range":{"start":{"numerator":"9","denominator":"2"},"end":{"numerator":"11","denominator":"2"}}}],"ready_for_planning":true,"diagnostics":[]});
    assert_eq!(serde_json::to_value(&complete).unwrap(),expected);
    assert_eq!(object(&file_and_stdin("render-video-coverage",&root("tests/fixtures/narrative-video-coverage-v1/valid-complete.json"),COVERAGE,&[],0)),expected);
    let unresolved = sdk::video_coverage(UNRESOLVED).unwrap();
    assert!(!unresolved.ready_for_planning);
    assert_eq!(serde_json::to_value(&unresolved).unwrap(),object(COVERAGE_STDOUT));
    assert_eq!(line(&serde_json::to_vec(&unresolved).unwrap()),text_receipt(COVERAGE_STDOUT));
    assert_eq!(file_and_stdin("render-video-coverage",&root("tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json"),UNRESOLVED,&[],0),text_receipt(COVERAGE_STDOUT));
}

#[test]
fn sdk_g04_all_coverage_diagnostics_keep_their_predeclared_code_and_path() {
    let mut cases: Vec<(Vec<u8>,&str,&str)> = Vec::new();
    for (file,code,path) in [
        ("invalid-version.json","unsupported_version","$.schema_version"),
        ("invalid-duplicate-field.raw","duplicate_field","$"),
        ("invalid-missing-selected.json","missing_selected_unit","$.selected_unit_refs[1]"),
        ("invalid-stale-unit.json","stale_unit_revision","$.selected_unit_refs[0].revision"),
        ("invalid-dangling-asset.json","dangling_asset","$.mappings[1].asset_ref.asset_id"),
        ("invalid-range.json","invalid_range","$.mappings[0].source_range.end")
    ] {
        cases.push((std::fs::read(root(&format!("tests/fixtures/narrative-video-coverage-v1/{file}"))).unwrap(),code,path));
    }
    // Each independent mutation isolates a dropped or normalized diagnostic.
    cases.push((changed(COVERAGE,|v| v["unexpected"]=json!(true)),"unknown_field","$"));
    cases.push((changed(COVERAGE,|v| v["coverage_id"]=json!(7)),"invalid_shape","$"));
    cases.push((changed(COVERAGE,|v| v["variant_id"]=json!("absent-variant")),"dangling_variant","$.variant_id"));
    cases.push((changed(COVERAGE,|v| v["selected_unit_refs"][0]["unit_id"]=json!("NO-SUCH-UNIT")),"dangling_unit","$.selected_unit_refs[0].unit_id"));
    cases.push((changed(COVERAGE,|v| v["selected_unit_refs"][1]=v["selected_unit_refs"][0].clone()),"duplicate_selection","$.selected_unit_refs[1].unit_id"));
    cases.push((changed(COVERAGE,|v| v["selected_unit_refs"].as_array_mut().unwrap().reverse()),"selection_order_mismatch","$.selected_unit_refs[1].unit_id"));
    cases.push((changed(COVERAGE,|v| v["mappings"][1]=v["mappings"][0].clone()),"duplicate_mapping","$.mappings[1].unit_ref.unit_id"));
    cases.push((changed(COVERAGE,|v| v["mappings"].as_array_mut().unwrap().reverse()),"mapping_order_mismatch","$.mappings[1].unit_ref"));
    cases.push((changed(COVERAGE,|v| {let asset=v["asset_catalog"]["assets"][0].clone();v["asset_catalog"]["assets"].as_array_mut().unwrap().push(asset);}),"duplicate_asset","$.asset_catalog.assets[1].asset_id"));
    cases.push((changed(COVERAGE,|v| v["mappings"][0]["asset_ref"]["revision"]=json!(2)),"stale_asset_revision","$.mappings[0].asset_ref.revision"));
    cases.push((changed(COVERAGE,|v| v["selected_unit_refs"]=json!(vec![json!({"unit_id":"S02","revision":1});513])),"too_many_items","$.selected_unit_refs"));
    for select in [false,true] {
        let input = changed(COVERAGE,|v| {
            v["narrative_document"]["units"].as_array_mut().unwrap().push(json!({"id":"S04","revision":1,"origin":"supplied_observation","text":"An unselected unit.","based_on":[]}));
            v["mappings"][1]["unit_ref"]=json!({"unit_id":"S04","revision":1});
            if select { v["selected_unit_refs"][1]=json!({"unit_id":"S04","revision":1}); }
        });
        cases.push((input,if select {"unit_outside_variant"} else {"mapping_outside_selection"},if select {"$.selected_unit_refs[1].unit_id"} else {"$.mappings[1].unit_ref.unit_id"}));
    }
    let too_deep = format!("{{\"schema_version\":1,\"extra\":{}0{}}}","[".repeat(33),"]".repeat(33));
    cases.push((too_deep.into_bytes(),"depth_exceeded","$"));
    assert_eq!(cases.len(),20); // The 21st profile code is an Ok diagnostic in G04.
    cases.push((changed(COVERAGE,|v| {
        v["selected_unit_refs"][0]["revision"]=json!(2);
        v["mappings"][0]["unit_ref"]["revision"]=json!(2);
        v["mappings"][0]["source_range"]["end"]["numerator"]=json!("99");
    }),"stale_unit_revision","$.selected_unit_refs[0].revision"));
    for (input,suffix,path) in cases {
        let before = input.clone();
        single_error("render-video-coverage","video_coverage",&input,sdk::video_coverage(&input).unwrap_err(),json!({"code":format!("video_coverage.{suffix}"),"path":path}));
        assert_eq!(input,before);
    }
}

#[test]
fn sdk_g05_planner_preserves_manual_order_exact_windows_and_conflict_details() {
    let planning = sdk::plan_story(PLANNER).unwrap();
    let value = serde_json::to_value(&planning).unwrap();
    for flag in ["executable","source_verified","host_verified"] { assert_eq!(value[flag],false); }
    assert_eq!(value["profile"],"storytelling.fixed-window-planner.v1");
    assert_eq!(value["scope"],"declared-fixed-window-plan-only");
    assert_eq!(value["selection_planned"],true);
    assert_eq!(value["duration_analysis"],"exact-fixed-windows");
    assert_eq!(value["content_clock_rate"],"10");
    assert_eq!(value["total_duration"],json!({"numerator":"3","denominator":"10"}));
    assert_eq!(value["target_status"],"met");
    assert_eq!(value["plan"]["plan_id"],"synthetic-fixed-plan");
    assert_eq!(value["plan"]["assets"].as_array().unwrap().iter().map(|v| v["id"].as_str().unwrap()).collect::<Vec<_>>(),["opening","detail"]);
    assert_eq!(value["plan"]["shots"],json!([
        {"id":"shot-000001","asset_id":"detail","source_range":{"start":{"numerator":"1","denominator":"10"},"end":{"numerator":"3","denominator":"10"}}},
        {"id":"shot-000002","asset_id":"opening","source_range":{"start":{"numerator":"0","denominator":"1"},"end":{"numerator":"1","denominator":"10"}}}]));
    assert_eq!(value["selections"][0]["timeline_start"],json!({"numerator":"0","denominator":"1"}));
    assert_eq!(value["selections"][0]["duration"],json!({"numerator":"1","denominator":"5"}));
    assert_eq!(value["selections"][1]["timeline_start"],json!({"numerator":"1","denominator":"5"}));
    assert_eq!(value["selections"][1]["duration"],json!({"numerator":"1","denominator":"10"}));
    assert_eq!(value["selections"][0]["declared_score"],-5);
    assert_eq!(value["selections"][1]["declared_score"],10);
    assert_eq!(value["skipped_candidates"],json!([]));
    assert_eq!(object(&file_and_stdin("plan-story",&root("tests/fixtures/planner-v1/valid.json"),PLANNER,&[],0)),json!({"ok":true,"planning":value}));
    let conflict = include_bytes!("../../../tests/fixtures/planner-v1/conflicting.json");
    single_error("plan-story","plan_story",conflict,sdk::plan_story(conflict).unwrap_err(),json!({"code":"planner.brief_conflict","path":"$.brief","conflicts":[{"kind":"required-excluded","path":"$.brief.manual_selected_asset_ids[1]","asset_ids":["opening"]}]}));
}

#[test]
fn sdk_g06_preparation_uses_one_frozen_response_and_matches_raw_baseline_bytes() {
    let inputs = [PROPOSAL.to_vec(),SNAPSHOT.to_vec(),CONTEXT.to_vec()];
    let before = inputs.clone();
    let prepared = sdk::prepare_story_review(&inputs[0],&inputs[1],&inputs[2]).unwrap();
    assert_eq!(line(prepared.bundle_json()),BUNDLE_STDOUT);
    assert_eq!(line(prepared.compiled_otio()),OTIO_STDOUT);
    assert_eq!(prepared.record().binding_sha256,"ff87f54300ac968089fa6170f1248920a990284c3961f574225088ce85aad932");
    assert_eq!(prepared.record().identity.artifact_sha256,"5404c0b552ddc4ccc394755c0a9a95e262f47f4a579fdca9b41797b920a838d0");
    assert_eq!(prepared.record().identity.plan_sha256,"83840ce9e418f6a343e473bbeaea100e77e1d18543943d4f968598b6d909d235");
    let artifact = object(prepared.compiled_otio());
    assert_eq!(artifact["name"],"proposal-A");
    assert_eq!(artifact["metadata"]["storytelling"]["selected_proposal_json"],std::str::from_utf8(PROPOSAL).unwrap());
    let clips = artifact["tracks"]["children"][0]["children"].as_array().unwrap();
    assert_eq!(clips.len(),1);
    assert_eq!(clips[0]["metadata"]["storytelling"]["occurrence_id"],"shot-000001");
    assert_eq!(clips[0]["source_range"]["start_time"]["value"],1);
    assert_eq!(clips[0]["source_range"]["duration"]["value"],3);
    assert_eq!(clips[0]["source_range"]["duration"]["rate"],3);
    let record = serde_json::to_value(prepared.record()).unwrap();
    for flag in ["executable","approval_granted","source_verified","host_verified"] { assert_eq!(record[flag],false); }
    let sandbox = Sandbox::new();
    let proposal = root("tests/fixtures/consumer-sdk-v1/proposal-response.json");
    let snapshot = root("tests/fixtures/openfilm/finite-v1/video-trim.json");
    let context = root("tests/fixtures/story-review-v1/context.json");
    let directory_before = sandbox.snapshot();
    for (command,expected) in [("prepare-story-review",BUNDLE_STDOUT),("export-story-otio",OTIO_STDOUT)] {
        let file = sandbox.run(command,&[proposal.as_os_str(),snapshot.as_os_str(),context.as_os_str()],None);
        let stdin = sandbox.run(command,&[std::ffi::OsStr::new("-"),snapshot.as_os_str(),context.as_os_str()],Some(PROPOSAL));
        exit(&file,0); exit(&stdin,0);
        assert_eq!(file.stdout,expected);
        assert_eq!(stdin.stdout,expected);
    }
    assert_eq!(sandbox.snapshot(),directory_before);
    for (path,input) in [(&proposal,PROPOSAL),(&snapshot,SNAPSHOT),(&context,CONTEXT)] { assert_eq!(std::fs::read(path).unwrap(),input); }
    assert_eq!(inputs,before);
}

fn freshness(current: bool, stale: &[&str]) -> Value {
    json!({"schema_version":1,"scope":"declared-planner-otio-review-only","current":current,"stale_fields":stale,
        "executable":false,"approval_granted":false,"source_verified":false,"host_verified":false})
}

fn check_cli(bundle: &[u8],proposal: &[u8],snapshot: &[u8],context: &[u8],code: i32) -> Value {
    let sandbox = Sandbox::new();
    let paths = [sandbox.put("bundle.raw",bundle),sandbox.put("proposal.raw",proposal),sandbox.put("snapshot.raw",snapshot),sandbox.put("context.raw",context)];
    let before = sandbox.snapshot();
    let file = sandbox.run("check-story-review",&paths.iter().map(|p| p.as_os_str()).collect::<Vec<_>>(),None);
    let stdin = sandbox.run("check-story-review",&[paths[0].as_os_str(),std::ffi::OsStr::new("-"),paths[2].as_os_str(),paths[3].as_os_str()],Some(proposal));
    exit(&file,code); exit(&stdin,code);
    assert_eq!(file.stdout,stdin.stdout);
    assert_eq!(sandbox.snapshot(),before);
    object(&file.stdout)
}

#[test]
fn sdk_g07_fresh_stale_and_malformed_are_distinct_and_stale_fields_stay_sorted() {
    let bundle = payload(BUNDLE_STDOUT);
    assert_eq!(serde_json::to_value(sdk::check_story_review(bundle,PROPOSAL,SNAPSHOT,CONTEXT).unwrap()).unwrap(),freshness(true,&[]));
    assert_eq!(check_cli(bundle,PROPOSAL,SNAPSHOT,CONTEXT,0),json!({"ok":true,"freshness":freshness(true,&[])}));
    // One added proposal byte changes artifact identity even though its plan is equal.
    let proposal = line(PROPOSAL);
    assert_eq!(serde_json::to_value(sdk::check_story_review(bundle,&proposal,SNAPSHOT,CONTEXT).unwrap()).unwrap(),freshness(false,&["artifact"]));
    assert_eq!(check_cli(bundle,&proposal,SNAPSHOT,CONTEXT,1),json!({"ok":true,"freshness":freshness(false,&["artifact"])}));
    // A coherent selected cut updates all duplicated timing declarations.
    let proposal = changed(PROPOSAL,|v| {
        v["planning"]["plan"]["shots"][0]["source_range"]["end"]["numerator"]=json!("5");
        v["planning"]["total_duration"]=json!({"numerator":"4","denominator":"3"});
        v["planning"]["selections"][0]["duration"]=json!({"numerator":"4","denominator":"3"});
    });
    let expected = freshness(false,&["artifact","plan","report"]);
    assert_eq!(serde_json::to_value(sdk::check_story_review(bundle,&proposal,SNAPSHOT,CONTEXT).unwrap()).unwrap(),expected);
    assert_eq!(check_cli(bundle,&proposal,SNAPSHOT,CONTEXT,1),json!({"ok":true,"freshness":expected}));
    let context = changed(CONTEXT,|v| {v["target_revision"]=json!("read-only-next");v["project_id"]=json!("synthetic-next");});
    let expected = freshness(false,&["context.project_id","context.target_revision"]);
    assert_eq!(serde_json::to_value(sdk::check_story_review(bundle,PROPOSAL,SNAPSHOT,&context).unwrap()).unwrap(),expected);
    assert_eq!(check_cli(bundle,PROPOSAL,SNAPSHOT,&context,1),json!({"ok":true,"freshness":expected}));
    let context = changed(CONTEXT,|v| v["loss_choices"]=json!(["metadata_only_locks","picture_only"]));
    let expected = freshness(false,&["artifact","context.loss_choices","report"]);
    assert_eq!(serde_json::to_value(sdk::check_story_review(bundle,PROPOSAL,SNAPSHOT,&context).unwrap()).unwrap(),expected);
    assert_eq!(check_cli(bundle,PROPOSAL,SNAPSHOT,&context,1),json!({"ok":true,"freshness":expected}));
    let snapshot = changed(SNAPSHOT,|v| v["assets"][0]["uri"]=json!("synthetic://replacement.mov"));
    let expected = freshness(false,&["artifact","composition"]);
    assert_eq!(serde_json::to_value(sdk::check_story_review(bundle,PROPOSAL,&snapshot,CONTEXT).unwrap()).unwrap(),expected);
    assert_eq!(check_cli(bundle,PROPOSAL,&snapshot,CONTEXT,1),json!({"ok":true,"freshness":expected}));
    let snapshot = changed(SNAPSHOT,|v| v["assets"][0]["contentHash"]=json!("c".repeat(64)));
    let expected = json!({"code":"story_review.asset_mismatch","path":"$.planning.plan.assets[0]","losses":[]});
    source_error(sdk::check_story_review(bundle,PROPOSAL,&snapshot,CONTEXT).unwrap_err(),"check_story_review",&expected);
    assert_eq!(check_cli(bundle,PROPOSAL,&snapshot,CONTEXT,1),json!({"ok":false,"error":expected}));
    // Splice inside RawValue: parsing/re-encoding the whole bundle would change more bytes.
    let mut tampered = bundle.to_vec();
    let at = tampered.windows(b"proposal-A".len()).rposition(|w| w==b"proposal-A").unwrap();
    tampered[at] = b'P';
    let expected = json!({"code":"story_review.artifact_digest_mismatch","path":"$.otio","losses":[]});
    source_error(sdk::check_story_review(&tampered,PROPOSAL,SNAPSHOT,CONTEXT).unwrap_err(),"check_story_review",&expected);
    assert_eq!(check_cli(&tampered,PROPOSAL,SNAPSHOT,CONTEXT,1),json!({"ok":false,"error":expected}));
    // Full-detail losses must survive the adapter; a two-field error would lose review choices.
    let context = changed(CONTEXT,|v| v["loss_choices"]=json!([]));
    source_error(sdk::prepare_story_review(PROPOSAL,SNAPSHOT,&context).unwrap_err(),"prepare_story_review",&json!({"code":"otio.loss_choice_required","path":"$.choices","losses":[{"feature":"audio.picture_only","path":"$.assets","disposition":"metadata-only","accepted":false}]}));
}

#[test]
fn sdk_g08_raw_version_duplicate_null_and_rich_errors_preserve_backend_priority() {
    for (command,operation,input) in [("validate","validate_plan",PLAN),("render-narrative","render_narrative",NARRATIVE),("track-narrative-revisions","track_revisions",LEDGER),("render-video-coverage","video_coverage",COVERAGE),("plan-story","plan_story",PLANNER)] {
        let future = changed(input,|v| {v["schema_version"]=json!(2);v["future"]=json!(true);});
        let (error,expected) = match operation {
            "validate_plan" => (sdk::validate_plan(&future).unwrap_err(),json!({"code":"input.invalid_json","path":"$"})),
            "render_narrative" => (sdk::render_narrative(&future).unwrap_err(),json!({"code":"narrative.unsupported_version","path":"$.schema_version"})),
            "track_revisions" => (sdk::track_revisions(&future).unwrap_err(),json!({"code":"narrative.unsupported_version","path":"$.schema_version"})),
            "video_coverage" => (sdk::video_coverage(&future).unwrap_err(),json!({"code":"video_coverage.unsupported_version","path":"$.schema_version"})),
            _ => (sdk::plan_story(&future).unwrap_err(),json!({"code":"planner.unsupported_version","path":"$.schema_version","conflicts":[]})),
        };
        single_error(command,operation,&future,error,expected);
    }
    let malformed = b"{";
    let future_plan = changed(PLAN,|v| v["schema_version"]=json!(2));
    single_error("validate","validate_plan",&future_plan,sdk::validate_plan(&future_plan).unwrap_err(),json!({"code":"plan.unsupported_version","path":"$.schema_version"}));
    let bytes = source_error(sdk::video_coverage(malformed).unwrap_err(),"video_coverage",&json!({"code":"input.invalid_json","path":null}));
    assert_eq!(bytes,b"{\"code\":\"input.invalid_json\",\"path\":null}");
    single_error("render-video-coverage","video_coverage",malformed,sdk::video_coverage(malformed).unwrap_err(),json!({"code":"input.invalid_json","path":null}));
    let cycle = include_bytes!("../../../tests/fixtures/narrative-v1/invalid-multiple-order-cycles.json");
    single_error("render-narrative","render_narrative",cycle,sdk::render_narrative(cycle).unwrap_err(),json!({"code":"narrative.order_conflict","path":"$.relations[2]","relation_ids":["R03","R04","R05"]}));
    let duplicate = include_bytes!("../../../tests/fixtures/narrative-v1/invalid-duplicate-field.json");
    single_error("render-narrative","render_narrative",duplicate,sdk::render_narrative(duplicate).unwrap_err(),json!({"code":"narrative.duplicate_field","path":"$"}));
    source_error(sdk::query_impact(&vec![b' ';1_048_577],"bad id").unwrap_err(),"query_impact",&json!({"code":"narrative.invalid_shape","path":"$.change_id"}));
    source_error(sdk::query_impact(LEDGER,&"a".repeat(129)).unwrap_err(),"query_impact",&json!({"code":"narrative.too_many_items","path":"$.change_id"}));
    for (id,code) in [("missing","revision.change_not_found"),("suggest-red-umbrella-for-b","revision.not_source_change")] {
        source_error(sdk::query_impact(LEDGER,id).unwrap_err(),"query_impact",&json!({"code":code,"path":"$.change_id"}));
    }
    let context = changed(CONTEXT,|v| v["schema_version"]=json!(2));
    source_error(sdk::prepare_story_review(PROPOSAL,SNAPSHOT,&context).unwrap_err(),"prepare_story_review",&json!({"code":"review.unsupported_version","path":"$.schema_version","losses":[]}));
    let proposal = changed(PROPOSAL,|v| v["planning"]["profile"]=json!("storytelling.fixed-window-planner.v2"));
    source_error(sdk::prepare_story_review(&proposal,SNAPSHOT,CONTEXT).unwrap_err(),"prepare_story_review",&json!({"code":"story_review.invalid_proposal_claims","path":"$.planning","losses":[]}));
    let proposal = br#"{"ok":true,"ok":true,"planning":{}}"#;
    source_error(sdk::prepare_story_review(proposal,SNAPSHOT,CONTEXT).unwrap_err(),"prepare_story_review",&json!({"code":"story_review.invalid_proposal_json","path":"$","losses":[]}));
    let snapshot = changed(SNAPSHOT,|v| v["composition"]["tracks"][0]["type"]=json!("audio"));
    source_error(sdk::prepare_story_review(PROPOSAL,&snapshot,CONTEXT).unwrap_err(),"prepare_story_review",&json!({"code":"import.unsupported_feature","path":"$.composition.tracks[0].type","losses":[{"feature":"import.unsupported_feature","path":"$.composition.tracks[0].type","disposition":"unsupported","accepted":false}]}));
    // A false outer ok preserves the valid record binding and reaches profile checking.
    let mut wrong_profile = payload(BUNDLE_STDOUT).to_vec();
    assert!(wrong_profile.starts_with(b"{\"ok\":true,"));
    wrong_profile.splice(6..10,b"false".iter().copied());
    source_error(sdk::check_story_review(&wrong_profile,b"{",b"{",b"{").unwrap_err(),"check_story_review",&json!({"code":"story_review.wrong_profile","path":"$.review","losses":[]}));
}

#[test]
fn sdk_g08_limits_and_multi_input_error_order_are_operation_specific() {
    let oversized = vec![b' ';1_048_577];
    for (operation,error,expected) in [
        ("validate_plan",sdk::validate_plan(&oversized).unwrap_err(),json!({"code":"input.too_large","path":"$"})),
        ("render_narrative",sdk::render_narrative(&oversized).unwrap_err(),json!({"code":"input.too_large","path":"$"})),
        ("track_revisions",sdk::track_revisions(&oversized).unwrap_err(),json!({"code":"input.too_large","path":"$"})),
        ("query_impact",sdk::query_impact(&oversized,"valid-id").unwrap_err(),json!({"code":"input.too_large","path":"$"})),
        ("video_coverage",sdk::video_coverage(&oversized).unwrap_err(),json!({"code":"input.too_large","path":null})),
        ("plan_story",sdk::plan_story(&oversized).unwrap_err(),json!({"code":"input.too_large","path":"$","conflicts":[]}))
    ] { source_error(error,operation,&expected); }
    for (input,operation) in [(PLAN,"validate_plan"),(NARRATIVE,"render_narrative"),(LEDGER,"track_revisions"),(COVERAGE,"video_coverage"),(PLANNER,"plan_story")] {
        let mut exact = input.to_vec(); exact.resize(1_048_576,b' ');
        match operation {
            "validate_plan" => { sdk::validate_plan(&exact).unwrap(); }
            "render_narrative" => { sdk::render_narrative(&exact).unwrap(); }
            "track_revisions" => { sdk::track_revisions(&exact).unwrap(); }
            "video_coverage" => { sdk::video_coverage(&exact).unwrap(); }
            _ => { sdk::plan_story(&exact).unwrap(); }
        }
    }
    for (operation,arrays,expected) in [("render_narrative",31,"narrative.unknown_field"),("render_narrative",32,"narrative.depth_exceeded"),("video_coverage",32,"video_coverage.unknown_field"),("video_coverage",33,"video_coverage.depth_exceeded")] {
        let bytes = format!("{{\"schema_version\":1,\"extra\":{}0{}}}","[".repeat(arrays),"]".repeat(arrays));
        let error = if operation=="render_narrative" { sdk::render_narrative(bytes.as_bytes()).unwrap_err() } else { sdk::video_coverage(bytes.as_bytes()).unwrap_err() };
        source_error(error,operation,&json!({"code":expected,"path":"$"}));
    }
    let p = vec![b' ';4_194_337]; let s = oversized; let c = vec![b' ';65_537]; let b = vec![b' ';4_292_673];
    for (proposal,snapshot,context,code) in [
        (p.as_slice(),s.as_slice(),c.as_slice(),"story_review.proposal_too_large"),
        (b"{".as_slice(),s.as_slice(),c.as_slice(),"input.too_large"),
        (b"{".as_slice(),b"{".as_slice(),c.as_slice(),"review.context_too_large"),
        (b"{".as_slice(),b"{".as_slice(),b"{".as_slice(),"review.invalid_context_json"),
        (b"{".as_slice(),b"{".as_slice(),CONTEXT,"story_review.invalid_proposal_json")
    ] { source_error(sdk::prepare_story_review(proposal,snapshot,context).unwrap_err(),"prepare_story_review",&json!({"code":code,"path":"$","losses":[]})); }
    for (bundle,proposal,snapshot,context,code) in [
        (b.as_slice(),p.as_slice(),s.as_slice(),c.as_slice(),"story_review.bundle_too_large"),
        (b"{".as_slice(),p.as_slice(),s.as_slice(),c.as_slice(),"story_review.proposal_too_large"),
        (b"{".as_slice(),b"{".as_slice(),s.as_slice(),c.as_slice(),"input.too_large"),
        (b"{".as_slice(),b"{".as_slice(),b"{".as_slice(),c.as_slice(),"review.context_too_large"),
        (b"{".as_slice(),b"{".as_slice(),b"{".as_slice(),b"{".as_slice(),"story_review.invalid_bundle_json"),
        (payload(BUNDLE_STDOUT),b"{".as_slice(),b"{".as_slice(),b"{".as_slice(),"review.invalid_context_json")
    ] { source_error(sdk::check_story_review(bundle,proposal,snapshot,context).unwrap_err(),"check_story_review",&json!({"code":code,"path":"$","losses":[]})); }
    let deep = format!("{}0{}","[".repeat(65),"]".repeat(65));
    source_error(sdk::prepare_story_review(deep.as_bytes(),SNAPSHOT,CONTEXT).unwrap_err(),"prepare_story_review",&json!({"code":"review.json_too_deep","path":"$","losses":[]}));
    source_error(sdk::check_story_review(deep.as_bytes(),PROPOSAL,SNAPSHOT,CONTEXT).unwrap_err(),"check_story_review",&json!({"code":"review.json_too_deep","path":"$","losses":[]}));
    let mut snapshot = SNAPSHOT.to_vec(); snapshot.resize(1_048_576,b' ');
    let mut context = CONTEXT.to_vec(); context.resize(65_536,b' ');
    let bound = sdk::prepare_story_review(PROPOSAL,&snapshot,&context).unwrap();
    let mut bundle = bound.bundle_json().to_vec(); bundle.resize(4_292_672,b' ');
    assert!(sdk::check_story_review(&bundle,PROPOSAL,&snapshot,&context).unwrap().current);
    let mut proposal = PROPOSAL.to_vec(); proposal.resize(4_194_336,b' ');
    source_error(sdk::prepare_story_review(&proposal,SNAPSHOT,CONTEXT).unwrap_err(),"prepare_story_review",&json!({"code":"otio.output_too_large","path":"$","losses":[]}));
}

#[derive(Debug,PartialEq)]
struct Replay {
    projection: sdk::NarrativeProjectionV1,
    values: Vec<Value>,
    bundle: Vec<u8>,
    artifact: Vec<u8>,
    error: Vec<u8>,
}

fn replay() -> Replay {
    let inputs: Vec<Vec<u8>> = [PLAN,NARRATIVE,LEDGER,COVERAGE,PLANNER,PROPOSAL,SNAPSHOT,CONTEXT].iter().map(|v| v.to_vec()).collect();
    let before = inputs.clone();
    let projection = sdk::render_narrative(&inputs[1]).unwrap();
    assert_eq!(projection.document_revision,2);
    let prepared = sdk::prepare_story_review(&inputs[5],&inputs[6],&inputs[7]).unwrap();
    assert_eq!(line(prepared.bundle_json()),BUNDLE_STDOUT);
    assert_eq!(line(prepared.compiled_otio()),OTIO_STDOUT);
    let fresh = sdk::check_story_review(prepared.bundle_json(),&inputs[5],&inputs[6],&inputs[7]).unwrap();
    assert_eq!(serde_json::to_value(&fresh).unwrap(),freshness(true,&[]));
    let impact = sdk::query_impact(&inputs[2],"shared-observation-change").unwrap();
    assert_eq!(serde_json::to_value(&impact).unwrap(),shared_impact());
    let error = source_error(sdk::video_coverage(b"{").unwrap_err(),"video_coverage",&json!({"code":"input.invalid_json","path":null}));
    let result = Replay {
        projection,
        values: vec![serde_json::to_value(sdk::validate_plan(&inputs[0]).unwrap()).unwrap(),
            serde_json::to_value(sdk::track_revisions(&inputs[2]).unwrap()).unwrap(),
            serde_json::to_value(impact).unwrap(),serde_json::to_value(sdk::video_coverage(&inputs[3]).unwrap()).unwrap(),
            serde_json::to_value(sdk::plan_story(&inputs[4]).unwrap()).unwrap(),serde_json::to_value(fresh).unwrap()],
        bundle: prepared.bundle_json().to_vec(),artifact: prepared.compiled_otio().to_vec(),error
    };
    assert_eq!(inputs,before);
    result // Every original input Vec is dropped before the caller inspects owned results.
}

#[test]
fn sdk_g09_owned_results_are_send_sync_and_caller_threads_replay_without_mutation() {
    fn owned_thread_safe<T: Send+Sync+'static>() {}
    owned_thread_safe::<sdk::PlanValidationV1>();
    owned_thread_safe::<sdk::NarrativeProjectionV1>();
    owned_thread_safe::<sdk::NarrativeRevisionReportV1>();
    owned_thread_safe::<sdk::NarrativeRevisionEventResultV1>();
    owned_thread_safe::<sdk::NarrativeVideoCoverageResultV1>();
    owned_thread_safe::<sdk::PlannedStoryV1>();
    owned_thread_safe::<sdk::PreparedStoryReviewV1>();
    owned_thread_safe::<sdk::ReviewFreshnessReportV1>();
    owned_thread_safe::<sdk::SdkErrorV1>();
    owned_thread_safe::<sdk::SdkEncodingErrorV1>();
    let expected = replay();
    assert_eq!(replay(),expected);
    let workers: Vec<_> = (0..3).map(|_| std::thread::spawn(replay)).collect();
    for worker in workers { assert_eq!(worker.join().unwrap(),expected); }
    let prepared = { let p=PROPOSAL.to_vec(); let s=SNAPSHOT.to_vec(); let c=CONTEXT.to_vec(); sdk::prepare_story_review(&p,&s,&c).unwrap() };
    // A mutable portable DTO clone must not alter its compiler-owned source handle.
    let mut untrusted = prepared.record().clone();
    untrusted.approval_granted=true;
    assert!(untrusted.approval_granted);
    assert!(!prepared.record().approval_granted);
    assert_eq!(line(prepared.bundle_json()),BUNDLE_STDOUT);
}
