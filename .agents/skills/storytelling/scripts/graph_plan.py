"""Plan declared hard prerequisites. Read UTF-8 JSON on stdin and emit one JSON report.

Input: nodes (unique IDs), edges ([before, after] pairs), optionally changed (unique IDs).
Only include hard prerequisites. A changed run assumes all unaffected inputs are current.
The entire DAG is validated on every call. No tool is executed and no graph is stored.
"""
from collections import deque
import json
import re
import sys

MAX_BYTES = 1 << 20
MAX_NODES = 4096
MAX_EDGES = 16384
ID = re.compile(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,127}")


class GraphError(ValueError):
    def __init__(self, code, cycle=None):
        super().__init__(code)
        self.code = code
        self.cycle = cycle


def unique_fields(pairs):
    fields = {}
    for key, value in pairs:
        if key in fields:
            raise GraphError("duplicate_field")
        fields[key] = value
    return fields


def reject_constant(_):
    raise GraphError("invalid_json")


def check_id(value):
    if not isinstance(value, str) or not ID.fullmatch(value):
        raise GraphError("invalid_id")


def check_list(value, limit):
    if not isinstance(value, list):
        raise GraphError("invalid_shape")
    if len(value) > limit:
        raise GraphError("limit_exceeded")


def cycle_witness(successors, indegrees):
    """Each blocked node has a blocked predecessor. Follow these back to a cycle."""
    predecessors = {}
    for node in successors:
        if indegrees[node]:
            for child in successors[node]:
                if indegrees[child]:
                    predecessors[child] = node
    node = next(iter(predecessors))
    positions = {}
    path = []
    while node not in positions:
        positions[node] = len(path)
        path.append(node)
        node = predecessors[node]
    # The predecessor walk is backward; report real directed edges in forward order.
    return [node, *reversed(path[positions[node] + 1:]), node]


def plan(doc):
    if not isinstance(doc, dict) or set(doc) not in (
        {"nodes", "edges"}, {"nodes", "edges", "changed"}
    ):
        raise GraphError("invalid_shape")
    nodes = doc["nodes"]
    edges = doc["edges"]
    check_list(nodes, MAX_NODES)
    check_list(edges, MAX_EDGES)
    successors = {}
    for node in nodes:
        check_id(node)
        if node in successors:
            raise GraphError("duplicate_node")
        # Dictionary keys keep first declaration order and support duplicate checks.
        successors[node] = {}
    indegrees = dict.fromkeys(nodes, 0)
    for edge in edges:
        if not isinstance(edge, list) or len(edge) != 2:
            raise GraphError("invalid_shape")
        before, after = edge
        check_id(before)
        check_id(after)
        if before not in successors or after not in successors:
            raise GraphError("unknown_node")
        if after in successors[before]:
            raise GraphError("duplicate_edge")
        successors[before][after] = None
        indegrees[after] += 1

    if "changed" in doc:
        changed = doc["changed"]
        check_list(changed, MAX_NODES)
        affected = set()
        for node in changed:
            check_id(node)
            if node not in successors:
                raise GraphError("unknown_node")
            if node in affected:
                raise GraphError("duplicate_change")
            affected.add(node)
    else:
        affected = set(nodes)

    ready = deque(node for node in nodes if not indegrees[node])
    order = []
    visited = 0
    while ready:
        node = ready.popleft()
        visited += 1
        recompute = node in affected
        if recompute:
            order.append(node)
        for child in successors[node]:
            if recompute:
                affected.add(child)
            indegrees[child] -= 1
            if not indegrees[child]:
                ready.append(child)
    if visited != len(nodes):
        raise GraphError("cycle", cycle_witness(successors, indegrees))
    return {"ok": True, "order": order, "affected": [node for node in nodes if node in affected]}


def main():
    if len(sys.argv) != 1:
        print('{"ok":false,"error":"usage"}')
        return 2
    try:
        data = sys.stdin.buffer.read(MAX_BYTES + 1)
        if len(data) > MAX_BYTES:
            raise GraphError("input_too_large")
        doc = json.loads(data.decode("utf-8"), object_pairs_hook=unique_fields,
                         parse_constant=reject_constant)
        report = plan(doc)
    except GraphError as exc:
        report = {"ok": False, "error": exc.code}
        if exc.cycle:
            report["cycle"] = exc.cycle
    except (ValueError, RecursionError):
        report = {"ok": False, "error": "invalid_json"}
    print(json.dumps(report, separators=(",", ":")))
    return 0 if report["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
