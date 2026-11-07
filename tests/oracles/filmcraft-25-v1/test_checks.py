# SPDX-License-Identifier: Apache-2.0
"""Negative checks must reject plausible-looking but incomplete host evidence."""
import importlib.util
import copy
import json
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("host_proof", Path(__file__).with_name("run.py"))
proof = importlib.util.module_from_spec(spec)
spec.loader.exec_module(proof)


class EvidenceChecks(unittest.TestCase):
    def baseline(self, filename):
        return json.loads(Path(__file__).with_name(filename).read_text())

    def test_accepts_complete_valid_import_and_sequence(self):
        proof.check_import(self.baseline("valid-import.json"))
        self.assertEqual(len(proof.check_sequence(self.baseline("valid-sequence.json"))), 3)

    def test_rejects_nonempty_import_errors_even_if_process_succeeded(self):
        data = self.baseline("valid-import.json")
        data["errors"] = ["failed"]
        with self.assertRaises(ValueError):
            proof.check_import(data)

    def test_rejects_offline_or_missing_import_fields(self):
        for field, value in [("offlineMedia", ["missing"]), ("report", ["loss"]), ("linkedMedia", 1)]:
            data = self.baseline("valid-import.json")
            data["documents"][0][field] = value
            with self.assertRaises(ValueError):
                proof.check_import(data)
        with self.assertRaises(ValueError):
            proof.check_import({})

    def test_rejects_wrong_movie_color_order(self):
        with self.assertRaises(ValueError):
            proof.check_colors(bytes([0, 0, 255]) * 45, ["red"] * 20 + ["blue"] * 5 + ["green"] * 20)

    def test_rejects_unexpected_native_ticks(self):
        baseline = self.baseline("valid-sequence.json")
        data = copy.deepcopy(baseline)
        data["duration"] += 1
        with self.assertRaises(ValueError):
            proof.check_sequence(data)
        for field, value in [("sourceIn", 0), ("speed", 2), ("reverse", True), ("item", baseline["video"][0]["items"][1]["item"])]:
            data = copy.deepcopy(baseline)
            data["video"][0]["items"][0][field] = value
            with self.assertRaises(ValueError):
                proof.check_sequence(data)

    def test_rejects_unrecognized_success_response_fields(self):
        with self.assertRaises(ValueError):
            proof.shape({"path": "a", "bytes": 1, "report": [], "errors": ["failed"]}, ("path", "bytes", "report"))


if __name__ == "__main__":
    unittest.main()
