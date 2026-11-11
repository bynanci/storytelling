<!-- SPDX-License-Identifier: MIT -->
# 案例 B：事件順序固定，揭露方式不同

N-06／N-R06 · 同一事件的兩種口述草稿，不是完整編劇契約。尚無本案例的真人操作紀錄。

## 給作者的材料

| 實際發生順序 | 來源 ID | 原始觀察 |
| --- | --- | --- |
| 1 | S02@1 | 阿梅扶門，阿安收緊上方鉸鏈的一顆螺絲。 |
| 2 | S03@1 | 兩人試關一次；這一次門沒有磨到地面。 |
| 3 | S04@1 | 兩人把工具收回工具箱。 |

請保持以上事件與人物不變，試讀兩種揭露方式：

**A．順敘。** 先交代兩人調整鉸鏈，再讓觀眾看到一次順利關門的結果，最後收工具。

**B．先結果，後交代。** 先讓觀眾看到這一次順利關門，再回看「剛才」兩人調整鉸鏈的動作，最後回到收工具的收尾。

B 讓觀眾較晚知道剛才做過什麼；它沒有把試關移到實際調整之前，也不把一次結果寫成永久修復或證明某個因果。請作者指出哪種開場符合用途，並說明讀者會在哪一段才知道鉸鏈曾被調整。

## 現有模型能表達什麼

[兩種揭露的 V1 範例](screenplay-current-v1.json) 讓兩個具名變體直接引用同一組來源與 revision。在本例中，每份的 beat 陣列用作觀眾接收順序；也可以另外建立一份具名的 chronology 變體列出事件順序，並為它指定自己的 `must_precede`。現有 V1 已能保存多組變體範圍內的順序。獨立的 [N-08 profile](screenplay-order-profile-v1.json) 可檢查 A 的動作先於測試，以及 B 的測試先於解釋。

```sh
./target/release/storytelling render-narrative docs/narrative/n06-cases/screenplay-current-v1.json
./target/release/storytelling check-narrative-style docs/narrative/n06-cases/screenplay-current-v1.json docs/narrative/n06-cases/screenplay-order-profile-v1.json
./target/release/storytelling render-narrative docs/narrative/n06-cases/screenplay-unsupported-chronology-v1.json
```

前兩個探針分別輸出兩稿與檢查兩項宣告的揭露規則。第三個檔案只增加 `occurrence_order`；它的拒絕只證明這個有型別的擴充欄位尚未被採用，不能據此聲稱 V1 無法結構化表示兩種順序。

第一個語意缺口是：若作者要求機器辨認哪份是實際 chronology、哪份是 reveal，以及所有視圖必須涵蓋同一事件集合與 revision，現有變體名稱只是慣例，沒有這些有型別的角色或跨視圖一致性保證。例如在 B 省略未被 profile 規則約束的 S04 收工具 beat，單一變體順序規則並不會因此要求把它補回；這是契約邊界的設計反例，不是本次探針新增的實測結果。N-08 通過也不會證明文字中的「剛才」正確，或讀者理解了回看。

## 最小修改與相容性

先把上方三列事件表與兩稿並排；必要時用額外的具名 chronology 變體和既有 `must_precede`／N-08 規則保存及檢查各份順序，交由作者判斷事實與閱讀效果。只有當作者確實需要機器辨識 occurrence/reveal 角色或檢查跨視圖事件集合／revision 一致性，才討論獨立 profile：它引用相同來源 ID／revision，明確標示角色及跨視圖要求，而非重做已有的變體順序檢查。不要重解既有 `association` 或 `must_precede` 的語意，也不向封閉 V1 加入場景、對白或時間欄。

N-04 的來源／變體歷史仍負責直接引用修訂；新的設計若被採用，須另定順序 profile 的 freshness，而不能把 N-08 通過當成作者核准。

## 驗證與成本假設

現有探針須讓兩稿都保留 S02／S03／S04；兩個合法揭露規則均通過，未採用的有型別事件欄位拒絕。未來若採用新 profile，驗收需區分 chronology/reveal 角色，加入跨視圖遺漏／多出事件與 revision 不一致的負例，保留各變體順序正例、循環與懸空 ID 回歸；確認改揭露不改事件、舊版引用失效時明確報告；作者另回答「先知道什麼」與「實際先發生什麼」。

成本是假設：現有文字／N-08 材料約 0.5–1 個工程工作日；有型別角色與跨視圖事件一致性驗證約 2–4 日，每日按六小時專注工程估算。不含場景／對白編輯、介面、招募與發布。尚不知道作者是否需要那些排除範圍，信心低。

## 作者回饋（待填）

已記錄的總體決定只有「暫緩三者，等實際需求」。未提供本案例的讀者答案、偏好排序或耗時。

| 匿名作者／日期 | A／B 各在何時交代動作 | 實際事件順序是否清楚 | 現有 beat 是否足夠 | 第一個缺少的語意 | 希望的最小改動 |
| --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |
