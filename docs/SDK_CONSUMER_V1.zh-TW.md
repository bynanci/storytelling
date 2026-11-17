<!-- SPDX-License-Identifier: MIT -->
# 唯讀 Rust Consumer SDK V1

[English／完整 API 與界線](SDK_CONSUMER_V1.md) · [ADR 0013](adr/0013-read-only-consumer-facade-v1.md) · [G2 決定](validation/m4-01-sdk-g2-decision.md) · [驗證紀錄](validation/m4-01-readonly-sdk.md)

已採納候選以既有 engine crate 的 `storytelling_engine::consumer_v1` 提供八個唯讀入口。本次是 Draft 實作，套件仍為 `0.1.0-dev.0`、不可發布；不代表穩定 SDK、正式執行權限或 Host 已驗收。

由 repo 根目錄，以 Rust 1.89.0 執行獨立範例：

```sh
cargo run --manifest-path examples/consumer-sdk-v1/Cargo.toml --locked -- \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

範例自己的 `main` 明確讀取檔案及輸出既有 coverage 結果；SDK 只收 bytes，不開啟路徑。預期第二列保持 `unresolved`、診斷為 `video_coverage.unresolved_required`、`ready_for_planning:false`。這是宣告仍缺素材，不是媒體或編輯器驗證。範例是獨立、禁止發布的測試 consumer，不加入四個 runtime workspace members。

| 入口 | 輸入／結果 |
| --- | --- |
| `validate_plan` | plan bytes → `PlanValidationV1` |
| `render_narrative` | narrative bytes → `NarrativeProjectionV1` |
| `track_revisions` | ledger bytes → `NarrativeRevisionReportV1` |
| `query_impact` | ledger bytes、change ID → `NarrativeRevisionEventResultV1` |
| `video_coverage` | coverage bytes → `NarrativeVideoCoverageResultV1` |
| `plan_story` | planner request bytes → `PlannedStoryV1` |
| `prepare_story_review` | proposal response、snapshot、context bytes → opaque `PreparedStoryReviewV1` |
| `check_story_review` | bundle、proposal response、snapshot、context bytes → `ReviewFreshnessReportV1` |

每個入口直接呼叫一次既有 backend，回傳 `Result<既有結果, SdkErrorV1>`。保留既有結果型別及必要的巢狀型別名稱；不新增 validator、公開 JSON Schema 或可信物件 constructor。Narrative 投影維持 typed result，不新增 JSON serializer；版本查詢只處理直接引用，宣告的作者決定也不驗證真人身分。

Review 吃的是含 `ok`／`planning` 的既有回應 bytes，不是 request 或單獨的 `PlannedStoryV1`。只建立／序列化一次並保留原 bytes，後續兩條路徑共用；不可反覆解析再編碼已選定提案。[凍結 fixture](../tests/fixtures/consumer-sdk-v1/README.md)示範這點。`record()`、`compiled_otio()`、`bundle_json()` 皆唯讀；prepared handle 不能從 JSON 或呼叫者提供的 record／artifact 偽造。CLI 比 backend payload 多一個 LF，不能任意 trim 或重編碼身分資料。

合法缺件仍為 `Ok` 且 planning=false；合法 stale review 仍是 `Ok` 且 current=false，CLI 既有 stale exit 1 不變。格式壞掉或來源雜湊不符則可能是 `Err`，不能混為 stale。

錯誤提供 `operation()`、原始 `code()`、可為 None 的 `path()`，以及可失敗的 `source_details_json()`。細節直接序列化原 error，保留 coverage 的 null path、narrative 的省略空 relation_ids、planner conflicts 及 OTIO losses；不是 CLI 的外層 envelope。編碼失敗以 `sdk.error_details_encoding_failed` 回報，原 domain error 仍保留。錯誤 wrapper 不提供公開 constructor 或 Deserialize；測試用 serializer 可觸發編碼失敗，正式程式沒有故障注入開關。

逐操作 byte／深度／集合／時間與錯誤優先順序不變。一般 input／snapshot 上限 1,048,576 bytes，proposal 4,194,336、context 65,536、bundle 4,292,672；change ID 沿用 128-byte 規則。呼叫是同步純函式、結果自有資料，支援型別的 Send／Sync 與 caller threads 重播有測試；呼叫者自行限制並行及記憶體。沒有 cache、背景工作或取消 token，drop 結果不會 Undo 編輯器。

預設及 all-features 均沒有 execute／approve／dispatch／retry／recover／journal-write／任意 shell／path／Host callback。#103–#106 的授權、執行與復原門檻保留。簽章數學、當前授權、編譯綁定、實際來源 bytes、畫面人工確認及 Host readback 仍是不同證據。

舊 root APIs、CLI stdout／exit、wire profiles、rational clocks、OTIO 與 P1/P2 digest 不變。可逐項遷移 import；破壞 API／型別／ownership／error accessor 必須新 facade major、遷移審查及移除前的相容期間。回退只移除新 module／範例與 consumer imports，無須改寫專案、Catalog 或媒體。#108 的 transport、實際 consumer 操作與 G5 發布各自決定。
