<!-- SPDX-License-Identifier: MIT -->
# 子 agent 情境驗證 · 2026-11-17

[English / 完整範圍、收據核對命令](subagent-scenarios-2026-11-17.md) · [原始資料清單](subagent-scenarios-2026-11-17/artifact-manifest.json) · [覆蓋彙整](subagent-scenarios-2026-11-17/coverage-overlay.json)

依請求者指示，本輪從 base `91bf8f80139f59c0406340eb3565d296bd562931` 啟動六個獨立提示的 AI 子 agent，執行作者、改稿接手者、主張查核、日誌審查、擷取工具審查及門檻盤點。以下是代理實際輸出，不是主代理事後編寫的角色對話。**新增真人與 Host 觀察均為 0**。分開脈絡仍共享檔案系統／模型環境，不是強制盲測或獨立人口樣本。Session 宣告模型為 `gpt-6.1-sol`；serving revision 與抽樣設定不可得，沒有另指定 model override。

| 角色 | 實際執行與原話 | 保留的意外結果／限制 |
| --- | --- | --- |
| 作者 SIM-AUTHOR-01 | [完整回覆](subagent-scenarios-2026-11-17/responses/author.txt)：只收作者卡與兩份留空檔，實際執行四次 CLI，交回四題答案。 | 發現 V1 錯誤 path 沒有 wrapper 的 coverage 層、錯誤未附範圍原因。用 Python 編輯而非文字編輯器、未開始連續任務計時，均保留，不能當作者耗時或真人可用性結果。 |
| 接手者 SIM-EDITOR-01 | [完整回覆](subagent-scenarios-2026-11-17/responses/editor.txt)：從已發表的合成開窯材料重演兩張獨立改稿卡，交回六稿。 | 保留「這一版」推定為 B 的不確定性，以及 C 的「目前缺少」未限定本批交件的含混；沒有改成通過答案。 |
| 主張查核 SIM-CLAIMS-01 | [完整回覆](subagent-scenarios-2026-11-17/responses/claims.txt)：十次實際 CLI，九份報告成功、一份預期重複引用拒絕。 | current 引用仍可能連到過時措辭或過度推論；support_status 仍需人判讀。有限範圍未發現產品契約違反。 |
| 日誌 AUDIT-JOURNAL-01 | [原始審查](subagent-scenarios-2026-11-17/responses/journal-audit.txt)提出歷史 readback 重送問題。 | 主代理以實際 SQLite 測試重現意外 `Ok(2)`，由 [PR #146](https://github.com/bynanci/storytelling/pull/146) 修正；沒有 Host 呼叫或實際媒體效果。 |
| 紀錄工具 AUDIT-CAPTURE-01 | [原始審查](subagent-scenarios-2026-11-17/responses/capture-audit.txt)以一致錯誤區間證明舊重播可能誤通過。 | [獨立 oracle 修正](n10-06a-replay-oracle-2026-11-17.zh-TW.md)補精確數學、JSON 型別與故障測試；原生轉換器本身沒有觀察到此缺陷。 |
| 門檻 AUDIT-GATES-01 | [原始盤點](subagent-scenarios-2026-11-17/responses/gates-audit.txt)：8 領域×6 類別、71 個 source 引用及未檢視項。 | 原審查當時的 pending-role 註記原樣保留；後來的實際產物另放彙整，不回頭改寫其觀察。 |

真實脈絡依來源保留：[VH-01-S3](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001) 是舊的維護者自操作證言；[開窯材料](../../examples/discovery/kiln-coverage.zh-TW.md)明確是合成，曾有真人試讀不會讓素材本身變成實拍事實；[M3-03](m3-03-filmcraft-execution.md)是真實隔離軟體執行，但 child 已成功退出後才扣留回應。新日誌失敗序列是合成延伸，不是新觀察到的 FilmCraft 事件。未知真人、私人事件與作者決定沒有補造。

## 功能覆蓋與修正

八領域為 A 精確輸入、敘事版本、商業主張、handoff、Planner／OTIO、Review／Approval、SQLite Journal 與研究擷取；六類別為 success／error／replay／identity／limit／interrupt。彙整將 56 個具名 source 錨點對上實際 workspace 通過 log；這是追溯，不是覆蓋百分比或所有可能情境均驗證的證明。四次 source-bound debug／release 測試各通過 319 項預設／338 項全 feature，各保留原有三個 ignored。[日誌收據](m3-02-historical-readback/workspace-checks.json)保留實際命令與 log；最終 head／合併後 CI 另見各 PR。

新日誌測試涵蓋重啟、八種後續狀態、readback 預算與 operation／attempt／fence；不認證 caller 的 Host 證據。重播 oracle 的首批 18 個失敗 subtest 中，16 個是真誤通過，兩個參照案例原已由後續 V1 拒絕；獨立 review 已校正文案。後續 preview-only bool／float 探針再指出 Python 相等比較的型別別名問題，必須以 JSON 型別敏感比較檢查固定參照及 preview／emit／V1 一致性。原失敗與收據分開保留，不當成原生轉換錯誤。

仍未驗證：六個唯讀領域的部分程序中斷／partial stdout／broken pipe、handoff collection／path／text／depth 的部分精確邊界、選定 revision／claim 上限、真人操作、實際媒體身分與內容、正式 Host in-flight／quiescence／復原。機器結果不能據此外推。

## 原始資料與後續門檻

[artifacts.zip](subagent-scenarios-2026-11-17/artifacts.zip)保存 143 個原始 packet、輸入版本、回覆及 tool trace。Markdown／程式檔加 `.txt` 作為資料保存，bytes 不變；六份回覆另有可讀預覽。雜湊與路徑映射見 manifest，可用英文版命令直接讀 zip 核對，無須解壓或執行其中腳本。

代理輸出只用來提出失敗情境與功能測試，沒有計算模型成功率、真人完成率、省時或因果效益。研究角色的 process time 不是作者時間，模擬產品資料中的「3 個團隊」也不是本輪真人。

#103 W01、#107 八個唯讀 SDK 入口、#129 B bridge／排序／escaping 仍需明確技術採納；代理沒有代簽。#104–#106 的正式執行／復原／Host、#108–#113 的 consumer／第二宿主／真人／G5 依賴保留。#121 原始資料不可得，這些輸出不冒充 R1；#122 前置資料未滿足，未建立 holdout。#129 獨立作者、#112 五人 Beta 及既有 #54 處置都不由這輪模擬取代。
