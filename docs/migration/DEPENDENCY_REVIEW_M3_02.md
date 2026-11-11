# M3-02 SQLite dependency review

Date: 2026-11-11. Candidate only; dependency and hosted CI review remain part of
the M3-02 PR gate.

The first local trial pinned `rusqlite = 0.37.0` with `bundled`, which resolved
`libsqlite3-sys = 0.35.0`. Checksum-pinned Cargo Audit 0.22.1 reported
[RUSTSEC-2026-0459](https://rustsec.org/advisories/RUSTSEC-2026-0459), CVSS 6.8,
dated 2026-10-30, affecting the bundled C SQLite. That dependency was rejected
before any PR was opened or merged.

The next trial pinned `rusqlite = 0.40.2`, but its semver range resolved
`libsqlite3-sys = 0.38.2`, which is still below the advisory's fixed line; it was
also rejected. The candidate now pins `rusqlite = 0.41.1`, disables default
features and enables only `bundled`; its lockfile resolves
`libsqlite3-sys = 0.39.1`, in the advisory's fixed line. `rusqlite 0.41.1`
declares Rust 1.89, so the workspace MSRV, `rust-toolchain.toml` and hosted Rust
jobs are updated together to 1.89.0. The implementation does not suppress the
advisory or use `--ignore-rust-version`. Both crates are MIT-licensed; no
OpenFilm source or code is imported. Bundling provides a pinned SQLite
implementation across hosts but brings C code into the native dependency
closure, so every candidate still requires lockfile audit and three-platform
build evidence.

Final lockfile versions, exact-head advisory snapshot and hosted run links are
recorded after the final PR head passes. The initial vulnerable trial is retained
here as a resolved dependency finding, not a current dependency claim.
