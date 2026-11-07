<!-- SPDX-License-Identifier: Apache-2.0 -->

# finite-v1 synthetic snapshots

These independently authored declarations exercise the JSON composition envelope
of `exportTimeline("json", composition, assets, settings)` in
`bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76`, specifically
`packages/exporters/src/index.ts` (blob
`ebc6e8cb8945c3bd26bb5b1fdbb65689b2f560ae`) and the model/time contracts in
`packages/core/src/models.ts` and `packages/core/src/timebase.ts`.
No upstream fixture, implementation, media, private path, GPS or credential is
copied. URI values identify nonexistent synthetic media and are never opened.
Every hash is a fabricated declaration, never evidence of actual source bytes.

All JSON files in this directory and their test mutations are source-inspired
Apache-2.0 component fixtures. JSON has no comment syntax, so this file supplies
their SPDX license marker: **SPDX-License-Identifier: Apache-2.0**.
The repository's original MIT LICENSE remains unchanged.

| Fixture | Intended finite-v1 result |
| --- | --- |
| still.json | Accepted; a five-second presentation hold despite supplied image duration 0.5; preserve locks and selection facts |
| video-trim.json | Accepted visual mapping; exact 0.1-to-2.6 trim; audio handoff remains blocked |
| repeat-source.json | Accepted; preserve occurrence order and reuse one source; hold window is maximum hold, not sum |
| future-schema.json | Unsupported envelope version before nested domain validation |
| fractional-rate.json | Accepted exact decimals/exponent; 29.97 is 2997/100; preserve original numeric spellings |
| unknown-effect.json | Unsupported with one structured loss and no accepted plan |

Oversized input, strings, key counts and depth are generated compactly by
`crates/storytelling-engine/tests/openfilm_snapshot.rs`. This avoids committing
a megabyte of repetitive fixture data. Mutations also cover embedded audio,
animation, unsupported edits, continuity, duplicate keys and collection limits.

Expected intentional differences from source legacy behavior are strict exact
continuity, no NTSC inference from 29.97, required lowercase 64-hex hash syntax,
and fail-closed unsupported declarations. Successful import is limited to
declared visual metadata; it does not establish host, media or rendering proof.
