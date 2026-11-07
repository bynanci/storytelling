<!-- SPDX-License-Identifier: MIT AND Apache-2.0 -->
# M3-01 dependency review

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

`cargo audit --file Cargo.lock` passed on 2026-11-07 (exit 0), loading 1,422
advisories from database commit
`309392b18f6b17022f82c701f300433126901068` (2026-11-06T16:34:32+01:00) and
scanning 39 locked crate packages. It emitted no advisory findings for this
lockfile. This is a snapshot, not a guarantee against later advisories; the
exact-head CI audit remains required.

`cargo tree -p storytelling-core -e features --locked` showed only the
`ed25519-dalek/fast` feature and Curve25519 precomputed tables for the new direct
dependency; `rand_core`, `zeroize`, `hazmat`, `pem` and `pkcs8` are not enabled.
The source guard also rejects secret-key/signer/RNG API identifiers in the M3-01
module. Independent security review remains pending.
