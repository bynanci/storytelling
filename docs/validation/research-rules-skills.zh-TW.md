# 研究規則與技能交付

[English](research-rules-skills.md) · 繁體中文

日期：2026-11-11。基準：`b7fcf869d01727dcf408c457fdd42929dca80c7a`。範圍：原創研究摘要、可攜編輯規則、六個按需流程與雙語整合。不改 Rust、schema、依賴或授權。提交 PR 記錄測試 head 與 hosted checks 結果。

交付時已無衝突納入 main `60e302e2ea3b235e9bd7aae58049690ec9dad0f2`（#78 聲音提示）。對該 main 的 PR diff 仍只有文件。整合後已重跑本機文件／套件檢查。

## 合成代理演練

全新 GPT-6.1 Sol 代理只讀取 `research-workflows.md` 與其連結的 `editorial-rules.md`，未取得預期答案、其他 repo 文件、搜尋或媒體工具。下列為主持代理提供的任務與觀察摘錄，不是完整逐字紀錄。只驗證一個合成案例，不代表人工結果或普遍有效。

### W5 輸入

- S1 第 1 版：修傘工作坊週六舉行，帶一把傘。
- S2 第 1 版：兩下合成敲擊，無人聲。
- A 第 1 版廣播：「[播放兩下合成敲擊。] 本週六來參加，帶一把傘。」
- B 第 1 版海報：「週六修傘工作坊，帶一把傘。」作者已核准僅 B 使用黃色紙弧。
- C 第 1 版提醒：「明天，帶一把傘來工作坊。」預定週五發送。
- 來源表：A/B → S1；A → S2；未記 C。
- 修改：S1 第 2 版改為週日；仍週五發送；保留核准創作；交付 A/B/C 實際文字副本與交接；無媒體工具。

### W5 觀察摘要

| 稿件 | 回傳文字／觀察 |
| --- | --- |
| A | 「[播放兩下合成敲擊。] 本週日來參加，帶一把傘。」 |
| B | 「週日修傘工作坊，帶一把傘。」黃弧只留 B。 |
| C | 「本週日，帶一把傘來工作坊。」仍週五發送。 |
| 來源與交接 | A → S1 第 2 版／S2 第 1 版；B → S1 第 2 版；語意審閱後新增 C → S1 第 2 版。保留基準稿。只有文字／腳本，未製作或播放檔案。 |

代理找出未記錄的相對日期依賴，保留單版隔離，也未聲稱完成媒體製作。不代表其他間接依賴都能找齊。確切英文輸入與觀察摘錄見[英文紀錄](research-rules-skills.md)。

### W6 輸入與觀察

任務要求判斷是否已證明受眾察覺兩個開場、編輯改得更快。現有資料只有順序檢查、一位偏好 A 的模型評審與操作說明，尚未招募人類讀者或編輯。

代理回覆：兩項都未獲證實，均待驗證。提出隱藏標籤、平衡順序的閱讀測試與配對交接任務；人工結果欄保持空白。

審閱指出小幅改善：繼續決策須明列多改，以及漏改、誤改、未交。雙語 W6 已加入此條件。文字修正已直接審閱，之後未重跑演練。本紀錄不含作者核准、參與者觀察或維護者驗收。

## 六項審閱修正與新增證據

此次已透過 main `c1f164354c3e88d90acb32075828954091c149d8` 納入 #84。[八情境演練紀錄](research-review-rehearsals.zh-TW.md)列明題目、來源版本與實際回稿；三個全新代理涵蓋 W1–W6，未取得 Review 討論或主持人答案。均為合成觀察，不是人工驗收。

| Review | 修正與證據 |
| --- | --- |
| 1 跨分支評估 | ST-R12／ST-R13、W6 同步 rating／inference_status、舊欄 unassessed，以及兩組獨立交接比較；RV8。 |
| 2 間接影響 | ST-R09／W5 必查跨稿、算術、緊鄰詮釋及構圖遮蔽；RV2–RV4。 |
| 3 負面反推 | W5 改寫對比、因果、驗證範圍時回呼 W4／ST-R02，技能路由同步；RV5。 |
| 4 覆蓋率 | RV1 實際使用四個 W4 標籤，RV2 分離玩家／角色所知，RV7 驗證媒體交付誠實。 |
| 5 授權撤回 | ST-R08／ST-R14、W5 禁止用改述、換說話者或二次匿名保留撤回個人經驗；RV6。 |
| 6 追溯 | 研究表 P07、P05／P11 補 #84；歷史 CI #300、本機受阻與新 head 結果分開記錄。 |

規則修改後，本機 evaluation 11 項、套件安裝 1 項及 repo 檢查均通過；最終文件樹與 hosted 結果記於 PR #86。

## 自動驗證

執行後填入結果。既有套件安裝測試逐檔比較完整技能，包含新參考文件。repo 檢查也拒絕逸出安裝資料夾的連結。這次文件修改不新增模仿實作的測試。

- `python3 scripts/check_repo.py`：通過，含連結、來源、依賴界線與需求測試引用。
- `python3 scripts/test_skill_distribution.py`：1 項通過，含離線打包／安裝與完整逐檔位元組比對。
- `python3 scripts/test_storytelling_graph_plan.py`：6 項通過；`python3 scripts/test_storytelling_graph_trace.py`：5 項通過。
- `python3 tests/oracles/filmcraft-25-v1/test_checks.py`：6 項證據拒絕測試通過，未呼叫平台。
- `git diff --check`：通過。
- 本機 Rust 格式、Clippy、文件、workspace 與 release 測試：受阻；PATH 與檢查的標準位置皆無 Cargo/rustup。
- 歷史 hosted [CI #300](https://github.com/bynanci/storytelling/actions/runs/42990788126) 已在 `fde7f39828903f040eef1d00b7b289e4b8946bff` 全數通過：quality（含 fmt、Clippy、Rust 文件與 release 測試）、audit、Linux／macOS／Windows Rust 與 Required gates。這只證明該版的 hosted 驗證，不表示本機已執行 Rust。此次審閱修正會產生新 head；其最新 hosted 結果記在 [PR #86](https://github.com/bynanci/storytelling/pull/86)，不沿用歷史結果。

## 仍需人工驗證

以真實參與者進行既有 N-01 與獨立閱讀／交接協定。招募前凍結答案、分母與決策條件。收集實際改稿與第一反應，不只核取表。保留準備成本、多改與未完成嘗試。產品與發版決策由維護者作出。
