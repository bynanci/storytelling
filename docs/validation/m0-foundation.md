# M0 foundation validation record

Date: 2026-11-07. Base: `551ff38920e2085e1d53a7ba777c9680678e9725`.

## Observed RED

Test scaffold commit: `3e0b43d9139f5a4d367c6b5f7a9b78a253c0dcef`.
Hosted run: https://github.com/bynanci/storytelling/actions/runs/42314783329
Ubuntu runner installed Rust 1.85.0 and compiled the three crates successfully.
Two CLI integration tests then failed: valid input could not succeed and the
invalid-range command did not emit its required JSON error. This was intentional
missing implementation, not a compiler/environment failure. Cargo stopped at that
CLI test binary; no claim is made that the engine suite executed in the RED run.
Cargo generated the checked-in lockfile during this run.

## Final verification procedure

The implementation commit replaces the stub with strict DTO/rational/source-range
validation and a real bounded read-only CLI. CI runs formatting, clippy, rustdoc,
SDLC traceability, release-mode tests and Linux/Windows/macOS build/workspace/CLI
tests. Use the PR's exact head and linked Actions runs for the actual final results;
this document is not a claim that a previous or in-progress run passed.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo doc --workspace --no-deps --locked
python3 scripts/check_repo.py
```

## Limits

The assistant's local container has no Rust/Cargo and no outbound DNS. Local Rust
execution was not performed; actual Rust evidence comes from hosted Actions.
Review was self-review, not an independent human/subagent audit. No real source
bytes, FilmCraft, OTIO, Resolve, installer, signing, GPU, audio listening or model
call was exercised. All input hashes are synthetic declarations. No source project
or main branch was modified. Branch protection/private reporting were not changed.

M0 implementation review does not satisfy any M1 host or later release gate.
