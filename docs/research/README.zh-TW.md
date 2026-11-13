# 研究、規則與技能設計

[English](README.md) · 繁體中文 · [專案理由](../WHY_STORYTELLING.zh-TW.md)

Storytelling 採 **一個可攜技能、六個按需流程**，搭配 14 個穩定編輯規則 ID。研究用來形成問題與防護，效益仍需本專案證據。本次增加 Markdown 指引，不新增 runtime 行為、schema、發版或人工驗收。

## 閱讀與貢獻入口

| 需求 | 標準文件 |
| --- | --- |
| 查證依據與存取限制 | [11 篇論文與 1 項標準](../../skills/storytelling/references/research-foundations.zh-TW.md) |
| 套用或審閱規則 | [ST-R01–ST-R14](../../skills/storytelling/references/editorial-rules.zh-TW.md) |
| 起草、比較、改編、審核、修訂、評估 | [W1–W6](../../skills/storytelling/references/research-workflows.zh-TW.md) |
| 載入可攜套件 | [技能入口](../../skills/storytelling/SKILL.zh-TW.md) |
| 查閱交付證據 | [驗證紀錄](../validation/research-rules-skills.zh-TW.md) |

契約、編輯與人工檢查的權限不同。作者可拒絕偏好建議；宣告檢查通過，不代表事實支持、身分、權限、美感或受眾反應。技能內參考文件是標準版本；`rules/` 只提供入口，不維護第二套規則。

## 研究 → 規則 → 流程 → 既有任務

最後整合以 main `8e2c08094f6bce70a7507e11ed84265bd98e3820` 為基準，保留已完成的 N-05／N-03 請求者審閱、N-06 明確暫緩、具體比較案例與 N-07 試用準備包。既有紀錄及執行路徑均保留。下方早期快照是歷史設計對照，不能據此重開已完成票，也不能把準備工作推成參與者結果。

審閱修正已納入 main `c1f164354c3e88d90acb32075828954091c149d8`，其中 [PR #84](https://github.com/bynanci/storytelling/pull/84) 來源 head 為 `ec4a879ccbf2ea1b04288d2f0fbec09b94f8dcce`。W6 已同步獨立 `rating`／`inference_status` 軸與 mapped–mapped-bounds 交接比較；W5 補齊算術、緊鄰詮釋、授權撤回與 W4 回查。這些是合成驗證及協定修正，並非參與者結果。

快照：2026-11-11，基準 `b7fcf869d01727dcf408c457fdd42929dca80c7a`。本表是設計對照，並非即時 issue 狀態。N-03 輸出、N-04 直接引用影響、N-08 宣告口述順序與 N-11 交接已有 repo 實作，人工結果仍須另驗。初次審閱時 PR #78（聲音提示）與 #83（商業證據審核）仍開啟。交付期間 #78 以 `60e302e2ea3b235e9bd7aae58049690ec9dad0f2` 合併，本分支已納入且未修改其 runtime 檔案。N-09 現有宣告提示表檢查；實際成音與人工結果仍另需證據。實作前重查 #83。

| 證據 | 規則 ID | 流程 | 需求／既有 issue | 下一項必要證據 |
| --- | --- | --- | --- | --- |
| P01 事件索引；P02 意圖規劃；P06 大綱控制 | ST-R04–ST-R06 | W1 意圖；W2 替案 | N-R03 / [#50](https://github.com/bynanci/storytelling/issues/50)；N-R06 / [#53](https://github.com/bynanci/storytelling/issues/53)；N-R10 / [#67](https://github.com/bynanci/storytelling/issues/67) | 先比較揭露、開放分支與缺素材影片，再選原生設定。 |
| P03 敘事視覺化；P09 多媒體學習 | ST-R06、ST-R07、ST-R12 | W2；W3 改編；W6 評估 | N-R08 / [#65](https://github.com/bynanci/storytelling/issues/65)；N-R09 / [#66](https://github.com/bynanci/storytelling/issues/66) | 盲讀第一反應與獨立聲音製作檢查；保留無障礙需求。 |
| P04 CoAuthor；P08 人機互動；S01 PROV-DM | ST-R01、ST-R03、ST-R08、ST-R11、ST-R14 | W1；W3；W5 修訂 | N-R04 / [#51](https://github.com/bynanci/storytelling/issues/51)；N-R11 / [#68](https://github.com/bynanci/storytelling/issues/68)；N-R12 / [#69](https://github.com/bynanci/storytelling/issues/69) | 實際作者選擇與檢視過的產物；來源宣告不授予權利。 |
| P07 原子事實支持 | ST-R01、ST-R02、ST-R10 | W4 主張審核 | N-R05 / [#52](https://github.com/bynanci/storytelling/issues/52)，相關 PR [#83](https://github.com/bynanci/storytelling/pull/83) 與 [#84](https://github.com/bynanci/storytelling/pull/84) | 審閱者逐項對照原文，保留限定與缺證據。 |
| P05 視覺修訂；P11 建置系統類比 | ST-R09、ST-R10、ST-R13 | W5；W6 | N-R04 / [#51](https://github.com/bynanci/storytelling/issues/51)；N-R07 / [#54](https://github.com/bynanci/storytelling/issues/54)；[#84](https://github.com/bynanci/storytelling/pull/84) 來源邊界／校準 | 與一般筆記比較實際改稿、間接影響與準備成本。 |
| P10 模型評審偏差 | ST-R12、ST-R13 | W6 | N-R01 / [#48](https://github.com/bynanci/storytelling/issues/48)；N-R07 / [#54](https://github.com/bynanci/storytelling/issues/54) | 真實人工觀察、平衡順序與明確分母；模型演練屬合成測試。 |

## 優先順序與停止決策

1. **先用既有契約。** 用 W1–W5 處理目前稿件，保留來源修訂與作者選擇，分開語意判斷。沿用 #52/#65/#66/#68，不開重複實作 issue。
2. **先取人工證據，再擴大 runtime。** N-01 與獨立閱讀／交接練習依各自凍結協定進行。記錄完成／嘗試、A/B 第一焦點評分與獨立的有據／無據／未評估推論、漏改／預期改稿、多改／收到且不需改的稿件、誤改／收到稿件、未交／要求稿件。notes–mapped 與 mapped–mapped-bounds 比較分開，後者主看誤改／已交回稿數。操作時間與準備／製圖／整理邊界時間分開；零分母填不適用。
3. **透過 N-06 最多選一個下一階段原生形式。** 選定後才推進 #67。#69 生產 adapter 決策與窄範圍可行性試驗分開。
4. **有證據再拆套件。** 獨立觸發、自足參考、獨立合成評估與維護者齊備後，才拆技能。一般 Markdown 使用不增加安裝或付費依賴。

招募前凍結繼續／縮小／修訂／停止條件。範例協定門檻是本地先導決策，不是論文效果量。若一般筆記已足夠、建圖成本大於效益，或漏改、多改、誤改、未交變差，應重估設計。不以文件核取方塊計算產品完成率。

## 開源界線

原創指引與合成範例沿用 MIT；既有 Apache 元件與告示保留其適用範圍。引用不授予散布論文 PDF、圖片、資料集、權重或程式的權利。未來匯入前記錄來源、修訂、授權與告示。本次僅加原創摘要與連結，不改授權。

npm 已包含 `references/*.md`，英文與繁中參考文件隨既有套件散布。複製技能中的本地連結均留在資料夾內；repo 協定使用公開 GitHub URL。核對離線打包／安裝與 repo 連結檢查。本次設計不含個人技能安裝、registry 發佈或 release tag。

## 評估層次

- **自動：**repo 連結／來源界線、完整離線套件安裝與既有 graph helpers。即使只改文件，Rust 檢查仍單獨記錄。
- **合成代理演練：**測間接修訂、單版隔離、產物誠實與缺乏人工觀察。記錄輸入與行為；模型不算參與者。
- **人工：**觀察目標語言、受眾與完成媒介，分開作者與接收者角色。沒有資料即待驗證。
- **維護者：**依紀錄決定範圍、驗收與發版。書目或 CI 通過不構成核准。

## Gemini R1技能評估紀錄（SK-EVAL-R1）

[40案模型評分報告與視覺化](evaluations/2026-11-13-gemini-r1/README.zh-TW.md)保存原始數值、來源勘誤與宣告分組限制。這是W4/W6的診斷觀察，不新增已驗收需求、真人結果或完成的技能改善。
沿用[#115](https://github.com/bynanci/storytelling/issues/115)總追蹤。[SK-E01／#118](https://github.com/bynanci/storytelling/issues/118)已合併。[SK-E02／#119](https://github.com/bynanci/storytelling/issues/119)新增[版本化證據計畫、裁定及資料包勘誤契約](evaluations/sk-e02/README.zh-TW.md)與合成完整性測試；不補造 R1 來源脈絡，也不產生新的評估結果。[#120](https://github.com/bynanci/storytelling/issues/120)、[#121](https://github.com/bynanci/storytelling/issues/121)及[#122](https://github.com/bynanci/storytelling/issues/122)仍為各自獨立工作。既有N與M驗收、#54處置及#112真人門檻維持各自證據範圍。
