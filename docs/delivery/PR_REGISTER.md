<!-- SPDX-License-Identifier: MIT -->
# PR 交付登錄

本登錄把已合併交付、關閉但未合併的提案，以及尚待審查的依賴更新分開記錄。
原始觀察是 `2026-11-07T05:41:45Z`，主線為 [`867fcde`](https://github.com/bynanci/storytelling/commit/867fcde6acd0dd1339122aaf4bb23003aac3942e)；
依賴 PR 的 mergeability 於 `2026-11-07T05:45:37Z` 重新讀取。
完整 head／tree／merge／時間／CI run 與 task issue 身份見
[機器登錄](pr-register.json)。這是可追溯快照，不會自動涵蓋之後新增的 PR。

## 已合併的實作交付

| PR／有限範圍 | Accepted head | Merge commit | Merged at UTC | 觀察到的 exact-head checks |
| --- | --- | --- | --- | --- |
| [#1](https://github.com/bynanci/storytelling/pull/1) M0 宣告驗證 foundation | [`4c5682c`](https://github.com/bynanci/storytelling/commit/4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b) | [`2227981`](https://github.com/bynanci/storytelling/commit/22279814ff6b147c386186569d7f77f3a803cf8b) | 2026-11-07 02:17:16 | 10 success |
| [#7](https://github.com/bynanci/storytelling/pull/7) M1-01 finite snapshot importer | [`c2fb478`](https://github.com/bynanci/storytelling/commit/c2fb478c278550b682efba5f898082e8bcd20e24) | [`610ed49`](https://github.com/bynanci/storytelling/commit/610ed49673d6f554e539008b34f5fc5479b9dbc1) | 2026-11-07 02:18:09 | 10 success |
| [#16](https://github.com/bynanci/storytelling/pull/16) M1-02 exact-time OTIO | [`fb65753`](https://github.com/bynanci/storytelling/commit/fb65753df617c7df80ef2581396eead6b07a6434) | [`9a5232e`](https://github.com/bynanci/storytelling/commit/9a5232e97435f543238b90a2006460f319f4e651) | 2026-11-07 03:09:33 | 10 success |
| [#17](https://github.com/bynanci/storytelling/pull/17) M2-01 bounded Brief audit | [`1c8c01d`](https://github.com/bynanci/storytelling/commit/1c8c01da48af75b860b1ce49241c3fd509b16c59) | [`422190e`](https://github.com/bynanci/storytelling/commit/422190e27bc604bd71485f78279373992f99e68c) | 2026-11-07 03:20:26 | 10 success |
| [#18](https://github.com/bynanci/storytelling/pull/18) M1-03 supplied-buffer／實際宿主證據 | [`2b859d2`](https://github.com/bynanci/storytelling/commit/2b859d29e22f6f2a39f73822086b59618bb8a402) | [`e0a5ade`](https://github.com/bynanci/storytelling/commit/e0a5ade7ee2c0414f87aa121bef1bdf259a7e75e) | 2026-11-07 03:26:40 | 10 success |
| [#22](https://github.com/bynanci/storytelling/pull/22) M2-02 fixed-window planner | [`1f4ff08`](https://github.com/bynanci/storytelling/commit/1f4ff082ed27dbc6223af2f0a0b1cfe71a06fe54) | [`895257b`](https://github.com/bynanci/storytelling/commit/895257b1bb9993d70cae3f1267f5f6fa3de91905) | 2026-11-07 03:59:29 | 10 success |

六項實作的 accepted-head tree 與實際 merge tree 均相同。這些是逐切片接受：
M0／M1-01 只驗證或映射宣告；M1-02 是有限格式編譯；M2-01 是硬限制稽核；
M2-02 是宣告固定區間規劃。M1-03 的實際證據只涵蓋 supplied-buffer SHA-256
與一個合成 FilmCraft Linux／25fps／CPU 案例，包含192×108預覽，
不是音訊、GUI／GPU、完整解析度或一般宿主 executor 的證明。

## 已合併的規劃與狀態文件

| PR／文件範圍 | Accepted head | Merge commit | Merged at UTC | 觀察到的 exact-head checks |
| --- | --- | --- | --- | --- |
| [#2](https://github.com/bynanci/storytelling/pull/2) 48 OF＋16 GAP 規劃索引 | [`95291d3`](https://github.com/bynanci/storytelling/commit/95291d31c47601181070bf89a28b1a182ddf6ed8) | [`9fb403b`](https://github.com/bynanci/storytelling/commit/9fb403be811643baf948acdb6d72232d4a6207bb) | 2026-11-07 02:17:48 | 15 success |
| [#11](https://github.com/bynanci/storytelling/pull/11) M0／M1-01 accepted lifecycle | [`dbf2fd4`](https://github.com/bynanci/storytelling/commit/dbf2fd45ef6a215e59cabbe9a6278bc09ca53daa) | [`440209f`](https://github.com/bynanci/storytelling/commit/440209f21d2b5fd6e7b4cad0823359c802cf8c8e) | 2026-11-07 02:30:58 | 10 success |
| [#19](https://github.com/bynanci/storytelling/pull/19) OTIO／Brief／宿主狀態同步 | [`8997314`](https://github.com/bynanci/storytelling/commit/89973143d9bcd6c1c99e40f37f6fa4f13972dc59) | [`c03d4c8`](https://github.com/bynanci/storytelling/commit/c03d4c8ede33563451b8ad590a6df982d0d8b82a) | 2026-11-07 03:35:43 | 10 success |
| [#23](https://github.com/bynanci/storytelling/pull/23) 固定視窗規劃 accepted lifecycle | [`88a188b`](https://github.com/bynanci/storytelling/commit/88a188b5d3aa54e6ac76f7f6340f15b937248693) | [`867fcde`](https://github.com/bynanci/storytelling/commit/867fcde6acd0dd1339122aaf4bb23003aac3942e) | 2026-11-07 04:06:47 | 10 success |

文件 PR 不增加 runtime 能力。#2 的 accepted head 觀察到 **15** 個成功 checks，
分屬三個 run；其他表列交付各有10個。不得把它們統一簡化為「全部10checks」。
各 run 的 canonical URL、job 名稱與結論保留在機器登錄。

## 已關閉但未合併

[#13](https://github.com/bynanci/storytelling/pull/13) 的 head 是 [`6268766`](https://github.com/bynanci/storytelling/commit/62687668ea7c36d8ffd555bb33d4c909a461368b)，
tree `bc9be7dc22d8c2cb4610986cd8b7adb2a453e361`；它有5個成功 checks，仍然**沒有 merge commit**。
這個 Draft 狀態整理提案已被 #11／#16／#17 接替；有用的來源漂移修正由 #17
選擇性核對後帶入。它不列為已接受的實作或「已合併完成」。

## 待處理的依賴與 Actions 更新

| PR | 變更閉包 | 觀察 head | Head 第一父提交 | 該 head 的 checks | 最新 REST mergeable／state |
| --- | --- | --- | --- | --- | --- |
| [#8](https://github.com/bynanci/storytelling/pull/8) | checkout 4.4.0 → 7.0.1；Node20 → Node24 | [`f3bf985`](https://github.com/bynanci/storytelling/commit/f3bf985acbd6abb67b3ce5f781535387dc602373) | [`2227981`](https://github.com/bynanci/storytelling/commit/22279814ff6b147c386186569d7f77f3a803cf8b) | 5 success | true / clean |
| [#9](https://github.com/bynanci/storytelling/pull/9) | serde_json1.0.145 → 1.0.154；ryu → zmij1.0.23 | [`cbb8e7d`](https://github.com/bynanci/storytelling/commit/cbb8e7dd2f46de274bcde2de887f706b057b360d) | [`e0a5ade`](https://github.com/bynanci/storytelling/commit/e0a5ade7ee2c0414f87aa121bef1bdf259a7e75e) | 5 success | true / clean |
| [#10](https://github.com/bynanci/storytelling/pull/10) | serde1.0.228 → 1.0.229；syn2 → syn3.0.9 | [`6a2456a`](https://github.com/bynanci/storytelling/commit/6a2456a2008d937b1658bbb77866bee4f2b694b0) | [`e0a5ade`](https://github.com/bynanci/storytelling/commit/e0a5ade7ee2c0414f87aa121bef1bdf259a7e75e) | 5 success | true / clean |
| [#21](https://github.com/bynanci/storytelling/pull/21) | setup-python6.3.0 → 7.1.0；仍為 Node24 | [`ad4e33f`](https://github.com/bynanci/storytelling/commit/ad4e33f0406a78336279fe7916d3372660169d02) | [`c03d4c8`](https://github.com/bynanci/storytelling/commit/c03d4c8ede33563451b8ad590a6df982d0d8b82a) | 5 success | true / clean |

四項 PR 在最新 REST 讀取時都可技術合併。較早的 normalized metadata 曾對
#8／#9／#10 回傳 false；重新讀取後為 true／clean，因此本登錄不宣稱有持續的
merge conflict。這些綠燈描述各自舊 head，不代表已完成目前主線整合或依賴審查。

目前 repository guard 檢查 dependency 集合及已審查的 SHA2 配置，
沒有額外釘死 serde／serde_json 版本；本次未觀察到它與這四項更新的直接規則衝突。
然而 #9 的新數值格式依賴、#10 的 proc-macro／syn major 閉包、#8 的 Action runtime
以及 #21 的新 Action distribution 都仍須各自審查。接受前應固定來源身份，
核對 Rust1.85／權限／授權／lockfile 或 Action distribution，並重新整合當時 main，
跑相同 head 的必要檢查及來源契約比對。這份整理沒有合併依賴 PR。

## 任務與 PR body 整理

快照時8個任務 issues 均為 completed，且沒有未勾選項目：
[#3](https://github.com/bynanci/storytelling/issues/3)、
[#4](https://github.com/bynanci/storytelling/issues/4)、
[#5](https://github.com/bynanci/storytelling/issues/5)、
[#6](https://github.com/bynanci/storytelling/issues/6)、
[#12](https://github.com/bynanci/storytelling/issues/12)、
[#14](https://github.com/bynanci/storytelling/issues/14)、
[#15](https://github.com/bynanci/storytelling/issues/15)、
[#20](https://github.com/bynanci/storytelling/issues/20)。

#1／#2／#7 的開頭已加上有日期與 snapshot main 的 accepted lifecycle note，
修正「M1-02／M1-03仍未完成」及「#13仍是等待中的承載 PR」等過期 current-state
描述。原 body 完整保留為歷史證據；readback 及重複執行不再新增 note 的檢查通過。
標記為 `<!-- storytelling-pr-lifecycle-register-v1 -->`。
未修改 PR comments、labels、分支、merge 狀態或 repository 管理設定。

## 如何延續

新增交付時，先讀 GitHub 的實際 state、accepted head、merge tree／時間與同 head
checks，再把新觀察或 delivery addendum 加入機器登錄。保留原 snapshot 身份與
已接受切片的原始證據；無法核實的項目保持 pending，closed-unmerged 不提升成 merged。
當前任務權威仍由 [TASKS](../../TASKS.md)、[需求](../REQUIREMENTS.md)、
逐任務 comparison records 與真實 GitHub metadata 共同提供。

完整 solver／M2-03、production execution／approval、人工剪輯驗收、發布和管理
權限不因 PR 數量、文件勾選或本登錄而完成。原始48＋16項 inventory 仍是覆蓋範圍，
不是產品完成百分比。

## 後續接受交付：2026-11-07 06:27 UTC

原始十五個 PR 快照保持原樣；本段另加後續兩個已合併交付，主線為
[`5533fcb`](https://github.com/bynanci/storytelling/commit/5533fcb5d0f857ad1066a76505071653b66d1f89)。
目前文件整理 PR 本身不在這個已觀察快照內；最新清單以
[GitHub 已合併 PR](https://github.com/bynanci/storytelling/pulls?q=is%3Apr+is%3Amerged)
為準。

| PR／範圍 | Accepted head | Merge commit | Merged at UTC | 同 head checks |
| --- | --- | --- | --- | --- |
| [#25](https://github.com/bynanci/storytelling/pull/25) 已完成／接替／待審 PR 登錄 | `0030fe206f62c7c058e2337e98945d0e72a06954` | `073b5a415324d588019e2155dcf24ddddd860adf` | 2026-11-07 05:56:33 | 10 success |
| [#26](https://github.com/bynanci/storytelling/pull/26) M2-03 選定範圍即時 shadow | `4bf888949107a21b560b09222175ff1f25118a3b` | `5533fcb5d0f857ad1066a76505071653b66d1f89` | 2026-11-07 06:27:14 | 10 success |

兩者 accepted-head tree 均與 merge tree 相同。#25 是文件交付；#26 新增測試工具與
Linux CI，production Rust／Cargo／既有 fixtures 與宿主 receipt 均未變。
#26 的兩組 CI 品質紀錄均顯示40項 shadow 測試通過，以及64組實際配對：
56一致、4已定義政策差異、4不支援探針、0未分類。乾淨已發布 head 的原始產物
另經獨立精確 Fraction 比對與336個產物雜湊查核；見
[M2-03 接受證據](../migration/records/M2-03.md#accepted-delivery-and-clean-candidate-proof)。

[任務 #24](https://github.com/bynanci/storytelling/issues/24) 已於06:27:33UTC全數勾選並
關閉 completed；原始八個已完成任務紀錄不變。截至該歷史快照，依賴 PR #8／#9／#10／#21
仍是待審更新；本次交付沒有接受其 dependency closure 或將舊 CI 當作目前主線驗證。
M2-03 僅接受 frozen profile 內的 synthetic selected domains，完整 solver、正式
宿主執行與人工剪輯驗收仍為後續工作。

## 後續觀察到的待審實作

另於2026-11-07觀察到 Draft [#28](https://github.com/bynanci/storytelling/pull/28)：
`c162a47dcb8de4c582a3c66c22396ecfd8cb0134`，基底為已接受的
`5533fcb5d0f857ad1066a76505071653b66d1f89`。它提出 bounded approval identity
與新的 Ed25519 相依套件；該 head 的10項 CI 已成功。獨立程式／依賴審查
未發現限定簽章驗證範圍的程式阻礙，並核對39packages／15新增版本的 features、
MSRV 與 licenses；但其依賴文件要求 exact-head CI audit，workflow 尚無
`cargo audit` 步驟。這項門檻及來源比較／第一方分類邊界仍需釐清，尚未接受或合併。這不改寫原始快照或上述已接受交付。

另開 [#27](https://github.com/bynanci/storytelling/issues/27) 準備 review-binding／
freshness 範圍；[規劃文件](../migration/M3-01-NEXT.md) 明列 proposed M3-RB IDs，
避免與 #28 的 M3-A IDs 混用。#27 保持 open，尚無實作驗收或勾選完成。


## Current lifecycle reconciliation: 2026-11-07 13:50 UTC

The earlier tables above remain historical observations. Since that snapshot,
PRs [#8](https://github.com/bynanci/storytelling/pull/8),
[#21](https://github.com/bynanci/storytelling/pull/21),
[#10](https://github.com/bynanci/storytelling/pull/10) and
[#9](https://github.com/bynanci/storytelling/pull/9) merged their action and
Serde dependency updates; their PR-head and post-merge main checks passed.

[PR #28](https://github.com/bynanci/storytelling/pull/28) merged head
`2d23b21dcfbfc49c3d76087f054cc76724c23053` at
`5a5702ba4c1392d763929f89da3cf7d16d5f13f4` on 2026-11-07 13:47:15 UTC. Both
same-head CI runs and the post-merge main run passed Quality, Required, Linux,
Windows, macOS and Dependency Audit. The audit loaded 1,422 advisories and
scanned the 40-package lockfile without findings. The merge integrates a bounded
candidate only: independent G2 review and product acceptance remain pending.
Issue [#27](https://github.com/bynanci/storytelling/issues/27) remains open for
the separate finite-OTIO review identity and declared-context freshness scope.
At this observation there are zero open pull requests; the prior machine snapshot
is preserved and this state is appended in `pr-register.json`.

## Current open M3-01 review-binding candidates — 2026-11-07 23:42 UTC

The following current-state observation supplements the historical snapshots above. It does not
accept the proposed contract, issue a G2 approval, complete issue #27, or authorize a merge.

| PR | State / base | Exact head | Same-head checks | Acceptance |
| --- | --- | --- | --- | --- |
| [#31](https://github.com/bynanci/storytelling/pull/31) | Draft; base main `9513913e849f8cf15d9e5bbb2977fe587ce8b953` | `67abf6d443abf11a0cbced519eeac2a86396a7f7` | All 6 passed in [run 42381290915](https://github.com/bynanci/storytelling/actions/runs/42381290915) | G2 contract review pending; documentation proposal only |
| [#32](https://github.com/bynanci/storytelling/pull/32) | Draft; stacked on #31 head `67abf6d443abf11a0cbced519eeac2a86396a7f7` | `a979c6ad6244a9d20a8c276f43e21d310d8d8fbd` | All 6 passed in [run 42426266361](https://github.com/bynanci/storytelling/actions/runs/42426266361): Quality, Required, Linux, Windows, macOS and Dependency Audit | Provisional implementation; independent G2 code/security and source-comparison disposition pending |

The #32 candidate adds same-input finite importer/OTIO binding, a private non-deserializable
in-process result, strict declared context, a read-only freshness CLI and a static source-informed
comparison. It does not verify a signature or signer authorization, observe the declared host or
binary, establish source provenance, prevent replay, or bridge planner output to OTIO. The
TypeScript source runtime was not executed; the comparison is explicitly not a paired-runtime oracle.

Issue [#27](https://github.com/bynanci/storytelling/issues/27) remains open with all six checklist
items unchecked. No reviewer has been requested; maintainer/G2 acceptance remains pending. PR #32
is mergeable but remains Draft and is intentionally stacked on #31 so the review packet and merged
PR #28 code remain in its hosted integration baseline. Rebase or merge only after the independent
G2 contract decision; do not interpret green CI as product acceptance.


## Conditional merge review — 2026-11-08

At 01:33 UTC, the remaining open feature/design PRs are the following Drafts.
Their green CI does not close their explicit independent G2 conditions.

| PR | Observed head | Delivery decision |
| --- | --- | --- |
| [#31](https://github.com/bynanci/storytelling/pull/31) | 67abf6d443abf11a0cbced519eeac2a86396a7f7 | Retain Draft; no submitted independent G2 review |
| [#32](https://github.com/bynanci/storytelling/pull/32) | 1a281aa93a73026d24f9305ea46a6fe91e204299 | Retain Draft; no submitted G2 review; static source comparison still needs disposition |

PR #32's six jobs passed in run 42427010497 and no review thread exists.
Issue #27 remains open. A separate scheduling optimisation of the accepted
M2-02 contract is tracked by [#33](https://github.com/bynanci/storytelling/issues/33);
its final CI/merge metadata belongs to that issue and its associated PR.


## Accepted ordering optimisation — 2026-11-08 reconciliation

PR [#34](https://github.com/bynanci/storytelling/pull/34) merged accepted head
`5bbc3a0e867a90e3f8829c3cf7ab6b2c3961b3b0` as
`f07ab6940484ef8b8e4f2bdb37b96893e9f92270` at 01:37:08 UTC.
Head runs 42434851833/42434822453 passed all 12 jobs, and main run 42435022548
passed all six; the file trees match. Issue #33 is fully checked and closed
completed. The scope preserves the accepted fixed-window profile and received
self-review under user merge authority. No release or M3 G2 grant is implied.

At the 04:21 UTC continuation, PRs #31/#32 still have no submitted review and
remain Draft. A separate cached-clock follow-up is tracked in
[issue #35](https://github.com/bynanci/storytelling/issues/35); its final hosted
checks/merge belong to that issue and its associated PR.

## Accepted cached-clock optimisation — 2026-11-08

PR [#36](https://github.com/bynanci/storytelling/pull/36) merged head
`bdd502a8f8cd36c7b822c91fba4ff87215e8beb1` as
`00ef8decbf1fecc7258bbc683973d41d656c8097` at 04:28:04 UTC. Head runs
42446786881/42446759799 passed all 12 jobs; main run 42446933790 passed all six.
The accepted head and merge file trees match. Issue #35 is fully checked and
closed completed. The scope retains the accepted fixed-window time/error
contract and was self-reviewed under user authority. M3 PRs #31/#32 remain
Draft candidates pending independent G2; no release was published.


## M3 contract/candidate integration — 2026-11-08 05:12 UTC

The user reported “審查完畢 merge” and explicitly requested integrating the
remaining PRs. Both are now merged after current-main conflict resolution and
new head checks; original M3 Rust code/test blobs were retained.

| PR | Refreshed head | Merge | Head runs / main run |
| --- | --- | --- | --- |
| [#31](https://github.com/bynanci/storytelling/pull/31) | 937cf5a1db61d1bccb401241c8d3001bc02e49f5 | baa97903e3d3c02e2eb2747140d62f1ddca3c60c | 42449119799, 42449115934 / 42449339199 |
| [#32](https://github.com/bynanci/storytelling/pull/32) | 58b536d0bc597b43c7f5a1eb87eef922cfadd062 | 3d121398e2c14c4a43d101a5b9a55a4616a81174 | 42449548994, 42449545432 / 42449705252 |

Each refreshed head passed all twelve jobs across two runs, and each merge
passed all six main jobs. Head/merge file trees match. GitHub review submissions
and threads were empty at inspection; the task-conversation statement is the
recorded maintainer authority, without inventing independent review artifacts.
Issue #27 checks delivered items 2/3/6 and remains open for 1/4/5 evidence. The
M3 review-source comparison is static. No trusted editorial approval, host
execution, full M3 acceptance or release is asserted by this integration.
