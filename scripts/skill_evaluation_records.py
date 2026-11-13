#!/usr/bin/env python3
"""Build and verify offline, blinded Storytelling skill-evaluation packets.

This tool handles research metadata only. It never calls a model, reads a Git
remote, or sends files anywhere. Paths in a record are relative to its bundle.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import random
import re
import secrets
import sys
from datetime import datetime
from pathlib import Path, PurePosixPath


RECORD_VERSION = "skill-evaluation-record/v1"
BLIND_VERSION = "skill-evaluation-blind/v1"
KEY_VERSION = "skill-evaluation-allocation/v1"
SHA256_RE = re.compile(r"^[a-f0-9]{64}$")
ID_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$")
TEXT_ID_RE = re.compile(r"^T-[23456789ABCDEFGHJKMNPQRSTVWXYZ]{6}$")
CONDITIONS = {"skill_on", "skill_off", "unknown"}
PROVENANCE = {"observed", "declared", "synthetic", "unknown"}


class RecordError(ValueError):
    """A stable validation failure suitable for a local research log."""

    def __init__(self, code: str, path: str, detail: str):
        self.code = code
        self.path = path
        self.detail = detail
        super().__init__(f"{code} at {path}: {detail}")


def _fail(code: str, path: str, detail: str):
    raise RecordError(code, path, detail)


def _json_no_duplicates(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            _fail("duplicate_field", f"/{key}", "JSON object repeats a field")
        result[key] = value
    return result


def _load_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=_json_no_duplicates)
    except RecordError:
        raise
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        _fail("invalid_json", path.name, str(error))


def _keys(value, expected, path):
    if not isinstance(value, dict):
        _fail("invalid_shape", path, "expected an object")
    missing = sorted(set(expected) - set(value))
    extra = sorted(set(value) - set(expected))
    if missing:
        _fail("missing_field", path, f"missing {', '.join(missing)}")
    if extra:
        _fail("unknown_field", path, f"unexpected {', '.join(extra)}")


def _string(value, path, *, allow_empty=False):
    if not isinstance(value, str) or (not allow_empty and not value):
        _fail("invalid_shape", path, "expected a non-empty string")


def _identifier(value, path):
    _string(value, path)
    if not ID_RE.fullmatch(value):
        _fail("invalid_id", path, "must use 1-64 portable ASCII letters, digits, dot, underscore or hyphen")


def _digest(value, path):
    if not isinstance(value, str) or not SHA256_RE.fullmatch(value):
        _fail("invalid_digest", path, "expected 64 lowercase hexadecimal SHA-256 digits")


def _relative_file(root: Path, value, path: str) -> tuple[Path, bytes]:
    _string(value, path)
    if "\\" in value:
        _fail("unsafe_path", path, "use a relative POSIX path, never a backslash")
    relative = PurePosixPath(value)
    if relative.is_absolute() or any(part in ("", ".", "..") for part in relative.parts):
        _fail("unsafe_path", path, "absolute, dot and parent paths are not allowed")
    resolved_root = root.resolve()
    resolved = (resolved_root / Path(*relative.parts)).resolve()
    if not resolved.is_relative_to(resolved_root):
        _fail("unsafe_path", path, "resolved path escapes the bundle")
    if not resolved.is_file():
        _fail("missing_file", path, f"bundle file does not exist: {value}")
    try:
        return resolved, resolved.read_bytes()
    except OSError as error:
        _fail("unreadable_file", path, str(error))


def _checked_file_ref(root, ref, path):
    _keys(ref, ("path", "sha256"), path)
    file_path, raw = _relative_file(root, ref["path"], path + "/path")
    _digest(ref["sha256"], path + "/sha256")
    actual = hashlib.sha256(raw).hexdigest()
    if actual != ref["sha256"]:
        _fail("hash_mismatch", path, f"recorded {ref['sha256']}, actual {actual}")
    try:
        return raw.decode("utf-8")
    except UnicodeDecodeError as error:
        _fail("invalid_utf8", path, str(error))


def _timestamp(value, path, record_kind):
    if value is None and record_kind == "synthetic_fixture":
        return
    _string(value, path)
    if not value.endswith("Z"):
        _fail("invalid_timestamp", path, "use an ISO-8601 UTC timestamp ending in Z")
    try:
        datetime.fromisoformat(value[:-1] + "+00:00")
    except ValueError as error:
        _fail("invalid_timestamp", path, str(error))


def _load_record(root: Path):
    record_path = root / "record.json"
    record = _load_json(record_path)
    _keys(record, ("schema_version", "evaluation_id", "record_kind", "cases"), "/")
    if record["schema_version"] != RECORD_VERSION:
        _fail("unsupported_version", "/schema_version", str(record["schema_version"]))
    _identifier(record["evaluation_id"], "/evaluation_id")
    if not isinstance(record["record_kind"], str) or record["record_kind"] not in (
            "observed_evaluation", "synthetic_fixture"):
        _fail("invalid_record_kind", "/record_kind", str(record["record_kind"]))
    if not isinstance(record["cases"], list) or not record["cases"]:
        _fail("invalid_shape", "/cases", "expected a non-empty array")

    case_ids = set()
    run_ids = set()
    generation_ids = set()
    text_ids = set()
    for case_index, case in enumerate(record["cases"]):
        case_path = f"/cases/{case_index}"
        _keys(case, ("case_id", "task", "audience", "packet_version", "generations"), case_path)
        _identifier(case["case_id"], case_path + "/case_id")
        if case["case_id"] in case_ids:
            _fail("duplicate_case_id", case_path + "/case_id", case["case_id"])
        case_ids.add(case["case_id"])
        for field in ("task", "audience", "packet_version"):
            _string(case[field], case_path + "/" + field)
        if not isinstance(case["generations"], list) or len(case["generations"]) < 2:
            _fail("invalid_pair", case_path + "/generations", "a blinded comparison needs at least two drafts")

        condition_counts = {"skill_on": 0, "skill_off": 0, "unknown": 0}
        shared_input_hashes = None
        for generation_index, generation in enumerate(case["generations"]):
            generation_path = f"{case_path}/generations/{generation_index}"
            _keys(generation, ("run_id", "generation_id", "text_id", "condition_provenance",
                              "model_runtime", "skill", "generated_at", "prompt", "evidence_packet",
                              "text"), generation_path)
            for field in ("run_id", "generation_id", "text_id"):
                _identifier(generation[field], generation_path + "/" + field)
            if not TEXT_ID_RE.fullmatch(generation["text_id"]):
                _fail("non_neutral_text_id", generation_path + "/text_id",
                      "use T- followed by six condition-neutral Crockford characters")
            for value, seen, code, field in (
                (generation["run_id"], run_ids, "duplicate_run_id", "run_id"),
                (generation["generation_id"], generation_ids, "duplicate_generation_id", "generation_id"),
                (generation["text_id"], text_ids, "duplicate_text_id", "text_id"),
            ):
                if value in seen:
                    _fail(code, generation_path + "/" + field, value)
                seen.add(value)

            provenance = generation["condition_provenance"]
            _keys(provenance, ("status", "condition", "evidence_ref"), generation_path + "/condition_provenance")
            if not isinstance(provenance["status"], str) or provenance["status"] not in PROVENANCE:
                _fail("invalid_provenance", generation_path + "/condition_provenance/status",
                      str(provenance["status"]))
            if not isinstance(provenance["condition"], str) or provenance["condition"] not in CONDITIONS:
                _fail("invalid_condition", generation_path + "/condition_provenance/condition",
                      str(provenance["condition"]))
            if ((provenance["status"] == "unknown") != (provenance["condition"] == "unknown")):
                _fail("condition_inferred", generation_path + "/condition_provenance",
                      "unknown status and unknown condition must appear together")
            if provenance["status"] == "synthetic" and record["record_kind"] != "synthetic_fixture":
                _fail("synthetic_as_observed", generation_path + "/condition_provenance/status",
                      "synthetic provenance belongs only in a synthetic_fixture")
            if record["record_kind"] == "synthetic_fixture" and provenance["status"] != "synthetic":
                _fail("fixture_provenance", generation_path + "/condition_provenance/status",
                      "synthetic fixtures must label every condition as synthetic")
            if provenance["status"] in ("observed", "declared", "synthetic"):
                _identifier(provenance["evidence_ref"], generation_path + "/condition_provenance/evidence_ref")
            elif provenance["evidence_ref"] is not None:
                _fail("invalid_provenance", generation_path + "/condition_provenance/evidence_ref",
                      "unknown provenance uses null evidence_ref")
            condition_counts[provenance["condition"]] += 1

            model = generation["model_runtime"]
            _keys(model, ("provider", "model", "version", "runtime", "settings"),
                  generation_path + "/model_runtime")
            for field in ("provider", "model", "version", "runtime"):
                if model[field] is not None:
                    _string(model[field], generation_path + "/model_runtime/" + field)
            if model["settings"] is not None and not isinstance(model["settings"], dict):
                _fail("invalid_shape", generation_path + "/model_runtime/settings", "expected an object or null")
            if model["settings"] is not None:
                def check_setting_keys(value, path):
                    if isinstance(value, dict):
                        for key, child in value.items():
                            if _SECRET_SETTING_KEY.search(str(key)):
                                _fail("sensitive_setting", path + "/" + str(key),
                                      "credentials and signed URLs must not be recorded")
                            check_setting_keys(child, path + "/" + str(key))
                    elif isinstance(value, list):
                        for index, child in enumerate(value):
                            check_setting_keys(child, path + "/" + str(index))
                check_setting_keys(model["settings"], generation_path + "/model_runtime/settings")

            skill = generation["skill"]
            if provenance["condition"] == "skill_on":
                _keys(skill, ("git_commit", "sha256"), generation_path + "/skill")
                if skill["git_commit"] is not None and not re.fullmatch(r"[a-f0-9]{40}", skill["git_commit"]):
                    _fail("invalid_digest", generation_path + "/skill/git_commit", "expected a full Git SHA-1")
                if skill["sha256"] is not None:
                    _digest(skill["sha256"], generation_path + "/skill/sha256")
                if provenance["status"] in ("observed", "declared") and (
                        skill["git_commit"] is None or skill["sha256"] is None):
                    _fail("missing_skill_identity", generation_path + "/skill",
                          "skill-on runs require the loaded skill commit and package digest")
            elif skill is not None:
                _fail("unexpected_skill_identity", generation_path + "/skill",
                      "only a skill-on generation may declare a skill identity")

            _timestamp(generation["generated_at"], generation_path + "/generated_at", record["record_kind"])
            _checked_file_ref(root, generation["prompt"], generation_path + "/prompt")
            _checked_file_ref(root, generation["evidence_packet"], generation_path + "/evidence_packet")
            input_hashes = (generation["prompt"]["sha256"], generation["evidence_packet"]["sha256"])
            if shared_input_hashes is None:
                shared_input_hashes = input_hashes
            elif shared_input_hashes != input_hashes:
                _fail("case_input_mismatch", generation_path,
                      "paired generations must use identical exact prompt and evidence-packet bytes")
            _checked_file_ref(root, generation["text"], generation_path + "/text")

        if condition_counts["skill_on"] != condition_counts["skill_off"]:
            _fail("unbalanced_conditions", case_path + "/generations",
                  f"skill_on={condition_counts['skill_on']} skill_off={condition_counts['skill_off']}")
    return record


def verify_bundle(bundle: Path | str):
    """Validate a record and every referenced input against raw UTF-8 bytes."""
    return _load_record(Path(bundle))


def _private_key(record, orders, seed):
    allocations = []
    for case, presented in zip(record["cases"], orders):
        position_by_id = {text_id: index + 1 for index, text_id in enumerate(presented)}
        for generation in case["generations"]:
            provenance = generation["condition_provenance"]
            skill = generation["skill"]
            allocations.append({
                "case_id": case["case_id"],
                "text_id": generation["text_id"],
                "run_id": generation["run_id"],
                "generation_id": generation["generation_id"],
                "condition": provenance["condition"],
                "provenance_status": provenance["status"],
                "evidence_ref": provenance["evidence_ref"],
                "skill_git_commit": skill["git_commit"] if skill else None,
                "skill_sha256": skill["sha256"] if skill else None,
                "presentation_position": position_by_id[generation["text_id"]],
            })
    return {
        "allocation_key_version": KEY_VERSION,
        "evaluation_id": record["evaluation_id"],
        "presentation_seed": seed,
        "allocations": allocations,
    }


def _guard_output_paths(bundle: Path, judge_out: Path, key_out: Path):
    judge_parent = judge_out.parent.resolve()
    key_parent = key_out.parent.resolve()
    resolved_bundle = bundle.resolve()
    if (judge_parent == key_parent or judge_parent.is_relative_to(key_parent)
            or key_parent.is_relative_to(judge_parent)):
        _fail("key_not_separated", "/outputs", "judge packet and private key must be written to different directories")
    if judge_out.resolve() == key_out.resolve():
        _fail("key_not_separated", "/outputs", "judge packet and private key paths must differ")
    if judge_parent.is_relative_to(resolved_bundle) or key_parent.is_relative_to(resolved_bundle):
        _fail("output_inside_bundle", "/outputs", "write generated packets outside the source bundle")
    if judge_out.exists() or key_out.exists():
        _fail("output_exists", "/outputs", "refusing to overwrite a prior packet or allocation key")
    if not judge_parent.is_dir() or not key_parent.is_dir():
        _fail("missing_output_directory", "/outputs", "create both output directories before preparation")


def prepare_blind_bundle(bundle: Path | str, judge_out: Path | str, key_out: Path | str,
                         seed: int | None = None):
    """Write a condition-free judge packet and a separate private allocation key."""
    root = Path(bundle)
    judge_path = Path(judge_out)
    key_path = Path(key_out)
    _guard_output_paths(root, judge_path, key_path)
    record = verify_bundle(root)
    actual_seed = secrets.randbits(64) if seed is None else seed
    if not isinstance(actual_seed, int) or isinstance(actual_seed, bool) or actual_seed < 0:
        _fail("invalid_seed", "/presentation_seed", "seed must be a non-negative integer")
    rng = random.Random(actual_seed)

    packet_cases = []
    orders = []
    for case in record["cases"]:
        case_path = "/cases/" + case["case_id"]
        first_generation = case["generations"][0]
        prompt_text = _checked_file_ref(root, first_generation["prompt"], case_path + "/prompt")
        evidence_text = _checked_file_ref(root, first_generation["evidence_packet"], case_path + "/evidence_packet")
        texts = []
        for generation_index, generation in enumerate(case["generations"]):
            _, text_bytes = _relative_file(
                root, generation["text"]["path"],
                f"/cases/{case['case_id']}/generations/{generation_index}/text/path")
            texts.append({
                "text_id": generation["text_id"],
                "text": text_bytes.decode("utf-8"),
            })
        rng.shuffle(texts)
        orders.append([text["text_id"] for text in texts])
        packet_cases.append({
            "case_id": case["case_id"],
            "task": case["task"],
            "audience": case["audience"],
            "packet_version": case["packet_version"],
            "prompt": prompt_text,
            "evidence_packet": evidence_text,
            "drafts": texts,
        })
    packet = {
        "blind_packet_version": BLIND_VERSION,
        "evaluation_id": record["evaluation_id"],
        "cases": packet_cases,
    }
    key = _private_key(record, orders, actual_seed)
    validate_blind_packet(packet)
    validate_allocation_key(record, key)
    judge_created = False
    key_created = False
    try:
        with judge_path.open("x", encoding="utf-8", newline="\n") as stream:
            judge_created = True
            stream.write(json.dumps(packet, ensure_ascii=False, indent=2) + "\n")
        with key_path.open("x", encoding="utf-8", newline="\n") as stream:
            key_created = True
            stream.write(json.dumps(key, ensure_ascii=False, indent=2) + "\n")
    except OSError:
        if judge_created:
            judge_path.unlink(missing_ok=True)
        if key_created:
            key_path.unlink(missing_ok=True)
        raise
    return packet, key


_BLIND_FORBIDDEN = {
    "allocation", "allocation_key", "condition", "condition_provenance", "evidence_ref",
    "generation_id", "git_commit", "model", "model_runtime", "prior_rating", "prior_ratings",
    "prior_score", "prior_scores", "provider", "provenance_status", "rating", "ratings",
    "run_id", "score", "scores", "seed", "settings", "skill", "skill_commit",
    "skill_git_commit", "skill_sha256", "sha256", "text_sha256", "version", "runtime",
}
_SECRET_SETTING_KEY = re.compile(
    r"(?:api[_-]?key|access[_-]?token|refresh[_-]?token|auth[_-]?token|authorization|"
    r"credential|secret|password|signed[_-]?url|private[_-]?key)", re.I
)


def validate_blind_packet(packet):
    """Fail closed if a public judge packet contains allocation or score metadata."""
    def visit(value, path="/"):
        if isinstance(value, dict):
            for key, child in value.items():
                normalized = re.sub(r"[^a-z0-9_]", "", str(key).lower())
                if normalized in _BLIND_FORBIDDEN:
                    code = "score_leak" if "score" in normalized or "rating" in normalized else "allocation_leak"
                    _fail(code, path + str(key), "judge packet contains private evaluation metadata")
                visit(child, path + str(key) + "/")
        elif isinstance(value, list):
            for index, child in enumerate(value):
                visit(child, path + str(index) + "/")
    visit(packet)
    _keys(packet, ("blind_packet_version", "evaluation_id", "cases"), "/")
    if packet["blind_packet_version"] != BLIND_VERSION:
        _fail("unsupported_version", "/blind_packet_version", str(packet["blind_packet_version"]))
    _identifier(packet["evaluation_id"], "/evaluation_id")
    if not isinstance(packet["cases"], list) or not packet["cases"]:
        _fail("invalid_shape", "/cases", "expected a non-empty array")
    case_ids = set()
    for case_index, case in enumerate(packet["cases"]):
        case_path = f"/cases/{case_index}"
        _keys(case, ("case_id", "task", "audience", "packet_version", "prompt", "evidence_packet", "drafts"), case_path)
        _identifier(case["case_id"], case_path + "/case_id")
        if case["case_id"] in case_ids:
            _fail("duplicate_case_id", case_path + "/case_id", case["case_id"])
        case_ids.add(case["case_id"])
        for field in ("task", "audience", "packet_version", "prompt", "evidence_packet"):
            _string(case[field], case_path + "/" + field, allow_empty=True)
        if not isinstance(case["drafts"], list) or len(case["drafts"]) < 2:
            _fail("invalid_pair", case_path + "/drafts", "expected at least two blind drafts")
        seen_ids = set()
        for index, draft in enumerate(case["drafts"]):
            draft_path = f"{case_path}/drafts/{index}"
            _keys(draft, ("text_id", "text"), draft_path)
            if not isinstance(draft["text_id"], str) or not TEXT_ID_RE.fullmatch(draft["text_id"]):
                _fail("non_neutral_text_id", draft_path + "/text_id", str(draft["text_id"]))
            if draft["text_id"] in seen_ids:
                _fail("duplicate_text_id", draft_path + "/text_id", draft["text_id"])
            seen_ids.add(draft["text_id"])
            _string(draft["text"], draft_path + "/text", allow_empty=True)
    # Strict field allowlists above reject any other unversioned packet data.
    return packet


def validate_allocation_key(record, key):
    """Confirm a private key is a complete, exact map of the private record."""
    _keys(key, ("allocation_key_version", "evaluation_id", "presentation_seed", "allocations"), "/")
    if key["allocation_key_version"] != KEY_VERSION or key["evaluation_id"] != record["evaluation_id"]:
        _fail("allocation_mismatch", "/", "version or evaluation ID differs from the record")
    seed = key["presentation_seed"]
    if not isinstance(seed, int) or isinstance(seed, bool) or seed < 0:
        _fail("allocation_mismatch", "/presentation_seed", "invalid seed")
    if not isinstance(key["allocations"], list):
        _fail("allocation_mismatch", "/allocations", "expected an array")
    expected = {}
    expected_positions = {}
    rng = random.Random(seed)
    for case in record["cases"]:
        order = [generation["text_id"] for generation in case["generations"]]
        rng.shuffle(order)
        expected_positions.update({(case["case_id"], text_id): position
                                   for position, text_id in enumerate(order, start=1)})
        for generation in case["generations"]:
            expected[(case["case_id"], generation["text_id"])] = generation
    actual = {}
    for index, allocation in enumerate(key["allocations"]):
        path = f"/allocations/{index}"
        required = ("case_id", "text_id", "run_id", "generation_id", "condition", "provenance_status",
                    "evidence_ref", "skill_git_commit", "skill_sha256", "presentation_position")
        _keys(allocation, required, path)
        pair = (allocation["case_id"], allocation["text_id"])
        if pair in actual:
            _fail("allocation_mismatch", path, "duplicate case/text mapping")
        actual[pair] = allocation
    if set(actual) != set(expected):
        _fail("allocation_mismatch", "/allocations", "case/text IDs do not match the private generation record")
    for pair, generation in expected.items():
        allocation = actual[pair]
        provenance = generation["condition_provenance"]
        skill = generation["skill"]
        wanted = {
            "run_id": generation["run_id"],
            "generation_id": generation["generation_id"],
            "condition": provenance["condition"],
            "provenance_status": provenance["status"],
            "evidence_ref": provenance["evidence_ref"],
            "skill_git_commit": skill["git_commit"] if skill else None,
            "skill_sha256": skill["sha256"] if skill else None,
        }
        if any(allocation[field] != value for field, value in wanted.items()):
            _fail("allocation_mismatch", "/allocations", f"private mapping differs for {pair[0]}/{pair[1]}")
        position = allocation["presentation_position"]
        if not isinstance(position, int) or isinstance(position, bool) or position < 1:
            _fail("allocation_mismatch", "/allocations/presentation_position", "expected a positive integer")
        if position != expected_positions[pair]:
            _fail("allocation_mismatch", "/allocations/presentation_position",
                  f"presentation order does not match recorded seed for {pair[0]}/{pair[1]}")
    for case in record["cases"]:
        positions = [actual[(case["case_id"], generation["text_id"])]["presentation_position"]
                     for generation in case["generations"]]
        if sorted(positions) != list(range(1, len(positions) + 1)):
            _fail("allocation_mismatch", "/allocations/presentation_position", "positions must be a complete order per case")
    return key


def hash_skill_tree(directory: Path | str):
    """Hash exact package paths and bytes using the documented v1 framing."""
    root = Path(directory).resolve()
    if not root.is_dir():
        _fail("missing_directory", "/directory", "skill path is not a directory")
    digest = hashlib.sha256(b"storytelling-skill-tree/v1\0")
    entries = sorted(root.rglob("*"))
    for path in entries:
        if path.is_symlink():
            _fail("unsafe_path", path.relative_to(root).as_posix(), "symlinks are not allowed in skill-tree hashes")
    files = [path for path in entries if path.is_file()]
    if not files:
        _fail("empty_directory", str(directory), "skill directory has no files")
    for path in files:
        relative = path.relative_to(root).as_posix().encode("utf-8")
        raw = path.read_bytes()
        digest.update(len(relative).to_bytes(8, "big"))
        digest.update(relative)
        digest.update(len(raw).to_bytes(8, "big"))
        digest.update(raw)
    return digest.hexdigest()


def _main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    verify = sub.add_parser("verify", help="verify a bundle's schema, IDs and raw-byte hashes")
    verify.add_argument("bundle", type=Path)
    prepare = sub.add_parser("prepare-blind", help="write a blind packet and separate private key")
    prepare.add_argument("bundle", type=Path)
    prepare.add_argument("--judge-out", required=True, type=Path)
    prepare.add_argument("--key-out", required=True, type=Path)
    prepare.add_argument("--seed", type=int, help="optional reproducible non-negative presentation seed")
    check_packet = sub.add_parser("verify-blind", help="verify that an existing packet has no private fields")
    check_packet.add_argument("packet", type=Path)
    check_key = sub.add_parser("verify-allocation", help="match a private key against the record bundle")
    check_key.add_argument("bundle", type=Path)
    check_key.add_argument("key", type=Path)
    hash_tree = sub.add_parser("hash-skill-tree", help="hash every file path and byte in a skill package")
    hash_tree.add_argument("directory", type=Path)
    args = parser.parse_args(argv)
    try:
        if args.command == "verify":
            record = verify_bundle(args.bundle)
            print(f"OK: {record['evaluation_id']} ({sum(len(c['generations']) for c in record['cases'])} generations)")
        elif args.command == "prepare-blind":
            packet, _ = prepare_blind_bundle(args.bundle, args.judge_out, args.key_out, args.seed)
            print(f"OK: {len(packet['cases'])} cases prepared in the requested separate output directories.")
            print("The presentation seed is stored only in the private allocation key.")
        elif args.command == "verify-blind":
            packet = _load_json(args.packet)
            validate_blind_packet(packet)
            print("OK: packet contains no condition or score metadata")
        elif args.command == "verify-allocation":
            record = verify_bundle(args.bundle)
            key = _load_json(args.key)
            validate_allocation_key(record, key)
            print(f"OK: {len(key['allocations'])} private mappings match the record")
        elif args.command == "hash-skill-tree":
            print(hash_skill_tree(args.directory))
        else:
            raise AssertionError("unhandled command")
        return 0
    except RecordError as error:
        print(str(error), file=sys.stderr)
        return 2
    except OSError as error:
        print(f"io_error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(_main())
