# Storytelling 多模態開發：給 Codex 的五段接續指令

已交付[前七段 prompts](CODEX_NARRATIVE_PROMPTS.zh-TW.md) 的團隊，可以從這裡繼續。每次複製一個文字框到有權讀取 `bynanci/storytelling` 的 Codex 工作階段。五段各自包含工作範圍、驗收案例和回報要求，可以單獨使用。

| 建議提交順序 | 這次要拿到什麼 | 何時能開始 |
| --- | --- | --- |
| 1 · [N-08／#65](#n-08) | 同一份素材的兩種敘事處理，以及一條可重跑的風格規則 | #49 契約已採納；和 #51 對齊 ID 與 revision 語意 |
| 2 · [N-09／#66](#n-09) | 錄製者看得懂且能用 CLI 查核的聲音提示表 | #49 已採納；可獨立於 #50 |
| 3 · [N-12／#69](#n-12) | 一個宿主實際產出的檔案、讀回證據和缺少能力時的交付 | 可用 #49 先做試驗；#68 的 manifest 是另一條可選路徑 |
| 4 · [N-11／#68](#n-11) | 文字、圖片與音訊的唯讀交接清單，以及改稿時受影響的產物 | 等 #51 可重跑地提供修訂判定 |
| 5 · [N-10／#67](#n-10) | #53 明選的一種新模式，透過正常本機入口使用 | 等 #53 留下單一採用決定；若暫緩則保留此票 |

前三項可平行。逐票提交時，可先做 #65 和 #66，再用 #69 檢驗一種真實檔案交付。#68 與 #67 之間沒有直接依賴。#51 的修訂判定可重跑便能派 #68；#53 明選一案後便能派 #67。開始前查當下的 main 和進行中的 PR，沿用已完成的工作。

**原本七段如何銜接。** [#49 的契約已採納](https://github.com/bynanci/storytelling/pull/71)。已派出的 [#50](https://github.com/bynanci/storytelling/issues/50) 依該契約實作 `storytelling render-narrative <FILE|->`，從已宣告內容渲染口述與構圖；[#51](https://github.com/bynanci/storytelling/issues/51) 接續做修訂判定。若這兩票已有工作分支，在原任務補充規格即可。[#48 的試用包](narrative/N-01-AUTHOR-STUDY.zh-TW.md) 可同步收集真人基準。[#52](https://github.com/bynanci/storytelling/issues/52) 與 [#53](https://github.com/bynanci/storytelling/issues/53) 在 #51 後可分頭進行；[#54](https://github.com/bynanci/storytelling/issues/54) 在 #52 後沿用同一試用包。

工程範圍可對照[多模態 backlog](../TASKS.md#multimodal-rust-backlog)、[需求與驗收](REQUIREMENTS.md#multimodal-authoring-and-follow-up-requirements)、[SDLC](SDLC.md) 和[已採納的 V1 規格](narrative/NARRATIVE_DOCUMENT_V1.md)。以下範例共用[修傘工作坊素材](../skills/storytelling/references/workshop-demo.md)，方便對照不同任務產出的證據。

### 已派工的任務，貼回原本的 Codex 對話

若原任務尚在進行，可直接貼上對應列的追加指令。多步工具的執行與重驗沿用[既有流程](../skills/storytelling/references/graph-loops.md)。同一份素材已有可靠來源時，直接按 ID 與 revision 使用。

| 原任務 | 可直接貼上的追加指令 |
| --- | --- |
| #50 | 請在本票沿用 ADR 0005 定案的 `storytelling render-narrative <FILE\|->`，渲染輸入中已宣告的一份口述與兩份構圖。以檔案及 stdin 重播，比對 UTF-8 輸出，測過期引用並驗證原始輸入不變。 |
| #51 | 請在修改共同事實與單一構圖前，先寫出各自預期受影響的變體集合。比對工具實際回報的漏報與誤報，並展示作者改寫或拒絕一項提案後，決定仍可追查。 |
| #53 | 三個候選各舉一個文字樣板無法查驗的具體錯誤，比較作者收益與驗證成本。記錄維護者明選的一個候選或暫緩三案；若只有研究者建議，留下建議與待決欄，供 #67 判定何時啟動。 |
| #48／#54 | 請沿用既有真人試用包。分開回報普通筆記與 Storytelling 的完成數／總數、修改耗時與歸因錯誤；沒有真人觀察的欄位留待試用。保留維護者的繼續／縮小／停止決定欄。 |

## N-08

**同一素材，兩種流派。** [Issue #65 · N-R08](https://github.com/bynanci/storytelling/issues/65)

```text
在 bynanci/storytelling 執行 #65（N-08／N-R08）。先讀 AGENTS.md、TASKS.md、docs/SDLC.md、ADR 0005 與 issue，核對最新 main 和進行中 PR；同票已有成果就接續。#49 契約已採納，與 #51 共用來源 unit_id、revision 與變體 ID。風格規則放入獨立版本的 profile，維持封閉 V1 相容。

從 $storytelling 的請求「讓鄰居知道週六修傘。給我紀實、意象兩個開場」開始。使用 skills/storytelling/references/workshop-demo.md。固定受眾、日期、要帶一把傘及素材 S1/S3；紀實版先給活動資訊，意象版先放兩聲敲擊，再揭露活動。輸出兩份可閱讀的稿、來源 ID、固定事實與作者可改的選擇。標出新增意象與作者提案，保留作者選、改或拒絕的地方。

交付可從正常本機入口重跑的一條結構規則。用明確的元素 ID 檢查指定變體內「線索早於揭露」；另一種變體可採相反順序。同一變體內互斥規則及未知 profile 版本各提供失敗 fixture、診斷與退出狀態。相同輸入與設定重跑兩次比對輸出。依 SDLC 先記錄功能 RED，再交 GREEN、影片回歸及同 head CI，在一個 PR 對照請求、稿件和重播命令。附去除流派標籤的兩稿與盲讀題；有真人回饋才記原話，無人試讀則保留 manual-pending。
```

## N-09

**錄製者可用的聲音提示表。** [Issue #66 · N-R09](https://github.com/bynanci/storytelling/issues/66)

```text
在 bynanci/storytelling 執行 #66（N-09／N-R09）。先核對最新 main、同票 PR、AGENTS.md、TASKS.md、docs/SDLC.md 及 #49 的 ADR 0005。沿用已有 ID／revision 與適合的解析邏輯；若 #50 口述稿已可用就取用，否則用已宣告的簡短旁白開始。新增資料用有版本的 cue sheet 表達，保持 NarrativeDocument V1 相容。

從 $storytelling 的請求「給錄音者一份兩段式修傘邀請：旁白、停一拍、兩聲敲擊」開始。沿用 workshop-demo.md 的 S1 及 skills/storytelling/assets/umbrella-workshop/taps.wav。輸出錄製者可照著操作的 cue 順序、角色／軌、來源或擬音、時間單位；區分已知時間、估時和未知。兩聲敲擊是合成音效；旁白是待錄台詞，未知長度保留未定。

先寫會失敗的 CLI 測試。測合法跨軌疊音、同軌區間首尾相接、已知反向區間、重複 cue ID、同一指定互斥軌上的衝突，以及時間未知時的判定。保存有效與無效輸入的輸出與退出狀態。重跑同一有效輸入，逐位元比對提示表並核對來源沒被改寫。將 Skill 請求、提示表、命令、RED／GREEN、回歸與同 head CI 放在一個 PR。驗收交付到可手動錄製的提示表。
```

## N-12

**實際拿到圖檔的宿主試驗。** [Issue #69 · N-R12](https://github.com/bynanci/storytelling/issues/69)

```text
在 bynanci/storytelling 執行 #69（N-12／N-R12，G0–G2 可行性）。先查最新 main、同票 PR、AGENTS.md、TASKS.md、docs/SDLC.md、#49 的資料邊界，以及 docs/migration/OPENFILM_RUST_PORT_MAP.md 中適用的 OF-37～39。若已有獲選的案例，延續原選擇。否則選「根據提供的修傘構圖交付 PNG」：一個已授權的本機宿主、一種輸出能力，可直接依 #49 開始。

用 $storytelling 接收「把工作檯示意構圖交成能打開的圖檔，保留左邊藍傘、右邊線軸」。採用 skills/storytelling/assets/umbrella-workshop/bench.svg 與 workshop-demo.md 的構圖要求。記錄宿主版本、權限、輸入、重播命令及產出檔。用解碼器重新開啟輸出的 PNG，核對實際尺寸、傘與線軸位置，記下讀回方法與觀察。若沿用語音轉錄案例，另備獲授權、有語音且有對照文字的合成音檔；套件內 taps.wav 只有敲擊聲。

用相同請求再跑一次宿主能力不可用的路徑，交付可用的構圖 brief（或該案例的來源描述），寫明尚缺能力。若兩條路徑中只有降級案例能跑，據實記錄阻礙、負責方與重跑條件，保留真實讀回待驗。交付一份可行性 PR，列出實際產物／回讀、降級產物、同 head 檢查，以及採用、縮小或暫緩的建議和依據；待決事項標明決策者。本票的工程範圍限於重現試驗。
```

## N-11

**一次改期，算出所有必須更新的產物。** [Issue #68 · N-R11](https://github.com/bynanci/storytelling/issues/68)

```text
在 bynanci/storytelling 執行 #68（N-11／N-R11）。先查最新 main、同票 PR、AGENTS.md、TASKS.md、docs/SDLC.md、#49 契約與 #51 的實作和重播證據。確認 #51 已能以穩定 ID／revision 查歷史並列出受影響變體。若仍缺這項能力，回報缺的入口、#51 的進度連結與可重跑的啟動條件，本輪停在這裡。#66 的 cue sheet 可選用。

從 $storytelling 的請求「修傘活動改成週日，請告訴接手的人哪些稿要改」開始。用 skills/storytelling/references/workshop-demo.md 的 S1 通知、S2 bench.svg 和 S3 taps.wav，建立本機唯讀、獨立版本的 manifest，對照邀請文、海報與兩版口述。區分檔案、作者描述、逐字稿、創作提案、待補素材。另跑只有描述或逐字稿的案例，交付仍能使用的部分。

先列預期影響集合再跑 #51：兩版口述日期、邀請文字與海報日期應更新；海報的左右構圖與 WAV 素材可沿用。逐項比對實際集合，列出漏報與誤報。給出不存在的來源 ID、有效 ID 配舊 revision、只屬海報的提案被另一版引用等反例；依宣告範圍與引用判斷。內容雜湊只用來核對位元組身分。先 RED 再 GREEN，驗證同輸入重跑報告一致、來源唯讀。用一個 PR 附前後對照、命令、影片回歸、同 head CI 與仍需作者檢視的語意差異。
```

## N-10

**把已選定的用途做成一條可用流程。** [Issue #67 · N-R10](https://github.com/bynanci/storytelling/issues/67)

```text
在 bynanci/storytelling 評估並執行 #67（N-10／N-R10）。先讀最新 main、同票 PR、AGENTS.md、TASKS.md、docs/SDLC.md、#49 與 #53。先貼出 #53 可追溯的採用決定，指出唯一獲選模式、作者需求與版本範圍。只有候選比較、推薦、決定暫緩或未定時，回報決策連結／缺口及重啟條件，維持 #67 待決，結束本次任務。若已有單一採用決定，將其記入 #67 並只執行對應的一行：

• 編劇：讓 $storytelling 保留事件 A→B，但讀者先看到 B 再看到 A。從正常本機入口重播兩種順序，拒絕指向不存在場景的引用。
• 分支構思：讓 $storytelling 保留 A↔B 的聯想和未決的兩個分支。從同一入口證明雙向聯想可通過、強制執行先後成環會被拒絕，且分支沒有被自動挑選。
• 故事接影片：讓 $storytelling 把一個選定單元連到已宣告素材，另一個缺素材的單元顯示未對應。從同一入口拒絕顯式綁定到不存在的素材，保留原始檔與剪輯專案。

沿用已採納的 V1 邊界與來源 ID；新模式使用有版本的契約。交付一個 PR，含作者可直接使用的請求與結果、所選案例的正反 fixture、命令／退出狀態、相同輸入的重播、功能 RED／GREEN、既有影片 V1 回歸與同 head CI。說明和現有文字樣板相比，這個入口實際檢查了什麼。其餘候選保留於 #53 供後續決策。
```
