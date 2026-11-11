<!-- SPDX-License-Identifier: MIT -->
# N-09／#66：修傘邀請錄音提示表

**狀態：** 有界 N-09 實作與自動化驗證完成；未宣稱真人聽感或錄音驗收。

**最新 rebase 基準：** `main` `b7fcf869d01727dcf408c457fdd42929dca80c7a`。此版本保留主線的 N-08、N-11、N-12 與指南更新。

**範圍：** 新增獨立版本化 `NarrativeCueSheetV1` 與本機 `render-cue-sheet`。`NarrativeDocumentV1` 保持原樣且仍為封閉契約。

## Skill 請求與素材對照

給 `$storytelling` 的請求：

> 給錄音者一份兩段式修傘邀請：旁白、停一拍、兩聲敲擊。

取自 [修傘工作坊範例](../../skills/storytelling/references/workshop-demo.md)的 S1：週六舉辦修傘活動，請帶一把傘。旁白「週六修傘工作坊，請帶一把傘。」只改寫這項通知，引用 `umbrella-workshop@1 / oral / invite / S1@1`。停頓依請求保留，500 ms 是本提示的估時。旁白仍待錄，長度未定。S3 是合成的兩聲敲擊，不推測敲擊來源。

Issue #50 的已合併口述稿寫的是鄰居修門，素材不適合修傘邀請，所以本票沒有挪用該句子；此提示使用 S1 的新 `NarrativeDocumentV1` 最小來源。`taps.wav` 經 WAV header 讀回為單聲道、16-bit PCM、16 kHz、17,280 frames，長度 1,080 ms；`verify-source` 對實際檔案 bytes 計得 SHA-256 `4008e27e4c94376747a887df8d873820e4ab88e668816f79ddca5c9d69bb202b`，共 34,604 bytes。這只核對指定檔案，不代表聽感或混音驗收。

## 本機命令與可讀提示表

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet \
  tests/fixtures/narrative-v1/n09-umbrella-source-v1.json \
  tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
```

退出碼為 0。逐 cue 的實際標準輸出保存在 [`valid.stdout`](n09-cue-sheet/valid.stdout)，對應退出碼在 [`valid.exit`](n09-cue-sheet/valid.exit)。輸出順序是待錄旁白、停一拍、播放現有 S3 WAV；每列都列出角色、聲軌與來源。三個相對時間區間因旁白長度未定而未解析，工具清楚標示「3 unresolved」，沒有把未知當 0，也沒有宣稱這張表已證明不存在軌道衝突。

對同一輸入重跑兩次，使用 `cmp` 逐位元比對標準輸出，退出碼 0；[比對結果](n09-cue-sheet/replay-byte-equality.exit)。整合測試也直接比較同一 CLI 執行兩次與 stdin 輸出 bytes，並確認 cue sheet 輸入不變、stdin 執行不在工作目錄寫檔。

## 正反案例與保存的輸出／退出碼

| 案例 | 預期／實際 | 保存輸出 | 退出碼 |
| --- | --- | --- | --- |
| 待錄旁白、估時停頓、已知 WAV 長度 | 接受；旁白未定時間保留，3 個區間未解析 | [CLI 文字](n09-cue-sheet/valid.stdout) | [0](n09-cue-sheet/valid.exit) |
| 兩個不同 exclusive 軌疊音：voice `[0,2000)`、effects `[1000,2080)` ms | 接受，2 個區間解析完成 | [CLI 文字](n09-cue-sheet/cross-track-overlap.stdout) | [0](n09-cue-sheet/cross-track-overlap.exit) |
| 同一 voice exclusive 軌首尾相接：`[0,1000)`、`[1000,1500)` ms | 接受，半開區間邊界不衝突 | [CLI 文字](n09-cue-sheet/adjacent.stdout) | [0](n09-cue-sheet/adjacent.exit) |
| 同一 voice exclusive 軌交疊：`[0,1500)`、`[1000,2000)` ms | 拒絕 `narrative.cue.track_conflict`，列出兩個 cue ID | [JSON diagnostic](n09-cue-sheet/track-conflict.stdout) | [1](n09-cue-sheet/track-conflict.exit) |
| 反向區間：start 1500、end 1000 ms | 拒絕 `narrative.cue.invalid_range` | [JSON diagnostic](n09-cue-sheet/reversed-interval.stdout) | [1](n09-cue-sheet/reversed-interval.exit) |
| 重複 cue ID | 拒絕 `narrative.cue.duplicate_id` | [JSON diagnostic](n09-cue-sheet/duplicate-id.stdout) | [1](n09-cue-sheet/duplicate-id.exit) |
| 指定 `taps.wav` SHA-256 的實際 bytes | 核對相符；34,604 bytes | [verify-source JSON](n09-cue-sheet/taps-asset-check.stdout) | [0](n09-cue-sheet/taps-asset-check.exit) |

輸出和退出碼檔是在實際執行以下正常 CLI 命令後保存，沒有從測試預期值合成：

```sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet DOCUMENT.json CUE_SHEET.json
cargo run --release --quiet --locked -p storytelling-cli -- verify-source skills/storytelling/assets/umbrella-workshop/taps.wav 4008e27e4c94376747a887df8d873820e4ab88e668816f79ddca5c9d69bb202b
```

## RED、GREEN 與回歸

先新增 CLI 整合測試與正反 fixture，再於當時的主線基準 `27d6985` 執行：

```sh
cargo test --locked -p storytelling-cli --test cue_sheet
```

實際 RED：CLI 尚未辨識 `render-cue-sheet`，8 個整合案例全部因 usage exit 2 而失敗；Cargo 測試程序退出 101。測試首次也觀察到未知命令提早關閉 stdin，因此調整測試寫入器接受預期的 BrokenPipe，讓缺少命令本身成為 RED。

實作後首次 GREEN 嘗試有 7/8 通過；剩下的斷言期待了不同的文字標籤，依實際 CLI 的 `ESTIMATED duration 500 ms`／`KNOWN duration 1080 ms` 輸出修正斷言。原始整合案例共 11 個，涵蓋跨軌疊音、首尾相接、同軌衝突、反向區間、重複 ID、未知時間、未知 schema version、過期 document revision、素材宣告長度不一致、檔案重播與 stdin；此次 review 後另加 multiline 顯示安全案例，總數為 12。

初次 N-09 實作曾 rebase 到含 N-04 與指南更新的 `213c496`。解決 PR #78 與新主線的衝突時，再 rebase 至上述最新 `b7fcf86`，並保留其 N-08/N-11/N-12 內容。審查發現角色、來源與時間文字可含換行，原始提示表會將其解讀成額外指示行。新增 `escapes_untrusted_multiline_cue_fields_in_human_readable_output` 整合案例先確認此問題，再以 JSON 字串格式輸出角色、來源與時間欄位；素材路徑另拒絕控制字元。修正後，整合測試 13 個案例全數通過。

完整驗證在 release 產物上完成：

| 命令 | 結果 |
| --- | --- |
| `cargo fmt --all -- --check` | 通過 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 通過 |
| `cargo test --workspace --locked` | 通過；僅忽略需 pinned FilmCraft 宿主的既有整合測試 |
| `cargo test --workspace --release --locked` | 通過；同一宿主測試按設定忽略 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked` | 通過 |
| `python3 scripts/check_repo.py`、graph plan/trace、FilmCraft oracle checks | 通過 |
| `python3 scripts/test_skill_distribution.py`、`bash scripts/test_skills_cli.sh` | 通過 |
| `git diff --check` | 通過 |

PR #78 的 required gates 必須在衝突解決後的同一 head 上通過；以 [PR checks](https://github.com/bynanci/storytelling/pull/78) 上顯示的 head SHA 和 workflow 結果為準。

## 重播與限制

```sh
source /workspace/.storytelling-env/activate.sh
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-cross-track-overlap-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-adjacent-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-conflict-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-reversed-v1.json
cargo run --release --quiet --locked -p storytelling-cli -- render-cue-sheet tests/fixtures/narrative-v1/n09-umbrella-source-v1.json tests/fixtures/narrative-v1/n09-cue-sheet-duplicate-id-v1.json
```

此切片交付的是可供錄音者手動操作的提示表，沒有實際配音、TTS、播放、混音或真人聽感驗收。未知區間不能參與排除衝突的判定；錄音後需提供旁白長度，再更新提示表與重跑檢查。
