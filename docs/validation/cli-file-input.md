# CLI regular-file input regression

English · [繁體中文](cli-file-input.zh-TW.md)

This shared CLI I/O fix supports M3-RB07 and N-R10-06A without changing their
contracts. Base: `c562ce0d06b7330813fa42cad739f24f4a49e9da`. It adds no media probe,
host operation, public DTO, diagnostic, dependency or byte budget.

## Problem and accepted scope

Previously the CLI called `File::open` before checking handle metadata. Opening
a named FIFO could wait indefinitely for a writer, so the existing regular-file
rejection was never reached. The regression uses normal command entry points,
including `verify-source`, legacy video coverage, the opt-in draft helper and
all review input positions.

The shared reader now checks path metadata before opening, retains its
opened-handle regular-file check and reads at most the existing limit plus one
byte. Generic commands and `verify-source` delegate to that same reader. Existing
argument validation, per-operation limits, domain diagnostic precedence and
file/stdin output semantics remain unchanged. A static nonregular named input
returns `{"ok":false,"error":{"code":"input.io","path":"$"}}`, exit 3,
with no stderr or path disclosure.

This is a static-path I/O safeguard, not a trusted local-media adapter or an OS
sandbox. Symlinks to regular files remain accepted. A pathname can still be
replaced between metadata and open; that race can still cause a blocking open.
The opened-handle check does not remove that race. Standard input still needs
EOF and can wait if the caller leaves its stream open. No TOCTOU protection,
path confinement, media identity, author approval or host acceptance is claimed.
N10-06C and execution authority retain their separate gates.

## Test-first evidence and replay

The test-first commit is `b05a083d4e911564b38f6a70e715fa71bcd4bd01`.
The [RED receipt](cli-file-input/red-receipt.json) and [log](cli-file-input/red.txt)
record exit 1 with 33 FIFO subcase failures: each controlled child timed out
after five seconds and was killed and waited for. Directory, regular-file/stdin
replay and size-limit checks already passed. These timeouts reproduce the input
bug; they are not author performance measurements.

Implementation commit `a6c9a367bedb5004fa39b5eafa763dc0ec4b6838` passed the
unchanged tests: [GREEN receipt](cli-file-input/green-receipt.json),
[log](cli-file-input/green.txt). The four test methods cover 33 explicit file
positions with a directory and a FIFO, plus supplied-byte verification,
regular-file/stdin replays, unchanged fixtures and existing oversized-input
diagnostics. FIFO creation uses Python `os.mkfifo` without an external tool.
Windows explicitly skips FIFO creation; directory and other checks run on all
three CI platforms. Each subprocess has bounded cleanup.

The Linux [workspace receipt](cli-file-input/workspace-checks.json) and
[actual output excerpts](cli-file-input/workspace-checks.txt) record passing
formatting, all-target/all-feature Clippy, default and all-feature debug/release
tests and rustdoc with warnings denied. Default test runs passed 312 tests each;
all-feature runs passed 331 each, including seven V1 video-coverage and twelve
draft-input CLI regressions. These totals include doc tests and do not count
the four Python methods. Repository checks and `git diff --check` also passed.

```sh
cargo build --locked -p storytelling-cli --features draft-video-input
python3 scripts/test_cli_file_input.py target/debug/storytelling
cargo test --locked -p storytelling-cli --all-features --test video_coverage --test draft_video_input
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo test --workspace --release --locked
cargo test --workspace --all-features --release --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
python3 scripts/check_repo.py
git diff --check
```

On Windows use `python` and `target/debug/storytelling.exe`; set `RUSTDOCFLAGS`
using the shell's environment syntax. The existing pinned-Python Rust matrix
runs the new script immediately after its all-features integration build,
before a later default-feature sample can replace the binary. Local evidence
is Linux-only; exact-head hosted results must be read before merge. Publication,
real media, human use and host recovery are not verified by these tests.
