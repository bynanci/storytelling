# Contributing

Start with [TASKS](TASKS.md), [SDLC](docs/SDLC.md), [requirements](docs/REQUIREMENTS.md)
and [AGENTS](AGENTS.md). Choose one scope, inspect existing PRs, and record the base.
Use a branch and PR; do not push feature work directly to main.

### Contributing a skill example or host guide

The portable package lives in [.agents/skills/storytelling](.agents/skills/storytelling).
A useful contribution can be a small example from an unrelated project: include the
author's request, the supplied material and the output you expect. For host
instructions, record the client and scope (project, user or hosted account) and
what you verified: package discovery, actual invocation or only the vendor's documented
installation procedure. Link the relevant [platform guide](docs/PLATFORMS.md) entry.
Keep helper and reference links relative to the skill so copying the whole directory works.

### Checking the project

Install the pinned Rust toolchain for the workspace checks and run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo doc --workspace --no-deps --locked
python3 scripts/check_repo.py
```

Python 3.11+ is needed for the repository metadata/link guard, not for the SDK/CLI.
Do not update Cargo.lock as an incidental effect. Dependency/toolchain updates are
small reviewed PRs with security, license and compatibility notes.

Every behavior change needs a regression and evidence. Use synthetic fixtures and
never upload private source files, local paths, GPS, secrets or user projects.
Describe tests that were not run and distinguish parser tests from real-host gates.

Include Task/Requirement IDs, RED/GREEN evidence, exact head CI, risks and migration
notes in the PR template. A successful code check is not automatic release approval.
No package publication, tags or branch-rule changes without explicit owner authority.

Respect the existing MIT license and per-file origins. Do not add another person's
DCO sign-off. Current bootstrap has no DCO enforcement; adopting it needs a policy
and a valid contributor workflow. AI assistance must be disclosed, and self-review
must not be described as independent review.
