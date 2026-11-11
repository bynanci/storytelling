<!-- SPDX-License-Identifier: MIT -->
# N-12／#69：取得可開啟 PNG 的宿主可行性

[English](n12-inkscape-png.md) · 繁體中文

**範圍：G0–G2 重現試驗，待維護者審閱建議。** 本次實際交付
[640×360 PNG](../../tests/experiments/n12-inkscape-png/evidence/available/bench.png)
及能力不可用時的[構圖說明](../../tests/experiments/n12-inkscape-png/evidence/unavailable/composition.zh-TW.md)。

## 問題、素材與通過條件

給 `$storytelling` 的原始要求為「給我可開啟的圖檔，保留左邊藍傘、右邊線軸。」
[請求檔](../../tests/experiments/n12-inkscape-png/request.zh-TW.txt)在兩條路徑的
SHA-256 均為 `14014b129c6790129e28ecd6cf2d26fc591bc25a5a122bfb6bbc0bbeb945c517`。
作者要拿到可開啟的檔案，文字樣板本身不足以驗收這個要求。

初始基準為 `main` `213c4963a1d72daf9d902cb0d75f737de86c1caf`；開發期間再次抓取後，
已整合最新 `main` `7ef55612f660efee230623cbdbc3756386739fe1` 的英文優先指南。
#69 沒有既有選案或留言，同票 PR 查無進行中的試驗，因此採用 N-12 的預設案例。
既有 FilmCraft 影片試驗沒有驗證這個 SVG→PNG 能力。#49 已採納的
[ADR 0005](../adr/0005-independent-narrative-document-v1.md)提供資料邊界；
#68 尚未提供可採用的 manifest，本案可依 #49 獨立開始。

唯一輸入是 Skill 原創、MIT 授權的 S2
[`bench.svg`](../../skills/storytelling/assets/umbrella-workshop/bench.svg)，及
[工作坊圖說](../../skills/storytelling/references/workshop-demo.md)。
來源供應 640×360 畫布、左側折藍傘、右側赭黃線軸；本次只轉換格式，不加活動文字。
使用者本次請求已授權這份合成素材的本機匯出與讀回。

執行前訂下的通過條件：

1. 只選一個宿主、一種產出能力，保存版本、命令、stdout／stderr／退出碼。
2. 產出真正 PNG，結束匯出後重新完整解碼，格式為 PNG、尺寸為 640×360。
3. 實際重新開啟圖片，確認左藍傘、右線軸。來源特定填色的像素重心應分別在 x=320 的兩側，作為補強證據。
4. 相同請求及來源在匯出入口不可用時仍交付可讀構圖說明，明示 PNG 待驗。
5. 兩條路徑都不得改寫原始 SVG；輸出使用新目錄。

## 單一宿主與真實匯出

宿主為預先安裝的 **Inkscape 1.4 (e7c3feb100, 2024-10-09)**，只使用本機
SVG→PNG 匯出能力。試驗平台為 Linux x86_64、CPython 3.12.14；Pillow 13.0.0
僅作解碼驗證器。沒有安裝或切換第二個 renderer，也沒有呼叫遠端模型。
每次執行使用獨立暫存 Inkscape profile/cache；不碰使用者平常的設定檔。

實際匯出命令（完整 argv 另見 receipt）：

```sh
/usr/bin/inkscape skills/storytelling/assets/umbrella-workshop/bench.svg \
  --export-type=png \
  --export-filename=tests/experiments/n12-inkscape-png/evidence/available/bench.png \
  --export-width=640 --export-height=360
```

| 實際結果 | 證據 |
| --- | --- |
| 匯出程序退出 0，產出 24,216 bytes PNG | [exit](../../tests/experiments/n12-inkscape-png/evidence/available/export.exit)、[stdout](../../tests/experiments/n12-inkscape-png/evidence/available/export.stdout)、[stderr](../../tests/experiments/n12-inkscape-png/evidence/available/export.stderr) |
| PNG SHA-256 `f2e11234e60fba34d6a29f6e53963a9268e503a55be16ad2a496589edb0afc38` | [匯出 receipt](../../tests/experiments/n12-inkscape-png/evidence/available/receipt.json) |
| `Image.verify()` 後重新開檔、`load()` 完整解碼：PNG、640×360、RGBA | [解碼結果](../../tests/experiments/n12-inkscape-png/evidence/available/readback.json) |
| 藍傘填色 `#437483`：5,719 px，重心 `(184.19,201.73)`，在左半 | 同上，這是特定素材色彩檢查，不是一般物件辨識 |
| 赭黃繞線填色 `#ce933e`：3,002 px，重心 `(476.54,186.63)`，在右半 | 同上 |
| 實際以 `view_image` 再開保存的 PNG：左折藍傘、右線軸、線尾與背景完整 | [視覺讀回紀錄](../../tests/experiments/n12-inkscape-png/evidence/visual-readback.md) |

Inkscape 的 stderr 有 PangoFT2FontMap 與 GtkRecentManager 初始化警告；已保留原文。
程序成功，這張無可見文字的合成圖可完整解碼且物件位置正確。此結果不外推到字型、
外部連結資源、文字排版或其他 SVG 的相容性。

## 同請求的能力不可用路徑

以一個空的 PATH 讓相同試驗行程無法找到選定的 Inkscape 入口。
這是**受控注入的入口不可用**，不是聲稱 Inkscape 1.4 原本不支援 PNG。
`shutil.which('inkscape')` 實際回傳空值，未呼叫版本或匯出程序。
請求與 S2 的 SHA 和可用路徑一致。試驗交付完整的 640×360 構圖、色彩、位置、
來源及重跑條件，狀態為 `brief_delivered_png_pending`。

[降級 receipt](../../tests/experiments/n12-inkscape-png/evidence/unavailable/receipt.json)
列出 `provider_invocations: []`、`png_created: false`。
[程序退出碼](../../tests/experiments/n12-inkscape-png/evidence/unavailable.exit)為 0，
只表示文字交付成功；此路徑的實際圖檔與讀回仍待驗。
這個缺能力試驗不是功能開發的 TDD RED。

兩條路徑的原始 SVG 前後 SHA-256 都是
`87687584b8f4d01360feea9fbf84e510560f44c745a5973e001e55c01a35d1c9`；
另以 Git 核對 Skill 原始檔沒有差異。

## 重播

從 repository 根目錄執行，輸出目錄須尚未存在：

```sh
python3 tests/experiments/n12-inkscape-png/replay.py /tmp/n12-png-replay --expect available
python_bin=$(command -v python3)
empty_path=$(mktemp -d)
env PATH="$empty_path" "$python_bin" tests/experiments/n12-inkscape-png/replay.py /tmp/n12-brief-replay --expect unavailable
rmdir "$empty_path"
```

試驗控制器、解碼器及原始輸出收錄於
[`tests/experiments/n12-inkscape-png`](../../tests/experiments/n12-inkscape-png/README.md)。
重跑需要可呼叫的 Inkscape、Pillow，以及新的可寫輸出目錄；降級只需 Python。
匯出若逾時，先檢查該次輸出是否存在並嘗試讀回，結果不明時保留待驗，勿覆寫重送。

## 儲存庫檢查

rebase 到 `7ef5561` 後，本機格式、嚴格 Clippy、工作區 debug／release 測試、
warnings-denied rustdoc、repository checks、graph plan／trace、FilmCraft 證據拒絕檢查、
skill distribution 與 skill CLI 均通過。既有 benchmark 與 pinned FilmCraft 宿主測試
按設定忽略，未放寬斷言。獨立證據複查沒有待修問題；遠端 CI 仍須按提交 PR head 核對。
本案只有 G0–G2 重現程式與證據，不宣稱產品功能的 RED／GREEN。

## G2 邊界與建議

建議**縮小採用：先採用本機、合成且不含外部資源的 SVG→PNG 手動交付程序**。
理由是本次真實圖檔與能力不可用時的可讀交接均已驗證；無需為這個案例建置通用 provider registry。
這是試驗建議，維護者尚未決定正式 adapter 的採用與排程。

[ADR 0008](../adr/0008-local-png-host-feasibility.md)提議把宿主、命令與讀回憑據放在獨立試驗資料中。
封閉 `NarrativeDocumentV1` 不增加 path、media、duration 或執行欄位；Rust core／engine
仍只驗證記憶體內的宣告。實際檔案、解碼、模型與輸出由外部宿主處理。
沿用 OF-37 的固定 tool build／參數與真實媒體讀回原則，以及 OF-39 的能力與啟用／授權分離。
OF-38 語音／模型與本案無關，保留原待辦；未將 OF-37～39 標示為已完成。

| 待決事項 | 決策者／重啟條件 |
| --- | --- |
| 是否排程正式 PNG adapter | 維護者；採納本試驗後另開工程票，定義輸入限制、程序隔離與結果 admission |
| 其他圖片、文字、字型、外部資源與跨平台 | 維護者提供具體獲授權樣本，再固定工具版本逐項實測 |
| 缺能力路徑的真正 PNG | 宿主提供匯出入口，依相同請求在新目錄重跑並重新讀回 |
| 作者覺得圖檔／brief 是否可用 | 實際作者；本次無真人試用，沒有填寫主觀回饋 |

這份 PR 只交付 G0–G2 重現試驗，沒有產品功能或公共 wire 變更。
本機及同一 PR head 的 CI 依專案門檻檢查；真人作者驗收與正式宿主支援仍分別待決。
