"""Regression checks for missed corrections, collateral damage and partial delivery."""
import csv
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from score_revisions import FIELDS, RESULT, read_scores


class ScoreTests(unittest.TestCase):
    def scores(self, *rows):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "expected.csv").write_text("case,card,expected\ngate-repair,R1,A B\n")
            path = root / "observations.csv"
            with path.open("w", encoding="utf-8", newline="") as handle:
                writer = csv.writer(handle)
                writer.writerow(FIELDS)
                writer.writerows(rows)
            with patch("score_revisions.HERE", root):
                return [dict(zip(RESULT, score, strict=True)) for score in read_scores(path)]

    def row(self, changed="A B", corrected="A B", collateral="", finished="yes", received="A B C"):
        return ("p01", "gate-repair", "en", "notes", "R1", received,
                changed, corrected, collateral, "37", finished)

    def test_empty_form_and_exact_revision(self):
        self.assertEqual(self.scores(), [])
        score = self.scores(self.row())[0]
        self.assertEqual(score["exact"], "yes")
        self.assertEqual(score["expected"], "A B")
        self.assertEqual(score["received_unaffected"], "C")

    def test_missed_extra_and_collateral_in_expected_draft(self):
        score = self.scores(self.row("A C", "A", "A"))[0]
        self.assertEqual((score["missed"], score["unnecessary"], score["damaged"], score["exact"]),
                         ("B", "C", "A", "no"))

    def test_published_account_review_calibration(self):
        fixture = Path(__file__).resolve().parents[1] / "studies/account-review/calibration-revisions.csv"
        scores = [dict(zip(RESULT, score, strict=True)) for score in read_scores(fixture)]
        self.assertEqual(
            [(s["participant"], s["missed"], s["unnecessary"], s["damaged"], s["exact"]) for s in scores],
            [("syn-1", "", "", "", "yes"), ("syn-2", "A B", "C", "", "no"),
             ("syn-3", "", "", "B", "no"), ("syn-4", "", "", "", "yes"),
             ("syn-5", "", "", "", "yes"), ("syn-6", "", "A", "", "no")],
        )

    def test_bilingual_calibration_tables_match_the_coded_returns(self):
        folder = Path(__file__).resolve().parents[1] / "studies/account-review"
        fields = ("changed", "corrected", "collateral", "missed", "unnecessary", "damaged", "exact")
        fixture = folder / "calibration-revisions.csv"
        with fixture.open(encoding="utf-8", newline="") as file:
            inputs = list(csv.DictReader(file))
        expected = []
        for input_row, score in zip(inputs, read_scores(fixture), strict=True):
            result = input_row | dict(zip(RESULT, score, strict=True))
            expected.append([result[field] or "—" for field in fields])
        for name in ("calibration.md", "calibration.zh-TW.md"):
            text = (folder / name).read_text(encoding="utf-8")
            table_rows = [line.split("|")[2:-1] for line in text.splitlines()
                          if line.startswith(("| R1", "| R2"))]
            with self.subTest(name=name):
                self.assertEqual([[cell.strip() for cell in row] for row in table_rows], expected)

    def test_material_condition_does_not_change_the_scoring_rule(self):
        row = self.row("A C", "A", "A")
        attempts = [(*row[:3], condition, *row[4:])
                    for condition in ("notes", "mapped", "mapped-bounds")]
        scores = self.scores(*attempts)
        self.assertEqual([score["condition"] for score in scores],
                         ["notes", "mapped", "mapped-bounds"])
        for score in scores:
            self.assertEqual((score["missed"], score["unnecessary"], score["damaged"]),
                             ("B", "C", "A"))
        with self.assertRaises(ValueError):
            self.scores((*row[:3], "unrecognized", *row[4:]))

    def test_timeout_preserves_observation(self):
        score = self.scores(self.row(finished="no"))[0]
        self.assertEqual((score["exact"], score["seconds"], score["finished"]), ("no", "37", "no"))

    def test_partial_return_does_not_count_unseen_drafts_as_undamaged(self):
        score = self.scores(self.row("A", "A", received="A", finished="no"))[0]
        self.assertEqual((score["received"], score["unreturned"], score["missed"]), ("A", "B C", "B"))
        self.assertEqual((score["received_unaffected"], score["unnecessary"], score["exact"]), ("", "", "no"))

    def test_zero_returns_still_count_as_an_attempt(self):
        score = self.scores(self.row("", "", received="", finished="no"))[0]
        self.assertEqual((score["missed"], score["received"], score["unreturned"]), ("A B", "", "A B C"))

    def test_rejects_bad_subset_duplicate_unknown_id_and_incomplete_delivery(self):
        for rows in [(self.row("A", "B"),), (self.row(), self.row()),
                     (self.row("A X", "A"),), (self.row("A A", "A"),),
                     (self.row(received="A B"),), (self.row(received="A", finished="no"),),
                     (self.row("A", "A", "C", finished="no", received="A"),)]:
            with self.subTest(rows=rows), self.assertRaises(ValueError):
                self.scores(*rows)


if __name__ == "__main__":
    unittest.main()
