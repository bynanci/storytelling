#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Optional scientific-dependency check in a disposable copy; never overwrite R1."""
import hashlib
import json
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[5]
PACKAGE = ROOT / "docs/research/evaluations/2026-11-13-gemini-r1"
GENERATED = ["ratings_numeric_R1.csv", "case_pairs_R1.csv", "ratings_normalized_R1.json",
             "dimension_summary_R1.csv", "statistics_R1.json"]

with tempfile.TemporaryDirectory(prefix="r1-numeric-reproduction-") as temp:
    copied = Path(temp) / "r1"
    shutil.copytree(PACKAGE, copied, ignore=shutil.ignore_patterns("__pycache__", ".venv"))
    result = subprocess.run([sys.executable, "analyze.py"], cwd=copied, capture_output=True)
    if result.returncode:
        print("Original analyzer failed; install its pinned requirements in an isolated environment.", file=sys.stderr)
        raise SystemExit(result.returncode)
    versions = json.loads((copied / "statistics_R1.json").read_text(encoding="utf-8"))["computation"]
    print("Python", platform.python_version(), "NumPy", versions["numpy"], "SciPy", versions["scipy"])
    for name in GENERATED:
        expected = (PACKAGE / name).read_bytes()
        if (copied / name).read_bytes() != expected:
            print(name, "DIFFERS (cross-version metadata may differ; inspect separately)")
            raise SystemExit(1)
        print(name, "BYTE_IDENTICAL", hashlib.sha256(expected).hexdigest())
print("Disposable copy removed; original package unchanged.")
