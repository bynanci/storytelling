<!-- SPDX-License-Identifier: MIT -->
# Open issues 查核與作者修訂紀錄 · 2026-11-11

[English](open-issues-2026-11-11.md) · 繁體中文

初次查核基準為 main `c1f164354c3e88d90acb32075828954091c149d8`，當時有 #48、#50、#52、#53、#54、#65、#67 共七張 open issues。它們都有未滿足的條件，不能只因程式已合併而結案。此次對話取得請求者對 N-03、N-05 的實際檢視與 N-06 明確暫緩決策，因此可完成三項工作。真人量測及 N-08 盲讀仍未執行。

## 逐票結果

| Issue | 已驗證的交付 | 結案或剩餘條件 |
| --- | --- | --- |
| [#50／N-03](https://github.com/bynanci/storytelling/issues/50) | 請求者已檢視三稿；revision 2 回應近景建議，原始單元、口述及寬景不變。 | 本次修訂及證據合併後結案；不宣稱請求者已再次讀過 C02 修訂稿。 |
| [#52／N-05](https://github.com/bynanci/storytelling/issues/52) | 已記錄逐項語意判斷；修訂稿刪除無依據的成效句，明示標語與 CTA 為提案。 | [PR #83](https://github.com/bynanci/storytelling/pull/83) 已合併，提交 `1d7086da59f64ca456dc23864fb11cfe1253c389`。 |
| [#53／N-06](https://github.com/bynanci/storytelling/issues/53) | 三候選的相容性與最小變更已比較；請求者明確暫緩三者。 | [PR #85](https://github.com/bynanci/storytelling/pull/85) 已合併，提交 `ad798e2a888850b4ac15e99ad65633cb67e51fe6`；未採用新模式。 |
| [#48／N-01](https://github.com/bynanci/storytelling/issues/48) | 合成素材、基準草稿、量測與擬議門檻已備妥；已依 #49 的明確豁免修正過期阻擋敘述。 | 保留開啟，待量測規則同意、實際成對觀察及維護者研究決策。#49／#50 已有獨立的限定授權。 |
| [#54／N-07](https://github.com/bynanci/storytelling/issues/54) | N-05 依賴已交付；既有 N-01 與補充評估包可供準備試用。 | 保留開啟，待一般筆記／Skill／native 的真人觀察、分母、改稿耗時、歸因錯誤與繼續／縮小／停止決策。 |
| [#65／N-08](https://github.com/bynanci/storytelling/issues/65) | 有效順序、矛盾、未知版本、同輸入重播均通過；獨立無標籤閱讀表已備妥。 | 保留開啟，待實際讀者答案；將已完成的重播與未完成的盲讀拆開。 |
| [#67／N-10](https://github.com/bynanci/storytelling/issues/67) | 可引用 [N-06 明確暫緩決策](https://github.com/bynanci/storytelling/issues/53#issuecomment-6792555714)。 | 保持開啟／暫緩。須有實際作者需求，且新決策採用單一模式與版本範圍才啟動。 |

## 實際回饋與修改

[N-03 回覆](https://github.com/bynanci/storytelling/issues/50#issuecomment-6792812347)指出口述、構圖差異及來源標示整體可用，並要求釐清近景與工具位置。回覆提到的錯別字並不存在於 fixture 或真實輸出；未虛構修字成果。[N-03 前後對照](n03-narrative-cli.md#requester-inspection-and-corrective-draft--2026-11-11)保留此核對與修訂理由。C02 是回應檢視而提出的近景構圖選擇，不是現場觀察，也不是已記錄的作者核准事件。

[N-05 回覆](https://github.com/bynanci/storytelling/issues/52#issuecomment-6792555292)支持限定為一次測試的句子，拒絕十分鐘平均值與 30% 的主張。交付採刪句；另一建議改寫仍含未獲來源支持的省費主張，沒有使用。標語與 CTA 保留提案類別，並加上清楚前綴。[修訂稿及 CLI 證據](n05-commercial-copy.md#revised-draft-and-normal-cli-replay)已保存。

N-06 的實際回答為「暫緩三者，等實際需求」。[比較及決策紀錄](n06-format-discovery.md)將此決策和先前建議分開，未執行的任務觀察欄位仍留白。

這些是請求者在對話提交的編輯判斷，不是獨立參與者研究；沒有據此填入人數、耗時、隨機比較、盲讀或發布核准。

## 本機重播

最後重播使用 main `ad798e2a888850b4ac15e99ad65633cb67e51fe6` 的 CLI 程式碼、既有案例及本次新增的 N-03 revision 2。[results.json](open-issues-2026-11-11/results.json)保存 14 條正常 CLI 案例的真實輸出、退出碼、重播及 12 個輸入檔的前後摘要。所有預期退出碼一致、重播逐位元相同、輸入未改。N-05 的缺引用／過期／懸空報告退出 0，代表成功回報引用狀態，不代表認定文案有佐證。

- N-03：[原稿](open-issues-2026-11-11/n03-render.stdout)、[修訂稿](open-issues-2026-11-11/n03-reviewed.stdout)、[修訂重播收據](open-issues-2026-11-11/n03-reviewed-replay.json)、[壞引用](open-issues-2026-11-11/n03-dangling.stdout)。
- N-04：[共用修訂](open-issues-2026-11-11/n04-shared-impact.stdout)、[單版修訂](open-issues-2026-11-11/n04-local-impact.stdout)。
- N-08：[兩稿](open-issues-2026-11-11/n08-render.stdout)、[有效順序](open-issues-2026-11-11/n08-valid.stdout)、[矛盾](open-issues-2026-11-11/n08-conflict.stdout)、[未知版本](open-issues-2026-11-11/n08-unknown-version.stdout)。
- N-05：[一般案例](open-issues-2026-11-11/n05-current.stdout)、[過期引用](open-issues-2026-11-11/n05-revised.stdout)、[懸空引用](open-issues-2026-11-11/n05-dangling.stdout)、[修訂稿](open-issues-2026-11-11/n05-reviewed.stdout)、[未知版本](open-issues-2026-11-11/n05-unknown-version.stdout)。

[15 個補充邊界檢查](open-issues-2026-11-11/n05-boundaries.json)在加入作者紀錄前的實作 head `41d6d4b5a10cbc3ff26678d0e8484f075f65fb4c` 執行，全部通過。範圍含空集合、128／129 主張、單項 64／65 引用、合計 2048／2049 引用、重複識別碼、未知 revision、512／513 字目的、1 MiB 邊界及過大輸出。Git 比對也確認該 head 的 Rust 與契約來源和最後重播的主線相同，已記入收據。

```sh
cargo build --release --locked -p storytelling-cli
target/release/storytelling render-narrative tests/fixtures/narrative-v1/n03-author-reviewed-v1.json
target/release/storytelling audit-copy-claims tests/fixtures/commercial-copy-v1/n05-copy-reviewed-v1.json tests/fixtures/narrative-v1/n04-revision-history-v1.json
target/release/storytelling query-narrative-impact tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
target/release/storytelling check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-v1.json
target/release/storytelling check-narrative-style tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json tests/fixtures/narrative-v1/n08-style-profile-conflicting-v1.json
```

最後一條刻意回傳 1。其餘命令與 stdin 值都在 manifest 逐項列出；本次查核不改寫素材、封閉契約、歷史紀錄或執行行為。

## 尚需實際執行的人工工作

N-08 應只把[獨立閱讀表](open-issues-2026-11-11/reader-sheet.zh-TW.md)交給新讀者。主持人分配 A／B 順序，記錄實際順序及是否看過相關稿件，待初步作答後再給來源說明。已接觸版本說明的人不能算新的盲讀。表中的答案仍留白。

N-01／N-07 沿用[五位作者成對試用規則](../narrative/N-01-AUTHOR-STUDY.zh-TW.md)及[補充評估包](../../examples/evaluation/README.zh-TW.md)，記錄實際請求、宿主／模型／commit、條件順序、完成數與分母、秒數、歸因錯誤及重複輸入。Skill 與 native 分組報告。請求者／維護者負責規則與繼續／縮小／停止決策；主持人負責招募和真實觀察。本次沒有收到這些資料。

[遠端檢查快照](open-issues-2026-11-11/hosted-checks.json)保留 PR #83、#85 的確切 head、合併提交及通過結果；本次查核 PR 另以自己的同 head CI 與審查紀錄驗證。

本機格式、Clippy、完整 debug／release 測試、嚴格 rustdoc、儲存庫查核、Skill 散布、圖形規劃／trace 及宿主證據拒絕測試均通過。既有 pinned FilmCraft 整合測試仍按設定忽略，未宣稱新的宿主實跑。收據目錄的 Git 屬性保留 `.stdout` 原始位元組及渲染器結尾空行，未修剪輸出以配合格式檢查。
