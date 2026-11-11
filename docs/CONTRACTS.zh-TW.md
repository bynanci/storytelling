# 命令列介面與文件契約

<a id="cli-and-document-contracts"></a>

[English](CONTRACTS.md) · 繁體中文

先按工作選擇命令，再準備該命令接受的文件。下表的大小單位均為位元組。
各規格定義傳輸欄位與資料上限。[命令列操作範例](CLI_GUIDE.zh-TW.md)

| 命令 | 必要輸入與大小上限 | 成功時的標準輸出 | 規格 |
| --- | --- | --- | --- |
| `validate` | M0 計畫 JSON，1,048,576 | `{"ok":true,"validation":...}` | [M0 計畫](#m0-plan-contract) |
| `import-openfilm` | 有限剪輯快照 JSON，1,048,576 | `{"ok":true,"import":...}` | [快照匯入](migration/FINITE_V1.md) |
| `export-otio` | 同一種剪輯快照，1,048,576 | 原生 OTIO `Timeline.1` JSON | [有限 OTIO](migration/FINITE_OTIO_V1.md) |
| `audit-brief` | StoryBrief v1 JSON，1,048,576 | `{"ok":true,"audit":...}` | [創作需求檢查](migration/STORY_BRIEF_V1.md) |
| `plan-story` | 含固定時間窗的規劃輸入 JSON，1,048,576 | `{"ok":true,"planning":...}` | [固定時間窗規劃器](migration/FIXED_WINDOW_PLANNER_V1.md) |
| `render-narrative` | NarrativeDocumentV1 JSON，1,048,576 | 內容固定、可直接閱讀的 UTF-8 文字。錯誤時輸出一行 JSON。 | [敘事文件](narrative/NARRATIVE_DOCUMENT_V1.md) |
| `render-cue-sheet` | NarrativeDocumentV1 與 NarrativeCueSheetV1 JSON，各 1,048,576 | 可重播提示表文字。錯誤時輸出一行 JSON。 | [提示表規格](narrative/NARRATIVE_CUE_SHEET_V1.zh-TW.md) |
| `render-handoff` | NarrativeHandoffManifestV1 JSON，1,048,576 | 可重播的來源清單、部分影響與審閱狀態；錯誤為一行 JSON。 | [交接清單](narrative/NARRATIVE_HANDOFF_MANIFEST_V1.zh-TW.md) |
| `track-narrative-revisions` | NarrativeRevisionLedgerV1 修訂紀錄 JSON，1,048,576 | 可重跑的來源與變體歷史、建議、作者選擇及目前的審閱狀態 | [敘事修訂紀錄](narrative/NARRATIVE_REVISION_LEDGER_V1.md) |
| `query-narrative-impact` | 相同的修訂紀錄，另附一個來源修訂事件 ID | 依直接參照計算該次修訂所影響的變體 | [敘事修訂紀錄](narrative/NARRATIVE_REVISION_LEDGER_V1.md) |
| `check-narrative-style` | NarrativeDocumentV1 與 NarrativeStyleProfileV1 JSON，各 1,048,576 位元組 | `{"ok":true,"style_check":...}`；錯誤時退出 1。 | [風格 profile V1（英文）](narrative/EDITORIAL_STYLE_PROFILE_V1.md) |
| `audit-copy-claims` | CommercialCopyDocumentV1 與 NarrativeRevisionLedgerV1 JSON，各 1,048,576 位元組 | `{"ok":true,"commercial_copy":...}`；錯誤時退出 1。 | [文案引用契約](narrative/COMMERCIAL_COPY_V1.md) |
| `verify-source` | 提供的位元組，16,777,216。另須指定預期的 SHA-256。 | `{"ok":true,"verification":...}` | [緩衝區驗證](migration/FILMCRAFT_25_V1.md#native-supplied-buffer-contract) |
| `prepare-story-review` | 規劃器輸出 4,194,336、剪輯快照檔 1,048,576、情境檔 65,536 | `{ok:true,review:...,otio:Timeline}`，含實際 OTIO 產物 | [所選提案的審閱](migration/PLANNER_OTIO_REVIEW_V1.md) |
| `export-story-otio` | 與上一列相同的三項輸入 | 直接輸出相同的原生 OTIO | [所選提案的審閱](migration/PLANNER_OTIO_REVIEW_V1.md) |
| `check-story-review` | 審閱封裝檔 4,292,672，另附目前的提案、快照與情境 | `{ok:true,freshness:...}`，內容過期時結束碼為 1 | [所選提案的審閱](migration/PLANNER_OTIO_REVIEW_V1.md) |

<a id="read-input-and-interpret-the-result"></a>

### 讀取輸入與判讀結果

單一輸入命令讀取指定的一般檔案。以 `-` 代替檔名可讀取標準輸入。
`verify-source` 還需要預期摘要參數。所選提案的三個審閱命令只允許從標準輸入讀取提案。
快照、情境與封裝檔均須明確指定一般檔案。

`export-otio` 在輸入參數之後接受明確的損失選項。成功時直接輸出 OTIO，沒有 `ok` 外層。
格式有效的創作需求若有強制限制衝突，結束碼為 1，回應為 `{"ok":false,"audit":...}`。
輸入格式錯誤時，回應含 `error` 物件。請依各命令的錯誤型別解析結果。

`plan-story` 的輸入包含嚴格格式的創作需求（Brief）、宣告的候選時間窗及分數。
呼叫端另須提供時長與鏡頭數的強制限制。規劃成功時，提案報告含有符合 M0 的 `plan`。
規劃失敗時，`error` 包含有型別的 `conflicts`。此時沒有計畫。

排序與固定時間窗只依宣告資料判定。一般可行性、媒體品質及宿主執行各需獨立證據。
使用 M0 驗證器時，須取出 `planning` 內的計畫。直接輸入整個 `planning` 回應無效。
`export-otio` 只接受指定規格的剪輯快照。

追蹤敘事修訂時，以檔名或標準輸入提供只新增事件的修訂紀錄。
`track-narrative-revisions <FILE|->` 列出事件重播後的歷史及狀態。
`query-narrative-impact <FILE|-> <CHANGE_ID>` 只接受來源修訂事件的 ID。
引擎依各變體的直接 `source_refs` 計算影響範圍。呼叫端不能傳入受影響清單。
此版本不沿 `based_on` 推導間接影響。若任一變體仍待審閱，引擎會拒絕新的來源修訂。
作者決定與代理建議各用獨立事件。命令記錄所宣告的選擇，不驗證作者身分。

\`check-narrative-style <DOCUMENT_FILE|-> <PROFILE_FILE|->\` 依綁定的文件 revision 檢查口述變體中宣告的 beat 順序。成功時輸出單行 JSON；錯誤時回報診斷並退出 1。此指令不判斷創作品質或作者核准。
`audit-copy-claims` 成功時輸出一行 JSON。`reference_status` 只說明來源 ID/revision 狀態，不判斷語意佐證。

`render-cue-sheet <DOCUMENT_FILE|-> <CUE_SHEET_FILE|->` 接受一份敘事文件及獨立、封閉的提示表，驗證宣告的識別碼、beat 與素材引用、時間及互斥聲軌衝突，然後輸出可重播的可讀文字。最多一項輸入可使用標準輸入。指令不讀取音檔、不錄音、播放或混音，也不修改 `NarrativeDocumentV1`。

| 通用結束碼 | 意義 |
| --- | --- |
| 0 | 評估成功。 |
| 1 | 領域規則拒絕輸入，或檢查發現衝突。 |
| 2 | 命令或參數用法錯誤。 |
| 3 | 輸入或輸出失敗。 |

JSON 回應只佔一行。說明與版本資訊使用純文字。
下列敘事命令成功時輸出可重跑的 UTF-8 純文字：

- `render-narrative`
- `render-handoff`
- `track-narrative-revisions`
- `query-narrative-impact`

若寫入標準輸出失敗，程式無法保證回應完整。
各規格分別限制 JSON 集合、巢狀深度、診斷及輸出大小。
來源位元組的上限不影響 JSON 上限。此處的命令都不執行剪輯器或授予核准。

<a id="m0-plan-contract"></a>

## M0 計畫契約

M1-02 命令 `export-otio` 依[有限 OTIO 契約](migration/FINITE_OTIO_V1.md)輸出原生 OTIO。
呼叫端明確指定可接受的轉換損失。
M1-01 命令 `import-openfilm <FILE|->` 依[有限快照契約](migration/FINITE_V1.md)匯入資料。
其 `plan` 輸出須符合 M0 契約及其輸入大小上限。產生的畫面映射不可執行。

`PlanDocumentV1` 是實驗性的有限資料結構。完整故事 SDK 與剪輯器轉接器輸入另有範圍。
[最小範例](../examples/minimal-plan.json) · [傳輸型別定義](../crates/storytelling-core/src/wire.rs)

<a id="document"></a>

<a id="document-structure"></a>

### 文件結構

| 物件 | 必填欄位 |
| --- | --- |
| 文件根物件 | `schema_version`（整數 1）、`plan_id`、`assets`、`shots` |
| 每份素材 | `id`、`content_hash`、`duration` |
| 每個依序排列的鏡頭 | `id`、`asset_id`、`source_range: {start,end}` |

文件不接受選用的擴充物件。驗證器拒絕未知欄位、重複欄位及錯誤的物件結構。

ID 須符合 `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`。素材 ID 在素材集合內須唯一。
鏡頭 ID 在鏡頭集合內也須唯一。多個鏡頭可以參照同一份素材。
雜湊須為 64 個小寫十六進位字元。此項檢查只驗證格式。

時間單位為秒。格式為 `{numerator: string, denominator: string}`。
分子與分母均以標準十進位無號 `u64` 字串表示。
分母須大於零，且 `gcd(n,d) = 1`。零須寫作 `0/1`，`2/2` 與 `00/1` 均無效。
字串表示法可避免大整數經 JavaScript 傳遞時失去精度。

來源區間採左閉右開，須符合 `0 <= start < end <= duration`。時長須為正值。
系統不推斷來源呈現時間戳記（PTS）、幀率（FPS）或音訊取樣率的轉換。

輸入上限為 1,048,576 位元組。素材數為 1 至 1,024，鏡頭數為 1 至 4,096。
`serde_json` 使用預設的遞迴深度限制。
M0 不接受任意檔案、網址、腳本或服務供應者欄位。
它不開啟宣告的來源媒體，也未保證支援特定時間軸或媒體格式。

<a id="validate-cli"></a>

<a id="run-the-validator"></a>

### 執行驗證

```text
storytelling validate <FILE|->
storytelling --help
storytelling --version
```

呼叫端須指定本機 JSON 檔，或以 `-` 讀取標準輸入。檔案符號連結可指向一般檔案。
此命令只在本機唯讀處理輸入。它沒有提供沙箱或伺服器端路徑代理。
標準輸入會等候檔案結尾（EOF）。`validate` 輸出一行 JSON，沒有夾雜紀錄訊息。
說明與版本資訊則為純文字。

成功範例如下。

```json
{"ok":true,"validation":{"schema_version":1,"plan_id":"...","assets":1,"shots":2,"scope":"declared-metadata-only"}}
```

來源區間超出上限時，回應如下。

```json
{"ok":false,"error":{"code":"plan.source_out_of_bounds","path":"$.shots[0].source_range.end"}}
```

錯誤回應不顯示來源內容或本機路徑。標準輸出寫入失敗時，程式以結束碼 3 離開。
此時無法保證產生 JSON 回應。

| 結束碼 | `validate` 的結果 |
| --- | --- |
| 0 | 宣告文件通過驗證，或已顯示說明或版本。 |
| 1 | JSON、版本、中繼資料、時間、參照、範圍或大小無效。 |
| 2 | CLI 命令或參數無效。 |
| 3 | 輸入或輸出失敗。 |

呼叫端請依穩定的錯誤碼分流。診斷路徑、自然語言訊息與行列位置仍可能調整。

- 輸入與用法：`input.invalid_json`、`input.too_large`、`input.io`、`cli.usage`。
- 版本與格式：`plan.unsupported_version`、`plan.invalid_id`、`plan.invalid_hash`。
- 時間與範圍：`plan.invalid_time`、`plan.invalid_range`、`plan.source_out_of_bounds`。
- 參照與集合：`plan.duplicate_id`、`plan.unknown_asset`、`plan.empty_collection`、`plan.too_many_items`。

<a id="m3-02-sqlite-operation-journal"></a>

## M3-02 SQLite 操作日誌

Rust 儲存轉接器提供 API。目前沒有 CLI 命令或宿主用戶端。
在未來的宿主 API 呼叫前，呼叫端須先保存不可變的操作身分與已編號的 `dispatching` 嘗試。
結果未知時，操作保持受阻，直到目前持有操作權的執行者記錄讀回結果。
只有明確觀察到 `NotApplied`，才能再次嘗試。

日誌記錄呼叫端的觀察與證據摘要。驗證宿主狀態、簽署者權限、核准及產物來源均另需證據。
[M3-02 日誌契約](migration/M3-02-OPERATION-JOURNAL.md)

<a id="m3-01-p1-accepted-bounded-review-binding"></a>
<a id="m3-01p1-accepted-bounded-review-binding"></a>

<a id="m3-01p1-review-binding"></a>

## M3-01/P1 有限範圍的審閱綁定

[#27](https://github.com/bynanci/storytelling/issues/27) 的首個有限規格已通過 G2 簽署驗收。
需求 M3-RB01 至 M3-RB07 僅在此規格內凍結。完整 M3-01、PR #28 候選方案與其他 M3 需求各自驗收。
[G2 驗收紀錄](migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md)

| 命令與 API | 輸入 | 行為 |
| --- | --- | --- |
| `prepare-review <COMPOSITION\|-> <CONTEXT_FILE>` ／ `prepare_review` | 剪輯快照上限 1 MiB。情境檔嚴格解析，上限 65,536 位元組。 | 讀取同一段快照位元組，依宣告的損失選擇匯入並編譯 OTIO。回傳仍須驗證的可攜審閱紀錄。 |
| `check-review <REVIEW_RECORD> <COMPOSITION\|-> <CONTEXT_FILE>` ／ `check_review` | 紀錄上限 98,304 位元組。目前快照上限 1 MiB。情境檔上限 65,536 位元組。 | 檢查不受信任的紀錄。重新匯入並編譯目前的位元組。排序並列出過期欄位。吻合只代表宣告內容一致。 |
| `BoundReviewV1` | 只有 `prepare_review` 能建立。 | 程序內的私有結果含可攜的資料傳輸物件（DTO）與確切的 OTIO 位元組。沒有 `Serialize`、`Deserialize`、`Default` 或公開且免驗證的建構函式。 |

<a id="context-fields-and-limits"></a>

### 情境欄位與限制

- 情境 v1 的欄位固定為 `schema_version`、`project_id`、`target_revision`、`host`、`loss_choices`。
- `host` 含 `id`、`version`、`profile_id`、`binary_sha256` 與 `declared_capabilities`。
- ID 長度為 1 至 128 個 ASCII 位元組。格式須符合 `[A-Za-z0-9][A-Za-z0-9._-]*`。
- 雜湊須為 64 個小寫十六進位字元。能力清單上限為 64 個 ID。ID 須依 ASCII 排序且不得重複。
- 損失選項只接受 `metadata_only_locks` 與 `picture_only` 的子集。選項須排序且不得重複。
- 未知、重複、缺少、值為 `null` 或格式錯誤的欄位均無效。
- 審閱 JSON 的巢狀深度上限為 64。OTIO 輸出上限為 4 MiB。

<a id="identity-digests"></a>

### 身分摘要的範圍

身分 v1 包含下列資料。

1. 原始快照位元組的 `composition_sha256`。
2. 有型別的 M0 計畫、OTIO 文件及完整 OTIO 損失報告。三者分開序列化，各自在獨立且有版本的用途域計算 SHA-256。
3. 引擎 crate／編譯器版本及選定的 OTIO 規格。
4. 完整情境摘要，以及各自在獨立用途域計算的情境欄位摘要。

最終綁定摘要涵蓋資料結構版本與完整身分。用途域為 `storytelling.finite-otio-review-binding.v1`。
承載資料長度以大端序 `u64` 表示。

快照摘要使用呼叫端提供的確切位元組。即使解析結果相同，尾端新增換行仍會使快照過期。
該摘要未正規化來源。情境陣列須採標準順序。其他有語意的陣列沿用來源或編譯器順序。
此處使用該版本 Rust DTO 的 JSON 編碼。通用 JSON 正規化方案（JCS）不適用。

<a id="order-of-checks"></a>

### 驗證順序

多項輸入同時無效時，API 與 CLI 的檢查順序各有測試。

- `prepare_review` API 先驗證有型別的情境，再匯入快照。
- `check_review` 先依序檢查紀錄、快照、情境的位元組上限。接著驗證紀錄、解析與驗證情境，最後重新匯入並編譯快照。
- CLI 先檢查參數格式，再依命令順序讀取各檔案並檢查大小。準備時依序讀取快照與情境（`composition, context`）。檢查時依序讀取紀錄、快照與情境（`record, composition, context`）。
- CLI 讀完所有輸入後才套用引擎的驗證順序。後一份檔案的 I/O 錯誤可能先於前一份輸入的 JSON 錯誤回傳。

<a id="review-results"></a>

### 審閱結果

可攜審閱紀錄是未受信任的資料物件。此紀錄無法授權執行。
`scope` 須為 `declared-review-binding-only`。
`executable`、`approval_granted`、`source_verified` 與 `host_verified` 均須為 `false`。

`check_review` 先重算綁定摘要，再比對目前輸入。過期時，檢查本身仍可成功回報。
此時回應為 `ok:true`、`current:false`，結束碼為 0。
紀錄格式錯誤或宣告遭偽造時，回傳領域錯誤，結束碼為 1。

過期欄位的穩定識別名稱如下。

- `composition`、`plan`、`artifact`、`report`、`compiler_version`、`profile_id`、`context`。
- `context.project_id`、`context.target_revision`、`context.host.id`、`context.host.version`。
- `context.host.profile_id`、`context.host.binary_sha256`、`context.host.declared_capabilities`、`context.loss_choices`。

<a id="evidence-and-limits"></a>

### 驗證證據與限制

1. 數學上有效的簽章只證明指定公鑰能驗證確切的簽署訊息。可攜的金鑰或簽章無法證明該金鑰可信或目前獲准使用。
2. 簽署者授權需要獨立管理的信任根或金鑰登錄，也需要授權與撤銷政策。本綁定規格沒有這些機制，亦不檢查簽章。
3. 綁定實際產物時，實作從同一份不可變輸入重建結果，並比對確切的編譯器輸出。此規格僅涵蓋一段有限快照位元組與其 OTIO 匯出。快照是否來自已驗證的建置、宿主、可信的產生端或規劃提案，仍須獨立證據。

宿主執行檔指紋與能力清單只記錄宣告值。本規格未提供下列功能。

- 核准防重播、請求消耗紀錄、可信時間、到期與撤銷狀態。
- 檔案身分驗證，或防範檢查與使用之間檔案變動的保護（TOCTOU）。
- 宿主觀察、宿主執行或發布。

M3-02 以操作日誌保存有限範圍的冪等狀態，並阻止過期的日誌更新。
此機制無法去除重複的外部宿主呼叫，也不會停止這些呼叫。
可信授權與宿主執行仍屬其他 M3 工作。
[命令列契約](CLI_GUIDE.zh-TW.md#7-建立與檢查有限審核識別) · [M3-01/P1 驗收紀錄](migration/records/M3-01-P1.md)

<a id="future-public-workflow-design-only"></a>
<a id="merged-pr-28-approval-identity-v1-candidate-not-accepted"></a>

<a id="pr-28-approval-identity-candidate"></a>

## PR #28 核准身分 v1 候選契約

這項核心契約提供限定範圍的密碼學身分與驗證器。PR #28 已合併，仍待獨立 G2 決定。
M3-01/P1 另有已驗收的有限規格。[P1 範圍](migration/M3-01-NEXT.md) · [核准身分契約](migration/APPROVAL_IDENTITY_V1.md)

此 API 沒有對應的 CLI 命令。它不執行或修改剪輯器專案。
呼叫端提供提案與產物的確切位元組，以及既有的來源憑證 `VerifiedBufferV1`。
API 建立私有 `BoundApprovalIdentityV1`，將這些輸入綁定。
此綁定無法證明編譯過程或各輸入都來自同一份快照。

簽署內容的每個身分欄位須與該憑證吻合。
呼叫端另須從自己的可信來源取得 `ExpectedApproverKeyV1`，並明確傳入。
只有金鑰綁定也吻合時，驗證器才接受簽署內容。

<a id="signed-bytes-and-encoding"></a>

### 簽署資料與編碼

簽署者先接收 `storytelling.approval.v1\0`，後接固定格式的精簡 `ApprovalPayloadV1` JSON 位元組。
專案固定版本的 `serde_json` 依 Rust 結構的欄位宣告順序序列化。
資料不包含映射表或浮點欄位。來源 ID、宿主能力及接受損失的資料列均須嚴格排序且不得重複。

`SignedApprovalV1` 的 JSON 上限為 1 MiB，拒絕未知欄位與重複欄位。
分離式簽章與公鑰都以小寫十六進位編碼。簽章為 64 位元組，公鑰為 32 位元組。
核心只提供驗證功能。

<a id="what-verification-establishes"></a>

### 驗證結果能證明什麼

`VerifiedApprovalV1` 的欄位為私有，僅提供唯讀存取方法。該型別未實作 Serde 特徵。
驗證結果只證明所提供的金鑰簽署了這份確切的資料。下列事項仍需其他證據。

- 呼叫端的金鑰登錄可信，且具名主體獲准操作。
- 標為 `observed` 的宿主聲明符合事實。
- 該核准仍有效，未遭重播或撤銷，也沒有過期。
- 宿主已執行該次操作。

上述證據分屬可信介面、M3-02 與 M3-03 的後續工作。
[PR #28 候選驗證紀錄](migration/records/M3-01-approval-candidate.md)

