# CLI regular-file input regression

English · [繁體中文](cli-file-input.zh-TW.md)

This shared CLI I/O fix supports M3-RB07 and N-R10-06A without changing their
contracts. Original base: `c562ce0d06b7330813fa42cad739f24f4a49e9da`. The first integrated
base is `5bbe80dc2cf8286abb15ad5278e86ec7384cdb4d`. It adds no media probe,
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

The original Linux [workspace receipt](cli-file-input/workspace-checks.json)
and [output excerpts](cli-file-input/workspace-checks.txt) are preserved as
`cache_ambiguous` / superseded. A shared build target reused artifacts from a
parallel worktree: the named source had eleven journal tests but these logs ran
thirteen. Their observed 312/331 test totals do **not** prove verification of the
named source commit. The fresh isolated-target run below supersedes that claim.
The focused FIFO result is distinct from this invalid workspace-provenance
claim. No historical output or count has been erased.

### Fresh integrated verification

Integration source `ed5bfa6a7d66cce9b66faf6ac8bef653b5e43485` was clean when
verification started. Its new, absent-before-run target was used by this
checkout alone, with `CARGO_INCREMENTAL=0`. The [isolated receipt](cli-file-input/isolated-checks.json)
and [actual output excerpts](cli-file-input/isolated-checks.txt) retain commands,
binary/source-blob hashes, raw-output hashes and counts. This is the authoritative
local workspace result; it supersedes the ambiguous receipt without deleting it.

All checks passed: fresh all-feature build, four CLI Python methods, fmt,
all-target/all-feature Clippy, default and all-feature debug/release tests,
rustdoc with warnings denied, repository checks and full-branch diff checks.
Default Rust runs passed 312 tests each and all-feature runs 331 each, including
thirteen integrated journal tests, seven V1 coverage tests and twelve draft-input
CLI tests. Totals include doc tests and exclude the Python methods. Existing
pilot, recorder, graph, host-proof rejection, SK-E01/E02/E04/E06 and R1 Python
checks also passed. One supplemental command used a nonexistent script name and
exited 2 without running tests; the corrected existing E04 suite passed. Both
invocations remain recorded.

Do not share `CARGO_TARGET_DIR` between worktrees. For a clean reproduction use a
new empty target directory for this checkout and pass that directory's CLI
binary to the Python test. The commands below use the default checkout-local
`target` directory; unset any shared target environment setting first. The
historical pre-rebase commits and rebased patch lineage are listed in the new
receipt; the fresh result binds the integrated source above.

### Test discovery compatibility correction

Before merging PR #139, broad Python discovery exposed a test-harness error:
the module's `CLI` global was unbound, so four methods produced ten errors while
trying to launch `None`. The [configuration RED receipt](cli-file-input/discovery-red-receipt.json)
and [log](cli-file-input/discovery-red.txt) reproduce this at `fdd8283`. This is
separate from the original product FIFO failure and adds no product test claim.

Latest main `3e0f0d0992df38acc53402f0eb1d1926f63cf8a7` was merged without rewriting
the pushed branch. At source `d1d8b64f8167eebe9a2e1ee915d3b0b3e2ef1923`, `setUp`
skips only when no explicit binary has been bound. The standalone invocation
binds its binary before running tests, so all four product test methods remain
mandatory and unchanged. The existing isolated binary and Rust source hashes
match the earlier fresh build; no Rust compilation was needed for this fix.

The [discovery GREEN receipt](cli-file-input/discovery-green-receipt.json) and
[complete output](cli-file-input/discovery-green.txt) record:

| Invocation | Actual result |
| --- | --- |
| Explicit feature-enabled CLI binary | Four product methods passed; no skips on Linux. |
| Targeted discovery without a binary | Four configuration skips; no product assertions executed. |
| Broad `scripts/test_*.py` discovery | 103 methods recorded: 99 passed, four configuration skips. |
| Integrated SK-E04 suite | 24 passed. |
| Repository and complete branch diff checks | Passed. |

CI runs the mandatory explicit-binary command first, then targeted discovery,
in two separate single-command steps. This prevents Windows PowerShell from
letting a later successful discovery overwrite a failed binary test exit code.
Windows still skips only the Unix FIFO case in the explicit-binary run. New-head
hosted CI is required; the old `fdd8283` results are not the final-head check.

```sh
cargo build --locked -p storytelling-cli --features draft-video-input
python3 scripts/test_cli_file_input.py target/debug/storytelling
python3 -m unittest discover -s scripts -p 'test_cli_file_input.py'
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
