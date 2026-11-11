<!-- SPDX-License-Identifier: MIT -->
# N-07 主持人腳本：普通筆記與實際 CLI

版本：`n07-native-notes-v2-proposed`。需求：N-R07／#54。這是 N-01 的**獨立 native round**，不是原本的 Skill 代理條件。當前可用真人紀錄為零；判定為「待真人證據」。#48 的判準同意紀錄仍缺，準備材料不等於已批准或執行試用。

## 開始前固定

1. 讀取[原 N-01 判準](../../../docs/narrative/N-01-AUTHOR-STUDY.md#measures-and-decisions)，由維護者在私人 agreement.csv 或 #48 記錄本腳本雜湊、是否同意將原判準用於 native round、下列歸因定義及退出處理。尚未同意時只做工程 dry run，不能做產品 continue／stop 決定。
2. 招募五位沒看過此題的作者。只用匿名碼；記錄既有 CLI／JSON 經驗。預定三人 notes→CLI、兩人 CLI→notes，主持人以事先記錄的 seed 打亂分配。預定五人不是已完成樣本；實際資料才填入 forms。
3. 固定繁體中文、同一電腦／編輯器及時間上限。記錄 OS、編輯器、二進位雜湊、來源 commit、材料 commit／SHA。兩條件均不用遠端模型；若另跑 Skill，獨立條件、版本與結果，不合併。
4. 使用原始 N-03／N-01 材料，紅傘 C01 在 A。不能換成新增 C02 的 reviewed 文件，也不能換成會移傘、改角色的 N-04 完整歷史。兩條件的 shared 原稿完全相同；CLI 額外拿到同詞的結構化模板，沒有填妥的更正答案。
5. 保存凍結原稿。每人每條件複製新的私人 participant-kit 工作資料；主持人保留 facilitator/key.md、主持人專用完成版文件快照／document-corrected.json、工程輸出與預期影響集合。不要讓作者在整個公開 repo 裡找評分答案。給兩臂同樣五分鐘介面說明與不同題的練習，實際說明時間另記，不算入八分鐘。

## 每個條件的八分鐘

主持人宣讀 shared/change-card.md，作者讀完後啟動連續計時；保存並提交修訂包才停止，八分鐘到即停。作者耗時包含看資料、思考、輸入、CLI 等待、錯誤恢復與保存。不能扣掉 CLI 執行時間來讓 native 組變快。

- **notes：** 在 shared 工作複本改來源卡、三份 narrative-drafts、commercial-draft 及 answers，不使用 Storytelling。
- **CLI：** 保留凍結的 base-document.json，在 working-document.json 建立 document_revision 2。依 CLI-instructions 修改來源、精確 revision 引用與三份變體文字，再用 render-narrative 產生實際三稿；不必再抄回 Markdown。修改 copy-template.json 並保存 audit 報告與 answers，不再抄同詞到 commercial-draft.md。

工具只組織宣告與引用，不會把改名自動重寫進草稿，也不判定 commercial current 引用是否足夠。JSON 編輯、精確 revision metadata 與手動重複的工作正是要量測的成本，不可由主持人先完成。I01/C01 的文字、origin 與範圍受保護；其 lineage 及 revision metadata 應隨來源更新，不能宣稱整個 unit payload 未變。metadata 編輯可另記次要成本，不算成同一更正事實的每次重複輸入。

每臂都要交回實際口述大綱、兩份單幅插畫 brief 及文案判讀。這輪測的是文字／brief，不是實際錄音、圖像成品或剪輯器操作。內容相同才可比較，不要求兩臂檔案格式相同。

## 主要與次要計分

主要完成定義沿用 N-01：480 秒內修正 S01／S02 和三份草稿姓名，保留 S03／S04、角色動作與 C01 的 A-only 範圍，正確說明 C01、I01、U01。實際收到的三份稿件逐份檢查；漏交不是零錯誤。商業判讀的 U01 回答仍是原門檻的一部分。

商業文案另外記錄：兩項事實主張中完成判讀幾項／2；兩項提案明確標示幾項／2；是否交付可追溯的處理理由。額外明細是次要診斷，不偷加成原先全包完成的必要條件，也不把 4/5 原門檻套用在四個 claim／proposal 分母上。若要單獨做產品商業案例或 V2 試用，事前另訂 protocol 和條件。

| 欄位 | 記錄方法 |
| --- | --- |
| 樣本與完成 | 分別報告可用作者數、開始嘗試數、完整配對數、完成／開始數。主要判斷需要預定五位的可比較配對。 |
| 作者修改時間 | 從讀完卡到保存提交的秒數。timeout=未完成，分析時間固定 480；原始經過時間也保留。退出另列，沒有事先退出規則就不補成零或排除後決策。 |
| 歸因錯誤 | C01 當觀察、I01 當事實、U01 當有佐證、C01 外溢。保存 offending clause、artifact/source ID、錯誤類型與評分者。偽造紅傘來源或 U01 依據為 major；另報至少一項 major 的作者數。 |
| 重複輸入 | 同一更正事實在第一次中心更正後的手動重寫次數，排除工具自動產生文字。每位／每條件記錄。 |
| 重工 | 因漏改、錯誤、遺失或不必要變動而返工的 episode 次數與理由，與純粹創作選擇分開；不擅自加入原門檻。 |
| 匿名回饋 | 保存作者自己的原句，別把主持人的推測填成回饋。先提交再問哪步最不清楚、重複或有幫助。 |
| CLI 時間 | 另用 cli-calls.csv 與工程 helper 結果。process wall time 含啟動／I/O，build／暖機另記；毫秒不是作者秒數或完成率。 |

主張與輸出的分母另列：三份 narrative outputs、兩個 factual claims、兩個 proposals。不要把一個參與者的多份稿件當多位作者。爭議由第二位評分者核對原稿與提交稿；未裁定時保留 missing／unassessed。

## 對照原先門檻，不把空白判成失敗

以下數字逐項保留 #48；原條件 B 是 Skill，轉用 native round 的同意必須先記錄。建議在收案前固定「不比 A 差」為 B 的歸因事件總數及 major 作者數均不高於 A；這是待同意的操作定義，不是已獲批准的改寫。

- **繼續：** B 至少 4/5 完成、無 major 歸因錯誤、歸因不比 A 差，且至少一項中位數改善：修改時間減少 20%，或每位少一次重複輸入，或每位少一次歸因錯誤。
- **停止：** B 少於 3/5 完成、至少兩位有 major，或品質、時間、重複及歸因完全沒有可觀察改善。
- **縮小再試：** 不屬以上兩者；保留有證據受益的範圍，重跑前另訂新例。小於 20% 的時間改善不夠 continue，但不等於「完全沒改善」。

五份有效配對、協議或關鍵欄位不足時先記「待真人證據」。blank 不是零秒／零錯誤；CLI success、例題通過、人工單次讚同都不能替代配對結果。先分條件彙總再比較；timeout 480 必須進入 N-01 主要時間中位數，不能改用只含 finished 的補充 scorer。

同題第二次有記憶效應：使用新複本、保留實際先後順序／period、兩臂之間不給評分答案，報告順序效應而不宣稱已消除。要換匹配新案例，先記為另一版協議。

## 真正失敗到合成回歸

目前沒有真人失敗可轉入 regression。工程預演發現同一 N-01 改名若連續送出 S01／S02 source_revision，第二筆會被既有 revision.pending_review 拒絕。這是已接受且已有測試的護欄，不先叫作 runtime defect。已保存合成重現；主試用因此改用可執行的版本化文件快照。獨立 S01／S02 影響探針只是工程控制，不冒充一份原子合併日誌。未知版、懸空引用、自我確認等也只是已知工程負例。

收到觀察後依序保存匿名錯誤原句與版本、重建不含私資的最小合成輸入、獨立寫預期／實際、先跑失敗再修正，連到具名 issue／測試。若屬語意判讀或已接受的 direct-only 邊界，先回到 G0/G1，不把它誤報成 parser defect 或直接加入傳遞失效。沒有可重現輸入就先記觀察與證據缺口。

目前下一個最值得先設計的問題是 **N07-UX01：如何保留來源歷史並完成一次多單元更正，而不偽造中間 review**。工程重現已存在，但尚無使用性或效益證據；先做 G0/G2 邊界決定，保留護欄，勿逕自改成傳遞失效或自動接受。作者在 snapshot metadata／多稿重複輸入及誤讀 current 的風險仍須由真人資料判斷，不因工程毫秒數先宣稱改善。

## 你需要做的最短步驟

1. 在 #48 記錄是否接受現有門檻轉用此 native round、腳本雜湊及評分／退出定義；有變更先版本化。
2. 主持人準備五位新作者、3 AB／2 BA 的隨機分配、每臂新私人副本及固定 CLI。先做一次工程重跑，別給作者看 key。
3. 每人完成 notes、CLI 各八分鐘並提交原始產物；填私人 forms，保留 timer、匿名回答與 reviewer 判讀。CLI 時間另存。
4. 回傳匿名彙總與必要的匿名片段：每臂樣本／完成分母、包含 timeout 的時間中位數、錯誤類型／major 人數、重複輸入與重工；再由維護者寫 continue／narrow／stop 及原因。

這份準備包沒有推薦 release candidate。G5 的相容性、遷移、回退、風險與明確維護者發布決定仍是獨立工作。

## Matched snapshot adaptation / 同材料版本說明

The study baseline is `n07-door-repair@1`, derived from the original N-01/N-03 source units. Units S01–S04/I01/C01 are identical source-unit objects (text, origin, revision and based_on) to the original. The two brief texts are adapted to the N-01 framing without the old N-03 unsupported indoor background; no C02 is introduced. Both arms receive exactly the same rendered starting words. This is an explicitly prepared native-round adaptation, not a claim of byte-identical original N-01 draft wording or prior protocol agreement. Metadata edits are timed, the original corpus stays unchanged, and original N-01 thresholds are not revised after observations.

主試用使用相同的衍生快照，而非默默替換為已審查 C02 稿。原來源單元保留、提議仍只在 A。請在收案前確認這項 native 條件格式轉換。
