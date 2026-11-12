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

## VH-01-S3：合併後作者親自操作

維護者提供的 [#67 操作紀錄](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001) 補上 CLI 上線後的單人操作。本次與前述 Codex 重播分開記錄。

| 項目 | VH-01-S3 紀錄 |
| --- | --- |
| 操作者與時間 | VH-01（專案維護者／作者本人），2026-11-12 13:02 +08:00；PR #97 與 #99 合併後。這是自我操作紀錄，沒有外部招募者或獨立觀察者。 |
| 先前接觸與順序 | VH-01 先前做過一般試算表 S1 及 CLI 實作前的契約核對 S2；本次沒有隨機分派或盲測。 |
| 原題與起始資料 | 把兩個選定單元交接至影片規劃：`u-open@2` 使用 `CLIP-01` 的宣告區間，`u-test@1` 保留為缺素材。合成測試影片的宣告片長為 5 秒。 |
| 操作與結果 | 作者先輸入結尾 `6/1`，CLI 回傳 exit 1、`video_coverage.invalid_range`、路徑 `$.mappings[0].source_range.end`；修正為 `5/1` 後重跑，取得 exit 0、`ready_for_planning: false` 及 `video_coverage.unresolved_required`（`$.mappings[1]`）。 |
| 作者耗時 | 145 秒，包含檢視片長、編輯 JSON、兩次 CLI 執行與查看結果。這是作者全程耗時，不等於 Codex 單次程序時間。 |
| 作者原話 | 「實際用合併後的 `render-video-coverage` 跑這次雙單元交接，一開始我把 `CLIP-01` 的結束時間填成 `6/1` 秒，CLI 馬上以 exit `1` 指到 `$.mappings[0].source_range.end` 報 `video_coverage.invalid_range`；我對照 `ffprobe` 的 5 秒長度改成 `5/1` 秒重跑後就順利拿到 exit `0`，而且因為 `u-test@1` 還缺門底特寫，輸出明確標出 `ready_for_planning: false` 跟 `$.mappings[1]` 的 `video_coverage.unresolved_required`。不過手寫 JSON 時發現有理數的 `numerator` 跟 `denominator` 必須加引號寫成字串（像 `"5"`、`"1"`），如果直接寫整數 `5` 會觸發 `video_coverage.invalid_shape`。」 |
| 觀察與限制 | JSONPath 讓作者容易定位越界欄位；手寫有理數欄位必須用 JSON 字串是摩擦點；first-error-wins 要逐次修正並重跑。作者另報告 S1 試算表耗時 260 秒且漏抓越界，但有先前接觸、固定順序及練習效應，兩次時間只作描述，不能推論工具造成節省。逐字原話、作者計時及操作來自維護者提供的紀錄，Codex 未旁觀操作；重播只核對輸出。 |

這份紀錄包含原題、起始資料、作者原話、觀察及限制／失敗紀錄。它提供單人、維護者自述的定性使用性證據，不是獨立使用者研究、隨機比較或統計樣本。先前 VH-01-S2 發生於 CLI 實作前，其結果仍只是手動對照契約；不能視為 CLI 執行。

## 合成 MP4 附件的唯讀核對

本次提供的 `clip-01.mp4` 被描述為以 FFmpeg `testsrc` 產生的合成測試圖樣，不是真實拍攝素材。Codex 以 `ffprobe -v error -select_streams v:0 -count_frames -show_entries stream=codec_name,width,height,r_frame_rate,duration,nb_read_frames:format=size,bit_rate -of json` 直接讀取附件；退出碼為 0，得到 H.264、1280×720、30/1 fps、5.000000 秒、150 格，檔案大小 75,276 bytes。核對只記錄公開安全的媒體中繼資料；影片、雜湊與本機路徑均未加入 repo。

此 `ffprobe` 核對只證明所提供合成檔的可讀中繼資料。`NarrativeVideoCoverageV1` 本身沒有讀取或驗證影片位元組，也沒有證明 JSON 中的 `CLIP-01` 就是這個附件；不檢查同步、不產生 OTIO，也不操作剪輯器。

## N10-05 狀態

依 maintainer 提供的 VH-01-S3 操作紀錄、合併後 CLI 重播及本次合成媒體探測，N10-05 的限定驗收證據已齊備，並以 [#67 紀錄留言](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001) 保留來源與證據限制。這不是外部或隨機使用者試驗；G5 發布仍是獨立維護者決定。
