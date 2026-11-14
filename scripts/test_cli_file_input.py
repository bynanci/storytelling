#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Normal-entry CLI checks for bounded regular-file inputs (no media/host calls).

Build the opt-in binary once, then run from any directory:
    cargo build --locked -p storytelling-cli --features draft-video-input
    python3 scripts/test_cli_file_input.py target/debug/storytelling

The Rust CI matrix runs this after its --all-features integration build. FIFO
checks use Python's os.mkfifo on Unix; Windows explicitly skips that test.
"""

import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
CLI = None
TIMEOUT_SECONDS = 5

# Every explicit file argument of every data command. No command is allowed to
# reach a model/editor/host; all bad paths fail in the CLI's input boundary.
FILE_COMMANDS = {
    "validate": 1,
    "import-openfilm": 1,
    "export-otio": 1,
    "audit-brief": 1,
    "plan-story": 1,
    "render-narrative": 1,
    "render-video-coverage": 1,
    "render-handoff": 1,
    "track-narrative-revisions": 1,
    "draft-video-coverage-input": 1,
    "render-cue-sheet": 2,
    "check-narrative-style": 2,
    "audit-copy-claims": 2,
    "prepare-story-review": 3,
    "export-story-otio": 3,
    "check-story-review": 4,
    "prepare-review": 2,
    "check-review": 3,
}


def cases_for(target, regular):
    cases = []
    for command, count in FILE_COMMANDS.items():
        for position in range(count):
            arguments = [str(regular)] * count
            arguments[position] = str(target)
            cases.append((f"{command}:file{position + 1}", [command, *arguments]))
    cases.append(("query-narrative-impact:file1", ["query-narrative-impact", str(target), "synthetic-change"]))
    cases.append(("verify-source:file1", ["verify-source", str(target), "0" * 64]))
    return cases


def invoke(arguments, input_bytes=None):
    try:
        result = subprocess.run(
            [str(CLI), *arguments],
            input=input_bytes,
            stdin=subprocess.DEVNULL if input_bytes is None else None,
            capture_output=True,
            timeout=TIMEOUT_SECONDS,
            check=False,
        )
    except subprocess.TimeoutExpired:
        # subprocess.run kills and waits for the controlled child before raising.
        return {"timed_out": True}
    return {"timed_out": False, "exit": result.returncode, "stdout": result.stdout, "stderr": result.stderr}


class FileInputTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory(prefix="storytelling-file-input-")
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.regular = self.root / "regular.json"
        self.regular.write_bytes(b"{}")

    def check_nonregular(self, path):
        cases = cases_for(path, self.regular)
        with ThreadPoolExecutor(max_workers=8) as executor:
            results = list(executor.map(lambda case: invoke(case[1]), cases))
        for (name, _arguments), result in zip(cases, results):
            with self.subTest(command=name):
                self.assertFalse(result["timed_out"], f"{name} blocked on nonregular input")
                self.assertEqual(result["exit"], 3, name)
                self.assertEqual(result["stderr"], b"", name)
                self.assertEqual(
                    json.loads(result["stdout"]),
                    {"ok": False, "error": {"code": "input.io", "path": "$"}},
                    name,
                )

    def test_directory_inputs_reject_with_io_exit_without_blocking(self):
        self.check_nonregular(self.root)

    @unittest.skipUnless(os.name != "nt" and hasattr(os, "mkfifo"), "FIFO creation is Unix-only; directory coverage runs on every platform")
    def test_fifo_inputs_reject_with_io_exit_without_blocking(self):
        fifo = self.root / "input-fifo.json"
        os.mkfifo(fifo)
        self.check_nonregular(fifo)

    def test_regular_file_and_stdin_replays_preserve_existing_results(self):
        fixtures = [
            ("validate", ROOT / "examples/minimal-plan.json", []),
            ("render-video-coverage", ROOT / "tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json", []),
            ("draft-video-coverage-input", ROOT / "tests/fixtures/n10-06a-draft/seconds-unresolved.draft.json", []),
        ]
        for command, path, extra in fixtures:
            with self.subTest(command=command):
                original = path.read_bytes()
                file_result = invoke([command, str(path), *extra])
                stdin_result = invoke([command, "-", *extra], original)
                self.assertFalse(file_result["timed_out"])
                self.assertFalse(stdin_result["timed_out"])
                self.assertEqual(file_result["exit"], 0)
                self.assertEqual(stdin_result["exit"], 0)
                self.assertEqual(file_result["stderr"], b"")
                self.assertEqual(stdin_result["stderr"], b"")
                self.assertEqual(file_result["stdout"], stdin_result["stdout"])
                self.assertEqual(path.read_bytes(), original)
        source = b"synthetic supplied bytes; no media"
        path = self.root / "supplied-bytes.txt"
        path.write_bytes(source)
        digest = hashlib.sha256(source).hexdigest()
        file_result = invoke(["verify-source", str(path), digest])
        stdin_result = invoke(["verify-source", "-", digest], source)
        self.assertFalse(file_result["timed_out"])
        self.assertFalse(stdin_result["timed_out"])
        self.assertEqual(file_result["exit"], 0)
        self.assertEqual(file_result["stdout"], stdin_result["stdout"])
        self.assertEqual(path.read_bytes(), source)

    def test_file_and_stdin_limits_still_reject_oversized_json(self):
        data = b" " * 1_048_577
        path = self.root / "oversized.json"
        path.write_bytes(data)
        for command, code in [("render-video-coverage", "input.too_large"), ("draft-video-coverage-input", "draft_time.input_too_large")]:
            for arguments, stdin in [([command, str(path)], None), ([command, "-"], data)]:
                with self.subTest(command=command, transport="stdin" if stdin else "file"):
                    result = invoke(arguments, stdin)
                    self.assertFalse(result["timed_out"])
                    self.assertEqual(result["exit"], 1)
                    self.assertEqual(result["stderr"], b"")
                    self.assertEqual(json.loads(result["stdout"])["error"]["code"], code)
        self.assertEqual(path.stat().st_size, len(data))


def main():
    global CLI
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary", type=Path, help="feature-enabled storytelling binary from the preceding integration build")
    args = parser.parse_args()
    CLI = args.binary.resolve()
    if os.name == "nt" and not CLI.suffix:
        CLI = CLI.with_suffix(".exe")
    if not CLI.is_file():
        parser.error("the explicit storytelling binary is missing")
    result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(FileInputTests))
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
