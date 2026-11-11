# 審閱修正：跨情境合成演練

[English 與原始輸出摘錄](research-review-rehearsals.md) · [交付紀錄](research-rules-skills.zh-TW.md)

日期：2026-11-11。三個全新 GPT-6.1 Sol 代理依修正後的技能與研究指引執行八個情境，未收到本次 Review、預期答案或主持人評分表。這是合成代理演練，不是人類讀者／編輯試驗，未量測改善幅度、完成時間或效果量。

原始案例來自已納入的 main `c1f164354c3e88d90acb32075828954091c149d8`（[PR #84](https://github.com/bynanci/storytelling/pull/84)，來源 head `ec4a879ccbf2ea1b04288d2f0fbec09b94f8dcce`）。代理只能讀指定 README、A/B 稿與技能指引；未讀 keys、來源邊界表、校準檔、先前驗證或父對話。W3/W6 只使用題附素材。主持代理收到回稿後才對照原始來源、修改卡及適用評分鍵。沒有改寫原始讀者資料包。

[英文紀錄](research-review-rehearsals.md)保留任務輸入、來源版本及原始繁中輸出摘錄，W5 四案包含完整 A/B/C 交回文字（格式正規化）。這不是完整工具逐字紀錄。W5 演練後另將 `contradicted` 措辭釐清為「證據與待審主張不相容」，避免把負面測試結果本身當成審閱標籤；W4 原已有該定義。此最終措辭經直接審閱，未另做獨立重跑。

| 情境 | 流程／修正 | 實際觀察 | 限制 |
| --- | --- | --- | --- |
| RV1 修門來源與四個主張 | W1/W4；ST-R01、ST-R02 | 初稿先交付；Mei 扶門為 supported、Mei 鎖螺絲為 contradicted、單次關門證明耐久為 insufficient、30% 成本為 unavailable。I01 與 C01 未混入事實。 | 未執行 FActScore；沒有真實計畫成效。 |
| RV2 渡口 R2 | W2/W5；ST-R04、ST-R09 | 同一拍先顯示碼頭再切全幅黑底字卡，維持第一聲鐘響前揭露；玩家先知道，伊沃第二聲拆紙條才知道。 | 是演出提案，未實作遊戲或試玩。 |
| RV3 UX R1 | W5→W4；ST-R02、ST-R09 | A/B/C 引語都更正；A「對按鈕的理解」退縮為當時傳送狀態的理解；操作序列與追問欄保留。 | 只核對這次回稿，無普遍漏改率。 |
| RV4 account-review R1 | W5/W4；ST-R09 | A/B 同步為 9,200 與 8%，C 原樣；搬遷因果仍待查。 | 算術與來源範圍核對。 |
| RV5 open-source-release R1 | W5→W4；ST-R02 | Linux 匯入與 Windows 編譯分述；沒有反推 Windows 匯入失敗或不支援；C 保留兩秒。 | 文字口播，未製作影片。 |
| RV6 nonprofit-report R1 | W5；ST-R08、ST-R14 | A/B 移除原話及個人經驗改述，保留 18 人次、B 邀請及 C；未混用獨立 R2。 | 目前同意範圍檢查，不是法律判斷。 |
| RV7 audio-description | W3；ST-R01、ST-R11 | 保留觀眾左側、對白空檔與 00:08 動作；半秒停頓列提案，六秒能否容納仍待試讀。 | 沒有錄音、混音或播放驗證。 |
| RV8 雙軸評分與舊紀錄 | W6；ST-R12、ST-R13 | 額外因果斷言為 unsupported；首焦 match 須有凍結答案。舊 match 保留，缺推論欄補 unassessed。notes–mapped 與 mapped–mapped-bounds 分開，後者主看 damaged/received。 | 校準回答為構造輸入，不計入實證分子／分母。 |

這八題涵蓋 W1–W6，但只是指定反例的行為證據。沒有參與者數、真人完成時間、條件間 damaged 差異或統計效果。原先兩題演練仍作為歷史紀錄保留。未重跑修正前代理基準，不能把這些觀察當成改版前後錯誤率比較。
