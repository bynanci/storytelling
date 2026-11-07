#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Verify finite Rust OTIO using official otio_json and frozen source evidence."""

from __future__ import annotations

import argparse
from decimal import Decimal
from fractions import Fraction
import hashlib
import importlib.metadata
import json
import math
from pathlib import Path
import platform
import subprocess
import sys

import opentimelineio as otio


HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
M1 = HERE.parent / "openfilm-finite-v1"
SOURCE = HERE / "source"
SOURCE_SHA = "7add4762fe8174d76f43b863375ea39b73121e76"
OTIO_VERSION = "0.18.1"
WHEEL_SHA = "8f0924f818b496cf771612d489082bdb83f42b6bd97ed06589dd2e2c086742c9"
WHEEL = "opentimelineio-0.18.1-cp312-cp312-manylinux2014_x86_64.manylinux_2_17_x86_64.whl"
SAFE_INTEGER = 9_007_199_254_740_991
CASE_IDS = ["one-still", "video-trim", "repeated-source", "decimal-rate",
            "explicit-ntsc", "unsupported-scale"]
SOURCE_ERROR_LIMIT = Fraction(1, 10**9)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def unique_pairs(pairs: list[tuple[str, object]]) -> dict:
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON property: {key}")
        result[key] = value
    return result


def parse(data: bytes | str) -> dict:
    return json.loads(data, parse_float=Decimal, parse_int=int,
                      object_pairs_hook=unique_pairs)


def exact(value: object) -> Fraction:
    require(type(value) in (int, Decimal), "expected an exact JSON number")
    return Fraction(value)


def wire(value: dict) -> Fraction:
    require(set(value) == {"numerator", "denominator"}, "unexpected rational fields")
    n, d = value["numerator"], value["denominator"]
    require(type(n) is str and type(d) is str and n.isascii() and d.isascii()
            and n.isdigit() and d.isdigit(), "expected unsigned decimal rational strings")
    require(int(d) > 0 and math.gcd(int(n), int(d)) == 1, "unreduced rational")
    return Fraction(int(n), int(d))


def compact_json_text(data: bytes) -> str:
    # Preserve every number/string lexeme; remove whitespace only outside strings.
    result = bytearray()
    inside = escaped = False
    for byte in data:
        if inside:
            result.append(byte)
            if escaped:
                escaped = False
            elif byte == 92:
                escaped = True
            elif byte == 34:
                inside = False
        elif byte == 34:
            inside = True
            result.append(byte)
        elif byte not in b" \t\r\n":
            result.append(byte)
    return result.decode("utf8")


def invoke(command: list[str]) -> subprocess.CompletedProcess:
    result = subprocess.run(command, capture_output=True, check=False, timeout=60)
    require(not result.stderr, f"unexpected CLI stderr: {result.stderr!r}")
    return result


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(REPO), *args],
                                   stderr=subprocess.PIPE, timeout=30)


def official_read(data: bytes | str):
    text = data.decode("utf8") if isinstance(data, bytes) else data
    value = otio.adapters.read_from_string(text, adapter_name="otio_json")
    require(isinstance(value, otio.schema.Timeline), "official parser did not read a timeline")
    return value


def native_fraction(time) -> Fraction:
    require(math.isfinite(time.value) and math.isfinite(time.rate) and time.rate > 0,
            "invalid official RationalTime")
    # Fraction(float) observes the actual binary native value; no decimal rounding.
    return Fraction(time.value) / Fraction(time.rate)


def range_fractions(time_range) -> tuple[Fraction, Fraction, Fraction]:
    return (native_fraction(time_range.start_time), native_fraction(time_range.duration),
            native_fraction(time_range.end_time_exclusive()))


def native_clock(document: object, rate: int, *, original: bool) -> int:
    count = 0
    if isinstance(document, dict):
        if document.get("OTIO_SCHEMA") == "RationalTime.1":
            for field in ("value", "rate"):
                value = document[field]
                require(type(value) is int if original else type(value) in (int, Decimal),
                        "native time field is not numeric integer")
                require(exact(value).denominator == 1 and 0 <= value <= SAFE_INTEGER,
                        "native time exceeds the exact safe-integer profile")
            require(document["rate"] == rate and rate > 0, "native rate is not the common clock")
            count += 1
        for child in document.values():
            count += native_clock(child, rate, original=original)
    elif isinstance(document, list):
        for child in document:
            count += native_clock(child, rate, original=original)
    return count


def declared(document: dict) -> dict:
    track = document["composition"]["tracks"][0]
    assets = {asset["id"]: asset for asset in document["assets"]}
    require(len(assets) == len(document["assets"]), "duplicate declared asset")
    cursor = Fraction(0)
    clips = []
    ends: dict[str, list[Fraction]] = {}
    for clip in track["clips"]:
        duration = exact(clip["timelineDuration"])
        start = exact(clip.get("sourceIn", 0))
        end = exact(clip["sourceOut"]) if "sourceOut" in clip else start + duration
        require(exact(clip["timelineStart"]) == cursor, "non-contiguous oracle input")
        require(end - start == duration, "oracle input has non-unit source timing")
        clips.append({"id": clip["id"], "asset": assets[clip["assetId"]],
                      "start": start, "end": end, "duration": duration,
                      "parent_start": cursor})
        ends.setdefault(clip["assetId"], []).append(end)
        cursor += duration
    require(cursor == exact(document["composition"]["duration"]), "wrong declared total")
    available = {key: max(ends[key]) if asset["mediaType"] == "image"
                 else exact(asset["duration"]) for key, asset in assets.items()}
    return {"track": track, "clips": clips, "duration": cursor, "available": available}


def check_source(source_bytes: bytes, document: dict) -> dict:
    """Record floating-source deltas; the Rust comparisons below remain exact."""
    raw = parse(source_bytes)
    expected = declared(document)
    owner = raw["metadata"]["openfilm"]
    require(owner["composition"] == document["composition"]
            and owner["settings"] == document["settings"], "source owner facts changed")
    require(owner["compatibility"]["realNleVerified"] is False, "source claims NLE verification")
    require(owner["compatibility"]["advancedEdits"] == "metadata-only", "source policy changed")
    parsed = official_read(source_bytes)
    require(parsed.name == document["composition"]["id"] and len(parsed.tracks) == 1,
            "source timeline identity/track count changed")
    track = parsed.tracks[0]
    require(track.name == expected["track"]["id"] and len(track) == len(expected["clips"]),
            "source track/occurrence count changed")
    errors = []

    def compare(actual: Fraction, wanted: Fraction, path: str) -> None:
        error = actual - wanted
        require(abs(error) <= SOURCE_ERROR_LIMIT, f"source error exceeds 1e-9: {path}")
        errors.append({"path": path, "actualSeconds": str(actual), "expectedSeconds": str(wanted),
                       "errorSeconds": str(error), "exact": error == 0,
                       "classification": "exact" if error == 0 else "source-floating-representation-error"})

    for index, (clip, want) in enumerate(zip(track, expected["clips"])):
        raw_clip = raw["tracks"]["children"][0]["children"][index]
        require(clip.metadata["openfilm"]["clipId"] == want["id"]
                and clip.metadata["openfilm"]["assetId"] == want["asset"]["id"],
                "source occurrence order or identity changed")
        require(raw_clip["metadata"]["openfilm"]["clip"] == expected["track"]["clips"][index],
                "source clip owner facts changed")
        require(clip.media_reference.target_url == want["asset"]["uri"], "source URI changed")
        require(clip.media_reference.metadata["openfilm"]["contentHash"] == want["asset"]["contentHash"],
                "source declared identity changed")
        require(not clip.effects, "source unexpectedly emitted native effects")
        for field, actual, wanted in zip(("start", "duration", "end"),
                                        range_fractions(clip.source_range),
                                        (want["start"], want["duration"], want["end"])):
            compare(actual, wanted, f"clips[{index}].source_range.{field}")
        # Upstream models a still occurrence's own hold as its available window.
        source_available = want["end"] if want["asset"]["mediaType"] == "image" else expected["available"][want["asset"]["id"]]
        compare(native_fraction(clip.media_reference.available_range.duration), source_available,
                f"clips[{index}].available_duration")
        parent = track.range_of_child_at_index(index)
        for field, actual, wanted in zip(("start", "duration", "end"), range_fractions(parent),
                                        (want["parent_start"], want["duration"],
                                         want["parent_start"] + want["duration"])):
            compare(actual, wanted, f"clips[{index}].parent.{field}")
    compare(native_fraction(parsed.duration()), expected["duration"], "timeline.duration")
    return {"officialSourceParse": True, "sourceSecondsTolerance": "1/1000000000",
            "sourceTimeComparisons": errors,
            "nonzeroSourceFloatingErrorCount": sum(not item["exact"] for item in errors),
            "sourceGlobalStartRate": str(raw["global_start_time"]["rate"]),
            "sourceMetadataOnlyEdits": owner["compatibility"]["metadataOnlyEdits"]}


def check_rust(document: dict, original: bytes, raw: dict, parsed, imported: dict,
               choices: dict, *, raw_native: bool) -> dict:
    expected = declared(document)
    owner = raw["metadata"]["storytelling"]
    report = owner["report"]
    require(report["schema_version"] == 1
            and report["profile"] == "storytelling.otio-visual-ticks.v1"
            and report["scope"] == "declared-otio-visual-interchange", "wrong compile scope")
    for flag in ("executable", "source_verified", "host_verified"):
        require(report[flag] is False, f"compile report incorrectly claims {flag}")
    require(report["host_name"] is None and report["host_version"] is None, "host identity invented")
    require(report["choices"] == choices, "loss choices changed")
    require(report["required_capabilities"] == ["single-video-track", "external-reference",
            "still-hold", "arbitrary-content-tick-rate"], "capability requirements changed")
    require(report["import_limitations"] == imported["losses"], "original importer limitations changed")
    require(report["project_clocks"] == imported["clocks"], "original importer clock facts changed")
    clocks = report["project_clocks"]
    require(wire(clocks["exact_frame_rate"]) == exact(document["settings"]["frameRate"]),
            "project FPS rational changed")
    require(Fraction(Decimal(clocks["declared_frame_rate"])) == exact(document["settings"]["frameRate"]),
            "project FPS lexeme value changed")
    require(clocks["explicit_timebase"] == document["settings"].get("timebase"), "explicit timebase changed")
    require(type(owner["source_snapshot_json"]) is str, "snapshot must be metadata string")
    require(owner["source_snapshot_json"] == compact_json_text(original), "snapshot lexemes changed")
    require(parse(owner["source_snapshot_json"]) == document, "snapshot semantics changed")
    rate_text = report["content_clock_rate"]
    require(type(rate_text) is str and rate_text.isascii() and rate_text.isdigit()
            and str(int(rate_text)) == rate_text, "noncanonical content clock")
    rate = int(rate_text)
    denominators = [value.denominator for value in expected["available"].values()]
    for clip in expected["clips"]:
        denominators.extend([clip["start"].denominator, clip["end"].denominator])
    require(rate == math.lcm(*denominators), "content clock is not canonical denominator LCM")
    projected_bounds = [expected["duration"], *expected["available"].values()]
    for clip in expected["clips"]:
        projected_bounds += [clip["start"], clip["end"], clip["duration"],
                             clip["parent_start"], clip["parent_start"] + clip["duration"]]
    require(all((value * rate).denominator == 1 and 0 <= value * rate <= SAFE_INTEGER
                for value in projected_bounds), "derived native endpoint exceeds the common-clock bound")
    native_count = native_clock(raw, rate, original=raw_native)
    require(native_count > 0, "missing native time fields")
    require(raw["global_start_time"] is None and parsed.global_start_time is None,
            "content clock was presented as a project global start")
    require(parsed.name == document["composition"]["id"] and parsed.tracks.name == "storytelling",
            "timeline/stack identity changed")
    require(len(parsed.tracks) == 1, "unexpected track count")
    track = parsed.tracks[0]
    require(track.name == expected["track"]["id"] and track.kind == "Video", "track identity/kind changed")
    require(track.source_range is None and parsed.tracks.source_range is None, "composition trim invented")
    require(not track.effects and not track.markers and not parsed.tracks.effects
            and not parsed.tracks.markers, "composition effects/markers invented")
    require(len(track) == len(expected["clips"]), "occurrences collapsed or added")
    for index, (clip, want) in enumerate(zip(track, expected["clips"])):
        own = clip.metadata["storytelling"]
        require(own["occurrence_id"] == want["id"] and own["asset_id"] == want["asset"]["id"],
                "occurrence identity/order changed")
        require(dict(own["source_range"]) == imported["plan"]["shots"][index]["source_range"],
                "M0 source range owner facts changed")
        require(clip.name == want["asset"]["name"] and clip.media_reference.name == want["asset"]["name"],
                "asset name changed")
        reference = clip.media_reference
        require(isinstance(reference, otio.schema.ExternalReference), "not an external reference")
        require(reference.target_url == want["asset"]["uri"], "opaque URI changed")
        reference_owner = reference.metadata["storytelling"]
        require(reference_owner["asset_id"] == want["asset"]["id"]
                and reference_owner["declared_content_hash"] == want["asset"]["contentHash"]
                and reference_owner["media_type"] == want["asset"]["mediaType"], "asset facts changed")
        require(reference_owner["available_range_kind"] == ("synthetic-still-hold"
                if want["asset"]["mediaType"] == "image" else "declared-video-duration"),
                "available range kind changed")
        require(not clip.effects and not clip.markers, "native effects/markers invented")
        require(range_fractions(clip.source_range) == (want["start"], want["duration"], want["end"]),
                "native source range not exact")
        available = expected["available"][want["asset"]["id"]]
        require(range_fractions(reference.available_range) == (Fraction(0), available, available),
                "native available range not exact")
        require(range_fractions(track.range_of_child_at_index(index)) ==
                (want["parent_start"], want["duration"], want["parent_start"] + want["duration"]),
                "native parent placement not exact")
    require(native_fraction(parsed.duration()) == expected["duration"]
            and wire(report["timeline_duration"]) == expected["duration"], "native/report total not exact")
    feature_rows = report["feature_matrix"]
    require(all(row["accepted"] is True and row["disposition"] in ("native", "metadata-only")
                for row in feature_rows), "unaccepted/baked/unsupported feature in document")
    expected_features = ["timeline.cuts", "media.references"]
    if any(asset["mediaType"] == "image" for asset in document["assets"]):
        expected_features.append("image.holds")
    expected_features += ["clock.content_ticks", "project_rate.metadata_only", "source_facts.metadata_only"]
    if any(asset["mediaType"] == "video" for asset in document["assets"]):
        expected_features.append("audio.picture_only")
    locks = expected["track"].get("locked", False) or any(clip.get("locked", False)
            for clip in expected["track"]["clips"]) or any(asset["state"].get("locked", False)
            for asset in document["assets"])
    if locks:
        expected_features.append("editing.locks_metadata_only")
    require([row["feature"] for row in feature_rows] == expected_features, "feature order/coverage changed")
    for row in feature_rows:
        native = row["feature"] in ("timeline.cuts", "media.references", "image.holds", "clock.content_ticks")
        require(row["disposition"] == ("native" if native else "metadata-only"), "feature disposition changed")
    return {"contentClockRate": rate_text, "timelineSeconds": str(expected["duration"]),
            "exactProjectFrameRate": str(wire(clocks["exact_frame_rate"])),
            "nativeTimeFieldCount": native_count, "featureMatrix": feature_rows,
            "importLimitations": report["import_limitations"],
            "snapshotSha256": sha256(owner["source_snapshot_json"].encode("utf8"))}


def check_hash(path: Path, expected: str, label: str) -> bytes:
    data = path.read_bytes()
    require(sha256(data) == expected, f"{label} hash mismatch")
    return data


def source_evidence() -> tuple[dict, dict, list[dict]]:
    evidence_bytes = (SOURCE / "source-evidence.json").read_bytes()
    evidence = parse(evidence_bytes)
    require(evidence["schemaVersion"] == 1 and evidence["scope"] == "pinned-source-otio-export-only"
            and evidence["sourceHead"] == SOURCE_SHA and evidence["allPassed"] is True,
            "wrong source OTIO evidence identity")
    require(evidence["sourceTrackedFilesUnchanged"] is True
            and evidence["producerOptions"] == {"preserveConfiguredFrameRate": True}, "wrong source producer options")
    check_hash(SOURCE / "run-source.mts", evidence["runnerSha256"], "source OTIO runner")
    check_hash(M1 / "package-lock.json", evidence["dependencyLockSha256"], "source dependency lock")
    m1_bytes = (M1 / "source-evidence.json").read_bytes()
    m1 = parse(m1_bytes)
    require(m1["sourceHead"] == SOURCE_SHA and m1["sourceTrackedFilesUnchanged"] is True,
            "wrong M1-01 source identity")
    check_hash(M1 / "run-source.mts", m1["runnerSha256"], "M1-01 source runner")
    check_hash(M1 / "package-lock.json", m1["dependencyLockSha256"], "M1-01 dependency lock")
    require([case["id"] for case in evidence["cases"]] == CASE_IDS
            and [case["id"] for case in m1["fixtures"]] == CASE_IDS, "frozen case order changed")
    records = []
    for case, frozen in zip(evidence["cases"], m1["fixtures"]):
        case_id = case["id"]
        require(case["inputSha256"] == frozen["inputSha256"], "producer input chain mismatch")
        input_bytes = check_hash(SOURCE / "producer-inputs" / f"{case_id}.json", case["inputSha256"], "OTIO producer input")
        check_hash(M1 / "producer-inputs" / f"{case_id}.json", frozen["inputSha256"], "M1 producer input")
        rust_input = check_hash(M1 / "fixtures" / f"{case_id}.json", frozen["exportedSha256"], "frozen M1 source JSON")
        output = check_hash(SOURCE / case["outputFile"], case["exportedSha256"], "source OTIO output")
        require(case["outputFile"] == f"fixtures/{case_id}.otio"
                and len(output) == case["exportedBytes"], "unexpected source output path/size")
        require(case["inputBytesUnchanged"] is True and case["suppliedValuesUnchanged"] is True,
                "source input mutation evidence")
        producer = parse(input_bytes)
        document = parse(rust_input)
        require(producer["composition"] == document["composition"]
                and producer["assets"] == document["assets"]
                and producer["settings"] == document["settings"], "source/Rust declarations differ")
        records.append({"id": case_id, "rust_input": M1 / "fixtures" / f"{case_id}.json",
                        "source_bytes": output, "source_sha": sha256(output),
                        "source_result": check_source(output, document)})
    return ({"sourceEvidenceSha256": sha256(evidence_bytes), "m1SourceEvidenceSha256": sha256(m1_bytes),
             "sourceFiles": evidence["sourceFiles"], "sourceRunnerSha256": evidence["runnerSha256"],
             "sourceNodeVersion": evidence["nodeVersion"], "sourceTsxVersion": evidence["tsxVersion"],
             "sourceDependencyLockSha256": evidence["dependencyLockSha256"]}, evidence, records)


def run_case(cli: Path, case: dict) -> dict:
    path = case["rust_input"]
    original = path.read_bytes()
    document = parse(original)
    has_video = any(asset["mediaType"] == "video" for asset in document["assets"])
    # These explicit flags apply only to the known synthetic declarations.
    flags = (["--picture-only"] if has_video else []) + (["--metadata-only-locks"] if case["id"] == "locked-still" else [])
    choices = {"picture_only": has_video, "metadata_only_locks": case["id"] == "locked-still"}
    command = [str(cli), "export-otio", str(path), *flags]
    result = invoke(command)
    response = parse(result.stdout)
    require(path.read_bytes() == original, "Rust changed the declared input file")
    record = {"id": case["id"], "command": command, "exitCode": result.returncode,
              "inputSha256": sha256(original), "stdoutSha256": sha256(result.stdout),
              "inputBytesUnchanged": True}
    if "source_result" in case:
        record.update(case["source_result"])
        record["sourceOutputSha256"] = case["source_sha"]
    if case["id"] == "unsupported-scale":
        require(result.returncode == 1 and response["ok"] is False, "scale must be rejected")
        require(set(response) == {"ok", "error"}, "rejected scale returned a partial document")
        error = response["error"]
        require(error["code"] == "import.unsupported_feature", "scale rejection code changed")
        require(any(row["feature"] == "import.unsupported_feature"
                    and row["disposition"] == "unsupported" and row["accepted"] is False
                    for row in error["losses"]), "scale lacks structured unsupported loss")
        require(case["source_result"]["sourceMetadataOnlyEdits"] ==
                [{"clipId": "trim-occurrence", "features": ["scale"]}], "source scale classification changed")
        return {**record, "classification": "unsupported-rust-source-metadata-only", "error": error, "passed": True}
    require(result.returncode == 0, "supported finite OTIO fixture rejected")
    require(result.stdout.endswith(b"\n") and b"\n" not in result.stdout[:-1], "CLI OTIO is not one stdout line")
    imported_process = invoke([str(cli), "import-openfilm", str(path)])
    imported_response = parse(imported_process.stdout)
    require(imported_process.returncode == 0 and imported_response["ok"] is True, "original importer failed")
    imported = imported_response["import"]
    first = official_read(result.stdout)
    checked = check_rust(document, original, response, first, imported, choices, raw_native=True)
    written = otio.adapters.write_to_string(first, adapter_name="otio_json")
    second = official_read(written)
    roundtrip_raw = parse(written)
    roundtrip_checked = check_rust(document, original, roundtrip_raw, second, imported, choices, raw_native=False)
    require(checked == roundtrip_checked, "official roundtrip changed semantic summary")
    require(response["metadata"]["storytelling"] == roundtrip_raw["metadata"]["storytelling"],
            "official roundtrip changed root report/snapshot metadata")
    if case["id"] == "fractional-lexemes":
        snapshot = roundtrip_raw["metadata"]["storytelling"]["source_snapshot_json"]
        require(all(token in snapshot for token in ("9007199254740993", "0.1000", "1e-1")), "raw number lexemes lost")
        require(checked["contentClockRate"] == "10" and checked["timelineSeconds"] == "3/10",
                "fractional 1/10 + 1/5 clock changed")
    return {**record, **checked, "classification": "preserved-seconds-identity-uri-with-intentional-content-clock-delta"
            if "source_result" in case else "official-roundtrip-rust-only",
            "officialReadWriteRead": True, "exactNativeSemantics": True,
            "sourceSnapshotStringExactRoundtrip": True, "officialWrittenSha256": sha256(written.encode("utf8")),
            "officialWrittenClipSchemas": [clip["OTIO_SCHEMA"] for clip in roundtrip_raw["tracks"]["children"][0]["children"]],
            "executable": False, "sourceVerified": False, "hostVerified": False, "passed": True}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("rust_cli", type=Path, help="actual built storytelling executable")
    parser.add_argument("--output", type=Path, required=True, help="evidence JSON outside the checkout")
    args = parser.parse_args()
    cli = args.rust_cli.resolve(strict=True)
    output = args.output.resolve()
    require(not output.is_relative_to(REPO), "evidence output must be outside the checkout")
    require(output != cli and output != Path(__file__).resolve(), "output would overwrite an input")
    require(sys.implementation.name == "cpython" and sys.version_info[:2] == (3, 12)
            and platform.system() == "Linux" and platform.machine() in ("x86_64", "AMD64"),
            "use the pinned CPython 3.12 Linux x86_64 wheel environment")
    require(otio.__version__ == OTIO_VERSION and importlib.metadata.version("opentimelineio") == OTIO_VERSION,
            "official OTIO version mismatch")
    requirements = (HERE / "requirements-linux-py312.txt").read_bytes()
    require(f"opentimelineio=={OTIO_VERSION}".encode() in requirements
            and WHEEL_SHA.encode() in requirements, "wheel requirements pin mismatch")
    status_before = git("status", "--porcelain", "--untracked-files=normal")
    tracked_before = git("diff", "HEAD", "--binary")
    binary_sha = sha256(cli.read_bytes())
    hashes, source, cases = source_evidence()
    cases += [{"id": "locked-still", "rust_input": REPO / "tests/fixtures/openfilm/finite-v1/still.json"},
              {"id": "fractional-lexemes", "rust_input": REPO / "tests/fixtures/openfilm/finite-v1/fractional-rate.json"}]
    records = [run_case(cli, case) for case in cases]
    require(git("status", "--porcelain", "--untracked-files=normal") == status_before
            and git("diff", "HEAD", "--binary") == tracked_before, "oracle changed checkout state")
    require(sha256(cli.read_bytes()) == binary_sha, "Rust executable changed during comparison")
    distribution = importlib.metadata.distribution("opentimelineio")
    native_files = [file for file in distribution.files or [] if str(file).endswith(".so")]
    evidence = {"schemaVersion": 1, "scope": "finite-otio-source-and-official-synthetic-oracle",
                "sourceHead": SOURCE_SHA, "targetHead": git("rev-parse", "HEAD").decode().strip(),
                "targetWorkingTreeDirty": bool(status_before), "targetTrackedDiffSha256": sha256(tracked_before),
                "targetStatusSha256": sha256(status_before), "rustCliSha256": binary_sha,
                "rustCliVersion": invoke([str(cli), "--version"]).stdout.decode().strip(),
                "comparatorSha256": sha256(Path(__file__).read_bytes()),
                "requirementsSha256": sha256(requirements), "pythonVersion": platform.python_version(),
                "pythonImplementation": platform.python_implementation(), "platform": platform.platform(),
                "officialOtioVersion": otio.__version__, "officialAdapter": "otio_json",
                "officialWheelPin": {"filename": WHEEL, "sha256": WHEEL_SHA,
                                     "verification": "pip --require-hashes at installation; not reconstructed from installed files"},
                "installedNativeFiles": [{"path": str(file), "sha256": sha256(distribution.locate_file(file).read_bytes())}
                                         for file in native_files], **hashes,
                "command": [sys.executable, str(Path(__file__).resolve()), str(cli), "--output", str(output)],
                "cases": records, "allPassed": all(record["passed"] for record in records),
                "intentionalDelta": source["classification"]["intentionalDelta"],
                "sourceFloatingErrors": "Recorded per comparison; <=1e-9 only for source. Every Rust time comparison is exact.",
                "notClaimed": ["actual source-media verification", "audio preservation", "host compatibility",
                               "FilmCraft or another editor acceptance", "execution approval", "rendered output",
                               "full upstream test suite", "same-head CI", "release readiness"]}
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf8")
    print(json.dumps({"ok": True, "cases": len(records), "output": str(output)}))


if __name__ == "__main__":
    main()
