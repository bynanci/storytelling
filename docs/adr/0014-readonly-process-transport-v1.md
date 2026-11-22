<!-- SPDX-License-Identifier: MIT -->
# ADR 0014 · Read-only process transport V1

- **Status:** proposed G2 candidate; not accepted, implemented or released.
- **Task / requirement:** M4-02 / M4-R02, [Issue #108](https://github.com/bynanci/storytelling/issues/108).
- **Actual base:** `62541d91c0c52ec6e49b5dc9c8350e92e83b409a`, checked 2026-11-17 UTC. No open PR existed for #108 when work started.
- **Dependency:** accepted [ADR 0013](0013-read-only-consumer-facade-v1.md) / #107. #106 remains mandatory for any future execution/recovery; none is proposed here.
- **Direction selected:** requester chose “本機程序＋原始 bytes 框架；先支援 Node.js（建議）” on 2026-11-17. Maintainer/reviewer acceptance of the exact G2 contract remains pending.

## Problem and scope

An independent TypeScript caller needs the same eight read-only operations as
`storytelling_engine::consumer_v1`, without Rust-private imports or CLI prose
parsing. Selected proposal, bundle and OTIO bytes must survive transport exactly.
A projected narrative has no existing core JSON serializer. A stale review and
unresolved coverage must remain successful domain results, distinguishable from
source rejection, transport failure and current authority.

The G2 delivery is a reviewable protocol, owned TypeScript view/API specification,
paired wire examples and falsifiable G3/G4 cases. It creates no client, Rust
dispatcher, CLI command, feature, dependency or executable. #108's implementation
checklist remains unchecked.

## Alternatives and recommendation

| Option | Benefit | Cost / scope |
| --- | --- | --- |
| **Recommend: one local child process per call, framed raw buffers** | Existing pure Rust backends; original bytes avoid JSON/base64 round trips; cancellation/failure isolated to one request. | Node.js local client first; process startup and bounded copies; binary frame parser and portable process tests required. |
| WASM in process | A possible browser path without a child executable. | New target/dependency, allocation, worker/cancellation and packaging evidence; still needs explicit result views. Deferred. |
| Node native addon | In-process calls and fewer transport copies. | Node ABI/native packaging and three-platform artifacts; synchronous work needs isolation. Deferred. |

A JSON/base64 process envelope is another encoding of the first option. Framed
raw buffers avoid base64 expansion and a new Rust codec dependency. This trades
readability for explicit length/EOF checks. No HTTP service, persistent session,
JSON-RPC batch, shell wrapper or network discovery is proposed.

For G3, propose adding `serde.workspace = true` directly to the CLI for a strict
duplicate-aware header visitor. This uses the existing locked `=1.0.229` package,
not a new graph version. Its intentional CLI dependency-whitelist update in
`scripts/check_repo.py` belongs in the later implementation PR and G2 decision;
neither manifest nor whitelist changes here.

The requester asked for “#108唯讀 transport G2”. A follow-up preference question
offered the three alternatives. The requester selected local process/raw bytes
with Node first; the [decision record](../validation/m4-02-transport-g2.md)
preserves the actual reply and its limit: choosing a direction does not approve
the new contract or authorize implementation. The exact format remains proposed.

## Proposed boundary

[Protocol](../proposals/m4-02-readonly-transport/protocol.md) is the normative
candidate for framing, limits, error order, identity, lifecycle and compatibility.
[TypeScript views](../proposals/m4-02-readonly-transport/types.md) define owned
client types; [test plan](../proposals/m4-02-readonly-transport/test-plan.md) defines
the required RED/GREEN and regression evidence. Candidate schemas/examples live
under `docs/proposals/`, outside the accepted `contracts/` directory.

Propose a default-off `readonly-transport` CLI feature and a new explicit
`storytelling sdk-readonly-v1` entry after G2 adoption. It consumes one stdin
frame, delegates once to the selected facade operation, writes one stdout frame
and exits. There is also a transport-only `describe` request. These names are
proposals, not commands currently available on main. Existing CLI stdout,
trailing LF, arguments and exit semantics remain unchanged.

The bridge does not parse source JSON before delegation. The Node caller owns
explicit file reads; portable requests contain buffers, never local paths.
Narrative projection gets a binding-only `narrative_projection_v1` view, without
adding `Serialize` to core. Existing serializable results use their original
serializer; error details use `source_details_json()` directly.

`plan_story` returns the existing planned-result JSON plus a once-created
existing `{"ok":true,"planning":...}` response byte sequence. This sequence is
frozen and supplied unchanged to review calls. Review preparation exposes
compiler-produced record, artifact and bundle from the same opaque Rust value.
The portable TypeScript view is untrusted data, not a reconstructed Rust handle,
signature, human approval or capability.

## Responsibilities and exclusions

| Layer | Responsibility |
| --- | --- |
| Core | Unchanged domain DTOs, rational strings, private identity types and accepted schemas. |
| Engine | Unchanged eight-operation facade, validators, planner, compiler, digest and source diagnostics. |
| CLI bridge, future G3 | Framing, bounded I/O/encoding, binding-only views, exactly one facade call; no source pre-parser or alternate compiler. |
| Independent Node client, future G3 | Its own public types/strict response decoding, raw byte ownership, exact request association, process lifetime and structured failures. |
| Application | Explicit input reads and trusted executable configuration, visible author decisions, bounded aggregate resources; no execution inferred from this SDK. |

Excluded: browser/bindings beyond Node, distribution binaries, full Reference App,
MCP, media/ffprobe, Host credentials, editor writes, journal/retry/recovery,
authority/signature changes and G5. #103–#106/#109 keep their gates.

## Security, compatibility and rollback

Treat the configured local executable as trusted code, not a sandbox. Spawn an
absolute executable path with a fixed argv, `shell:false`, no inherited secret
environment, no request-derived cwd/path and no child-spawn behavior in the
bridge. An allowlisted Windows `SystemRoot` is local configuration, never
portable data. A replaced/malicious executable is outside content-validation
assurance; protocol/build strings do not authenticate it.

No transport header, request ID, framing byte or view version enters an accepted
digest. No trimming, newline conversion, re-encoding, float conversion, profile
upgrade or CLI-prose fallback is allowed. The transport has its own input/output
caps; these can reject a large otherwise valid domain result as a transport
resource failure. They are not new backend semantics or a proof of process memory
bounds.

Before G3, the maintainer must adopt the exact scope, framing/views/errors,
version/limit policy, one-shot lifecycle and validation plan. A sensitive runtime
or public-format change is not accepted merely because this Draft PR's CI passes.
Rollback of a future bridge/client removes an opt-in entry/package only; the
accepted Rust facade, existing CLI/documents/digests and user data need no
migration. Distribution and publication remain G5.
