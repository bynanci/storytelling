# ADR 0002: Strict wire DTOs and exact rational source ranges

Date: 2026-11-07. Status: proposed for maintainer acceptance; implemented M0 boundary.

Decision: use private reduced nonnegative rational time with u64 decimal strings at
the wire boundary and u128 cross-products for comparison. Reject zero denominators,
noncanonical/overflowing integers and unreduced values. Half-open source ranges must
fit positive declared duration. Unknown JSON fields fail closed; wire DTOs remain
explicitly untrusted until validation.

Alternatives: f64 seconds were simpler but lose large integer precision; one giant
tick clock couples every consumer to a chosen timebase. Neither is needed for M0.

Consequences: no arithmetic sums, rate conversion or host snapping is promised.
Future SourceTime/TimelineTime/FrameIndex/SampleIndex types must not be conflated.
The successful report is metadata-only, not source-hash or host verification.

See [contract](../CONTRACTS.md) and [requirements](../REQUIREMENTS.md).
