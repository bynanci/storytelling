<!-- SPDX-License-Identifier: MIT -->
# SK-E04 · 嚴格輸入與證據身分回歸

[English](sk-e04-input-boundary.md) · 繁體中文 · [#121](https://github.com/bynanci/storytelling/issues/121) · [協定](../research/evaluations/sk-e04/README.zh-TW.md)

Base：`c562ce0d06b7330813fa42cad739f24f4a49e9da`。本修復保留既有 v1／v2 紀錄欄位與報告計算，不修改 Rust／CLI Schema、Skill 規則、R1 草稿、分數、宣告分組、錯誤旗標或研究結論。僅執行合成工具輸入，沒有模型、參與者、真實媒體或編輯器操作。

## 重現與結果

| 邊界 | 修復前 | 修復後／回歸 |
| --- | --- | --- |
| 重複 JSON 欄位，包括巢狀 outcome／preference | 後值靜默覆蓋前值，矛盾呼叫結果仍可產生成功報告。 | 分析前拒絕，exit 2，不產生報告；診斷不回顯不可信欄位或控制文字。 |
| 辨識欄位型別錯誤 | `schema_version`／`record_kind` 為陣列或物件時拋出未處理 `TypeError`，CLI exit 1 並顯示 traceback。 | API 拋出 `RecordError`；CLI exit 2，不產生 traceback 或報告。 |
| 無效 UTF-8 | 解碼例外外洩，exit 1。 | 沿用輸入錯誤路徑，exit 2，不產生報告。 |
| 非有限常數／解碼器數值錯誤 | 型別驗證稍後拒絕非有限欄位；標準解碼器既有整數轉換失敗卻會外洩。 | 明確拒絕非有限常數，處理既有解碼失敗；不新增數值、大小或深度政策。 |
| v2 相同不透明輸出 ID 對應不同 SHA-256 | 同案或跨案矛盾中繼資料仍獲接受。 | 全紀錄檢查並拒絕；同 ID／同雜湊仍可重複使用。 |

Artifact 檢查只建立宣告一致性，不讀取或認證證據位元組。評審上下文 ID 範圍不變。既有翻轉計數／比率仍按配對加權；逐稿分數變化與兩種順序的分數差移動量，仍先在案例內平均，再跨案例平均。不新增欄位或指標。

## 實際 RED 與 GREEN

測試先行 commit 為 `c2a0d05b7984a7212e113db86d54293413b6d9cb`。[首次 RED](sk-e04-input-boundary/red.log) 執行 22 個 test methods，記錄 **11 個 failures 與 4 個 errors**。Subtests 與 methods 重疊，不能相減編出通過數。四個 errors 就是已重現的 API `TypeError` 缺陷，不是測試環境設定錯誤。

首次實作通過 22 項後，審查先新增兩個 CLI 回歸，再修改對應行為。[追加 RED](sk-e04-input-boundary/red-parser-diagnostics.log) 執行 24 個 methods，兩項失敗分別是解碼器既有整數轉換錯誤，以及重複欄位診斷回顯換行。該次使用尚未提交的中間實作；[收據](sk-e04-input-boundary/receipts.json)記錄實際 source／test hashes，不虛構 commit。數值回歸只在子程序設定 `PYTHONINTMAXSTRDIGITS=4300`，使既有解碼失敗可重現。

[GREEN](sk-e04-input-boundary/green-final.log) 記錄全部 24 項通過。收據包含精確日誌與程式／測試雜湊。最初共享磁碟已滿，sandbox 命令無法啟動；此環境失敗未列為 RED。清理可再產生的建置資料後才恢復測試。

Unittest 原始失敗輸出含行尾空白，首次 staged diff 檢查有抓到，原始位元組未改寫。`.gitattributes` 僅將這些原始 `.log` 收據排除於空白 lint，並固定 LF 位元組。程式及文件仍保留空白檢查。

實作前擷取既有 v1／v2 合成 fixture CLI 輸出，再與修復後逐位元組比較。Exit code 仍為 0，stdout SHA-256 維持：

| Fixture | 修復前後 stdout SHA-256 |
| --- | --- |
| v1 | `b1de9eb18c1e5d1c8301be9651afdd65365a50bc59ce69cc64ba07d739048c3b` |
| v2 | `ec2aba1d6f4b3f3d54c28fff0f0752f136df3ff74a3b961a43b20fa0ff68d363` |

## 檢查與剩餘門檻

```sh
python3 scripts/test_sk_e04_order_analysis.py -v
python3 -m unittest discover -s scripts -p 'test_*.py' -v
python3 -m unittest discover -s examples/evaluation/n07-pilot -p 'test_*.py'
python3 -m unittest discover -s examples/evaluation/n07-recorder -p 'test_*.py'
python3 -m unittest discover -s examples/evaluation -p 'test_*.py'
python3 tests/oracles/filmcraft-25-v1/test_checks.py
python3 tests/fixtures/execution-contract-v1/check_design.py
python3 scripts/check_repo.py
git diff --check
```

Scripts suite 通過 99 項、pilot helpers 12 項、recorder helpers 執行 22 項（因本 checkout 無建好的 CLI，跳過一項 native replay）、editorial checks 11 項、宿主證據拒絕檢查 6 項、execution-design checks 5 項。這些檢查不操作宿主，也不提供真人證據。文件完成後另行記錄 repository／diff 檢查。目前另一個 agent 使用共享 Cargo slot，本變更尚未本機執行 Rust workspace 檢查。合併前，最終 PR head 必須通過必要 CI 與擴充後的 Linux／Windows／macOS SK-E04 workflow；此處不宣稱已有結果。

#121 原始重評仍缺已授權的精確草稿、資料包、提示／量表及評審設定。#122 維持受阻，不建立 holdout。凍結分派清單與已授權證據位元組仍需另行核對。G2 決定、真人接受與 G5 發布門檻維持。

## Journal 修正後的整合

兩個 commits 已 rebase 至 main `5bbe80dc2cf8286abb15ad5278e86ec7384cdb4d`，保留 journal 與 SK-E04 的需求／任務追溯。在 rebase 後的實作 head `80218b7d070218195933cb7ca5639bd2fb1b4e87`，重新通過 24 項 focused methods、v1／v2 stdout 逐位元組比對、repository 與乾淨 diff 檢查，見[整合收據](sk-e04-input-boundary/integration.json)。Rebase 將測試先行 commit 改寫為 `5ca1e3ad50830cb59a94f5cbda6c58da8e10470b`；原始 RED 仍是實際於 `c2a0d05` 觀察，保留上述紀錄。此次整合未呼叫 Cargo，不宣稱已有本機 Rust 或 hosted CI 結果。
