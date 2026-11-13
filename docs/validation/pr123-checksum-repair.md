<!-- SPDX-License-Identifier: MIT -->
# PR #123: committed-byte checksum repair

Date: 2026-11-13. Reviewed original head:
`e70bafac1ef2756b3e317cc77de6ad44e0065446`.
Scope: SK-EVAL-R1 package integrity; no study re-scoring, Rust/CLI schema,
follow-up issue completion, merge or release.

## Reproduced failure and byte-level fix

The original manifest verified 11/15 entries. Exactly four CSV entries recorded
the SHA-256 of CRLF versions, while their committed Git blobs already contained
UTF-8 BOM plus LF. Converting each original blob's LF to CRLF reproduced its
old manifest digest. The original CSV blobs were preserved without modification.

| CSV | Original manifest (CRLF) | Correct committed blob (LF) |
| --- | --- | --- |
| case_pairs_R1.csv | `7859e3dc339ca90b84dbf2fa8c9723ea87e588de60aed434e134d9ced415028c` | `9b313ec4ac541fadfa06506a5c7ac59d99b7e6d03e8cb91a08bd9a47f5473d03` |
| dimension_summary_R1.csv | `4f113e3720ba7394369316b1628d1303c64c619985666beb936d0cb0b3b8813d` | `d1932092d0e8dec0967ddff51dad64e49a0b63eb15c04db1cf5af1d8d83535ea` |
| ratings_numeric_R1.csv | `4a325650575964c5acbc20fd560cddff82deb821c84972adfd9e6da4cda1071b` | `80a663ac29ccbc2f607706cfbdc152cd43fee091661898976cdbfa6dffef2260` |
| source_audit_R1.csv | `4efc974e8550dd2e03f3ef34d1980108a6aaa43a93afb8eff6cb72c42f9a1c7d` | `9b25b950e94cab13f0a1569a1622e4f9dc968da65bedf548ac76aab13b0c917d` |

The manifest also refreshes the two READMEs and the analysis/plot scripts because
their reproduction instructions or explicit newline writes changed. Its inventory
remains 15 files. It is rebuilt from staged Git objects via `git cat-file`, then
checked against the committed objects and raw checkout bytes. It never hashes a
normalized spreadsheet export or an unstaged working copy.

Root `.gitattributes` pins the complete package to `text eol=lf`, covering JSON,
Markdown, Python, SVG and text as well as CSV. CSV output retains UTF-8 BOM;
writers explicitly select LF. Generated JSON/SVG text writes also specify LF.
The bilingual package READMEs document the staging, refresh, commit and final
clean-checkout verification sequence. Static source adjudication is not regenerated.

## RED / GREEN and reproduction

`python3 scripts/test_r1_package_checksums.py` was first run against the original
head before the repair. It failed in all three checkout subcases: false/input
reported the four CSV mismatches; true additionally exposed checkout conversion
of the manifest itself. These were actual byte mismatches, not missing tools.

After repair, the suite passes five tests, including three fresh local Git clones
with `core.autocrlf=false`, `input`, and `true`. It verifies all manifested raw
bytes against committed objects and leaves each clone clean. Negative tests
reject CRLF mutation, incomplete/duplicate/unsafe manifest entries, and demonstrate
that refresh reads staged bytes when working bytes have subsequently changed.
The existing Linux/Windows/macOS CI matrix runs this stdlib-only suite.

A subsequent Codex Review found a locale-dependent JSON read in `plot.py`.
The added input-loading regression first failed with a simulated CP1252 default:
`UnicodeDecodeError` at byte 3427. The script now explicitly decodes the retained
statistics as UTF-8. The test executes the actual input-loading statement without
requiring plotting dependencies in CI. A separate local full plotting run under
the simulated default also reproduces the retained SVG byte-for-byte.

| Executed local command / check | Actual result |
| --- | --- |
| `python3 scripts/r1_package_checksums.py verify --ref HEAD --worktree` | 15/15 verified |
| `python3 scripts/test_r1_package_checksums.py` | 5 tests passed; all three autocrlf checkouts and simulated CP1252 input passed |
| `sha256sum -c MANIFEST.sha256` from package directory | 15/15 verified |
| `python3 scripts/check_repo.py`; `git diff --check` | passed |
| Temporary-copy `python3 analyze.py`; `python3 plot.py` | passed; all generated CSV/JSON/SVG outputs byte-identical to retained files |
| `cargo fmt --all -- --check` | passed |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | passed |
| `cargo test --workspace --locked --offline` | passed |
| `cargo test --workspace --release --locked --offline` | passed |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked --offline` | passed |

Numeric/visual reproduction used Python 3.12.14, NumPy 2.3.5, SciPy 1.17.0 and
Matplotlib 3.10.8 in a temporary package copy with a writable temporary
`MPLCONFIGDIR`. The four CSVs, `score_input.py`, normalized JSON, retained
statistics and SVG remain byte-identical to the original PR head. Both READMEs
only add reproduction guidance; the research conclusions remain intact.

Unchanged observations: 40 cases / 80 drafts / 560 ratings; declared story mean
4.360714285714286, control mean 4.542857142857143, difference
−0.18214285714285713 and exploratory CI [−0.41428571428571426,
0.053571428571428624]. Preferences remain 15 story / 22 control / 3 tie; original
critical flags remain 8 story / 7 control. No assignment key or flag was repaired.

## Review and merge readiness

A separate Codex agent independently verified the committed checksums and the
unchanged data blobs and reviewed the repair; no blocking finding was reported.
This is AI review, not maintainer acceptance or an independent rerun of the
numeric computation. Exact remote-head CI and the original Review thread's
disposition are recorded in PR #123 after submission.

Checksum integrity does not supply the missing original judge JSON, generation
manifests or source snapshots. Those study limitations and the conclusions remain
unchanged. Cross-version computation may differ; preserved R1 observations must
not be overwritten. #118–#122 remain open. The maintainer decides whether to merge;
this work does not merge the PR or publish a version.
