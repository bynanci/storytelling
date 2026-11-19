#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Offline structural and immutable-evidence checks for the F01 follow-up packet."""
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FOLLOWUP_ROOT = ROOT / "docs/research/evaluations/sk-e06-f01-followup-v1"
BASE_COMMIT = "26602d37d849d06ceac1d4a09f7ca19d2949c990"

EXPECTED_ORIGINAL_ARTIFACTS = {
    "docs/research/evaluations/sk-e06/cases/variant_scope.json": {
        "sha256": "7332ebbb05fc73975bdb8852779fdb950e28a1a34322fbebbc6302cd40e15970",
        "git_blob_sha1": "862e94ff2349e29d87b24e2c67994711083ebe6e",
    },
    "docs/research/evaluations/sk-e06-runs-2026-11-17/inputs/variant_scope.json": {
        "sha256": "c4bb144346c9e6946b450cf5d70a656e0e3edc7212d63bdc31d41a1e599bd735",
        "git_blob_sha1": "534dd7e47dad4d7295825eb10880d22801284168",
    },
    "docs/research/evaluations/sk-e06-runs-2026-11-17/responses/variant_scope.txt": {
        "sha256": "8eeb1ed3815008c146db2890a530a947cdec875ab239cd0a9c61188cd32390ab",
        "git_blob_sha1": "e646497dd64575899d41f202b4875771137d9c0e",
    },
}
EXPECTED_PRESERVED_RECORDS = {
    "docs/research/evaluations/sk-e06-runs-2026-11-17/records/variant_scope.json": "934983ed545adc36fe0bc1b424f160937f8a3e00",
    "docs/research/evaluations/sk-e06-runs-2026-11-17/facilitator-review.json": "169ca3ad535ed721d5f28810ad9b979f6b401081",
    "docs/research/evaluations/sk-e06-runs-2026-11-17/independent-review.json": "bfbb090d0808710336d34412c8dcc64cb849c632",
    "docs/research/evaluations/sk-e06-runs-2026-11-17/capture-seal.json": "11b2e0d158363687b3a5305622a8be8d7e52bae0",
}
EXPECTED_CASE_IDS = ("SK-E06-F01-A", "SK-E06-F01-B")
EXPECTED_LOCALES = ("en", "zh-TW")
OBS = "OBS-DOOR-01@1"
CHOICE_B = "CHOICE-UMBRELLA-01@1"
CHOICE_A_ORAL = "CHOICE-UMBRELLA-02@1"
SUGGESTION = "MODEL-SUGGESTION-01@1"


class F01FollowupError(ValueError):
    """Raised when the supplement or immutable evidence pin is invalid."""


def _duplicate_keys(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise F01FollowupError("Duplicate JSON object key.")
        result[key] = value
    return result


def _nonfinite(_value):
    raise F01FollowupError("Non-finite JSON number.")


def strict_json_loads(raw):
    """Decode bounded UTF-8 JSON; reject duplicate fields and NaN/Infinity."""
    if not isinstance(raw, bytes) or len(raw) > 1_048_576:
        raise F01FollowupError("Invalid or oversized JSON input.")
    try:
        return json.loads(raw.decode("utf-8", errors="strict"),
                          object_pairs_hook=_duplicate_keys, parse_constant=_nonfinite)
    except F01FollowupError:
        raise
    except (UnicodeDecodeError, json.JSONDecodeError, TypeError, ValueError) as error:
        raise F01FollowupError("Invalid UTF-8 JSON input.") from error


def _read_json(path):
    try:
        value = strict_json_loads(path.read_bytes())
    except OSError as error:
        raise F01FollowupError("A required packet file is unavailable.") from error
    if not isinstance(value, dict):
        raise F01FollowupError("Packet JSON must be an object.")
    return value


def _require(condition, message):
    if not condition:
        raise F01FollowupError(message)


def _git_blob_sha1(raw):
    return hashlib.sha1(b"blob " + str(len(raw)).encode("ascii") + b"\0" + raw).hexdigest()


def _verify_pins(root):
    originals, records = {}, {}
    for relative, expected in EXPECTED_ORIGINAL_ARTIFACTS.items():
        try:
            raw = (root / relative).read_bytes()
        except OSError as error:
            raise F01FollowupError("A pinned original artifact is unavailable.") from error
        actual = {"sha256": hashlib.sha256(raw).hexdigest(), "git_blob_sha1": _git_blob_sha1(raw)}
        _require(actual == expected, "A pinned original artifact changed.")
        originals[relative] = True
    for relative, expected in EXPECTED_PRESERVED_RECORDS.items():
        try:
            raw = (root / relative).read_bytes()
        except OSError as error:
            raise F01FollowupError("A preserved review record is unavailable.") from error
        _require(_git_blob_sha1(raw) == expected, "A preserved review record changed.")
        records[relative] = True
    return originals, records


def _expected_maps(case_id):
    if case_id == "SK-E06-F01-A":
        return {
            "spoken_outline": {"repair_observation": OBS, "umbrella_authority": None, "state": "absent_from_this_variant"},
            "illustration_a": {"repair_observation": OBS, "umbrella_authority": None, "state": "absent_from_this_variant"},
            "illustration_b": {"repair_observation": OBS, "umbrella_authority": CHOICE_B, "state": "imagined"},
        }
    if case_id == "SK-E06-F01-B":
        return {
            "spoken_outline": {"repair_observation": OBS, "umbrella_authority": CHOICE_A_ORAL, "state": "imagined"},
            "illustration_a": {"repair_observation": OBS, "umbrella_authority": CHOICE_A_ORAL, "state": "imagined"},
            "illustration_b": {"repair_observation": OBS, "umbrella_authority": CHOICE_B, "state": "imagined"},
        }
    raise F01FollowupError("Unknown successor case.")


def _validate_locales(case):
    locales = case.get("locales", {})
    _require(isinstance(locales, dict) and set(locales) == set(EXPECTED_LOCALES), "Both locales are required.")
    expected_source_ids = set(case.get("source_references", []))
    expected_maps = _expected_maps(case["case_id"])
    for locale in EXPECTED_LOCALES:
        localized = locales[locale]
        _require(isinstance(localized.get("prompt"), str) and localized["prompt"].strip(), "A localized prompt is missing.")
        sources = localized.get("sources")
        _require(isinstance(sources, list), "Localized sources must be a list.")
        source_map = {item.get("source_id"): item for item in sources if isinstance(item, dict)}
        _require(len(source_map) == len(sources) and set(source_map) == expected_source_ids, "Source IDs must match the case.")
        _require(source_map[OBS]["source_kind"] == "supplied_observation", "Observation source kind changed.")
        if case["case_id"] == "SK-E06-F01-A":
            choice, suggestion = source_map[CHOICE_B], source_map[SUGGESTION]
            _require(choice["source_kind"] == "fictional_author_decision" and choice["adoption_status"] == "adopted_in_fixture", "B choice must remain fictional and adopted in-fixture.")
            _require(choice["decision_scope"] == ["illustration_b"] and choice["changes_observation"] is False, "B choice scope changed.")
            _require(suggestion["source_kind"] == "model_suggestion" and suggestion["adoption_status"] == "unadopted" and suggestion["decision_scope"] == [], "Model suggestion was promoted to authority.")
        else:
            choice_b, choice_a = source_map[CHOICE_B], source_map[CHOICE_A_ORAL]
            _require(choice_b["source_kind"] == "fictional_author_decision" and choice_b["decision_scope"] == ["illustration_b"], "B choice scope changed.")
            _require(choice_a["source_kind"] == "fictional_author_decision" and choice_a["adoption_status"] == "adopted_in_fixture", "A/oral needs an explicit fictional decision.")
            _require(choice_a["decision_scope"] == ["spoken_outline", "illustration_a"], "New creative choice scope changed.")
            _require(choice_a["changes_observation"] is False and choice_a["creative_only"] is True, "Creative choice cannot modify the observation.")
        expected = localized.get("expected")
        _require(isinstance(expected, dict) and expected.get("human_adoption_claimed") is False, "A fictional fixture cannot claim human confirmation.")
        deliverables = expected.get("deliverables")
        _require(isinstance(deliverables, list), "Deliverable expectations are missing.")
        actual = {}
        for item in deliverables:
            _require(isinstance(item, dict), "Invalid deliverable expectation.")
            key, sm = item.get("deliverable_id"), item.get("source_map")
            _require(key not in actual and isinstance(sm, dict), "Each deliverable needs a unique source map.")
            actual[key] = {"repair_observation": sm.get("repair_observation"), "umbrella_authority": sm.get("umbrella_authority"), "state": item.get("expected_umbrella_state")}
        _require(actual == expected_maps, "Per-deliverable source map or variant scope changed.")
        failures = localized.get("failure_conditions")
        _require(isinstance(failures, list) and len(failures) >= 4, "Explicit failure conditions are required.")
        _require(all(isinstance(item, dict) and item.get("id") and item.get("condition") for item in failures), "Failure conditions must be named.")


def _validate_cases(packet):
    _require(packet.get("schema_version") == "ske06-f01-successor-cases/v1" and packet.get("packet_id") == "SK-E06-F01-SUCCESSORS-1", "Unsupported case packet identity.")
    _require(packet.get("base_commit") == BASE_COMMIT and packet.get("status") == "design_only_pending_maintainer_decision", "Successor cases must remain pinned and design-only.")
    execution = packet.get("execution", {})
    _require(execution.get("can_dispatch") is False and execution.get("dispatch_authorized") is False, "Case packet must not be dispatchable.")
    _require(all(execution.get(key) == 0 for key in ("model_runs", "human_observations", "human_participants", "host_observations")), "Execution/human/Host results are not allowed.")
    _require(execution.get("holdout") is False and execution.get("narrative_quality_scored") is False and execution.get("efficacy_conclusion") is None, "No holdout, quality or efficacy claim is permitted.")
    cases = packet.get("cases")
    _require(isinstance(cases, list) and all(isinstance(case, dict) for case in cases), "Invalid case list.")
    _require(tuple(case.get("case_id") for case in cases) == EXPECTED_CASE_IDS, "Both successor cases are required in order.")
    for case in cases:
        _require(case.get("case_version") == "1.0.0", "Unexpected case version.")
        _require(case.get("execution") == {"status": "not_run", "dispatch_authorized": False, "raw_output": None, "review_records": []}, "Case contains execution material.")
        _require(case.get("workflows") == ["W1", "W3", "W5"] and case.get("rules") == ["ST-R01", "ST-R03", "ST-R08", "ST-R09"], "Workflow/rule mapping changed.")
        _validate_locales(case)


def validate_followup(followup_root=FOLLOWUP_ROOT, repository_root=ROOT):
    """Validate packet structure and pinned source bytes; never dispatch or score."""
    followup_root, repository_root = Path(followup_root), Path(repository_root)
    adj = _read_json(followup_root / "adjudication-proposal.json")
    cases = _read_json(followup_root / "cases.json")
    _require(adj.get("schema_version") == "ske06-f01-adjudication-proposal/v1" and adj.get("proposal_id") == "SK-E06-F01-ADJUDICATION-2026-11-19", "Unsupported adjudication identity.")
    _require(adj.get("record_kind") == "append_only_adjudication_proposal_not_decision" and adj.get("base_commit") == BASE_COMMIT, "Proposal must remain append-only and base-pinned.")
    _require(adj.get("status") == "proposal_pending_maintainer_decision" and adj.get("maintainer_disposition") is None, "Maintainer disposition must remain pending.")
    _require(adj.get("overall_conformance_status") == "disputed_no_single_label" and adj.get("aggregate_score") is None, "Do not create one score or pass/fail.")
    outcome = adj.get("outcome", {})
    _require(outcome.get("r1_rescore") is False and outcome.get("original_r1_modified") is False, "Original R1 must remain unchanged.")
    axes = adj.get("axis_findings")
    expected_axes = {"factual_fidelity", "creative_authority", "variant_scope", "source_traceability"}
    _require(isinstance(axes, list) and {item.get("axis") for item in axes} == expected_axes, "Four separate axes are required.")
    _require(all(item.get("status") and item.get("finding") and item.get("evidence") for item in axes), "Each axis needs evidence.")
    _require(adj.get("original_evidence") == [{"path": path, **pins} for path, pins in EXPECTED_ORIGINAL_ARTIFACTS.items()], "Original artifact pins differ from baseline.")
    _require(adj.get("preserved_review_records") == [{"path": path, "git_blob_sha1": sha} for path, sha in EXPECTED_PRESERVED_RECORDS.items()], "Review pins differ from baseline.")
    _require(cases.get("source_artifacts") == [{"path": path, **pins} for path, pins in EXPECTED_ORIGINAL_ARTIFACTS.items()], "Successor source pins differ.")
    originals, records = _verify_pins(repository_root)
    _validate_cases(cases)
    return {"scope":"followup_design_integrity_only","case_count":len(cases["cases"]),"locales":list(EXPECTED_LOCALES),"pinned_original_artifact_count":len(originals),"verified_original_artifacts":originals,"verified_preserved_records":records,"can_dispatch":False,"maintainer_decision":"pending"}


def main(argv=None):
    if sys.argv[1:] if argv is None else argv:
        sys.stdout.write(json.dumps({"error":{"code":"sk_e06.f01_followup_invalid","message":"F01 follow-up integrity check failed."}}) + "\n")
        return 2
    try:
        result = validate_followup()
    except F01FollowupError:
        sys.stdout.write(json.dumps({"error":{"code":"sk_e06.f01_followup_invalid","message":"F01 follow-up integrity check failed."}}) + "\n")
        return 1
    sys.stdout.write(json.dumps(result, ensure_ascii=False, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
