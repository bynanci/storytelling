<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# PR #28 approval-candidate dependency review

This records a bounded dependency review for the concurrent PR #28 candidate,
not G2 acceptance of the canonical M3-01/P1 finite-OTIO review-binding scope.
Independent G2 review of the candidate remains pending.

## Direct dependency change

- `ed25519-dalek = 3.0.0`: direct pure-core Ed25519 verifier; workspace pin is
  exact, default features are disabled, and only `fast` is enabled.
- Crates.io metadata observed with `cargo info ed25519-dalek@3.0.0`: MSRV 1.85,
  BSD-3-Clause. This matches the workspace Rust 1.85 policy.
- Production code imports only `Signature` and `VerifyingKey`, calling
  `VerifyingKey::from_bytes` and `verify_strict`. It does not call signing,
  key-generation, random-number, private-key, PEM, or serialization APIs.
- `serde_json` is already pinned in the workspace at `1.0.145`; core now declares
  this existing codec directly because strict JSON parsing and compact payload
  serialization are part of the public contract. No additional JSON codec was added.

The lockfile adds `ed25519-dalek 3.0.0` (BSD-3-Clause), `ed25519 3.0.0`
(Apache-2.0 OR MIT), `curve25519-dalek 5.0.0` (BSD-3-Clause),
`curve25519-dalek-derive 0.1.1` (MIT/Apache-2.0), `signature 3.0.0`
(Apache-2.0 OR MIT), `sha2 0.11.0`, `digest 0.11.3`, `block-buffer 0.12.1`,
`crypto-common 0.2.2`, `hybrid-array 0.4.15`, `fiat-crypto 0.3.0`, `subtle 2.6.1`
(BSD-3-Clause), `cpufeatures 0.3.1`, `rustc_version 0.4.1` and `semver 1.0.28`;
the SHA-2 support crates use MIT/Apache-2.0, and `fiat-crypto` uses
MIT/Apache-2.0/BSD-1-Clause. Cargo metadata reports every license expression;
the full transitive closure is locked. Cargo selected Rust-1.85-compatible
versions.

## Test material and secrets

The tests use the public RFC 8032 known-answer `(public key, message, signature)`
vector and a separate synthetic approval signature created with a one-off local
key. The generated private key was held only in an automatically removed
TemporaryDirectory; only its public key and detached signature are in the test.
No private seed, key, credential, user identity or personal media is checked in.

## Advisory state

`cargo audit --file Cargo.lock` passed locally on 2026-11-07 (exit 0), loading 1,422
advisories from database commit
`309392b18f6b17022f82c701f300433126901068` (2026-11-06T16:34:32+01:00) and
scanning 39 locked crate packages. It emitted no advisory findings for this
lockfile. The audited `Cargo.lock` SHA-256 was
`ac030012f2595fe5eb118cb2640a2e2e06c858f72d72f76cad0fa82313112bb0` and remains
unchanged through candidate head `11741c5dbc982a76230fe89eefc974c5bf966ca8`.

The workflow now defines a dedicated hosted Dependency audit job. It downloads
Cargo Audit `0.22.1` from the RustSec release asset and checks the asset SHA-256
(`1890badd5f15831a9af4b074399fcd21e6f7c0fe42c84e9254cdffc9f813765c`) before
running `cargo audit --file Cargo.lock`; the Required gate depends on this job.
The candidate head must pass this new job before its hosted audit can be claimed.
The job fetches the current RustSec advisory database, so each run is a
time-specific scan rather than a guarantee against later advisories. The local
snapshot above remains separate evidence.

`cargo tree -p storytelling-core -e features --locked` showed only the
`ed25519-dalek/fast` feature and Curve25519 precomputed tables for the new direct
dependency; `rand_core`, `zeroize`, `hazmat`, `pem` and `pkcs8` are not enabled.
The source guard also rejects secret-key/signer/RNG API identifiers in the PR #28
module. Independent security review remains pending.
