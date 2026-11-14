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
CALL_OUTCOMES = {
    "valid_annotation",
    "refusal",
    "invalid_annotation",
    "timeout",
    "request_error",
    "not_attempted",
}
FAILED_OUTCOMES = {"refusal", "invalid_annotation", "timeout", "request_error"}
HASH = re.compile(r"^[0-9a-f]{64}$")
ARTIFACT_ID = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$")


class RecordError(ValueError):
    """The record cannot be compared without violating the frozen protocol."""


def _unique_json_object(pairs):
    value = {}
    for key, item in pairs:
        if key in value:
            raise RecordError("duplicate JSON member")
        value[key] = item
    return value


def _reject_json_constant(value):
    raise RecordError(f"non-finite JSON constant: {value}")


def _parse_record_json(text):
    try:
        return json.loads(
            text, object_pairs_hook=_unique_json_object, parse_constant=_reject_json_constant,
        )
    except (json.JSONDecodeError, RecordError):
        raise
    except ValueError as exc:
        # Preserve the decoder's own numeric conversion limits; do not change them.
        raise RecordError("JSON numeric value cannot be decoded") from exc


def _keys(value, required, label):
    if not isinstance(value, dict) or set(value) != set(required):
        raise RecordError(f"{label} fields must be exactly {sorted(required)}")


def _hash(value, label):
    if not isinstance(value, str) or not HASH.fullmatch(value):
        raise RecordError(f"{label} must be a lowercase SHA-256 hex digest")


def _validate_annotation(annotation, label):
    _keys(annotation, {"preference", "ratings", "source_checks", "rationale"}, label)
    if not isinstance(annotation["preference"], str) or annotation["preference"] not in {"X", "Y", "tie"}:
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
        claim_id = check["claim_id"]
        if not isinstance(claim_id, str) or not claim_id or claim_id in seen:
            raise RecordError(f"{label} source claim IDs must be nonempty and unique")
        state = check["state"]
        if not isinstance(state, str) or state not in W4_STATES:
            raise RecordError(f"{label} source check has an unsupported W4 state")
        location = check["evidence_location"]
        if location is not None and (not isinstance(location, str) or not location):
            raise RecordError(f"{label} evidence location must be nonempty text or null")
        if state in {"supported", "contradicted"} and location is None:
            raise RecordError(f"{label} supported/contradicted checks need an evidence location")
        seen.add(claim_id)


def _validate_adjudicated(adjudicated, label):
    _keys(adjudicated, {"status", "reviewer_id", "reason", "annotation"}, label)
    if adjudicated["status"] == "pending":
        if any(adjudicated[field] is not None for field in ("reviewer_id", "reason", "annotation")):
            raise RecordError(f"{label} pending adjudication fields must remain null")
    elif adjudicated["status"] == "complete":
        if not all(isinstance(adjudicated[field], str) and adjudicated[field]
                   for field in ("reviewer_id", "reason")):
            raise RecordError(f"{label} adjudication needs reviewer ID and reason")
        _validate_annotation(adjudicated["annotation"], f"{label}.annotation")
    else:
        raise RecordError(f"{label}.status must be pending or complete")


def _validate_evidence_reference(run, label, outcome):
    digest = run["raw_output_sha256"]
    artifact_id = run["raw_output_ref"]
    if (digest is None) != (artifact_id is None):
        raise RecordError(f"{label} raw output digest and artifact reference must both be present or null")
    if digest is not None:
        _hash(digest, f"{label}.raw_output_sha256")
        if not isinstance(artifact_id, str) or not ARTIFACT_ID.fullmatch(artifact_id):
            raise RecordError(f"{label}.raw_output_ref must be an opaque artifact ID, not a path")
    if outcome in {"valid_annotation", "refusal", "invalid_annotation"} and digest is None:
        raise RecordError(f"{label}.{outcome} must retain the output digest and artifact reference")
    if outcome == "not_attempted" and digest is not None:
        raise RecordError(f"{label}.not_attempted cannot have output bytes")


def validate_record(record):
    _keys(record, {"schema_version", "record_kind", "dimensions", "cases"}, "record")
    version = record["schema_version"]
    if not isinstance(version, str) or version not in {
        "skill-evaluation-order-record/v1", "skill-evaluation-order-record/v2"
    }:
        raise RecordError("unsupported order-record version")
    v2 = version.endswith("/v2")
    if not isinstance(record["record_kind"], str) or record["record_kind"] not in {
        "synthetic_fixture", "observed_order_rerun"
    }:
        raise RecordError("record_kind must identify a synthetic fixture or observed rerun")
    if record["dimensions"] != list(DIMENSIONS):
        raise RecordError("dimension names and order differ from rubric v1")
    if not isinstance(record["cases"], list) or not record["cases"]:
        raise RecordError("cases must be a nonempty array")

    case_ids = set()
    artifact_digests = {}
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
            raise RecordError(f"{label} must contain explicit X_then_Y/Y_then_X run slots for every replicate pair")
        by_replicate = {}
        run_ids = set()
        contexts = set()
        for run_index, run in enumerate(runs):
            run_label = f"{label}.runs[{run_index}]"
            if v2:
                run_fields = {"run_id", "judge_context_id", "replicate_id", "order", "presented_ids",
                              "call_outcome", "raw_output_sha256", "raw_output_ref", "raw", "adjudicated"}
            else:
                run_fields = {"run_id", "judge_context_id", "replicate_id", "order", "presented_ids",
                              "raw_output_sha256", "raw", "adjudicated"}
            _keys(run, run_fields, run_label)
            run_id = run["run_id"]
            if not isinstance(run_id, str) or not run_id or run_id in run_ids:
                raise RecordError(f"{run_label}.run_id must be nonempty and unique within the case")
            run_ids.add(run_id)

            outcome = run["call_outcome"] if v2 else "valid_annotation"
            if not isinstance(outcome, str) or outcome not in CALL_OUTCOMES:
                raise RecordError(f"{run_label}.call_outcome is unsupported")
            context_id = run["judge_context_id"]
            if outcome == "not_attempted":
                if context_id is not None:
                    raise RecordError(f"{run_label}.not_attempted must not claim a judge context")
            else:
                if not isinstance(context_id, str) or not context_id or context_id in contexts:
                    raise RecordError(f"{run_label}.judge_context_id must be nonempty and unique within the case")
                contexts.add(context_id)

            if not isinstance(run["order"], str) or run["order"] not in {"X_then_Y", "Y_then_X"}:
                raise RecordError(f"{run_label}.order must be X_then_Y or Y_then_X")
            replicate_id = run["replicate_id"]
            if not isinstance(replicate_id, str) or not replicate_id:
                raise RecordError(f"{run_label}.replicate_id must be nonempty")
            orders = by_replicate.setdefault(replicate_id, {})
            if run["order"] in orders:
                raise RecordError(f"{label} replicate must have one X_then_Y and one Y_then_X run slot")
            expected_ids = ["X", "Y"] if run["order"] == "X_then_Y" else ["Y", "X"]
            if run["presented_ids"] != expected_ids:
                raise RecordError(f"{run_label}.presented_ids disagree with declared order")

            if v2:
                _validate_evidence_reference(run, run_label, outcome)
                artifact_id = run["raw_output_ref"]
                if artifact_id is not None:
                    digest = run["raw_output_sha256"]
                    if artifact_id in artifact_digests and artifact_digests[artifact_id] != digest:
                        raise RecordError(f"{run_label}.raw_output_ref has conflicting output digests")
                    artifact_digests[artifact_id] = digest
                if outcome == "valid_annotation":
                    _validate_annotation(run["raw"], f"{run_label}.raw")
                    _validate_adjudicated(run["adjudicated"], f"{run_label}.adjudicated")
                elif run["raw"] is not None or run["adjudicated"] is not None:
                    raise RecordError(f"{run_label} non-valid call must keep annotation and adjudication null")
            else:
                _hash(run["raw_output_sha256"], f"{run_label}.raw_output_sha256")
                _validate_annotation(run["raw"], f"{run_label}.raw")
                _validate_adjudicated(run["adjudicated"], f"{run_label}.adjudicated")
            orders[run["order"]] = run
        min_replicates = 1 if record["record_kind"] == "synthetic_fixture" else 2
        if len(by_replicate) < min_replicates:
            raise RecordError(f"{label} needs at least {min_replicates} replicate-pair slots")
        if any(set(orders) != {"X_then_Y", "Y_then_X"} for orders in by_replicate.values()):
            raise RecordError(f"{label} is missing an explicit order slot; use call_outcome=not_attempted")
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


def _outcome(run, record_version):
    return run["call_outcome"] if record_version.endswith("/v2") else "valid_annotation"


def _empty_score_changes():
    return {draft: {dimension: [] for dimension in DIMENSIONS} for draft in ("X", "Y")}


def _summarize_score_changes(changes):
    return {
        draft: {dimension: _mean(values) for dimension, values in by_dimension.items()}
        for draft, by_dimension in changes.items()
    }


def analyze_record(record):
    validate_record(record)
    record_version = record["schema_version"]
    per_case = []
    preference_flips = 0
    preference_pairs = 0
    changed_preference_cases = 0
    shifts = []
    case_score_changes = _empty_score_changes()
    unresolved = {"insufficient": 0, "unavailable": 0}
    disagreement_case_ids = set()
    pending_adjudications = 0
    outcome_counts = {outcome: 0 for outcome in sorted(CALL_OUTCOMES)}
    planned_pair_count = 0
    comparable_pair_count = 0
    failed_pair_count = 0
    not_attempted_pair_count = 0
    comparable_case_count = 0

    for case in record["cases"]:
        by_replicate = {}
        case_outcomes = {outcome: 0 for outcome in sorted(CALL_OUTCOMES)}
        for run in case["runs"]:
            outcome = _outcome(run, record_version)
            outcome_counts[outcome] += 1
            case_outcomes[outcome] += 1
            by_replicate.setdefault(run["replicate_id"], {})[run["order"]] = run

        flips_in_case = 0
        comparable_replicates = []
        case_failed_pair_count = 0
        case_not_attempted_pair_count = 0
        for replicate_id in sorted(by_replicate):
            pair = by_replicate[replicate_id]
            xy = pair["X_then_Y"]
            yx = pair["Y_then_X"]
            xy_outcome = _outcome(xy, record_version)
            yx_outcome = _outcome(yx, record_version)
            if xy_outcome in FAILED_OUTCOMES or yx_outcome in FAILED_OUTCOMES:
                failed_pair_count += 1
                case_failed_pair_count += 1
            if xy_outcome == "not_attempted" or yx_outcome == "not_attempted":
                not_attempted_pair_count += 1
                case_not_attempted_pair_count += 1
            if xy_outcome == "valid_annotation" and yx_outcome == "valid_annotation":
                comparable_replicates.append((replicate_id, xy, yx))
                preference_pairs += 1
                if xy["raw"]["preference"] != yx["raw"]["preference"]:
                    preference_flips += 1
                    flips_in_case += 1

        case_planned_pairs = len(by_replicate)
        planned_pair_count += case_planned_pairs
        case_comparable_pairs = len(comparable_replicates)
        comparable_pair_count += case_comparable_pairs
        xy_preferences = [xy["raw"]["preference"] for _, xy, _ in comparable_replicates]
        yx_preferences = [yx["raw"]["preference"] for _, _, yx in comparable_replicates]
        if comparable_replicates:
            comparable_case_count += 1
            xy_profile = {pref: xy_preferences.count(pref) for pref in ("X", "Y", "tie")}
            yx_profile = {pref: yx_preferences.count(pref) for pref in ("X", "Y", "tie")}
            preference_profile_changed = xy_profile != yx_profile
            if preference_profile_changed:
                changed_preference_cases += 1
            pair_shifts = []
            local_score_changes = _empty_score_changes()
            for _, xy, yx in comparable_replicates:
                pair_shifts.append(_gap(xy["raw"]) - _gap(yx["raw"]))
                xy_ratings = xy["raw"]["ratings"]
                yx_ratings = yx["raw"]["ratings"]
                for draft in ("X", "Y"):
                    for dimension_index, dimension in enumerate(DIMENSIONS):
                        local_score_changes[draft][dimension].append(
                            abs(xy_ratings[draft][dimension_index] - yx_ratings[draft][dimension_index])
                        )
            gap_xy = _mean([_gap(xy["raw"]) for _, xy, _ in comparable_replicates])
            gap_yx = _mean([_gap(yx["raw"]) for _, _, yx in comparable_replicates])
            shift = _mean(pair_shifts)
            shifts.append(shift)
            summarized_local_changes = _summarize_score_changes(local_score_changes)
            for draft in ("X", "Y"):
                for dimension in DIMENSIONS:
                    case_score_changes[draft][dimension].append(summarized_local_changes[draft][dimension])
        else:
            xy_profile = None
            yx_profile = None
            preference_profile_changed = None
            gap_xy = None
            gap_yx = None
            shift = None
            summarized_local_changes = {draft: {dimension: None for dimension in DIMENSIONS}
                                        for draft in ("X", "Y")}

        for run in case["runs"]:
            if _outcome(run, record_version) != "valid_annotation":
                continue
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
            "planned_pair_count": case_planned_pairs,
            "comparable_pair_count": case_comparable_pairs,
            "incomplete_pair_count": case_planned_pairs - case_comparable_pairs,
            "failed_pair_count": case_failed_pair_count,
            "not_attempted_pair_count": case_not_attempted_pair_count,
            "call_outcome_counts": case_outcomes,
            "preference_flip_pair_count": flips_in_case,
            "preference_flip_rate": round(flips_in_case / case_comparable_pairs, 6) if case_comparable_pairs else None,
            "preference_profile_xy": xy_profile,
            "preference_profile_yx": yx_profile,
            "preference_profile_changed": preference_profile_changed,
            "x_minus_y_gap_xy": gap_xy,
            "x_minus_y_gap_yx": gap_yx,
            "order_gap_shift": shift,
            "mean_absolute_score_change_by_draft_and_dimension": summarized_local_changes,
        })

    n = len(per_case)
    return {
        "analysis_kind": "descriptive_only",
        "record_kind": record["record_kind"],
        "record_schema_version": record_version,
        "analysis_unit": "case",
        "case_count": n,
        "case_count_with_comparable_pairs": comparable_case_count,
        "planned_pair_count": planned_pair_count,
        "comparable_pair_count": comparable_pair_count,
        "incomplete_pair_count": planned_pair_count - comparable_pair_count,
        "failed_pair_count": failed_pair_count,
        "not_attempted_pair_count": not_attempted_pair_count,
        "run_count": sum(outcome_counts.values()),
        "valid_annotation_run_count": outcome_counts["valid_annotation"],
        "failed_call_count": sum(outcome_counts[outcome] for outcome in FAILED_OUTCOMES),
        "not_attempted_call_count": outcome_counts["not_attempted"],
        "call_outcome_counts": outcome_counts,
        "matched_preference_flip_count": preference_flips,
        "matched_preference_pair_count": preference_pairs,
        "matched_preference_flip_rate": round(preference_flips / preference_pairs, 6) if preference_pairs else None,
        "case_preference_profile_changed_count": changed_preference_cases,
        "mean_x_minus_y_order_gap_shift": _mean(shifts),
        "mean_absolute_score_change_by_draft_and_dimension": {
            draft: {dimension: _mean(values) for dimension, values in by_dimension.items()}
            for draft, by_dimension in case_score_changes.items()
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
        record = _parse_record_json(args.record.read_text(encoding="utf-8"))
        report = analyze_record(record)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, RecordError) as exc:
        print(f"sk_e04_order_analysis: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
