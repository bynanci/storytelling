<!-- SPDX-License-Identifier: MIT -->
# NarrativeHandoffManifestV1 規格

[English](NARRATIVE_HANDOFF_MANIFEST_V1.md) · 繁體中文

交接清單用來辨識來源修訂後，文字、圖像與聲音草稿中哪些部分須改。
[Schema](../../contracts/narrative-handoff-manifest-v1.schema.json) 是獨立且封閉的 V1 文件。
它內嵌既有的 [NarrativeRevisionLedgerV1](NARRATIVE_REVISION_LEDGER_V1.md)，
不擴充該修訂簿或已採納、封閉的 `NarrativeDocumentV1`。

## 執行範例

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/before-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/after-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-handoff tests/fixtures/narrative-handoff-v1/descriptions-only-v1.json
```

修傘日期由週六改成週日。新命令執行前已將預期集合記入
[expected-impact.json](../../tests/fixtures/narrative-handoff-v1/expected-impact.json)：

| 動作 | 部分 |
| --- | --- |
| `UPDATE` | `invite/text`、`poster/date`、`oral-info/date`、`oral-poetic/date` |
| `REUSE` | `poster/layout`、`oral-poetic/taps`、`original-wav/audio` |

`UPDATE` 列出指定來源修訂事件當時的影響，不表示已審閱的草稿仍待修改。
`status=needs_review` 表示草稿待審；`status=current` 表示已有相符的宣告審閱。
修改後的範例保留相同影響集合，並提供週日用語。
[N-11 驗證紀錄](../validation/n11-handoff.md)列出實際輸出、退出碼、前後稿、重播與來源位元組核對。

## 來源與產物

根物件包含 `schema_version`、`manifest_id`、`ledger`、`change_id`、`sources`
及 `artifacts`。`change_id` 必須指向重播後修訂簿中的來源修訂事件。
引擎沿用 N-04 的直接引用影響結果；呼叫端不提供受影響變體清單。

每項來源描述指向穩定 `unit_id` 與修訂簿中的**目前來源 revision**。
同一單元只能列一項描述。`representation` 標籤區分下列輸入：

| 種類 | 欄位 | 宣告意義 |
| --- | --- | --- |
| `file` | `media_kind`、`relative_path`、`sha256` | 宣告的本機文字、圖像或聲音檔；摘要須有 64 個小寫十六進位字元。 |
| `description` | `media_kind`、`text` | 提供的素材圖說或描述，不表示原檔已交付。 |
| `transcript` | `text` | 提供的文字標註；工具沒有檢視底層錄音。 |
| `creative_proposal` | `variant_id`、`text` | 修訂簿 origin 為 `creative_proposal`，且限於該變體的提案。 |
| `missing` | `media_kind`、`reason` | 明列缺件，仍可交接可用文字。 |

檔案路徑最多 1,024 個 Unicode scalar values。拒絕絕對路徑、空段、點或雙點段、
反斜線、冒號及控制字元。引擎不開啟路徑或計算摘要。
需要驗證實際位元組時，另以 `verify-source` CLI 核對。

每項產物指定媒介、一個既有變體 ID 與各部分的來源 ID。
引擎從實際修訂簿變體快照取得草稿 revision，呼叫端不能覆寫。
同一產物的部分 ID 不得重複；各部分來源 ID 的聯集必須涵蓋所綁變體的**全部直接引用**。
同一來源可供不同部分共用。遺漏引用時拒絕 `handoff.incomplete_mapping`，不會悄悄漏報影響。

提案範圍依整個綁定快照檢查，包括部分對照表省略的引用。
海報專用提案不能進入口述版。修訂簿 origin 為創作提案的來源必須使用提案表示法，
改標成其他種類也不能繞過範圍限制。

待審草稿可合理保留 `S1@1`，同時目前來源已是 `S1@2`。
輸出分列草稿與目前來源引用。若**來源描述**仍以 `S1@1` 冒充目前版本，
則拒絕 `handoff.stale_revision`。歷史快照仍由原本的 N-04 修訂簿保存。

## 限制、診斷與證據

輸入最多 1 MiB，沿用敘事讀取器的 32 層 JSON 深度限制與重複 key 預檢。
所有物件封閉，版本必須是整數 `1`。ID 格式為 `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`。
來源描述 1–256 項；產物 1–64 項；各產物部分與各部分來源 ID 皆為 1–64 項。
文字長度為 1–8,192 個 Unicode scalar values。內嵌修訂簿沿用全部原有限制。

命令接受一個檔案或標準輸入 `-`。成功時輸出可重播的 UTF-8 可讀文字，退出碼 0。
資料不合法時輸出一行 JSON 診斷，退出碼 1。用法錯誤為 2，輸入輸出錯誤為 3。
診斷包含 `handoff.dangling_reference`、`handoff.stale_revision`、`handoff.variant_scope`、
`handoff.incomplete_mapping`、`handoff.duplicate_field`、`handoff.unknown_field` 及
`handoff.unsupported_version`。內嵌修訂簿錯誤保留既有代碼。

引擎檢查宣告引用與部分對照，不推斷文案是否含日期、不驗明作者身分、不檢視圖像位置、
不轉錄聲音或產生海報，也不追查 `based_on` 的間接影響。
作者仍須檢視語意差異。CLI 只讀清單並寫標準輸出，不改寫來源、草稿或媒體。
