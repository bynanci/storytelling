#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Offline SK-E06 layout snapshot integrity; never execution authority."""

from collections import Counter
import hashlib
import io
import json
import os
from pathlib import Path
import posixpath
import re
import stat
import zipfile
import zlib


MAX_RECORD_BYTES = 1024 * 1024
MAX_ARCHIVE_BYTES = 2 * 1024 * 1024
MAX_EXPANDED_BYTES = 4 * 1024 * 1024
MAX_MEMBER_BYTES = 1024 * 1024
MAX_MEMBERS = 256


class LayoutError(ValueError):
    """The prospective snapshot cannot be verified within the layout scope."""


def _require(condition):
    if not condition:
        raise LayoutError("Layout snapshot integrity check failed.")


def _member_path(name):
    _require(isinstance(name, str) and name.startswith("skills/storytelling/"))
    parts = name.split("/")
    _require(all(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]*", part)
                 and not part.endswith(".")
                 and not re.fullmatch(r"CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9]", part.split(".")[0], re.I)
                 for part in parts))


def _unique_object(pairs):
    value = {}
    for key, item in pairs:
        _require(key not in value)
        value[key] = item
    return value


def _nonfinite(_value):
    raise LayoutError("Layout snapshot integrity check failed.")


def _parse_record(raw):
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=_unique_object, parse_constant=_nonfinite)
    except (UnicodeError, ValueError, RecursionError) as error:
        raise LayoutError("Layout snapshot integrity check failed.") from error


def _keys(value, keys):
    _require(isinstance(value, dict) and set(value) == set(keys))


def _text(value):
    _require(isinstance(value, str) and bool(value.strip()))


def _hash(value, length):
    _require(isinstance(value, str) and re.fullmatch(r"[0-9a-f]{" + str(length) + r"}", value))


def _integer(value):
    _require(type(value) is int and value >= 0)


def _record(raw):
    _require(type(raw) is bytes and len(raw) <= MAX_RECORD_BYTES)
    record = _parse_record(raw)
    _keys(record, {"record_kind", "version", "prepared_at_utc", "follow_up", "predecessor_run", "purpose",
                   "skill_commit", "archive", "mechanical_preflight", "original_case_hashes", "execution_records",
                   "raw_responses", "model_settings", "instruction_conformance", "human_trial_outcomes",
                   "narrative_quality", "human_participants", "host_editor_observations", "dispatch_status", "decision"})
    _require(record["record_kind"] == "prospective_prepared_packet_not_executed"
             and record["version"] == "SK-E06-LAYOUT-PREFLIGHT-2" and record["follow_up"] == "SK-E06-F02")
    for key in ("prepared_at_utc", "predecessor_run", "purpose", "dispatch_status"):
        _text(record[key])
    _hash(record["skill_commit"], 40)
    archive = record["archive"]
    _keys(archive, {"path", "sha256", "bytes", "files"})
    _require(isinstance(archive["path"], str) and re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]*\.zip", archive["path"]))
    _hash(archive["sha256"], 64)
    _integer(archive["bytes"])
    _require(archive["bytes"] <= MAX_ARCHIVE_BYTES)
    _require(isinstance(archive["files"], list) and 0 < len(archive["files"]) <= MAX_MEMBERS)
    for item in archive["files"]:
        _keys(item, {"path", "sha256", "git_blob", "bytes"})
        _member_path(item["path"])
        _hash(item["sha256"], 64)
        _hash(item["git_blob"], 40)
        _integer(item["bytes"])
        _require(item["bytes"] <= MAX_MEMBER_BYTES)
    _require(sum(item["bytes"] for item in archive["files"]) <= MAX_EXPANDED_BYTES)
    mechanical = record["mechanical_preflight"]
    _keys(mechanical, {"ordinary_git_blobs_only", "exact_git_bytes", "symlinks", "relative_markdown_links_resolved",
                       "links", "unavailable_linked_guides", "scope_limit"})
    _require(mechanical["ordinary_git_blobs_only"] is True and mechanical["exact_git_bytes"] is True
             and mechanical["symlinks"] is False and mechanical["unavailable_linked_guides"] == [])
    _integer(mechanical["relative_markdown_links_resolved"])
    _text(mechanical["scope_limit"])
    _require(isinstance(mechanical["links"], list))
    for link in mechanical["links"]:
        _keys(link, {"source", "target", "resolved"})
        _member_path(link["source"])
        _member_path(link["resolved"])
        _text(link["target"])
    cases = record["original_case_hashes"]
    _require(isinstance(cases, list) and len(cases) == 5)
    for case in cases:
        _keys(case, {"case_id", "pattern", "input_sha256", "reference_case_sha256", "record_id"})
        for key in ("case_id", "pattern", "record_id"):
            _text(case[key])
        _hash(case["input_sha256"], 64)
        _hash(case["reference_case_sha256"], 64)
    _require(len({case["case_id"] for case in cases}) == 5)
    _require(record["execution_records"] == [] and record["raw_responses"] == [])
    for key in ("model_settings", "instruction_conformance", "human_trial_outcomes", "narrative_quality", "decision"):
        _require(record[key] is None)
    for key in ("human_participants", "host_editor_observations"):
        _require(type(record[key]) is int and record[key] == 0)
    return record


def _validate_layout(record_bytes, archive_bytes):
    _require(type(archive_bytes) is bytes and len(archive_bytes) <= MAX_ARCHIVE_BYTES)
    record = _record(record_bytes)
    _require(hashlib.sha256(archive_bytes).hexdigest() == record["archive"]["sha256"])
    _require(len(archive_bytes) == record["archive"]["bytes"])
    local_links = []
    with zipfile.ZipFile(io.BytesIO(archive_bytes)) as archive:
        members = archive.infolist()
        _require(len(members) <= MAX_MEMBERS
                 and sum(member.file_size for member in members) <= MAX_EXPANDED_BYTES)
        for member in members:
            _require(member.orig_filename == member.filename)
            _member_path(member.filename)
            _require(member.compress_type in {zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED})
            _require(0 <= member.file_size <= MAX_MEMBER_BYTES and 0 <= member.compress_size <= MAX_ARCHIVE_BYTES)
            _require(stat.S_IFMT(member.external_attr >> 16) in {0, stat.S_IFREG}
                     and not member.external_attr & 0x10 and not member.flag_bits & 0x41)
        _require(len({member.filename.lower() for member in members}) == len(members))
        _require(len({item["path"].lower() for item in record["archive"]["files"]}) == len(record["archive"]["files"]))
        _require({member.filename for member in members} == {item["path"] for item in record["archive"]["files"]})
        declared = {item["path"]: item for item in record["archive"]["files"]}
        for member in members:
            with archive.open(member) as stream:
                raw = stream.read(MAX_MEMBER_BYTES + 1)
            _require(len(raw) <= MAX_MEMBER_BYTES and len(raw) == member.file_size)
            item = declared[member.filename]
            _require(len(raw) == item["bytes"])
            _require(hashlib.sha256(raw).hexdigest() == item["sha256"])
            _require(hashlib.sha1(b"blob " + str(len(raw)).encode("ascii") + b"\0" + raw).hexdigest() == item["git_blob"])
            if member.filename.endswith(".md"):
                text = raw.decode("utf-8")
                for match in re.finditer(r"\[[^\[\]\r\n]*\]\(([^()\[\]\s]+)(?:[ \t]+[^()\[\]\r\n]*)?\)", text):
                    target = match.group(1)
                    if not target.startswith(("#", "//")) and not re.match(r"[A-Za-z][A-Za-z0-9+.-]*:", target):
                        resolved = posixpath.normpath(posixpath.join(posixpath.dirname(member.filename), target.split("#")[0]))
                        _require(resolved in declared)
                        local_links.append((member.filename, target, resolved))
    mechanical = record["mechanical_preflight"]
    declared_links = [(link["source"], link["target"], link["resolved"]) for link in mechanical["links"]]
    _require(Counter(local_links) == Counter(declared_links)
             and len(local_links) == mechanical["relative_markdown_links_resolved"])
    return {
        "scope": "layout_snapshot_integrity_only",
        "file_count": len(members),
        "local_link_count": len(local_links),
        "can_dispatch": False,
        "source_commit_membership_verified": False,
        "observed_model_executions": 0,
        "human_participants": 0,
        "host_editor_observations": 0,
    }


def validate_layout(record_bytes, archive_bytes):
    """Check supplied immutable bytes without extracting or executing files.

    Git blob hashes establish consistency with declarations, not membership in
    the declared source commit, authenticity, conformance or launch authority.
    Link scanning covers simple inline Markdown destinations in the retained
    v2 style, not full CommonMark, HTML, reference-style or heading validation.
    """
    try:
        return _validate_layout(record_bytes, archive_bytes)
    except (ValueError, UnicodeError, OSError, RuntimeError, EOFError, NotImplementedError,
            zipfile.BadZipFile, zipfile.LargeZipFile, zlib.error) as error:
        raise LayoutError("Layout snapshot integrity check failed.") from error


def _read_snapshot(path, limit):
    before = path.lstat()
    _require(stat.S_ISREG(before.st_mode) and 0 <= before.st_size <= limit)
    flags = os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0) | getattr(os, "O_NONBLOCK", 0) | getattr(os, "O_BINARY", 0)
    with os.fdopen(os.open(path, flags), "rb") as stream:
        opened = os.fstat(stream.fileno())
        _require(stat.S_ISREG(opened.st_mode) and 0 <= opened.st_size <= limit
                 and (before.st_dev, before.st_ino) == (opened.st_dev, opened.st_ino))
        raw = stream.read(limit + 1)
    _require(len(raw) <= limit and len(raw) == opened.st_size)
    return raw


def preflight_directory(directory):
    """Read one bounded regular-file snapshot of each input.

    Static symlinks/nonregular files are rejected. This is not a qualified OS
    stable-read-view check or a guarantee against concurrent ancestor changes.
    """
    try:
        directory = Path(directory)
        _require(stat.S_ISDIR(directory.lstat().st_mode))
        record_path = directory / "future-run-layout-v2.json"
        record_bytes = _read_snapshot(record_path, MAX_RECORD_BYTES)
        record = _record(record_bytes)
        archive_path = directory / record["archive"]["path"]
        archive_bytes = _read_snapshot(archive_path, MAX_ARCHIVE_BYTES)
        return validate_layout(record_bytes, archive_bytes)
    except (OSError, ValueError, RuntimeError) as error:
        raise LayoutError("Layout snapshot integrity check failed.") from error
