"""Check published source maps, correction keys and independent reader materials."""
import csv
import re
import unittest
from pathlib import Path

EVALUATION = Path(__file__).resolve().parent
EXAMPLES = EVALUATION.parent


class CatalogTests(unittest.TestCase):
    def test_each_source_has_a_separate_handoff_boundary_in_both_languages(self):
        for suffix in (".md", ".zh-TW.md"):
            boundaries = (EVALUATION / ("source-boundaries" + suffix)).read_text(encoding="utf-8")
            sections = re.split(r'^<a name="([a-z-]+)"></a>$', boundaries, flags=re.M)
            folders = {p.name for p in (EXAMPLES / "studies").iterdir() if p.is_dir()}
            self.assertCountEqual(sections[1::2], folders)
            for case, body in zip(sections[1::2], sections[2::2], strict=True):
                with self.subTest(case=case, language=suffix):
                    original = (EXAMPLES / "studies" / case / ("README" + suffix)).read_text(encoding="utf-8")
                    expected = re.findall(r"^\| ([SP][0-9]+) \|", original, re.M)
                    rows = re.findall(r"^\| `?([SP][0-9]+)`? \| ([^|\n]+) \| ([^|\n]+) \|$", body, re.M)
                    self.assertTrue(expected)
                    self.assertCountEqual([row[0] for row in rows], expected)
                    for _, supported, limit in rows:
                        self.assertTrue(supported.strip() and limit.strip())
                        self.assertNotRegex(supported + limit, r"\bR[12]\b")

    def test_source_maps_and_scorer_agree_in_both_languages(self):
        folders = {p.name for p in (EXAMPLES / "studies").iterdir() if p.is_dir()}
        with (EVALUATION / "expected.csv").open(encoding="utf-8", newline="") as file:
            rows = list(csv.DictReader(file))
        gold = {(r["case"], r["card"]): set(r["expected"].split()) for r in rows}
        self.assertEqual(len(gold), len(rows))
        self.assertEqual(set(gold), {(case, card) for case in folders for card in ("R1", "R2")})
        for suffix in (".md", ".zh-TW.md"):
            index = (EXAMPLES / ("README" + suffix)).read_text(encoding="utf-8")
            key = (EVALUATION / ("keys" + suffix)).read_text(encoding="utf-8")
            sections = re.split(r'^<a name="([a-z-]+)"></a>$', key, flags=re.M)
            self.assertCountEqual(sections[1::2], folders)
            for case, body in zip(sections[1::2], sections[2::2], strict=True):
                with self.subTest(case=case, language=suffix):
                    self.assertIn(f"studies/{case}/README{suffix}", index)
                    refs = {draft: set(values.split(", ")) for draft, values in
                            re.findall(r"^\| ([ABC]) \| ([A-Z0-9, ]+) \|$", body, re.M)}
                    self.assertEqual(set(refs), set("ABC"))
                    cards = re.split(r"^### (R[12]) · ([A-Z][0-9]+)$", body, flags=re.M)
                    self.assertCountEqual(cards[1::3], ["R1", "R2"])
                    for card, source, answers in zip(cards[1::3], cards[2::3], cards[3::3], strict=True):
                        expected = gold[case, card]
                        self.assertEqual({d for d in refs if source in refs[d]}, expected)
                        self.assertEqual(set(re.findall(r"^- \*\*([ABC])\*\*", answers, re.M)), expected)
                        self.assertIn(len(expected), (2, 3) if card == "R1" else (1,))
                    folder = EXAMPLES / "studies" / case
                    a, b = ((folder / (draft + suffix)).read_text(encoding="utf-8") for draft in "AB")
                    self.assertTrue(a.strip() and b.strip() and a != b)
                    for excerpt in (a, b):
                        self.assertNotRegex(excerpt, r"(?m)^#|\]\([^)]*\)|\b[SPR][12]\b")


if __name__ == "__main__":
    unittest.main()
