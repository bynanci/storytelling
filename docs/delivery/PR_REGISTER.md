<!-- SPDX-License-Identifier: MIT -->
# Pull request delivery register

<a name="pr-交付登錄"></a>

[繁體中文](PR_REGISTER.zh-TW.md)

Use this register to find the reviewed commit, the merge commit, and the checks for each delivery.
Each dated section describes the state at that time. Later sections record subsequent decisions.
The [machine-readable register](pr-register.json) contains the full commit and tree IDs, UTC times, CI runs and linked task issues.

The first observation was at `2026-11-07T05:41:45Z` on main [`867fcde`](https://github.com/bynanci/storytelling/commit/867fcde6acd0dd1339122aaf4bb23003aac3942e).
The second REST check for dependency PRs was at `2026-11-07T05:45:37Z`.

<a name="已合併的實作交付"></a>

## Implementation in the first snapshot

| PR and scope | Accepted head | Merge commit | Merged at (UTC) | Checks on the accepted head |
| --- | --- | --- | --- | --- |
| [#1](https://github.com/bynanci/storytelling/pull/1) M0 declaration validation | [`4c5682c`](https://github.com/bynanci/storytelling/commit/4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b) | [`2227981`](https://github.com/bynanci/storytelling/commit/22279814ff6b147c386186569d7f77f3a803cf8b) | 2026-11-07 02:17:16 | 10 passed |
| [#7](https://github.com/bynanci/storytelling/pull/7) M1-01 snapshot importer for the finite profile | [`c2fb478`](https://github.com/bynanci/storytelling/commit/c2fb478c278550b682efba5f898082e8bcd20e24) | [`610ed49`](https://github.com/bynanci/storytelling/commit/610ed49673d6f554e539008b34f5fc5479b9dbc1) | 2026-11-07 02:18:09 | 10 passed |
| [#16](https://github.com/bynanci/storytelling/pull/16) M1-02 OTIO with exact time values | [`fb65753`](https://github.com/bynanci/storytelling/commit/fb65753df617c7df80ef2581396eead6b07a6434) | [`9a5232e`](https://github.com/bynanci/storytelling/commit/9a5232e97435f543238b90a2006460f319f4e651) | 2026-11-07 03:09:33 | 10 passed |
| [#17](https://github.com/bynanci/storytelling/pull/17) M2-01 Brief constraint audit | [`1c8c01d`](https://github.com/bynanci/storytelling/commit/1c8c01da48af75b860b1ce49241c3fd509b16c59) | [`422190e`](https://github.com/bynanci/storytelling/commit/422190e27bc604bd71485f78279373992f99e68c) | 2026-11-07 03:20:26 | 10 passed |
| [#18](https://github.com/bynanci/storytelling/pull/18) M1-03 supplied buffer and host evidence | [`2b859d2`](https://github.com/bynanci/storytelling/commit/2b859d29e22f6f2a39f73822086b59618bb8a402) | [`e0a5ade`](https://github.com/bynanci/storytelling/commit/e0a5ade7ee2c0414f87aa121bef1bdf259a7e75e) | 2026-11-07 03:26:40 | 10 passed |
| [#22](https://github.com/bynanci/storytelling/pull/22) M2-02 fixed-window planner | [`1f4ff08`](https://github.com/bynanci/storytelling/commit/1f4ff082ed27dbc6223af2f0a0b1cfe71a06fe54) | [`895257b`](https://github.com/bynanci/storytelling/commit/895257b1bb9993d70cae3f1267f5f6fa3de91905) | 2026-11-07 03:59:29 | 10 passed |

The tree at each of the six accepted heads matches its merge tree.
M0 and M1-01 check or map declarations. M1-02 compiles a limited format. M2-01 checks hard constraints. M2-02 plans fixed windows.
M1-03 checks the SHA-256 of supplied bytes. The host evidence covers one synthetic FilmCraft case on Linux at 25 fps, using a CPU and a 192 × 108 preview.
It leaves audio, GPU and GUI execution, full-resolution output, and other hosts unverified.

<a name="已合併的規劃與狀態文件"></a>

## Planning and status documents in the first snapshot

| PR and document scope | Accepted head | Merge commit | Merged at (UTC) | Checks on the accepted head |
| --- | --- | --- | --- | --- |
| [#2](https://github.com/bynanci/storytelling/pull/2) Planning index for 48 OF entries and 16 gaps | [`95291d3`](https://github.com/bynanci/storytelling/commit/95291d31c47601181070bf89a28b1a182ddf6ed8) | [`9fb403b`](https://github.com/bynanci/storytelling/commit/9fb403be811643baf948acdb6d72232d4a6207bb) | 2026-11-07 02:17:48 | 15 passed |
| [#11](https://github.com/bynanci/storytelling/pull/11) M0 and M1-01 acceptance records | [`dbf2fd4`](https://github.com/bynanci/storytelling/commit/dbf2fd45ef6a215e59cabbe9a6278bc09ca53daa) | [`440209f`](https://github.com/bynanci/storytelling/commit/440209f21d2b5fd6e7b4cad0823359c802cf8c8e) | 2026-11-07 02:30:58 | 10 passed |
| [#19](https://github.com/bynanci/storytelling/pull/19) OTIO, Brief and host status | [`8997314`](https://github.com/bynanci/storytelling/commit/89973143d9bcd6c1c99e40f37f6fa4f13972dc59) | [`c03d4c8`](https://github.com/bynanci/storytelling/commit/c03d4c8ede33563451b8ad590a6df982d0d8b82a) | 2026-11-07 03:35:43 | 10 passed |
| [#23](https://github.com/bynanci/storytelling/pull/23) Fixed-window planner acceptance record | [`88a188b`](https://github.com/bynanci/storytelling/commit/88a188b5d3aa54e6ac76f7f6340f15b937248693) | [`867fcde`](https://github.com/bynanci/storytelling/commit/867fcde6acd0dd1339122aaf4bb23003aac3942e) | 2026-11-07 04:06:47 | 10 passed |

PR #2 had 15 successful checks across three runs. Each other listed delivery had 10.
The [machine-readable register](pr-register.json) records the run URLs, job names and results. These documentation changes add no runtime behavior.

<a name="已關閉但未合併"></a>

## Proposal closed without a merge

[PR #13](https://github.com/bynanci/storytelling/pull/13) had five successful checks on head [`6268766`](https://github.com/bynanci/storytelling/commit/62687668ea7c36d8ffd555bb33d4c909a461368b).
Its tree was `bc9be7dc22d8c2cb4610986cd8b7adb2a453e361`.
The draft closed without a merge commit. PRs #11, #16 and #17 superseded it. PR #17 checked and adopted the relevant source correction.

<a name="待處理的依賴與-actions-更新"></a>

## Dependency and Actions updates in the first snapshot

| PR | Change | Observed head | First parent of head | Checks on head | REST mergeable / state |
| --- | --- | --- | --- | --- | --- |
| [#8](https://github.com/bynanci/storytelling/pull/8) | checkout 4.4.0 → 7.0.1, Node 20 → Node 24 | [`f3bf985`](https://github.com/bynanci/storytelling/commit/f3bf985acbd6abb67b3ce5f781535387dc602373) | [`2227981`](https://github.com/bynanci/storytelling/commit/22279814ff6b147c386186569d7f77f3a803cf8b) | 5 passed | true / clean |
| [#9](https://github.com/bynanci/storytelling/pull/9) | serde_json 1.0.145 → 1.0.154, ryu → zmij 1.0.23 | [`cbb8e7d`](https://github.com/bynanci/storytelling/commit/cbb8e7dd2f46de274bcde2de887f706b057b360d) | [`e0a5ade`](https://github.com/bynanci/storytelling/commit/e0a5ade7ee2c0414f87aa121bef1bdf259a7e75e) | 5 passed | true / clean |
| [#10](https://github.com/bynanci/storytelling/pull/10) | serde 1.0.228 → 1.0.229, syn 2 → syn 3.0.9 | [`6a2456a`](https://github.com/bynanci/storytelling/commit/6a2456a2008d937b1658bbb77866bee4f2b694b0) | [`e0a5ade`](https://github.com/bynanci/storytelling/commit/e0a5ade7ee2c0414f87aa121bef1bdf259a7e75e) | 5 passed | true / clean |
| [#21](https://github.com/bynanci/storytelling/pull/21) | setup-python 6.3.0 → 7.1.0, Node 24 | [`ad4e33f`](https://github.com/bynanci/storytelling/commit/ad4e33f0406a78336279fe7916d3372660169d02) | [`c03d4c8`](https://github.com/bynanci/storytelling/commit/c03d4c8ede33563451b8ad590a6df982d0d8b82a) | 5 passed | true / clean |

An earlier normalized read reported a false mergeability value for PRs #8, #9 and #10. The later REST read reported `true / clean` for all four PRs.
Each set of five checks applied to its recorded head. Integration with the latest main still required a new check.

At this observation, the repository guard checked the dependency set and the reviewed SHA2 configuration. It did not pin serde or serde_json versions separately.
PRs #9 and #10 required review of the new number formatter and the proc-macro and syn changes. PRs #8 and #21 required review of the Action runtime and distribution.
Reviewers still needed to check source identity, Rust 1.85 compatibility, permissions, licenses and the lockfile or Action distribution.
The later entry dated 13:50 UTC records the merges and the checks after integration.

<a name="任務與-pr-body-整理"></a>

## Issues and PR descriptions

All eight task issues were complete at this observation. Each issue had all its checklist items marked complete.

[#3](https://github.com/bynanci/storytelling/issues/3), [#4](https://github.com/bynanci/storytelling/issues/4), [#5](https://github.com/bynanci/storytelling/issues/5), [#6](https://github.com/bynanci/storytelling/issues/6), [#12](https://github.com/bynanci/storytelling/issues/12), [#14](https://github.com/bynanci/storytelling/issues/14), [#15](https://github.com/bynanci/storytelling/issues/15), [#20](https://github.com/bynanci/storytelling/issues/20).

PRs #1, #2 and #7 received a dated note identifying the snapshot of main.
The note replaced outdated statements about M1-02, M1-03 and the role of PR #13. The original descriptions remained available below the note.
A second update showed that the process did not add the note twice.
The marker was `<!-- storytelling-pr-lifecycle-register-v1 -->`. The update affected only the PR descriptions.

<a name="如何延續"></a>

## Add a delivery

1. Read the PR state, accepted head, merge tree and merge time on GitHub.
2. Check the CI results on the same head.
3. Append the observation to the machine-readable register. Keep the original evidence for each accepted change.
4. Leave an unverified result pending. Record a closed proposal with no merge commit as closed without a merge.

Use the [task list](../../TASKS.md), [requirements](../REQUIREMENTS.md), task comparison records and GitHub metadata to check delivery.
The 48 OF entries and 16 gaps define inventory coverage. The solver, production execution, approval, editorial acceptance, release and administrative permissions each require their own evidence.

<a name="後續接受交付2026-11-07-0627-utc"></a>

## Additional accepted deliveries on 2026-11-07 at 06:27 UTC

This observation adds two merges to the original snapshot of 15 PRs. The observed main was [`5533fcb`](https://github.com/bynanci/storytelling/commit/5533fcb5d0f857ad1066a76505071653b66d1f89).
To find later changes, see the [merged pull requests](https://github.com/bynanci/storytelling/pulls?q=is%3Apr+is%3Amerged).

| PR and scope | Accepted head | Merge commit | Merged at (UTC) | Checks on head |
| --- | --- | --- | --- | --- |
| [#25](https://github.com/bynanci/storytelling/pull/25) Register for accepted, superseded and pending PRs | `0030fe206f62c7c058e2337e98945d0e72a06954` | `073b5a415324d588019e2155dcf24ddddd860adf` | 2026-11-07 05:56:33 | 10 passed |
| [#26](https://github.com/bynanci/storytelling/pull/26) M2-03 paired runtime comparison for selected cases | `4bf888949107a21b560b09222175ff1f25118a3b` | `5533fcb5d0f857ad1066a76505071653b66d1f89` | 2026-11-07 06:27:14 | 10 passed |

Each accepted head had the same tree as its merge. PR #25 delivered documentation.
PR #26 added a test harness and Linux CI for the selected domains. Production Rust code, Cargo, the existing fixtures and host receipt stayed unchanged.
Each of the two CI runs passed 40 shadow tests. The 64 comparisons produced 56 matches, four specified policy differences, four unsupported probes and zero unclassified results.
An independent check compared exact fractions and checked 336 artifact hashes. See the [M2-03 acceptance record](../migration/records/M2-03.md#accepted-delivery-and-clean-candidate-proof).

[Issue #24](https://github.com/bynanci/storytelling/issues/24) closed as completed at 06:27:33 UTC. All checklist items were complete.
Dependency PRs #8, #9, #10 and #21 still awaited review at that time. M2-03 acceptance covered the selected synthetic domains in the frozen profile.
The full solver, production host execution and editorial acceptance remained open.

<a name="後續觀察到的待審實作"></a>

## Draft implementation observed on 2026-11-07

[PR #28](https://github.com/bynanci/storytelling/pull/28) proposed a limited approval identity and new Ed25519 dependencies.
The draft head was `c162a47dcb8de4c582a3c66c22396ecfd8cb0134`. Its base was `5533fcb5d0f857ad1066a76505071653b66d1f89`. Ten checks passed on the head.
An independent review found no code blocker for the specified signature check. It checked features, minimum Rust versions and licenses for 39 packages, including 15 new versions.
The dependency documentation required a CI audit on that head. The workflow still lacked a `cargo audit` step.
The audit, source comparison and first-party boundary needed a decision before acceptance.

[Issue #27](https://github.com/bynanci/storytelling/issues/27) opened for review binding and freshness.
The [plan](../migration/M3-01-NEXT.md) reserves the M3-RB IDs for this work. PR #28 uses M3-A IDs.
Issue #27 stayed open with no completed items at this observation.

## Current lifecycle reconciliation: 2026-11-07 13:50 UTC

By this observation,
PRs [#8](https://github.com/bynanci/storytelling/pull/8),
[#21](https://github.com/bynanci/storytelling/pull/21),
[#10](https://github.com/bynanci/storytelling/pull/10) and
[#9](https://github.com/bynanci/storytelling/pull/9) merged their action and
Serde dependency updates. Their PR-head and post-merge main checks passed.

[PR #28](https://github.com/bynanci/storytelling/pull/28) merged head
`2d23b21dcfbfc49c3d76087f054cc76724c23053` at
`5a5702ba4c1392d763929f89da3cf7d16d5f13f4` on 2026-11-07 13:47:15 UTC. Both
same-head CI runs and the post-merge main run passed Quality, Required, Linux,
Windows, macOS and Dependency Audit. The audit loaded 1,422 advisories and
scanned the 40-package lockfile without findings. The merge integrated a candidate with a limited scope. Independent G2 review and product acceptance were still pending.
Issue [#27](https://github.com/bynanci/storytelling/issues/27) remains open for
the separate finite-OTIO review identity and declared-context freshness scope.
At this time, GitHub showed zero open pull requests. The machine-readable register contains both observations.

## Current open M3-01 review-binding candidates — 2026-11-07 23:42 UTC

At this observation, both drafts awaited a G2 decision. Issue #27 remained open.

| PR | State / base | Exact head | Same-head checks | Acceptance |
| --- | --- | --- | --- | --- |
| [#31](https://github.com/bynanci/storytelling/pull/31) | Draft on main `9513913e849f8cf15d9e5bbb2977fe587ce8b953` | `67abf6d443abf11a0cbced519eeac2a86396a7f7` | All 6 passed in [run 42381290915](https://github.com/bynanci/storytelling/actions/runs/42381290915) | G2 contract review pending (documents only) |
| [#32](https://github.com/bynanci/storytelling/pull/32) | Draft based on #31 head `67abf6d443abf11a0cbced519eeac2a86396a7f7` | `a979c6ad6244a9d20a8c276f43e21d310d8d8fbd` | All 6 passed in [run 42426266361](https://github.com/bynanci/storytelling/actions/runs/42426266361): Quality, Required, Linux, Windows, macOS and Dependency Audit | Provisional implementation. Independent G2 code and security review and a decision on the source comparison were pending. |

PR #32 bound the finite importer and OTIO output to the same input. Its private in-process result does not support deserialization.
It added strict checks on the declared context, a read-only freshness command and a static comparison with the source.
Signature checks, signer authorization, host or binary observation, source provenance, replay protection and planner-to-OTIO conversion were outside that scope.
The comparison used source text. It did not execute the TypeScript runtime.

Issue [#27](https://github.com/bynanci/storytelling/issues/27) had six open checklist items. GitHub showed no review request. Maintainer and G2 acceptance remained pending.
PR #32 could merge, but it remained a draft based on PR #31. This base included the review packet and code from merged PR #28.
The plan required an independent G2 contract decision before a rebase or merge.


## Conditional merge review — 2026-11-08

At 01:33 UTC, the remaining open feature/design PRs are the following Drafts.
Each still required an independent G2 decision.

| PR | Observed head | Delivery decision |
| --- | --- | --- |
| [#31](https://github.com/bynanci/storytelling/pull/31) | 67abf6d443abf11a0cbced519eeac2a86396a7f7 | Keep as draft. No independent G2 review |
| [#32](https://github.com/bynanci/storytelling/pull/32) | 1a281aa93a73026d24f9305ea46a6fe91e204299 | Keep as draft. No G2 review. Static source comparison awaits a decision |

PR #32's six jobs passed in run 42427010497 and no review thread exists.
Issue #27 remained open. [Issue #33](https://github.com/bynanci/storytelling/issues/33) records the scheduling change for the accepted M2-02 contract. The issue and its PR contain the CI and merge records.


## Accepted ordering optimisation — 2026-11-08 reconciliation

PR [#34](https://github.com/bynanci/storytelling/pull/34) merged accepted head
`5bbc3a0e867a90e3f8829c3cf7ab6b2c3961b3b0` as
`f07ab6940484ef8b8e4f2bdb37b96893e9f92270` at 01:37:08 UTC.
Head runs 42434851833/42434822453 passed all 12 jobs, and main run 42435022548
passed all six. The trees match. Issue #33 closed as completed with all items checked.
The change preserved the accepted fixed-window profile. The author reviewed it under the user's merge authority. Release and M3 G2 approval remained separate decisions.

At 04:21 UTC, PRs #31 and #32 remained drafts with no review submissions. [Issue #35](https://github.com/bynanci/storytelling/issues/35) and its PR record the cached-clock change, checks and merge.

## Accepted cached-clock optimisation — 2026-11-08

PR [#36](https://github.com/bynanci/storytelling/pull/36) merged head
`bdd502a8f8cd36c7b822c91fba4ff87215e8beb1` as
`00ef8decbf1fecc7258bbc683973d41d656c8097` at 04:28:04 UTC. Head runs
42446786881/42446759799 passed all 12 jobs. Main run 42446933790 passed all six.
The accepted head and merge have the same tree. Issue #35 closed as completed with all items checked.
The author reviewed the change under the user's authority. The change preserved the fixed-window contract for time and errors.
PRs #31 and #32 remained drafts pending independent G2 review. The project did not publish a release.


## M3 contract/candidate integration — 2026-11-08 05:12 UTC

The user reported review completion and asked the maintainer to merge the remaining PRs. Both merged after the maintainer resolved conflicts with main and checked the new heads. The merges kept the original M3 Rust code and test blobs.

| PR | Refreshed head | Merge | Head runs / main run |
| --- | --- | --- | --- |
| [#31](https://github.com/bynanci/storytelling/pull/31) | 937cf5a1db61d1bccb401241c8d3001bc02e49f5 | baa97903e3d3c02e2eb2747140d62f1ddca3c60c | 42449119799, 42449115934 / 42449339199 |
| [#32](https://github.com/bynanci/storytelling/pull/32) | 58b536d0bc597b43c7f5a1eb87eef922cfadd062 | 3d121398e2c14c4a43d101a5b9a55a4616a81174 | 42449548994, 42449545432 / 42449705252 |

Each refreshed head passed all twelve jobs across two runs, and each merge
passed all six main jobs. Head/merge file trees match. GitHub showed no review submissions or threads. The maintainer authorized the merges in the task conversation.
At this 05:12 integration snapshot, Issue #27 showed items 2/3/6 checked and
1/4/5 open. PR #39 later updated item 2's unsupported historical functional-
RED claim, and PR #40 added the bounded runtime evidence described below. Trusted editorial approval, host execution, full M3 acceptance and release remained outside the integration scope.


## M3 evidence correction and paired runtime follow-up — 2026-11-08 11:51 UTC

PR [#39](https://github.com/bynanci/storytelling/pull/39) updated the pinned
source-test blob and the historical RED classification. Head
`6f4fea7180b2941b4997c0e17b17ae3835516d6f` merged as
`0611026eb1dd0379c685b9098dbb60839c83e1e2` at 11:24:50 UTC. Runs
42474991033/42474986836 passed all six jobs each. Post-merge run 42475637859
passed all six jobs.

PR [#40](https://github.com/bynanci/storytelling/pull/40) adds the actual
pinned upstream review-service oracle, a fixed Rust digest vector, and API/CLI
error-order regressions. Final head
`a9b80723e0171497d9b1cc63bc30c6fac88e0ecc` merged as
`3d858aea9901fdcd8436503ae7d314cfb0c75431` at 11:48:59 UTC. Both head runs
42477209609/42477206413 passed all six jobs, and post-merge run 42477392236
passed all six. The accepted head and merge tree are identical. Full local
debug/release suites each passed 219 tests. Strict Clippy, rustdoc, format,
repo guard and diff checks passed. The source harness ran locally against
upstream commit `7add4762fe8174d76f43b863375ea39b73121e76`.

The first, superseded PR #40 candidate (`19b38d6`) exposed a Windows-only
golden digest mismatch on the embedded fixture. The final head marks that
fixture as non-text so checkout cannot transform its bytes. Both final Windows
runs passed. The change made the input bytes deterministic on Windows. The upstream identity algorithm stayed unchanged.

Issue #27 remains open with items 3/4/6 checked and 1/2/5 unchecked. The two runtimes ran synthetic review cases only.
Digest parity, trusted signer authorization, full upstream parity, independent approval and G2 acceptance still needed separate evidence.
The record did not show a failing functional test before implementation or a release.

## M3-01/P1 signed bounded G2 acceptance — 2026-11-08

The maintainer/reviewer supplied the sign-off `NAN CHUN WANG
<by.nanci.tw@gmail.com>` and accepted M3-RB01–RB07 for the finite-OTIO profile
at reviewed main `4f43de0010e4e912c6367338d15b75bae5cf2720`. The decision,
guarantee boundaries, CI evidence and explicit historical-RED waiver are in the
[signed acceptance record](../migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md).
[PR #42](https://github.com/bynanci/storytelling/pull/42) delivered that record. It merged as `92f7f20b63656ec3e676309310f8a5350d6d0662` at 2026-11-08T13:24:51Z.
Its exact-head CI run 42484654708 and post-merge main run 42484845782 passed all
six jobs. Issue #27 closed with reason `completed` for this bounded profile.

The reviewer waived the requirement to prove the historical failing test for completion. The result remains unproven. PR #28 remains a
separate unaccepted Ed25519 candidate. This bounded acceptance does not establish
trusted signer authorization, source/build provenance, the planner-to-OTIO
bridge, host observation, replay protection, execution, full upstream parity or
release.

All six issue items show completion. Item 2 records the waiver for the missing historical test result. The checked-in machine
work index and appended `pr-register.json` observation carry the same final
receipts and preserve the earlier pre-acceptance observations.

## Selected planner proposal → read-only OTIO — 2026-11-11

[PR #44](https://github.com/bynanci/storytelling/pull/44) delivers bounded M3-01/P2,
M3-PB01–PB06. It compiles the selected proposal's actual cuts/order against matched
snapshot asset facts and includes the real artifact in a checked review bundle.
Changed assets, cuts or loss choices make the result stale or cause rejection. The compiler rejects unsupported audio and effects.
The operation leaves the Catalog and FilmCraft project untouched.

Head `42329d987f032ba979d76b4b0c1db993102a8383` merged as
`29efb79e4f68291ffd70259573a63e9f2bf2f113`. Both share tree
`5b36af3c76656924359f5043338c7fde027fadc9`. Head CI runs
42931642146/42931558985 and main run 42932270960 each passed all six jobs, including
Linux/Windows/macOS fixed digest vectors and the official OTIO roundtrip step.
The change preserves the original composition-only regression tests, lockfile, licenses and production dependency boundary.

This is a bounded technical delivery under the user's verified-merge authority,
with self-review disclosed. PR #28's independent G2 security review and explicit
candidate acceptance remain pending. Human editorial acceptance and execution
are separate. Tasks now queue M3-02 journal, M3-03 FilmCraft apply/readback, then
M4 upstream feature flag. A host apply timeout requires persisted unknown state
and readback before retry. An ambiguous outcome stops the operation to prevent a duplicate application.


## M3-PB07 allocation-path optimization — 2026-11-11

[PR #46](https://github.com/bynanci/storytelling/pull/46) adds a direct OTIO API
and reuses the compile/bind result in freshness checks, avoiding an unused portable
bundle on both paths. Prepare output and artifact identity remain byte-stable.

Head `1a77e4fd0c942ece58223135013d83a744d96921` (tree `28aa466ed03737ccb13e364296dfe7fc9b9025ea`) merged as
`d9778ebdd4b7666cff0c47889c81f6c91079acce` at 2026-11-11T00:58:14Z. Exact-head runs
[42935098963](https://github.com/bynanci/storytelling/actions/runs/42935098963) and
[42935144867](https://github.com/bynanci/storytelling/actions/runs/42935144867), plus
post-merge run [42935450529](https://github.com/bynanci/storytelling/actions/runs/42935450529),
each passed all six jobs. The record contains no performance measurements.
