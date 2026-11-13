#!/usr/bin/env python3
"""Stdlib checks for SK-E02 evidence plans, adjudications and packet errata."""
from __future__ import annotations

import hashlib
import json
import re
import sys
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "docs/research/evaluations/sk-e02/fixtures"
SHA256 = re.compile(r"^[a-f0-9]{64}$")
W4_STATES = {"supported", "contradicted", "insufficient", "unavailable"}


class ContractError(ValueError):
    """An SK-E02 record violates a frozen evaluation boundary."""


def _fail(message: str) -> None:
    raise ContractError(message)


def _object(value, required, allowed, label):
    if not isinstance(value, dict):
        _fail(f"{label} must be an object")
    missing = set(required) - value.keys()
    extra = value.keys() - set(allowed)
    if missing or extra:
        _fail(f"{label} fields: missing={sorted(missing)}, extra={sorted(extra)}")


def _timestamp(value, label):
    if value is None:
        return None
    if not isinstance(value, str) or not value.endswith("Z"):
        _fail(f"{label} must be a UTC RFC3339 string ending in Z or null")
    try:
        parsed = datetime.fromisoformat(value[:-1] + "+00:00")
    except ValueError:
        _fail(f"{label} is not a valid timestamp")
    return parsed


def _digest(value, label, nullable=False):
    if value is None and nullable:
        return
    if not isinstance(value, str) or not SHA256.fullmatch(value):
        _fail(f"{label} must be a lowercase SHA-256 digest")


def _choice(value, options, label):
    if not isinstance(value, str) or value not in options:
        _fail(f"invalid {label}")


def _unique(values, label):
    if len(values) != len(set(values)):
        _fail(f"{label} must be unique")


def validate_evidence_plan(plan):
    _object(plan,
            {"schema_version", "record_kind", "evaluation_id", "phase", "evidence_mode",
             "packet", "freeze", "sources", "claims"},
            {"schema_version", "record_kind", "evaluation_id", "phase", "evidence_mode",
             "packet", "freeze", "sources", "claims"}, "evidence plan")
    if plan["schema_version"] != "skill-evaluation-evidence-plan/v1":
        _fail("unsupported evidence-plan schema version")
    _choice(plan["record_kind"], {"evaluation_plan", "retrospective_record", "synthetic_fixture"},
            "evidence-plan record kind")
    _choice(plan["phase"], {"prospective", "retrospective"}, "evidence-plan phase")
    _choice(plan["evidence_mode"], {"packet_only", "packet_plus_archived_sources"}, "evidence mode")
    if not isinstance(plan["evaluation_id"], str) or not plan["evaluation_id"]:
        _fail("evaluation_id must be nonempty")

    _object(plan["packet"], {"packet_id", "revision", "sha256"},
            {"packet_id", "revision", "sha256"}, "packet")
    if not all(isinstance(plan["packet"][key], str) and plan["packet"][key]
               for key in ("packet_id", "revision")):
        _fail("packet identity and revision must be nonempty")
    _digest(plan["packet"]["sha256"], "packet.sha256",
            nullable=plan["phase"] == "retrospective")

    _object(plan["freeze"], {"claims_frozen_at", "variant_generation_started_at"},
            {"claims_frozen_at", "variant_generation_started_at"}, "freeze")
    frozen = _timestamp(plan["freeze"]["claims_frozen_at"], "claims_frozen_at")
    drafted = _timestamp(plan["freeze"]["variant_generation_started_at"],
                         "variant_generation_started_at")
    if plan["phase"] == "prospective":
        if frozen is None:
            _fail("prospective plan must freeze claims before generation")
        if drafted is not None and drafted <= frozen:
            _fail("variant generation must start strictly after the claims freeze")
    elif frozen is not None or drafted is not None:
        _fail("retrospective records cannot backfill unrecorded freeze or generation times")

    sources = plan["sources"]
    claims = plan["claims"]
    if not isinstance(sources, list) or not isinstance(claims, list) or not claims:
        _fail("sources and nonempty claims must be arrays")
    source_ids = []
    source_by_id = {}
    source_allowed = {"source_id", "source_revision", "revision_status", "citation_url",
                      "access_status", "retrieved_at", "http_status", "content_type",
                      "response_sha256", "archive_ref", "archived_passage_location"}
    for source in sources:
        _object(source, source_allowed, source_allowed, "source")
        sid = source["source_id"]
        if not isinstance(sid, str) or not sid:
            _fail("source_id must be nonempty")
        source_ids.append(sid)
        source_by_id[sid] = source
        _choice(source["revision_status"], {"known", "unknown", "not_recorded"}, f"revision status for {sid}")
        if (source["revision_status"] == "known") != (isinstance(source["source_revision"], str)
                                                          and bool(source["source_revision"])):
            _fail(f"source revision and revision status disagree for {sid}")
        if not isinstance(source["citation_url"], str) or not source["citation_url"].startswith("https://"):
            _fail(f"source citation must be an HTTPS URL for {sid}")
        status = source["access_status"]
        _choice(status, {"available", "unavailable", "not_attempted", "not_recorded"},
                f"source-access status for {sid}")
        _timestamp(source["retrieved_at"], f"{sid}.retrieved_at")
        if source["http_status"] is not None and (not isinstance(source["http_status"], int)
                                                    or not 100 <= source["http_status"] <= 599):
            _fail(f"invalid HTTP status for {sid}")
        _digest(source["response_sha256"], f"{sid}.response_sha256", nullable=True)
        for field in ("content_type", "archive_ref", "archived_passage_location"):
            if source[field] is not None and (not isinstance(source[field], str) or not source[field]):
                _fail(f"{sid}.{field} must be nonempty or null")
        if status == "available":
            if (source["retrieved_at"] is None or source["http_status"] is None
                    or not 200 <= source["http_status"] < 300 or source["content_type"] is None
                    or source["response_sha256"] is None or source["archive_ref"] is None
                    or source["archived_passage_location"] is None):
                _fail(f"available source {sid} needs response hash and archived passage provenance")
            if source["revision_status"] != "known":
                _fail(f"available source {sid} needs a known source revision")
        elif any(source[field] is not None for field in
                 ("response_sha256", "archive_ref", "archived_passage_location")):
            _fail(f"unavailable or unattempted source {sid} cannot claim archived evidence")
        if plan["evidence_mode"] == "packet_only" and status not in {"not_attempted", "not_recorded"}:
            _fail("packet_only plans cannot record original-source access as evaluation evidence")
    _unique(source_ids, "source IDs")

    claim_ids = []
    claim_allowed = {"claim_id", "proposition_summary", "requiredness", "source_refs"}
    ref_allowed = {"source_id", "source_revision", "packet_location", "archived_source_location"}
    for claim in claims:
        _object(claim, claim_allowed, claim_allowed, "claim")
        cid = claim["claim_id"]
        if not isinstance(cid, str) or not cid:
            _fail("claim_id must be nonempty")
        claim_ids.append(cid)
        if not isinstance(claim["proposition_summary"], str) or not claim["proposition_summary"]:
            _fail(f"{cid} needs a neutral proposition summary")
        if plan["phase"] == "prospective":
            if not isinstance(claim["requiredness"], str) or claim["requiredness"] not in {"required", "optional"}:
                _fail(f"prospective claim {cid} must be marked required or optional")
        elif claim["requiredness"] != "unknown":
            _fail(f"retrospective claim {cid} must leave unrecorded requiredness unknown")
        refs = claim["source_refs"]
        if not isinstance(refs, list) or not refs:
            _fail(f"{cid} needs at least one source reference")
        for ref in refs:
            _object(ref, ref_allowed, ref_allowed, f"{cid} source reference")
            source = source_by_id.get(ref["source_id"])
            if source is None:
                _fail(f"{cid} references missing source {ref['source_id']}")
            if ref["source_revision"] != source["source_revision"]:
                _fail(f"{cid} source revision does not match source register")
            if not isinstance(ref["packet_location"], str) or not ref["packet_location"]:
                _fail(f"{cid} needs a packet support location")
            if source["access_status"] == "available":
                if not isinstance(ref["archived_source_location"], str) or not ref["archived_source_location"]:
                    _fail(f"{cid} needs an archived source location for an available source")
            elif ref["archived_source_location"] is not None:
                _fail(f"{cid} cannot cite an unavailable source passage")
            if plan["evidence_mode"] == "packet_only" and ref["archived_source_location"] is not None:
                _fail("packet_only claims cannot cite archived original-source passages")
    _unique(claim_ids, "claim IDs")
    return plan


def validate_adjudication(record):
    allowed = {"schema_version", "record_kind", "evidence_plan_sha256", "claim_checks",
               "packet_scope_violations", "packet_source_mismatches"}
    _object(record, allowed, allowed, "adjudication")
    if record["schema_version"] != "skill-evaluation-adjudication/v1":
        _fail("unsupported adjudication schema version")
    _choice(record["record_kind"], {"adjudication", "synthetic_fixture"}, "adjudication record kind")
    _digest(record["evidence_plan_sha256"], "evidence_plan_sha256")
    if not isinstance(record["claim_checks"], list):
        _fail("claim_checks must be an array")
    claim_ids = []
    check_fields = {"claim_id", "source_access_status", "support_state", "evidence_locations", "note"}
    for check in record["claim_checks"]:
        _object(check, check_fields, check_fields, "claim check")
        claim_ids.append(check["claim_id"])
        _choice(check["source_access_status"], {"available", "unavailable", "not_attempted", "not_recorded"},
                f"source access state for {check['claim_id']}")
        _choice(check["support_state"], W4_STATES, f"W4 support state for {check['claim_id']}")
        if (check["source_access_status"] in {"unavailable", "not_attempted", "not_recorded"}
                and check["support_state"] not in {"insufficient", "unavailable"}):
            _fail("failed source access supports only insufficient or unavailable")
        if not isinstance(check["evidence_locations"], list) or not isinstance(check["note"], str):
            _fail(f"invalid evidence locations or note for {check['claim_id']}")
    _unique(claim_ids, "adjudicated claim IDs")
    for field in ("packet_scope_violations", "packet_source_mismatches"):
        if not isinstance(record[field], list):
            _fail(f"{field} must be a separate array")
        for annotation in record[field]:
            _object(annotation, {"claim_id", "detail", "evidence_location"},
                    {"claim_id", "detail", "evidence_location"}, field)
            if not all(isinstance(annotation[key], str) and annotation[key]
                       for key in ("claim_id", "detail", "evidence_location")):
                _fail(f"{field} entries need claim and evidence locations")
    return record


def validate_errata(record):
    allowed = {"schema_version", "record_kind", "errata_revision", "source_audit_ref",
               "r1_exact_packet_bytes", "mutates_r1_artifacts", "errata"}
    _object(record, allowed, allowed, "errata record")
    if record["schema_version"] != "skill-evaluation-packet-errata/v1":
        _fail("unsupported errata schema version")
    if record["record_kind"] != "reported_historical_errata":
        _fail("errata must identify historical reported corrections")
    if not isinstance(record["errata_revision"], str) or not record["errata_revision"]:
        _fail("errata revision must be nonempty")
    _object(record["r1_exact_packet_bytes"], {"status", "value"},
            {"status", "value"}, "r1_exact_packet_bytes")
    if record["r1_exact_packet_bytes"] != {"status": "unavailable", "value": None}:
        _fail("R1 exact packet bytes must remain explicitly unavailable")
    if record["mutates_r1_artifacts"] is not False:
        _fail("R1 packets, outputs, scores, assignments and flags are immutable")
    if not isinstance(record["errata"], list) or len(record["errata"]) != 2:
        _fail("the R1 follow-up must include exactly C13 and C28 errata")
    if any(not isinstance(item, dict) or not isinstance(item.get("case_id"), str)
           for item in record["errata"]):
        _fail("each erratum needs a string case_id")
    entries = {item["case_id"]: item for item in record["errata"]}
    if set(entries) != {"C13", "C28"}:
        _fail("errata case IDs must be C13 and C28")
    item_fields = {"case_id", "field", "original_fragment", "original_fragment_fidelity",
                   "corrected_fragment", "corrected_fragment_fidelity", "corrected_packet_revision",
                   "source_review"}
    source_fields = {"source_id", "source_revision", "revision_status", "citation_url",
                     "access_status", "support_state", "retrieved_at", "http_status",
                     "content_type", "response_sha256", "archive_ref", "retained_in_repository",
                     "support_location_summary"}
    for case_id, item in entries.items():
        _object(item, item_fields, item_fields, f"{case_id} erratum")
        if not all(isinstance(item[key], str) and item[key]
                   for key in ("field", "original_fragment", "corrected_fragment", "corrected_packet_revision")):
            _fail(f"{case_id} erratum needs original and corrected wording")
        if item["original_fragment"] == item["corrected_fragment"]:
            _fail(f"{case_id} erratum does not change the disputed fragment")
        _choice(item["original_fragment_fidelity"], {"exact_fragment_from_public_R1_audit",
                                                      "paraphrase_from_public_R1_audit"},
                f"{case_id} original wording fidelity")
        _choice(item["corrected_fragment_fidelity"], {"direct_current_source_review",
                                                       "quoted_in_public_R1_audit",
                                                       "paraphrase_from_public_R1_audit"},
                f"{case_id} corrected wording fidelity")
        source = item["source_review"]
        _object(source, source_fields, source_fields, f"{case_id} source review")
        if source["source_id"] != case_id:
            _fail(f"{case_id} source identity or revision status is invalid")
        _choice(source["revision_status"], {"known", "unknown", "not_recorded"},
                f"{case_id} source revision status")
        if source["source_revision"] is not None or source["revision_status"] != "not_recorded":
            _fail(f"{case_id} historical R1 source revision was not recorded")
        _choice(source["access_status"], {"captured_transiently", "unavailable"},
                f"{case_id} source access status")
        _choice(source["support_state"], W4_STATES, f"{case_id} source W4 support state")
        _timestamp(source["retrieved_at"], f"{case_id}.retrieved_at")
        if source["http_status"] is not None and (not isinstance(source["http_status"], int)
                                                    or not 100 <= source["http_status"] <= 599):
            _fail(f"{case_id} HTTP status is invalid")
        _digest(source["response_sha256"], f"{case_id}.response_sha256", nullable=True)
        if source["archive_ref"] is not None or source["retained_in_repository"] is not False:
            _fail(f"{case_id} public record must not claim a retained copyrighted source archive")
        if source["access_status"] == "unavailable":
            if (source["support_state"] != "unavailable" or source["response_sha256"] is not None
                    or source["http_status"] == 200):
                _fail(f"{case_id} failed access remains unavailable without a source hash")
        elif source["support_state"] == "unavailable" or source["response_sha256"] is None:
            _fail(f"{case_id} transient successful source review needs the observed response hash")
        if not isinstance(source["support_location_summary"], str) or not source["support_location_summary"]:
            _fail(f"{case_id} source review needs a minimal location summary")
    if entries["C13"]["original_fragment"] != "鄭博宇" or entries["C13"]["corrected_fragment"] != "鄭博隅":
        _fail("C13 name erratum differs from the published source audit")
    if entries["C28"]["source_review"]["support_state"] != "unavailable":
        _fail("C28 access failure cannot be represented as a contradiction")
    return record


def _safe_fixture_path(relative):
    if not isinstance(relative, str) or not relative or "\\" in relative:
        _fail("fixture reference must be a relative POSIX path")
    path = Path(relative)
    if path.is_absolute() or ".." in path.parts:
        _fail("fixture reference escapes fixture root")
    resolved = (FIXTURES / path).resolve()
    if not resolved.is_relative_to(FIXTURES.resolve()) or not resolved.is_file():
        _fail("fixture reference is missing or escapes fixture root")
    return resolved


def verify_fixture_hashes(plan):
    errors = []
    packet_path = _safe_fixture_path("synthetic-packet.txt")
    packet_digest = hashlib.sha256(packet_path.read_bytes()).hexdigest()
    if packet_digest != plan["packet"]["sha256"]:
        errors.append("synthetic packet SHA-256 mismatch")
    for source in plan["sources"]:
        if source["access_status"] == "available":
            path = _safe_fixture_path(source["archive_ref"])
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            if digest != source["response_sha256"]:
                errors.append(f"{source['source_id']} archived fixture SHA-256 mismatch")
    return errors


def run_fixture_checks():
    schema_dir = ROOT / "docs/research/evaluations/sk-e02/schemas"
    schema_paths = sorted(schema_dir.glob("*.schema.json"))
    if len(schema_paths) != 3:
        _fail("expected three versioned SK-E02 schemas")
    for path in schema_paths:
        schema = json.loads(path.read_text(encoding="utf-8"))
        if (schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema"
                or schema.get("type") != "object" or schema.get("additionalProperties") is not False
                or not isinstance(schema.get("$id"), str) or not isinstance(schema.get("required"), list)
                or not isinstance(schema.get("properties"), dict)):
            _fail(f"invalid or open SK-E02 schema root: {path.name}")
    plan_path = FIXTURES / "prospective-evidence-plan.json"
    plan = json.loads(plan_path.read_text(encoding="utf-8"))
    adjudication = json.loads((FIXTURES / "synthetic-adjudication.json").read_text(encoding="utf-8"))
    errata = json.loads((FIXTURES / "r1-packet-errata.json").read_text(encoding="utf-8"))
    validate_evidence_plan(plan)
    validate_adjudication(adjudication)
    validate_errata(errata)
    hash_errors = verify_fixture_hashes(plan)
    if hash_errors:
        _fail("; ".join(hash_errors))
    if hashlib.sha256(plan_path.read_bytes()).hexdigest() != adjudication["evidence_plan_sha256"]:
        _fail("synthetic adjudication does not bind the exact evidence-plan bytes")
    if not (ROOT / "docs/research/evaluations/2026-11-13-gemini-r1/source_audit_R1.csv").is_file():
        _fail("historical R1 source audit reference is missing")
    negative_examples = (
        ("negative/invalid-multiple-evidence-modes.json", validate_evidence_plan),
        ("negative/invalid-generation-before-freeze.json", validate_evidence_plan),
        ("negative/invalid-unavailable-as-contradicted.json", validate_adjudication),
        ("negative/invalid-unavailable-as-supported.json", validate_adjudication),
        ("negative/invalid-r1-score-rewrite.json", validate_errata),
    )
    for relative, validator in negative_examples:
        candidate = json.loads((FIXTURES / relative).read_text(encoding="utf-8"))
        try:
            validator(candidate)
        except ContractError:
            continue
        _fail(f"invalid example unexpectedly passed: {relative}")


if __name__ == "__main__":
    if sys.argv[1:] not in ([], ["check"]):
        print("usage: python scripts/sk_e02_contract.py [check]", file=sys.stderr)
        raise SystemExit(2)
    try:
        run_fixture_checks()
    except (ContractError, OSError, json.JSONDecodeError) as error:
        print(f"SK-E02 contract check failed: {error}", file=sys.stderr)
        raise SystemExit(1)
    print("SK-E02 schemas, synthetic fixtures and integrity checks passed.")
