#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Read-only exact diagnostics over frozen R1 scores; no new efficacy study."""
import argparse
from decimal import Decimal, localcontext
from fractions import Fraction
import hashlib
import itertools
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / "docs/research/evaluations/2026-11-13-gemini-r1"
MAX_BYTES = 1_048_576
MAX_DEPTH = 33
DIMENSIONS = ["factual_accuracy", "source_traceability", "uncertainty_boundary",
              "narrative_coherence", "clarity", "audience_fit", "task_completion"]
EARLY = "conversation_summary_only"
PUBLISHED = "published_batch_answer_key"


class InputError(ValueError):
    """The existing numeric-transcription boundary was not satisfied."""


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise InputError("duplicate JSON member")
        result[key] = value
    return result


def _reject_number(_value):
    raise InputError("non-integer or non-finite JSON number")


def parse_json(data):
    if len(data) > MAX_BYTES:
        raise InputError("input exceeds 1048576 bytes")
    try:
        payload = json.loads(data.decode("utf-8"), object_pairs_hook=_unique_object,
                             parse_constant=_reject_number, parse_float=_reject_number)
    except (UnicodeError, ValueError, RecursionError) as error:
        raise InputError("invalid strict UTF-8 JSON") from error
    pending = [(payload, 1)]
    while pending:
        value, depth = pending.pop()
        if depth > MAX_DEPTH:
            raise InputError("input exceeds nesting depth 33")
        if isinstance(value, dict):
            pending.extend((item, depth + 1) for item in value.values())
        elif isinstance(value, list):
            pending.extend((item, depth + 1) for item in value)
    return payload


def validate_payload(payload):
    if not isinstance(payload, dict) or payload.get("dimensions") != DIMENSIONS:
        raise InputError("expected the seven ordered R1 dimensions")
    if not isinstance(payload.get("scope"), str) or not payload["scope"]:
        raise InputError("numeric-transcription scope is required")
    cases = payload.get("cases")
    if not isinstance(cases, list) or not 1 <= len(cases) <= 40:
        raise InputError("expected 1 through 40 cases")
    seen = set()
    for case in cases:
        if not isinstance(case, dict):
            raise InputError("case must be an object")
        case_id = case.get("case_id")
        if not isinstance(case_id, str) or not 1 <= len(case_id) <= 64 or case_id in seen:
            raise InputError("case IDs must be nonempty and unique")
        seen.add(case_id)
        batch = case.get("batch")
        if type(batch) is not int or not 1 <= batch <= 40:
            raise InputError("batch must be an integer from 1 through 40")
        for variant in ("X", "Y"):
            ratings = case.get("ratings_" + variant)
            if (not isinstance(ratings, list) or len(ratings) != len(DIMENSIONS)
                    or any(type(v) is not int or not 1 <= v <= 5 for v in ratings)):
                raise InputError("each draft needs seven integer ratings from 1 through 5")
        if case.get("declared_story_variant") not in ("X", "Y"):
            raise InputError("declared story variant must be X or Y")
        if case.get("assignment_provenance") not in (EARLY, PUBLISHED):
            raise InputError("unknown assignment provenance")
        if case.get("presentation_order") != "X_then_Y":
            raise InputError("this diagnostic expects the recorded R1 X_then_Y order")
    if sum(c["assignment_provenance"] == EARLY for c in cases) > 10:
        raise InputError("early-key enumeration is limited to ten cases")
    return sorted(cases, key=lambda c: c["case_id"])


def rational(value):
    """Keep exact fractions authoritative; decimal text is only an approximation."""
    with localcontext() as context:
        context.prec = 40
        decimal = Decimal(value.numerator) / Decimal(value.denominator)
        approximate = format(decimal, ".6f")
    return {"fraction": f"{value.numerator}/{value.denominator}", "approx_decimal": approximate}


def _mean(values):
    return sum(values, Fraction()) / len(values) if values else None


def _as_rational(value):
    return rational(value) if value is not None else None


def _range(values):
    present = [value for value in values if value is not None]
    return {"minimum": rational(min(present)), "maximum": rational(max(present))} if present else None


def _difference(case, omit_dimension=None):
    # Whole-case paired difference; the seven dimensions are correlated measures.
    x = [v for j, v in enumerate(case["ratings_X"]) if j != omit_dimension]
    y = [v for j, v in enumerate(case["ratings_Y"]) if j != omit_dimension]
    raw = Fraction(sum(x) - sum(y), len(x))
    return raw if case["declared_story_variant"] == "X" else -raw


def _enumeration(cases, differences):
    early = [i for i, c in enumerate(cases) if c["assignment_provenance"] == EARLY]
    held = [i for i, c in enumerate(cases) if c["assignment_provenance"] != EARLY]
    fixed_sum = sum((differences[i] for i in held), Fraction())
    raw = [Fraction(sum(cases[i]["ratings_X"]) - sum(cases[i]["ratings_Y"]), len(DIMENSIONS)) for i in early]
    declared_x_count = sum(cases[i]["declared_story_variant"] == "X" for i in early)
    unrestricted, balanced = [], []
    for signs in itertools.product((-1, 1), repeat=len(early)):
        value = (fixed_sum + sum((sign * v for sign, v in zip(signs, raw)), Fraction())) / len(cases)
        unrestricted.append(value)
        if signs.count(1) == declared_x_count:
            balanced.append(value)
    return {
        "assumption": "Only early summary-only keys vary; every other declared key is held fixed, not verified.",
        "case_ids": [cases[i]["case_id"] for i in early],
        "other_declared_keys_held_fixed": [cases[i]["case_id"] for i in held],
        "unrestricted": {"arrangements": len(unrestricted), "range": _range(unrestricted)},
        "preserve_declared_X_count": {"declared_X_count": declared_x_count,
                                      "arrangements": len(balanced), "range": _range(balanced)},
        "interpretation": "Sensitivity scenarios over the same cases; neither new cases nor confidence intervals.",
    }


def analyze_payload(payload):
    cases = validate_payload(payload)
    differences = [_difference(case) for case in cases]
    case_rows, dimension_rows, batch_rows, omitted_batches = [], [], [], []
    case_values, dimension_values, batch_values = [], [], []
    for i, case in enumerate(cases):
        value = _mean(differences[:i] + differences[i + 1:])
        case_values.append(value)
        case_rows.append({"removed_case_id": case["case_id"], "remaining_case_count": len(cases) - 1,
                          "difference": _as_rational(value)})
    for j, name in enumerate(DIMENSIONS):
        value = _mean([_difference(case, j) for case in cases])
        dimension_values.append(value)
        dimension_rows.append({"removed_dimension": name, "remaining_case_count": len(cases),
                               "remaining_dimensions_per_draft": len(DIMENSIONS) - 1,
                               "difference": rational(value)})
    for batch in sorted({case["batch"] for case in cases}):
        selected = [i for i, case in enumerate(cases) if case["batch"] == batch]
        remaining = [v for i, v in enumerate(differences) if i not in selected]
        value = _mean(remaining)
        batch_values.append(value)
        batch_rows.append({"batch": batch, "case_count": len(selected),
                           "case_ids": [cases[i]["case_id"] for i in selected],
                           "difference": rational(_mean([differences[i] for i in selected]))})
        omitted_batches.append({"removed_batch": batch, "remaining_case_count": len(remaining),
                                "difference": _as_rational(value)})
    extent = _mean([abs(v) for v in differences])
    story_scores, control_scores = [], []
    for case in cases:
        story = case["declared_story_variant"]
        story_scores.extend(case["ratings_" + story])
        control_scores.extend(case["ratings_" + ("Y" if story == "X" else "X")])
    return {
        "analysis_kind": "descriptive_existing_numeric_data",
        "input_scope": payload["scope"],
        "analysis_unit": "case",
        "denominators": {"cases": len(cases), "drafts": 2 * len(cases),
                         "dimensions_per_draft": len(DIMENSIONS), "ratings": 2 * len(cases) * len(DIMENSIONS)},
        "difference_definition": "Declared story minus control; equal-weight mean of seven ordinal ratings, not a validated scale.",
        "declared_mean_difference": rational(_mean(differences)),
        "leave_one_case_out": {"rows": case_rows, "range": _range(case_values)},
        "leave_one_dimension_out": {"rows": dimension_rows, "range": _range(dimension_values)},
        "individual_batches": batch_rows,
        "leave_one_batch_out": {"rows": omitted_batches, "range": _range(batch_values)},
        "early_key_sensitivity": _enumeration(cases, differences),
        "all_keys_unknown_bounds": {**_range([-extent, extent]),
                                    "assumption": "Every case may label either X or Y as story; no allocation-count constraint.",
                                    "interpretation": "Arithmetic label-uncertainty bounds, not confidence intervals."},
        "ceiling": {"rating_denominator_per_declared_condition": len(story_scores),
                    "declared_story_at_5": story_scores.count(5),
                    "declared_control_at_5": control_scores.count(5)},
        "limitations": [
            "Declared assignments remain unverified generation conditions; none is repaired or authenticated.",
            "All recorded presentations are X_then_Y; these diagnostics cannot identify a presentation-order effect.",
            "Purposefully selected cases may share topics, sources and a judge; no population or causal efficacy claim.",
            "Dimensions, omission runs and enumerated assignments add no independent observations.",
            "Sensitivity extrema are not confidence intervals; no new p-values, confidence intervals or equivalence test.",
            "This is neither a rescore, a new judge run, human evidence, a holdout nor product/host acceptance.",
        ],
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args(argv)
    try:
        names = ("ratings_normalized_R1.json", "score_input.py", "statistics_R1.json")
        sources = {}
        for name in names:
            with (PACKAGE / name).open("rb") as handle:
                data = handle.read(MAX_BYTES + 1)
            if len(data) > MAX_BYTES:
                raise InputError("source exceeds 1048576 bytes")
            sources[name] = data
        report = analyze_payload(parse_json(sources["ratings_normalized_R1.json"]))
        report["input_sha256"] = {name: hashlib.sha256(data).hexdigest() for name, data in sources.items()}
        # Write UTF-8 bytes so Windows text-mode newline translation cannot
        # change this reproducible report's LF bytes.
        sys.stdout.buffer.write((json.dumps(report, ensure_ascii=False, indent=2) + "\n").encode("utf-8"))
        return 0
    except (InputError, OSError) as error:
        # Do not echo private paths or attacker-controlled JSON keys.
        print(f"R1 robustness input error: {error if isinstance(error, InputError) else 'source unavailable'}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
