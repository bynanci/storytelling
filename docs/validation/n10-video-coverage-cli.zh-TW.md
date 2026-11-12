<!-- SPDX-License-Identifier: MIT -->
# N-10 G3 CLI 實作與驗證

[English](n10-video-coverage-cli.md) · 繁體中文

**需求：** N-R10／[Issue #67](https://github.com/bynanci/storytelling/issues/67)  
**已接受契約：**[#67 維護者決定](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322)，由 [PR #93](https://github.com/bynanci/storytelling/pull/93) 交付，合併至 `80577b7334110abf0c0765ec1d1c3e4cf4ae3996`。  
**G3 分支：**`feat/n10-video-coverage-cli-20261112`，以該 main commit 為基礎。

## 交付行為

`storytelling render-video-coverage <FILE|->` 驗證已接受的封閉 `NarrativeVideoCoverageV1` 輸入，輸出一行精簡 JSON。口述大綱與單幅插畫兩種變體都可選。引擎檢查敘事單元 revision 是否有效、選取單元是否屬於變體且順序一致、目錄宣告身分及精確有理數區間，再依選取順序輸出映射。

合法的 `unresolved` 列會保留在結果中，退出碼為 0，`ready_for_planning` 設為 `false`，並加入 `video_coverage.unresolved_required`。完整宣告可令 `ready_for_planning` 為 `true`；這只代表宣告通過此 profile，不代表影片真實存在、身分可信或適合匯出。契約錯誤回傳穩定錯誤碼及安全路徑、退出碼 1。用法與輸入輸出錯誤分別退出 2、3。

Core 負責 wire DTO；engine 執行有界純驗證與確定性投影；CLI 只讀提供的 JSON，並只寫 stdout。沒有目錄、檔案、媒體、網路、模型、OTIO、專案或剪輯器呼叫。既有 NarrativeDocumentV1、video V1 與 OTIO schema 均未變更。

## 測試先行證據

實作前先從真正 CLI 入口執行前四個測試，全部失敗，原因是 `render-video-coverage` 尚未註冊：合法輸入錯收為 usage 退出 2，而不是 0；無效輸入也錯收為 usage 退出 2，而不是契約錯誤退出 1。擴充為七個測試後，另將完整測試檔套用在 exact base commit `80577b7334110abf0c0765ec1d1c3e4cf4ae3996` 的封存內容上，結果仍為 **0 通過、7 失敗**，皆卡在未知指令的 usage 邊界。這確認新增斷言能區分原有行為。相同命令在實作分支為 **7/7 通過**。

```sh
cargo +1.89.0 test --locked -p storytelling-cli --test video_coverage
```

七個測試：

- `accepts_complete_video_coverage_without_media_access`
- `reports_unresolved_video_unit_and_blocks_planning`
- `rejects_contract_failures_with_stable_codes_and_safe_paths`
- `compares_file_stdin_replay_and_preserves_input_bytes`
- `accepts_illustration_variant_sources`
- `rejects_shape_order_and_json_limit_violations`
- `rejects_cross_reference_duplicate_and_bound_errors_by_contract_code`

測試涵蓋已接受的結構與跨引用錯誤碼、兩種敘事變體、完整與 unresolved 映射、九個已接受的合成 fixture、缺漏／過期／懸空引用、無效有理數區間、未知版本、重複原始 JSON 欄位、未知欄位、選取順序、素材 revision、first-error 順序、1 MiB 輸入上限、深度 33、檔案與 stdin 逐位元重播、輸入及相鄰檔案不變，以及沒有影片檔。unresolved 結果會與 `expected-valid-unresolved.stdout.json` 逐位元比較。

## 重現命令與輸出

在 repo 根目錄執行：

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

預期與實際 stdout（單行；退出碼 0）：

```json
{"schema_version":1,"profile":"storytelling.narrative-video-coverage.v1","coverage_id":"two-unit-cut","narrative_ref":{"document_id":"n10-synthetic-story","document_revision":1,"variant_id":"rough-cut"},"asset_catalog_ref":{"catalog_id":"synthetic-catalog","revision":1},"selected_unit_refs":[{"unit_id":"S02","revision":1},{"unit_id":"S03","revision":1}],"mappings":[{"unit_ref":{"unit_id":"S02","revision":1},"status":"mapped","asset_ref":{"asset_id":"CLIP-01","revision":1},"source_range":{"start":{"numerator":"5","denominator":"2"},"end":{"numerator":"9","denominator":"2"}}},{"unit_ref":{"unit_id":"S03","revision":1},"status":"unresolved","reason":"No video asset was selected for this unit."}],"ready_for_planning":false,"diagnostics":[{"code":"video_coverage.unresolved_required","path":"$.mappings[1]"}]}
```

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage tests/fixtures/narrative-video-coverage-v1/invalid-range.json
```

實際 stdout（退出碼 1）：

```json
{"error":{"code":"video_coverage.invalid_range","path":"$.mappings[0].source_range.end"}}
```

標準輸入也有測試：

```sh
cat tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json | cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage -
```

輸出位元組與檔案入口完全相同。fixture 沒有 MP4 或 MOV；`CLIP-01` 及片長只是合成目錄宣告。

## 真人與媒體證據限制

維護者在 [#67 紀錄](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322)提供匿名代碼 `VH-01` 與兩次 session。每次都列出原題、起始資料、作者原話、觀察及限制。提供的順序是先試算表，再手動核對結構化契約；未提供 session 時間、先前接觸情形或隨機化程序。紀錄未經獨立稽核。第二次 session 早於本 CLI 實作，因此其中的狀態碼是依已接受契約進行手動對照，不是 Rust CLI 的執行收據。若要主張 CLI 可用性，需以本 CLI 重跑。

作者陳述本機 `clip-01` 量得 5 秒／30 fps 下 150 格；沒有公開檔案、雜湊或獨立稽核的量測。V1 只檢查宣告的有理數片長與區間，不讀取或雜湊影音位元組、不驗證影格率或同步、不確認 asset ID 指向該檔、不產生 OTIO，也不在剪輯器匯入或播放。Repo 未加入任何媒體。這不算完成真實媒體驗證門檻。

## G4 驗證狀態

實作分支的本機驗證已通過：

- CLI 專項測試 7/7。
- Workspace 測試的 debug 與 release 各為 309 通過、0 失敗、3 忽略（忽略的 FilmCraft 案例需要指定宿主）。
- 格式、拒絕 warning 的 Clippy、拒絕 warning 的 rustdoc 與 `scripts/check_repo.py` 均通過。
- N-07 helper 回歸 12 項；story graph plan 與 trace 分別 6、5 項；FilmCraft rejection checks 6 項；凍結 OpenFilm finite-v1 差異 oracle 6 案例，全數通過。
- 既有 Rust video V1、OTIO 與 story-review CLI 回歸，在兩種 workspace profile 均通過。本機未安裝選配的官方 Python OTIO parser，因此未執行該 oracle；hosted CI workflow 會安裝固定版本，仍須以同一 head 通過。

實作 PR 的 exact-head hosted CI 與審查尚待完成。本工作未授權 G5 發布或上架。
