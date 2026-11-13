#!/usr/bin/env python3
"""Integrity checks for the synthetic SK-E03 instruction-conformance cases."""

import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CASES = ROOT / "docs/research/evaluations/sk-e03/cases-v1.json"
EXPECTED_CASES = {
    "C03-SYN-01",
    "C04-C06-SYN-01",
    "C07-SYN-01",
    "C24-SYN-01",
}
LOCALES = {"en", "zh-TW"}


def load_cases():
    return json.loads(CASES.read_text(encoding="utf-8"))


class SkillEvaluationCaseTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.record = load_cases()

    def test_record_is_versioned_and_provenance_limits_are_explicit(self):
        record = self.record
        self.assertEqual(record["schema_version"], "skill-evaluation-cases/v1")
        self.assertEqual(record["record_kind"], "synthetic_instruction_conformance_development_run")
        run = record["run"]
        self.assertEqual(run["skill_commit"], "e6b27d852ef259faf392645a8461c7e6dc006302")
        self.assertIsNone(run["model_revision"])
        self.assertFalse(run["blinded"])
        self.assertEqual(run["human_observations"], 0)
        self.assertFalse(run["holdout"])
        self.assertFalse(run["narrative_quality_scored"])
        self.assertIn("revision", run["capture_method"])

    def test_all_required_patterns_have_bilingual_material_and_captured_outputs(self):
        cases = self.record["cases"]
        self.assertEqual({case["case_id"] for case in cases}, EXPECTED_CASES)
        for case in cases:
            with self.subTest(case_id=case["case_id"]):
                self.assertTrue(case["pattern"])
                self.assertTrue(case["sources"])
                self.assertEqual(set(case["locales"]), LOCALES)
                self.assertEqual(case["assessment"]["narrative_quality"], "not scored")
                self.assertEqual(case["assessment"]["instruction_conformance"], "self-reviewed pass")
                self.assertFalse(case["assessment"]["failed_model_output_observed"])
                for locale, material in case["locales"].items():
                    with self.subTest(locale=locale):
                        for field in ("prompt", "input", "observed_output"):
                            self.assertIsInstance(material[field], str)
                            self.assertTrue(material[field].strip())
                        self.assertGreaterEqual(len(material["expected_invariants"]), 2)
                        self.assertGreaterEqual(len(material["failure_conditions"]), 2)
                        self.assertTrue(material["assertions"]["must_contain"])
                        self.assertTrue(material["assertions"]["must_not_contain"])

    def test_captured_outputs_meet_mechanical_presence_checks(self):
        for case in self.record["cases"]:
            for locale, material in case["locales"].items():
                output = material["observed_output"].casefold()
                with self.subTest(case_id=case["case_id"], locale=locale):
                    for token in material["assertions"]["must_contain"]:
                        self.assertIn(token.casefold(), output, f"missing required token: {token}")
                    for token in material["assertions"]["must_not_contain"]:
                        self.assertNotIn(token.casefold(), output, f"forbidden token present: {token}")

    def test_cases_cover_required_attribution_and_evidence_boundaries(self):
        by_id = {case["case_id"]: case for case in self.record["cases"]}
        self.assertIn("attribution", by_id["C03-SYN-01"]["pattern"])
        self.assertIn("agency", by_id["C04-C06-SYN-01"]["pattern"])
        self.assertIn("absent", by_id["C07-SYN-01"]["pattern"])
        self.assertIn("causal", by_id["C24-SYN-01"]["pattern"])
        c07_failures = " ".join(by_id["C07-SYN-01"]["locales"]["en"]["failure_conditions"]).casefold()
        self.assertIn("optional refreshment omission", c07_failures)
        self.assertIn("contradict", c07_failures)
        c24_en = by_id["C24-SYN-01"]["locales"]["en"]
        self.assertTrue(any("denominator" in item for item in c24_en["expected_invariants"]))
        self.assertTrue(any("Mina Park" in item for item in c24_en["expected_invariants"]))


if __name__ == "__main__":
    unittest.main()
