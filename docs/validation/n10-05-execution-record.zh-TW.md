<!-- SPDX-License-Identifier: MIT -->
# N-10／N-R10 合併後實際操作紀錄

[English](n10-05-execution-record.md) · 繁體中文

**Issue：** [#67](https://github.com/bynanci/storytelling/issues/67) · **實作：** [PR #97](https://github.com/bynanci/storytelling/pull/97)，合併 commit `169bf6baff33cd452238337237d437e15ac5b292`。**執行來源：** [PR #98 合併](https://github.com/bynanci/storytelling/commit/5499f91c94cd12c6c82779978dc986843d403fe5)後的 `main` 原始碼，Linux x86_64、Rust/Cargo 1.89.0。使用兩份附件 JSON，未將輸入複製進 repo。

## 對附件輸入重跑 CLI

Codex 使用合併後 CLI 執行附件 `vh01-valid-unresolved.json` 與 `vh01-invalid-range.json`。這是以作者提供輸入進行的**機器重播，不是作者親自操作的使用性試用**。

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage vh01-valid-unresolved.json
```

退出碼 0；CLI 保留宣告的缺件並阻擋規劃：

```json
{"schema_version":1,"profile":"storytelling.narrative-video-coverage.v1","coverage_id":"vh01-two-unit-cut","narrative_ref":{"document_id":"vh01-gate-short","document_revision":2,"variant_id":"var-main"},"asset_catalog_ref":{"catalog_id":"cat-local-01","revision":1},"selected_unit_refs":[{"unit_id":"u-open","revision":2},{"unit_id":"u-test","revision":1}],"mappings":[{"unit_ref":{"unit_id":"u-open","revision":2},"status":"mapped","asset_ref":{"asset_id":"CLIP-01","revision":1},"source_range":{"start":{"numerator":"0","denominator":"1"},"end":{"numerator":"5","denominator":"1"}}},{"unit_ref":{"unit_id":"u-test","revision":1},"status":"unresolved","reason":"Missing close-up take of the door bottom clearing the ground."}],"ready_for_planning":false,"diagnostics":[{"code":"video_coverage.unresolved_required","path":"$.mappings[1]"}]}
```

```sh
cargo +1.89.0 run --locked -p storytelling-cli -- render-video-coverage vh01-invalid-range.json
```

退出碼 1：

```json
{"error":{"code":"video_coverage.invalid_range","path":"$.mappings[0].source_range.end"}}
```

兩份輸入的檔案入口與 stdin 入口輸出逐位元相同，退出碼相同；執行後輸入檔未變。編譯完成後，以直接啟動 CLI 程序測得單次耗時：合法案例 3.568 ms、無效案例 1.839 ms，包含程序啟動與輸入輸出。這只描述本次 Codex Linux 環境，不是作者耗時或效能基準。這次沒有收集參與者時間、反應或使用性觀察。

另於此原始碼樹重跑合併後 CLI 回歸測試：

```sh
cargo +1.89.0 test --locked -p storytelling-cli --test video_coverage
```

結果：**7 通過、0 失敗**。

## 附件中的媒體探測回報與限制

附件紀錄表示曾執行 `ffprobe -v error -select_streams v:0 -count_frames ... clip-01.mp4` 並以退出碼 0 回傳 H.264、1280×720、30/1 fps、5.000000 秒及 150 個解碼影格。紀錄稱原始素材留在隔離環境。本次附件沒有提供媒體檔，因此 Codex 沒有重跑 `ffprobe`，也無法獨立稽核該輸出。Repo 未加入私人媒體、路徑、雜湊或可識別來源的原文。

這是維護者提供的另一項媒體探測回報；它不代表 `NarrativeVideoCoverageV1` 讀取檔案或驗證 `CLIP-01` 確實指向該檔。CLI 只檢查提供的 JSON 宣告與有理數區間，沒有讀取影音位元組、檢查同步、產生 OTIO 或操作剪輯器。

## N10-05 狀態

附件輸入已完成合併後 CLI 重播，涵蓋成功與 `invalid_range` 錯誤路徑。先前 VH-01 兩次 session 仍是維護者提供且未經稽核；第二次發生於 CLI 實作前。作者親自操作 CLI 的新 session、作者耗時與逐字使用性觀察仍待補。真人證據及維護者決定完成前，#67 保持開啟；G5 發布另行決定。
