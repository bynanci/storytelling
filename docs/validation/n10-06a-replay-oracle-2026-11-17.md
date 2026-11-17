<!-- SPDX-License-Identifier: MIT -->
# N10-06A replay oracle correction · 2026-11-17

[繁體中文](n10-06a-replay-oracle-2026-11-17.zh-TW.md) · [Current rehearsal](../../examples/evaluation/n10-06a-rehearsal/README.md) · [Original PR #145 evidence](n10-06a-roundtable-2026-11-17.md) · [Receipt index](n10-06a-replay-oracle-2026-11-17/receipt-index.json)

Base: `91bf8f80139f59c0406340eb3565d296bd562931`. This is a correction to the synthetic evaluation tool. The supplied unmodified native CLI produced the correct ranges and passed all four original tests. No native conversion defect is claimed.

## Failure and correction

The old replay checked file/stdin/repeat agreement and preview/emit/accepted-V1 parity, but never compared valid ranges to an independent expected result. Consistently changing a legal start from `5/2` to `2501/1000` therefore passed all eight cases and 37 calls. The V1 validator accepts either legal declaration and cannot establish whether the draft-time conversion was correct. The [preserved false-pass receipt](n10-06a-replay-oracle-2026-11-17/baseline-false-pass.json) deliberately contains injected output; it is not an observation of incorrect native arithmetic.

The replay now computes expected endpoints with Python's standard-library `Fraction`, independently of native output: decimal 2.5–4.5, decimal 0–0.1, frames [75,135) at 30/1, and frames [0,150) at 30000/1001. It checks both endpoints of every valid mapped row, including both rows of `declaration-complete`, plus S02@1/S03@1 selected and mapping references, mapped status/asset reference, and the unchanged unresolved reason. It retains the original exact-output, emit/V1 parity and native first-error diagnostics. No Rust, product schema, fixture or author material changes are included.

## Observed RED and GREEN

The final portable test shim calls the replay's real native invocation function, injects one consistent JSON fault into successful helper outputs and their recorded raw bytes, and runs `main()` unchanged in a Python subprocess. It changes preview/emit/validation consistently so parity alone cannot reject the fault. There are no production injection hooks or executable-confinement claims.

- [Baseline](n10-06a-replay-oracle-2026-11-17/baseline-tests.log): the original four tests passed on the explicit native binary.
- [RED](n10-06a-replay-oracle-2026-11-17/oracle-red-portable.log): final tests against the untouched base replay ran seven methods, with 18 expected subtest failures across three regression methods; the original four still passed. Each failure observed a false successful exit rather than a crash or test setup error.
- [GREEN](n10-06a-replay-oracle-2026-11-17/oracle-final-green.log): all seven methods passed without skips, including 12 endpoint mutations, two reference mutations and four unresolved-reason mutations.
- [Correct native replay](n10-06a-replay-oracle-2026-11-17/native-green.json): passed, eight cases/37 calls. [Injected start regression after correction](n10-06a-replay-oracle-2026-11-17/mutant-rejected.json): failed with `wrong_canonical_declarations`, retaining one attempted case and three observed calls.

An additional compatibility check found the existing test driver interpreted unittest's `discover` argument as an executable path ([19 observed failures](n10-06a-replay-oracle-2026-11-17/discovery-red.log)). Positional binary parsing now applies only when the file runs directly. [Unconfigured discovery](n10-06a-replay-oracle-2026-11-17/discovery-green.log) runs seven methods with four native methods explicitly skipped; this is separate from the required explicit-native run, which passes all seven without skips.

Readable RED logs omit trailing spaces only; exact captured bytes remain in the matching `.log.gz` files, with uncompressed hashes in the receipt index. The full discovery RED reproduces the driver before its argument guard.

Run the current checks with the actual feature-enabled native path:

```sh
python3 examples/evaluation/n10-06a-rehearsal/test_replay.py /path/to/storytelling
python3 scripts/check_repo.py
```

The saved [fault configuration](n10-06a-replay-oracle-2026-11-17/range-fault.json) and [test-only shim](../../examples/evaluation/n10-06a-rehearsal/mutant_cli.py) reproduce the single start fault with `python3 mutant_cli.py CONFIG REPLAY --cli NATIVE --source-commit SHA --output NEW_RECEIPT`. Use the full repository-relative paths. Use a new receipt path; the capture still refuses existing observations.

## Evidence limits

The PR #145 roundtable, initial/primary machine receipts and RED/GREEN logs remain byte-for-byte unchanged. New evidence is in this separate directory. The original planned/attempted case-count correction remains intact. The previously built native binary is reused with its original declared build source; the replay still explicitly does not authenticate source/build binding. No Cargo build or network/Host call was made for this correction. Local checks ran on Linux; the portable Python shim is included in the existing explicit-native CI entry for all three OSes, but this note supplies no new observed Windows/macOS result.

This is synthetic AI technical review and fault injection, not human participation, an author-time measurement, ethics review, G2 acceptance or a new research outcome. Human participants and Host observations remain zero. The author handoff and separate #129, media, authority and G5 gates retain their existing scope.
