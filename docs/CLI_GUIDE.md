<!-- SPDX-License-Identifier: MIT -->
# CLI 使用指南

本指南從 repository 根目錄執行，使用已接受的有限資料契約。各指令是不同的入口，
不是把前一個 stdout 直接交給下一個指令的通用 pipeline。格式與穩定錯誤碼見
[契約索引](CONTRACTS.md)，實作進度見 [TASKS](../TASKS.md)。

## 建置與依賴

安裝官方 Rust 工具鏈後，repository 的 `rust-toolchain.toml` 會選用固定 1.85.0。
使用已提交的 lockfile 建置：

```sh
cargo build --release --locked -p storytelling-cli
cargo run --release --quiet --locked -p storytelling-cli -- --help
```

產物在 `target/release/storytelling`；Windows 為 `storytelling.exe`。
下方使用 `cargo run`，免去 shell 與執行檔副檔名差異。這六個 production 指令
不需要 Node、Python、FFmpeg 或 FilmCraft。來源比對、官方 OTIO oracle 與真實
宿主測試另有 test-only 工具需求，見後段。

## 選擇入口

| 指令 | 輸入 | 結果與證據範圍 |
| --- | --- | --- |
| `validate` | M0 `PlanDocumentV1` JSON | 宣告 ID、hash 形狀、reference 與 exact source ranges 合法 |
| `audit-brief` | `StoryBriefV1` JSON | 硬限制的參照、包含／排除與順序衝突；手動選擇保留 |
| `plan-story` | Brief＋候選固定視窗／宣告分數＋hard bounds | 保留必要選擇的 deterministic fixed-window proposal 與決策原因 |
| `import-openfilm` | 有限 OpenFilm composition export JSON | declared visual plan 與原始事實／限制報告 |
| `export-otio` | 同一個有限 composition export JSON | 可儲存的原生 OTIO Timeline JSON，附明確 loss 選擇 |
| `verify-source` | 明確提供的 FILE／stdin bytes 與預期 SHA-256 | 該次提供的實際 bytes 符合預期 digest |

JSON 輸入最多 1 MiB；`verify-source` 的 bytes 最多 16 MiB。更細的 collection、
metadata、diagnostic 和 output 預算由各自的 frozen contract 管理。
每個指令均可用 `-` 讀取 stdin；輸入結束時須送 EOF。

## 1. 驗證已宣告的 plan

```sh
cargo run --release --quiet --locked -p storytelling-cli -- validate examples/minimal-plan.json
```

預期退出碼 0，stdout 為 `{"ok":true,"validation":...}`。
`validation.scope` 是 `declared-metadata-only`。檔案中的 64 字元 hash 只是宣告，
不代表已開啟素材或計算其真實內容。

下例應退出 1，錯誤碼為 `plan.source_out_of_bounds`：

```sh
cargo run --release --quiet --locked -p storytelling-cli -- validate tests/fixtures/invalid-source-range.json
```

此預期失敗例應獨立執行；不要把退出 1 誤當成建置或 I/O 失敗。

## 2. 稽核 Brief 的硬限制

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/valid.json
```

預期退出碼 0，stdout 為 `{"ok":true,"audit":...}`。audit 保留手動必選項與順序、
硬限制的來源位置及軟偏好。此入口只檢查已實作的宣告限制；
`selection_planned:false`、`duration_analysis:"not-performed"`，不產生 shots。

下例是有效格式的衝突 Brief，應退出 1，並保留完整 `audit` 而非 `error`：

```sh
cargo run --release --quiet --locked -p storytelling-cli -- audit-brief tests/fixtures/brief-v1/conflicting.json
```

呼叫者應讀 `audit.conflicts` 修正自己的要求；程式不會偷偷刪掉手動選擇以得到綠燈。
不合法 JSON、未知欄位或超限輸入則回傳 `{"ok":false,"error":...}`。
完整語意見 [StoryBrief 契約](migration/STORY_BRIEF_V1.md)。

## 3. 從 Brief 提出固定視窗 plan

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

預期退出碼 0，stdout 為 `{"ok":true,"planning":...}`。
`planning.scope` 是 `declared-fixed-window-plan-only`，`selection_planned:true`、
`duration_analysis:"exact-fixed-windows"`。輸入 envelope 包含原有 Brief、每個 candidate
的宣告 hash／source duration／固定 source range／整數 score，以及硬性
`max_duration`、`max_shots`。不會開啟來源媒體，也不把宣告分數當成已驗證品質。

這個 fixture 手動要求 `detail` 在 `opening` 之前；即使 `opening` 有較高分數和
preference，兩個必要選擇仍保留原順序。預期 planning clock 為 10 ticks/s，
total duration 為 `3/10` 秒，`target_status:"met"`：

| Shot | Asset | Source range（秒） | Timeline range（秒） |
| --- | --- | --- | --- |
| `shot-000001` | `detail` | `[1/10,3/10)` | `[0,1/5)` |
| `shot-000002` | `opening` | `[0,1/10)` | `[1/5,3/10)` |

必要項目先保留；選擇更多項目時依明確 preference／score／ASCII ID 政策排序，
最後尊重 Brief 的 precedence edges。固定 source windows 不會為了湊時長被偷偷裁短。
必要數量／時長超出 hard bound 或缺少必要 candidate 時直接拒絕；軟目標未達則
回傳有界 proposal 與 target status，不聲稱全域 solver 無解。
每個 selection 的 `reason_paths` 指回使用者提供的宣告。

下例應因 Brief 硬衝突退出 1，回傳 `planner.brief_conflict`，且不帶 plan：

```sh
cargo run --release --quiet --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/conflicting.json
```

此 fixture 同時手動必選並排除 `opening`，typed conflict 指向
`$.brief.manual_selected_asset_ids[1]`；程式不會刪掉該選擇來修復矛盾。

`planning.plan` 是可供 M0 validator 消費的內嵌物件；整個 planning envelope 不能直接
當作 M0 input。它也不是 composition export，不能直接送給 `export-otio`。
planner envelope 的 `brief` 接受原始 Brief，不接受 `audit-brief` 回應 envelope。
這個切片沒有多 beat allocation、coverage、locks 或完整 solver parity；
`executable/source_verified/host_verified` 仍為 false。
見 [fixed-window 契約](migration/FIXED_WINDOW_PLANNER_V1.md)。

## 4. 唯讀映射 OpenFilm composition export

```sh
cargo run --release --quiet --locked -p storytelling-cli -- import-openfilm tests/fixtures/openfilm/finite-v1/still.json
```

預期退出碼 0，stdout 為 `{"ok":true,"import":...}`。
`import.plan` 遵守 M0 plan 契約，但整個回應 envelope 本身不是 M0 plan。
`import.source_snapshot` 保留原始事實與數字 lexemes；詳細欄位名稱以
[finite-v1 契約](migration/FINITE_V1.md) 為準。

輸入是 composition export，不是 OpenFilm project／Catalog database，亦不是
StoryBrief。此映射只支援一條連續 picture track 的 still／video cuts；
未知效果、animation、明確 audio 等輸入會拒絕。成功仍為
`declared-visual-mapping-only`、`executable:false`。影片的 audio/media 限制仍須閱讀，
不能以 `ok:true` 推定來源可用或音訊已保留。

## 5. 輸出有限 OTIO

```sh
cargo run --release --quiet --locked -p storytelling-cli -- export-otio tests/fixtures/openfilm/finite-v1/still.json --metadata-only-locks
```

此 fixture 宣告了 lock，所以顯式選擇 `--metadata-only-locks`。
預期退出碼 0，stdout 根物件直接是 `Timeline.1` OTIO JSON，沒有 `ok` envelope。
呼叫者可把它儲存到自己選擇的新檔案；指令本身不寫入專案或開啟素材 URI。

影片需明確加 `--picture-only`，有 true lock 需 `--metadata-only-locks`；
兩者皆放在輸入路徑後，缺少必要選擇時回傳 `otio.loss_choice_required`。
這些選擇允許有限格式降級，不是宿主執行批准。
`export-otio` 直接接受原 composition export，不能把 `import-openfilm` 的完整 envelope
或 `audit-brief` 的輸出接進此入口。精確 content ticks 也不等同 project FPS。
見 [OTIO 契約](migration/FINITE_OTIO_V1.md)。

## 6. 比對明確提供的 bytes

下例比對 repository 範例 JSON 的原始 LF bytes：

```sh
cargo run --release --quiet --locked -p storytelling-cli -- verify-source examples/minimal-plan.json 54e254ed9d62c604306305f95fc844d635f43bfc89cf835e05d4d74318cb6408
```

預期退出碼 0，stdout 為 `{"ok":true,"verification":...}`，
`scope:"provided-buffer-bytes"`。這個 digest 綁定範例檔本身；改變換行或任何內容
都會改變結果。它不驗證 JSON 內宣告的媒體，亦不證明素材權利或來源身份。

實際使用時，預期 digest 應來自可信的 producer／已審閱來源。
傳入格式合法但不相符的 digest 會退出 1，錯誤碼 `source.hash_mismatch`；
CLI 不回傳素材內容或實際不符的 digest。Rust API 的成功結果私有持有該次 verified bytes，
而 CLI receipt 只有 metadata，不能反序列化成可信 buffer。
來源換檔後的未來讀取仍需 filesystem identity／TOCTOU 契約。
見 [supplied-buffer 契約](migration/FILMCRAFT_25_V1.md#native-supplied-buffer-contract)。

## 回應、退出碼與檔案處理

| 退出碼 | 呼叫者應如何理解 |
| --- | --- |
| 0 | 該入口的有限檢查或轉換成功；以 response scope 判斷證據強度 |
| 1 | domain／格式／限制拒絕，或有效 Brief 的硬衝突；讀取對應 `error`／`audit` |
| 2 | 指令或參數用法錯誤 |
| 3 | input／stdout 等 I/O 失敗 |

成功與一般錯誤均為一行 JSON；help/version 是純文字。stdout 本身失敗時無法保證
完整 JSON。CLI 可讀取明確本機檔案並解析指向 regular file 的 symlink，
不是 server path broker 或 OS sandbox。路徑、檔名與檔內文字都不授予新權限。

這六個入口沒有通用的 approve／apply／render 指令。即使 Brief 沒有衝突、
bytes digest 相符且 OTIO 可解析，正式 executor／批准／恢復與人工驗收仍有各自門檻。

## 重播來源與官方格式驗證

純 Rust 的一般驗證：

```sh
cargo test --workspace --locked
cargo test --workspace --release --locked
```

重播固定來源案例與官方 oracle 的說明分開存放：

| 驗證 | Test-only 需求與入口 |
| --- | --- |
| 六個 importer 案例 | Python stdlib；[finite importer oracle](../tests/oracles/openfilm-finite-v1/README.md) |
| 八個 source／官方 OTIO 案例 | 指定 Linux／CPython 3.12／hash-pinned OTIO 0.18.1 wheel；[OTIO oracle](../tests/oracles/otio-finite-v1/README.md) |
| 十個硬限制政策案例 | Python stdlib 重播已凍結輸出；[Brief oracle](../tests/oracles/story-brief-v1/README.md) |
| 十二個 fixed-window 規劃案例 | Python stdlib；[planner oracle](../tests/oracles/fixed-window-planner-v1/README.md)，保留固定視窗／排名政策的 intentional deltas，不宣稱完整 solver parity |

只有重新執行 OpenFilm 來源 producer 才需要固定 Node／tsx 與 pinned source checkout；
重播 frozen outputs 不等於重新執行所有上游測試。重新產生來源 evidence 時使用獨立
輸出目錄，審查差異後再更新 fixture，避免把舊 golden output 靜默覆寫。
以上工具不進入 production Rust dependency closure。

## 重現有界 FilmCraft 實跑

已接受證據使用 [官方 FilmCraft v0.18.0 release](https://github.com/storytold/filmcraft/releases/tag/v0.18.0)
的 Linux x86_64 CLI。先核對 release／archive／binary identity；完整固定值與已觀察
範圍見 [M1-03 record](migration/records/M1-03.md)。harness 不下載或安裝工具，
執行前再次核對 host binary digest。

將下列 placeholder 換成明確的工具絕對路徑，並選擇尚不存在的新 run directory：

```sh
python3 tests/oracles/filmcraft-25-v1/run.py --storytelling /absolute/storytelling --filmcraft /absolute/filmcraft-cli --ffmpeg /absolute/ffmpeg --ffprobe /absolute/ffprobe --run-root /absolute/new-filmcraft-proof
```

父目錄須存在，run directory 本身須尚未建立。所有合成素材、native projects、
影片、logs 和 `evidence.json` 都由這次明確呼叫放入該目錄；這是明確測試路徑，
不是 OS filesystem jail。既有已提交 receipt 是先前實跑的紀錄，不能直接當作新素材通過。

此案例只覆蓋 Linux headless picture-only／25 fps：native sequence 為 1920×1080，
CPU H.264 preview 為 192×108，45 格共 1.8 秒；另檢查真實 import、精確 native ranges、
儲存重開、OTIO 再匯入、source hashes 與逐格順序。
音訊、GUI／GPU、全解析度、更多時鐘／媒體、人工剪輯驗收和正式宿主 executor
仍保留為後續工作。詳細重播入口見 [host harness README](../tests/oracles/filmcraft-25-v1/README.md)。
