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
FIXTURE_V1 = ROOT / "docs/research/evaluations/sk-e04/fixtures/synthetic-order-pairs-v1.json"
FIXTURE_V2 = ROOT / "docs/research/evaluations/sk-e04/fixtures/synthetic-order-pairs-v2.json"
PROTOCOL_V1 = ROOT / "docs/research/evaluations/sk-e04/protocol-v1.json"
PROTOCOL_V2 = ROOT / "docs/research/evaluations/sk-e04/protocol-v2.json"


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


class OrderAnalysisTests(unittest.TestCase):
    def setUp(self):
        self.fixture = load_json(FIXTURE_V2)

    def test_protocol_freezes_missing_call_and_comparability_rules(self):
        protocol = load_json(PROTOCOL_V2)
        self.assertEqual(protocol["schema_version"], "skill-evaluation-order-protocol/v2")
        self.assertEqual(protocol["supersedes"], "skill-evaluation-order-protocol/v1")
        self.assertEqual(protocol["record_schema_version"], "skill-evaluation-order-record/v2")
        self.assertEqual(protocol["status"], "prepared_blocked")
        self.assertEqual(protocol["analysis_unit"], "case")
        inventory = protocol["evidence_inventory"]
        self.assertEqual(inventory["r1_case_count"], 40)
        self.assertEqual(inventory["r1_presentation_order"], "X_then_Y for all 40 cases")
        self.assertFalse(inventory["original_pair_texts_available_in_public_package"])
        self.assertFalse(inventory["exact_original_prompts_available"])
        self.assertFalse(inventory["judge_model_revision_and_settings_available"])
        self.assertEqual(protocol["order_only_run"]["status"], "not_started_blocked")
        self.assertIn("not_attempted", protocol["record_contract"]["run_slot_fields"]["call_outcome"])
        self.assertIn("comparable_pair_count", protocol["record_contract"]["required_denominators"])

    def test_v1_complete_record_remains_accepted(self):
        legacy = load_json(FIXTURE_V1)
        report = analyze_record(legacy)
        self.assertEqual(report["record_schema_version"], "skill-evaluation-order-record/v1")
        self.assertEqual(report["planned_pair_count"], 4)
        self.assertEqual(report["comparable_pair_count"], 4)
        self.assertEqual(report["incomplete_pair_count"], 0)
        self.assertEqual(report["mean_absolute_score_change_by_draft_and_dimension"]["X"]["factual_accuracy"], 0.25)
        self.assertEqual(report["mean_absolute_score_change_by_draft_and_dimension"]["Y"]["factual_accuracy"], 0.25)

    def test_v2_preserves_failed_and_unattempted_calls_in_denominators(self):
        report = analyze_record(self.fixture)
        self.assertEqual(report["record_kind"], "synthetic_fixture")
        self.assertEqual(report["case_count"], 4)
        self.assertEqual(report["case_count_with_comparable_pairs"], 3)
        self.assertEqual(report["planned_pair_count"], 5)
        self.assertEqual(report["comparable_pair_count"], 3)
        self.assertEqual(report["incomplete_pair_count"], 2)
        self.assertEqual(report["failed_pair_count"], 2)
        self.assertEqual(report["not_attempted_pair_count"], 1)
        self.assertEqual(report["matched_preference_flip_count"], 1)
        self.assertEqual(report["matched_preference_pair_count"], 3)
        self.assertEqual(report["matched_preference_flip_rate"], 0.333333)
        self.assertEqual(report["mean_x_minus_y_order_gap_shift"], 0.666667)
        self.assertEqual(report["call_outcome_counts"]["valid_annotation"], 7)
        self.assertEqual(report["call_outcome_counts"]["timeout"], 1)
        self.assertEqual(report["call_outcome_counts"]["refusal"], 1)
        self.assertEqual(report["call_outcome_counts"]["not_attempted"], 1)
        self.assertEqual(report["per_case"][1]["comparable_pair_count"], 0)
        self.assertIsNone(report["per_case"][1]["order_gap_shift"])
        self.assertIsNone(report["per_case"][1]["preference_flip_rate"])
        self.assertEqual(report["raw_adjudicated_disagreement_case_ids"], ["SYN-ORDER-02"])
        self.assertEqual(report["pending_adjudication_run_count"], 1)
        self.assertEqual(report["unresolved_source_check_counts"], {"insufficient": 2, "unavailable": 2})

    def test_draft_score_changes_are_not_averaged_together(self):
        one_sided = copy.deepcopy(self.fixture)
        case = one_sided["cases"][0]
        xy = next(run for run in case["runs"] if run["replicate_id"] == "R1" and run["order"] == "X_then_Y")
        yx = next(run for run in case["runs"] if run["replicate_id"] == "R1" and run["order"] == "Y_then_X")
        xy["raw"]["ratings"]["X"] = [1] * 7
        yx["raw"]["ratings"]["X"] = [5] * 7
        xy["raw"]["ratings"]["Y"] = [3] * 7
        yx["raw"]["ratings"]["Y"] = [3] * 7
        report = analyze_record(one_sided)
        changes = report["per_case"][0]["mean_absolute_score_change_by_draft_and_dimension"]
        self.assertEqual(changes["X"]["factual_accuracy"], 4.0)
        self.assertEqual(changes["Y"]["factual_accuracy"], 0.0)

    def test_case_without_comparable_pairs_is_kept_with_null_metrics(self):
        no_results = copy.deepcopy(self.fixture)
        case = no_results["cases"][2]
        for run in case["runs"]:
            run.update({
                "call_outcome": "not_attempted",
                "judge_context_id": None,
                "raw_output_sha256": None,
                "raw_output_ref": None,
                "raw": None,
                "adjudicated": None,
            })
        report = analyze_record(no_results)
        self.assertEqual(report["case_count"], 4)
        self.assertEqual(report["case_count_with_comparable_pairs"], 2)
        self.assertEqual(report["per_case"][2]["planned_pair_count"], 1)
        self.assertEqual(report["per_case"][2]["comparable_pair_count"], 0)
        self.assertIsNone(report["per_case"][2]["preference_profile_xy"])
        self.assertIsNone(report["per_case"][2]["mean_absolute_score_change_by_draft_and_dimension"]["X"]["clarity"])

    def test_invalid_annotation_and_request_error_are_counted_as_failures(self):
        for outcome in ("invalid_annotation", "request_error"):
            with self.subTest(outcome=outcome):
                record = copy.deepcopy(self.fixture)
                run = next(run for case in record["cases"] for run in case["runs"]
                           if run["call_outcome"] == "valid_annotation")
                run["call_outcome"] = outcome
                run["raw"] = None
                run["adjudicated"] = None
                if outcome == "request_error":
                    run["raw_output_sha256"] = None
                    run["raw_output_ref"] = None
                report = analyze_record(record)
                self.assertEqual(report["call_outcome_counts"][outcome], 1)
                self.assertEqual(report["failed_call_count"], 3)
                self.assertEqual(report["failed_pair_count"], 3)

    def test_observed_run_requires_two_replicate_pairs_per_case(self):
        observed = copy.deepcopy(self.fixture)
        observed["record_kind"] = "observed_order_rerun"
        with self.assertRaises(RecordError):
            validate_record(observed)

    def test_all_incomplete_record_has_null_aggregate_comparisons(self):
        no_results = copy.deepcopy(self.fixture)
        for case in no_results["cases"]:
            for run in case["runs"]:
                run.update({
                    "call_outcome": "not_attempted",
                    "judge_context_id": None,
                    "raw_output_sha256": None,
                    "raw_output_ref": None,
                    "raw": None,
                    "adjudicated": None,
                })
        report = analyze_record(no_results)
        self.assertEqual(report["comparable_pair_count"], 0)
        self.assertIsNone(report["matched_preference_flip_rate"])
        self.assertIsNone(report["mean_x_minus_y_order_gap_shift"])
        self.assertIsNone(report["mean_absolute_score_change_by_draft_and_dimension"]["X"]["clarity"])

    def test_missing_order_slot_cannot_be_silently_omitted(self):
        bad = copy.deepcopy(self.fixture)
        bad["cases"][0]["runs"].pop()
        with self.assertRaises(RecordError):
            validate_record(bad)

    def test_not_attempted_requires_null_context_and_evidence(self):
        bad = copy.deepcopy(self.fixture)
        not_attempted = next(run for case in bad["cases"] for run in case["runs"]
                             if run["call_outcome"] == "not_attempted")
        not_attempted["judge_context_id"] = "CTX-SHOULD-NOT-EXIST"
        with self.assertRaises(RecordError):
            validate_record(bad)

    def test_output_digest_and_opaque_reference_must_match_presence(self):
        bad = copy.deepcopy(self.fixture)
        timeout = next(run for case in bad["cases"] for run in case["runs"]
                       if run["call_outcome"] == "timeout")
        timeout["raw_output_ref"] = None
        with self.assertRaises(RecordError):
            validate_record(bad)

    def test_refusal_keeps_raw_artifact_but_no_annotation(self):
        bad = copy.deepcopy(self.fixture)
        refusal = next(run for case in bad["cases"] for run in case["runs"]
                       if run["call_outcome"] == "refusal")
        refusal["raw"] = {"unexpected": "parsed result"}
        with self.assertRaises(RecordError):
            validate_record(bad)

    def test_rejects_order_mutation_and_prompt_rubric_model_drift(self):
        bad_order = copy.deepcopy(self.fixture)
        bad_order["cases"][0]["runs"][1]["order"] = "X_then_Y"
        with self.assertRaises(RecordError):
            validate_record(bad_order)
        bad_model = copy.deepcopy(self.fixture)
        bad_model["cases"][1]["shared_inputs"]["model"]["model_revision"] = "fixture-v2"
        with self.assertRaises(RecordError):
            validate_record(bad_model)

    def test_rejects_history_score_leak_and_bad_annotation(self):
        bad_history = copy.deepcopy(self.fixture)
        bad_history["cases"][0]["r1_preference"] = "X"
        with self.assertRaises(RecordError):
            validate_record(bad_history)
        bad_score = copy.deepcopy(self.fixture)
        good_run = next(run for case in bad_score["cases"] for run in case["runs"]
                        if run["call_outcome"] == "valid_annotation")
        good_run["raw"]["ratings"]["X"][0] = True
        with self.assertRaises(RecordError):
            validate_record(bad_score)

    def test_cli_emits_valid_json_for_v2_synthetic_fixture(self):
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts/sk_e04_order_analysis.py"), "analyze", str(FIXTURE_V2)],
            cwd=ROOT, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        )
        report = json.loads(result.stdout)
        self.assertEqual(report["case_count"], 4)
        self.assertEqual(report["analysis_kind"], "descriptive_only")
        self.assertEqual(report["incomplete_pair_count"], 2)
        self.assertEqual(report["call_outcome_counts"]["timeout"], 1)


if __name__ == "__main__":
    unittest.main()
