# SK-E04 · 評審校準與反序複評

[English](README.md) · 繁體中文 · [總追蹤 #115](https://github.com/bynanci/storytelling/issues/115) · [Issue #121](https://github.com/bynanci/storytelling/issues/121)

本資料包先準備、但尚未執行 Gemini R1 40 案反序重評。R1 報告記錄 40 案全為 X 先於 Y；偏好為 X 27、Y 10、平手 3，560 個評分中有 325 個為 5。這些數字本身不能識別順序效果。

## 前置檢查結果：受阻

公開 R1 資料包有數值評分與摘要，卻沒有全部原始 X/Y 草稿位元組、完整來源資料包、精確提示、原評分規則、評審模型修訂版／設定，以及逐字評審請求／回應。部分早期案例的 X/Y 分組鍵也只保留在壓縮對話摘要中。[來源報告](../2026-11-13-gemini-r1/README.zh-TW.md)記錄了這些限制。本資料包不重建任何缺失的歷史欄位。

反序比較必須固定稿件、資料包、提示、規則及模型設定，因此無法使用目前 repo 對原 R1 文字執行有效反序重評。另做合成／前瞻研究屬於不同 run，不能稱為 R1 重評。本資料包不呼叫模型或外部服務。

## 凍結計畫與評分規則

- [`protocol-v2.json`](protocol-v2.json) 是目前前瞻紀錄契約，取代已準備但未執行的 [`protocol-v1.json`](protocol-v1.json)，並凍結缺失呼叫狀態、明確配對時段、可比較配對的分母及逐稿分數摘要。兩個版本都沒有真人或模型實際評分紀錄。
- [`rubric-v1.md`](rubric-v1.md) 為事實準確、來源可追溯、不確定性、敘事連貫、清晰度、受眾適配及必要任務涵蓋提供獨立錨點。它只供另一個校準 run 使用；不能取代反序比較中未知的 R1 規則。
- 反序 run 每案至少做兩組重複配對；每組的 X→Y 與 Y→X 各用新的隔離上下文。共用完全相同的 X/Y 文字、資料包、評審提示、規則、模型修訂與設定，只更換呈現順序。隨機化四個上下文的執行順序並由主持人私下保存分派鍵。評審只見中性 ID，不得看到 R1 分數、宣告分組或答案鍵。
- 先保留原始輸出與標註，再做裁定。分析器仍接受舊版 v1 完整紀錄，但新 run 或恢復的 run 必須使用 v2。每個預定順序時段都要有紀錄，包括 `not_attempted`；缺少時段列是格式錯誤。只有兩個順序皆為 `valid_annotation` 才算可比較配對。失敗或缺少標註的配對留在預定分母中，只從比較指標排除。
- V2 呼叫結果為 `valid_annotation`、`refusal`、`invalid_annotation`、`timeout`、`request_error` 與 `not_attempted`。拒絕或無效標註須保留輸出雜湊及不透明 artifact ID。逾時或請求錯誤則須同時填入或同時留空。精確輸出留在授權的證據儲存區；公開紀錄不得含私人回應文字或本機路徑。重試使用新上下文與新 replicate ID，不能取代原失敗時段。

分析工具會分開回報預定、可比較、不完整、失敗及未嘗試配對數、呼叫結果、逐案資料、原始／裁定標註差異及未解 W4 來源檢查。每個評分維度會分別回報 X 稿與 Y 稿的平均絕對分數變動。分數變動與兩種順序的分數差移動量先在案例內平均，再跨案例摘要。沒有可比較配對的案例仍會列出，其比較指標為 null。輸出只屬描述性分析，不做顯著性或母體推論；重複呼叫及七個評分維度不會增加獨立案例數。

`matched_preference_flip_count`、`matched_preference_pair_count` 與 `matched_preference_flip_rate` 是次要的**按配對加權**計數／比率，不是各案例等權平均。可比較重複配對較多的案例，對此比率影響較大。解讀合併比率前，先檢視逐案翻轉率與可比較配對分母；分析單位仍為案例。此說明不更改計算、欄位或 Schema。

## 輸入與證據邊界

CLI 對任意巢狀層級的重複 JSON 欄位、非有限 JSON 常數、無效 UTF-8 及錯誤的紀錄辨識欄位型別，回傳 exit code 2 且不輸出 JSON 報告。合法的 v1／v2 完整紀錄保留既有欄位與成功輸出。v2 中，同一個不透明 `raw_output_ref` 在整份紀錄只能對應同一個 SHA-256，包括保留部分輸出的失敗呼叫；相同 ID 與相同雜湊可重複出現。這些檢查僅建立中繼資料一致性，不讀取證據儲存區、不認證實際位元組、不證明評審上下文全新，也不能找回整組被漏掉的預定配對。主持人仍須另行核對凍結分派清單及已授權證據位元組。

[輸入邊界驗證紀錄](../../../validation/sk-e04-input-boundary.zh-TW.md)保留實際 RED／GREEN 證據。不修改原始 R1 內容、宣告分組、錯誤旗標、分數或結論。原始重評與 #122 保留集仍受阻；通過合成工具檢查不會產生新的模型或真人結果。

## 合成工具 fixtures

`fixtures/synthetic-order-pairs-v1.json` 保留作舊版完整紀錄相容性測試。[`fixtures/synthetic-order-pairs-v2.json`](fixtures/synthetic-order-pairs-v2.json) 是目前的合成回歸 fixture，含四個虛構案例，其中包括逾時、拒絕及未嘗試時段。預期工具分母為 5 組預定配對、3 組可比較、2 組不完整、2 組含失敗、1 組含未嘗試時段。這些數值只測試計數，不是模型觀察或評審行為估計。

執行：

```sh
python3 scripts/sk_e04_order_analysis.py analyze \
  docs/research/evaluations/sk-e04/fixtures/synthetic-order-pairs-v2.json
python3 scripts/test_sk_e04_order_analysis.py
```

fixture 結果必須保留 `synthetic_fixture` 標籤。它只示範缺失呼叫如何計數及逐稿分數如何分開，並不代表實際評審效果的大小或方向。

## 恢復條件

執行 R1 反序評分前，須取得可授權使用的精確草稿、資料包、原評審提示與評分規則，以及當時模型／設定紀錄。缺任一項就明列缺失並暫停配對比較。兩種順序及分析都須先凍結，之後才可揭露分派鍵。來源裁定或新版規則另用不同 run ID。model-only 協定不會產生真人證據或產品效益。

目前機器可讀門檻見 [`protocol-v2.json`](protocol-v2.json)。取得原始研究材料授權，並完成合格重評與逐案報告前，Issue #121 保持開啟且受阻。
