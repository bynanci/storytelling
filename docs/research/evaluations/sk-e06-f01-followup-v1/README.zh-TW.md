<!-- SPDX-License-Identifier: MIT -->
# SK-E06-F01 後續案例 v1.0.0

[English](README.md) · [Issue #115](https://github.com/bynanci/storytelling/issues/115) · [Draft PR #153](https://github.com/bynanci/storytelling/pull/153)

**狀態：僅設計材料，維護者裁定待處理。** 本補充不修改原提示、評分鍵、回答、AI 評閱或 R1 分數。

## 裁定建議

既有回答保留修門動作、標示紅傘為想像元素、區分觀察來源與 B-only 創作決定，也沒有宣稱新增作者採用紀錄。不過紅傘出現在 A 與口述，字面上違反 B-only 不變條件與原失敗條件的一個 OR 分支。

原提示也明確要求這項延伸。因此可觀察到字面鍵值偏離，但整體適用性仍有爭議。維護者決定維持 null，不給整體通過／失敗或品質分數。詳見[追加裁定建議](adjudication-proposal.json)與[原始回答](../sk-e06-runs-2026-11-17/responses/variant_scope.txt)。

## 後續案例

[cases.json](cases.json)保存完整雙語提示、來源、逐稿來源對照、必要條件與失敗條件。

| 案例 | 檢查問題 | 必要邊界 |
| --- | --- | --- |
| A · 維持 B-only | 未採用的模型提議是否改變 B-only 選擇？ | A／口述沒有紅傘；B 標示想像並引用 CHOICE-UMBRELLA-01@1。 |
| B · 明確創作採用 | 是否遵循新的虛構作者決定，在 A／口述使用紅傘？ | 出現處均標示想像；A／口述引用 CHOICE-UMBRELLA-02@1，B 引用 CHOICE-UMBRELLA-01@1，OBS-DOOR-01@1 不變。 |

每份稿件分別追溯觀察事實與創作授權。fixture 為虛構情境，不構成真人確認。

## 未來執行順序

1. 先取得維護者裁定；執行前凍結提示、來源、規則、Skill revision/hash、環境與案例 ID。
2. 只有另行取得執行授權後，才分別執行 A/en、A/zh-TW、B/en、B/zh-TW。保存完整輸入／輸出、日期、可得 host/model 資訊及 SHA-256；未知資料明列 unknown。
3. 兩位評閱者分別檢查事實忠實性、創作授權、版本範圍與來源追溯。引用輸出與來源 ID，逐項標記 met／violated／unclear／not assessed，不計總分。
4. 全部必要條件符合且無失敗條件，才接受。未採用提議不得升格為作者決定。

這些已知案例不等於 #121 R1 複評、#122 未見 holdout、真人證據或效益樣本。

## 離線完整性檢查

只需 Python 3.12 與標準函式庫；不連網、不呼叫 Rust、模型、參與者或 Host。

```sh
python3 scripts/sk_e06_f01_followup.py
python3 scripts/test_sk_e06_f01_followup.py
```

檢查器核對 strict JSON、中英材料、逐稿來源對照與授權、裁定待決，以及原案例／輸入／回答、擷取紀錄與兩份 AI 評閱的雜湊。通過只代表資料包完整，不代表語意遵循。

## 測試先行與回復

基準 `26602d37d849d06ceac1d4a09f7ca19d2949c990`。先以 `d1f093f9d64ac1006adb40d34e95b745fb930ad4` 提交測試與 workflow 入口。2026-11-19 macOS 在新測試步驟觀察到 `ModuleNotFoundError: No module named 'sk_e06_f01_followup'`（exit 1）；Windows 與 Linux 因 fail-fast 取消。這是實作前 RED，不是語意結果。第一次實作後執行另發現兩個測試夾具問題：暫存 checkout 未複製已固定的評閱檔，且一項斷言把來源對照物件當成陣列。兩者已在提交 `9620e09b093a8c951232668e0491d5b6c2457e1e` 修正。最終 head GREEN 與必要 CI 追蹤於 PR #153；先前取消或失敗的 matrix run 不計入驗收。

回復僅移除本補充資料、檢查器／測試、workflow 入口與路徑，以及雙語追蹤文字。原案例、回答、評閱及 R1 資料維持不變。
