"""Exercise the installed helper through stdin/stdout. All graph IDs are synthetic."""
from itertools import permutations
import json
from pathlib import Path
import runpy
import subprocess
import sys
import unittest

HELPER = (Path(__file__).resolve().parents[1] / ".agents" / "skills" /
          "storytelling" / "scripts" / "graph_plan.py")


def invoke(payload, *args):
    data = payload if isinstance(payload, bytes) else json.dumps(payload).encode()
    process = subprocess.run([sys.executable, str(HELPER), *args], input=data,
                             capture_output=True, check=False, timeout=10)
    if process.stderr:
        raise AssertionError(process.stderr.decode())
    return process.returncode, json.loads(process.stdout)


class GraphPlanTests(unittest.TestCase):
    def test_fifo_not_global_minimum_and_only_affected_descendants(self):
        graph = {"nodes": ["after", "first", "other", "check", "unused"],
                 "edges": [["first", "after"], ["after", "check"], ["other", "check"]]}
        status, full = invoke(graph)
        self.assertEqual(status, 0)
        self.assertEqual(full["order"], ["first", "other", "unused", "after", "check"])
        self.assertEqual(full["affected"], graph["nodes"])
        status, partial = invoke(dict(graph, changed=["first"]))
        self.assertEqual((status, partial),
                         (0, {"ok": True, "order": ["first", "after", "check"],
                              "affected": ["after", "first", "check"]}))
        self.assertEqual(invoke(dict(graph, changed=[])),
                         (0, {"ok": True, "order": [], "affected": []}))

    def test_edge_order_breaks_tie_for_newly_ready_nodes(self):
        graph = {"nodes": ["root", "A", "B"], "edges": [["root", "B"], ["root", "A"]]}
        self.assertEqual(invoke(graph)[1]["order"], ["root", "B", "A"])

    def test_rejects_cycle_outside_selected_subgraph_with_real_closed_witness(self):
        graph = {"nodes": ["tail", "unaffected", "a", "b", "c"],
                 "edges": [["a", "b"], ["b", "c"], ["c", "a"], ["a", "tail"]],
                 "changed": ["unaffected"]}
        status, failure = invoke(graph)
        self.assertEqual(status, 1)
        self.assertEqual(failure["error"], "cycle")
        self.assertNotIn("order", failure)
        cycle = failure["cycle"]
        self.assertEqual(cycle[0], cycle[-1])
        self.assertEqual(set(cycle), {"a", "b", "c"})
        self.assertTrue(all([a, b] in graph["edges"] for a, b in zip(cycle, cycle[1:])))
        self.assertEqual(invoke({"nodes": ["x"], "edges": [["x", "x"]]})[1]["cycle"], ["x", "x"])

    def test_strict_wire_and_bounds(self):
        for payload, error in [
            (b'{', 'invalid_json'),
            (b'\xff', 'invalid_json'),
            (b'{"nodes":[],"nodes":[],"edges":[]}', 'duplicate_field'),
            ({"nodes": ["a"], "edges": [], "extra": True}, 'invalid_shape'),
            ({"nodes": ["a", "a"], "edges": []}, 'duplicate_node'),
            ({"nodes": ["a"], "edges": [["a", "b"]]}, 'unknown_node'),
            ({"nodes": ["a"], "edges": [[{}, "a"]]}, 'invalid_id'),
            ({"nodes": ["a", "b"], "edges": [["a", "b"], ["a", "b"]]}, 'duplicate_edge'),
            ({"nodes": ["a"], "edges": [], "changed": ["b"]}, 'unknown_node'),
            ({"nodes": ["a"], "edges": [], "changed": ["a", "a"]}, 'duplicate_change'),
            ({"nodes": ["a" * 129], "edges": []}, 'invalid_id'),
            ({"nodes": [f"n{i}" for i in range(4097)], "edges": []}, 'limit_exceeded'),
            ({"nodes": ["a"], "edges": [["a", "a"]] * 16385}, 'limit_exceeded'),
            (b' ' * (1024 * 1024 + 1), 'input_too_large'),
        ]:
            with self.subTest(error=error):
                status, failure = invoke(payload)
                self.assertNotEqual(status, 0)
                self.assertEqual(failure, {"ok": False, "error": error})
        self.assertEqual(invoke(b'', '--run'), (2, {"ok": False, "error": "usage"}))

    def test_every_three_node_graph_matches_permutation_and_closure_oracles(self):
        # Exercise all 64 directed graphs without self-edges. The reference uses
        # brute-force permutations and repeated set expansion, independent of Kahn.
        helper = runpy.run_path(str(HELPER))
        nodes = ["a", "b", "c"]
        pairs = list(permutations(nodes, 2))
        for mask in range(1 << len(pairs)):
            edges = [list(pair) for i, pair in enumerate(pairs) if mask & (1 << i)]
            schedules = [seq for seq in permutations(nodes)
                         if all(seq.index(a) < seq.index(b) for a, b in edges)]
            graph = {"nodes": nodes, "edges": edges}
            with self.subTest(mask=mask):
                if not schedules:
                    with self.assertRaises(helper["GraphError"]) as caught:
                        helper["plan"](dict(graph, changed=[]))
                    self.assertEqual(caught.exception.code, "cycle")
                    cycle = caught.exception.cycle
                    self.assertEqual(cycle[0], cycle[-1])
                    self.assertTrue(all([a, b] in edges for a, b in zip(cycle, cycle[1:])))
                    continue
                result = helper["plan"](graph)
                self.assertIn(tuple(result["order"]), schedules)
                for seed in nodes:
                    reachable = {seed}
                    for _ in nodes:
                        reachable |= {b for a, b in edges if a in reachable}
                    partial = helper["plan"](dict(graph, changed=[seed]))
                    self.assertEqual(partial["affected"], [n for n in nodes if n in reachable])
                    self.assertEqual(partial["order"], [n for n in result["order"] if n in reachable])

    def test_empty_and_maximum_length_chain(self):
        self.assertEqual(invoke({"nodes": [], "edges": []}),
                         (0, {"ok": True, "order": [], "affected": []}))
        nodes = [f"n{i}" for i in range(4096)]
        edges = list(zip(nodes, nodes[1:]))
        status, result = invoke({"nodes": nodes, "edges": edges, "changed": nodes[-2:-1]})
        self.assertEqual((status, result["order"], result["affected"]), (0, nodes[-2:], nodes[-2:]))


if __name__ == "__main__":
    unittest.main()
