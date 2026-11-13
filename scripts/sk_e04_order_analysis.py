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
    _keys(annotation, {"preference", "ratings", "source_checks", "rationale"}, label)
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
    if not isinstance(annotation["rationale"], str) or not annotation["rationale"]:
        raise RecordError(f"{label}.rationale must retain the annotation rationale")
    seen = set()
    for index, check in enumerate(annotation["source_checks"]):
        _keys(check, {"claim_id", "state", "evidence_location"}, f"{label}.source_checks[{index}]")
        if not isinstance(check["claim_id"], str) or not check["claim_id"] or check["claim_id"] in seen:
            raise RecordError(f"{label} source claim IDs must be nonempty and unique")
        if check["state"] not in W4_STATES:
            raise RecordError(f"{label} source check has an unsupported W4 state")
        location = check["evidence_location"]
        if location is not None and (not isinstance(location, str) or not location):
            raise RecordError(f"{label} evidence location must be nonempty text or null")
        if check["state"] in {"supported", "contradicted"} and location is None:
            raise RecordError(f"{label} supported/contradicted checks need an evidence location")
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
    frozen_cross_case = None
    for case_index, case in enumerate(record["cases"]):
        label = f"cases[{case_index}]"
        _keys(case, {"case_id", "shared_inputs", "runs"}, label)
        case_id = case["case_id"]
        if not isinstance(case_id, str) or not case_id or case_id in case_ids:
            raise RecordError("case IDs must be nonempty and unique")
        case_ids.add(case_id)

        shared = case["shared_inputs"]
        _keys(shared, {"draft_x_sha256", "draft_y_sha256", "packet_sha256", "prompt_sha256", "rubric_sha256", "model"},
              f"{label}.shared_inputs")
        for field in ("draft_x_sha256", "draft_y_sha256", "packet_sha256", "prompt_sha256", "rubric_sha256"):
            _hash(shared[field], f"{label}.shared_inputs.{field}")
        model = shared["model"]
        _keys(model, {"provider", "model_id", "model_revision", "settings_sha256"}, f"{label}.shared_inputs.model")
        for field in ("provider", "model_id", "model_revision"):
            if not isinstance(model[field], str) or not model[field]:
                raise RecordError(f"{label}.shared_inputs.model.{field} must be explicit")
        _hash(model["settings_sha256"], f"{label}.shared_inputs.model.settings_sha256")
        cross_case = (shared["prompt_sha256"], shared["rubric_sha256"], json.dumps(model, sort_keys=True))
        if frozen_cross_case is None:
            frozen_cross_case = cross_case
        elif cross_case != frozen_cross_case:
            raise RecordError("prompt, rubric and model/settings must be identical across all cases")

        runs = case["runs"]
        if not isinstance(runs, list) or len(runs) < 2 or len(runs) % 2:
            raise RecordError(f"{label} must contain complete X_then_Y/Y_then_X replicate pairs")
        by_replicate = {}
        run_ids = set()
        contexts = set()
        for run_index, run in enumerate(runs):
            run_label = f"{label}.runs[{run_index}]"
            _keys(run, {"run_id", "judge_context_id", "replicate_id", "order", "presented_ids", "raw_output_sha256", "raw", "adjudicated"}, run_label)
            for field, seen in (("run_id", run_ids), ("judge_context_id", contexts)):
                value = run[field]
                if not isinstance(value, str) or not value or value in seen:
                    raise RecordError(f"{run_label}.{field} must be nonempty and unique within the pair")
                seen.add(value)
            if run["order"] not in {"X_then_Y", "Y_then_X"}:
                raise RecordError(f"{run_label}.order must be X_then_Y or Y_then_X")
            replicate_id = run["replicate_id"]
            if not isinstance(replicate_id, str) or not replicate_id:
                raise RecordError(f"{run_label}.replicate_id must be nonempty")
            orders = by_replicate.setdefault(replicate_id, {})
            if run["order"] not in {"X_then_Y", "Y_then_X"} or run["order"] in orders:
                raise RecordError(f"{label} replicate must have one X_then_Y and one Y_then_X run")
            expected_ids = ["X", "Y"] if run["order"] == "X_then_Y" else ["Y", "X"]
            if run["presented_ids"] != expected_ids:
                raise RecordError(f"{run_label}.presented_ids disagree with declared order")
            _hash(run["raw_output_sha256"], f"{run_label}.raw_output_sha256")
            _validate_annotation(run["raw"], f"{run_label}.raw")
            adjudicated = run["adjudicated"]
            _keys(adjudicated, {"status", "reviewer_id", "reason", "annotation"}, f"{run_label}.adjudicated")
            if adjudicated["status"] == "pending":
                if any(adjudicated[field] is not None for field in ("reviewer_id", "reason", "annotation")):
                    raise RecordError(f"{run_label} pending adjudication fields must remain null")
            elif adjudicated["status"] == "complete":
                if not all(isinstance(adjudicated[field], str) and adjudicated[field]
                           for field in ("reviewer_id", "reason")):
                    raise RecordError(f"{run_label} adjudication needs reviewer ID and reason")
                _validate_annotation(adjudicated["annotation"], f"{run_label}.adjudicated.annotation")
            else:
                raise RecordError(f"{run_label}.adjudicated.status must be pending or complete")
            orders[run["order"]] = run
        min_replicates = 1 if record["record_kind"] == "synthetic_fixture" else 2
        if len(by_replicate) < min_replicates:
            raise RecordError(f"{label} needs at least {min_replicates} replicate pairs")
        if any(set(orders) != {"X_then_Y", "Y_then_X"} for orders in by_replicate.values()):
            raise RecordError(f"{label} has an incomplete order replicate")
    return record


def _mean(values):
    return round(sum(values) / len(values), 6) if values else None


def _gap(annotation):
    x = sum(annotation["ratings"]["X"]) / len(DIMENSIONS)
    y = sum(annotation["ratings"]["Y"]) / len(DIMENSIONS)
    return x - y


def _canonical_annotation(annotation):
    return {
        "preference": annotation["preference"],
        "ratings": annotation["ratings"],
        "source_checks": sorted(
            ({"claim_id": item["claim_id"], "state": item["state"], "evidence_location": item["evidence_location"]}
             for item in annotation["source_checks"]),
            key=lambda item: item["claim_id"],
        ),
    }


def analyze_record(record):
    validate_record(record)
    per_case = []
    preference_flips = 0
    preference_pairs = 0
    changed_preference_cases = 0
    shifts = []
    case_score_changes = {dimension: [] for dimension in DIMENSIONS}
    unresolved = {"insufficient": 0, "unavailable": 0}
    disagreement_case_ids = set()
    pending_adjudications = 0

    for case in record["cases"]:
        by_replicate = {}
        for run in case["runs"]:
            by_replicate.setdefault(run["replicate_id"], {})[run["order"]] = run
        replicate_ids = sorted(by_replicate)
        xy_runs = [by_replicate[key]["X_then_Y"] for key in replicate_ids]
        yx_runs = [by_replicate[key]["Y_then_X"] for key in replicate_ids]
        xy_preferences = [run["raw"]["preference"] for run in xy_runs]
        yx_preferences = [run["raw"]["preference"] for run in yx_runs]
        flips_in_case = 0
        for replicate_id in replicate_ids:
            xy_pref = by_replicate[replicate_id]["X_then_Y"]["raw"]["preference"]
            yx_pref = by_replicate[replicate_id]["Y_then_X"]["raw"]["preference"]
            preference_pairs += 1
            if xy_pref != yx_pref:
                preference_flips += 1
                flips_in_case += 1
        xy_profile = {pref: xy_preferences.count(pref) for pref in ("X", "Y", "tie")}
        yx_profile = {pref: yx_preferences.count(pref) for pref in ("X", "Y", "tie")}
        preference_profile_changed = xy_profile != yx_profile
        if preference_profile_changed:
            changed_preference_cases += 1
        gap_xy = _mean([_gap(run["raw"]) for run in xy_runs])
        gap_yx = _mean([_gap(run["raw"]) for run in yx_runs])
        shift = gap_xy - gap_yx
        shifts.append(shift)
        for dimension_index, dimension in enumerate(DIMENSIONS):
            changes = []
            for replicate_id in replicate_ids:
                xy = by_replicate[replicate_id]["X_then_Y"]["raw"]["ratings"]
                yx = by_replicate[replicate_id]["Y_then_X"]["raw"]["ratings"]
                for draft in ("X", "Y"):
                    changes.append(abs(xy[draft][dimension_index] - yx[draft][dimension_index]))
            case_score_changes[dimension].append(_mean(changes))
        for run in case["runs"]:
            for check in run["raw"]["source_checks"]:
                if check["state"] in unresolved:
                    unresolved[check["state"]] += 1
            adjudicated = run["adjudicated"]
            if adjudicated["status"] == "pending":
                pending_adjudications += 1
            elif _canonical_annotation(adjudicated["annotation"]) != _canonical_annotation(run["raw"]):
                disagreement_case_ids.add(case["case_id"])
        per_case.append({
            "case_id": case["case_id"],
            "replicate_pair_count": len(replicate_ids),
            "preference_flip_pair_count": flips_in_case,
            "preference_profile_xy": xy_profile,
            "preference_profile_yx": yx_profile,
            "preference_profile_changed": preference_profile_changed,
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
        "matched_preference_flip_count": preference_flips,
        "matched_preference_pair_count": preference_pairs,
        "matched_preference_flip_rate": round(preference_flips / preference_pairs, 6),
        "case_preference_profile_changed_count": changed_preference_cases,
        "mean_x_minus_y_order_gap_shift": _mean(shifts),
        "mean_absolute_score_change_by_dimension": {
            key: _mean(values) for key, values in case_score_changes.items()
        },
        "raw_adjudicated_disagreement_case_count": len(disagreement_case_ids),
        "raw_adjudicated_disagreement_case_ids": sorted(disagreement_case_ids),
        "pending_adjudication_run_count": pending_adjudications,
        "unresolved_source_check_counts": unresolved,
        "per_case": per_case,
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
