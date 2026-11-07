# M1-03 dependency decision

Approved sole new direct dependency: `sha2 = "=0.10.9"`,
`default-features = false`, `features = ["force-soft"]`, in the pure core.
Its crates.io normalized manifest identifies edition 2018, MIT OR Apache-2.0,
upstream https://github.com/RustCrypto/hashes, no build script, optional assembly
disabled. `force-soft` selects portable Rust compression rather than CPU-specific
SHA instructions. It is not an unsafe-free dependency claim: SHA2 contains a
documented layout cast, and `cpufeatures` remains a target dependency. Own crates
continue to forbid unsafe code and preserve the pure I/O guard.

Justification: real SHA-256 over bounded caller-supplied bytes closes the native
digest gap for this proof without shelling out, reading paths in core, adding a
crate or hand-writing cryptography. The verified-buffer boundary is narrower
than pinned OpenFilm `withVerifiedSourceFile` and must not be called filesystem
identity or TOCTOU protection. No runtime, network, SDK or host dependency enters
the product.

Independent G2 review approved this exact bounded design before implementation.
Functional RED was observed for valid core vectors and CLI input before hashing
and dispatch were implemented. Rust 1.85 whole-workspace tests and clippy passed.
Actual `cargo tree -e features` confirms `sha2/force-soft`, no SHA2 std/assembly
features. `generic-array` has a build script using `version_check`; the no-build
script statement applies only to SHA2 itself.

The actual generated lock adds SHA2 0.10.9, cfg-if 1.0.5, cpufeatures 0.2.17,
digest 0.10.7, block-buffer 0.10.4, crypto-common 0.1.7, generic-array 0.14.7,
typenum 1.20.1, version_check 0.9.5 and target-conditional libc 0.2.191. All ten
downloaded archive checksums match Cargo.lock; inspected manifests identify
MIT OR Apache-2.0 except generic-array (MIT). The conditional libc manifest
declares Rust 1.65 and MIT OR Apache-2.0. Actual archive/manifest details are in
[dependency evidence](dependency-evidence-m1-03.json). Linux x86_64 does not build
libc through this edge; cross-platform CI remains required. No universal target
or formal cryptography audit is claimed.

The 16 MiB bound limits input bytes, not total process memory: the caller's
buffer and the verified owned copy can coexist. Metadata serialization is a
report; it cannot reconstruct authority over later filesystem bytes.
