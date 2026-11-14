<!-- SPDX-License-Identifier: MIT -->
# M4-01 · 唯讀 Consumer SDK 設計

[English](README.md) · 繁體中文 · [Issue #107](https://github.com/bynanci/storytelling/issues/107) · [ADR 0013](../../adr/0013-read-only-consumer-facade-v1.md)

**狀態：G2 設計提案。** 本資料包沒有實作 facade module、凍結公開 API 或交付
CLI／SDK 等價測試結果。起始 source snapshot 為
`52dc6c315f7b3d20693f3c51e052509997ab791b`（2026-11-14 UTC）。本次核對時
#107 仍 Open、沒有接受留言；議題明確允許先做唯讀設計。PR #117 後續已於
`a23a2909cf3734d207c66b372f29a85c5e541c86` 合併提案文件，此交付沒有接受
#103 的 G2／W01，也沒有建立正式執行權限。

## Consumer 預計使用的入口

候選方案是在既有 `storytelling-engine` crate 提供 `consumer_v1` 公開 module，
只收斂八種同步唯讀操作：計畫驗證、敘事投影、版本歷史、影響查詢、影片 coverage、
規劃、選定提案的審核準備與 freshness 檢查。每個入口直接呼叫一次既有純 engine
函式，保留已接受的 domain result 型別。工作區維持四個 crate。

以下是**尚未實作、目前不能執行的設計草圖**：

```rust,ignore
use storytelling_engine::consumer_v1;

let coverage = consumer_v1::video_coverage(&coverage_bytes)?;
if !coverage.ready_for_planning {
    // 將 unresolved 單元與診斷保留給作者。
}
let proposal = consumer_v1::plan_story(&planner_input_bytes)?;
let prepared = consumer_v1::prepare_story_review(
    &proposal_response_bytes, &snapshot_bytes, &context_bytes,
)?;
let exact_bundle = prepared.bundle_json();
```

Consumer 自行執行明確的輸入讀取，facade 只接收記憶體內的 bytes。它不讀路徑、
不建立 RPC dispatcher，也不重新實作 validator、planner、compiler 或 digest。
`PreparedStoryReviewV1` 沿用既有不透明物件，只有不可變的 record／artifact／bundle
存取；不能從可攜 DTO 反序列化出可信物件或取得可變的內部儲存。

有效但缺素材的 coverage 仍為 `Ok`／`ready_for_planning=false`；有效但過期的
審核仍為 `Ok`／`current=false`，保留全部 stale fields。這些狀態沒有授予 approve、
execute、recover 或 Host 存取能力。簽章有效、來源位元組、作者內容確認、當前授權
與 Host readback 都有獨立證據要求。

## 現在已可檢視的既有流程

以下命令已存在，不需要模型或剪輯器：

```sh
cargo run --quiet --locked -p storytelling-cli -- render-narrative \
  tests/fixtures/narrative-v1/n03-author-reviewed-v1.json
cargo run --quiet --locked -p storytelling-cli -- query-narrative-impact \
  tests/fixtures/narrative-v1/n04-revision-history-v1.json shared-observation-change
cargo run --quiet --locked -p storytelling-cli -- render-video-coverage \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
cargo run --quiet --locked -p storytelling-cli -- plan-story \
  tests/fixtures/planner-v1/valid.json
```

這些是可用的檢視步驟，沒有構成新的 facade 執行收據。現有輸入、stdout 與退出碼
以 [CONTRACTS.md](../../CONTRACTS.md) 為準。CLI 的 usage／I/O 錯誤仍由 I/O 層
負責，不能改寫成 engine domain error。

## 本次需要審查的決定

| 項目 | 候選方案／未決點 |
| --- | --- |
| API／套件邊界 | 既有 `storytelling_engine::consumer_v1`；不新增第五個 crate、不發布套件。 |
| 結果與所有權 | 收斂既有 accepted result 型別；沿用編譯器建立且不可偽造的審核物件。 |
| 錯誤介面 | 候選不透明 `SdkErrorV1` 保留 operation、原 code、可為 null 的 path，以及完整原始錯誤細節；細節 accessor 回傳 Result，序列化失敗使用候選 adapter code `sdk.error_details_encoding_failed`，並保留原錯誤；G2 須明確接受。 |
| 限制／版本 | 各操作原有 byte、depth、collection、time 限制、檢查順序與未支援版本錯誤直接保留；參與 digest 的 bytes 不做正規化。 |
| CLI 相容性 | 原 stdout／exit 逐位元組回歸；敘事／歷史／impact 以 typed fields 和原文字 golden 各自核對，不新增 JSON 契約或反向解析文字。 |
| 執行緒 | 同步純函式、唯讀輸入、結果自有資料；Send／Sync、重跑與 caller-thread 比較列為待實作測試。Consumer 自行限制並行記憶體。 |
| Mutation／recovery | 本 surface 不提供。#103–#106 接受及獨立能力設計完成前，不能由 feature 或 callback 啟用。 |

[測試計畫](test-plan.md)與[既有 fixture／test anchors](golden-plan.json)是預先
定義的斷言，沒有將它們勾成通過。G2 接受後另做聚焦實作 PR，先保存真正觀察到的
RED，再實作到 GREEN，檢查外部 consumer 編譯、CLI／SDK 等價與最終 head 的
Windows／Linux／macOS CI。既有 backend tests 不能證明尚未實作的 wrapper 正確。

## 相容性、遷移與證據

此方案不更動原 schema／profile、有理數時鐘、P1/P2 digest framing、OTIO bytes、
已接受的損失選擇或 CLI。舊的 crate-root exports 維持可用。Facade 等價測試通過後，
consumer 可逐項改 import；破壞 signature、type、ownership 或 error accessor 的
變更須新 facade major 與遷移審查，移除前先記錄 deprecation。回退是移除新 module
及可能的 CLI import refactor，不需要修改專案、Catalog、媒體或輸入檔。

| 證據 | 目前狀態 |
| --- | --- |
| G2 scope／API／版本政策接受 | pending |
| Facade 實作、RED／GREEN | 未實作／未執行 |
| CLI／SDK golden 等價 | 未執行 |
| 獨立 Rust／TypeScript consumer 操作 | 未觀察 |
| Host、當前授權與 recovery | 本介面不提供，依 #103–#106 |
| G5 發布 | 沒有決定 |

合併此提案只能記錄設計交付。#107 保持 Open，實作／驗收項目不能勾選完成。
#108 的 transport 選擇、#109 的 app／單一寫入者，以及 #111／#112 的真實宿主與使用者
證據保留各自依賴。維護者／審查者須對精確 candidate commit 及錯誤細節的 fallible accessor 與 adapter code
做實際接受或修改決定；通用 merge 指令沒有代替這些技術決定。
