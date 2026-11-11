# SPDX-License-Identifier: MIT
"""Synthetic helper regressions; process and clock mocks are not timing evidence."""
import copy
import importlib.util
import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("n07_run_cli", HERE / "run_cli.py")
run_cli = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(run_cli)


class MeasurementTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.input = self.root / "input.json"
        self.input.write_text('{"synthetic":true}\n', encoding="utf-8")
        self.cli = self.root / "storytelling"
        self.cli.write_bytes(b"mock executable, never invoked")
        self.manifest = self.root / "cases.json"
        self.data = {"schema_version": 1, "input_files": ["input.json"], "cases": [
            {"id": "narrative", "args": ["render-narrative", "input.json"], "expected_exit": 0}
        ]}
        self.save()
        self.out = self.root / "receipts"
        self.checkout = patch.object(run_cli, "_checkout_identity", return_value={
            "commit": "1" * 40, "dirty": False})
        self.checkout.start()
        self.addCleanup(self.checkout.stop)

    def save(self):
        self.manifest.write_text(json.dumps(self.data), encoding="utf-8")

    def invoke(self, **kwargs):
        return run_cli.run_manifest(self.cli, self.manifest, self.out,
                                    root=self.root, **kwargs)

    def test_refuses_existing_output_without_touching_private_records(self):
        self.out.mkdir()
        sentinel = self.out / "private-observations.csv"
        sentinel.write_text("do not overwrite", encoding="utf-8")
        with patch.object(run_cli.subprocess, "run") as proc:
            with self.assertRaises(ValueError):
                self.invoke()
            proc.assert_not_called()
        self.assertEqual(sentinel.read_text(encoding="utf-8"), "do not overwrite")

    def test_rejects_bad_manifest_paths_commands_and_run_bounds_before_writing(self):
        valid = copy.deepcopy(self.data)
        bad = []
        for field, value in [("schema_version", 2), ("unexpected", True),
                             ("input_files", ["../outside.json"]),
                             ("input_files", ["missing.json"]), ("cases", [])]:
            data = copy.deepcopy(valid)
            data[field] = value
            bad.append(data)
        for case in [
            {"id": "../escape", "args": ["render-narrative", "input.json"], "expected_exit": 0},
            {"id": "bad", "args": ["export-otio", "input.json"], "expected_exit": 0},
            {"id": "bad", "args": ["render-narrative", "-"], "expected_exit": 0},
            {"id": "bad", "args": ["render-narrative", "undeclared.json"], "expected_exit": 0},
            {"id": "bad", "args": ["render-narrative", "input.json"], "expected_exit": 2},
        ]:
            data = copy.deepcopy(valid)
            data["cases"] = [case]
            bad.append(data)
        duplicate = copy.deepcopy(valid)
        duplicate["cases"] *= 2
        bad.append(duplicate)
        with patch.object(run_cli.subprocess, "run") as proc:
            for data in bad:
                with self.subTest(data=data):
                    self.data = data
                    self.save()
                    with self.assertRaises(ValueError):
                        self.invoke()
                    self.assertFalse(self.out.exists())
            self.data = valid
            self.save()
            for kwargs in [{"runs": 0}, {"runs": 21}, {"timeout": 0}, {"timeout": 61}]:
                with self.subTest(kwargs=kwargs):
                    with self.assertRaises(ValueError):
                        self.invoke(**kwargs)
                    self.assertFalse(self.out.exists())
            proc.assert_not_called()

    def test_warmup_is_excluded_from_process_timing_and_human_metrics_absent(self):
        answer = subprocess.CompletedProcess([], 0, b"deterministic\n", b"")
        clocks = [0, 100, 101, 101.004, 102, 102.006]
        with patch.object(run_cli.subprocess, "run", return_value=answer) as proc, \
                patch.object(run_cli.time, "perf_counter", side_effect=clocks):
            result = self.invoke(runs=2)
        self.assertEqual(proc.call_count, 3)
        case = result["cases"][0]
        self.assertEqual(case["wallclock_ms_summary"]["count"], 2)
        self.assertAlmostEqual(case["wallclock_ms_summary"]["min"], 4)
        self.assertAlmostEqual(case["wallclock_ms_summary"]["median"], 5)
        self.assertAlmostEqual(case["wallclock_ms_summary"]["max"], 6)
        self.assertTrue(result["machine_success"])
        self.assertFalse(result["human_evidence_included"])
        self.assertTrue(result["all_inputs_unchanged"])
        self.assertFalse(result["cli"]["source_binding_verified"])
        self.assertEqual((self.out / "narrative.stdout").read_bytes(), b"deterministic\n")
        self.assertEqual((self.out / "narrative.stderr").read_bytes(), b"")
        self.assertEqual(json.loads((self.out / "results.json").read_text()), result)
        encoded = json.dumps(result)
        self.assertNotIn('"participant"', encoded)
        self.assertNotIn('"completion_rate"', encoded)
        self.assertNotIn('"author_seconds"', encoded)
        for call in proc.call_args_list:
            self.assertEqual(call.kwargs["cwd"], self.root.resolve())
            self.assertEqual(call.kwargs["timeout"], 30)
            self.assertEqual(call.args[0], [str(self.cli.resolve()), "render-narrative", "input.json"])

    def test_deep_manifest_is_rejected_before_output_or_process_invocation(self):
        depth = 20_000
        raw = ('{"schema_version":1,"input_files":["input.json"],'
               '"cases":[],"extra":' + "[" * depth + "0" + "]" * depth + "}")
        self.manifest.write_text(raw, encoding="utf-8")
        self.assertLess(self.manifest.stat().st_size, run_cli.MAX_MANIFEST_BYTES)
        before = self.input.read_bytes()
        with patch.object(run_cli.subprocess, "run") as proc:
            with self.assertRaisesRegex(ValueError, "manifest"):
                self.invoke()
            proc.assert_not_called()
        self.assertFalse(self.out.exists())
        self.assertEqual(self.input.read_bytes(), before)

    def test_unexpected_exit_is_machine_failure_with_saved_evidence(self):
        answer = subprocess.CompletedProcess([], 1, b'{"ok":false}\n', b"failure\n")
        with patch.object(run_cli.subprocess, "run", return_value=answer):
            result = self.invoke(runs=1)
        self.assertFalse(result["machine_success"])
        self.assertIn("unexpected_exit", result["cases"][0]["failures"])
        self.assertEqual((self.out / "narrative.stderr").read_bytes(), b"failure\n")

    def test_timeout_is_recorded_separately_and_does_not_become_completion(self):
        with patch.object(run_cli.subprocess, "run", side_effect=subprocess.TimeoutExpired(
                ["storytelling"], 1, output=b"partial", stderr=b"")):
            result = self.invoke(runs=1, timeout=1)
        self.assertFalse(result["machine_success"])
        self.assertIn("timeout", result["cases"][0]["failures"])
        self.assertTrue(result["cases"][0]["measured_runs"][0]["timed_out"])
        self.assertIsNone(result["cases"][0]["measured_runs"][0]["exit_code"])

    def test_manifest_mutation_is_failure_and_original_identity_is_retained(self):
        original = self.manifest.read_bytes()

        def changing_manifest(*args, **kwargs):
            self.manifest.write_text("changed after loading", encoding="utf-8")
            return subprocess.CompletedProcess([], 0, b"same", b"")

        with patch.object(run_cli.subprocess, "run", side_effect=changing_manifest):
            result = self.invoke(runs=1)
        self.assertFalse(result["machine_success"])
        self.assertIn("manifest_changed", result["global_failures"])
        self.assertEqual(result["manifest_sha256"], run_cli.hashlib.sha256(original).hexdigest())
        self.assertNotEqual(result["manifest_sha256"], result["manifest_sha256_after"])

    def test_nondeterminism_and_source_mutation_are_failures(self):
        invocations = 0

        def changing(*args, **kwargs):
            nonlocal invocations
            invocations += 1
            self.input.write_text("changed by mocked process", encoding="utf-8")
            return subprocess.CompletedProcess([], 0, str(invocations).encode(), b"")

        with patch.object(run_cli.subprocess, "run", side_effect=changing):
            result = self.invoke(runs=2)
        self.assertFalse(result["machine_success"])
        self.assertFalse(result["all_inputs_unchanged"])
        self.assertIn("nondeterministic_replay", result["cases"][0]["failures"])
        self.assertNotEqual(result["input_sha256_before"], result["input_sha256_after"])


if __name__ == "__main__":
    unittest.main()
