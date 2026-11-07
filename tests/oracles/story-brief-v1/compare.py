#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Replay ten frozen source policy outcomes against the actual Rust brief CLI."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import sys


ROOT = Path(__file__).resolve().parent
SOURCE_SHA = "7add4762fe8174d76f43b863375ea39b73121e76"
CASES = {
    "manual-outside-candidates", "manual-order", "order-implies-membership",
    "manual-excluded", "include-excluded", "missing-manual", "missing-excluded",
    "order-cycle", "manual-order-cycle", "excluded-preference",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def parse(data: bytes) -> dict:
    def unique(pairs: list[tuple[str, object]]) -> dict:
        result = {}
        for key, value in pairs:
            require(key not in result, "duplicate JSON property")
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=unique)


def git(repo: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(repo), *args], stderr=subprocess.PIPE)


def invoke(command: list[str], data: bytes | None = None) -> subprocess.CompletedProcess:
    result = subprocess.run(command, input=data, capture_output=True, check=False)
    require(not result.stderr, "unexpected CLI stderr")
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("rust_cli", type=Path)
    parser.add_argument("--source-output", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path, required=True,
                        help="write run evidence outside the checkout")
    args = parser.parse_args()
    cli = args.rust_cli.resolve(strict=True)
    source_output = args.source_output.resolve(strict=True)
    repo = ROOT.parents[2]
    output = args.output.resolve()
    require(not output.is_relative_to(repo), "run evidence must be outside the checkout")
    source_bytes = (source_output / "source-evidence.json").read_bytes()
    source = parse(source_bytes)
    require(source["sourceHead"] == SOURCE_SHA and source["allPassed"] is True,
            "source evidence not successful at the pinned revision")
    require(source["runnerSha256"] == sha256((ROOT / "run-source.mts").read_bytes()),
            "source runner changed; execute source oracle again")
    require(len(source["cases"]) == len(CASES) and
            {case["id"] for case in source["cases"]} == CASES,
            "expected exact frozen ten-case source oracle")
    records = []
    for case in source["cases"]:
        case_id = case["id"]
        path = source_output / "inputs" / f"{case_id}.json"
        original = path.read_bytes()
        require(sha256(original) == case["briefSha256"], "source brief hash mismatch")
        document = parse(original)
        command = [str(cli), "audit-brief", str(path)]
        process = invoke(command)
        response = parse(process.stdout)
        require(set(response) == {"ok", "audit"}, "audit did not return its typed report")
        audit = response["audit"]
        consistent = case["outcome"] == "consistent-selected-policy"
        require(process.returncode == (0 if consistent else 1) and
                response["ok"] is consistent and audit["consistent"] is consistent,
                f"{case_id}: Rust/source selected policy outcome differs")
        require(audit["scope"] == "declared-constraint-audit-only" and
                audit["executable"] is False and audit["selection_planned"] is False and
                audit["duration_analysis"] == "not-performed", "audit authority expanded")
        require(audit["brief_id"] == document["brief_id"] and
                audit["intent"] == document["intent"] and
                audit["manual_selected_asset_ids"] == document["manual_selected_asset_ids"] and
                audit["preferences"] == document["preferences"], "input choices changed")
        required = [binding["asset_id"] for binding in audit["required_assets"]]
        excluded = [binding["asset_id"] for binding in audit["excluded_assets"]]
        edges = [(edge["before"], edge["after"]) for edge in audit["precedence_edges"]]
        # Source planner output is not the downstream report. Compare only the
        # policies actually exercised upstream; never sort semantic arrays.
        if consistent:
            require(required == case["expectedRequired"], "hard membership/order changed")
            require(excluded == case["expectedExcluded"], "hard exclusion changed")
            require(not audit["conflicts"], "clean source case gained a hard conflict")
            expected_order = case["expectedOrder"]
            require(edges == list(zip(expected_order, expected_order[1:])),
                    "manual/declared relative order changed")
            expected_warnings = ([case["expectedWarning"]] if "expectedWarning" in case else [])
            require([warning["kind"] for warning in audit["warnings"]] == expected_warnings,
                    "downstream preference warning changed")
        else:
            require([conflict["kind"] for conflict in audit["conflicts"]] ==
                    [case["conflictKind"]], "typed hard conflict differs")
            for asset_id in document["manual_selected_asset_ids"]:
                require(asset_id in required, "conflicting manual choice was discarded")
            if case["conflictKind"] == "order-cycle":
                cycle = audit["conflicts"][0]["asset_ids"]
                require(len(cycle) >= 2 and len(set(cycle)) == len(cycle), "invalid cycle witness")
                require(all(pair in edges for pair in zip(cycle, cycle[1:] + cycle[:1])),
                        "reported witness is not an actual cycle")
        stdin = invoke([str(cli), "audit-brief", "-"], original)
        require(stdin.returncode == process.returncode and stdin.stdout == process.stdout,
                "file and stdin behavior differ")
        require(path.read_bytes() == original, "CLI changed the input bytes")
        records.append({"id": case_id, "exitCode": process.returncode,
                        "inputSha256": sha256(original), "stdoutSha256": sha256(process.stdout),
                        "sourceOutcome": case["outcome"], "requiredAssetIds": required,
                        "excludedAssetIds": excluded, "precedenceEdges": edges,
                        "conflictKinds": [item["kind"] for item in audit["conflicts"]],
                        "warningKinds": [item["kind"] for item in audit["warnings"]],
                        "fileStdinEqual": True, "inputUnchanged": True, "passed": True})
    evidence = {
        "schemaVersion": 1, "scope": "ten-case-selected-source-policy-to-rust-comparison",
        "sourceHead": SOURCE_SHA, "targetHead": git(repo, "rev-parse", "HEAD").decode().strip(),
        "targetWorkingTreeDirty": bool(git(repo, "status", "--porcelain", "--untracked-files=normal")),
        "targetTrackedDiffSha256": sha256(git(repo, "diff", "HEAD", "--binary")),
        "rustCliSha256": sha256(cli.read_bytes()), "pythonVersion": platform.python_version(),
        "rustCliVersion": invoke([str(cli), "--version"]).stdout.decode().strip(),
        "comparatorSha256": sha256(Path(__file__).read_bytes()),
        "sourceEvidenceSha256": sha256(source_bytes),
        "command": [sys.executable, str(Path(__file__).resolve()), str(cli),
                    "--source-output", str(source_output), "--output", str(output)],
        "cases": records, "allPassed": all(record["passed"] for record in records),
        "intentionalDelta": source["intentionalDelta"],
        "notClaimed": ["full upstream suite", "complete solver/planner parity", "duration feasibility",
                       "selection planning", "source/media/host verification", "same-head CI", "release"],
    }
    output.write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf8")
    print(json.dumps({"ok": True, "cases": len(records), "output": str(output)}))


if __name__ == "__main__":
    main()
