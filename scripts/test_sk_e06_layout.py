#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Offline layout snapshot checks; never dispatch a model or open media."""

import copy
import hashlib
import io
import json
import os
import struct
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
import warnings
import zipfile
from unittest import mock

import sk_e06_layout
from sk_e06_layout import LayoutError, validate_layout


ROOT = Path(__file__).resolve().parents[1]
RECORD = ROOT / "docs/research/evaluations/sk-e06-runs-2026-11-17"
CLI = ROOT / "scripts/sk_e06_packet.py"


class LayoutTests(unittest.TestCase):
    def setUp(self):
        self.record_bytes = (RECORD / "future-run-layout-v2.json").read_bytes()
        self.record = json.loads(self.record_bytes)
        self.archive_bytes = (RECORD / "future-run-skill-layout-v2.zip").read_bytes()

    def encoded(self, record):
        return json.dumps(record, ensure_ascii=False).encode("utf-8")

    def rewritten_archive(self, entries, *, declare=True, links=None, compression=zipfile.ZIP_DEFLATED):
        """Build real ZIP input; declarations describe only the given bytes."""
        output = io.BytesIO()
        with zipfile.ZipFile(output, "w", compression=compression) as archive:
            for name, data in entries:
                member = zipfile.ZipInfo(name)
                member.external_attr = 0o100644 << 16
                member.compress_type = compression
                archive.writestr(member, data)
        raw = output.getvalue()
        record = copy.deepcopy(self.record)
        record["archive"]["sha256"] = hashlib.sha256(raw).hexdigest()
        record["archive"]["bytes"] = len(raw)
        if links is not None:
            record["mechanical_preflight"].update(links=links, relative_markdown_links_resolved=len(links))
        if declare:
            record["archive"]["files"] = [
                {"path": name, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data),
                 "git_blob": hashlib.sha1(b"blob " + str(len(data)).encode("ascii") + b"\0" + data).hexdigest()}
                for name, data in entries
            ]
        return record, raw

    def entries(self):
        with zipfile.ZipFile(io.BytesIO(self.archive_bytes)) as archive:
            return [(item.filename, archive.read(item)) for item in archive.infolist()]

    def test_normal_cli_checks_the_retained_layout_without_authorizing_dispatch(self):
        # A missing flag or a declaration-only count must not replace a real
        # archive check. These counts come from the retained 28-file fixture.
        argv = [sys.executable, str(CLI), "--layout-preflight"]
        process = subprocess.run(argv, capture_output=True, text=True)
        self.assertEqual(process.returncode, 0,
                         f"argv={argv!r}\nstdout={process.stdout!r}\nstderr={process.stderr!r}")
        self.assertEqual(process.stderr, "")
        self.assertEqual(json.loads(process.stdout), {
            "scope": "layout_snapshot_integrity_only",
            "file_count": 28,
            "local_link_count": 79,
            "can_dispatch": False,
            "source_commit_membership_verified": False,
            "observed_model_executions": 0,
            "human_participants": 0,
            "host_editor_observations": 0,
        })

    def test_rejects_changed_archive_checksum_before_trusting_its_layout(self):
        # Removing the byte digest comparison would accept this altered ZIP.
        with self.assertRaises(LayoutError):
            validate_layout(self.record_bytes, self.archive_bytes + b"changed")

    def test_rejects_archive_length_disagreement(self):
        # Trusting the digest alone would ignore this false byte declaration.
        self.record["archive"]["bytes"] += 1
        with self.assertRaises(LayoutError):
            validate_layout(self.encoded(self.record), self.archive_bytes)

    def test_rejects_missing_or_extra_zip_members_even_with_updated_archive_hash(self):
        # A declaration-only check would miss a flattened layout or missing guide.
        original = self.entries()
        for entries in ([(Path(name).name, data) for name, data in original],
                        original[1:], original + [("skills/storytelling/extra.txt", b"extra")]):
            with self.subTest(names=[name for name, _data in entries]):
                record, raw = self.rewritten_archive(entries, declare=False)
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), raw)

    def test_rejects_member_content_hash_blob_and_length_disagreements(self):
        # Omitting any per-file comparison would trust its false declaration.
        for field, value in (("sha256", "0" * 64), ("git_blob", "0" * 40), ("bytes", 1065)):
            with self.subTest(field=field):
                record = copy.deepcopy(self.record)
                record["archive"]["files"][0][field] = value
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), self.archive_bytes)

    def test_rejects_duplicate_or_case_colliding_members_and_declarations(self):
        # Collapsing names into a set or dict must not hide a second member.
        original = self.entries()
        for name in (original[0][0], "skills/storytelling/license"):
            with self.subTest(name=name), warnings.catch_warnings():
                warnings.simplefilter("ignore", UserWarning)
                record, raw = self.rewritten_archive(original + [(name, original[0][1])],
                                                     declare=name != original[0][0])
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), raw)
        record = copy.deepcopy(self.record)
        record["archive"]["files"].append(copy.deepcopy(record["archive"]["files"][0]))
        with self.assertRaises(LayoutError):
            validate_layout(self.encoded(record), self.archive_bytes)

    def test_rejects_dangerous_member_paths_even_when_fully_declared(self):
        # Matching declarations do not make traversal or ambiguous paths safe.
        names = ["../outside", "/skills/storytelling/absolute", "C:/outside",
                 "skills/storytelling/../outside", "skills/storytelling/./dot",
                 "skills/storytelling//empty", "skills/storytelling/trailing/",
                 "skills\\storytelling\\backslash", "skills/storytelling/nonascii-é",
                 "skills/elsewhere/file", "skills/storytelling/NUL.txt",
                 "skills/storytelling/trailing.", "skills/storytelling/a%2fb"]
        for name in names:
            with self.subTest(name=name):
                record, raw = self.rewritten_archive([(name, b"data")], links=[])
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), raw)

    def test_rejects_nonregular_and_encrypted_zip_entries(self):
        # ZIP metadata must not turn a declared file into a link, device or FIFO.
        for mode in (0o120777, 0o040755, 0o010644, 0o020644, 0o060644, 0o140644):
            with self.subTest(mode=oct(mode)):
                record, raw = self.rewritten_archive([("skills/storytelling/file", b"data")], links=[])
                changed = bytearray(raw)
                central = changed.index(b"PK\x01\x02")
                struct.pack_into("<I", changed, central + 38, mode << 16)
                raw = bytes(changed)
                record["archive"].update(sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), raw)
        record, raw = self.rewritten_archive([("skills/storytelling/file", b"data")], links=[])
        changed = bytearray(raw)
        struct.pack_into("<H", changed, 6, 1)
        central = changed.index(b"PK\x01\x02")
        struct.pack_into("<H", changed, central + 8, 1)
        raw = bytes(changed)
        record["archive"].update(sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))
        with self.assertRaises(LayoutError):
            validate_layout(self.encoded(record), raw)

    def test_rejects_duplicate_json_fields_and_nonfinite_constants(self):
        # Permissive JSON would hide the first declaration or accept NaN.
        duplicate = self.record_bytes.replace(b'"record_kind":', b'"record_kind":"wrong","record_kind":', 1)
        nested_duplicate = self.record_bytes.replace(b'"sha256":', b'"sha256":"wrong","sha256":', 1)
        for raw in (duplicate, nested_duplicate,
                    *(self.record_bytes.replace(b'"model_settings": null', b'"model_settings": ' + value)
                      for value in (b"NaN", b"Infinity", b"-Infinity"))):
            with self.subTest(raw_prefix=raw[:60]), self.assertRaises(LayoutError):
                validate_layout(raw, self.archive_bytes)

    def test_rejects_wrong_record_identity_or_field_types(self):
        # Python's bool/int equality and unchecked metadata must not authorize
        # a malformed prospective record.
        mutations = [
            (("record_kind",), "observed_run"), (("version",), "future"),
            (("follow_up",), "other"), (("skill_commit",), "unknown"),
            (("prepared_at_utc",), []), (("purpose",), 7),
            (("archive", "bytes"), 101442.0), (("archive", "sha256"), None),
            (("archive", "path"), "../outside.zip"),
            (("mechanical_preflight", "ordinary_git_blobs_only"), 1),
            (("mechanical_preflight", "exact_git_bytes"), False),
            (("mechanical_preflight", "symlinks"), 0),
            (("mechanical_preflight", "relative_markdown_links_resolved"), 79.0),
            (("mechanical_preflight", "unavailable_linked_guides"), ["missing"]),
            (("archive", "files", 0, "bytes"), 1064.0),
            (("archive", "files", 0, "git_blob"), []),
            (("original_case_hashes", 0, "input_sha256"), "unknown"),
        ]
        for path, value in mutations:
            with self.subTest(path=path):
                record = copy.deepcopy(self.record)
                cursor = record
                for part in path[:-1]:
                    cursor = cursor[part]
                cursor[path[-1]] = value
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), self.archive_bytes)
        for record in ([], None, {}, {**self.record, "unexpected": None}):
            with self.subTest(record_type=type(record).__name__), self.assertRaises(LayoutError):
                validate_layout(self.encoded(record), self.archive_bytes)

    def test_rejects_prepopulated_execution_or_human_evidence(self):
        # A prospective layout must not silently relabel supplied observations
        # as the zero executions reported by this offline command.
        for key, value in (
            ("execution_records", [{}]), ("raw_responses", ["invented"]),
            ("model_settings", {}), ("instruction_conformance", "pass"),
            ("human_trial_outcomes", []), ("narrative_quality", 1), ("decision", "approved"),
            ("human_participants", 1), ("host_editor_observations", 1),
            ("human_participants", False), ("host_editor_observations", 0.0),
        ):
            with self.subTest(key=key, value=value):
                record = copy.deepcopy(self.record)
                record[key] = value
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), self.archive_bytes)


    def test_rejects_declared_links_that_disagree_with_archive_markdown(self):
        # Checking only declared count/path existence misses a false source,
        # target, resolved path, omitted occurrence, or count.
        for mutation in ("count", "remove", "duplicate", "target", "source", "resolved"):
            with self.subTest(mutation=mutation):
                record = copy.deepcopy(self.record)
                mechanical = record["mechanical_preflight"]
                if mutation == "count":
                    mechanical["relative_markdown_links_resolved"] = 80
                elif mutation == "remove":
                    mechanical["links"].pop()
                elif mutation == "duplicate":
                    mechanical["links"].append(copy.deepcopy(mechanical["links"][0]))
                else:
                    mechanical["links"][0][mutation] = {
                        "source": "skills/storytelling/SKILL.zh-TW.md", "target": "SKILL.md",
                        "resolved": "skills/storytelling/SKILL.md",
                    }[mutation]
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), self.archive_bytes)


    def test_rejects_consistently_declared_link_to_missing_guide(self):
        # Agreement with the manifest is insufficient when no target exists.
        record, raw = self.rewritten_archive([("skills/storytelling/SKILL.md", b"[guide](missing.md)\n")])
        record["mechanical_preflight"].update(relative_markdown_links_resolved=1, links=[{
            "source": "skills/storytelling/SKILL.md", "target": "missing.md",
            "resolved": "skills/storytelling/missing.md",
        }])
        with self.assertRaises(LayoutError):
            validate_layout(self.encoded(record), raw)


    def test_counts_duplicate_local_links_and_ignores_external_mail_and_anchor_only_links(self):
        # Treating links as a set, or treating an external scheme as a local
        # file, would reject this independently specified two-link snapshot.
        text = (b"[one](references/guide.md#unverified-heading)\n"
                b"[two](references/guide.md#unverified-heading)\n"
                b"[web](https://example.invalid/x) [mail](MAILTO:example@example.invalid)\n"
                b"[anchor](#heading) [web](//example.invalid/x) [urn](urn:example:item)\n")
        record, raw = self.rewritten_archive([
            ("skills/storytelling/SKILL.md", text), ("skills/storytelling/references/guide.md", b"guide\n"),
        ])
        link = {"source": "skills/storytelling/SKILL.md", "target": "references/guide.md#unverified-heading",
                "resolved": "skills/storytelling/references/guide.md"}
        record["mechanical_preflight"].update(relative_markdown_links_resolved=2, links=[link, copy.deepcopy(link)])
        try:
            result = validate_layout(self.encoded(record), raw)
        except LayoutError:
            self.fail("a complete two-link snapshot must ignore external/mail/anchor-only destinations")
        self.assertEqual(result["local_link_count"], 2)
        self.assertEqual(result["file_count"], 2)
        record["mechanical_preflight"].update(relative_markdown_links_resolved=1, links=[link])
        with self.assertRaises(LayoutError):
            validate_layout(self.encoded(record), raw)


    def test_enforces_manifest_compressed_member_total_and_member_count_limits(self):
        # Accepting these internally consistent bytes would exceed one of the
        # offline check's resource bounds, without relying on a bad checksum.
        oversized_manifest = self.record_bytes + b" " * (1048577 - len(self.record_bytes))
        with self.subTest(bound="manifest"), self.assertRaises(LayoutError):
            validate_layout(oversized_manifest, self.archive_bytes)
        fixtures = [
            ("compressed", [(f"skills/storytelling/file{i}", b"x" * 700000) for i in range(3)], zipfile.ZIP_STORED),
            ("member", [("skills/storytelling/file", b"x" * 1048577)], zipfile.ZIP_DEFLATED),
            ("total", [(f"skills/storytelling/file{i}", b"x" * 900000) for i in range(5)], zipfile.ZIP_DEFLATED),
            ("count", [(f"skills/storytelling/file{i}", b"") for i in range(257)], zipfile.ZIP_DEFLATED),
        ]
        for bound, entries, compression in fixtures:
            record, raw = self.rewritten_archive(entries, links=[], compression=compression)
            with self.subTest(bound=bound), self.assertRaises(LayoutError):
                validate_layout(self.encoded(record), raw)


    def test_cli_reports_malformed_inputs_with_one_safe_fixed_error(self):
        # Parser/ZIP/filesystem exceptions must not escape as a traceback or
        # include a private pathname, ZIP member, or untrusted record text.
        with tempfile.TemporaryDirectory(prefix="private-layout-") as temp:
            directory = Path(temp)
            record_path = directory / "future-run-layout-v2.json"
            archive_path = directory / "future-run-skill-layout-v2.zip"
            scenarios = [(b"{private malformed json", self.archive_bytes),
                         (b'"private wrong record type"', self.archive_bytes)]
            for raw in (b"private not a zip", self.archive_bytes[:-12]):
                record = copy.deepcopy(self.record)
                record["archive"].update(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
                scenarios.append((self.encoded(record), raw))
            for record_bytes, archive_bytes in scenarios:
                with self.subTest(record_prefix=record_bytes[:20], archive_length=len(archive_bytes)):
                    record_path.write_bytes(record_bytes)
                    archive_path.write_bytes(archive_bytes)
                    process = subprocess.run([sys.executable, str(CLI), "--layout-preflight", temp],
                                             capture_output=True, text=True)
                    self.assertEqual(process.returncode, 1)
                    self.assertEqual(process.stderr, "")
                    self.assertEqual(json.loads(process.stdout), {"error": {
                        "code": "sk_e06.layout_invalid", "message": "Layout snapshot integrity check failed.",
                    }})
            record_path.unlink()
            process = subprocess.run([sys.executable, str(CLI), "--layout-preflight", temp],
                                     capture_output=True, text=True)
            self.assertEqual(process.returncode, 1)
            self.assertEqual(process.stderr, "")
            self.assertEqual(json.loads(process.stdout)["error"]["code"], "sk_e06.layout_invalid")


    def test_cli_rejects_symlink_record_archive_or_directory(self):
        # Following a static symlink would accept an input outside the chosen
        # regular-file snapshot and conceal the link from the checker.
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp) / "record"
            directory.mkdir()
            record_path = directory / "future-run-layout-v2.json"
            archive_path = directory / "future-run-skill-layout-v2.zip"
            record_path.write_bytes(self.record_bytes)
            archive_path.write_bytes(self.archive_bytes)
            for path in (record_path, archive_path):
                data = path.read_bytes()
                external = Path(temp) / (path.name + ".external")
                external.write_bytes(data)
                path.unlink()
                try:
                    path.symlink_to(external)
                except (OSError, NotImplementedError) as error:
                    self.skipTest(f"symlink creation unavailable: {type(error).__name__}")
                process = subprocess.run([sys.executable, str(CLI), "--layout-preflight", str(directory)],
                                         capture_output=True, text=True)
                with self.subTest(path=path.name):
                    self.assertEqual(process.returncode, 1)
                    self.assertEqual(process.stderr, "")
                    self.assertEqual(json.loads(process.stdout)["error"]["code"], "sk_e06.layout_invalid")
                path.unlink()
                path.write_bytes(data)
            alias = Path(temp) / "alias"
            alias.symlink_to(directory, target_is_directory=True)
            process = subprocess.run([sys.executable, str(CLI), "--layout-preflight", str(alias)],
                                     capture_output=True, text=True)
            self.assertEqual(process.returncode, 1)
            self.assertEqual(process.stderr, "")


    @unittest.skipUnless(hasattr(os, "mkfifo"), "named pipes unavailable")
    def test_cli_rejects_fifo_inputs_without_waiting_for_a_writer(self):
        # A plain read_bytes() blocks indefinitely on this static nonregular
        # input. The subprocess timeout bounds the test, not production work.
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            record_path = directory / "future-run-layout-v2.json"
            archive_path = directory / "future-run-skill-layout-v2.zip"
            for target in (record_path, archive_path):
                record_path.write_bytes(self.record_bytes)
                archive_path.write_bytes(self.archive_bytes)
                target.unlink()
                os.mkfifo(target)
                try:
                    process = subprocess.run([sys.executable, str(CLI), "--layout-preflight", temp],
                                             capture_output=True, text=True, timeout=2)
                except subprocess.TimeoutExpired:
                    self.fail("layout preflight blocked on a FIFO input")
                self.assertEqual(process.returncode, 1)
                self.assertEqual(process.stderr, "")
                self.assertEqual(json.loads(process.stdout)["error"]["code"], "sk_e06.layout_invalid")
                target.unlink()


    def test_rejects_unbounded_memory_compression_methods(self):
        # Only stored/DEFLATE members have the supported bounded decoder scope.
        for compression in (zipfile.ZIP_BZIP2, zipfile.ZIP_LZMA):
            with self.subTest(compression=compression):
                record, raw = self.rewritten_archive([("skills/storytelling/file", b"data")],
                                                     links=[], compression=compression)
                with self.assertRaises(LayoutError):
                    validate_layout(self.encoded(record), raw)


    def test_rejects_zip_names_that_zipfile_truncates_at_nul(self):
        # A sanitized ZipInfo.filename must not hide the original archive name.
        record, raw = self.rewritten_archive([("skills/storytelling/fileXhidden", b"data")], links=[])
        raw = raw.replace(b"skills/storytelling/fileXhidden", b"skills/storytelling/file\0hidden")
        record["archive"].update(sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))
        record["archive"]["files"][0]["path"] = "skills/storytelling/file"
        with self.assertRaises(LayoutError):
            validate_layout(self.encoded(record), raw)


    def test_cli_link_scanning_handles_unclosed_brackets_within_bounded_time(self):
        # Repeated rescanning from each '[' makes a small Markdown file consume
        # unbounded work. This adversarial file contains no actual links.
        record, raw = self.rewritten_archive([("skills/storytelling/SKILL.md", b"[" * 200000)], links=[])
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            (directory / "future-run-layout-v2.json").write_bytes(self.encoded(record))
            (directory / "future-run-skill-layout-v2.zip").write_bytes(raw)
            try:
                process = subprocess.run([sys.executable, str(CLI), "--layout-preflight", temp],
                                         capture_output=True, text=True, timeout=2)
            except subprocess.TimeoutExpired:
                self.fail("link scanning repeatedly rescanned unclosed Markdown brackets")
            self.assertEqual(process.returncode, 0, process.stderr)
            self.assertEqual(json.loads(process.stdout)["local_link_count"], 0)


    def test_snapshot_validation_uses_memory_and_cli_leaves_input_tree_unchanged(self):
        # This is a regression for the already passing no-extraction behavior:
        # opening/extracting/executing a member would fail or alter this tree.
        with mock.patch("builtins.open", side_effect=AssertionError("filesystem access during pure validation")), \
                mock.patch.object(os, "open", side_effect=AssertionError("filesystem access during pure validation")):
            result = sk_e06_layout.validate_layout(self.record_bytes, self.archive_bytes)
        self.assertEqual(result["file_count"], 28)
        self.assertEqual(result["local_link_count"], 79)
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            before = {"future-run-layout-v2.json": self.record_bytes,
                      "future-run-skill-layout-v2.zip": self.archive_bytes}
            for name, raw in before.items():
                (directory / name).write_bytes(raw)
            process = subprocess.run([sys.executable, str(CLI), "--layout-preflight", temp],
                                     capture_output=True, text=True)
            self.assertEqual(process.returncode, 0, process.stderr)
            after = {path.relative_to(directory).as_posix(): path.read_bytes()
                     for path in directory.rglob("*") if path.is_file()}
            self.assertEqual(after, before)


if __name__ == "__main__":
    unittest.main()
