# FilmCraft Host Proof Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prove one real FilmCraft v0.18.0 25 fps picture-only round trip and verify its supplied source bytes in native Rust.

**Architecture:** The pure core verifies bounded buffers; the existing CLI reads an explicit input. A separate test-only Python harness generates synthetic media, invokes pinned release tools and checks native/readback/movie evidence. No production host adapter is added.

**Tech Stack:** Rust 1.85; serde; proposed sha2 exact 0.10.9 software implementation; Python stdlib; FFmpeg/FFprobe; official external FilmCraft CLI v0.18.0.

**Spec:** [FilmCraft picture-only 25 fps proof v1](../../migration/FILMCRAFT_25_V1.md).

## Global Constraints

- Rust remains 1.85 with exactly the existing three implemented crates.
- Explicit source buffer limit is 16,777,216 bytes; existing JSON limit stays 1 MiB.
- Native core remains I/O-free; original MIT LICENSE bytes remain unchanged.
- Source-informed components carry Apache-2.0 SPDX and pinned per-file provenance.
- No merge before M1-02 lands and exact-head hosted CI passes.

## Review Focus

- Hash reports cannot be deserialized into trusted verified buffers; compile-fail API examples pin this boundary.
- Over-budget files must stop after the limit plus one byte; CLI tests exercise domain failure without leaking paths.
- Host import may exit zero with JSON errors; harness rejects structured errors/offline media/reports.
- Native source times and repeated source identity must survive reopen and OTIO reimport; external mapping and exact native tick assertions pin this.
- Valid-looking encoded output can have wrong frames; FFprobe timestamps/count and every decoded frame color pin order/hold lengths, while exact native ticks prove trim offsets.

### Task 1: Native explicit-buffer evidence

**Files:** Create core `src/source_evidence.rs`, core `tests/source_evidence.rs`, CLI `tests/source_evidence.rs`; modify workspace/core Cargo manifests and actual Cargo.lock, core/engine exports, CLI input dispatch and `scripts/check_repo.py`; add `docs/migration/DEPENDENCY_REVIEW_M1_03.md`.

**Interfaces:** Produce `verify_supplied_sha256(&[u8], &str) -> Result<VerifiedBufferV1, ValidationError>`, immutable getters `bytes()`, `byte_length()`, `sha256()`, and CLI `verify-source <FILE|-> <SHA256>` as specified.

- [x] Freeze G2 and obtain independent dependency/contract review before production code.
- [x] Write tests for known empty/abc/binary vectors, mismatch, malformed hashes, boundary/over-budget inputs, immutable owned bytes and metadata-only serialization.
- [x] Run RED: CLI currently returns usage for valid `verify-source`, and core stub rejects a standard valid vector. Save actual failures in the ignored ledger.
- [x] Implement the specified bounded pure digest API and explicit CLI reader; generate Cargo.lock with Cargo 1.85. Keep the purity guard and add only reviewed SHA2 allowance.
- [x] Run whole workspace tests, rustfmt, clippy and repository guard; record GREEN and review lockfile dependency closure.

### Task 2: Real isolated host proof

**Files:** Create `tests/oracles/filmcraft-25-v1/{README.md,run.py,evidence.json}`; add pinned `docs/migration/provenance-m1-03.json`, requirements M1-Hxx, task/work tracking and evidence notes.

**Interfaces:** Harness consumes explicit Storytelling/FilmCraft/FFmpeg/FFprobe executables and a fresh run root; produces validated JSON evidence and external ID mapping, with full bulky artifacts only in the run root.

- [x] Build harness assertions first; negative self-tests must reject nonempty import errors, offline media, unexpected native ticks and wrong encoded frame order.
- [x] Generate authorized synthetic sources and verify through native CLI; emit M1-02 OTIO and assert common clock 25.
- [x] Execute new/import/inspect/native save/reopen/CPU encoded export/OTIO readback/reimport with explicit paths and isolated XDG; assert every exact profile invariant and all structured warnings/errors.
- [x] Independently check FFprobe timestamps/count/duration and each frame color; verify source/empty-project immutability and collect hashes/ID mappings.
- [x] After parent lands M1-02, rebase main, build the actual candidate and rerun complete proof; replace provisional evidence with final hashes and actual outcomes.
- [ ] Run all repository checks, focused independent review and exact-head hosted CI; create focused tracking issue and Draft PR, then merge only after parent signal and green same-head CI under existing user authorization.
