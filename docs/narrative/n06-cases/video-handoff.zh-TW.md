<!-- SPDX-License-Identifier: MIT -->
# 案例 C：選定單元對應素材，缺少的保持 unresolved

N-06／N-R06 · 只比較宣告與缺口；沒有影片檔、實際匯入、播放或宿主寫入證據。

## 給作者的銜接表

請把選定的 S02@1（調整鉸鏈）和 S03@1（一次關門測試）交給影片工作流。素材卡是合成宣告：`clip-hinge-declared` 長度 12/1 秒，雜湊是 64 個 a 的佔位值，**並非由實際檔案算出**。

| 選定單元 | 宣告的 asset | 來源區間（秒、半開） | 對應狀態 | 未解問題 |
| --- | --- | --- | --- | --- |
| S02@1 | clip-hinge-declared | [5/2, 9/2)，即 2.5 秒至 4.5 秒，共 2 秒 | declared，只是對應宣告 | 實際片段是否真的拍到此動作？未提供檔案。 |
| S03@1 | null | null | unresolved | 沒有提供關門測試的錄影，不得捏造片段或默默刪掉此單元。 |

這張表是 [探索草圖](video-mapping-sketch.json)，不是已採用的 JSON 契約。秒的有理數也不代表已取得影片的 frame rate、sample clock 或宿主支援。請作者補實際 asset 和區間，或保留缺口；在所有必需的已選單元得到可審核對應前，整份銜接仍是 unresolved。

## 現有模型能表達什麼

NarrativeDocumentV1 保存 S02／S03 的來源與 revision，不保存媒體 asset 或剪點。N-11 能描述宣告檔案、缺少素材與產物元件，不連到 Planner asset_id／來源區間。Planner V1 可以驗證合成 asset 的有理數區間是否位於宣告的 12 秒長度內。

```sh
./target/release/storytelling plan-story docs/narrative/n06-cases/video-resolved-subset-v1.json
./target/release/storytelling plan-story docs/narrative/n06-cases/video-unsupported-join-v1.json
```

第一份**刻意只送已宣告的 S02 素材子集**。Planner 能產生這兩秒的宣告提案，且仍標示非 executable、未驗證來源／宿主。這正顯示第一個整合失敗：送入 Planner 的契約沒有「作者曾選 S03」的資訊；若上游省略缺片段的 S03，Planner 無法發現覆蓋缺口。這不是 Planner 違反其範圍，而是敘事與 asset 間缺少受檢查的 join。

第二份在同一 Planner 輸入加上未採用的 `narrative_mapping` 欄位，應被封閉契約拒絕。正常 Planner 通過不等於整份敘事銜接完整；此子集不得當作已完成的故事提案或已核准 OTIO。

## 最小修改與相容性

先使用上方非執行表，讓每個選定的單元都有一列，包括 unresolved。若真實作者需要自動覆蓋檢查，再評估獨立、有限的來源→asset mapping validator。它需綁定敘事 ID／revision、變體／選定單元、宣告 asset catalog 身分與精確來源區間，並在缺必需對應時停止。不能把單元 ID 當作 asset_id，也不能靠字串相同推導媒體。

保留 NarrativeDocumentV1、N-04、N-11、StoryBrief/Planner 和既有 OTIO/P1 digest 的契約。只在將來採用明確版本後增加轉接檢查；輸出有效性、實際媒體雜湊、宿主匯入、授權及執行仍各需其證據。

## 驗證與成本假設

現有探針只證明已宣告子集的區間有效，並證明新增 join 欄位尚未被接受。未來可獨立驗收的 mapping 切片至少需：完整正例、缺必需對應、過期敘事 revision、懸空 asset、錯誤範圍與超出來源長度；任何 unresolved 必需單元都不得被默默省略。既有 Planner／N-11／官方 OTIO 回歸須保持，不能寫 Catalog 或剪輯器。

成本是假設：手動覆蓋表約 0.5–1 個工作日；有限的純 mapping validator 約 4–7 個工程工作日，每日按六小時估算。不含真實媒體蒐集、獨立剪輯器的匯入／播放／儲存／重開、介面與發布；若加入它們需另估。沒有實際素材或交接任務，信心低。

## 作者回饋（待填）

已記錄的總體決定是「暫緩三者，等實際需求」；未提供本案例的真實 clip 或實際操作觀察。

| 匿名作者／日期 | 真正要剪的片段 | 哪些選定單元無素材 | 誰能確認 asset／區間 | 表格是否足夠 | 最小想自動化的步驟 |
| --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |
