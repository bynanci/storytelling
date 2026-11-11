<!-- SPDX-License-Identifier: MIT -->
# M3-01/P1 source-informed comparison

Status: **static source map plus bounded paired runtime oracle**. OpenFilm is pinned to
`7add4762fe8174d76f43b863375ea39b73121e76`. The table below records source behavior inspected
by path/blob identity. A later synthetic run exercised the actual `NleHandoffService.review/publish`
runtime and corresponding Rust CLI outcomes; see the [runtime record](M3-01-RUNTIME-ORACLE-2026-11-08.md).
This small comparison is not full runtime parity, source verification or acceptance of M3-01.
Independent profile/G2 review remains open.

| Pinned OpenFilm source/test | Observed policy from inspected source/test | Rust candidate correspondence | Intentional boundary/delta |
| --- | --- | --- | --- |
| `packages/application/src/nle-handoff.ts`, blob `7af8931a969e4eb13abe92b3db0b6b859c494e1c`: `HandoffReview` | Review state carries declared project ID, revision and handoff plan. | Context binds `project_id`, `target_revision`; identity binds typed importer plan, artifact and report. | This does not observe the live editor project or prove a host revision. Fields remain caller declarations. |
| `packages/application/src/scene-script-review.ts`, blob `11c4bcfa1b29e3094f636787a5010de29b445461`: `savedScriptProjectIdentity` | Migrates supported legacy project shape, removes incidental `updatedAt`, then derives a JSON-string identity. | Context fields receive separate domain-separated SHA-256; review binds exact composition bytes. | Raw-byte identity intentionally stales on whitespace/lexeme changes that upstream's normalized saved-project identity can ignore. No NFC normalization or general JCS is claimed. |
| `packages/application/tests/story-plan.test.ts`, blob `7ac3357de7aa9885f893aece8a9ec3effc2bb5c5`: stale story/graph after preview | A plan preview becomes stale when its underlying story or graph changes. | `changed_composition_and_each_declared_context_field_are_reported_stale`; `semantic_shot_order_change_invalidates_plan_and_compiled_artifact_binding`. | The Rust test mutates finite composition bytes/plan semantics; it does not port the full story graph or planner. |
| `packages/application/tests/silence-review.test.ts`, blob `11fbcd692e37db9ef106b8689e3e3b64fcd2cbc2`: altered review envelope, cross-project copy, stale timeline and same-size source change | Review validity depends on the reviewed context/source state and altered or cross-project records are rejected. | `review_record_rejects_unknown_duplicate_forged_claims_and_invalid_digests`; `every_declared_context_field_has_a_stable_stale_identifier`; exact composition digest is checked. | Rust record SHA-256 is unkeyed and portable; it cannot establish who created or approved a record. Source-file identity/TOCTOU is not implemented. |
| `packages/application/tests/silence-jobs.test.ts`, blob `bd383faf764ad78f6059ce4792420865d1aa8823`: acknowledgement failure, changed request reuse and exact replay | Source runtime retains receipts across acknowledgement failure and distinguishes request/replay cases. | M3-02 PR #56 adds bounded operation/request idempotency and recovery state. | The journal does not consume approval nonces or provide approval replay protection, host retry execution or exactly-once effects. |
| `packages/core/src/preset-json.ts`, blob `d9d232c4255acdd9510153e15a0347c13a4b08fd`: bounded strict JSON preflight | Source includes a bounded strict parse boundary for canonical preset JSON. | `context_parser_rejects_unknown_duplicate_unsorted_and_over_budget_values`; `json_depth_scanner_accepts_exact_limit_and_ignores_string_contents`; `json_depth_and_record_byte_budgets_are_enforced_before_deserialization`. | New context/record DTOs use Rust `serde_json`; this does not claim general cross-runtime canonical JSON parity. |

The Rust implementation's typed domain-framed SHA-256 is an intentional identity-encoding change
from upstream's local `JSON.stringify` identities and other source-local fingerprints. Existing
Rust tests provide synthetic input mutations and deterministic expected outcomes. They are
regression tests informed by the pinned source behaviors above, not copied upstream vectors or a
live two-runtime oracle. No OpenFilm code, test, fixture or media was imported.

The digest has no secret/key and proves no signature. A mathematically valid signature would still
not prove the key is trusted or authorized; authorization would require an independent trust-root
and revocation policy. Separately, this Rust builder does bind its OTIO output to the exact supplied
composition bytes by compiling from the same slice and re-compiling for freshness, but it does not
prove the bytes came from a verified source build, an observed host snapshot or a reviewed planner
proposal. These claims must not be combined.

## Follow-up runtime evidence

The [2026-11-08 evidence audit](M3-01-EVIDENCE-AUDIT-2026-11-08.md) corrected the story-plan
test blob above and documented why its first pass was static-only. The follow-up
[runtime record](M3-01-RUNTIME-ORACLE-2026-11-08.md) now executes the pinned OpenFilm service
alongside corresponding Rust CLI cases, adds a fixed Rust digest vector and freezes API/CLI error
precedence. Input-domain and digest-encoding differences remain explicit; this does not establish
complete upstream parity or resolve independent G2/profile acceptance.
