#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Offline layout snapshot checks; never dispatch a model or open media."""

import json
from pathlib import Path
import subprocess
import sys
import unittest


ROOT = Path(__file__).resolve().parents[1]
RECORD = ROOT / "docs/research/evaluations/sk-e06-runs-2026-11-17"
CLI = ROOT / "scripts/sk_e06_packet.py"


class LayoutTests(unittest.TestCase):
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


if __name__ == "__main__":
    unittest.main()
