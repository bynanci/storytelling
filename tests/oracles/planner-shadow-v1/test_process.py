# SPDX-License-Identifier: Apache-2.0
"""Real-child budget and owned-descendant cleanup regressions; no mock processes."""
from __future__ import annotations
import os
from pathlib import Path
import signal
import sys
import tempfile
import time
import unittest
from process import ProcessFailure, run_bounded

@unittest.skipUnless(sys.platform.startswith("linux"), "Linux-only shadow process contract")
class BoundedProcessTests(unittest.TestCase):
    def run_python(self, code, data=None, **kwargs):
        deadline = kwargs.pop("deadline", time.monotonic() + 5)
        return run_bounded([sys.executable, "-c", code], data, deadline, **kwargs)

    def absent(self, pid):
        with self.assertRaises(ProcessLookupError):
            os.kill(pid, 0)

    def test_real_stdin_stdout_stderr_and_nonzero_exit_are_captured(self):
        result = self.run_python("import sys;sys.stdout.buffer.write(sys.stdin.buffer.read().upper());sys.stderr.write('note');sys.exit(7)", b"hello")
        self.assertEqual(result["exit"], 7)
        self.assertEqual(result["stdout"], b"HELLO")
        self.assertEqual(result["stderr"], b"note")
        self.assertTrue(result["started"])
        self.assertTrue(result["capture_complete"])
        self.assertEqual(result["pid"], result["process_group"])
        self.absent(result["pid"])

    def test_none_and_empty_stdin_both_deliver_eof(self):
        for data in [None, b""]:
            with self.subTest(data=data):
                result = self.run_python("import sys;print(len(sys.stdin.buffer.read()),end='')", data)
                self.assertEqual(result["stdout"], b"0")
                self.assertEqual(result["exit"], 0)

    def test_stdout_exact_limit_accepts_and_next_byte_rejects_bounded(self):
        result = self.run_python("import os;os.write(1,b'x'*256)", stdout_limit=256)
        self.assertEqual(result["stdout"], b"x" * 256)
        with self.assertRaises(ProcessFailure) as ctx:
            self.run_python("import os,time;os.write(1,b'x'*257);time.sleep(60)", stdout_limit=256)
        self.assertEqual(ctx.exception.code, "stdout-overflow")
        self.assertEqual(len(ctx.exception.receipt["stdout"]), 256)
        self.assertTrue(ctx.exception.receipt["stdout_truncated"])
        self.absent(ctx.exception.receipt["pid"])

    def test_stderr_exact_limit_accepts_and_next_byte_rejects_bounded(self):
        result = self.run_python("import os;os.write(2,b'e'*128)", stderr_limit=128)
        self.assertEqual(result["stderr"], b"e" * 128)
        with self.assertRaises(ProcessFailure) as ctx:
            self.run_python("import os,time;os.write(2,b'e'*129);time.sleep(60)", stderr_limit=128)
        self.assertEqual(ctx.exception.code, "stderr-overflow")
        self.assertEqual(len(ctx.exception.receipt["stderr"]), 128)
        self.assertTrue(ctx.exception.receipt["stderr_truncated"])
        self.absent(ctx.exception.receipt["pid"])

    def test_child_timeout_reaps_the_owned_direct_child(self):
        start = time.monotonic()
        with self.assertRaises(ProcessFailure) as ctx:
            self.run_python("import time;time.sleep(60)", timeout=0.1)
        self.assertEqual(ctx.exception.code, "timeout")
        self.assertLess(time.monotonic() - start, 2)
        self.absent(ctx.exception.receipt["pid"])

    def test_expired_global_deadline_never_spawns_a_child(self):
        with tempfile.TemporaryDirectory() as tmp:
            marker = Path(tmp) / "must-not-exist"
            with self.assertRaises(ProcessFailure) as ctx:
                self.run_python(f"from pathlib import Path;Path({str(marker)!r}).write_text('bad')", deadline=time.monotonic() - 1)
            self.assertEqual(ctx.exception.code, "deadline")
            self.assertFalse(ctx.exception.receipt["started"])
            self.assertFalse(marker.exists())

    def test_shared_deadline_cannot_reset_for_the_second_command(self):
        deadline = time.monotonic() + 0.3
        first = self.run_python("import time;time.sleep(0.05);print('first')", deadline=deadline)
        self.assertEqual(first["stdout"], b"first\n")
        with self.assertRaises(ProcessFailure) as ctx:
            self.run_python("import time;time.sleep(60)", deadline=deadline, timeout=2)
        self.assertEqual(ctx.exception.code, "deadline")
        self.absent(ctx.exception.receipt["pid"])

    def test_timeout_kills_and_reaps_owned_grandchild(self):
        code = "import subprocess,sys,time;p=subprocess.Popen([sys.executable,'-c','import time;time.sleep(60)']);print(p.pid,flush=True);time.sleep(60)"
        with self.assertRaises(ProcessFailure) as ctx:
            self.run_python(code, timeout=0.2)
        self.assertEqual(ctx.exception.code, "timeout")
        self.assertRegex(ctx.exception.receipt["stdout"].decode(), r"^\d+\n$")
        self.absent(int(ctx.exception.receipt["stdout"]))
        self.absent(ctx.exception.receipt["pid"])

    def test_overflow_kills_and_reaps_owned_grandchild(self):
        code = "import subprocess,sys,time,os;p=subprocess.Popen([sys.executable,'-c','import time;time.sleep(60)']);print(p.pid,flush=True);os.write(1,b'x'*1024);time.sleep(60)"
        with self.assertRaises(ProcessFailure) as ctx:
            self.run_python(code, stdout_limit=256)
        self.assertEqual(ctx.exception.code, "stdout-overflow")
        first = ctx.exception.receipt["stdout"].splitlines()[0]
        self.assertRegex(first.decode(), r"^\d+$")
        self.absent(int(first))
        self.absent(ctx.exception.receipt["pid"])

    def test_success_also_cleans_owned_background_grandchild(self):
        code = "import subprocess,sys;p=subprocess.Popen([sys.executable,'-c','import time;time.sleep(60)'],stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);print(p.pid,flush=True)"
        result = self.run_python(code)
        self.assertRegex(result["stdout"].decode(), r"^\d+\n$")
        self.assertEqual(result["exit"], 0)
        self.absent(int(result["stdout"]))
        self.absent(result["pid"])

    def test_input_budget_rejects_before_spawning(self):
        with self.assertRaises(ProcessFailure) as ctx:
            self.run_python("pass", b"x" * (1_048_576 + 1))
        self.assertEqual(ctx.exception.code, "input-overflow")
        self.assertFalse(ctx.exception.receipt["started"])

    def test_missing_executable_records_failed_spawn_without_fake_identity(self):
        with self.assertRaises(ProcessFailure) as ctx:
            run_bounded(["/nonexistent/shadow-owned-tool"], None, time.monotonic() + 1)
        self.assertEqual(ctx.exception.code, "spawn-failed")
        self.assertFalse(ctx.exception.receipt["started"])
        self.assertIsNone(ctx.exception.receipt["pid"])
        self.assertIsNone(ctx.exception.receipt["exit"])

if __name__ == "__main__":
    unittest.main()
