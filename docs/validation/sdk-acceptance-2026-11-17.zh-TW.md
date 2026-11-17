<!-- SPDX-License-Identifier: MIT -->
# 唯讀 SDK 驗收與議題收斂 · 2026-11-17

[English／完整條件對照與收據命令](sdk-acceptance-2026-11-17.md) · [SDK 指南](../SDK_CONSUMER_V1.zh-TW.md) · [機器狀態快照](sdk-acceptance-2026-11-17/post-disposition.json)

八個唯讀 Rust V1 入口已完成工程驗收。[PR #149](https://github.com/bynanci/storytelling/pull/149) 已轉 Ready 並合併為 `9d15382045fb7453827accda772c56ddc0abee44`，與驗證 head `5bf480c017f1d27cce307ae5931470d772bef5d6` 的 tree 相同。**#107 全部原始條件已勾選並以 completed 結案；Open Issue 由 16 收斂為 15。** 這是 GitHub 狀態數量，不是產品完成百分比；已接受的 Rust 介面不等於套件已發布或 Host 執行已驗收。

## 實際授權與工程證據

請求者先採納精確的 ADR 0013 G2 候選，再提出：

> 幫我後續驗收 並收斂issues

Codex 依此逐條驗收，使用儲存庫既有「完成且同 head 檢查通過可合併」授權執行。舊 Draft／Open 限制是先前 G2-only 階段，後續請求推進了該階段。[原 G2 決定](m4-01-sdk-g2-decision.md)保留原樣，不改寫成當時已授權 merge。這是委託工程驗收，不是代簽、DCO、獨立真人安全審查或真人 SDK 試用。

另位 AI [逐條驗收](sdk-acceptance-2026-11-17/sdk-acceptance-audit.txt)確認八項條件與技術性 consumer 出口均符合：版本化公開介面、完整原錯誤與結果、八個委派入口、無執行／復原能力、ownership／並行／未來單一寫入者界線、CLI／SDK 同 fixture 等價、未知版本拒絕、遷移／棄用規則。獨立 standalone coverage 範例加上只匯入 facade 的八入口 integration consumer 提供相容性證據；#107 沒有要求新真人研究才可完成這個唯讀範圍。

本輪[實際重跑](sdk-acceptance-2026-11-17/fresh-verification.json)：all-feature release **369 通過／0 失敗／3 個既有 ignored**、SDK **14/14**、獨立 consumer、fmt、Clippy `-D warnings`、rustdoc `-D warnings`、repository 與乾淨 tracked diff 全通過。ignored 是兩個資訊性 benchmark 與需要固定 FilmCraft 的隔離案例，沒有執行或算成 Host 證據。

原 fresh receipt 記 argv，未記環境變數。另以[append-only 環境註記](sdk-acceptance-2026-11-17/invocation-environment.json)轉錄實際控制腳本的明確 toolchain／target 覆寫，包括第 5 步的 `RUSTDOCFLAGS=-D warnings`。這是主代理轉錄，不是獨立環境 dump 或重跑；原 receipt／streams 不改寫。

| 來源 | 六項一般 CI | 三平台資料包檢查 |
| --- | --- | --- |
| 最終 head `5bf480c` | [44069823511](https://github.com/bynanci/storytelling/actions/runs/44069823511)，全成功 | [44069823440](https://github.com/bynanci/storytelling/actions/runs/44069823440)，全成功 |
| 合併後 `9d15382` | [44074036241](https://github.com/bynanci/storytelling/actions/runs/44074036241)，全成功 | [44074036178](https://github.com/bynanci/storytelling/actions/runs/44074036178)，全成功 |

## 剩餘十五項與恢復條件

負責者是角色，不是新增 GitHub 指派。[原十六項盤點](sdk-acceptance-2026-11-17/issue-convergence-audit.txt)與[前置快照](sdk-acceptance-2026-11-17/open-issues.json)完整保留；其 #107 尚待最後決定的描述，早於本次實際驗收／合併。[後置快照](sdk-acceptance-2026-11-17/post-disposition.json)記錄後來的 closed 狀態，沒有回頭修掉原觀察。

| Issue | 未滿足條件／負責角色／何時可繼續 |
| --- | --- |
| #103 | 維護者先決定精確 W01 範圍；工程師提供無 Host 的確定性 RED／GREEN，之後還須完整 authority／provider／fencing／readback 契約驗收。提案合併不等於通過。 |
| #104 | #103 接受後才實作正式單一寫入者、dispatch／authority／artifact 綁定與實際 readback，保留隔離 Host 證據。 |
| #105 | #104 後補 persisted reconciliation 與各 timeout／crash／晚回應案例，unknown 持久保留；只有新鮮、明確 NotApplied 才能重試。日誌維護修正不是正式復原器。 |
| #106 | #104／#105 完成後，由實際 operator 提供 build／OS／profile／project／review／receipt 矩陣與內容對照，維護者決定 accept／narrow／stop M3。 |
| **#108** | **現在可做唯讀 transport G2**。工程／維護者先選 binding、操作／view、framing／version／error／原 bytes／失敗規則，再做 TS 等價與 fresh-checkout 範例；唯讀不必等 #106。 |
| #109 | 可先規範唯讀 app 的 UI／client；完整 issue 仍缺 app、單一寫入者、持久 receipt／readback、重啟及 lost-reply demo，執行須等 #106。選 TS 才需要 #108。 |
| #110 | 選配唯讀 tool 設計可先規範，尚無已接受 schema；真人核准／執行與反自我核准／unknown 重試案例仍依 #109／#106。不阻擋 #108。 |
| #111 | 可用 #109／M3 工作流後，維護者選一個第二宿主，由 operator 實際匯入／播放／儲存／重開／讀回並列損失；目前沒有已合格目標。 |
| #112 | 先滿足 #109／#111／M3，試用前凍結 build／素材／任務／評分，再招募真正五位獨立使用者。舊讀者或模型不能充當 Beta 參與者。 |
| #113 | 相依完成後，release engineer 彙整版本綁定的乾淨安裝、相容性、遷移、回退、復原、安全與試用證據；G5 發布另外決定。 |
| #114 | #107 child 已勾選，父議題維持 Open，直到正式執行／Reference Flow／第二宿主／Beta／RC 出口成立或明確另作處置。 |
| #115 | 五份模型原文與兩位 AI 評閱已交付；F02 是未執行預檢。評估維護者先釐清／版本化 F01 題目與判準爭議，再重跑或改規則；#121／#122 仍缺證據。 |
| #121 | 原始研究資料管理者須提供已授權原草稿／資料包／提示／量表／模型設定，才可原樣反序複評；不能重建成 R1。 |
| #122 | 研究負責者須先有合格 #121 pilot variation 與有依據的指引修訂，再於觀察前凍結未見案例、量測、樣本、停止及分析規則。 |
| #129 | A 工程已完成，獨立真人操作仍缺；B 須自己 G2 再實作；C 須隱私／G2／可信本機 read view／實際檔案綁定；D 五條證據線獨立，Host／authority 依 #103–#106。 |

下一個聚焦交付是 **#108 唯讀 binding G2**，先選 transport 與支援 view，保留 rational strings、選定 proposal／bundle／OTIO 原 bytes、完整來源診斷、未知版本拒絕及 bounded output／取消／程序失敗。Narrative projection 刻意沒有 core JSON serializer，bridge 必須另定 view，不能解析 CLI 散文或靜默擴充 V1。這是待決設計，不是已採納 transport；#103 W01 與 #129 B/C 各有自己的技術決定。

本次新增真人、媒體探測與 Host 觀察均為 **0**。沒有補造作者原話／耗時、R1 原資料、當前授權、Host 成功或 G5。unknown 不授權直接重送或 Undo。本次後續 PR 僅改狀態與追溯，不改 Rust／CLI／Schema、原 fixtures 或分數；英文版提供收據核對命令，該 PR 另保留最終 head／合併後 CI。
