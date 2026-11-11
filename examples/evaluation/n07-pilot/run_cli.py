# SPDX-License-Identifier: MIT
"""Measure read-only CLI processes; never generate human trial observations."""
import argparse
import hashlib
import json
import math
import os
import platform
import re
import statistics
import subprocess
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SAFE_ID = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]{0,127}\Z")
COMMANDS = {"render-narrative", "track-narrative-revisions",
            "query-narrative-impact", "audit-copy-claims"}
MAX_MANIFEST_BYTES = 1_048_576
MAX_INPUT_BYTES = 16_777_216


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate manifest field")
        result[key] = value
    return result


def _sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _checkout_identity(root):
    try:
        commit = subprocess.run(["git", "rev-parse", "HEAD"], cwd=root,
                                capture_output=True, check=False, timeout=10)
        status = subprocess.run(["git", "status", "--porcelain"], cwd=root,
                                capture_output=True, check=False, timeout=10)
    except (OSError, subprocess.TimeoutExpired):
        return {"commit": None, "dirty": None, "available": False}
    value = commit.stdout.decode("ascii", errors="replace").strip()
    valid = commit.returncode == 0 and re.fullmatch(r"[0-9a-f]{40,64}", value)
    return {"commit": value if valid else None,
            "dirty": bool(status.stdout) if status.returncode == 0 else None,
            "available": bool(valid)}


def _input_path(value, root):
    if (not isinstance(value, str) or not 1 <= len(value) <= 1024
            or "\\" in value or ":" in value
            or any(ord(char) < 32 for char in value)
            or any(part in {"", ".", ".."} for part in value.split("/"))):
        raise ValueError("input_files must use bounded repository-relative paths")
    path = (root / value).resolve()
    if not path.is_relative_to(root) or not path.is_file():
        raise ValueError("input file is absent or outside the repository")
    if path.stat().st_size > MAX_INPUT_BYTES:
        raise ValueError("input file exceeds the measurement bound")
    return path


def _load_manifest(manifest_path, root):
    with manifest_path.open("rb") as source:
        raw = source.read(MAX_MANIFEST_BYTES + 1)
    if len(raw) > MAX_MANIFEST_BYTES:
        raise ValueError("manifest exceeds 1 MiB")
    try:
        data = json.loads(raw.decode("utf-8"), object_pairs_hook=_unique_object)
    except (json.JSONDecodeError, UnicodeError) as exc:
        raise ValueError("invalid UTF-8 JSON manifest") from exc
    except RecursionError as exc:
        raise ValueError("manifest JSON nesting exceeds parser bound") from exc
    if (not isinstance(data, dict)
            or set(data) != {"schema_version", "input_files", "cases"}
            or type(data["schema_version"]) is not int or data["schema_version"] != 1):
        raise ValueError("use closed manifest schema_version 1")
    inputs = data["input_files"]
    if (not isinstance(inputs, list) or not 1 <= len(inputs) <= 64
            or not all(isinstance(value, str) for value in inputs)
            or len(set(inputs)) != len(inputs)):
        raise ValueError("declare 1–64 unique input files")
    paths = {value: _input_path(value, root) for value in inputs}
    cases = data["cases"]
    if not isinstance(cases, list) or not 1 <= len(cases) <= 32:
        raise ValueError("declare 1–32 measurement cases")
    seen = set()
    for case in cases:
        if (not isinstance(case, dict)
                or set(case) != {"id", "args", "expected_exit"}
                or not isinstance(case["id"], str)
                or not SAFE_ID.fullmatch(case["id"]) or case["id"] in seen):
            raise ValueError("use unique safe case IDs and a closed case object")
        seen.add(case["id"])
        if type(case["expected_exit"]) is not int or case["expected_exit"] not in {0, 1}:
            raise ValueError("expected_exit must be integer 0 or 1")
        args = case["args"]
        if (not isinstance(args, list) or not 2 <= len(args) <= 4
                or not all(isinstance(arg, str) and 1 <= len(arg) <= 2048
                           and not any(ord(char) < 32 for char in arg) for arg in args)
                or args[0] not in COMMANDS):
            raise ValueError("use a bounded read-only command and argument list")
        command = args[0]
        if command in {"render-narrative", "track-narrative-revisions"}:
            if len(args) != 2:
                raise ValueError("single-input command has unexpected arguments")
            file_args = args[1:]
        elif command == "query-narrative-impact":
            if len(args) != 3 or not SAFE_ID.fullmatch(args[2]):
                raise ValueError("query needs one ledger and a safe change ID")
            file_args = args[1:2]
        else:
            if len(args) not in {3, 4} or (len(args) == 4 and args[3] != "--report-v2"):
                raise ValueError("audit needs copy, ledger and optional trailing report flag")
            file_args = args[1:3]
        if any(value not in paths for value in file_args):
            raise ValueError("every command input must be declared in input_files; stdin is excluded")
    return data, paths, hashlib.sha256(raw).hexdigest()


def _bytes(value):
    if value is None:
        return b""
    return value if isinstance(value, bytes) else value.encode("utf-8")


def _invoke(cli, args, root, timeout, index):
    started = time.perf_counter()
    timed_out = False
    launch_error = False
    try:
        answer = subprocess.run([str(cli), *args], cwd=root, capture_output=True,
                                timeout=timeout, check=False, shell=False, stdin=subprocess.DEVNULL)
        exit_code, stdout, stderr = answer.returncode, _bytes(answer.stdout), _bytes(answer.stderr)
    except subprocess.TimeoutExpired as exc:
        exit_code, stdout, stderr = None, _bytes(exc.stdout), _bytes(exc.stderr)
        timed_out = True
    except OSError:
        exit_code, stdout, stderr = None, b"", b""
        launch_error = True
    elapsed = round((time.perf_counter() - started) * 1000, 6)
    return ({"index": index, "exit_code": exit_code, "wallclock_ms": elapsed,
             "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
             "stderr_sha256": hashlib.sha256(stderr).hexdigest(),
             "timed_out": timed_out, "launch_error": launch_error}, stdout, stderr)


def _after_hashes(paths):
    hashes = {}
    for name, path in paths.items():
        try:
            hashes[name] = _sha256(path)
        except OSError:
            hashes[name] = None
    return hashes


def _write_new(path, value):
    with path.open("xb") as target:
        target.write(value)


def run_manifest(cli, manifest_path, out_dir, runs=5, timeout=30, *, root=None):
    """Run one excluded warmup and N measured attempts; output is engineering evidence only."""
    if type(runs) is not int or not 1 <= runs <= 20:
        raise ValueError("runs must be an integer from 1 to 20")
    if (isinstance(timeout, bool) or not isinstance(timeout, (int, float))
            or not math.isfinite(timeout) or not 1 <= timeout <= 60):
        raise ValueError("timeout must be between 1 and 60 seconds")
    root = Path(ROOT if root is None else root).resolve()
    if not root.is_dir():
        raise ValueError("repository root is absent")
    out_dir = Path(out_dir)
    if os.path.lexists(out_dir):
        raise ValueError("out-dir already exists; use a fresh path outside private study records")
    out_dir = out_dir.resolve()
    cli = Path(cli)
    if not cli.is_absolute() or not cli.is_file():
        raise ValueError("cli must name an existing absolute executable path")
    cli = cli.resolve()
    manifest_path = Path(manifest_path)
    if not manifest_path.is_absolute():
        manifest_path = root / manifest_path
    data, paths, manifest_before = _load_manifest(manifest_path, root)
    before = {name: _sha256(path) for name, path in paths.items()}
    executable_before = _sha256(cli)
    checkout = _checkout_identity(root)
    out_dir.mkdir(parents=True, exist_ok=False)
    cases = []
    for case in data["cases"]:
        attempts = [_invoke(cli, case["args"], root, timeout, index)
                    for index in range(runs + 1)]
        identities = [(record["exit_code"], stdout, stderr)
                      for record, stdout, stderr in attempts]
        identical = all(value == identities[0] for value in identities[1:])
        records = [attempt[0] for attempt in attempts]
        failures = []
        if any(record["timed_out"] for record in records):
            failures.append("timeout")
        if any(record["launch_error"] for record in records):
            failures.append("launch_error")
        if any(record["exit_code"] != case["expected_exit"]
               for record in records if not record["timed_out"] and not record["launch_error"]):
            failures.append("unexpected_exit")
        if not identical:
            failures.append("nondeterministic_replay")
        measured = records[1:]
        times = [record["wallclock_ms"] for record in measured]
        _write_new(out_dir / f'{case["id"]}.stdout', attempts[1][1])
        _write_new(out_dir / f'{case["id"]}.stderr', attempts[1][2])
        cases.append({"id": case["id"], "args": case["args"],
                      "expected_exit": case["expected_exit"], "warmup": records[0],
                      "measured_runs": measured,
                      "wallclock_ms_summary": {"count": len(times), "min": min(times),
                                               "median": statistics.median(times), "max": max(times)},
                      "replay_identical": identical, "failures": failures,
                      "success": not failures})
    after = _after_hashes(paths)
    executable_after = _after_hashes({"cli": cli})["cli"]
    manifest_after = _after_hashes({"manifest": manifest_path})["manifest"]
    global_failures = []
    if manifest_before != manifest_after:
        global_failures.append("manifest_changed")
    if before != after:
        global_failures.append("source_changed")
    if executable_before != executable_after:
        global_failures.append("cli_binary_changed")
    result = {
        "schema_version": 1, "evidence_kind": "engineering-cli-measurement",
        "human_evidence_included": False,
        "recorded_utc": datetime.now(timezone.utc).isoformat(),
        "checkout": checkout,
        "cli": {"name": cli.name, "sha256_before": executable_before,
                "sha256_after": executable_after, "source_binding_verified": False},
        "manifest_sha256": manifest_before, "manifest_sha256_after": manifest_after,
        "platform": {"system": platform.system(), "release": platform.release(),
                     "machine": platform.machine(), "python": platform.python_version()},
        "run_policy": {"warmup_runs_per_case": 1, "measured_runs_per_case": runs,
                       "timeout_seconds": timeout, "warmup_excluded_from_statistics": True,
                       "timing_scope": "process wall time including startup and I/O; all measured attempts"},
        "input_sha256_before": before, "input_sha256_after": after,
        "all_inputs_unchanged": before == after,
        "cases": cases, "global_failures": global_failures,
        "machine_success": not global_failures and all(case["success"] for case in cases),
    }
    encoded = json.dumps(result, ensure_ascii=False, indent=2).encode("utf-8") + b"\n"
    _write_new(out_dir / "results.json", encoded)
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--out-dir", required=True, type=Path)
    parser.add_argument("--runs", type=int, default=5)
    parser.add_argument("--timeout", type=float, default=30)
    args = parser.parse_args(argv)
    try:
        result = run_manifest(args.cli, args.manifest, args.out_dir,
                              args.runs, args.timeout)
    except (OSError, ValueError) as exc:
        parser.exit(2, f"{parser.prog}: {exc}\n")
    print(json.dumps({"machine_success": result["machine_success"],
                      "human_evidence_included": False, "cases": len(result["cases"])}))
    return 0 if result["machine_success"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
