<!-- SPDX-License-Identifier: MIT -->
# M3-01 bounded upstream runtime oracle — 2026-11-08

This follow-up runs the pinned OpenFilm review service and the Storytelling
read-only CLI against corresponding synthetic review cases. It closes the
previously missing *small runtime-oracle execution* for this finite input domain.
It does not establish digest parity, a full service port, an approved profile,
independent G2 acceptance, real-host behavior or media correctness.

## Pinned inputs and commands

- OpenFilm source: `7add4762fe8174d76f43b863375ea39b73121e76`.
- Synthetic Rust/host composition fixture:
  `tests/fixtures/openfilm/finite-v1/repeat-source.json`, SHA-256
  `04e320f4fe6c1ebd65c11c0392ab48b3c61abc8ce44de09c1e7ffccd1ef087b9`.
- The fixture's `.gitattributes` entry disables text conversion because the
  importer and identity hash the exact embedded bytes on every platform.
- Rust dependency lock SHA-256:
  `f3a69dd97731666785474156249bd1c27d2d1c3048162edfeb111e98024d7e66`.
- The source harness is [m3-01-openfilm-runtime.mjs](../../../scripts/oracles/m3-01-openfilm-runtime.mjs).
  It dynamically imports the actual pinned `NleHandoffService` module, creates
  a synthetic saved project in an OS temporary directory and checks review and
  stale-publish outcomes. It does not copy or reimplement upstream algorithms.

With the pinned OpenFilm checkout as the working directory and a test-only
`tsx` loader available, run:

```sh
OPENFILM_SOURCE_SHA=7add4762fe8174d76f43b863375ea39b73121e76 \
OPENFILM_SOURCE_ROOT=/path/to/open-film \
STORYTELLING_ROOT=/path/to/storytelling \
node --import /path/to/tsx/dist/loader.mjs \
  /path/to/storytelling/scripts/oracles/m3-01-openfilm-runtime.mjs
```

The local run used Node 24.22.1 and `tsx` 4.23.15. Its source-runtime result:

```json
{
  "source": {
    "revision": "7add4762fe8174d76f43b863375ea39b73121e76",
    "service": "NleHandoffService.review/publish",
    "unchanged": "same revision",
    "updated_at_only": "same revision",
    "project_id_change": "changed revision; old publish rejected",
    "composition_change": "changed revision; old publish rejected",
    "forged_approval": "unsupported request rejected"
  },
  "fixture_sha256": "04e320f4fe6c1ebd65c11c0392ab48b3c61abc8ce44de09c1e7ffccd1ef087b9"
}
```

The upstream composition change changes one clip occurrence ID while keeping
the finite timeline valid. Its `publish` call rejects the prior review before
staging or writing output. The forged request adds `approved: true`; upstream
rejects the unsupported request field.

The Rust CLI was built from the same workspace with the locked dependencies and
ran `prepare-review` plus `check-review` against the same fixture and a declared
context. Rust reported:

| Case | OpenFilm `NleHandoffService` | Storytelling CLI |
| --- | --- | --- |
| Unchanged composition/context | Repeated review has the same revision | `current:true`, no stale fields |
| Change project identity | New project/review identity; old publish rejected | `current:false`, `context.project_id` |
| Change clip occurrence ID | New revision; old publish rejected | `current:false`, `artifact`, `composition`, `plan` |
| Add forged `approved:true` | Publish request rejected as unsupported | Portable record rejected as `review.invalid_record_json` |

The upstream service ignores a change to `updatedAt` alone and retains the same
review revision. `updatedAt` is outside the Rust review input, so that row is
not a cross-runtime equivalence claim. The two implementations use distinct
input models and digest encodings; matching decisions do not imply digest
parity. The CLI always returned false for `executable`, `approval_granted`,
`source_verified` and `host_verified`.

No media file, renderer, editor process or network service was accessed. The
only upstream file operation was a temporary synthetic `project.json` used by
the actual service's saved-project freshness check.

## Fixed digest vector and error order

The Rust test `same_input_produces_stable_opaque_binding_and_check_recompiles_current_artifact`
now asserts fixed expected values for the exact fixture and context, including
binding `85ee60f9a2ea0327ea977b48f73e6160f769e65921ee64ce7dbef81c3eb14a4f`,
composition `04e320f4fe6c1ebd65c11c0392ab48b3c61abc8ce44de09c1e7ffccd1ef087b9`,
plan `2083a48c4862e1be7289602e5f218dac615f5c98e149141bf42e832bc0da482b`,
artifact `6456c0a184a3cb3105f1b4d5026954df52d12a34006228da69589fcbeb09412b`,
report `5ddb5d4978099360cae5ae70044cdc908cec99b75b0ea27ee2fb1cb2c0408e1d`
and context `59c96b9162978a60a22c21fcaa1542ced70fbc345b526e2b9cc1d976fc5f533c`.
The fixed assertions ran in the Linux/Windows/macOS matrix on final PR #40 head
`a9b80723e0171497d9b1cc63bc30c6fac88e0ecc`. Both six-job head runs
42477209609 and 42477206413 passed; post-merge run 42477392236 also passed all
six jobs. The accepted head and merge tree are identical.

`direct_review_api_error_precedence_is_size_record_context_then_compile` and
`review_cli_reads_arguments_before_domain_validation_and_keeps_error_order`
freeze the API/CLI ordering documented in [CONTRACTS](../../CONTRACTS.md).
They cover combined over-budget inputs, malformed record/context/composition,
and a later missing file. The public CLI reads and size-checks every argument
in command order before engine domain parsing; the direct API validates all
byte budgets before parsing a record, then validates record and context before
recompiling composition bytes.

The follow-up branch passed `cargo fmt --all --check`, strict workspace Clippy,
the complete debug and release workspace suites (219 tests in each),
warning-denied workspace rustdoc, `scripts/check_repo.py`, and `git diff --check`.
The scoped review suite has 11 engine and four CLI integration tests; the new
cases pin fixed cross-platform digests and multi-invalid-input ordering. These
are local results; exact-head hosted Linux, Windows and macOS jobs establish the
cross-platform run for the merged candidate.

The first superseded candidate, `19b38d6cc8f8c9c8dedbc8a8ced2217aeed7f54f`,
failed the fixed aggregate hash on Windows while Linux passed. The final change
marks the embedded source fixture `-text` in `.gitattributes`, preserving its
exact bytes across checkouts, and checks component hashes before the aggregate
digest. Final Windows runs passed on the corrected head.

## Limits and acceptance

The runtime oracle shows corresponding stale-review behavior only for unchanged
input, a project identity change, one composition identity change and a forged
claim. It does not compare cryptographic values, all upstream review services,
all accepted source shapes, trust-key authorization, expiration/revocation,
durable replay, filesystem races, planning output, host observation or execution.
The historical pre-implementation functional RED remains unproven; these new
tests and oracle run do not rewrite that history. Independent G2/profile review
and maintainer acceptance remain separate from this AI-authored evidence.
