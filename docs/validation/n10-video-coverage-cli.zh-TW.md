<!-- SPDX-License-Identifier: MIT -->
# N-10 G3 CLI 實作與驗證

[English](n10-video-coverage-cli.md) · 繁體中文

**需求：** N-R10／[Issue #67](https://github.com/bynanci/storytelling/issues/67)  
**已接受契約：**[#67 維護者決定](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322)，由 [PR #93](https://github.com/bynanci/storytelling/pull/93) 交付，合併至 `80577b7334110abf0c0765ec1d1c3e4cf4ae3996`。  
**G3 分支：**`feat/n10-video-coverage-cli-20261112`，以該 main commit 為基礎。
**實作：**[PR #97](https://github.com/bynanci/storytelling/pull/97) 已合併為 `169bf6baff33cd452238337237d437e15ac5b292`；合併前 exact head 為 `aa9cafae6dcd51bc78188ac5b80b9bbe3155d234`。
**Hosted 驗證：**該 exact head 的 [run #376](https://github.com/bynanci/storytelling/actions/runs/43148901399) 六項工作全數通過；自動程式審查完成且沒有發現。

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

測試涵蓋已接受的結構與跨引用錯誤碼、兩種敘事變體、完整與 unresolved 映射、九個已接受的合成 fixture、缺漏／過期／懸空引用、無效有理數區間、未知版本、重複原始 JSON 欄位、未知欄位、選取順序、素材 revision、first-error 順序、1 MiB 輸入上限、深度 33、檔案與 stdin 逐位元重播、輸入及相鄰檔案不變，以及沒有影片檔。unresolved 結果與預期 fixture 會各自正規化主機平台的 JSON 行尾後再比較；同一主機上的重跑及檔案／stdin 輸出仍須逐位元相同。

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

維護者在 [#67 原始紀錄](https://github.com/bynanci/storytelling/issues/67#issuecomment-6812948322)提供匿名代碼 `VH-01` 與兩次較早的 session。提供順序是先試算表，再手動核對結構化契約；第二次 session 早於本 CLI 實作，其中狀態碼是依契約人工比對，並非 Rust CLI 執行收據。後續維護者補上[合併後 VH-01-S3](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001)：一場作者親自操作 session，自述耗時 145 秒、先收到 `invalid_range`、修正成 `5/1` 後取得阻擋規劃的 unresolved 結果，並提供逐字原話及使用觀察。VH-01 先前已有接觸；未隨機、未盲測，且無獨立觀察者。Codex 未旁觀或獨立核實作者操作與計時；Codex 另行重播附件 JSON 並重現所述輸出狀態，此重播只核對輸出，不能證實作者操作。見[合併後操作紀錄](n10-05-execution-record.zh-TW.md)。

提供的 `clip-01.mp4` 說明為 FFmpeg `testsrc` 合成測試圖樣。Codex 直接對附件執行 `ffprobe -count_frames`，確認 H.264、1280×720、30/1 fps、5.000000 秒、150 格及 75,276 bytes。檔案沒有複製進 repo。此探測只驗證該附件中繼資料；V1 僅檢查宣告的有理數片長與區間，產品不讀取或雜湊影音位元組、不驗證影格率或同步、不證明 asset ID 指向該檔、不產生 OTIO，也不在剪輯器匯入或播放。證據界線見[操作紀錄](n10-05-execution-record.zh-TW.md)。

## G4 驗證狀態

實作分支的本機驗證已通過：

- CLI 專項測試 7/7。
- Workspace 測試的 debug 與 release 各為 309 通過、0 失敗、3 忽略（忽略的 FilmCraft 案例需要指定宿主）。
- 格式、拒絕 warning 的 Clippy、拒絕 warning 的 rustdoc 與 `scripts/check_repo.py` 均通過。
- N-07 helper 回歸 12 項；story graph plan 與 trace 分別 6、5 項；FilmCraft rejection checks 6 項；凍結 OpenFilm finite-v1 差異 oracle 6 案例，全數通過。
- 既有 Rust video V1、OTIO 與 story-review CLI 回歸，在兩種 workspace profile 均通過。本機未安裝選配的官方 Python OTIO parser；exact-head hosted Quality and Contracts 工作安裝固定版 parser，並在 run #376 通過。

Windows 最初發現 CLI 輸出與預期 fixture 的行尾格式不同；測試現會同時正規化兩者的 JSON 行尾，仍以原始 bytes 驗證同主機重播與輸入檔不變。最終 exact-head Windows 工作已通過。

PR #97 的自動化 G4 驗證及 N10-05 限定證據門檻已完成。真人證據是一位維護者／作者在先前接觸後自我操作，耗時為本人自述；另有機器重播與合成媒體探測。這不是獨立或隨機使用者研究。本工作未授權 G5 發布或上架。
