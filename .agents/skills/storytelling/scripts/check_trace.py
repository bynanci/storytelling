"""Validate an observed dependency trace from stdin. Events are caller assertions.

A pass records a completed validation. This tool checks ordering and consistency,
without executing operations or independently verifying their results.
"""
import json
import sys

# Loading the bundled helper must not write a cache into an installed skill.
sys.dont_write_bytecode = True
from graph_plan import (GraphError, MAX_BYTES, MAX_NODES, check_id, check_list,
                        plan, reject_constant, unique_fields)

TERMINALS = ("pass", "fail", "unknown")
MAX_EVENTS = 2 * MAX_NODES


def failure(code, **details):
    return {"ok": False, "error": code, **details}


def check_trace(doc):
    if not isinstance(doc, dict) or set(doc) != {"nodes", "edges", "events", "outcome"}:
        raise GraphError("invalid_shape")
    nodes, edges, events, outcome = (doc[key] for key in ("nodes", "edges", "events", "outcome"))
    check_list(events, MAX_EVENTS)
    if outcome not in ("complete", "partial"):
        return failure("invalid_outcome")

    # Validate the entire graph, including components never reached in this trace.
    plan({"nodes": nodes, "edges": edges})
    prerequisites = {node: [] for node in nodes}
    for before, after in edges:
        prerequisites[after].append(before)
    states = dict.fromkeys(nodes, "pending")
    for index, event in enumerate(events):
        if not isinstance(event, list) or len(event) != 2:
            return failure("invalid_event", event_index=index)
        action, node = event
        if action not in ("start", *TERMINALS):
            return failure("invalid_event", event_index=index)
        check_id(node)
        if node not in states:
            return failure("unknown_node", event_index=index, node=node)
        if action == "start":
            if states[node] != "pending":
                return failure("invalid_transition", event_index=index, node=node)
            for parent in prerequisites[node]:
                if states[parent] != "pass":
                    return failure("prerequisite_unverified", event_index=index,
                                   node=node, prerequisite=parent)
            states[node] = "running"
        else:
            if states[node] != "running":
                return failure("invalid_transition", event_index=index, node=node)
            states[node] = action

    counts = dict.fromkeys(("pending", *TERMINALS), 0)
    for node, state in states.items():
        if state == "running":
            return failure("unfinished", node=node)
        counts[state] += 1
    if (outcome == "complete") != (counts["pass"] == len(nodes)):
        return failure("outcome_mismatch", counts=counts)
    return {"ok": True, "outcome": outcome, "counts": counts}


def main():
    if len(sys.argv) != 1:
        print('{"ok":false,"error":"usage"}')
        return 2
    try:
        data = sys.stdin.buffer.read(MAX_BYTES + 1)
        if len(data) > MAX_BYTES:
            raise GraphError("input_too_large")
        report = check_trace(json.loads(data.decode("utf-8"), object_pairs_hook=unique_fields,
                                        parse_constant=reject_constant))
    except GraphError as exc:
        report = failure(exc.code)
        if exc.cycle:
            report["cycle"] = exc.cycle
    except (ValueError, RecursionError):
        report = failure("invalid_json")
    print(json.dumps(report, separators=(",", ":")))
    return 0 if report["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
