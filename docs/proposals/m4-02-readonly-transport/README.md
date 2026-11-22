<!-- SPDX-License-Identifier: MIT -->
# #108 · Read-only TypeScript transport G2 candidate

[繁體中文](README.zh-TW.md) · [ADR 0014](../../adr/0014-readonly-process-transport-v1.md) · [Decision / evidence](../../validation/m4-02-transport-g2.md)

**Proposed, not accepted or runnable.** This packet recommends a local Node.js
client and one framed stdin/stdout child process per read-only call. It adds no
runtime code, schemas to `contracts/`, dependency, npm package or Host access.

The requester selected this local-process/raw-byte/Node direction on 2026-11-17.
That preference reply does not accept the exact contract or authorize G3.

| Review material | Purpose |
| --- | --- |
| [Protocol](protocol.md) | Exact framing, operations, original bytes, errors/order, limits, cancellation and versions. |
| [Owned TypeScript API/views](types.md) | Public result/error fields independent of Rust-private imports; narrative binding view. |
| [Test plan](test-plan.md) | Named G3/G4 cases, RED before code, regressions, matrix and missing evidence. |
| [Request header schema](request-header.schema.json) / [response header schema](response-header.schema.json) | Closed proposed JSON headers; schemas alone do not check byte framing, duplicate keys or lexical integers. |
| [Narrative projection view schema](narrative-projection-view.schema.json) | Binding-only JSON view, not a new core serializer or NarrativeDocument V2. |
| [Case inventory](cases.json) / [example manifest](examples/manifest.json) | Expected outcomes, original-source hashes and synthetic proposed frame bytes. They are not captured product outputs. |
| [Recheck the packet](verification.md) | Reproduce the static schema/hash checks; all 64 transport runtime cases remain unexecuted. |

## Current executable reference

The accepted Rust workflow is available now:

```sh
cargo run --manifest-path examples/consumer-sdk-v1/Cargo.toml --locked -- \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

It reports declared unresolved coverage with `ready_for_planning:false`.
The new transport CLI and TypeScript package described here do not exist yet.

## G2 review and later implementation

Review the exact protocol, public types, schemas and failure cases together.
Record an actual accept/narrow/reject decision against a PR head. G3 starts only
after that decision; code must first fail meaningful real-entry tests and then
pass them. Keep #108 Open until its actual client, parity, failure tests and
fresh-checkout example pass. No author/Host observation or publication is inferred.

Proposed G3 prerequisites: Rust 1.89.0; Node.js 22.20+; no browser, editor, model,
network service or secret. Reuse the repository's three-OS CI and separately
validate the independent TypeScript consumer/toolchain. The current root npm
package distributes the Skill, so a future SDK consumer lives in its own
unpublished directory with a lockfile. It must not silently alter the Skill
package or its installation contract.

These are independently written first-party MIT integration specifications and
synthetic examples. Existing profiles and fixtures remain in place under their
own licenses/notices; no Apache/OpenFilm source or original receipt is rewritten.
