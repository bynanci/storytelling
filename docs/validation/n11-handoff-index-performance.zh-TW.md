# N-11 交接來源索引：本機效能驗證

[English / 凍結流程與完整紀錄](n11-handoff-index-performance.md)

本次將既有唯讀 `render-handoff` 的逐次線性搜尋改成每份產物的私有來源索引。
索引只使用已綁定變體的來源，不以素材目錄取代舊版引用；輸出順序、版本、創作
提議範圍與第一個錯誤維持原契約。沒有新增公開格式、權限、媒體讀取或 Host 動作。

基準為 `87113b9`，實作來源為 `c5b4c61`。先凍結四種合成負載、seed 910206、
每種 21 組配對與兩種大型負載中位數比值 ≤0.75 的判準，再實作與量測。兩個
release binary 使用獨立、初始不存在的 Cargo target 建置。所有負載先暖機，
84 組配對的負載與組內執行順序均隨機排列，保留實際順序和每次結果。

| 產物數 | 基準中位毫秒 | 修改後中位毫秒 | 中位數比值 |
| --- | ---: | ---: | ---: |
| 1 | 8.836 | 7.314 | 0.8278 |
| 8 | 26.962 | 16.617 | 0.6163 |
| 32 | 89.083 | 48.761 | 0.5474 |
| 63 | 173.874 | 95.283 | 0.5480 |

[原始配對紀錄](n11-handoff-index-performance/paired-release.json)中的 176 次 CLI
呼叫均成功、stderr 為空且同負載輸出位元組完全相同。兩個大型負載達標；63 份
產物的中位耗時約減少 45.2%。計時包含啟動、讀取與 stdout 收集，CPU 時間另記。
這是同一 Linux 機器的四種合成負載；重跑不是新增真人樣本，也不是 Skill 成效
或作者省時證據。每種負載都有 256 個來源，尚未量測極小來源集合及其他硬體／
作業系統效能。經驗 p95 不是母體上界或信賴區間。

初次量測的來源 SHA 參數填錯，該[收據](n11-handoff-index-performance/exploratory-mislabeled-source.json)
原樣保留且未納入主要結果；修正身分後沿用同一凍結流程重跑。新增 Rust 測試
首次編譯的 iterator 錯誤亦[保留](n11-handoff-index-performance/characterization-compile-failure.log)，
修正的是測試寫法，沒有捏造原功能的 RED。效能重構的原行為刻畫在修改前已通過。

[完整檢查](n11-handoff-index-performance/checks.json)：debug／release 預設功能
各 315 項、全部 feature 各 334 項通過，各保留原有三項 ignored；fmt、Clippy
與 rustdoc 的警告門檻、文件檢查、九項量測工具測試及四項 CLI 檔案邊界回歸
通過。獨立 AI review 未發現阻擋問題，並明列小集合效能尚未量測。這不是
維護者 G2、真人接受或 Host 驗收；最終 PR head 的跨平台 CI 仍須另外核對。

在根目錄執行 `python scripts/test_benchmark_n11_handoff.py`；完整二進位比較
命令與回退方式見英文紀錄。#103、#107、#129 B/C/D 與 G5 保留各自門檻。
