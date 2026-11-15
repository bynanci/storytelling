#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Synthetic subprocess calibration tests, not native CLI performance evidence."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("benchmark_n11_handoff.py")


def module():
    spec = importlib.util.spec_from_file_location("n11_benchmark", SCRIPT)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


class HandoffBenchmarkTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.helper = self.root / "synthetic_cli.py"
        self.helper.write_text("""from pathlib import Path
import sys
import time
mode = sys.argv[1]
source = Path(sys.argv[-1])
if mode == 'timeout':
    time.sleep(2)
if mode == 'mutate':
    source.write_bytes(b'changed')
if mode == 'error':
    print('synthetic failure', file=sys.stderr)
    raise SystemExit(1)
print('synthetic calibration' if mode != 'different' else 'different synthetic calibration')
""", encoding="utf-8")
        self.benchmark = module()

    def run_calibration(self, candidate="good", pairs=3, timeout=5, workloads=(1,)):
        commands = {"baseline": [sys.executable, str(self.helper), "good"],
                    "candidate": [sys.executable, str(self.helper), candidate]}
        return self.benchmark.benchmark(commands, {"baseline": "0" * 40, "candidate": "1" * 40},
                                        pairs=pairs, workloads=workloads, timeout=timeout)

    def test_calibration_pairs_preserve_actual_seeded_order_and_all_observations(self):
        first = self.run_calibration(pairs=7)
        second = self.run_calibration(pairs=7)
        self.assertEqual(first["status"], "ok")
        first_row, second_row = first["workloads"][0], second["workloads"][0]
        self.assertEqual([p["order"] for p in first_row["pairs"]],
                         [p["order"] for p in second_row["pairs"]])
        self.assertEqual(len(first_row["warmup"]), 1)
        self.assertEqual(len(first_row["pairs"]), 7)
        self.assertEqual({tuple(p["order"]) for p in first_row["pairs"]},
                         {("baseline", "candidate"), ("candidate", "baseline")})
        for pair in [*first_row["warmup"], *first_row["pairs"]]:
            for arm in ("baseline", "candidate"):
                observation = pair["observations"][arm]
                self.assertEqual(observation["exit"], 0)
                self.assertGreater(observation["wall_ns"], 0)
                self.assertEqual(observation["stderr_bytes"], 0)
                self.assertEqual(observation["stdout_sha256"], first_row["stdout_sha256"])
                if self.benchmark.resource is None:
                    self.assertIsNone(observation["process_cpu_seconds"])
                else:
                    self.assertGreaterEqual(observation["process_cpu_seconds"], 0)

    def test_workload_pairs_are_globally_shuffled_after_all_warmups(self):
        first = self.run_calibration(pairs=5, workloads=(1, 8, 32, 63))
        second = self.run_calibration(pairs=5, workloads=(1, 8, 32, 63))
        self.assertEqual(first["execution_order"], second["execution_order"])
        schedule = first["execution_order"]
        self.assertEqual([row["phase"] for row in schedule[:4]], ["warmup"] * 4)
        self.assertEqual([row["phase"] for row in schedule[4:]], ["measured"] * 20)
        actual = [row["artifacts"] for row in schedule[4:]]
        self.assertNotEqual(actual, sorted(actual))
        self.assertEqual(sorted(actual), [1] * 5 + [8] * 5 + [32] * 5 + [63] * 5)
        self.assertEqual({(row["artifacts"], row["index"]) for row in schedule[4:]},
                         {(count, index) for count in (1, 8, 32, 63) for index in range(1, 6)})

    def test_different_stdout_is_a_preserved_failure_not_a_speedup(self):
        result = self.run_calibration("different")
        self.assertEqual(result["status"], "failed")
        self.assertEqual(result["failure"]["reason"], "stdout_changed")
        self.assertNotIn("summary", result["workloads"][0])
        self.assertEqual(len(result["workloads"][0]["warmup"][0]["observations"]), 2)

    def test_mutating_input_is_rejected(self):
        result = self.run_calibration("mutate")
        self.assertEqual(result["status"], "failed")
        self.assertEqual(result["failure"]["reason"], "input_changed")

    def test_child_timeout_is_incomplete_and_retains_observation(self):
        result = self.run_calibration("timeout", timeout=0.1)
        self.assertEqual(result["status"], "failed")
        self.assertEqual(result["failure"]["reason"], "timeout")
        trial = result["workloads"][0]["warmup"][0]
        self.assertTrue(trial["observations"]["candidate"]["timed_out"])
        self.assertIsNone(trial["observations"]["candidate"]["exit"])

    def test_unsuccessful_cli_is_not_treated_as_a_valid_equal_output(self):
        result = self.run_calibration("error")
        self.assertEqual(result["status"], "failed")
        self.assertEqual(result["failure"]["reason"], "unsuccessful_cli")

    def test_summary_keeps_ratio_of_medians_distinct_from_paired_ratio(self):
        pairs = [{"observations": {"baseline": {"wall_ns": b, "process_cpu_seconds": None},
                                    "candidate": {"wall_ns": c, "process_cpu_seconds": None}}}
                 for b, c in [(10, 30), (20, 10), (100, 40)]]
        result = self.benchmark.summarize(pairs)
        self.assertEqual(result["ratio_of_wall_medians"], 1.5)
        self.assertEqual(result["median_paired_wall_ratio"], 0.5)
        self.assertEqual(result["baseline_wall_p95_ns"], 100)
        self.assertIsNone(result["candidate_process_cpu_median_seconds"])

    def test_frozen_workloads_have_the_original_sizes_and_ids(self):
        for count, size in [(1, 93273), (8, 199918), (32, 565580), (63, 1037896)]:
            with self.subTest(artifacts=count):
                data = self.benchmark.build_workload(count)
                self.assertEqual(len(data), size)
                value = json.loads(data)
                self.assertEqual(value["artifacts"][-1]["id"], f"a{count - 1}")
                self.assertEqual(value["sources"][-1]["unit_id"], "u255")
        with self.assertRaises(ValueError):
            self.benchmark.build_workload(64)

    def test_emit_workload_is_reproducible_and_under_the_existing_input_limit(self):
        with tempfile.TemporaryDirectory() as tmp:
            target = Path(tmp) / "input.json"
            command = [sys.executable, str(SCRIPT), "--emit-workload", "63", "--output", str(target)]
            first = subprocess.run(command, capture_output=True, check=False)
            self.assertEqual(first.returncode, 0, first.stderr.decode())
            original = target.read_bytes()
            second = subprocess.run(command, capture_output=True, check=False)
            self.assertEqual(second.returncode, 0, second.stderr.decode())
            self.assertEqual(target.read_bytes(), original)
            self.assertEqual(len(original), 1_037_896)
            document = json.loads(original)
            self.assertEqual(len(document["artifacts"]), 63)
            self.assertEqual(len(document["sources"]), 256)
            self.assertEqual(len(document["artifacts"][0]["components"]), 32)
            self.assertEqual(len(document["artifacts"][0]["components"][31]["unit_ids"]), 64)
            self.assertEqual(document["sources"][0]["revision"], 2)


if __name__ == "__main__":
    unittest.main()
