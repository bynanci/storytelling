#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Check SK-E06 packet integrity, never model behavior or human performance."""

import argparse
import hashlib
import json
import re
from pathlib import Path, PurePosixPath


EXPECTED_CASES = {
    "SK-E06-VARIANT-01": "variant_scope",
    "SK-E06-INDIRECT-01": "indirect_source_removal",
    "SK-E06-INJECTION-01": "untrusted_instruction_and_consent",
    "SK-E06-EVIDENCE-01": "invented_human_evidence",
    "SK-E06-ARTIFACT-01": "unavailable_artifact_tools",
}
SKILL_PATHS = {
    "skills/storytelling/SKILL.md",
    "skills/storytelling/SKILL.zh-TW.md",
    "skills/storytelling/references/research-workflows.md",
    "skills/storytelling/references/research-workflows.zh-TW.md",
    "skills/storytelling/references/editorial-rules.md",
    "skills/storytelling/references/editorial-rules.zh-TW.md",
}
LABELS = {
    "en": "[Authored reference example; not a captured model or human output.]",
    "zh-TW": "[編寫的參考範例；不是擷取的模型或真人輸出。]",
}
HASH = re.compile(r"^[0-9a-f]{64}$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")
SOURCE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*@[1-9][0-9]*$")


class PacketError(ValueError):
    """Packet integrity failed; this is not a conformance score."""


def _require(condition, message):
    if not condition:
        raise PacketError(message)


def _keys(value, required, label):
    _require(isinstance(value, dict) and set(value) == set(required), f"{label}: unexpected or missing fields")


def _text(value, label):
    _require(isinstance(value, str) and bool(value.strip()), f"{label}: expected nonempty text")


def _texts(value, label, minimum=1):
    _require(isinstance(value, list) and len(value) >= minimum, f"{label}: incomplete list")
    for item in value:
        _text(item, label)
    _require(len(set(value)) == len(value), f"{label}: duplicate entry")


def _strict_object(pairs):
    result = {}
    for key, value in pairs:
        _require(key not in result, "JSON contains a duplicate field")
        result[key] = value
    return result


def _non_json_constant(_value):
    raise PacketError("JSON contains a non-finite constant")


def _read_json(path, label):
    try:
        data = path.read_bytes()
        _require(b"\r" not in data, f"{label}: expected canonical LF bytes")
        return json.loads(data.decode("utf-8"), object_pairs_hook=_strict_object, parse_constant=_non_json_constant)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise PacketError(f"{label}: cannot read strict UTF-8 JSON") from error


def load_packet(root):
    return _read_json(Path(root) / "packet-v1.json", "packet-v1.json")


def _artifact_path(root, name):
    _text(name, "artifact path")
    relative = PurePosixPath(name)
    _require(not relative.is_absolute() and re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._/-]*", name)
             and all(part not in {"", ".", ".."} for part in relative.parts), "artifact path escapes the packet")
    path = root.joinpath(*relative.parts)
    _require(path.resolve().is_relative_to(root.resolve()), "artifact path escapes the packet")
    cursor = root
    for part in relative.parts:
        cursor = cursor / part
        _require(not cursor.is_symlink(), "packet artifact must not be a symlink")
    _require(path.is_file(), "required packet artifact is missing")
    return path


def _blank_form(form, packet_id):
    groups = {
        "host": {"name", "version", "version_unavailable_reason", "capabilities"},
        "skill": {"commit", "files_sha256"},
        "model": {"availability", "provider", "model_id", "revision", "revision_unavailable_reason", "settings"},
        "raw_output": {"verbatim", "utf8_sha256"},
        "review": {"invariant_results", "failure_conditions_seen", "exact_output_evidence", "unexpected_answer_verbatim",
                   "unresolved_questions", "reviewer_code", "instruction_conformance", "narrative_quality", "human_trial_outcomes"},
    }
    scalar = {"record_id", "case_id", "locale", "execution_kind", "date_utc", "started_at_utc", "ended_at_utc",
              "fresh_context", "prior_exposure", "decision"}
    _keys(form, {"schema_version", "record_kind", "packet_id", *scalar, *groups}, "manual form")
    _require(form["schema_version"] == "skill-adversarial-manual-record/v1"
             and form["record_kind"] == "blank_manual_execution_form"
             and form["packet_id"] == packet_id, "manual form: wrong identity or version")
    _require(all(form[field] is None for field in scalar), "manual form must not prepopulate execution or decisions")
    for name, keys in groups.items():
        _keys(form[name], keys, f"manual form {name}")
        _require(all(value is None for value in form[name].values()), f"manual form {name} must remain blank")


def _case(case):
    _keys(case, {"case_id", "pattern", "workflows", "rules", "source_references", "locales"}, "case")
    case_id = case["case_id"]
    _require(isinstance(case_id, str) and case_id in EXPECTED_CASES, "case: unknown case ID")
    _require(case["pattern"] == EXPECTED_CASES[case_id], "case: required pattern mismatch")
    _texts(case["workflows"], "case workflows")
    _require(set(case["workflows"]) <= {f"W{i}" for i in range(1, 7)}, "case: unknown workflow")
    _texts(case["rules"], "case rules")
    _require(set(case["rules"]) <= {f"ST-R{i:02d}" for i in range(1, 15)}, "case: unknown rule")
    _texts(case["source_references"], "case source references")
    _require(all(SOURCE.fullmatch(item) for item in case["source_references"]), "case: invalid source/revision reference")
    _keys(case["locales"], LABELS, "case locales")
    for language, material in case["locales"].items():
        _keys(material, {"prompt", "sources", "reference_output_kind", "reference_output", "expected_invariants",
                         "failure_conditions", "reference_literal_checks"}, f"case {language}")
        _text(material["prompt"], "case prompt")
        _require(material["reference_output_kind"] == "authored_example_not_execution", "case: reference output is not execution evidence")
        reference = material["reference_output"]
        _text(reference, "reference output")
        _require(reference.startswith(LABELS[language] + "\n"), "reference output must display its authored-example label")
        _texts(material["expected_invariants"], "expected invariants", 3)
        _texts(material["failure_conditions"], "failure conditions", 3)
        sources = material["sources"]
        _require(isinstance(sources, list) and bool(sources), "case: source fixtures missing")
        references = []
        for source in sources:
            _keys(source, {"source_id", "revision", "kind", "text"}, "source fixture")
            _text(source["source_id"], "source ID")
            _require(type(source["revision"]) is int and source["revision"] > 0, "source: revision must be a positive integer")
            _text(source["kind"], "source kind")
            _text(source["text"], "source text")
            references.append(f'{source["source_id"]}@{source["revision"]}')
        _require(len(references) == len(set(references)) and set(references) == set(case["source_references"]),
                 "case: source reference/revision does not resolve")
        checks = material["reference_literal_checks"]
        _keys(checks, {"must_contain", "must_not_contain"}, "reference literal checks")
        for name in checks:
            _texts(checks[name], f"reference {name}")
        lowered = reference.casefold()
        _require(all(token.casefold() in lowered for token in checks["must_contain"]), "authored reference is missing a required literal")
        _require(all(token.casefold() not in lowered for token in checks["must_not_contain"]), "authored reference contains a forbidden literal")
    return case_id


def validate_packet(packet, root):
    """Validate declared packet files/labels/literal checks, not semantic outcomes."""
    root = Path(root)
    _keys(packet, {"schema_version", "packet_id", "task", "base_commit", "prepared_on_utc", "record_kind", "reference_authoring",
                   "evidence_state", "observed_execution_records", "human_trial_results", "model_evaluation_results",
                   "efficacy_conclusion", "narrative_quality_scored", "holdout", "skill_snapshot", "case_files", "manual_form",
                   "artifacts_sha256"}, "packet")
    _require(packet["schema_version"] == "skill-adversarial-reference-packet/v1", "packet: unsupported version")
    _require(packet["packet_id"] == "SK-E06-REFERENCE-1" and packet["task"] == "SK-E06 / #115", "packet: wrong identity")
    _require(packet["record_kind"] == "synthetic_reference_examples_and_manual_protocol"
             and packet["evidence_state"] == "protocol_and_reference_examples_only", "packet: reference examples cannot become observed runs")
    _text(packet["reference_authoring"], "reference authoring")
    _require("not captured" in packet["reference_authoring"], "packet: authoring limits must be explicit")
    _require(packet["observed_execution_records"] == [] and packet["human_trial_results"] is None
             and packet["model_evaluation_results"] is None and packet["efficacy_conclusion"] is None
             and packet["narrative_quality_scored"] is False and packet["holdout"] is False,
             "packet: absent execution/efficacy results must remain absent")
    _require(isinstance(packet["base_commit"], str) and COMMIT.fullmatch(packet["base_commit"]), "packet: base commit is not pinned")
    _require(isinstance(packet["prepared_on_utc"], str) and re.fullmatch(r"\d{4}-\d{2}-\d{2}", packet["prepared_on_utc"]),
             "packet: preparation date is missing")
    snapshot = packet["skill_snapshot"]
    _keys(snapshot, {"commit", "files_sha256"}, "skill snapshot")
    _require(isinstance(snapshot["commit"], str) and COMMIT.fullmatch(snapshot["commit"]), "packet: skill commit is not pinned")
    _require(isinstance(snapshot["files_sha256"], dict) and set(snapshot["files_sha256"]) == SKILL_PATHS,
             "packet: scoped skill entry/workflow/rule snapshot hashes are missing")
    _require(all(isinstance(value, str) and HASH.fullmatch(value) for value in snapshot["files_sha256"].values()),
             "packet: malformed skill snapshot hash")
    names = packet["case_files"]
    _texts(names, "case files", 5)
    _require(len(names) == 5, "packet: expected five independent patterns")
    _text(packet["manual_form"], "manual form path")
    artifacts = packet["artifacts_sha256"]
    _require(isinstance(artifacts, dict) and set(artifacts) == {*names, packet["manual_form"]},
             "packet: artifact manifest differs from required files")
    paths = {}
    for name, digest in artifacts.items():
        _require(isinstance(digest, str) and HASH.fullmatch(digest), "packet: malformed artifact SHA-256")
        path = _artifact_path(root, name)
        try:
            actual = hashlib.sha256(path.read_bytes()).hexdigest()
        except OSError as error:
            raise PacketError("packet artifact cannot be read") from error
        _require(actual == digest, "packet artifact checksum mismatch")
        paths[name] = path
    case_ids = [_case(_read_json(paths[name], name)) for name in names]
    _require(len(set(case_ids)) == 5 and set(case_ids) == set(EXPECTED_CASES), "packet: duplicated or missing case")
    _blank_form(_read_json(paths[packet["manual_form"]], packet["manual_form"]), packet["packet_id"])
    return {"packet_id": packet["packet_id"], "case_count": len(case_ids), "locale_count": 2 * len(case_ids),
            "scope": "reference_packet_integrity_only"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("packet_dir", nargs="?", type=Path,
                        default=Path(__file__).resolve().parents[1] / "docs/research/evaluations/sk-e06")
    args = parser.parse_args()
    try:
        result = validate_packet(load_packet(args.packet_dir), args.packet_dir)
    except PacketError as error:
        print(json.dumps({"error": {"code": "sk_e06.packet_invalid", "message": str(error)}}))
        return 1
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
