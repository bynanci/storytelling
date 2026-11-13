#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Local-only clean Git checkout checks, without numeric/model dependencies."""
import hashlib
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True

from r1_package_checksums import (
    MANIFEST, PACKAGE, ROOT, package_blobs, parse_manifest, render_manifest, verify, verify_worktree,
)


class R1Checksums(unittest.TestCase):
    def test_clean_checkouts_preserve_every_manifest_byte(self):
        committed = package_blobs(ROOT, "HEAD")
        # A separate checkout exercises Git's EOL conversion, not an archive or
        # a copy of the developer's working directory. No network is accessed.
        for mode in ("false", "input", "true"):
            with self.subTest(autocrlf=mode), tempfile.TemporaryDirectory() as temp:
                checkout = Path(temp) / "checkout"
                env = os.environ.copy()
                env["GIT_CONFIG_GLOBAL"] = os.devnull
                env["GIT_CONFIG_SYSTEM"] = os.devnull
                def run(*args):
                    return subprocess.run(["git", *args], env=env, check=True,
                                          stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                run("clone", "--quiet", "--shared", "--no-checkout", str(ROOT), str(checkout))
                run("-C", str(checkout), "config", "core.autocrlf", mode)
                run("-C", str(checkout), "checkout", "--quiet", "--detach", "HEAD")
                count = verify_worktree(checkout, committed)
                self.assertEqual(count, len(committed) - 1)
                status = run("-C", str(checkout), "status", "--porcelain").stdout
                self.assertEqual(status, b"", "Verification must leave a clean checkout")

    def test_changed_csv_line_endings_do_not_get_normalized_away(self):
        csv = b"\xef\xbb\xbfcase,value\nC01,5\n"
        digest = hashlib.sha256(csv).hexdigest()
        valid = {MANIFEST: f"{digest}  sample.csv\n".encode(), "sample.csv": csv}
        self.assertEqual(verify(valid), 1)
        changed = dict(valid, **{"sample.csv": csv.replace(b"\n", b"\r\n")})
        with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
            verify(changed)
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for name, data in changed.items():
                target = root / PACKAGE / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(data)
            with self.assertRaisesRegex(ValueError, "Checkout bytes differ"):
                verify_worktree(root, valid)

    def test_manifest_rejects_missing_duplicate_and_unsafe_entries(self):
        digest = hashlib.sha256(b"data\n").hexdigest()
        with self.assertRaisesRegex(ValueError, "coverage"):
            verify({MANIFEST: f"{digest}  missing.txt\n".encode(), "actual.txt": b"data\n"})
        for names in (("a", "a"), ("../escape",), ("/absolute",), (MANIFEST,)):
            with self.subTest(names=names), self.assertRaises(ValueError):
                parse_manifest("".join(f"{digest}  {name}\n" for name in names).encode())

    def test_refresh_reads_staged_blobs_not_unstaged_working_bytes(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            package = root / PACKAGE
            package.mkdir(parents=True)
            staged = b"\xef\xbb\xbfcase,value\nC01,5\n"
            (package / "sample.csv").write_bytes(staged)
            (package / MANIFEST).write_bytes(b"placeholder\n")
            subprocess.run(["git", "init", "--quiet", str(root)], check=True,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            subprocess.run(["git", "-C", str(root), "-c", "core.autocrlf=false",
                            "add", "--", PACKAGE], check=True,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            (package / "sample.csv").write_bytes(staged.replace(b",5", b",1").replace(b"\n", b"\r\n"))
            blobs = package_blobs(root)
            self.assertEqual(blobs["sample.csv"], staged)
            manifest = render_manifest(blobs)
            self.assertEqual(parse_manifest(manifest)["sample.csv"], hashlib.sha256(staged).hexdigest())
            self.assertEqual(verify({**blobs, MANIFEST: manifest}), 1)


if __name__ == "__main__":
    unittest.main()
