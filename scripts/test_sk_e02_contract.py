#!/usr/bin/env python3
"""Synthetic regressions for the SK-E02 evidence-plan and errata contract."""
import copy
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from sk_e02_contract import (  # noqa: E402
    ContractError,
    validate_adjudication,
    validate_errata,
    validate_evidence_plan,
    verify_fixture_hashes,
)


FIXTURES = ROOT / "docs/research/evaluations/sk-e02/fixtures"


def load(name):
    return json.loads((FIXTURES / name).read_text(encoding="utf-8"))


def load_negative(name):
    return json.loads((FIXTURES / "negative" / name).read_text(encoding="utf-8"))


class EvidencePlanTests(unittest.TestCase):
    def test_prospective_fixture_is_frozen_before_variant_generation(self):
        plan = load("prospective-evidence-plan.json")
        self.assertEqual(validate_evidence_plan(plan), plan)
        self.assertEqual(plan["record_kind"], "synthetic_fixture")
        self.assertIsNone(plan["freeze"]["variant_generation_started_at"])
        self.assertTrue(all(c["requiredness"] in {"required", "optional"} for c in plan["claims"]))
        self.assertEqual(verify_fixture_hashes(plan), [])

    def test_hashed_fixtures_survive_clean_autocrlf_checkouts(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "source"
            source.mkdir()
            (source / "scripts").mkdir()
            audit_dir = source / "docs/research/evaluations/2026-11-13-gemini-r1"
            audit_dir.mkdir(parents=True)
            shutil.copy2(ROOT / ".gitattributes", source / ".gitattributes")
            shutil.copy2(ROOT / "scripts/sk_e02_contract.py", source / "scripts/sk_e02_contract.py")
            shutil.copy2(
                ROOT / "docs/research/evaluations/2026-11-13-gemini-r1/source_audit_R1.csv",
                audit_dir / "source_audit_R1.csv",
            )
            shutil.copytree(ROOT / "docs/research/evaluations/sk-e02",
                            source / "docs/research/evaluations/sk-e02")
            env = os.environ.copy()
            env["GIT_CONFIG_GLOBAL"] = os.devnull
            env["GIT_CONFIG_SYSTEM"] = os.devnull

            def git(*args):
                return subprocess.run(
                    ["git", *args], env=env, check=True,
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                )

            git("init", "--quiet", str(source))
            git("-C", str(source), "config", "core.autocrlf", "false")
            git("-C", str(source), "config", "user.name", "SK-E02 test")
            git("-C", str(source), "config", "user.email", "sk-e02-test@example.invalid")
            git("-C", str(source), "add", "--", ".gitattributes", "scripts/sk_e02_contract.py",
                "docs/research/evaluations/sk-e02", "docs/research/evaluations/2026-11-13-gemini-r1/source_audit_R1.csv")
            git("-C", str(source), "commit", "--quiet", "-m", "synthetic SK-E02 EOL fixture")
            for mode in ("false", "input", "true"):
                with self.subTest(autocrlf=mode):
                    checkout = root / f"checkout-{mode}"
                    git("clone", "--quiet", "--shared", "--no-checkout", str(source), str(checkout))
                    git("-C", str(checkout), "config", "core.autocrlf", mode)
                    git("-C", str(checkout), "checkout", "--quiet", "--detach", "HEAD")
                    check = subprocess.run(
                        [sys.executable, "scripts/sk_e02_contract.py", "check"],
                        cwd=checkout, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                    )
                    self.assertEqual(check.returncode, 0, check.stderr.decode("utf-8", errors="replace"))
                    status = git("-C", str(checkout), "status", "--porcelain").stdout
                    self.assertEqual(status, b"", "Verification must leave a clean checkout")

    def test_evidence_mode_is_one_frozen_string_not_a_combination(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["evidence_mode"] = ["packet_only", "packet_plus_archived_sources"]
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)

    def test_unsupported_evidence_mode_is_rejected(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["evidence_mode"] = "live_web_search"
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)

    def test_committed_invalid_mode_example_is_rejected(self):
        with self.assertRaises(ContractError):
            validate_evidence_plan(load_negative("invalid-multiple-evidence-modes.json"))

    def test_variant_generation_must_follow_claim_freeze(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["freeze"]["variant_generation_started_at"] = plan["freeze"]["claims_frozen_at"]
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)

    def test_committed_invalid_freeze_example_is_rejected(self):
        with self.assertRaises(ContractError):
            validate_evidence_plan(load_negative("invalid-generation-before-freeze.json"))

    def test_unknown_requiredness_is_allowed_only_for_retrospective_records(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["claims"][0]["requiredness"] = "unknown"
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)

    def test_retrospective_records_cannot_backfill_requiredness(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["phase"] = "retrospective"
        plan["record_kind"] = "retrospective_record"
        plan["packet"]["sha256"] = None
        plan["freeze"] = {"claims_frozen_at": None, "variant_generation_started_at": None}
        for claim in plan["claims"]:
            claim["requiredness"] = "unknown"
        self.assertEqual(validate_evidence_plan(plan), plan)
        plan["claims"][0]["requiredness"] = "required"
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)

    def test_claim_source_ids_and_revisions_must_resolve(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["claims"][0]["source_refs"][0]["source_id"] = "S-MISSING"
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)

    def test_packet_only_mode_cannot_smuggle_archived_source_passages(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["evidence_mode"] = "packet_only"
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)

    def test_packet_only_plan_keeps_external_source_access_out_of_scope(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        plan["evidence_mode"] = "packet_only"
        for source in plan["sources"]:
            source.update({
                "access_status": "not_attempted",
                "retrieved_at": None,
                "http_status": None,
                "content_type": None,
                "response_sha256": None,
                "archive_ref": None,
                "archived_passage_location": None,
            })
        for claim in plan["claims"]:
            for ref in claim["source_refs"]:
                ref["archived_source_location"] = None
        self.assertEqual(validate_evidence_plan(plan), plan)

    def test_unavailable_source_has_no_archive_hash_or_passage(self):
        plan = copy.deepcopy(load("prospective-evidence-plan.json"))
        source = next(s for s in plan["sources"] if s["source_id"] == "S-UNAVAILABLE")
        source["response_sha256"] = "a" * 64
        with self.assertRaises(ContractError):
            validate_evidence_plan(plan)


class AdjudicationTests(unittest.TestCase):
    def test_w4_support_states_and_scope_annotations_are_separate(self):
        record = load("synthetic-adjudication.json")
        self.assertEqual(validate_adjudication(record), record)
        self.assertEqual(
            {item["support_state"] for item in record["claim_checks"]},
            {"supported", "contradicted", "insufficient", "unavailable"},
        )
        self.assertIn("packet_scope_violations", record)
        self.assertIn("packet_source_mismatches", record)

    def test_failed_source_access_cannot_be_relabelled_contradicted(self):
        record = copy.deepcopy(load("synthetic-adjudication.json"))
        check = next(c for c in record["claim_checks"] if c["claim_id"] == "CL-UNAVAILABLE")
        check["support_state"] = "contradicted"
        with self.assertRaises(ContractError):
            validate_adjudication(record)

    def test_committed_unavailable_contradiction_example_is_rejected(self):
        with self.assertRaises(ContractError):
            validate_adjudication(load_negative("invalid-unavailable-as-contradicted.json"))

    def test_unknown_or_new_support_state_is_rejected(self):
        record = copy.deepcopy(load("synthetic-adjudication.json"))
        record["claim_checks"][0]["support_state"] = "fabricated"
        with self.assertRaises(ContractError):
            validate_adjudication(record)


class ErrataTests(unittest.TestCase):
    def test_c13_c28_are_versioned_without_rewriting_r1_outcomes(self):
        record = load("r1-packet-errata.json")
        self.assertEqual(validate_errata(record), record)
        c13, c28 = record["errata"]
        self.assertEqual((c13["original_fragment"], c13["corrected_fragment"]), ("鄭博宇", "鄭博隅"))
        self.assertEqual(c28["corrected_fragment"], "at least 12 new state legislative chambers")
        self.assertEqual(c28["original_fragment_fidelity"], "paraphrase_from_public_R1_audit")
        self.assertEqual(c28["source_review"]["support_state"], "unavailable")
        self.assertFalse(record["mutates_r1_artifacts"])
        self.assertEqual(record["r1_exact_packet_bytes"], {"status": "unavailable", "value": None})

    def test_errata_cannot_include_replacement_scores_assignments_or_flags(self):
        record = copy.deepcopy(load("r1-packet-errata.json"))
        record["scores"] = {"C13": 5}
        with self.assertRaises(ContractError):
            validate_errata(record)

    def test_committed_r1_mutation_example_is_rejected(self):
        with self.assertRaises(ContractError):
            validate_errata(load_negative("invalid-r1-score-rewrite.json"))

    def test_exact_original_packet_text_cannot_be_invented(self):
        record = copy.deepcopy(load("r1-packet-errata.json"))
        record["errata"][1]["original_fragment_fidelity"] = "exact_packet_text"
        with self.assertRaises(ContractError):
            validate_errata(record)


if __name__ == "__main__":
    unittest.main()
