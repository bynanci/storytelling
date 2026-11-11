# M3-02 operation journal validation

Status: implementation candidate under review; no acceptance or merge claim.

| Item | Evidence |
| --- | --- |
| Base | `main` at `59a7d8585d9b0d9f4ce0b6cc3935bbc58d5d5dc1` |
| Branch | `codex/m3-02-sqlite-journal-20261111` |
| Initial RED | `cargo test -p storytelling-journal --test journal -- --nocapture`: 3 tests executed, all failed at the deliberately incomplete journal implementation (`not implemented`) |
| Focused GREEN | Final journal integration suite: 11 passed, including restart recovery, owner/attempt fencing, idempotency conflict, operation/readback bounds, no-retry state flow, foreign-database refusal and SQLite busy-lock rollback |
| Additional RED/GREEN | Late old-response/readback, per-attempt ambiguous-readback bound, conflicting readback before retry, and unrelated-database preservation regressions each produced a failing test before their safety check was implemented; all pass in the final 11-test suite |
| Dependency audit iteration | `rusqlite 0.37.0` / `libsqlite3-sys 0.35.0` failed Cargo Audit; `rusqlite 0.40.2` still resolved vulnerable `libsqlite3-sys 0.38.2`; both rejected. Final candidate is `rusqlite 0.41.1` / `libsqlite3-sys 0.39.1`, requiring Rust 1.89 |
| Formatting | `cargo fmt --all --check`: passed on Rust 1.89.0 |
| Clippy | `cargo clippy --workspace --all-targets --locked -- -D warnings`: passed on Rust 1.89.0 |
| Workspace debug/release | `cargo test --workspace --locked` and `cargo test --workspace --release --locked`: passed on Rust 1.89.0 |
| Rust documentation | `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked`: passed |
| Repository guard | `python3 scripts/check_repo.py` and `git diff --check`: passed |
| Current lockfile audit | Pinned Cargo Audit 0.22.1 loaded 1,446 advisories and scanned 52 dependencies with no findings; local Cargo emitted a crates.io-index update warning after completing the lockfile scan |
| Hosted exact-head gates | Pending PR head; Linux, Windows, macOS, Quality, Required and Dependency Audit must all pass |

Tests create synthetic temporary SQLite files and remove the database, WAL and
SHM files. No host, editor, Catalog, FilmCraft project, external asset or private
data is opened. Cross-platform filesystem, power-loss and network-filesystem
semantics are not established by the local Linux run; the host-side behavior is
reserved for M3-03.
