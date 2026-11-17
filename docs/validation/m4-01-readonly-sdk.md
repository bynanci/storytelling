<!-- SPDX-License-Identifier: MIT -->
# M4-01 / M4-R01 · Read-only SDK implementation evidence

[繁中使用指南](../SDK_CONSUMER_V1.zh-TW.md) · [English guide](../SDK_CONSUMER_V1.md) · [Actual G2 decision](m4-01-sdk-g2-decision.md) · [ADR 0013](../adr/0013-read-only-consumer-facade-v1.md) · [Issue #107](https://github.com/bynanci/storytelling/issues/107)

Implementation base is `0b39c00354f343d7a61821a029777d88a8373254`. The requesting maintainer adopted the exact ADR 0013 candidate for a focused read-only **Draft** implementation; no execution, final SDK/consumer acceptance, merge or G5 decision is inferred. New code delegates existing pure functions and adds only the curated namespace and opaque error adapter. Existing CLI production paths, four runtime crates, dependency graph, schemas and backend behavior are unchanged. New MIT integration glue references existing component fixtures in place with their provenance/notices intact.

## Observed RED and first GREEN

| Phase | Actual observation / retained evidence |
| --- | --- |
| Baseline | Locally built original CLI, before facade code; exact 1,573-byte planner stdout reused once. [Baseline receipt](m4-01-readonly-sdk/baseline.json), [bundle](m4-01-readonly-sdk/baseline-bundle.stdout), [OTIO](m4-01-readonly-sdk/baseline-otio.stdout). |
| Independent consumer RED | Cargo check exited 101 / E0432: `consumer_v1` absent. [Receipt](m4-01-readonly-sdk/red-consumer.json), [raw stderr](m4-01-readonly-sdk/red-consumer.stderr). This is compile-time RED, not a runtime assertion or infrastructure failure. |
| Eleven-test RED | All eleven integration tests were written/frozen before implementation; missing-module compile exited 101. Source SHA-256 `902030098aaf7ce33a4e6342de7f065cb64756abeab123404903656a9dee8a54`; [receipt](m4-01-readonly-sdk/red-parity.json) and [stderr](m4-01-readonly-sdk/red-parity.stderr). RED source commit `e4945a8` preserves assertions; rustfmt later changes formatting only. |
| First runtime GREEN | All eleven real CLI/SDK integration tests passed at the first implementation run, no skips. [Source-bound receipt](m4-01-readonly-sdk/adapter-attempt-1.json), [raw stdout](m4-01-readonly-sdk/adapter-attempt-1.stdout). |
| Review-driven fault regression | Independent review found parsed-JSON comparisons missed serializer ordering. Three literal raw-error tests were added without changing the original eleven assertions. A temporary, explicitly controlled `Value`-roundtrip mutant gave **12 pass / 2 fail** (planner and OTIO order); narrative order was already alphabetical. Correct production bytes were restored exactly; **14/14 passed**. [Mutant receipt](m4-01-readonly-sdk/error-order-mutant-red.json), [raw failure](m4-01-readonly-sdk/error-order-mutant-red.stdout), [restored GREEN](m4-01-readonly-sdk/error-order-green.json). This is a test-gap experiment, not a naturally observed production defect. No production fault hook or mutant is committed. |
| Encoding/opacity/consumer | One test-only failing serializer proves the fixed encoding-error branch. Engine doctests: 22 pass, including 15 new compile-fail cases. Standalone consumer actually compiles/runs, reporting unresolved coverage with planning=false. [Commands/hashes](m4-01-readonly-sdk/focused-checks.json); raw green outputs are retained alongside it. |

The serializer test exercises the production generic encoding boundary without adding a production fault switch. Current backend error payloads normally serialize successfully; no natural backend serialization failure or human consumer session was invented. Early read-only AI [backend](m4-01-readonly-sdk/backend-audit.txt) and [golden](m4-01-readonly-sdk/golden-audit.txt) audits identified concrete integration hazards; they are source inspection, not human acceptance or executed tests.

## Requirement/test map

| ID | Real verification |
| --- | --- |
| SDK-G01 | Literal validation fields and unchanged exact CLI JSON; bad range; file/stdin and sentinel/input preservation. |
| SDK-G02 | Reviewed revision 2 and original revision 1; literal provenance/variant fields; existing text goldens. Private CLI formatter consumes the SDK projection in a unit test. No new narrative JSON serializer. |
| SDK-G03 | Historical revisions and author choices, exact shared/local direct impacts; untrusted self-confirmation rejection. |
| SDK-G04 | Complete/unresolved objects, row order, rational ranges, existing coverage diagnostic codes/paths. Unresolved is successful with planning blocked. |
| SDK-G05 | Literal selected order/windows/duration and hard-conflict evidence; same CLI domain result and absent authority flags. |
| SDK-G06 | One frozen original proposal byte sequence; SDK payload plus exactly one CLI LF equals both actual CLI and frozen bundle/OTIO stdout. Fixed identity vectors retained. |
| SDK-G07 | Fresh/stale/malformed distinction; URI/cut/context/loss changes and stale field ordering; source-hash mismatch and artifact tampering remain errors. |
| SDK-G08 | Profile-specific malformed/duplicate/future/null/rich errors; full conflicts/relation_ids/losses; exact-byte bounds/depth/compound priority, including query ID-first. |
| SDK-G09 | Owned result lifetime and Send/Sync types; deterministic sequential/caller-thread replay and unchanged inputs/files/sentinels. |
| SDK-G10 | Fifteen new compile-fail cases preserve unavailable mutation/version symbols, opaque constructors/deserialization and immutable review storage. Positive independent consumer confirms the namespace exists. |

Tests are in [consumer_sdk.rs](../../crates/storytelling-cli/tests/consumer_sdk.rs), [private formatter test](../../crates/storytelling-cli/src/main.rs), [encoding/doctests](../../crates/storytelling-engine/src/consumer_v1.rs) and the [standalone example](../../examples/consumer-sdk-v1/src/main.rs). They import the facade for domain calls rather than core/private backends. Existing Planner, Narrative, N-11, OTIO, review/approval and journal regressions remain unchanged.

Audit corrections are preserved explicitly: the original G02 inventory named a reviewed fixture while its anchor used the authoring fixture; both are now tested. Reviewed close-brief prose mentions excluded C01, so source tags, not text substring absence, define scope. CLI JSON envelopes differ by command. SDK source-error serializer field order is compared separately from CLI `Value` envelopes. A hash mismatch can reject rather than return stale. Original design inventory remains a historical proposal, not a fabricated passed run.

## Workspace, review and hosted checks

[Initial workspace receipts](m4-01-readonly-sdk/workspace-checks.json) bind outputs to code commit `ca98398092948962e3c8860a01919a9d60bd8d13`. After the review-driven test addition, [final workspace receipts](m4-01-readonly-sdk/final-workspace-checks.json) bind the repeated required checks to source/test commit `77e4b217eee9afa863943875594a62c33d167cda`; the production module is byte-identical between these commits. Default/all-feature debug/release, fmt, Clippy, rustdoc and repository checks are tracked separately. Final PR head/OS results are recorded on the Draft PR; cancelled or earlier-head results cannot substitute. The added CI step compiles/runs the standalone consumer on Windows/Linux/macOS; quality also checks its formatting and Clippy. No hosted workflow calls a model or an editor for this feature.

[Fresh-context AI review](m4-01-readonly-sdk/implementation-review.txt) found no unresolved correctness/security blocker at `77e4b21`, and independently verified the resolved test gap, exact original eleven assertions, controlled mutant/final source hashes, error bytes, fixture identity and type closure. It used source/hash inspection, not an independent build, human review or Host execution. G4 runtime/hosted outcomes are the separate implementer/CI records.

Final local results: default debug/release **350 passed each**, all-feature debug/release **369 passed each**, with **3 existing ignored** per run. The ignored cases are two informational planner benchmarks and the pinned isolated FilmCraft integration; none was executed or counted as Host evidence. Workspace/example fmt and Clippy `-D warnings`, rustdoc `-D warnings`, repository links/architecture and whitespace checks all passed. [Preservation and exact counts](m4-01-readonly-sdk/preservation-and-summary.json) verify 194 unchanged original crate/contract/fixture files, unchanged CLI production bytes, and no new third-party package version/checksum. [Receipt index](m4-01-readonly-sdk/receipt-index.json) records exact file lengths/hashes for retained receipts and code/fixture identities. Recheck retained bytes without executing a model or Host:

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib, json
root = Path('docs/validation/m4-01-readonly-sdk')
index = json.loads((root / 'receipt-index.json').read_bytes())
for item in index['receipts']:
    raw = (root / item['path']).read_bytes()
    assert len(raw) == item['bytes']
    assert hashlib.sha256(raw).hexdigest() == item['sha256']
for item in index['source_and_fixtures']:
    raw = Path(item['path']).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == item['sha256']
print('SDK code/fixture identities and retained receipts match; human/Host observations: 0')
PY
```

Source-informed original fixtures remain unchanged. `.gitattributes` preserves exact new proposal/baseline bytes and the context/text golden across checkouts; raw compiler/test streams are retained, including terminal blank lines. The independent consumer's locked third-party package versions/checksums are checked against the unchanged workspace lock. Four runtime workspace members remain intact.

## Remaining acceptance and rollback

This is automatic engineering evidence plus separately identified AI review, not observed human Rust/TypeScript SDK usability. Human participants and Host observations added: **0**. #107 remains Open and the PR remains Draft for final implementation acceptance. Execution/authority/recovery and real Host evidence under #103–#106 remain unavailable; #108 transport/#109 app/second Host/beta/G5 retain their own gates.

No public factory can synthesize the trusted compiled handle. A portable result/record is still a declaration, never an approval token. Any inability to preserve exact serializer/digest/error/ownership semantics is a stop condition; repair or narrow the facade rather than widening existing contracts. Rollback removes only the new facade/consumer imports/example and its tests; no project, Catalog, media or wire-data migration is needed.
