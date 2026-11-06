# Rust foundation design

Status: implementation scope requested by the maintainer, 2026-11-07.

## Intent
Extract a local-first story planning and reviewed-edit workflow SDK, not a new nonlinear editor. Preserve OpenFilm as a consumer and FilmCraft as a future host.

## Scope
M0 delivers three crates (core, engine, CLI), strict bounded JSON plan validation, exact nonnegative rational source intervals, synthetic fixtures, tests, CI and traceable SDLC documentation. Validation checks declared facts only; it does not read source media, authenticate a hash, plan a story or invoke an editor.

## Approaches
A whole OpenFilm rewrite and a FilmCraft fork are excluded. Use an independent pure Rust contract/engine and thin process boundary. Add workflow/adapters only when real host execution is needed.

## Decisions
Keep the repository's existing MIT LICENSE unchanged. This bootstrap writes new implementation and synthetic tests, and imports no OpenFilm implementation or media. Future Apache-2.0 extraction requires per-file provenance and license review. Rust 2024, resolver 3, pinned Rust 1.85.0; this is a reproducible bootstrap baseline, not a claim to use the newest toolchain. No unsafe in first-party crates.

## Contract
Schema v1 is an experimental structural subset: plan_id, declared assets (id, SHA-256, duration), ordered shots (id, asset_id, source_range). Time is a reduced nonnegative rational with canonical u64 decimal strings. Bounds: 1 MiB JSON, 1024 assets, 4096 shots, 128 ASCII identifier bytes. References, duplicate identities, interval order and duration bounds must validate. Unknown fields and malformed JSON fail closed.

## Delivery
Branch/PR only; no main merge, tag, package publication, branch protection change, private media upload or signature on behalf of the maintainer. Build/test evidence must distinguish the local environment (no Rust or outbound DNS) from actual hosted Rust checks.
