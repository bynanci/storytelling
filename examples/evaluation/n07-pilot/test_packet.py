# SPDX-License-Identifier: MIT
"""Integrity of the proposed study packet, not synthetic participant outcomes."""
import csv
import json
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def read(relative):
    return json.loads((HERE / relative).read_text(encoding="utf-8"))


class PacketIntegrity(unittest.TestCase):
    def test_both_arms_start_without_prefilled_corrections(self):
        base = (HERE / "participant-kit/cli/base-document.json").read_bytes()
        self.assertEqual(base, (HERE / "participant-kit/cli/working-document.json").read_bytes())
        before = json.loads(base)
        original = json.loads((ROOT / "tests/fixtures/narrative-v1/n03-authoring-v1.json").read_text())
        self.assertEqual(before["units"], original["units"])
        self.assertEqual(before["document_id"], "n07-door-repair")
        self.assertNotIn("阿菊", base.decode())
        self.assertNotIn("C02", {u["id"] for u in before["units"]})

    def test_corrected_snapshot_preserves_facts_origins_and_exact_revisions(self):
        before = read("participant-kit/cli/base-document.json")
        after = read("facilitator/document-corrected.json")
        self.assertEqual(after["document_revision"], 2)
        old = {u["id"]: u for u in before["units"]}
        new = {u["id"]: u for u in after["units"]}
        for uid in ("S03", "S04"):
            self.assertEqual(old[uid], new[uid])
        for uid in ("S01", "S02"):
            self.assertEqual(new[uid]["text"], old[uid]["text"].replace("阿梅", "阿菊").replace("阿安", "阿哲"))
        for uid in ("I01", "C01"):
            self.assertEqual(old[uid]["text"], new[uid]["text"])
            self.assertEqual(old[uid]["origin"], new[uid]["origin"])
        for uid in ("S01", "S02", "I01", "C01"):
            self.assertEqual(new[uid]["revision"], 2)
        def check(value):
            if isinstance(value, dict):
                if "unit_id" in value and "revision" in value:
                    self.assertEqual(value["revision"], new[value["unit_id"]]["revision"])
                for child in value.values():
                    check(child)
            elif isinstance(value, list):
                for child in value:
                    check(child)
        check(after)
        scoped = [v["id"] for v in after["variants"] if any(r["unit_id"] == "C01" for r in v.get("unit_refs", []))]
        self.assertEqual(scoped, ["illustration-wide"])

    def test_forms_and_report_do_not_invent_human_observations(self):
        for path in (HERE / "forms").glob("*.csv"):
            with path.open(newline="", encoding="utf-8") as stream:
                rows = list(csv.DictReader(stream))
            self.assertEqual(rows, [], path.name)
        status = read("observations.json")
        self.assertEqual(status["available_human_participants"], 0)
        self.assertIsNone(status["protocol_agreement"])
        self.assertEqual(status["decision"], "待真人證據")
        self.assertFalse(status["release_candidate_recommended"])
        self.assertEqual(status["participant_failures"], [])

    def test_batch_guard_is_a_negative_control_not_a_fake_review(self):
        ledger = read("facilitator/ledger-batch-pending.json")
        self.assertEqual([event["kind"] for event in ledger["events"]], ["source_revision", "source_revision"])
        manifest = read("cases.json")
        guard = next(case for case in manifest["cases"] if case["id"] == "n01-atomic-batch-guard")
        self.assertEqual(guard["expected_exit"], 1)
        self.assertFalse(any("--report-v2" in case["args"] for case in manifest["cases"]))


if __name__ == "__main__":
    unittest.main()
