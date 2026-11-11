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
CLI tests. The official oracle checks format only. No independent human review is asserted; PR #28 G2 acceptance, media/host
observation, human editorial acceptance and execution remain separate.

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

## PR #44 delivery receipt

Head `42329d987f032ba979d76b4b0c1db993102a8383`, tree
`5b36af3c76656924359f5043338c7fde027fadc9` merged as
`29efb79e4f68291ffd70259573a63e9f2bf2f113`; the merge tree equals the verified head.
Both head runs [42931642146](https://github.com/bynanci/storytelling/actions/runs/42931642146)
and [42931558985](https://github.com/bynanci/storytelling/actions/runs/42931558985)
passed all six jobs, including the new official-parser bridge step. Post-merge
main run [42932270960](https://github.com/bynanci/storytelling/actions/runs/42932270960)
also passed Quality, Required, Linux, Windows, macOS and Dependency audit.
The fixed artifact/binding vector passed on all three OSes.

This integrates/accepts the bounded technical delivery under the user's standing
verified-merge instruction. It is technical self-review, not an independent human
G2 approval or editorial sign-off; PR #28 acceptance remains pending.


## M3-PB07 bundle-allocation follow-up

The new `export_story_otio` API and CLI path return the shared compile/bind
artifact directly; freshness checking uses the same helper without building a
portable bundle. The prepare command remains the only bundle-producing path.
The parity assertion passed, the fixed artifact/binding vector remains unchanged,
and existing stale/tamper regressions passed in debug and release workspace suites.

PR [#46](https://github.com/bynanci/storytelling/pull/46) head `1a77e4fd0c942ece58223135013d83a744d96921`
(tree `28aa466ed03737ccb13e364296dfe7fc9b9025ea`) merged as `d9778ebdd4b7666cff0c47889c81f6c91079acce`. Exact-head runs
[42935098963](https://github.com/bynanci/storytelling/actions/runs/42935098963) and
[42935144867](https://github.com/bynanci/storytelling/actions/runs/42935144867), and
post-merge run [42935450529](https://github.com/bynanci/storytelling/actions/runs/42935450529),
all passed six jobs.

This removes construction of an unused bundle on two paths; no memory or latency
measurement is claimed. All inputs remain caller-supplied declarations; no host,
Catalog, media or editor project is opened.
