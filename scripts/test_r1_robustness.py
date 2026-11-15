#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Synthetic arithmetic and real-entry regressions; no judge or human calls."""
import contextlib
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.dont_write_bytecode = True
import r1_robustness as tool

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "docs/research/evaluations/r1-robustness"


def fixture():
    return json.loads((EVIDENCE / "fixtures/synthetic-four-cases.json").read_text(encoding="utf-8"))


class R1RobustnessTests(unittest.TestCase):
    def test_synthetic_cases_are_units_and_exact_means(self):
        report = tool.analyze_payload(fixture())
        self.assertEqual(report["denominators"], {"cases": 4, "drafts": 8, "dimensions_per_draft": 7, "ratings": 56})
        self.assertEqual(report["declared_mean_difference"]["fraction"], "1/4")
        self.assertEqual(report["leave_one_case_out"]["range"]["minimum"]["fraction"], "-1/3")
        self.assertEqual(report["leave_one_case_out"]["range"]["maximum"]["fraction"], "1/1")
        self.assertTrue(all(row["remaining_case_count"] == 3 for row in report["leave_one_case_out"]["rows"]))
        self.assertTrue(all(row["difference"]["fraction"] == "1/4" for row in report["leave_one_dimension_out"]["rows"]))
        self.assertEqual([row["difference"]["fraction"] for row in report["individual_batches"]], ["3/2", "-1/1"])
        self.assertEqual(report["leave_one_batch_out"]["range"]["minimum"]["fraction"], "-1/1")
        self.assertEqual(report["leave_one_batch_out"]["range"]["maximum"]["fraction"], "3/2")

    def test_enumeration_and_unconstrained_bounds_are_explicit(self):
        report = tool.analyze_payload(fixture())
        sensitivity = report["early_key_sensitivity"]
        self.assertEqual(sensitivity["case_ids"], ["S01", "S02"])
        self.assertEqual(sensitivity["other_declared_keys_held_fixed"], ["S03", "S04"])
        for key, count, low, high in [("unrestricted", 4, "-5/4", "1/4"), ("preserve_declared_X_count", 2, "-5/4", "1/4")]:
            self.assertEqual(sensitivity[key]["arrangements"], count)
            self.assertEqual(sensitivity[key]["range"]["minimum"]["fraction"], low)
            self.assertEqual(sensitivity[key]["range"]["maximum"]["fraction"], high)
        self.assertEqual(report["all_keys_unknown_bounds"]["minimum"]["fraction"], "-5/4")
        self.assertEqual(report["all_keys_unknown_bounds"]["maximum"]["fraction"], "5/4")

        # Both raw differences positive: preserving one X and one Y can no
        # longer attain the unrestricted extrema. Hand sums: fixed -2, early
        # +/- (1+2) unrestricted versus +/- (1-2) constrained, then divide by 4.
        swapped = fixture()
        swapped["cases"][1]["ratings_X"], swapped["cases"][1]["ratings_Y"] = (
            swapped["cases"][1]["ratings_Y"], swapped["cases"][1]["ratings_X"])
        constrained = tool.analyze_payload(swapped)["early_key_sensitivity"]
        self.assertEqual(constrained["unrestricted"]["range"]["minimum"]["fraction"], "-5/4")
        self.assertEqual(constrained["preserve_declared_X_count"]["range"]["minimum"]["fraction"], "-3/4")
        self.assertEqual(constrained["preserve_declared_X_count"]["range"]["maximum"]["fraction"], "-1/4")

    def test_dimension_omission_recalculates_mean_instead_of_reusing_composite(self):
        payload = fixture()
        payload["cases"][0]["ratings_X"][0] = 4
        report = tool.analyze_payload(payload)
        self.assertEqual(report["declared_mean_difference"]["fraction"], "3/14")
        rows = report["leave_one_dimension_out"]["rows"]
        self.assertEqual(rows[0]["difference"]["fraction"], "1/4")
        self.assertEqual(rows[1]["difference"]["fraction"], "5/24")

    def test_case_and_batch_input_order_does_not_change_diagnostics(self):
        payload = fixture()
        expected = tool.analyze_payload(payload)
        payload["cases"].reverse()
        self.assertEqual(tool.analyze_payload(payload), expected)

    def test_bad_scores_references_and_domains_are_rejected(self):
        mutations = [
            lambda p: p["cases"][0]["ratings_X"].__setitem__(0, True),
            lambda p: p["cases"][0]["ratings_X"].__setitem__(0, 3.5),
            lambda p: p["cases"][0]["ratings_X"].__setitem__(0, 0),
            lambda p: p["cases"][0]["ratings_X"].append(5),
            lambda p: p["cases"][0].__setitem__("batch", True),
            lambda p: p["cases"][0].__setitem__("batch", 0),
            lambda p: p["cases"][0].__setitem__("declared_story_variant", "Z"),
            lambda p: p["cases"][0].__setitem__("assignment_provenance", "verified"),
            lambda p: p["cases"][0].__setitem__("case_id", "S02"),
            lambda p: p["dimensions"].__setitem__(0, "unknown"),
            lambda p: p.__setitem__("cases", []),
        ]
        for mutation in mutations:
            with self.subTest(mutation=mutations.index(mutation)):
                payload = fixture(); mutation(payload)
                with self.assertRaises(tool.InputError):
                    tool.analyze_payload(payload)

    def test_single_case_or_single_batch_omissions_stay_null(self):
        payload = fixture(); payload["cases"] = payload["cases"][:1]
        report = tool.analyze_payload(payload)
        self.assertIsNone(report["leave_one_case_out"]["rows"][0]["difference"])
        self.assertIsNone(report["leave_one_case_out"]["range"])
        self.assertIsNone(report["leave_one_batch_out"]["range"])

    def test_exact_fraction_and_decimal_labels(self):
        from fractions import Fraction
        self.assertEqual(tool.rational(Fraction(-1, 3)), {"fraction": "-1/3", "approx_decimal": "-0.333333"})
        self.assertEqual(tool.rational(Fraction(0)), {"fraction": "0/1", "approx_decimal": "0.000000"})

    def test_strict_json_and_input_bounds(self):
        for data in [b'{"cases":[],"cases":[]}', b'{"nested":{"x":1,"x":2}}', b'{"value":NaN}', b'\xff', b'[' * 40 + b']' * 40, b' ' * (tool.MAX_BYTES + 1)]:
            with self.subTest(data=data[:40]), self.assertRaises(tool.InputError):
                tool.parse_json(data)

    def test_missing_evidence_returns_exit_two_and_no_report(self):
        with tempfile.TemporaryDirectory() as temp, mock.patch.object(tool, "PACKAGE", Path(temp)):
            out, err = io.StringIO(), io.StringIO()
            with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
                code = tool.main([])
            self.assertEqual(code, 2)
            self.assertEqual(out.getvalue(), "")
            self.assertIn("R1 robustness input error", err.getvalue())

    def test_real_entry_is_deterministic_matches_report_and_preserves_sources(self):
        originals = {p.name: p.read_bytes() for p in tool.PACKAGE.iterdir() if p.is_file()}
        commands = [sys.executable, str(ROOT / "scripts/r1_robustness.py")]
        first = subprocess.run(commands, check=True, capture_output=True)
        second = subprocess.run(commands, check=True, capture_output=True)
        self.assertEqual(first.stderr, b"")
        self.assertEqual(first.stdout, second.stdout)
        self.assertEqual(first.stdout, (EVIDENCE / "report.json").read_bytes())
        self.assertEqual(originals, {p.name: p.read_bytes() for p in tool.PACKAGE.iterdir() if p.is_file()})
        report = json.loads(first.stdout)
        self.assertEqual(report["denominators"]["cases"], 40)
        self.assertEqual(report["declared_mean_difference"]["fraction"], "-51/280")
        self.assertEqual(report["early_key_sensitivity"]["unrestricted"]["arrangements"], 1024)
        self.assertEqual(report["early_key_sensitivity"]["preserve_declared_X_count"]["arrangements"], 252)
        for name, digest in report["input_sha256"].items():
            self.assertEqual(hashlib.sha256((tool.PACKAGE / name).read_bytes()).hexdigest(), digest)


if __name__ == "__main__":
    unittest.main()
