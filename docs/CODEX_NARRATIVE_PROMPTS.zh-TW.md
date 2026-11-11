# 從 Storytelling Skill 開始的七個 Codex 任務

已派出這七段的團隊請接[多模態續篇：新增五段 prompts 與啟動條件](CODEX_MULTIMODAL_PROMPTS.zh-TW.md)。第 2 段的 [#49 契約已採納](https://github.com/bynanci/storytelling/pull/71)，第 3 段請在既有 #50 任務沿用已定案的 `storytelling render-narrative <FILE|->`。第 1 與第 7 段共用[真人試用包](narrative/N-01-AUTHOR-STUDY.zh-TW.md)。

在 `bynanci/storytelling` 執行，每次貼一段。從最新 main、AGENTS.md、TASKS.md、docs/SDLC.md 與對應 Issue 接續。每個 PR 交付該階段的產物、證據與未完成的人工作業。功能變更依現有 SDLC 先取得 RED，再驗證 GREEN 與同一 head 的 CI。

作者在 [Storytelling Skill](../skills/storytelling/SKILL.md) 用日常語言描述需求。Skill 可直接起草內容；需要可重跑的判定時，再連接已實作且有明確輸入的本機工具。Skill 產出的示例、Rust 的機器驗證、真人使用結果各自記錄。以下任務對應[驗收需求](REQUIREMENTS.md#proposed-cross-media-narrative-requirements)。

## 1. 需求與基準

```text
在 bynanci/storytelling 執行 #48／N-R01。從 $storytelling 的自然語言請求開始，用「鄰居修門」做一份口述與兩份插畫構圖，紅傘只放在作者指定的一版。另做一份缺乏佐證的商業文案。保存請求、原始觀察、輸出和三者間的差異。準備與普通筆記使用同一修改題的計時表、錯誤記錄及繼續／縮小／停止判準。交付可直接給作者操作的文件 PR，真人結果依實填寫。
```

## 2. 資料契約

```text
在 bynanci/storytelling 執行 #49／N-R02，接續 #48 的範圍決定。以 Skill 收到的原話及作者看到的產物為兩端，設計獨立版本的結構化資料契約與 ADR。定義來源、修訂、變體、推測及創作提議，區別自由聯想循環和強制順序衝突。給出合法、懸空參照、未知版本、重複欄位及超過上限的案例。說明如何保持影片 V1／OTIO 相容，以及何時應繼續用文字草稿。交付設計 PR。
```

## 3. 第一個可重跑的跨媒介流程

```text
在 bynanci/storytelling 執行 #50／N-R03，依 #49 已接受契約，把同一份來源產出口述與兩份構圖的本機工作流接回 $storytelling。讓作者直接說明需求，Skill 根據已提供資料整理輸入並呼叫新增的 CLI；記錄有實際執行的命令及結果。測試無素材檔與片長、壞參照、輸入不變與相同輸入的確定性。同步更新 Skill 交接指引，附一組自然語言操作及 CLI 重播，跑既有影片回歸。沿用現有工作區與 I/O 邊界。
```

## 4. 修訂與影響範圍

```text
在 bynanci/storytelling 執行 #51／N-R04，接續 #50。透過 $storytelling 接收「修改共同資料」及「只改構圖 B」兩種請求，接上來源 revision 與受影響變體的實際查詢。驗證共享來源變更能找出全部引用、專屬細節只影響一版、舊版仍可查。採用插畫提議不能改寫原始觀察。交付 PR、自然語言前後對照、CLI 回歸與測試證據。
```

## 5. 商業文案與依據

```text
在 bynanci/storytelling 執行 #52／N-R05，接續 #51。從 $storytelling 的文案請求產出受眾、主張、佐證、標語與行動呼籲；讓作者能追問一句話來自哪份資料。分別測試有引用、無引用、參照不存在和來源已修訂。用試行組從 10 到 8 分鐘的例子，示範如何保留資料適用範圍。交付可重跑的機器檢查、人可閱讀的結果與作者審閱欄。
```

## 6. 選擇下一個用途

```text
在 bynanci/storytelling 執行 #53／N-R06，接續 #51。以三個實際 Skill 請求比較：保留兩個互斥腦暴分支、區別劇本事件與揭露順序、把影片節拍配到現有素材並保留尚無素材的節拍。列出文字工作流已能完成的部分、首個失敗案例、最小引擎修改、驗證成本及作者回饋。交付比較與決策 PR，最多推薦一個後續實作。
```

## 7. 真人試用與交付決定

```text
在 bynanci/storytelling 執行 #54／N-R07，接續 #52；#53 可獨立進行。讓作者從 $storytelling 完成 #48 同一組口述、插畫修訂與文案查核任務，再與普通筆記基準比較。記錄試用版本、完成數／總數、修改耗時、歸因錯誤與回饋。CLI 測試和計時另列。缺真人資料時先交付可執行試用包，標明待填觀察。提出繼續／縮小／停止建議，若建議發布候選版，列出相容性、遷移及回退證據，由維護者依 G5 決定。
```
