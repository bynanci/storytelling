# ADR 0001: Pure Rust story foundation, not a second editor

Date: 2026-11-07. Status: proposed for maintainer acceptance; implemented M0 boundary.

Context: OpenFilm already has media analysis, story, editing and rendering. A new
Rust repository should extract valuable decisions without replacing the entire NLE.

Decision: start with core -> engine -> CLI in dependency order. Core/engine are pure.
Implement a real read-only fixture validator. Defer workflow/adapters to executable
slices. Preserve OpenFilm authority and target FilmCraft through external contracts.

Rejected: copying all OpenFilm modules, forking FilmCraft, linking private FilmCraft
crates immediately, or generating many empty crates and every language binding.

Consequences: initial functionality is deliberately narrow. A real finite handoff
must succeed before broad SDK extraction. No GPU/codec/mixer work is implied.

Review evidence: [requirements](../REQUIREMENTS.md), [plan](../superpowers/plans/2026-11-07-foundation.md).
