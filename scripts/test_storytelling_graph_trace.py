"""Run the portable orchestration trace checker on declared observations."""
import json
from pathlib import Path
import subprocess
import sys
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "skills/storytelling/scripts/check_trace.py"


def invoke(payload, *args):
    data = payload if isinstance(payload, bytes) else json.dumps(payload).encode()
    result = subprocess.run([sys.executable, str(SCRIPT), *args], input=data,
                            capture_output=True, check=False, timeout=10)
    if result.stderr:
        raise AssertionError(result.stderr.decode())
    return result.returncode, json.loads(result.stdout)


def trace(events, outcome="complete"):
    return {"nodes": ["source", "alternate", "publish"], "edges": [["source", "publish"]],
            "events": events, "outcome": outcome}


class TraceTests(unittest.TestCase):
    def test_accepts_concurrent_independent_starts_and_verified_prerequisite(self):
        events = [["start", "source"], ["start", "alternate"], ["pass", "source"],
                  ["start", "publish"], ["pass", "publish"], ["pass", "alternate"]]
        code, report = invoke(trace(events))
        self.assertEqual(code, 0)
        self.assertTrue(report["ok"])
        self.assertEqual(report["outcome"], "complete")
        self.assertEqual(report["counts"], {"pending": 0, "pass": 3, "fail": 0, "unknown": 0})

    def test_refuses_start_before_pass_for_pending_running_failed_and_unknown(self):
        for prior in [[], [["start", "source"]],
                      [["start", "source"], ["fail", "source"]],
                      [["start", "source"], ["unknown", "source"]]]:
            with self.subTest(prior=prior):
                code, report = invoke(trace(prior + [["start", "publish"]], "partial"))
                self.assertEqual(code, 1)
                self.assertEqual(report["error"], "prerequisite_unverified")
                self.assertEqual((report["event_index"], report["node"], report["prerequisite"]),
                                 (len(prior), "publish", "source"))

    def test_lifecycle_and_terminal_consistency(self):
        for events, outcome, error in [
            ([["pass", "source"]], "partial", "invalid_transition"),
            ([["start", "source"], ["start", "source"]], "partial", "invalid_transition"),
            ([["start", "source"], ["fail", "source"], ["pass", "source"]], "partial", "invalid_transition"),
            ([["start", "source"], ["unknown", "source"], ["start", "source"]], "partial", "invalid_transition"),
            ([["start", "source"]], "partial", "unfinished"),
            ([["start", "source"], ["pass", "source"]], "complete", "outcome_mismatch"),
            ([], "other", "invalid_outcome"),
        ]:
            with self.subTest(events=events, outcome=outcome):
                code, report = invoke(trace(events, outcome))
                self.assertEqual((code, report["error"]), (1, error))
        completed = {"nodes": ["x"], "edges": [], "events": [["start", "x"], ["pass", "x"]],
                     "outcome": "partial"}
        self.assertEqual(invoke(completed)[1]["error"], "outcome_mismatch")

    def test_valid_partial_keeps_ambiguous_outcome_and_missing_work_visible(self):
        code, report = invoke(trace([["start", "source"], ["unknown", "source"],
                                     ["start", "alternate"], ["fail", "alternate"]], "partial"))
        self.assertEqual(code, 0)
        self.assertEqual(report["counts"], {"pending": 1, "pass": 0, "unknown": 1, "fail": 1})
        self.assertEqual(invoke(trace([], "partial"))[1]["counts"]["pending"], 3)

    def test_cycles_and_malformed_declarations(self):
        empty = {"nodes": [], "edges": [], "events": [], "outcome": "complete"}
        self.assertEqual(invoke(empty)[0], 0)
        for body, error in [
            (dict(empty, nodes=["x"], edges=[["x", "x"]], outcome="partial"), "cycle"),
            (dict(empty, events=[["start", "missing"]]), "unknown_node"),
            (dict(empty, events=[["run", "x"]]), "invalid_event"),
            (dict(empty, events=[["start"]]), "invalid_event"),
            (dict(empty, changed=[]), "invalid_shape"),
            (dict(empty, events=None), "invalid_shape"),
            (dict(empty, events=[["start", "x"]] * 8193), "limit_exceeded"),
            (b'{"nodes":[],"edges":[],"events":[],"events":[],"outcome":"complete"}', "duplicate_field"),
            (b' ' * (1024 * 1024 + 1), "input_too_large"),
        ]:
            with self.subTest(error=error):
                code, report = invoke(body)
                self.assertEqual((code, report["error"]), (1, error))
        self.assertEqual(invoke(b'', '--apply'), (2, {"ok": False, "error": "usage"}))


if __name__ == "__main__":
    unittest.main()
