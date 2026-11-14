<!-- SPDX-License-Identifier: MIT -->
# N10-06A · 精確影片區間輸入提案

[English](README.md) · 繁體中文 · [Issue #129](https://github.com/bynanci/storytelling/issues/129) · [ADR 0012 候選](../../adr/0012-draft-exact-video-range-input.md)

**狀態：G2 提案；實作受阻，等待維護者針對候選內容決定。** 本次 base main 為 `52dc6c315f7b3d20693f3c51e052509997ab791b`（2026-11-14 UTC 核對）。本資料包定義最小 A 輔助層與驗收輸入。以下新命令、feature、DTO 及診斷均為提案，目前 checkout 尚不能執行。[Gate 盤點](../../validation/n10-06a-gates.md)記錄既有能力與依賴。

## 作者任務與候選輸入

選取兩個目前版本的故事單元，用秒數或影格邊界將其中一個對應到宣告素材，另一個保留 unresolved。輔助層必須保留選取順序、精確的 `unit_id@revision`、缺素材原因及原始時間輸入。產生的宣告是否有效，由既有 V1 驗證器決定。

候選外層含 `draft_version: "n10-06a.1-draft"` 與保存既有 V1 欄位的 `coverage`。mapped 列的 `source_range` 使用以下其中一種封閉格式：

```json
{"unit":"seconds","start":"2.5","end":"4.5"}
```

```json
{"unit":"frames","start_frame":"75","end_frame":"135","fps":{"numerator":"30","denominator":"1"},"timing":"constant"}
```

兩個範例均須轉為秒數 `[5/2,9/2)`，以 canonical `numerator`／`denominator` 字串輸出。影格 IN 零起算，OUT 不包含。明確的固定影格 timing 是呼叫者宣告，沒有證明檔案時間戳；VFR、未知 timing 及 timecode／drop-frame 標籤必須拒絕影格模式。秒數採精確 ASCII 十進位字串，不使用浮點轉換。

預設回應提案會保留每個原始時間物件及輸入／輸出 JSONPath，並列出精確轉換、有效 V1 輸入與原 V1 結果。`--emit-v1` 預計只印出有效 V1 輸入；失敗不得輸出 coverage 文件。有效 unresolved 列仍可見，且 `ready_for_planning=false`。即使為 true，也只能標示 **「宣告可進入規劃」**，不能標示實體素材已驗證或可匯出。

## 目前可執行的 baseline

以下命令已存在，只讀明確指定的 JSON 輸入：

```sh
cargo run --quiet --locked -p storytelling-cli -- render-video-coverage \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
cargo test --locked -p storytelling-cli --test video_coverage
```

既有預期行為見[已接受的 N-10 紀錄](../../validation/n10-video-coverage-cli.md)。本 PR 的實際結果須另留收據；範例命令不等於已執行證據。

## G2 接受後的檢查計畫

候選命令為 `draft-video-coverage-input <FILE|-> [--emit-v1]`，由預設關閉的 `draft-video-input` feature 啟用。**G2 接受且完成實作前，不可將以下候選命令視為可用：**

```sh
cargo test --locked -p storytelling-engine --features draft-video-input
cargo test --locked -p storytelling-cli --features draft-video-input
```

| 計畫檢查 | 預先固定的驗收／失敗證據 |
| --- | --- |
| 精確秒數 | `2.5 → 5/2`、`0.1 → 1/10`、`0 → 0/1`；小數尾零可約分，不四捨五入。 |
| 精確影格 | `[75,135)` @ `30/1 → [5/2,9/2)`；邊界 `150` @ `30000/1001 → 1001/200`。 |
| 寬整數中間值 | 先約分再檢查最終 u64；涵蓋未約分分子／分母超過 u64 但結果可表達、`1/2^63` 的 63 位精確小數及真正溢位。 |
| 輔助層拒絕 | 負數、指數、NaN、地區格式、無效秒數／影格字串、無效／零 fps、VFR／未知／缺少 timing、重複鍵、未知 draft 欄位及 byte／depth 上限，回報候選 helper code 與安全路徑。 |
| V1 最終權威 | 空／反向／超出宣告片長的區間、過期引用、順序及缺 mapping，保留原 V1 錯誤與 first-error 順序；不夾限、不刪列。 |
| 唯讀重播 | 檔案與 stdin 重跑逐位元組一致；輸入及目錄 sentinel 不變；不讀媒體、不連網、不修改 Catalog 或專案。 |
| 相容性 | 既有七項 `video_coverage` CLI、Planner、Narrative、N-11、OTIO、M3/core 與 workspace 回歸維持通過。 |

[驗收向量](../../../tests/fixtures/n10-06a-draft/cases.json)是規格，不是觀察結果。實作者須先加入正常 CLI 入口測試，保存實際失敗輸出及 commit 作 RED，再實作並保存實際 GREEN。不得補造歷史失敗，也不能把計畫中的測試勾為通過。

行為實作後，執行 workspace debug／release tests、`cargo fmt --all -- --check`、Clippy `-D warnings`、rustdoc warnings denied 與 `python3 scripts/check_repo.py`。最終 PR head 的 Windows、Linux、macOS 與其餘必要 CI gate 都須通過；head 有變更就重取相關結果。

## 證據欄

| 證據 | 目前狀態 | 收據／決定 |
| --- | --- | --- |
| 明確候選 head 的 G2 接受 | `blocked` | null |
| 新 helper CLI／engine RED | `not_run` | null |
| 新 helper CLI／engine GREEN | `not_run` | null |
| 實作最終 head 回歸／CI | `not_run` | null |
| 獨立作者輸入秒數／影格 | `not_observed` | null |
| 可信本機媒體綁定／probe | `not_verified` | null |
| Host readback／執行授權 | `blocked` | null |
| G5 發布決定 | `not_requested` | null |

機器檢查、媒體觀察、作者操作、Host readback、維護者接受與 Release 各須獨立日期及來源／收據。合成 fixture 不提供真人成果。私人路徑、媒體及敏感證據留在公開 repository 之外。

## 維護者決定與後續

維護者可針對候選 commit 明確接受或要求修改，例如：

> 接受 N10-06A G2，候選 commit 為 `<exact candidate SHA>`：接受 ADR 0012 的 draft 外層、秒數／影格文法與上限、預設關閉的 `draft-video-input` feature 及候選 engine API、CLI／輸出、helper 診斷與順序、精確 checked 整數算法，以及 V1 最終權威界線。授權依此進行聚焦 A 的 G3 RED／GREEN 實作。B／C／D、媒體存取、Host 執行、G5 及 merge 仍各自待決定。

此段是決定範本，尚無接受紀錄。若審查要求修改欄位或診斷，先修訂候選再進 G3。

B 的逐列預檢／advisory UX 接續已接受且驗證的 A；未來 SDK／UI 遵循 #107–#109。C 須獨立的本機檔案契約、威脅模型與隱私 G2。#103／PR #117 及 #104–#106 未完成前，D 維持 dry-run 清單。任何隱藏 V1 首個錯誤、靜默夾限、補造缺素材、把 metadata 當內容證明、允許自我核准或自動重試 unknown 的方案，都須縮小或停止。回退時移除 helper／feature；既有 V1 流程仍可使用。
