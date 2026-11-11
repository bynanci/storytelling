<!-- SPDX-License-Identifier: MIT -->
# M3-01/P2 validation

Base `a7907d071894959487a0ae6df6948e09fef70596`, Rust/Cargo 1.85.0,
locked existing dependencies. Implementation branch:
`codex/planner-otio-review-20261111`. No release or host writes.

Observed functional RED before implementation:

```text
cargo test --locked -p storytelling-cli --test story_review
3 executed, 0 passed, 3 failed, exit 101
new commands returned cli.usage / exit 2
```

After implementation, that same test target passed 3/3. The separate native
contract/security target adds selected-subset/order/exact-byte binding, strict
unknown/duplicate/forged/mismatched input cases, still bounds, size/depth boundaries
and cross-profile record rejection. Full-workspace, official-oracle and hosted
results follow below.

The implemented path is read-only. Synthetic Catalog/project sentinels and explicit
input files are compared byte-for-byte; PATH excludes external runtimes in actual
CLI tests. The official oracle checks format only. Independent human profile
review, PR #28 G2 acceptance, media/host observation and execution remain separate.

## Observed local results

- Formatting, strict all-target clippy, full workspace debug and release tests,
  and warning-free Rust docs passed. The accepted composition-only digest vector
  and all previous native tests still pass.
- New native bridge target: 8/8 debug and release, including a fixed digest vector;
  CLI: 3/3; opaque-construction/mutation/deserialization doctests: 3/3.
- Python's independent hashlib confirmed fixture artifact SHA-256
  `5404c0b552ddc4ccc394755c0a9a95e262f47f4a579fdca9b41797b920a838d0`;
  binding vector is `ff87f54300ac968089fa6170f1248920a990284c3961f574225088ce85aad932`.
  The exact video fixture is marked `-text` to prevent Windows checkout newline drift.
- Official OTIO 0.18.1: 2 actual planner/bridge parse-write-read cases passed
  (video rational trim and repeated-source still hold), with no host invocation.
- Existing frozen importer 6, official/source OTIO 8, Brief 10, fixed-window
  planner 12 cases and 6 host-evidence rejection tests passed.
- `python3 scripts/check_repo.py` and `git diff --check` passed; original MIT,
  retained notices, Cargo manifests/lockfile and production dependency closure unchanged.

Self-review checked source/type/time bounds, record/artifact integrity, input-only
I/O, capability/loss changes and the timeout dependency boundary. This is not an
independent review. Same-head hosted checks are recorded on the PR after upload;
no local run is described as hosted or as a human acceptance.
