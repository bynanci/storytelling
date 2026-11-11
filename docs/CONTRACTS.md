# CLI and document contracts

<a id="命令列介面與文件契約"></a>

English · [繁體中文](CONTRACTS.zh-TW.md)

Choose the command for your task. Supply the document that its profile accepts. All size limits in this table use bytes. See the [CLI guide](CLI_GUIDE.md) for working examples.

| Command | Required input and limit | Successful standard output | Profile |
| --- | --- | --- | --- |
| `validate` | M0 plan JSON, 1,048,576 | `{"ok":true,"validation":...}` | [M0 plan](#m0-plan-contract) |
| `import-openfilm` | Finite composition JSON, 1,048,576 | `{"ok":true,"import":...}` | [Snapshot import](migration/FINITE_V1.md) |
| `export-otio` | The same snapshot format, 1,048,576 | Native OTIO `Timeline.1` JSON | [Finite OTIO](migration/FINITE_OTIO_V1.md) |
| `audit-brief` | StoryBrief v1 JSON, 1,048,576 | `{"ok":true,"audit":...}` | [Brief audit](migration/STORY_BRIEF_V1.md) |
| `plan-story` | Fixed-window planning document, 1,048,576 | `{"ok":true,"planning":...}` | [Fixed-window planner](migration/FIXED_WINDOW_PLANNER_V1.md) |
| `render-narrative` | NarrativeDocumentV1 JSON, 1,048,576 | Reproducible UTF-8 text. Errors use one JSON line. | [Narrative document](narrative/NARRATIVE_DOCUMENT_V1.md) |
| `track-narrative-revisions` | NarrativeRevisionLedgerV1 JSON, 1,048,576 | Source and variant history, suggestions, author choices and current review status. | [Revision ledger](narrative/NARRATIVE_REVISION_LEDGER_V1.md) |
| `query-narrative-impact` | The same ledger and one source change event ID | Variants that directly reference the changed source revision. | [Revision ledger](narrative/NARRATIVE_REVISION_LEDGER_V1.md) |
| `check-narrative-style` | NarrativeDocumentV1 JSON and NarrativeStyleProfileV1 JSON, each 1,048,576 | `{"ok":true,"style_check":...}`; errors exit 1. | [Editorial style profile V1](narrative/EDITORIAL_STYLE_PROFILE_V1.md) |
| `verify-source` | Supplied bytes, 16,777,216, and expected SHA-256 | `{"ok":true,"verification":...}` | [Supplied buffer](migration/FILMCRAFT_25_V1.md#native-supplied-buffer-contract) |
| `prepare-story-review` | Planner response, 4,194,336. Snapshot, 1,048,576. Context, 65,536. | `{ok:true,review:...,otio:Timeline}` with the actual OTIO artifact. | [Selected proposal](migration/PLANNER_OTIO_REVIEW_V1.md) |
| `export-story-otio` | Same three inputs | The same native OTIO directly. | [Selected proposal](migration/PLANNER_OTIO_REVIEW_V1.md) |
| `check-story-review` | Review bundle, 4,292,672, and current proposal, snapshot and context | `{ok:true,freshness:...}`. A stale result returns exit 1. | [Selected proposal](migration/PLANNER_OTIO_REVIEW_V1.md) |

<a id="讀取輸入與判讀結果"></a>

### Read input and interpret the result

Single-document commands read a named regular file or standard input with `-`. The byte check also needs an expected digest. The three proposal review commands accept standard input only for the proposal. Supply regular files for the snapshot, context and bundle.

Place loss options for `export-otio` after the input. On success, the command writes OTIO directly without an `ok` wrapper. A valid brief with hard conflicts returns exit 1 and `{"ok":false,"audit":...}`. Malformed input returns an `error` object. Use the typed errors from the selected profile.

The input for `plan-story` includes a strict brief, candidate windows, scores and hard bounds for duration and shot count. A successful proposal contains an M0 `plan`. A rejection returns an `error` with typed `conflicts` and no plan. The planner uses declared rank and fixed timing. General feasibility, media quality and host execution each need independent evidence.

Extract `planning.plan` for the M0 validator. It rejects a full `planning` response. `export-otio` accepts a composition snapshot.

Supply an append-only ledger as a file or on standard input. Use `track-narrative-revisions <FILE|->` to replay history and review state. Use `query-narrative-impact <FILE|-> <CHANGE_ID>` with a source revision event ID. The engine computes impact from direct `source_refs`. The caller cannot supply the result list. The engine does not propagate impact through indirect `based_on` links. Any pending variant review blocks the next source revision. Author decisions and assistant suggestions require separate events. The CLI cannot authenticate who submitted an author decision.

The `check-narrative-style` command takes a document and its revision-bound profile. It checks declared spoken beat order and does not evaluate style or author approval.

| Common exit code | Meaning |
| --- | --- |
| 0 | Evaluation succeeded. |
| 1 | Domain rejection or conflict. |
| 2 | Invalid command or arguments. |
| 3 | Input or output failure. |

JSON responses occupy one line. Help, version information and successful narrative commands return plain text. A failed write to standard output can leave a partial result. Each profile has its own collection, depth, diagnostic and output limits. Byte verification limits do not extend JSON limits. These commands do not run editors or grant approval.

<a id="m0-計畫契約"></a>

## M0 plan contract

`export-otio` emits a native OTIO document under the [finite OTIO contract](migration/FINITE_OTIO_V1.md). The caller explicitly chooses conversion losses. The M1-01 command `import-openfilm <FILE|->` reads the separate [finite snapshot format](migration/FINITE_V1.md). Its returned `plan` follows the M0 contract and input size limit. Its visual mapping cannot execute.

`PlanDocumentV1` is an experimental structural subset. The complete SDK and editor input formats have separate scopes. See the [minimal example](../examples/minimal-plan.json) and [wire types](../crates/storytelling-core/src/wire.rs).

<a id="文件結構"></a>
<a id="document"></a>

### Document structure

| Object | Required fields |
| --- | --- |
| Root | `schema_version` (integer 1), `plan_id`, `assets`, `shots` |
| Each asset | `id`, `content_hash`, `duration` |
| Each ordered shot | `id`, `asset_id`, `source_range: {start,end}` |

The validator rejects extension objects, unknown or duplicate fields and malformed shapes. IDs must match `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`. Asset IDs and shot IDs must be unique within their own collections. Several shots can reference one asset. A hash requires 64 lowercase hexadecimal characters. This check tests only spelling.

Time uses seconds in `{numerator: string, denominator: string}`. Both strings hold canonical nonnegative decimal `u64` values. The denominator must exceed zero and `gcd(n,d) = 1`. Use `0/1` for zero. The validator rejects `2/2` and `00/1`. Strings preserve large integer values during JavaScript transport.

A source range is half open, with `0 <= start < end <= duration`. Duration must be positive. The validator makes no conversion from presentation timestamps, frame rates or sample rates.

Input may contain at most 1,048,576 bytes. A valid document contains 1 to 1,024 assets and 1 to 4,096 shots. The default recursion limit in `serde_json` remains active. M0 excludes arbitrary file, URL, script and provider fields. It does not open media or establish timeline or media format support.

<a id="執行驗證"></a>
<a id="validate-cli"></a>

### Run the validator

```text
storytelling validate <FILE|->
storytelling --help
storytelling --version
```

Specify a local JSON file or `-` for standard input. A symbolic link can resolve to a regular file. This local read-only CLI provides no sandbox or server path policy. Standard input waits for EOF. Validation emits one JSON line without interleaved logs. Help and version return plain text.

A successful result looks like this.

```json
{"ok":true,"validation":{"schema_version":1,"plan_id":"...","assets":1,"shots":2,"scope":"declared-metadata-only"}}
```

A range outside the declared duration produces this error.

```json
{"ok":false,"error":{"code":"plan.source_out_of_bounds","path":"$.shots[0].source_range.end"}}
```

Errors omit source content and local paths. A failed write to standard output returns exit 3. The caller may receive incomplete JSON.

| Exit | Validator result |
| --- | --- |
| 0 | Declared document passed, or command printed help or version. |
| 1 | Invalid JSON, version, metadata, time, reference, range or size. |
| 2 | Invalid command or arguments. |
| 3 | Input or output failure. |

Use the stable codes below. Diagnostic paths, prose and line or column numbers can change.

- Input and usage: `input.invalid_json`, `input.too_large`, `input.io`, `cli.usage`.
- Version and format: `plan.unsupported_version`, `plan.invalid_id`, `plan.invalid_hash`.
- Time and range: `plan.invalid_time`, `plan.invalid_range`, `plan.source_out_of_bounds`.
- References and collections: `plan.duplicate_id`, `plan.unknown_asset`, `plan.empty_collection`, `plan.too_many_items`.

<a id="m3-02-sqlite-操作日誌"></a>

## M3-02 SQLite operation journal

The Rust storage adapter exposes an API. It provides no CLI command or host client. Before any future host call, the caller must persist the immutable operation identity and a numbered `dispatching` attempt. An unknown outcome blocks another attempt until the current owner records readback. Only an explicit `NotApplied` observation permits a later attempt.

The journal records the caller's observation and evidence digest. Host truth, signer authority, approval and artifact origin require separate evidence. See the [journal contract](migration/M3-02-OPERATION-JOURNAL.md).

<a id="m3-01p1-有限範圍的審閱綁定"></a>
<a id="m3-01p1-accepted-bounded-review-binding"></a>
<a id="m3-01-p1-accepted-bounded-review-binding"></a>

## M3-01/P1 review binding

The maintainer accepted the first bounded profile for [issue #27](https://github.com/bynanci/storytelling/issues/27) under the [signed G2 record](migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md). M3-RB01 through M3-RB07 apply only to this profile. The full milestone, PR #28 candidate and other M3 requirements have separate acceptance criteria.

| Command or API | Input | Behavior |
| --- | --- | --- |
| `prepare-review <COMPOSITION\|-> <CONTEXT_FILE>` / `prepare_review` | Snapshot up to 1 MiB. Strict context up to 65,536 bytes. | Imports and compiles the same snapshot bytes under the declared loss choices. Returns an untrusted portable record. |
| `check-review <REVIEW_RECORD> <COMPOSITION\|-> <CONTEXT_FILE>` / `check_review` | Record up to 98,304 bytes. Current snapshot up to 1 MiB. Context up to 65,536 bytes. | Checks the record and recompiles current input. Lists stale fields in order. A match establishes only consistency among declarations. |
| `BoundReviewV1` | Only `prepare_review` can construct it. | Private in-process result with the portable data transfer object (DTO) and exact compiled bytes. No `Serialize`, `Deserialize`, `Default` or public unchecked constructor. |

<a id="情境欄位與限制"></a>

### Context fields and limits

- Context v1 has exactly `schema_version`, `project_id`, `target_revision`, `host` and `loss_choices`.
- The `host` object has `id`, `version`, `profile_id`, `binary_sha256` and `declared_capabilities`.
- IDs use 1 to 128 ASCII bytes and must match `[A-Za-z0-9][A-Za-z0-9._-]*`.
- Hashes require 64 lowercase hexadecimal characters. Capabilities require at most 64 unique IDs in ASCII order.
- Loss choices form a sorted, unique subset of `metadata_only_locks` and `picture_only`.
- The parser rejects unknown, duplicate, missing, `null` or malformed fields.
- Review JSON allows up to 64 nesting levels. OTIO output allows up to 4 MiB.

<a id="身分摘要的範圍"></a>

### Identity digests

Identity v1 covers the following values.

1. The exact raw composition bytes in `composition_sha256`.
2. The serialized typed M0 plan, OTIO document and complete loss report. Each uses SHA-256 with a separate versioned domain.
3. The engine crate or compiler version and the selected OTIO profile.
4. The full context digest and separate digests for context fields. Each field has its own domain.

The final binding digest covers the schema version and full identity. Its domain is `storytelling.finite-otio-review-binding.v1`. A big-endian `u64` records the payload length.

A trailing newline changes the source digest even if the compiler produces the same artifact. The source digest covers the exact supplied bytes. Context arrays require canonical order. Other semantic arrays keep source or compiler order. Serialization uses the JSON encoding of these versioned Rust DTOs. It does not implement general JSON canonicalization (JCS).

<a id="驗證順序"></a>

### Order of checks

The API and CLI have separate tests for simultaneous invalid inputs.

- `prepare_review` checks the typed context before importing the snapshot.
- `check_review` applies byte limits to the record, snapshot and context in that order. It checks the record, parses and checks the context, then imports and compiles the snapshot.
- The CLI checks argument shape first. It reads each file and checks its size in argument order. Prepare reads `composition, context`. Check reads `record, composition, context`. A later file error can therefore precede an earlier JSON error. After the reads, the engine applies its API order.

<a id="審閱結果"></a>

### Review results

Treat every portable record as untrusted input. The `scope` value must equal `declared-review-binding-only`. The flags `executable`, `approval_granted`, `source_verified` and `host_verified` must all equal `false`. The API recomputes the binding digest before comparing the current inputs.

A valid stale record produces `ok:true`, exit 0 and `current:false`. Malformed records or false claims produce exit 1. The following identifiers can appear in a stale result.

- `composition`, `plan`, `artifact`, `report`, `compiler_version`, `profile_id`, `context`.
- `context.project_id`, `context.target_revision`, `context.host.id`, `context.host.version`.
- `context.host.profile_id`, `context.host.binary_sha256`, `context.host.declared_capabilities`, `context.loss_choices`.

<a id="驗證證據與限制"></a>

### Evidence and limits

A valid signature establishes that the supplied public key verifies the exact signed message. Trust and current signer authorization require a separately governed key registry and revocation policy. This profile provides neither and performs no signature check.

Artifact binding requires recompiling the same immutable input and comparing the exact output. This profile does that for one supplied snapshot and its OTIO export. It cannot establish a verified build, host snapshot, trusted producer or planner proposal.

The profile lacks replay protection, request consumption, authoritative time, expiry and revocation state. It does not establish filesystem identity or protect a later read from file replacement. It cannot observe a host, execute or publish. The host fingerprint and capabilities are declarations.

M3-02 persists bounded operation idempotency and prevents late journal writes. It cannot deduplicate or stop external host calls. Trusted grants and host execution remain broader M3 work. See the [command example](CLI_GUIDE.md#7-prepare-and-check-a-review-record) and [P1 acceptance record](migration/records/M3-01-P1.md).

<a id="pr-28-核准身分-v1-候選契約"></a>
<a id="merged-pr-28-approval-identity-v1-candidate-not-accepted"></a>
<a id="future-public-workflow-design-only"></a>

## PR 28 approval identity candidate

This separate cryptographic identity candidate merged in PR #28. Its G2 acceptance remains pending. See the separate [P1 profile](migration/M3-01-NEXT.md).

The [approval identity contract](migration/APPROVAL_IDENTITY_V1.md) exposes a core API. It supplies no CLI command or editor mutation. The API constructs a private `BoundApprovalIdentityV1` from exact proposal and artifact bytes plus existing `VerifiedBufferV1` source tokens. It binds these inputs. It cannot prove compilation or a common snapshot.

A signed payload must match every identity field in the token. The caller must supply a matching `ExpectedApproverKeyV1` from its own trusted source.

<a id="簽署資料與編碼"></a>

### Signed bytes and encoding

The signer receives `storytelling.approval.v1\0` followed by the frozen compact `ApprovalPayloadV1` JSON bytes. The pinned `serde_json` serializes fields in Rust declaration order. There are no maps or floating-point fields. Source IDs, host capabilities and accepted-loss rows must use strict sorted order without duplicates.

`SignedApprovalV1` rejects duplicate and unknown fields. Its JSON limit is 1 MiB. The detached signature uses 64 bytes and the public key uses 32 bytes. Both use lowercase hexadecimal encoding. Core only verifies signatures.

<a id="驗證結果能證明什麼"></a>

### What verification establishes

`VerifiedApprovalV1` has private fields and read-only accessors. It has no Serde traits. The token establishes that the supplied key signed the exact payload. It cannot establish registry trust, principal authorization or the truth of an `observed` host claim. It cannot establish absence of replay, revocation or expiry. It cannot establish host execution.

Trusted approval surfaces, the journal and the host executor have separate responsibilities. See the [PR #28 evidence record](migration/records/M3-01-approval-candidate.md).
