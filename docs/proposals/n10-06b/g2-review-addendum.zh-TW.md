<!-- SPDX-License-Identifier: MIT -->
# N10-06B · G2 補充提案

[English](g2-review-addendum.md) · 繁體中文 · [原提案](README.zh-TW.md) · [前瞻案例](g2-review-cases.json)

狀態為提案，待維護者採納。核對基準 `04d2ac6f5cdb43d16cc7e4c94a2ad14fad49974d`。這份補充把[圓桌建議](../../validation/g2-roundtable-2026-11-14.zh-TW.md)收斂成 [#129](https://github.com/bynanci/storytelling/issues/129) 可審查的候選。原案例檔保留 B01–B11，B12 起補上缺少的案例。

## 入口與 Rust bridge

- `storytelling preflight-video-coverage-input <FILE|-> [--lang zh-TW|en]` 使用既有的選用 feature `draft-video-input`。CLI 已把同名 feature 傳給 engine。語言旗標放在輸入之後，省略時採 `zh-TW`。無效語言、重複旗標或多餘參數回傳 usage exit 2。
- Engine 在該 feature 下提供 `#[doc(hidden)] pub fn render_draft_video_preflight(bytes: &[u8], english: bool) -> (String, bool)`，供 CLI 跨 crate 呼叫。第一個值是完整 UTF-8 文字，換行採 LF。第二個值代表宣告是否有效，完整或含 unresolved 的有效宣告都回 `true`。CLI 將 `true` 對應 exit 0，`false` 對應 exit 1，並負責讀取及使用錯誤。stdout 寫入成功才採用 bridge 對應的 exit，I/O 失敗回傳 exit 3。Bridge 不屬於穩定 SDK，列與提示型別維持私有。
- 既有 `normalize_draft_video_coverage` 接收完整原始 bytes，回傳自有 JSON 成功結果，或一份含 `stage`、`code`、`path` 的錯誤。成功結果含正規化宣告 JSON 及 V1 的 `ready_for_planning`、`diagnostics`。拒絕只提供首錯，沒有部分換算。B 呼叫一次既有入口，以該結果決定權威區塊及宣告是否有效，再輸出文字預覽。作者可透過 A 的 `--emit-v1` 明確取得宣告 JSON。[Engine 入口](../../../crates/storytelling-engine/src/draft_video_input.rs) · [CLI feature 與分派](../../../crates/storytelling-cli/src/main.rs)
- 取得權威結果後，提示沿用同一套嚴格原始 JSON 解析及精確秒數、FPS、影格函式。原始 bytes 超限、UTF-8、語法、重複鍵或深度遭拒時，省略所有列及提示。正規化後的 V1 大小或深度遭拒時，同樣省略投影並保留權威結果。可解讀的外層只含 `draft_version` 與 `coverage`，值須為已採用的版本及 coverage 物件。外層符合時，即使換算或 V1 遭拒，仍可顯示能安全解讀的欄位，且須將提示換算標為暫定。影格時間為 VFR 或未知時，不顯示換算端點。既有限制為原始輸入 1 MiB，draft 深度 34，V1 深度 33。[Draft 文法](../../adr/0012-draft-exact-video-range-input.md) · [V1 權威驗證](../../../crates/storytelling-engine/src/narrative_video_coverage.rs)

## 引用及提示順序

保留每個選定引用的原始 index 及順序。可顯示的引用是只含 `unit_id` 及 `revision` 的物件。ID 不可留空，最多 128 個 ASCII bytes，首字元為英數，其後只接受 ASCII 英數及 `.`、`_`、`-`。Revision 必須為正 `u32`。這是顯示時的解讀規則。存在性、變體範圍與版本新鮮度由 V1 判定。無法解讀的引用只顯示固定 index 及 `單元引用格式無效` / `Invalid unit reference`，省略來源欄位且不做 mapping 配對。Mapping 的 `unit_ref` 遵循同一規則。

配對保留完整 `(unit_id, revision)`。依輸入順序建立 mapping 索引並列出所有相符的 mapping indices。重複選定引用仍各有一列，後續相同組別指到第一個已列出完整 indices 的 selection index。每份 mapping 細節只呈現一次，未配對的 mapping 依原 index 另列。這可保留完整配對資訊，避免大量重複值展開相同索引與來源欄位。格式不符或版本過期的 mapping 仍保留原 index。重複 mapping 與互相衝突的狀態都保留供檢查。可解讀的選定引用若找不到精確配對，才顯示缺列提示。

非權威提示依 mapping index 排序。同一列採固定欄位順序 `unit_ref`、`status`、`asset_ref`、`source_range.start`、`source_range.end`、`reason`。適用整段區間的 `unit`、`timing`、`fps` 提示放在端點之前，沿用 A 的先後順序。影格的輸入位置採 `start_frame`、`end_frame`，V1 位置採 `start`、`end`。所有 mapping 提示之後再依 selection index 顯示缺列提示。未知鍵與值不顯示，權威診斷保留原代碼、位置與順序。

只有素材目錄內恰有一筆符合 `asset_id@revision`，且片長是正值的 canonical V1 有理數，才能顯示宣告片長。重複的完全相符列顯示 `無法確認宣告片長` / `Declared duration unavailable`。標示所有片長資料都來自宣告。

開始投影前，選取和 mapping 各限 512 列，素材目錄限 1,024 列。任一超限便省略整個提示區，顯示 `資料列過多，已省略預覽` / `Preview omitted: too many rows`。提示文字含轉義與標籤最多 4 MiB UTF-8 bytes，附加前計入；若會超限，捨棄提示區並顯示 `超過顯示上限，已省略預覽` / `Preview omitted: display limit`。兩種省略都保留權威區塊及原有 exit。

## 終端文字

以引號包住顯示的輸入字串，轉義引號和反斜線。將 C0（`U+0000–U+001F`）、DEL（`U+007F`）、C1（`U+0080–U+009F`）以及雙向文字控制字元（`U+061C`、`U+200E–U+200F`、`U+202A–U+202E`、`U+2066–U+2069`）統一呈現為可見的大寫十六進位 `\uXXXX`。例如理由中的 CR 顯示為 `\u000D`，C1 控制字元顯示為 `\u009B`，雙向覆寫顯示為 `\u202E`。標籤、縮排和換行由呈現器產生。診斷標籤及 JSONPath 不複製終端控制字元、解析器原文、輸入檔路徑或未知資料。原始 bytes 和 A 解讀後的理由照原樣保留。

## 決定與驗證

請一併審查命令、feature、語言、exits、bridge 與其有效值的含義，以及上方的投影規則與案例。[新增案例檔](g2-review-cases.json)的 B 欄位都是預期結果，A 的權威結果欄則對照附記的基準重播。待維護者明確採納後，G3 用 B 的實際命令記錄 RED／GREEN。目前狀態見 [Issue #129](https://github.com/bynanci/storytelling/issues/129)。

### 基準權威結果重播

2026-11-20 在 Linux 以 Rust 1.89.0 從上述提交全新建置，命令為 `cargo build --release --offline --locked -p storytelling-cli --features draft-video-input --target-dir <new-target>`。執行檔 SHA-256 為 `4fb7a412728f036f6cd36977e1b3e02ab9eb8d8ac92617ee3d662b32c0248c3d`。

重播時，每個案例都從一份新的基準 fixture 套用欄位修改，或直接使用案例的 `raw_input`。以 stdin 交給 `storytelling draft-video-coverage-input -`，核對拒絕的 exit、stage、code、path，或成功的 readiness 與指定診斷。再從同樣的輸入執行 `storytelling draft-video-coverage-input - --emit-v1`。成功時將輸出的宣告送入 `storytelling render-video-coverage -`，對照 A 的 `validation`。拒絕時核對兩次 A 呼叫的錯誤 stdout 與 exit 完全一致。

本機重播共符合 21 個權威結果，含原提案 11 個及補充 10 個。16 個拒絕案例加上 `--emit-v1` 後仍回傳完全相同的錯誤 stdout。五個有效案例輸出的 JSON 交由 V1 直接驗證，結果一致。兩個理由案例也保留解碼後的原字串。B18 的 103,977 bytes 含 512 個重複選定引用及 512 個 mapping，首錯為 `video_coverage.duplicate_selection`，位置 `$.selected_unit_refs[1].unit_id`，exit 1。B19 含 513 筆選定引用，首錯仍是 A helper 錯誤。B20 回傳重複素材錯誤。B21 接受固定影格率輸入並保留 unresolved 單元。B 的文字與提示驗收留待 G3。
