# SPDX-License-Identifier: Apache-2.0
"""Bounded Linux process groups for the selected-domain test-only shadow.

Each command owns a new session. Live pipe capture is bounded before retention;
cleanup kills its group and reaps adopted descendants without touching other
process groups. This is lifecycle management, not an OS filesystem sandbox.
"""
from __future__ import annotations

import ctypes
import math
import os
import selectors
import signal
import subprocess
import sys
import time

MAX_INPUT = 1_048_576
MAX_STDOUT = 4_194_304
MAX_STDERR = 65_536


class ProcessFailure(RuntimeError):
    """Execution/budget failure with actual bounded partial observations."""

    def __init__(self, code: str, receipt: dict):
        super().__init__(code)
        self.code = code
        self.receipt = receipt


class _Abort(Exception):
    def __init__(self, code: str):
        self.code = code


def _subreaper() -> tuple[ctypes.CDLL, int]:
    """Temporarily adopt orphaned grandchildren so their owned group is reaped."""
    libc = ctypes.CDLL(None, use_errno=True)
    old = ctypes.c_int()
    if libc.prctl(37, ctypes.byref(old), 0, 0, 0) != 0:  # PR_GET_CHILD_SUBREAPER
        raise OSError(ctypes.get_errno(), "subreaper query failed")
    if old.value != 1 and libc.prctl(36, ctypes.c_ulong(1), 0, 0, 0) != 0:
        raise OSError(ctypes.get_errno(), "subreaper setup failed")
    return libc, old.value


def _kill_group(group: int) -> None:
    try:
        os.killpg(group, signal.SIGKILL)
    except ProcessLookupError:
        pass


def _cleanup(child: subprocess.Popen) -> None:
    _kill_group(child.pid)
    child.wait(timeout=1)
    # Only reap the newly owned group; never wait for unrelated caller children.
    finish = time.monotonic() + 1
    while True:
        try:
            pid, _ = os.waitpid(-child.pid, os.WNOHANG)
        except ChildProcessError:
            return
        if pid:
            continue
        if time.monotonic() >= finish:
            raise OSError("owned descendants did not reap within cleanup budget")
        time.sleep(0.005)


def run_bounded(argv, data, deadline, *, stdout_limit=MAX_STDOUT,
                stderr_limit=MAX_STDERR, timeout=30, env=None):
    """Run one explicit command under an absolute monotonic global deadline.

    Nonzero child exits are returned for the caller's semantic checks. Transport
    failures raise ProcessFailure; its captured prefixes remain within the caps.
    Cleanup overhead is bounded separately and does not start another command.
    """
    started_at = time.monotonic()
    receipt = {"argv": [os.fspath(value) for value in argv], "exit": None,
               "stdout": b"", "stderr": b"", "elapsed": 0.0,
               "pid": None, "process_group": None, "started": False,
               "capture_complete": False, "stdout_truncated": False,
               "stderr_truncated": False}
    child = None
    selector = None
    subreaper = None
    buffers = {"stdout": bytearray(), "stderr": bytearray()}
    failure = None
    try:
        if not math.isfinite(deadline) or started_at >= deadline:
            raise _Abort("deadline")
        if (not receipt["argv"] or not os.path.isabs(receipt["argv"][0])
                or not sys.platform.startswith("linux")):
            raise _Abort("invalid-command")
        if data is not None and (not isinstance(data, bytes) or len(data) > MAX_INPUT):
            raise _Abort("input-overflow")
        if (not isinstance(stdout_limit, int) or not 0 <= stdout_limit <= MAX_STDOUT
                or not isinstance(stderr_limit, int) or not 0 <= stderr_limit <= MAX_STDERR
                or not math.isfinite(timeout) or not 0 < timeout <= 30):
            raise _Abort("invalid-limits")
        try:
            subreaper = _subreaper()
            child = subprocess.Popen(receipt["argv"], stdin=subprocess.PIPE if data is not None else subprocess.DEVNULL,
                                     stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                     env=env, bufsize=0, close_fds=True, start_new_session=True)
        except (OSError, ValueError) as exc:
            raise _Abort("spawn-failed") from exc
        receipt.update(pid=child.pid, process_group=child.pid, started=True)
        selector = selectors.DefaultSelector()
        for name in ("stdout", "stderr"):
            stream = getattr(child, name)
            os.set_blocking(stream.fileno(), False)
            selector.register(stream, selectors.EVENT_READ, name)
        sent = 0
        if child.stdin is not None:
            if data:
                os.set_blocking(child.stdin.fileno(), False)
                selector.register(child.stdin, selectors.EVENT_WRITE, "stdin")
            else:
                child.stdin.close()
        command_end = started_at + timeout
        finish = min(deadline, command_end)
        group_finished = False

        def close(stream):
            try:
                selector.unregister(stream)
            except (KeyError, ValueError):
                pass
            stream.close()

        while selector.get_map() or child.poll() is None:
            now = time.monotonic()
            if now >= finish:
                raise _Abort("deadline" if deadline <= command_end else "timeout")
            for key, _ in selector.select(min(0.05, finish - now)):
                stream, name = key.fileobj, key.data
                if name == "stdin":
                    try:
                        count = os.write(stream.fileno(), data[sent:sent + 65_536])
                    except BrokenPipeError:
                        close(stream)
                        continue
                    except (BlockingIOError, InterruptedError):
                        continue
                    sent += count
                    if sent == len(data):
                        close(stream)
                    continue
                limit = stdout_limit if name == "stdout" else stderr_limit
                remaining = limit - len(buffers[name])
                try:
                    block = os.read(stream.fileno(), min(65_536, remaining + 1))
                except (BlockingIOError, InterruptedError):
                    continue
                if not block:
                    close(stream)
                    continue
                buffers[name].extend(block[:remaining])
                if len(block) > remaining:
                    receipt[name + "_truncated"] = True
                    raise _Abort(name + "-overflow")
            if child.poll() is not None and not group_finished:
                # A successful parent must not leave background worker descendants.
                _kill_group(child.pid)
                group_finished = True
        receipt["exit"] = child.wait()
        receipt["capture_complete"] = True
    except _Abort as exc:
        failure = exc.code
    except Exception:
        failure = "capture-failed"
    finally:
        if child is not None:
            try:
                _cleanup(child)
                receipt["exit"] = child.returncode
            except (OSError, subprocess.TimeoutExpired):
                failure = "cleanup-failed"
            for stream in (child.stdin, child.stdout, child.stderr):
                if stream is not None:
                    stream.close()
        if selector is not None:
            selector.close()
        if subreaper is not None:
            libc, previous = subreaper
            if previous != 1 and libc.prctl(36, ctypes.c_ulong(previous), 0, 0, 0) != 0:
                failure = "cleanup-failed"
        receipt["stdout"] = bytes(buffers["stdout"])
        receipt["stderr"] = bytes(buffers["stderr"])
        receipt["elapsed"] = time.monotonic() - started_at
    if failure is not None:
        receipt["capture_complete"] = False
        raise ProcessFailure(failure, receipt)
    return receipt
