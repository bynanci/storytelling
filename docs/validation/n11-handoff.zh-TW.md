<!-- SPDX-License-Identifier: MIT -->
# N-11：修傘工作坊的唯讀交接清單

[English](n11-handoff.md) · 繁體中文

**工作／需求：** #68／N-R11。**基準：** `main` `003191c834bfc46903f1a436fd73c5df6edd9a77`。**範圍：** 一份獨立版本的 manifest 及唯讀 CLI；既有 NarrativeDocumentV1、NarrativeRevisionLedgerV1 與影片 V1 契約保持原樣。[ADR 0009 提案](../adr/0009-read-only-mixed-media-handoff.md)

## 作者請求與素材

> 修傘活動改成週日，請告訴接手的人哪些稿要改。

[Storytelling Skill](../../skills/storytelling/SKILL.md)沿用[工作坊素材](../../skills/storytelling/references/workshop-demo.md)：S1 是通知，S2 是左側藍傘、右側赭黃色線軸的圖說，S3 是合成兩聲敲擊的描述。改成週日是本次指定修訂。C1 是只屬海報的排字提案，將文字放在物件間留白。本試驗查核宣告的引用，不讀圖、不播放音訊、不用雜湊推斷語意，也不驗證作者決定者身分。

## 先重播 #51

[#51](https://github.com/bynanci/storytelling/issues/51)已隨 [PR #76](https://github.com/bynanci/storytelling/pull/76)合併而結案。在實作 N-11 前，既有 CLI 的兩案均實際退出 0：

```sh
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json b-only-detail-change
```

[共用來源輸出](n11-handoff/n04-shared.stdout)對 S02@1→2 列出 `oral, illustration-wide, illustration-close`。[單版來源輸出](n11-handoff/n04-local.stdout)對 C01@1→2 只列 `illustration-close`。更新 main 後再重跑保存。工作坊的[獨立 ledger](../../tests/fixtures/narrative-handoff-v1/date-ledger-v1.json)也退出 0，列出邀請文、海報及兩版口述四個變體：[實際 #51 輸出](n11-handoff/date-impact.stdout)。

## 預期與實際逐項比對

[預期集合](../../tests/fixtures/narrative-handoff-v1/expected-impact.json)在執行新 CLI 前固定。[比對紀錄](n11-handoff/impact-comparison.json)的更新與沿用集合皆為**漏報 0、誤報 0**。

| 部件 | 預期／實際 | 改稿前 → 已提供的審閱後草稿 |
| --- | --- | --- |
| `invite/text` | UPDATE／UPDATE | 週六邀請 → 週日邀請 |
| `poster/date` | UPDATE／UPDATE | 週六日期 → 週日日期 |
| `oral-info/date` | UPDATE／UPDATE | 直接告知版週六 → 週日 |
| `oral-poetic/date` | UPDATE／UPDATE | 意象版週六 → 週日 |
| `poster/layout` | REUSE／REUSE | 藍傘仍在左、線軸仍在右，C1 仍只屬海報 |
| `oral-poetic/taps` | REUSE／REUSE | 同一兩聲敲擊安排 |
| `original-wav/audio` | REUSE／REUSE | 同一 S3@1 檔案身分 |

before 是 S1 已改成 revision 2、四份草稿仍引用 S1@1 而待審的時點；歷史草稿引用舊版是合法的。來源清單若把「目前 S1」錯標為 revision 1，則應拒絕。after 追加明確的變體審閱事件，保留週六稿並提供週日稿。`UPDATE` 表示指定修訂當時的影響，`status=current` 另表示 ledger 已收到審閱後稿件；兩者均不證明語意正確。

每份產物的部件聯集必須涵蓋綁定變體的全部來源 ID。省略已知日期引用會被拒絕，不能隱藏影響。提案範圍對整份綁定的草稿查核，不能靠省略 C1 的部件對照來隱藏它出現在別版。

## 正常 CLI 重播

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/before-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/after-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/descriptions-only-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/transcript-missing-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/missing-source-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/stale-revision-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/proposal-leak-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- query-narrative-impact tests/fixtures/narrative-handoff-v1/date-ledger-v1.json date-to-sunday
```

[實際命令與退出碼](n11-handoff/commands.json)使用剛建好的 release binary，所有標準錯誤皆空。

| 案例 | 實際標準輸出 | 退出碼 |
| --- | --- | --- |
| 待審的週六草稿 | [before](n11-handoff/before.stdout) | [0](n11-handoff/before.exit) |
| 已審閱的週日草稿 | [after](n11-handoff/after.stdout) | [0](n11-handoff/after.exit) |
| 只有來源描述 | [descriptions](n11-handoff/descriptions-only.stdout) | [0](n11-handoff/descriptions-only.exit) |
| 圖片待補與非語音文字標註 | [部分交付](n11-handoff/transcript-missing.stdout) | [0](n11-handoff/transcript-missing.exit) |
| 不存在的來源 | [handoff.dangling_reference](n11-handoff/missing-source.stdout) | [1](n11-handoff/missing-source.exit) |
| 目前來源錯帶 S1 舊 revision | [handoff.stale_revision](n11-handoff/stale-revision.stdout) | [1](n11-handoff/stale-revision.exit) |
| 海報專用 C1 被直接告知版引用 | [handoff.variant_scope](n11-handoff/proposal-leak.stdout) | [1](n11-handoff/proposal-leak.exit) |

只有描述時，仍交付可用邀請文與構圖說明，清楚標示原始素材檔未交付。transcript 案例是作者提供的「敲擊、停頓、敲擊」非語音標註，沒有宣稱 taps.wav 有台詞或本工具完成轉錄。missing 保留缺項與仍可使用的圖說。[fixture 說明](../../tests/fixtures/narrative-handoff-v1/README.md)

## 重播、來源未變與測試

[重播紀錄](n11-handoff/replay.json)記錄再次由檔案及 stdin 執行均退出 0，標準輸出逐位元相同。整合測試另核對隔離目錄沒有新增檔案，輸入 bytes 保持原樣。[前後雜湊](n11-handoff/source-readonly.json)包含前後通知、SVG、WAV、各 manifest、ledger 和預期集合；命令未改寫任何檔案。

renderer 只檢查摘要格式。另以真正的 `verify-source` 核對[通知 bytes](n11-handoff/verify-S1.stdout)、[SVG bytes](n11-handoff/verify-S2.stdout)、[WAV bytes](n11-handoff/verify-S3.stdout)，均退出 0。這些核對只證明位元組身分，不判斷改稿是否忠於來源或素材品質。

實際先測 RED：`cargo test --locked -p storytelling-cli --test handoff` 退出 101；最初 10 案全因 CLI 尚無 `render-handoff` 而收到 usage exit 2。[原始紀錄](n11-handoff/tdd-red.log) · [退出碼](n11-handoff/tdd-red.exit)。最終 GREEN 為 12 個 CLI 整合案例，另含省略已知來源、隱藏提案範圍的回歸。保留原斷言強度；範圍句改明「來源」檔案，因 CLI 必須讀取 manifest 輸入。

[本機門檻與退出碼](n11-handoff/local-checks.json)列出格式、嚴格 Clippy、工作區 debug/release 與 warnings-denied Rustdoc。既有影片 V1、摘要與 OTIO 測試均在工作區套件內。同一送審提交的 Required gates、稽核、三平台 Rust 與完整品質／oracle 工作流須在 PR 核對。沒有真人稿件驗收、聽感或畫質結果；語意分組、週日措辭是否忠於來源及宣告的作者選擇仍待作者確認。

另一代理進行獨立自動審查，實際重播省略來源與隱藏提案的案例。審查發現不完整部件對照會隱藏受影響來源；修正後，完整覆蓋與整份變體的提案範圍查核都會拒絕這些案例。最終複查沒有其他待修問題，也驗證四個正例符合 schema，獨立重算所有保存的來源雜湊。自動審查不等同真人驗收；實作者另自行審閱最終差異。
