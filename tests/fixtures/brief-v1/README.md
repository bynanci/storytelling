<!-- SPDX-License-Identifier: Apache-2.0 -->

# StoryBrief v1 synthetic declarations

These independently authored JSON declarations test the bounded downstream
StoryBrief audit in [STORY_BRIEF_V1.md](../../../docs/migration/STORY_BRIEF_V1.md).
The manual required membership/order and contradiction policy is informed by
`bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76`, particularly
`packages/core/src/models.ts` (`StoryConstraint`, `StoryBeat`; blob
`58dbe93f132577842b7971e0d8aad61aff1d78c6`) and
`packages/solver/src/index.ts` (`compose`, `orderedAssets`; blob
`c701b985495c8c76292880b5acbc9c062b03006c`).

The brief schema, preference warnings and typed diagnostics are downstream
design. These fixtures do not demonstrate full upstream solver or planner
parity. No upstream tests, fixtures, implementation or media are copied. All
identifiers are invented; the fixtures contain no file paths, real media,
private facts, GPS or credentials.

JSON has no comment syntax. This README supplies the license marker for both
JSON files and their test mutations: **SPDX-License-Identifier: Apache-2.0**.
The repository's original MIT LICENSE remains unchanged.

| Fixture | Expected audit |
| --- | --- |
| `valid.json` | Consistent declared constraints; retain manual order, required membership and original preferences; no planned selection or duration proof |
| `conflicting.json` | Unknown required/excluded references, required/excluded overlap, one actual cycle witness and soft preference warnings; keep every declaration |

Large limits, strict-wire refusals and Unicode intent cases are generated in
`crates/storytelling-engine/tests/brief.rs` to avoid committing repetitive data.
An audit only checks declared reference, membership and ordering rules. It does
not inspect source availability, choose media, plan shots or prove feasibility.
