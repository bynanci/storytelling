<!-- SPDX-License-Identifier: MIT -->
# N10-06C · Local media identity precheck candidate

English · [繁體中文](README.zh-TW.md) · [Issue #129](https://github.com/bynanci/storytelling/issues/129)

**State: proposed G2 contract and threat model; adapter implementation, OS/tool qualification and real-file access are blocked pending separate maintainer/privacy approval.** This is an independently scoped C design. Base at preparation: `52dc6c315f7b3d20693f3c51e052509997ab791b`. The [candidate sidecar schema](sidecar-v0.schema.json), [synthetic unknown example](synthetic-unknown.example.json) and [planned acceptance cases](cases-v0.json) are review artifacts, not an accepted public format or observations.

## Result and existing boundary

An operator needs to know whether a declared `asset_id@revision` still refers to the exact selected local file and whether its selected source range fits the observable video time domain. A hash match, metadata observation, author content judgment, host capability and execution approval answer different questions. Report each independently.

The accepted [coverage contract](../../adr/0010-narrative-video-coverage-v1.md) validates declarations only. [N10-05](../../validation/n10-05-execution-record.md) recorded readable metadata from a supplied synthetic MP4, without binding its file to the JSON asset ID. [PR #130](https://github.com/bynanci/storytelling/pull/130) is A's exact input helper; B's advisory UX remains separate. C changes no V1 field, diagnostic or command. No media access belongs in core or engine.

Existing `verify_supplied_sha256` checks caller-supplied owned bytes with a 16 MiB limit; it does not open a path, identify an asset, grant access or prove a mutable file was stable. The existing exact-time type can validate reduced nonnegative u64 rational strings. The [FilmCraft oracle](../../../tests/oracles/filmcraft-25-v1/README.md) supplies a pattern of explicit pinned tool identities and isolated runs, but its host proof is not a production file adapter or this C contract. No source implementation or third-party test was copied.

## Candidate data ownership and contract

The candidate is a **private local sidecar**, outside portable V1, never an access credential. Its exact version is `n10-06c.local-sidecar.0-candidate`. Keep it and its evidence store outside the public repository. A future parser must reject unknown versions, unknown fields, duplicate keys and malformed values before resolving any file reference. The schema checks shape only; the relational rules below require code and tests after G2. Candidate sidecar input is limited to 262,144 UTF-8 bytes and 16 nested containers, with strict duplicate-key preflight, at most 32 diagnostics, 128-byte opaque IDs, canonical u64 rational strings and i128-bounded signed ticks. The future adapter returns an in-memory private report; persistence needs an explicit separate caller-owned evidence destination and is not an implicit source-media write. Reuse fails closed if private evidence refs are unavailable.

| Field group | Candidate meaning |
| --- | --- |
| `coverage_ref` | Exact supplied coverage-byte SHA-256, catalog ID/revision, asset ID/revision, declared duration and selected half-open source range. Changing any of them invalidates reuse. It does not update the catalog or assign a new revision automatically. |
| `selection_ref` | Opaque `grant_id`, `root_scope_ref` and `selected_file_ref` issued by a trusted local chooser for an explicitly selected regular file. No path, URL or caller-selected command is accepted in this JSON. These IDs are report references, not reusable handles or permission tokens. |
| `file_access` | Whether that selected file object was opened read-only and readable during this attempt. Existence/readability is independent from hash agreement. |
| `stability` | Identity references before/after and the qualified read-view/lease profile. An equal stat tuple or equal hashes alone does not prove immutability during the probe. |
| `byte_identity` | Complete-file SHA-256 before and after, byte length and the **separately supplied** expected digest/evidence for this exact asset revision. A first observation without an authorized expected digest stays `not_verified`; do not copy an observed hash into expected and call that a match. |
| `metadata` | Tool/probe evidence refs, explicit selected video stream, exact time-base/tick observations, timing mode and boundary basis. Metadata alone does not certify decoding, playback, synchronization or story content. |
| `duration_check` / `range_check` | Separate compatibility results based on qualified exact observations. Never clamp the declared duration or range. |
| `content_review` | A separate human review reference; metadata and SHA-256 cannot set it to confirmed. An imported sidecar cannot authenticate the person. |
| `binding_state` | `not_verified`, `bound_for_read_view`, `unknown` or `rejected`. There is no global `media_verified` or export-ready state. |

For a future **local observation**, `bound_for_read_view` requires all of: a live matching selection grant; an accepted stable-read profile; the same selected object across the attempt; complete hashes equal to the expected asset-revision digest; compatible qualified duration/range observations; and immutable evidence refs. It describes only that read view and attempt. `content_review`, host and authority remain independent. A serialized `bound_for_read_view` value is untrusted data when loaded later and cannot revive a grant or authorize a read.

Normative cross-field rules for a future validator: every completed check has a non-null private evidence ref; `matches` requires non-null expected/ref, both complete hashes and byte length, with all three digests equal. A stable view requires non-null accepted profile/read-view/identity evidence. A local observation has a real UTC observation time; synthetic examples keep it null. `unknown`/`rejected` retains a fixed diagnostic. An imported value claiming any of these states remains untrusted until its protected evidence is checked; JSON Schema cannot establish these equalities or qualifications. Unsafe scope/process failures stop further reads and leave later checks not_verified; already obtained observations are retained without upgrading the binding.

If the asset ID/revision stays the same but bytes differ, retain `mismatch`, reject reuse and preserve the earlier evidence. A new file selection, source revision, declared range, tool/profile version or expected-evidence revision creates a new binding attempt. Rechecking never overwrites a newer attempt or silently erases a failed one.

## Trusted selection and stable read procedure

All steps below are **planned**, not implemented or qualified by this PR.

1. Obtain a narrow operator selection through a trusted local UI/CLI chooser. The adapter receives an owned read-only file capability plus a root-scope capability; it does not resolve arbitrary JSON strings. A filename, extension or matching asset label never selects a file.
2. Open only the explicitly selected local regular-file object, validate all path components through OS handles, and reject symlinks/reparse points, parent traversal, network/UNC shares, device/FIFO/socket inputs, cross-mount/directory escapes and unsupported aliases. String `canonicalize` followed by a later path open is insufficient. The initial profile rejects multi-link files unless alias behavior is separately qualified.
3. Establish a qualified stable read view using an OS-enforced immutable snapshot or an effective deny-write/delete lease, with no media write or copy performed by this adapter. Advisory `flock`, a read-only descriptor, a read-only file flag or a before/after stat check alone does not supply this guarantee. Pre-existing writable handles/mappings and rename/delete races must be covered by OS-profile tests. If the platform cannot provide and prove the required view, continue only as an observation with `stability: unknown` and `binding_state: unknown`, or stop.
4. Record safe local file-identity references and size, then hash the **complete same read view** with a bounded streaming hash. Do not relax the existing 16 MiB in-memory API merely to fit larger files; any streaming adapter is a separate local I/O implementation and must not expose a forgeable `VerifiedBufferV1` constructor.
5. Give the controlled probe the inherited selected descriptor/read view. Do not reopen the original pathname. Reject a tool/OS combination that cannot consume the selected object under the declared sandbox. A pathname-only fallback loses the binding and is `unknown`.
6. After the probe, hash the complete view again and recheck identity/size. Any replacement, truncation, write, lost lease, short read or discrepancy invalidates the result. Equal before/after hashes are additional observations; they cannot detect all change-and-restore (ABA) attacks without the stable read-view guarantee.
7. Compare expected fingerprint, declared duration and selected range using exact integers/rationals, preserve each result and release capabilities. Unknown/failed attempts remain recorded. A retry requires a fresh explicit operation/selection decision and a new attempt ID; it does not turn an unknown result into success.

No OS stable-read profile is qualified by this design. In particular, ordinary mutable files on Linux, Windows or macOS must not be advertised as bound merely because both hashes match.

## Controlled `ffprobe` boundary

A future adapter must pin an absolute trusted executable identity/version/hash and invoke an argument array through the process API (`shell=false` / Rust `Command::arg`), never shell interpolation, `sh -c`, PATH lookup or user-supplied flags. Clear loader/plugin/proxy variables; use a fixed locale and minimal environment. No profile may mutate HOME or alter repository/system settings.

Proposed transport profile: one inherited selected descriptor as stdin, `pipe:0` input, an allowlist containing only the required descriptor/pipe protocol, an explicit demuxer allowlist, no network capability, and a filesystem sandbox denying media opens other than the inherited capability. Protocol/format flags alone are not a sandbox. File-relative external references, playlists, URL protocols and parser-triggered side reads must fail. Tool/library access needs an explicit separate sandbox allowlist. If a container needs seeking or another access mode that this profile cannot safely supply, return `unsupported`/`unknown`; do not reopen its path or download missing material.

The proposed metadata argument array is:

```text
[pinned_ffprobe,
 "-v", "error", "-protocol_whitelist", "pipe",
 "-format_whitelist", "mov,matroska,webm",
 "-show_entries", "format=format_name,start_time,duration:stream=index,codec_type,codec_name,time_base,start_pts,duration_ts,nb_frames,avg_frame_rate,r_frame_rate,width,height:stream_disposition=attached_pic,default",
 "-of", "json", "-i", "pipe:0"]
```

A separately bounded complete-timestamp pass would use the same prefix, an
explicit decimal stream index validated against the first result,
`"-select_streams", selected_index, "-show_packets", "-show_entries",
"packet=stream_index,pts,duration", "-of", "json", "-i", "pipe:0"`.
The adapter resets and streams the same qualified read view to each process;
it permits no temporary media-file copy. Stdin/pipe transport supplies no
arbitrary seek or pathname fallback. Containers that cannot be completely
examined this way remain unsupported/unknown. The process API receives these
as separate arguments, not a constructed command string.

Exact executable builds, field names, demuxers, descriptor behavior and supported OSes must be validated and frozen at C's G2; this list is not a supported-host claim. A future profile's field allowlist excludes filenames, tags, GPS, chapters and attachments. Retain raw stderr/stdout only in the private evidence store; public diagnostics contain fixed codes and logical JSONPaths and never echo a pathname, untrusted key or parser text.

Candidate per-attempt budgets: at most one selected file, 268,435,456 bytes (256 MiB), two complete SHA-256 passes, 30 seconds total wall time, 16 MiB total probe stdout, 64 KiB stderr, 16 streams, two probe processes and 100,000 packet/timestamp records. Cap adapter media reads/pipe delivery at 1 GiB total across the two hash passes and two probe passes, with each pass at most the selected file byte length. Enforce byte/stream/record caps while reading, not after unbounded capture. Terminate the supervised process tree and confirm quiescence on budget expiry; lack of confirmation leaves the attempt unknown. No automatic retry or partial-output success. All limits are proposed and need G2 review; real usability or performance at these bounds is unmeasured.

## Exact duration, stream and source-range decisions

- Require an explicit stream index from the operator when more than one eligible video stream exists. Attached pictures and unsupported stream types do not silently become the chosen footage. An absent/out-of-range/ambiguous selector is unknown or rejected.
- Preserve reported `time_base`, i128-bounded signed `start_pts` and end boundary ticks as strings. Subtract the profile-defined presentation origin with checked signed arithmetic, then form nonnegative seconds using checked u128 arithmetic/GCD and final u64 rational bounds; unsupported negative/overflow/discontinuous/gapped timelines remain unknown. Never parse reported decimal duration or average fps through floating point.
- A header `duration`, `duration_ts`, `nb_frames`, `r_frame_rate` or `avg_frame_rate` is a declared/estimated observation, not proof of the complete observable interval. A `compatible` duration/range result needs a qualified complete-boundary basis, such as a bounded complete selected-stream packet/timestamp examination whose interpretation has profile tests. A partial scan, missing PTS/durations, truncated file, parser disagreement or unqualified codec yields unknown. Packet timestamps still do not prove full decode/playback or audiovisual synchronization.
- Proposed strict comparison: duration is `compatible` only when a qualified complete normalized interval starts at 0 and its exclusive end equals the declared duration exactly. With qualified contradictory bounds it is `incompatible`; without a complete qualified basis it is `unknown`. There is no implicit tolerance or rounding. Range is checked separately: the selected half-open `[start,end)` must lie in the qualified continuous observed interval and satisfy unchanged V1 validation. Gaps/retiming not interpreted by the profile remain unknown. A shorter declared duration may contain a safe range while still failing the independent duration-equality check. Preserve origin/precision/basis and each contradiction; never change catalog declarations.
- VFR frame indexing is unsupported without a separately accepted per-frame PTS design. Seconds ranges may be assessed only against a qualified observed interval. Nominal or average fps never supplies constant cadence. Multiple streams, edit lists and retiming remain unsupported until a profile states and tests their interpretation.
- The author must separately confirm that the selected pictures depict the narrative unit. A matching hash can identify the wrong clip perfectly.

## Threat and failure review

| Threat / first failure | Required response |
| --- | --- |
| Caller invents an asset/file handle ref or replays an old sidecar | Resolve only live trusted capabilities in the current selection context; reject unbound refs and require fresh checks. |
| Symlink/reparse/`..`/mount/hardlink escape changes the file opened | Handle-based scope checks; reject unsupported path/alias semantics before probe. |
| File replaced by rename after hash | Pass the held object to probe and compare identity; reopening a pathname cannot establish the binding. |
| Writer changes bytes then restores them while both hashes match | Equal hashes are insufficient; absent qualified stability guarantee makes binding unknown. |
| `ffprobe` opens a relative file, protocol URL or credential-bearing endpoint | Descriptor-only input plus network/filesystem denial; reject the profile when containment is not enforceable. |
| Malicious media/tool emits huge, slow or private metadata | Supervised budgets, bounded capture, fixed field allowlist, private raw evidence and safe public codes. |
| Multi-stream/VFR/header-duration assumption selects the wrong time domain | Explicit stream selection and qualified exact boundary evidence; otherwise unknown. |
| Digest match or manual statement is treated as execution approval | Keep content/host/authority states independent; this sidecar enables no execution. |

Proposed safe diagnostics are separate from V1: `media_identity.input_too_large`, `unsupported_version`, `invalid_sidecar`, `selection_required`, `scope_rejected`, `unreadable`, `unsupported_profile`, `stability_unproven`, `file_changed`, `fingerprint_mismatch`, `probe_failed`, `budget_exceeded`, `stream_ambiguous`, `timing_unknown`, `duration_incompatible`, `range_incompatible`. Every listed suffix uses the `media_identity.` prefix. Proposed parse-error response is `{"error":{"code":"media_identity.invalid_sidecar","path":null}}` without echoing parser text. Raw byte/depth/duplicate/syntax checks precede version and closed-shape checks, which precede any capability lookup. A missing selection or rejected scope stops before probe; later independent check failures retain their fixed/indexed logical paths and leave unperformed checks not_verified. The complete candidate sidecar records obtained observations; it grants no write permit. These choices and the final diagnostic order require C G2 acceptance.

## Acceptance, evidence and stop conditions

The [planned cases](cases-v0.json) include both a qualified immutable-view positive and mutable/ambiguous negatives. They are specifications. Before runtime implementation, the maintainer must accept the sidecar fields/statuses, selection/privacy scope, qualified OS read-view and sandbox profile, pinned tool identity/argv/limits, exact timing interpretation and diagnostics. Tests must first observe RED for handle substitution, ABA/lease loss, bounds and unknown propagation, then GREEN. Supported OS/tool combinations need their own direct evidence; CI cannot invent parity.

No probe, real-file hash, author operation, host readback, G2 acceptance or release was performed for this design. The schema/example may be mechanically checked without media; that verifies design shape only. Author completion/time/control and actual image meaning require separately recorded observations. #117 was merged as proposal documents only at `a23a2909cf3734d207c66b372f29a85c5e541c86`; #103 technical G2/W01 and #104–#106 retain their execution/host gates; #107–#109 own later public consumer integration.

Stop or narrow C if reliable read-view binding or containment cannot be established, if the task needs unsupported VFR/stream/codec semantics, if budgets make the observation incomplete, or if a small declaration-only helper already serves the author. Keep unknown/not_verified visible. Rollback removes the optional adapter/sidecar reader; accepted V1 inputs and A remain usable, and no media/catalog/project mutation needs undoing.
