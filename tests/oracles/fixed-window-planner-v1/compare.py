#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Check frozen source cases and independently declared native planner expectations."""

from __future__ import annotations

import argparse
from fractions import Fraction
import hashlib
import json
from math import gcd
from pathlib import Path
import platform
import re
import subprocess
import sys


ROOT = Path(__file__).resolve().parent
REPO = ROOT.parents[2]
SOURCE_SHA = "7add4762fe8174d76f43b863375ea39b73121e76"
MAX_U64 = (1 << 64) - 1
CASE_IDS = [
    "one-image", "manual-order", "exclude-high-score", "required-above-soft-target",
    "required-hard-maximum", "required-short-video", "soft-target-shortfall",
    "all-excluded", "declared-score", "ascii-tie", "skip-fixed-window-too-long",
    "explicit-preference-over-score",
]
SOURCE_FILES = [
    {"path": "packages/core/src/models.ts", "blob": "58dbe93f132577842b7971e0d8aad61aff1d78c6"},
    {"path": "packages/core/src/validation.ts", "blob": "ba60475333f60fd9f48c77df2e34f0e7cac8536f"},
    {"path": "packages/solver/src/index.ts", "blob": "c701b985495c8c76292880b5acbc9c062b03006c"},
    {"path": "packages/story/src/index.ts", "blob": "da4d47ce45772d402db57b11659cc71f1b71e946"},
]
REPORT_KEYS = {
    "schema_version", "profile", "scope", "executable", "source_verified", "host_verified",
    "selection_planned", "duration_analysis", "content_clock_rate", "total_duration",
    "target_status", "brief_audit", "plan", "selections", "skipped_candidates",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def compact(value: object) -> bytes:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), allow_nan=False).encode()


def parse(data: bytes) -> dict:
    def unique(pairs: list[tuple[str, object]]) -> dict:
        result = {}
        for key, value in pairs:
            require(key not in result, "duplicate JSON property")
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=unique,
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)))


def exact(time: dict) -> Fraction:
    require(isinstance(time, dict) and set(time) == {"numerator", "denominator"},
            "unexpected time shape")
    n, d = time["numerator"], time["denominator"]
    require(isinstance(n, str) and isinstance(d, str)
            and re.fullmatch(r"0|[1-9][0-9]*", n) is not None
            and re.fullmatch(r"[1-9][0-9]*", d) is not None,
            "noncanonical integer time strings")
    numerator, denominator = int(n), int(d)
    require(numerator <= MAX_U64 and denominator <= MAX_U64
            and gcd(numerator, denominator) == 1, "unsafe or unreduced time")
    return Fraction(numerator, denominator)


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(REPO), *args], stderr=subprocess.PIPE)


def invoke(command: list[str], data: bytes | None = None) -> subprocess.CompletedProcess:
    result = subprocess.run(command, input=data, capture_output=True, check=False, timeout=30)
    require(not result.stderr, "unexpected CLI stderr")
    require(result.stdout.endswith(b"\n") and result.stdout.count(b"\n") == 1,
            "CLI must return one JSON line")
    return result


def source_evidence(output: Path) -> tuple[dict, bytes]:
    raw = (output / "source-evidence.json").read_bytes()
    source = parse(raw)
    require(source["sourceHead"] == SOURCE_SHA and source["allPassed"] is True,
            "source revision/result mismatch")
    require(source["sourceFiles"] == SOURCE_FILES, "source blob identities changed")
    require(source["nodeVersion"] == "v24.22.1" and source["tsxVersion"] == "4.23.15",
            "source tool identities changed")
    require(all(re.fullmatch(r"[a-f0-9]{64}", source[name]) is not None
                for name in ("nodeBinarySha256", "tsxLoaderSha256", "tsxPackageSha256")),
            "source executable/loader identity missing")
    require(source["runnerSha256"] == sha256((ROOT / "run-source.mts").read_bytes()),
            "runner changed; execute source oracle again")
    require(source["dependencyLockPath"] == "tests/oracles/openfilm-finite-v1/package-lock.json",
            "dependency lock path changed")
    require(source["dependencyLockSha256"] ==
            sha256((REPO / source["dependencyLockPath"]).read_bytes()) ==
            "6697b505f83230df597922faea37a88535458366ca554a7cb52ec3cdc3b9e535",
            "test dependency lock changed")
    require([case["id"] for case in source["cases"]] == CASE_IDS,
            "frozen twelve cases/order changed")
    for case in source["cases"]:
        require(case["inputPath"] == f"inputs/{case['id']}.json", "case path changed")
        require(case["inputUnchanged"] is True, "source input mutation recorded")
        require(sha256(compact(case["sourceInput"])) == case["sourceInputSha256"],
                "source input hash mismatch")
        observed = case["observedSource"]
        if observed["outcome"] == "ok":
            normalized = observed["normalizedComposition"]
            require("id" not in normalized and all("id" not in track and
                    all("id" not in clip for clip in track["clips"])
                    for track in normalized["tracks"]), "generated IDs not normalized as declared")
            wanted = normalized
        else:
            require(observed["outcome"] == "error" and
                    observed["name"] == "CompositionConstraintError", "unexpected source error")
            wanted = {"name": observed["name"], "message": observed["message"]}
        require(sha256(compact(wanted)) == observed["normalizedOutputSha256"],
                "normalized source output hash mismatch")
    return source, raw


def check_success(cli: Path, document: dict, expected: dict, report: dict) -> dict:
    require(set(report) == REPORT_KEYS, "planner report shape changed")
    require(type(report["schema_version"]) is int and report["schema_version"] == 1
            and report["profile"] == "storytelling.fixed-window-planner.v1"
            and report["scope"] == "declared-fixed-window-plan-only"
            and report["executable"] is False and report["source_verified"] is False
            and report["host_verified"] is False and report["selection_planned"] is True
            and report["duration_analysis"] == "exact-fixed-windows", "planner authority changed")
    require(len(compact(report)) <= 4_194_304, "report output budget exceeded")
    require(report["content_clock_rate"] == expected["contentClockRate"], "planning clock differs")
    rate = int(report["content_clock_rate"])
    require(0 < rate <= MAX_U64, "unbounded planning clock")
    require(report["target_status"] == expected["targetStatus"]
            and exact(report["total_duration"]) == exact(expected["totalDuration"]),
            "frozen total/soft target outcome differs")
    audit_process = invoke([str(cli), "audit-brief", "-"], compact(document["brief"]))
    audit_response = parse(audit_process.stdout)
    require(audit_process.returncode == 0 and audit_response["ok"] is True,
            "known source case must have a consistent brief")
    audit = audit_response["audit"]
    require(report["brief_audit"] == audit, "original actual brief audit changed")

    # Selection/order expectations are literals frozen before Rust implementation.
    selected = expected["selectedAssetIds"]
    require([row["asset_id"] for row in report["selections"]] == selected,
            "selected semantic order differs from the frozen expectation")
    facts = {candidate["asset_id"]: (index, candidate)
             for index, candidate in enumerate(document["candidates"])}
    required = {binding["asset_id"]: binding["origins"] for binding in audit["required_assets"]}
    expected_assets = [{"id": c["asset_id"], "content_hash": c["content_hash"], "duration": c["duration"]}
                       for c in document["candidates"] if c["asset_id"] in selected]
    expected_shots = [{"id": f"shot-{index:06d}", "asset_id": asset_id,
                       "source_range": facts[asset_id][1]["source_range"]}
                      for index, asset_id in enumerate(selected, 1)]
    expected_plan = {"schema_version": 1, "plan_id": document["brief"]["brief_id"],
                     "assets": expected_assets, "shots": expected_shots}
    require(report["plan"] == expected_plan, "nested plan identities/facts/order changed")
    require(len(compact(report["plan"])) <= 1_048_576, "nested M0 plan budget exceeded")
    validation = invoke([str(cli), "validate", "-"], compact(report["plan"]))
    checked = parse(validation.stdout)
    require(validation.returncode == 0 and checked["ok"] is True
            and checked["validation"]["scope"] == "declared-metadata-only",
            "actual M0 consumer refused the nested plan or expanded authority")

    position = Fraction(0)
    for index, (row, asset_id) in enumerate(zip(report["selections"], selected), 1):
        candidate_index, candidate = facts[asset_id]
        require(set(row) == {"asset_id", "shot_id", "required", "declared_score",
                             "timeline_start", "duration", "reason_paths"}, "decision shape changed")
        window = candidate["source_range"]
        start, end = exact(window["start"]), exact(window["end"])
        length = end - start
        require(Fraction(0) <= start < end <= exact(candidate["duration"]), "candidate source bounds")
        require(row["shot_id"] == f"shot-{index:06d}" and
                row["required"] is (asset_id in required) and
                row["declared_score"] == candidate["score"], "decision identities/score facts changed")
        require(exact(row["timeline_start"]) == position and exact(row["duration"]) == length,
                "exact source-window duration/contiguous placement changed")
        for value in (start, end, length, position):
            ticks = value * rate
            require(ticks.denominator == 1 and 0 <= ticks <= MAX_U64,
                    "native range/placement not exactly bounded at shared clock")
        if asset_id in required:
            origins = ["$.brief" + path[1:] for path in required[asset_id]]
        else:
            preferred = document["brief"]["preferences"]["preferred_asset_ids"]
            origins = ([f"$.brief.preferences.preferred_asset_ids[{preferred.index(asset_id)}]"]
                       if asset_id in preferred else [])
            origins += [f"$.candidates[{candidate_index}].score"]
        origins += [f"$.candidates[{candidate_index}].source_range"]
        require(row["reason_paths"] == origins, "selection evidence paths changed")
        position += length
    require(position == exact(expected["totalDuration"]) <= exact(document["max_duration"])
            and len(selected) <= document["max_shots"], "actual plan exceeds a hard bound")
    require((position * rate).denominator == 1 and position * rate <= MAX_U64,
            "total ticks overflow shared clock")
    for edge in audit["precedence_edges"]:
        require(selected.index(edge["before"]) < selected.index(edge["after"]),
                "selected precedence edge relaxed")
    require(set(required).issubset(selected), "required asset silently discarded")
    require(not set(selected).intersection(binding["asset_id"] for binding in audit["excluded_assets"]),
            "excluded candidate selected")
    expected_skipped = [{"asset_id": row["assetId"], "path": f"$.candidates[{facts[row['assetId']][0]}]",
                         "reason": row["reason"]} for row in expected["skippedCandidates"]]
    require(report["skipped_candidates"] == expected_skipped, "skipped reasons/order changed")
    return {"selectedAssetIds": selected, "totalDuration": str(position),
            "contentClockRate": str(rate), "targetStatus": report["target_status"],
            "nestedM0Accepted": True, "skippedCandidates": report["skipped_candidates"]}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("rust_cli", type=Path)
    parser.add_argument("--source-output", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    cli = args.rust_cli.resolve(strict=True)
    source_output = args.source_output.resolve(strict=True)
    output = args.output.resolve()
    require(not output.is_relative_to(REPO), "run evidence must be outside the checkout")
    source, source_bytes = source_evidence(source_output)
    records = []
    for case in source["cases"]:
        path = source_output / case["inputPath"]
        original = path.read_bytes()
        require(sha256(original) == case["inputSha256"], "native input hash mismatch")
        document = parse(original)
        process = invoke([str(cli), "plan-story", str(path)])
        response = parse(process.stdout)
        expected = case["expectedNative"]
        if expected["outcome"] == "ok":
            require(process.returncode == 0 and set(response) == {"ok", "planning"}
                    and response["ok"] is True, f"{case['id']}: native success expected")
            checked = check_success(cli, document, expected, response["planning"])
        else:
            require(process.returncode == 1 and set(response) == {"ok", "error"}
                    and response["ok"] is False, f"{case['id']}: typed rejection expected")
            error = response["error"]
            require(set(error) == {"code", "path", "conflicts"}
                    and error["code"] == expected["code"] and error["conflicts"] == []
                    and isinstance(error["path"], str) and error["path"].startswith("$"),
                    "typed native error differs")
            if case["id"] == "required-hard-maximum":
                require(error["path"] == "$.max_duration", "hard-bound error path changed")
            checked = {"error": error}
        stdin = invoke([str(cli), "plan-story", "-"], original)
        require(stdin.returncode == process.returncode and stdin.stdout == process.stdout,
                "file/stdin output differs")
        repeat = invoke([str(cli), "plan-story", str(path)])
        require(repeat.returncode == process.returncode and repeat.stdout == process.stdout,
                "fixed inputs produced a nondeterministic proposal")
        require(path.read_bytes() == original, "planner changed input bytes")
        records.append({"id": case["id"], "classification": case["classification"],
                        "sourceOutcome": case["observedSource"]["outcome"],
                        "inputSha256": sha256(original), "stdoutSha256": sha256(process.stdout),
                        "exitCode": process.returncode, "fileStdinEqual": True,
                        "repeatIdentical": True, "inputUnchanged": True, **checked, "passed": True})
    version = invoke([str(cli), "--version"])
    require(version.returncode == 0, "CLI version command failed")
    evidence = {
        "schemaVersion": 1, "scope": "twelve-case-fixed-window-source-and-native-conformance",
        "sourceHead": SOURCE_SHA, "targetHead": git("rev-parse", "HEAD").decode().strip(),
        "targetWorkingTreeDirty": bool(git("status", "--porcelain", "--untracked-files=normal")),
        "targetTrackedDiffSha256": sha256(git("diff", "HEAD", "--binary")),
        "rustCliSha256": sha256(cli.read_bytes()), "rustCliVersion": version.stdout.decode().strip(),
        "pythonVersion": platform.python_version(),
        "comparatorSha256": sha256(Path(__file__).read_bytes()),
        "sourceEvidenceSha256": sha256(source_bytes), "sourceRunnerSha256": source["runnerSha256"],
        "sourceToolHashes": {name: source[name] for name in
                             ("nodeBinarySha256", "tsxLoaderSha256", "tsxPackageSha256")},
        "dependencyLockSha256": source["dependencyLockSha256"],
        "command": [sys.executable, str(Path(__file__).resolve()), str(cli),
                    "--source-output", str(source_output), "--output", str(output)],
        "cases": records, "allPassed": all(record["passed"] for record in records),
        "intentionalDeltas": source["intentionalDeltas"],
        "notClaimed": ["full upstream suite", "multi-beat/coverage/pins/graph/preset solver parity",
                       "global optimality or feasibility", "media/source/host verification",
                       "same-head hosted CI", "release or editorial acceptance"],
    }
    output.write_text(json.dumps(evidence, indent=2) + "\n")
    print(json.dumps({"ok": True, "cases": len(records), "output": str(output)}))


if __name__ == "__main__":
    main()
