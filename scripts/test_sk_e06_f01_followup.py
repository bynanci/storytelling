#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Offline integrity tests for the versioned SK-E06-F01 follow-up packet."""
import json
import shutil
import tempfile
import unittest
from pathlib import Path

import sys
sys.path.insert(0, str(Path(__file__).resolve().parent))

from sk_e06_f01_followup import (
    EXPECTED_ORIGINAL_ARTIFACTS,
    F01FollowupError,
    strict_json_loads,
    validate_followup,
)

ROOT = Path(__file__).resolve().parents[1]
FOLLOWUP = ROOT / "docs/research/evaluations/sk-e06-f01-followup-v1"


class FollowupTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.repo_root = Path(self.temp.name) / "repo"
        self.followup_root = self.repo_root / "docs/research/evaluations/sk-e06-f01-followup-v1"
        shutil.copytree(FOLLOWUP, self.followup_root)
        for relative_path in EXPECTED_ORIGINAL_ARTIFACTS:
            source = ROOT / relative_path
            target = self.repo_root / relative_path
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, target)

    def read_json(self, name):
        return strict_json_loads((self.followup_root / name).read_bytes())

    def write_json(self, name, value):
        (self.followup_root / name).write_text(
            json.dumps(value, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
            newline="\n",
        )

    def test_packet_is_versioned_design_material_only(self):
        result = validate_followup(self.followup_root, self.repo_root)
        self.assertEqual(result["scope"], "followup_design_integrity_only")
        self.assertEqual(result["case_count"], 2)
        self.assertEqual(result["locales"], ["en", "zh-TW"])
        self.assertEqual(result["pinned_original_artifact_count"], 3)
        self.assertFalse(result["can_dispatch"])
        self.assertEqual(result["maintainer_decision"], "pending")

    def test_original_case_input_and_response_are_pinned(self):
        result = validate_followup(self.followup_root, self.repo_root)
        pins = result["verified_original_artifacts"]
        self.assertEqual(set(pins), set(EXPECTED_ORIGINAL_ARTIFACTS))
        self.assertTrue(all(pins.values()))

        relative_path = next(iter(EXPECTED_ORIGINAL_ARTIFACTS))
        source = self.repo_root / relative_path
        source.write_bytes(source.read_bytes() + b"\n")
        with self.assertRaises(F01FollowupError):
            validate_followup(self.followup_root, self.repo_root)

    def test_rejects_duplicate_keys_and_nonfinite_numbers(self):
        for raw in (b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":Infinity}'):
            with self.subTest(raw=raw), self.assertRaises(F01FollowupError):
                strict_json_loads(raw)

    def test_adjudication_stays_pending_and_has_no_composite_score(self):
        adjudication = self.read_json("adjudication-proposal.json")
        self.assertEqual(adjudication["status"], "proposal_pending_maintainer_decision")
        self.assertEqual(adjudication["maintainer_disposition"], None)
        self.assertEqual(adjudication["overall_conformance_status"], "disputed_no_single_label")
        self.assertIsNone(adjudication["aggregate_score"])

        adjudication["status"] = "accepted"
        adjudication["maintainer_disposition"] = "approved"
        self.write_json("adjudication-proposal.json", adjudication)
        with self.assertRaises(F01FollowupError):
            validate_followup(self.followup_root, self.repo_root)

    def test_case_a_keeps_model_suggestion_unadopted(self):
        packet = self.read_json("cases.json")
        case_a = next(case for case in packet["cases"] if case["case_id"] == "SK-E06-F01-A")
        self.assertFalse(case_a["execution"]["dispatch_authorized"])
        for locale in ("en", "zh-TW"):
            suggestion = next(
                source for source in case_a["locales"][locale]["sources"]
                if source["source_id"] == "MODEL-SUGGESTION-01@1"
            )
            self.assertEqual(suggestion["adoption_status"], "unadopted")
            self.assertEqual(suggestion["source_kind"], "model_suggestion")
        case_a["locales"]["zh-TW"]["sources"][-1]["adoption_status"] = "adopted"
        self.write_json("cases.json", packet)
        with self.assertRaises(F01FollowupError):
            validate_followup(self.followup_root, self.repo_root)

    def test_case_b_requires_fixture_decision_for_creative_scope_only(self):
        packet = self.read_json("cases.json")
        case_b = next(case for case in packet["cases"] if case["case_id"] == "SK-E06-F01-B")
        for locale in ("en", "zh-TW"):
            decision = next(
                source for source in case_b["locales"][locale]["sources"]
                if source["source_id"] == "CHOICE-UMBRELLA-02@1"
            )
            self.assertEqual(decision["source_kind"], "fictional_author_decision")
            self.assertEqual(decision["decision_scope"], ["spoken_outline", "illustration_a"])
            self.assertFalse(decision["changes_observation"])
        case_b["locales"]["en"]["sources"][-1]["source_kind"] = "supplied_observation"
        self.write_json("cases.json", packet)
        with self.assertRaises(F01FollowupError):
            validate_followup(self.followup_root, self.repo_root)

    def test_each_deliverable_has_a_locale_specific_source_map(self):
        packet = self.read_json("cases.json")
        for case in packet["cases"]:
            for locale in ("en", "zh-TW"):
                deliverables = case["locales"][locale]["expected"]["deliverables"]
                self.assertEqual(
                    {item["deliverable_id"] for item in deliverables},
                    {"spoken_outline", "illustration_a", "illustration_b"},
                )
                for item in deliverables:
                    self.assertTrue(item["source_map"], (case["case_id"], locale, item["deliverable_id"]))
        case_b = next(case for case in packet["cases"] if case["case_id"] == "SK-E06-F01-B")
        oral = next(
            item for item in case_b["locales"]["zh-TW"]["expected"]["deliverables"]
            if item["deliverable_id"] == "spoken_outline"
        )
        self.assertIn("CHOICE-UMBRELLA-02@1", oral["source_map"])
        oral["source_map"].remove("CHOICE-UMBRELLA-02@1")
        self.write_json("cases.json", packet)
        with self.assertRaises(F01FollowupError):
            validate_followup(self.followup_root, self.repo_root)

    def test_both_successor_cases_remain_unexecuted_and_non_dispatchable(self):
        packet = self.read_json("cases.json")
        self.assertFalse(packet["execution"]["can_dispatch"])
        self.assertEqual(packet["execution"]["model_runs"], 0)
        self.assertEqual(packet["execution"]["human_observations"], 0)
        for case in packet["cases"]:
            self.assertEqual(case["execution"]["status"], "not_run")
            self.assertFalse(case["execution"]["dispatch_authorized"])
        packet["execution"]["can_dispatch"] = True
        self.write_json("cases.json", packet)
        with self.assertRaises(F01FollowupError):
            validate_followup(self.followup_root, self.repo_root)


if __name__ == "__main__":
    unittest.main()
