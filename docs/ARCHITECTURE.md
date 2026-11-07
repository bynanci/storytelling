# Architecture

Status: M0 validation and M1-01 finite visual import candidates, plus future design.
See [M0 contract](CONTRACTS.md) and [finite-v1 contract](migration/FINITE_V1.md).

## Implemented dependency direction

```text
CLI: local bytes / stdout / exit codes
  -> engine: bounded validation + pure finite snapshot mapping
    -> core: strict DTOs + rational source time + typed errors
```

Core uses Serde only. Engine adds serde_json but has no filesystem, clocks, random
state, database, networking or editor dependency. CLI owns I/O. `scripts/check_repo.py`
checks direct dependency boundaries; review must also examine transitive dependencies
and behavior. First-party unsafe is forbidden, not a claim about every dependency.

## Future boundaries, not empty crates

`workflow -> engine + core`: application use cases and ports.
`adapters -> workflow ports + core`: SQLite, OpenFilm reader, OTIO, FilmCraft and
optional provider implementations. `cli -> workflow + adapters` composes them.
Add these crates only with an executable slice, not for speculative architecture.
TypeScript initially wraps the CLI/JSON protocol. MCP, WASM and HTTP are transports;
no transport owns planning or approval policy.

## Authority

| Data | Authority |
| --- | --- |
| Story intent, constraints, curated membership, proposals and decisions | storytelling |
| Source bytes, actual filesystem access and media analysis | authorized producer/adapter |
| OpenFilm Composition, Catalog and history | OpenFilm application services |
| Host timeline after handoff and user finishing | target editor |
| Mapping/approval/execution/verification records | storytelling workflow |

Plan documents are immutable proposal inputs, not a second editable timeline.
The M0 subset does not model images, tracks, speed, audio automation or host effects;
unsupported fields are refused. M1-01 separately maps a strict composition-export
profile into this subset, retaining complete source facts and structured limitations.
It is not general OpenFilm-project, media or host compatibility.

## Time and identity

M0 time uses reduced nonnegative u64 numerator/denominator strings. Private valid
`RationalTime` values compare through u128 cross-products, without float rounding.
Ranges are half-open `[start,end)`. No FPS/sample snapping or denominator guessing.
Future source/timeline/frame/sample wrappers must remain distinct; host quantization
must produce an explicit report. Decimal 29.97 is not silently 30000/1001.

Asset identity and shot occurrence identity are distinct. Reusing one asset twice is
valid. A SHA string is a declaration only; source byte verification is future adapter
work. M0 accepts no paths, URLs, commands, model credentials or arbitrary extension
fields inside the portable document.

## Future execution safety

A compiled plan must bind the proposal, actual sources, compiler, host capabilities
and explicit loss choices. Human approval comes from a trusted surface, not an LLM
`confirmed:true`. Start in a new isolated host project. External timeout becomes
unknown -> readback/reconcile, never blind retry or Undo N times. SQLite cannot make
host effects transactional. Execution receipts distinguish succeeded, failed,
partial and unknown; planning, job lifecycle and freshness stay separate.

## Decisions

[ADR 0001](adr/0001-scope-and-boundaries.md) records the Rust extraction strategy;
[ADR 0002](adr/0002-exact-time-and-wire.md) records time and wire constraints;
[ADR 0003](adr/0003-license-and-provenance.md) preserves the actual repository license.
