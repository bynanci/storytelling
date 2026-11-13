#!/usr/bin/env python3
"""Validate and summarize paired order-only judge records descriptively."""

import argparse
import json
import re
import sys
from pathlib import Path


DIMENSIONS = (
    "factual_accuracy",
    "source_traceability",
    "uncertainty_boundary",
    "narrative_coherence",
    "clarity",
    "audience_fit",
    "required_task_coverage",
)
W4_STATES = {"supported", "contradicted", "insufficient", "unavailable"}
HASH = re.compile(r"^[0-9a-f]{64}$")


class RecordError(ValueError):
    """The record cannot be compared without violating the frozen protocol."""


def _keys(value, required, label):
    if not isinstance(value, dict) or set(value) != set(required):
        raise RecordError(f"{label} fields must be exactly {sorted(required)}")


def _hash(value, label):
    if not isinstance(value, str) or not HASH.fullmatch(value):
        raise RecordError(f"{label} must be a lowercase SHA-256 hex digest")


def _validate_annotation(annotation, label):
    _keys(annotation, {"preference", "ratings", "source_checks"}, label)
    if annotation["preference"] not in {"X", "Y", "tie"}:
        raise RecordError(f"{label}.preference must be X, Y or tie")
    _keys(annotation["ratings"], {"X", "Y"}, f"{label}.ratings")
    for draft in ("X", "Y"):
        values = annotation["ratings"][draft]
        if not isinstance(values, list) or len(values) != len(DIMENSIONS):
            raise RecordError(f"{label}.ratings.{draft} must have seven ordered scores")
        if any(type(score) is not int or not 1 <= score <= 5 for score in values):
            raise RecordError(f"{label}.ratings.{draft} scores must be integers from 1 to 5")
    if not isinstance(annotation["source_checks"], list):
        raise RecordError(f"{label}.source_checks must be an array")
    seen = set()
    for index, check in enumerate(annotation["source_checks"]):
        _keys(check, {"claim_id", "state"}, f"{label}.source_checks[{index}]")
        if not isinstance(check["claim_id"], str) or not check["claim_id"] or check["claim_id"] in seen:
            raise RecordError(f"{label} source claim IDs must be nonempty and unique")
        if check["state"] not in W4_STATES:
            raise RecordError(f"{label} source check has an unsupported W4 state")
        seen.add(check["claim_id"])


def validate_record(record):
    _keys(record, {"schema_version", "record_kind", "dimensions", "cases"}, "record")
    if record["schema_version"] != "skill-evaluation-order-record/v1":
        raise RecordError("unsupported order-record version")
    if record["record_kind"] not in {"synthetic_fixture", "observed_order_rerun"}:
        raise RecordError("record_kind must identify a synthetic fixture or observed rerun")
    if record["dimensions"] != list(DIMENSIONS):
        raise RecordError("dimension names and order differ from rubric v1")
    if not isinstance(record["cases"], list) or not record["cases"]:
        raise RecordError("cases must be a nonempty array")

    case_ids = set()
    for case_index, case in enumerate(record["cases"]):
        label = f"cases[{case_index}]"
        _keys(case, {"case_id", "shared_inputs", "runs"}, label)
        case_id = case["case_id"]
        if not isinstance(case_id, str) or not case_id or case_id in case_ids:
            raise RecordError("case IDs must be nonempty and unique")
        case_ids.add(case_id)

        shared = case["shared_inputs"]
        _keys(shared, {"draft_pair_sha256", "packet_sha256", "prompt_sha256", "rubric_sha256", "model"},
              f"{label}.shared_inputs")
        for field in ("draft_pair_sha256", "packet_sha256", "prompt_sha256", "rubric_sha256"):
            _hash(shared[field], f"{label}.shared_inputs.{field}")
        model = shared["model"]
        _keys(model, {"provider", "model_id", "model_revision", "settings_sha256"}, f"{label}.shared_inputs.model")
        for field in ("provider", "model_id", "model_revision"):
            if not isinstance(model[field], str) or not model[field]:
                raise RecordError(f"{label}.shared_inputs.model.{field} must be explicit")
        _hash(model["settings_sha256"], f"{label}.shared_inputs.model.settings_sha256")

        runs = case["runs"]
        if not isinstance(runs, list) or len(runs) != 2:
            raise RecordError(f"{label} must contain exactly the two order-only runs")
        order_by_name = {}
        run_ids = set()
        contexts = set()
        for run_index, run in enumerate(runs):
            run_label = f"{label}.runs[{run_index}]"
            _keys(run, {"run_id", "judge_context_id", "order", "presented_ids", "raw", "adjudicated"}, run_label)
            for field, seen in (("run_id", run_ids), ("judge_context_id", contexts)):
                value = run[field]
                if not isinstance(value, str) or not value or value in seen:
                    raise RecordError(f"{run_label}.{field} must be nonempty and unique within the pair")
                seen.add(value)
            if run["order"] not in {"X_then_Y", "Y_then_X"} or run["order"] in order_by_name:
                raise RecordError(f"{label} must have one X_then_Y and one Y_then_X run")
            expected_ids = ["X", "Y"] if run["order"] == "X_then_Y" else ["Y", "X"]
            if run["presented_ids"] != expected_ids:
                raise RecordError(f"{run_label}.presented_ids disagree with declared order")
            _validate_annotation(run["raw"], f"{run_label}.raw")
            adjudicated = run["adjudicated"]
            _keys(adjudicated, {"status", "annotation"}, f"{run_label}.adjudicated")
            if adjudicated["status"] == "pending":
                if adjudicated["annotation"] is not None:
                    raise RecordError(f"{run_label} pending adjudication must remain null")
            elif adjudicated["status"] == "complete":
                _validate_annotation(adjudicated["annotation"], f"{run_label}.adjudicated.annotation")
            else:
                raise RecordError(f"{run_label}.adjudicated.status must be pending or complete")
            order_by_name[run["order"]] = run
        if set(order_by_name) != {"X_then_Y", "Y_then_X"}:
            raise RecordError(f"{label} lacks a balanced order pair")
    return record


def _mean(values):
    return round(sum(values) / len(values), 6) if values else None


def _gap(annotation):
    x = sum(annotation["ratings"]["X"]) / len(DIMENSIONS)
    y = sum(annotation["ratings"]["Y"]) / len(DIMENSIONS)
    return x - y


def analyze_record(record):
    validate_record(record)
    per_case = []
    flips = 0
    stable = 0
    shifts = []
    score_changes = {dimension: [] for dimension in DIMENSIONS}
    unresolved = {"insufficient": 0, "unavailable": 0}
    disagreement_case_ids = set()
    pending_adjudications = 0

    for case in record["cases"]:
        by_order = {run["order"]: run for run in case["runs"]}
        xy = by_order["X_then_Y"]
        yx = by_order["Y_then_X"]
        xy_pref = xy["raw"]["preference"]
        yx_pref = yx["raw"]["preference"]
        if xy_pref == yx_pref:
            stable += 1
        else:
            flips += 1
        gap_xy = _gap(xy["raw"])
        gap_yx = _gap(yx["raw"])
        shift = gap_xy - gap_yx
        shifts.append(shift)
        for run in (xy, yx):
            for dimension_index, dimension in enumerate(DIMENSIONS):
                for draft in ("X", "Y"):
                    score_changes[dimension].append(abs(
                        xy["raw"]["ratings"][draft][dimension_index]
                        - yx["raw"]["ratings"][draft][dimension_index]
                    ))
            for check in run["raw"]["source_checks"]:
                if check["state"] in unresolved:
                    unresolved[check["state"]] += 1
            adjudicated = run["adjudicated"]
            if adjudicated["status"] == "pending":
                pending_adjudications += 1
            elif adjudicated["annotation"] != run["raw"]:
                disagreement_case_ids.add(case["case_id"])
        per_case.append({
            "case_id": case["case_id"],
            "preference_xy": xy_pref,
            "preference_yx": yx_pref,
            "preference_flipped": xy_pref != yx_pref,
            "x_minus_y_gap_xy": round(gap_xy, 6),
            "x_minus_y_gap_yx": round(gap_yx, 6),
            "order_gap_shift": round(shift, 6),
        })

    n = len(per_case)
    return {
        "analysis_kind": "descriptive_only",
        "record_kind": record["record_kind"],
        "analysis_unit": "case",
        "case_count": n,
        "preference_flip_count": flips,
        "preference_stable_count": stable,
        "preference_flip_rate": round(flips / n, 6),
        "mean_x_minus_y_order_gap_shift": _mean(shifts),
        "mean_absolute_score_change_by_dimension": {
            key: _mean(values) for key, values in score_changes.items()
        },
        "raw_adjudicated_disagreement_case_count": len(disagreement_case_ids),
        "raw_adjudicated_disagreement_case_ids": sorted(disagreement_case_ids),
        "pending_adjudication_run_count": pending_adjudications,
        "unresolved_source_check_counts": unresolved,
        "per_case": per_case,
        "claims": [],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    analyze_parser = subparsers.add_parser("analyze", help="validate and summarize a paired order record")
    analyze_parser.add_argument("record", type=Path)
    args = parser.parse_args()
    try:
        record = json.loads(args.record.read_text(encoding="utf-8"))
        report = analyze_record(record)
    except (OSError, json.JSONDecodeError, RecordError) as exc:
        print(f"sk_e04_order_analysis: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
