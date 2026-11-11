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
