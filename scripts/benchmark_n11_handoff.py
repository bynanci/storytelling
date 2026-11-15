#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Paired, read-only native process benchmark on synthetic N-11 declarations.

This is machine replay, not author-time or Skill-effect evidence. Explicit binary
and source identities are required; a supplied source SHA does not prove a build.
No editor, media probe, model or network is invoked. Timing includes process
startup, JSON/file I/O and stdout capture. Input generation and hashing are outside
the timed interval. Repetitions on one machine are not independent user samples.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path
import platform
import random
import statistics
import subprocess
import sys
import tempfile
import time

try:
    import resource
except ImportError:  # Windows: stdlib has no equivalent child CPU accounting.
    resource = None

DEFAULT_SEED = 910206
DEFAULT_PAIRS = 21
WORKLOADS = (1, 8, 32, 63)
PROTOCOL = "n11-handoff-process-v1"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def build_workload(artifacts):
    if artifacts not in WORKLOADS:
        raise ValueError("unsupported synthetic workload")
    units = [{"id": f"u{i:03}", "revision": 1, "origin": "supplied_observation",
              "text": "synthetic source", "based_on": []} for i in range(256)]
    beats = [{"id": f"b{i:03}", "unit_ref": {"unit_id": f"u{i:03}", "revision": 1},
              "text": "synthetic beat"} for i in range(256)]
    components = [{"id": f"c{i}", "unit_ids": [f"u{j:03}" for j in
                   range(64 * i if i < 4 else 192, 64 * (i + 1) if i < 4 else 256)]}
                  for i in range(32)]
    value = {
        "schema_version": 1, "manifest_id": "profile",
        "ledger": {"schema_version": 1, "base_document": {
            "schema_version": 1, "document_id": "profile", "document_revision": 1,
            "purpose": "synthetic profile", "audience": "machine", "units": units,
            "relations": [], "variants": [{"id": "v", "kind": "spoken_outline",
                                            "name": "profile", "beats": beats}]},
            "events": [{"id": "e", "kind": "source_revision", "unit_id": "u000",
                        "expected_revision": 1, "text": "revised synthetic source"}]},
        "change_id": "e",
        "sources": [{"unit_id": f"u{i:03}", "revision": 2 if i == 0 else 1,
                     "representation": {"kind": "description", "media_kind": "text",
                                        "text": "synthetic description"}} for i in range(256)],
        "artifacts": [{"id": f"a{i}", "name": "profile", "medium": "text",
                       "variant_id": "v", "components": components} for i in range(artifacts)]}
    data = json.dumps(value, separators=(",", ":")).encode("utf-8")
    if len(data) > 1_048_576:
        raise ValueError("synthetic workload exceeds the accepted input limit")
    return data


def child_cpu_seconds():
    if resource is None:
        return None
    value = resource.getrusage(resource.RUSAGE_CHILDREN)
    return value.ru_utime + value.ru_stime


def invoke(command, path, timeout):
    before_cpu = child_cpu_seconds()
    start = time.perf_counter_ns()
    try:
        result = subprocess.run([*command, "render-handoff", str(path)],
                                stdin=subprocess.DEVNULL, capture_output=True,
                                timeout=timeout, check=False)
        end = time.perf_counter_ns()
        after_cpu = child_cpu_seconds()
        observation = {"timed_out": False, "exit": result.returncode,
                       "stdout_sha256": digest(result.stdout), "stdout_bytes": len(result.stdout),
                       "stderr_sha256": digest(result.stderr), "stderr_bytes": len(result.stderr)}
        payload = result.stdout
    except subprocess.TimeoutExpired as error:
        end = time.perf_counter_ns()
        after_cpu = child_cpu_seconds()
        stdout, stderr = error.stdout or b"", error.stderr or b""
        observation = {"timed_out": True, "exit": None, "stdout_sha256": digest(stdout),
                       "stdout_bytes": len(stdout), "stderr_sha256": digest(stderr),
                       "stderr_bytes": len(stderr)}
        payload = stdout
    observation["wall_ns"] = end - start
    observation["process_cpu_seconds"] = (None if before_cpu is None or after_cpu is None
                                           else max(0.0, after_cpu - before_cpu))
    return observation, payload


def nearest_rank_p95(values):
    ordered = sorted(values)
    return ordered[math.ceil(len(ordered) * 0.95) - 1]


def summarize(pairs):
    baseline = [pair["observations"]["baseline"]["wall_ns"] for pair in pairs]
    candidate = [pair["observations"]["candidate"]["wall_ns"] for pair in pairs]
    return {"pairs": len(pairs), "baseline_wall_median_ns": statistics.median(baseline),
            "candidate_wall_median_ns": statistics.median(candidate),
            "ratio_of_wall_medians": statistics.median(candidate) / statistics.median(baseline),
            "median_paired_wall_ratio": statistics.median([c / b for b, c in zip(baseline, candidate)]),
            "baseline_wall_p95_ns": nearest_rank_p95(baseline),
            "candidate_wall_p95_ns": nearest_rank_p95(candidate),
            "baseline_process_cpu_median_seconds": cpu_median(pairs, "baseline"),
            "candidate_process_cpu_median_seconds": cpu_median(pairs, "candidate")}


def cpu_median(pairs, arm):
    values = [pair["observations"][arm]["process_cpu_seconds"] for pair in pairs]
    return None if any(value is None for value in values) else statistics.median(values)


def binary_identity(command, source):
    data = Path(command[0]).read_bytes()
    return {"name": Path(command[0]).name, "sha256": digest(data), "bytes": len(data),
            "source_commit_supplied": source, "source_build_binding": "externally_recorded_not_verified_by_harness"}


def benchmark(commands, sources, *, pairs=DEFAULT_PAIRS, seed=DEFAULT_SEED,
              workloads=WORKLOADS, timeout=30):
    if pairs < 1 or timeout <= 0 or not workloads or any(n not in WORKLOADS for n in workloads):
        raise ValueError("positive pairs/timeout and supported workloads required")
    rng = random.Random(seed)
    report = {"protocol": PROTOCOL, "status": "ok", "seed": seed, "pairs_per_workload": pairs,
              "warmups_per_arm_per_workload": 1,
              "cpu_accounting": "RUSAGE_CHILDREN user+system delta" if resource else "not_available",
              "environment": {"python": platform.python_version(), "system": platform.system(),
                              "release": platform.release(), "machine": platform.machine()},
              "harness_sha256": digest(Path(__file__).read_bytes()),
              "binaries": {arm: binary_identity(command, sources[arm]) for arm, command in commands.items()},
              "scope": "synthetic process replay; repetitions are not independent author samples",
              "workloads": []}
    report["execution_order"] = []
    with tempfile.TemporaryDirectory(prefix="storytelling-n11-benchmark-") as tmp:
        inputs, rows, expected = {}, {}, {}
        for count in workloads:
            original = build_workload(count)
            path = Path(tmp) / f"synthetic-handoff-{count:02}.json"
            path.write_bytes(original)
            inputs[count] = (path, original)
            row = {"artifacts": count, "input_sha256": digest(original), "input_bytes": len(original),
                   "warmup": [], "pairs": []}
            rows[count] = row
            expected[count] = None
            report["workloads"].append(row)
        warmups = list(workloads)
        rng.shuffle(warmups)
        measured = [(count, index) for count in workloads for index in range(1, pairs + 1)]
        rng.shuffle(measured)
        schedule = [("warmup", count, 0) for count in warmups]
        schedule.extend(("measured", count, index) for count, index in measured)
        for sequence, (phase, count, index) in enumerate(schedule):
            path, original = inputs[count]
            row = rows[count]
            order = ["baseline", "candidate"]
            rng.shuffle(order)
            trial = {"index": index, "sequence": sequence, "order": order, "observations": {}}
            (row["warmup"] if phase == "warmup" else row["pairs"]).append(trial)
            report["execution_order"].append({"sequence": sequence, "phase": phase,
                                               "artifacts": count, "index": index, "order": order})
            for arm in order:
                observation, payload = invoke(commands[arm], path, timeout)
                trial["observations"][arm] = observation
                failure = None
                try:
                    unchanged = path.read_bytes() == original
                except OSError:
                    unchanged = False
                if not unchanged:
                    failure = "input_changed"
                elif observation["timed_out"]:
                    failure = "timeout"
                elif observation["exit"] != 0 or observation["stderr_bytes"] or not payload:
                    failure = "unsuccessful_cli"
                elif expected[count] is not None and payload != expected[count]:
                    failure = "stdout_changed"
                if failure:
                    report.update(status="failed", failure={"reason": failure, "artifacts": count,
                                                          "index": index, "arm": arm})
                    return report
                expected[count] = payload
        for count in workloads:
            row = rows[count]
            row["summary"] = summarize(row["pairs"])
            row["stdout_sha256"] = digest(expected[count])
            row["stdout_bytes"] = len(expected[count])
    for arm, command in commands.items():
        if binary_identity(command, sources[arm]) != report["binaries"][arm]:
            report.update(status="failed", failure={"reason": "binary_changed", "arm": arm})
            return report
    return report


def source_sha(value):
    if len(value) != 40 or any(c not in "0123456789abcdef" for c in value):
        raise argparse.ArgumentTypeError("a full lowercase 40-character source SHA is required")
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--emit-workload", type=int, choices=WORKLOADS)
    parser.add_argument("--baseline", type=Path)
    parser.add_argument("--candidate", type=Path)
    parser.add_argument("--baseline-source", type=source_sha)
    parser.add_argument("--candidate-source", type=source_sha)
    parser.add_argument("--pairs", type=int, default=DEFAULT_PAIRS)
    parser.add_argument("--seed", type=int, default=DEFAULT_SEED)
    parser.add_argument("--timeout", type=float, default=30)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if args.emit_workload is not None:
        if args.output is None:
            parser.error("--emit-workload requires --output")
        args.output.write_bytes(build_workload(args.emit_workload))
        return 0
    if None in (args.baseline, args.candidate, args.baseline_source, args.candidate_source):
        parser.error("both binaries and their full source SHAs are required")
    if args.pairs < 1 or not math.isfinite(args.timeout) or args.timeout <= 0:
        parser.error("--pairs and --timeout must be positive finite values")
    commands = {"baseline": [str(args.baseline.resolve())], "candidate": [str(args.candidate.resolve())]}
    sources = {"baseline": args.baseline_source, "candidate": args.candidate_source}
    try:
        report = benchmark(commands, sources, pairs=args.pairs, seed=args.seed, timeout=args.timeout)
    except (OSError, ValueError) as error:
        parser.exit(2, f"benchmark unavailable: {type(error).__name__}\n")
    output = (json.dumps(report, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    if args.output:
        args.output.write_bytes(output)
    else:
        sys.stdout.buffer.write(output)
    return 0 if report["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
