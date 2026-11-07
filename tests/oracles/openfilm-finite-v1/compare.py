#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Compare actual source exports with the actual Rust CLI; stdlib/test-only."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from decimal import Decimal
from fractions import Fraction
from pathlib import Path
import platform
import subprocess
import sys


ROOT = Path(__file__).resolve().parent
SOURCE_SHA = "7add4762fe8174d76f43b863375ea39b73121e76"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def pairs_unique(pairs: list[tuple[str, object]]) -> dict:
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON property: {key}")
        result[key] = value
    return result


def parse(data: bytes) -> dict:
    return json.loads(data, parse_float=Decimal, parse_int=int,
                      object_pairs_hook=pairs_unique)


def number(value: int | Decimal) -> Fraction:
    require(type(value) in (int, Decimal), "expected an exact JSON number")
    return Fraction(value)


def wire(value: dict) -> Fraction:
    require(set(value) == {"numerator", "denominator"}, "unexpected M0 time fields")
    n, d = value["numerator"], value["denominator"]
    require(isinstance(n, str) and isinstance(d, str), "M0 integers must be strings")
    require(n.isascii() and d.isascii() and n.isdigit() and d.isdigit(),
            "M0 rational must contain unsigned decimal strings")
    require(int(d) > 0 and math.gcd(int(n), int(d)) == 1, "M0 rational must be reduced")
    return Fraction(int(n), int(d))


def invoke(command: list[str], data: bytes | None = None) -> subprocess.CompletedProcess:
    result = subprocess.run(command, input=data, capture_output=True, check=False)
    require(not result.stderr, f"unexpected CLI stderr for {command}: {result.stderr!r}")
    return result


def git(repo: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(repo), *args], stderr=subprocess.PIPE)


def check_supported(document: dict, imported: dict) -> dict:
    require(imported["profile"] == "openfilm-finite-v1", "wrong profile")
    require(imported["scope"] == "declared-visual-mapping-only", "wrong evidence scope")
    require(imported["executable"] is False, "an import must not authorize execution")
    # Dict equality ignores JSON object property order; list equality preserves every
    # occurrence and source array order. Decimal prevents binary-float roundoff.
    require(imported["source_snapshot"] == document, "source snapshot semantics changed")
    original_clips = document["composition"]["tracks"][0]["clips"]
    plan = imported["plan"]
    require(plan["schema_version"] == 1, "M0 wire schema changed")
    require(plan["plan_id"] == document["composition"]["id"], "composition identity changed")
    require(len(plan["shots"]) == len(original_clips), "occurrences were collapsed")
    require(len(plan["assets"]) == len(document["assets"]), "source count changed")
    cursor = Fraction(0)
    source_ends: dict[str, list[Fraction]] = {}
    for original, shot in zip(original_clips, plan["shots"]):
        require(shot["id"] == original["id"], "clip order/identity changed")
        require(shot["asset_id"] == original["assetId"], "clip-to-source identity changed")
        length = number(original["timelineDuration"])
        require(number(original["timelineStart"]) == cursor, "oracle input is not contiguous")
        cursor += length
        start = number(original.get("sourceIn", 0))
        end = number(original["sourceOut"]) if "sourceOut" in original else start + length
        require(wire(shot["source_range"]["start"]) == start, "source in changed")
        require(wire(shot["source_range"]["end"]) == end, "source out changed")
        source_ends.setdefault(original["assetId"], []).append(end)
    require(cursor == number(document["composition"]["duration"]), "composition end changed")
    required_losses = {("media.not_verified", "$.assets", "not-verified")}
    for index, (original, mapped) in enumerate(zip(document["assets"], plan["assets"])):
        require(mapped["id"] == original["id"], "asset order/identity changed")
        require(mapped["content_hash"] == original["contentHash"], "declared hash changed")
        image = original["mediaType"] == "image"
        duration = max(source_ends[original["id"]]) if image else number(original["duration"])
        require(wire(mapped["duration"]) == duration, "declared/hold duration changed")
        required_losses.add(("image.hold_window" if image else "audio.not_mapped",
                             f"$.assets[{index}]",
                             "intentional-delta" if image else "blocks-host-handoff"))
    actual_losses = {(loss["code"], loss["path"], loss["disposition"])
                     for loss in imported["losses"]}
    require(required_losses <= actual_losses, "media/audio/still limitations disappeared")
    settings = document["settings"]
    clocks = imported["clocks"]
    require(wire(clocks["exact_frame_rate"]) == number(settings["frameRate"]),
            "decimal frame rate silently changed")
    require(Fraction(Decimal(clocks["declared_frame_rate"])) == number(settings["frameRate"]),
            "declared decimal frame rate changed")
    require(clocks["explicit_timebase"] == settings.get("timebase"), "explicit clock facts changed")
    return {"plan": plan, "lossCodes": [loss["code"] for loss in imported["losses"]],
            "exactFrameRate": str(wire(clocks["exact_frame_rate"]))}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("rust_cli", type=Path, help="built storytelling executable")
    parser.add_argument("--source-output", type=Path, default=ROOT,
                        help="directory written by run-source.mts")
    parser.add_argument("--output", type=Path, default=ROOT / "differential-evidence.json")
    args = parser.parse_args()
    cli = args.rust_cli.resolve(strict=True)
    source_output = args.source_output.resolve(strict=True)
    source_evidence_bytes = (source_output / "source-evidence.json").read_bytes()
    source_evidence = parse(source_evidence_bytes)
    require(source_evidence["sourceHead"] == SOURCE_SHA, "source evidence SHA mismatch")
    require(source_evidence["runnerSha256"] == sha256((ROOT / "run-source.mts").read_bytes()),
            "rerun source oracle after changing the runner")
    require(source_evidence["dependencyLockSha256"] == sha256((ROOT / "package-lock.json").read_bytes()),
            "rerun source oracle after changing dependency lock")
    records = []
    for fixture in source_evidence["fixtures"]:
        fixture_id = fixture["id"]
        path = source_output / "fixtures" / f"{fixture_id}.json"
        original = path.read_bytes()
        require(sha256(original) == fixture["exportedSha256"], "source output hash mismatch")
        producer_input = source_output / "producer-inputs" / f"{fixture_id}.json"
        require(sha256(producer_input.read_bytes()) == fixture["inputSha256"], "producer input hash mismatch")
        command = [str(cli), "import-openfilm", str(path)]
        process = invoke(command)
        response = parse(process.stdout)
        record = {"id": fixture_id, "command": command, "exitCode": process.returncode,
                  "sourceOutputSha256": sha256(original), "stdoutSha256": sha256(process.stdout),
                  "inputBytesUnchanged": path.read_bytes() == original}
        require(record["inputBytesUnchanged"], "Rust CLI changed the source fixture")
        if fixture_id == "unsupported-scale":
            require(process.returncode == 1 and response["ok"] is False, "scale was accepted")
            require("import" not in response and "plan" not in response, "rejected input returned a plan")
            error = response["error"]
            require(error["code"] == "import.unsupported_feature", "wrong unsupported error")
            require(any(loss["code"] == "import.unsupported_feature" and
                        loss["disposition"] == "unsupported" for loss in error["losses"]),
                    "unsupported scale lost its structured rejection")
            record.update(classification="unsupported", errorCode=error["code"], passed=True)
        else:
            require(process.returncode == 0 and response["ok"] is True, "supported shape rejected")
            checked = check_supported(parse(original), response["import"])
            validation_command = [str(cli), "validate", "-"]
            validation = invoke(validation_command, json.dumps(checked.pop("plan")).encode())
            validated = parse(validation.stdout)
            require(validation.returncode == 0 and validated["ok"] is True,
                    "returned plan failed actual M0 CLI validation")
            require(validated["validation"]["scope"] == "declared-metadata-only", "verification scope expanded")
            record.update(checked)
            record.update(classification="intentional-delta" if fixture_id == "decimal-rate" else "preserved",
                          sourceSnapshotSemanticEquality=True, identityOrderAndRangesPreserved=True,
                          executable=False, m0ValidationCommand=validation_command, m0ValidationExit=0, passed=True)
            if fixture_id == "decimal-rate":
                require(record["exactFrameRate"] == "2997/100", "decimal 29.97 was snapped to NTSC")
                require(fixture["sourceDerivedTimebase"]["rateNumerator"] == 30000 and
                        fixture["sourceDerivedTimebase"]["rateDenominator"] == 1001,
                        "source clock oracle changed")
        records.append(record)
    require(len(records) == 6, "expected the frozen six-case source oracle")
    repo = ROOT.parents[2]
    status = git(repo, "status", "--porcelain", "--untracked-files=normal")
    evidence = {
        "schemaVersion": 1, "scope": "six-case-source-to-rust-differential",
        "sourceHead": SOURCE_SHA, "targetHead": git(repo, "rev-parse", "HEAD").decode().strip(),
        "targetWorkingTreeDirty": bool(status),
        "targetTrackedDiffSha256": sha256(git(repo, "diff", "HEAD", "--binary")),
        "rustCliSha256": sha256(cli.read_bytes()), "pythonVersion": platform.python_version(),
        "rustCliVersion": invoke([str(cli), "--version"]).stdout.decode().strip(),
        "comparatorSha256": sha256(Path(__file__).read_bytes()),
        "sourceEvidenceSha256": sha256(source_evidence_bytes),
        "command": [sys.executable, str(Path(__file__).resolve()), str(cli),
                    "--source-output", str(source_output), "--output", str(args.output.resolve())],
        "cases": records, "allPassed": all(record["passed"] for record in records),
        "intentionalDelta": "29.97 remains exact 2997/100; source legacy derivation is 30000/1001",
        "unsupported": "source preserves scale in JSON; finite Rust importer rejects it with a structured loss",
        "notClaimed": ["full upstream test suite", "actual source-byte verification", "audio handoff",
                       "OTIO compatibility", "host compatibility", "same-head CI", "release readiness"],
    }
    args.output.write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf8")
    print(json.dumps({"ok": True, "cases": len(records), "output": str(args.output)}))


if __name__ == "__main__":
    main()
