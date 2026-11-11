# M3-02 operation journal validation

Status: accepted bounded storage slice; PR #56 is merged. No host executor,
readback verifier or exactly-once effect is included.

| Item | Evidence |
| --- | --- |
| Base | `main` at `52236f7c2185a1224873f0da3989d709b757314d` |
| Branch | `codex/m3-02-sqlite-journal-20261111` |
| PR head / merge | [PR #56](https://github.com/bynanci/storytelling/pull/56), head `3cd078e56ed4f2d3dd7accf655446851322cd94d`, merged as `73196930e13d8ce53ba792c574ca4de156883a73` at 2026-11-11 02:07:53 UTC |
| Initial RED | `cargo test -p storytelling-journal --test journal -- --nocapture`: 3 tests executed, all failed at the deliberately incomplete journal implementation (`not implemented`) |
| Focused GREEN | Final journal integration suite: 11 passed, including restart recovery, owner/attempt fencing, idempotency conflict, operation/readback bounds, no-retry state flow, foreign-database refusal and SQLite busy-lock rollback |
| Additional RED/GREEN | Late old-response/readback, per-attempt ambiguous-readback bound, conflicting readback before retry, and unrelated-database preservation regressions each produced a failing test before their safety check was implemented; all pass in the final 11-test suite |
| Dependency audit iteration | `rusqlite 0.37.0` / `libsqlite3-sys 0.35.0` failed Cargo Audit; `rusqlite 0.40.2` still resolved vulnerable `libsqlite3-sys 0.38.2`; both rejected. Final candidate is `rusqlite 0.41.1` / `libsqlite3-sys 0.39.1`, requiring Rust 1.89 |
| Formatting | `cargo fmt --all --check`: passed on Rust 1.89.0 |
| Clippy | `cargo clippy --workspace --all-targets --locked -- -D warnings`: passed on Rust 1.89.0 |
| Workspace debug/release | `cargo test --workspace --locked` and `cargo test --workspace --release --locked`: passed on Rust 1.89.0 |
| Rust documentation | `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked`: passed |
| Repository guard | `python3 scripts/check_repo.py` and `git diff --check`: passed |
| Current lockfile audit | Pinned Cargo Audit 0.22.1 loaded 1,446 advisories and scanned 52 dependencies with no findings; local Cargo emitted a crates.io-index update warning after completing the cached scan |
| Hosted exact-head gates | [PR run 42943528675](https://github.com/bynanci/storytelling/actions/runs/42943528675) passed Quality, Required, Linux, Windows, macOS and Dependency Audit for the exact PR head |
| Post-merge main gates | [Main run 42943853913](https://github.com/bynanci/storytelling/actions/runs/42943853913) passed all six jobs on merge commit `73196930e13d8ce53ba792c574ca4de156883a73` |

Tests create synthetic temporary SQLite files and remove the database, WAL and
SHM files. No host, editor, Catalog, FilmCraft project, external asset or private
data is opened. Cross-platform hosted build/test evidence passed, but OS-specific
power-loss and network-filesystem semantics are not established. Host-side
readback and retry safety remain M3-03.
