#!/usr/bin/env python3
"""Synthetic regressions for versioned, blinded skill-evaluation records."""
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True

from skill_evaluation_records import (
    RecordError,
    hash_skill_tree,
    prepare_blind_bundle,
    validate_allocation_key,
    validate_blind_packet,
    verify_bundle,
)


ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "docs/research/evaluations/sk-e01/fixtures/synthetic-valid"
R1_STATUS = ROOT / "docs/research/evaluations/sk-e01/r1-provenance-status.json"
SCHEMAS = ROOT / "docs/research/evaluations/sk-e01/schemas"


class SkillEvaluationRecordTests(unittest.TestCase):
    def test_synthetic_fixture_verifies_exact_input_hashes(self):
        record = verify_bundle(FIXTURE)
        self.assertEqual(record["record_kind"], "synthetic_fixture")
        self.assertEqual(record["schema_version"], "skill-evaluation-record/v1")

    def test_skill_tree_digest_is_repeatable_and_byte_sensitive(self):
        with tempfile.TemporaryDirectory() as directory:
            skill = Path(directory) / "skill"
            skill.mkdir()
            (skill / "SKILL.md").write_bytes(b"one\n")
            first = hash_skill_tree(skill)
            self.assertEqual(first, hash_skill_tree(skill))
            (skill / "SKILL.md").write_bytes(b"one\r\n")
            self.assertNotEqual(first, hash_skill_tree(skill))

    def test_skill_tree_digest_rejects_symlinked_directories(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "skill"
            target = Path(directory) / "external"
            root.mkdir()
            target.mkdir()
            (root / "SKILL.md").write_text("local\n", encoding="utf-8")
            (target / "secret.md").write_text("outside\n", encoding="utf-8")
            link = root / "linked-material"
            try:
                link.symlink_to(target, target_is_directory=True)
            except (OSError, NotImplementedError) as error:
                self.skipTest(f"directory symlinks unavailable: {error}")
            with self.assertRaises(RecordError) as caught:
                hash_skill_tree(root)
            self.assertEqual(caught.exception.code, "unsafe_path")

    def test_skill_tree_digest_sorts_relative_posix_paths(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "skill"
            (root / "a").mkdir(parents=True)
            (root / "a" / "x").write_bytes(b"nested\n")
            (root / "a.txt").write_bytes(b"sibling\n")
            digest = hashlib.sha256(b"storytelling-skill-tree/v1\0")
            for name, raw in (("a.txt", b"sibling\n"), ("a/x", b"nested\n")):
                path = name.encode("utf-8")
                digest.update(len(path).to_bytes(8, "big"))
                digest.update(path)
                digest.update(len(raw).to_bytes(8, "big"))
                digest.update(raw)
            self.assertEqual(hash_skill_tree(root), digest.hexdigest())

    def test_hashed_fixture_verifies_in_clean_autocrlf_checkouts(self):
        for mode in ("false", "input", "true"):
            with self.subTest(autocrlf=mode), tempfile.TemporaryDirectory() as directory:
                checkout = Path(directory) / "checkout"
                env = os.environ.copy()
                env["GIT_CONFIG_GLOBAL"] = os.devnull
                env["GIT_CONFIG_SYSTEM"] = os.devnull

                def git(*args):
                    return subprocess.run(
                        ["git", *args], env=env, check=True,
                        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                    )

                git("clone", "--quiet", "--shared", "--no-checkout", str(ROOT), str(checkout))
                git("-C", str(checkout), "config", "core.autocrlf", mode)
                git("-C", str(checkout), "checkout", "--quiet", "--detach", "HEAD")
                verify = subprocess.run(
                    [sys.executable, "scripts/skill_evaluation_records.py", "verify",
                     "docs/research/evaluations/sk-e01/fixtures/synthetic-valid"],
                    cwd=checkout, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                )
                self.assertEqual(verify.returncode, 0, verify.stderr.decode("utf-8", errors="replace"))
                status = git("-C", str(checkout), "status", "--porcelain").stdout
                self.assertEqual(status, b"", "Verification must leave a clean checkout")

    def test_r1_missing_provenance_remains_explicitly_unknown(self):
        status = json.loads(R1_STATUS.read_text(encoding="utf-8"))
        for key in (
            "per_draft_generation_manifest",
            "per_draft_condition",
            "per_draft_prompt_bytes_and_sha256",
            "per_draft_evidence_packet_bytes_and_sha256",
            "per_draft_model_runtime_version_and_settings",
            "per_draft_skill_commit_and_sha256",
            "per_draft_generation_timestamp",
        ):
            self.assertEqual(status[key], {"status": "unknown", "value": None})
        self.assertEqual(status["published_group_assignments"]["status"], "preserved_as_declared")
        self.assertEqual(status["reconstruction_from_style_or_published_group"], "prohibited")

    def test_versioned_schemas_are_well_formed_and_closed(self):
        schemas = sorted(SCHEMAS.glob("*.schema.json"))
        self.assertEqual(len(schemas), 3)
        for path in schemas:
            with self.subTest(path=path.name):
                schema = json.loads(path.read_text(encoding="utf-8"))
                self.assertEqual(schema["$schema"], "https://json-schema.org/draft/2020-12/schema")
                self.assertFalse(schema["additionalProperties"])

    def test_changed_text_bytes_fail_the_recorded_digest(self):
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / "bundle"
            shutil.copytree(FIXTURE, copied)
            record = json.loads((copied / "record.json").read_text(encoding="utf-8"))
            text_path = copied / record["cases"][0]["generations"][0]["text"]["path"]
            text_path.write_bytes(text_path.read_bytes() + b" changed")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "hash_mismatch")

    def test_paired_generations_must_share_exact_prompt_and_packet_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / "bundle"
            shutil.copytree(FIXTURE, copied)
            alternate = copied / "inputs/alternate-prompt.txt"
            alternate.write_text("A different prompt.\n", encoding="utf-8")
            record_path = copied / "record.json"
            record = json.loads(record_path.read_text(encoding="utf-8"))
            raw = alternate.read_bytes()
            record["cases"][0]["generations"][1]["prompt"] = {
                "path": "inputs/alternate-prompt.txt",
                "sha256": hashlib.sha256(raw).hexdigest(),
            }
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "case_input_mismatch")

    def test_duplicate_neutral_ids_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / "bundle"
            shutil.copytree(FIXTURE, copied)
            record_path = copied / "record.json"
            record = json.loads(record_path.read_text(encoding="utf-8"))
            generations = record["cases"][0]["generations"]
            generations[1]["text_id"] = generations[0]["text_id"]
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "duplicate_text_id")

    def test_unknown_condition_cannot_be_inferred_from_a_declared_label(self):
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / "bundle"
            shutil.copytree(FIXTURE, copied)
            record_path = copied / "record.json"
            record = json.loads(record_path.read_text(encoding="utf-8"))
            provenance = record["cases"][0]["generations"][0]["condition_provenance"]
            provenance["status"] = "unknown"
            provenance["condition"] = "skill_on"
            provenance["evidence_ref"] = None
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "condition_inferred")

    def test_ratings_cannot_be_added_to_generation_record(self):
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / "bundle"
            shutil.copytree(FIXTURE, copied)
            record_path = copied / "record.json"
            record = json.loads(record_path.read_text(encoding="utf-8"))
            record["cases"][0]["generations"][0]["scores"] = {"accuracy": 5}
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "unknown_field")

    def test_observed_skill_on_requires_commit_and_package_hash(self):
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / "bundle"
            shutil.copytree(FIXTURE, copied)
            record_path = copied / "record.json"
            record = json.loads(record_path.read_text(encoding="utf-8"))
            record["record_kind"] = "observed_evaluation"
            for generation in record["cases"][0]["generations"]:
                generation["condition_provenance"]["status"] = "observed"
                generation["generated_at"] = "2026-11-13T00:00:00Z"
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "missing_skill_identity")

            record["cases"][0]["generations"][0]["skill"] = {
                "git_commit": "a" * 40,
                "sha256": "b" * 64,
            }
            record_path.write_text(json.dumps(record), encoding="utf-8")
            self.assertEqual(verify_bundle(copied)["record_kind"], "observed_evaluation")

            record["cases"][0]["generations"][0]["condition_provenance"]["evidence_ref"] = "/Users/private/run.log"
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "invalid_id")

            record["cases"][0]["generations"][0]["condition_provenance"]["evidence_ref"] = "E-RUN-01"
            record["cases"][0]["generations"][0]["model_runtime"]["settings"] = {"max_tokens": 512}
            record_path.write_text(json.dumps(record), encoding="utf-8")
            self.assertEqual(verify_bundle(copied)["record_kind"], "observed_evaluation")

            record["cases"][0]["generations"][0]["model_runtime"]["settings"] = {"api_key": "never-store"}
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "sensitive_setting")

    def test_blind_packet_and_private_allocation_key_are_separate(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            judge_dir = root / "judge"
            key_dir = root / "private"
            judge_dir.mkdir()
            key_dir.mkdir()
            packet, key = prepare_blind_bundle(
                FIXTURE, judge_dir / "packet.json", key_dir / "allocation.json", seed=1
            )
            self.assertTrue((judge_dir / "packet.json").is_file())
            self.assertTrue((key_dir / "allocation.json").is_file())
            self.assertNotEqual((judge_dir / "packet.json").parent,
                                (key_dir / "allocation.json").parent)
            validate_blind_packet(packet)
            validate_allocation_key(verify_bundle(FIXTURE), key)
            self.assertEqual(len(packet["cases"][0]["drafts"]), 2)
            self.assertEqual({item["condition"] for item in key["allocations"]},
                             {"skill_on", "skill_off"})
            self.assertEqual(packet["cases"][0]["drafts"][0]["text_id"], "T-M2F8WC")

    def test_nested_allocation_directory_cannot_be_shared_with_judges(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            judge_dir = root / "judge"
            private_dir = judge_dir / "private"
            private_dir.mkdir(parents=True)
            with self.assertRaises(RecordError) as caught:
                prepare_blind_bundle(
                    FIXTURE, judge_dir / "packet.json", private_dir / "allocation.json", seed=1
                )
            self.assertEqual(caught.exception.code, "key_not_separated")

    def test_allocation_key_id_mismatch_is_rejected(self):
        record = verify_bundle(FIXTURE)
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            judge_dir = root / "judge"
            key_dir = root / "private"
            judge_dir.mkdir()
            key_dir.mkdir()
            _, key = prepare_blind_bundle(
                FIXTURE, judge_dir / "packet.json", key_dir / "allocation.json", seed=1
            )
            key["allocations"][0]["generation_id"] = "generation-unknown"
            with self.assertRaises(RecordError) as caught:
                validate_allocation_key(record, key)
            self.assertEqual(caught.exception.code, "allocation_mismatch")

            _, key = prepare_blind_bundle(
                FIXTURE, judge_dir / "packet-2.json", key_dir / "allocation-2.json", seed=1
            )
            key["allocations"][0]["presentation_position"] = 1
            key["allocations"][1]["presentation_position"] = 2
            with self.assertRaises(RecordError) as caught:
                validate_allocation_key(record, key)
            self.assertEqual(caught.exception.code, "allocation_mismatch")

    def test_blind_packet_rejects_condition_and_prior_score_fields(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            judge_dir = root / "judge"
            key_dir = root / "private"
            judge_dir.mkdir()
            key_dir.mkdir()
            packet, _ = prepare_blind_bundle(
                FIXTURE, judge_dir / "packet.json", key_dir / "allocation.json", seed=1
            )
        packet["cases"][0]["condition"] = "skill_on"
        with self.assertRaises(RecordError) as caught:
            validate_blind_packet(packet)
        self.assertEqual(caught.exception.code, "allocation_leak")

        del packet["cases"][0]["condition"]
        packet["cases"][0]["prior_scores"] = {"T-7K4Q9D": 5}
        with self.assertRaises(RecordError) as caught:
            validate_blind_packet(packet)
        self.assertEqual(caught.exception.code, "score_leak")

    def test_absolute_and_parent_traversal_paths_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / "bundle"
            shutil.copytree(FIXTURE, copied)
            record_path = copied / "record.json"
            record = json.loads(record_path.read_text(encoding="utf-8"))
            record["cases"][0]["generations"][0]["text"]["path"] = "../outside.txt"
            record_path.write_text(json.dumps(record), encoding="utf-8")
            with self.assertRaises(RecordError) as caught:
                verify_bundle(copied)
            self.assertEqual(caught.exception.code, "unsafe_path")


if __name__ == "__main__":
    unittest.main()
