#!/usr/bin/env python3
"""Tests for SK-E04 protocol and descriptive paired-order analysis."""

import copy
import json
import subprocess
import sys
import unittest
from pathlib import Path

from sk_e04_order_analysis import RecordError, analyze_record, validate_record


ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "docs/research/evaluations/sk-e04/fixtures/synthetic-order-pairs-v1.json"
PROTOCOL = ROOT / "docs/research/evaluations/sk-e04/protocol-v1.json"


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


class OrderAnalysisTests(unittest.TestCase):
    def setUp(self):
        self.fixture = load_json(FIXTURE)

    def test_protocol_records_known_baseline_and_missing_replay_material(self):
        protocol = load_json(PROTOCOL)
        self.assertEqual(protocol["schema_version"], "skill-evaluation-order-protocol/v1")
        self.assertEqual(protocol["status"], "prepared_blocked")
        self.assertEqual(protocol["analysis_unit"], "case")
        inventory = protocol["evidence_inventory"]
        self.assertEqual(inventory["r1_case_count"], 40)
        self.assertEqual(inventory["r1_presentation_order"], "X_then_Y for all 40 cases")
        self.assertFalse(inventory["original_pair_texts_available_in_public_package"])
        self.assertFalse(inventory["exact_original_prompts_available"])
        self.assertFalse(inventory["judge_model_revision_and_settings_available"])
        self.assertEqual(protocol["order_only_run"]["status"], "not_started_blocked")

    def test_synthetic_pair_result_is_descriptive_and_case_based(self):
        report = analyze_record(self.fixture)
        self.assertEqual(report["record_kind"], "synthetic_fixture")
        self.assertEqual(report["analysis_unit"], "case")
        self.assertEqual(report["case_count"], 4)
        self.assertEqual(report["preference_flip_count"], 1)
        self.assertEqual(report["preference_stable_count"], 3)
        self.assertEqual(report["preference_flip_rate"], 0.25)
        self.assertEqual(report["mean_x_minus_y_order_gap_shift"], 0.5)
        self.assertEqual(report["raw_adjudicated_disagreement_case_ids"], ["SYN-ORDER-02"])
        self.assertEqual(report["pending_adjudication_run_count"], 1)
        self.assertEqual(report["unresolved_source_check_counts"], {"insufficient": 2, "unavailable": 2})
        self.assertEqual(report["claims"], [])

    def test_rejects_unbalanced_or_changed_order_input(self):
        bad = copy.deepcopy(self.fixture)
        bad["cases"][0]["runs"][1]["order"] = "X_then_Y"
        bad["cases"][0]["runs"][1]["presented_ids"] = ["X", "Y"]
        with self.assertRaises(RecordError):
            validate_record(bad)

    def test_rejects_a_history_score_leak(self):
        bad = copy.deepcopy(self.fixture)
        bad["cases"][0]["r1_preference"] = "X"
        with self.assertRaises(RecordError):
            validate_record(bad)

    def test_rejects_malformed_digest_and_noninteger_rating(self):
        bad_hash = copy.deepcopy(self.fixture)
        bad_hash["cases"][0]["shared_inputs"]["packet_sha256"] = "not-a-digest"
        with self.assertRaises(RecordError):
            validate_record(bad_hash)
        bad_score = copy.deepcopy(self.fixture)
        bad_score["cases"][0]["runs"][0]["raw"]["ratings"]["X"][0] = True
        with self.assertRaises(RecordError):
            validate_record(bad_score)

    def test_cli_emits_valid_json_for_synthetic_fixture(self):
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts/sk_e04_order_analysis.py"), "analyze", str(FIXTURE)],
            cwd=ROOT, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        )
        report = json.loads(result.stdout)
        self.assertEqual(report["case_count"], 4)
        self.assertEqual(report["analysis_kind"], "descriptive_only")


if __name__ == "__main__":
    unittest.main()
