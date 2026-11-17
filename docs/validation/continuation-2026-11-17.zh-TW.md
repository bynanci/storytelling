<!-- SPDX-License-Identifier: MIT -->
# 驗證交付與待完成門檻 · 2026-11-17

本文件是 [Issue #114](https://github.com/bynanci/storytelling/issues/114) 產品化追蹤圖的最新狀態收據，依據 main `f77ccad6dc40affdd6f7b2541ae93ba88095665d`。核對時 GitHub 顯示零個 Open PR、十六個 Open Issue。本更新記錄已交付工程範圍及重啟阻擋任務所需證據，不代表接受契約、維護者決策、結束產品里程碑或授權發布。

## 新增已驗證交付

| PR | 提交 head → 合併 | 結果 |
| --- | --- | --- |
| [#142](https://github.com/bynanci/storytelling/pull/142) | `eac63b8a5adaac5208436ff1622d3d66e1f4bb6d` → `a219201f46bc78595225deea79795bd64f5ef671` | 對既有 R1 分數新增僅追加的精確算術敏感度報告。原五份衍生產物逐位元組重現；原分數、宣告分組、旗標與結論未更動。新增十項回歸及既有 24 項 SK-E04 測試通過。分析單位為 40 個目的性選取案例；列舉分組只是敏感度情境，不是新樣本。無法據此證明 Skill 的因果效益。 |
| [#143](https://github.com/bynanci/storytelling/pull/143) | `9d238b8c2cea51205f333546753582b2c81b248a` → `f77ccad6dc40affdd6f7b2541ae93ba88095665d` | 為唯讀交接輸出建立已綁定來源的索引。三項 CLI 行為刻畫測試保留順序、版本、變體範圍與診斷。事前訂定大型合成 release 負載中位數比值 ≤0.75；32／63 份產物量得 0.5474／0.5480，176 次程序執行輸出位元組均一致。這是單機程序量測，不是作者時間或宿主驗收。 |

兩個 PR 的最終 head CI 均通過六項一般工作及三項 SK-E06 平台工作；合併後也各通過相同九項。精確執行收據見 [#142 最終 CI](https://github.com/bynanci/storytelling/actions/runs/43692841559)、[#142 合併後 CI](https://github.com/bynanci/storytelling/actions/runs/43693164744)、[#143 最終 CI](https://github.com/bynanci/storytelling/actions/runs/43693354711)、[#143 合併後 CI](https://github.com/bynanci/storytelling/actions/runs/43693659339)。兩個專用 matrix 執行連結見各 PR。合併樹與 #143 已驗證 head 一致；原 R1 套件 15 項 checksum 仍與 Git blob 及工作目錄位元組相符。

## 未完成議題與恢復條件

| Open Issue | 目前阻擋原因 | 恢復條件 |
| --- | --- | --- |
| [#103](https://github.com/bynanci/storytelling/issues/103) | 執行契約及圓桌建議仍是提案；尚未記錄正式技術 G2 接受與 W01 授權／實作。 | 維護者接受明確契約範圍；完成確定性、無宿主測試；維護者針對精確測試 commit 記錄決定。 |
| [#104–#106](https://github.com/bynanci/storytelling/issues/104) | Executor 依賴 #103；復原依賴 executor；宿主驗收需要實際支援設定下的執行與讀回。 | 按依賴順序完成不可變操作身分、單寫入者 fencing、unknown 結果復原、隔離 FilmCraft 實際讀回及維護者接受／縮小／停止決策。本狀態文件不表示可修改正式專案。 |
| [#107](https://github.com/bynanci/storytelling/issues/107) | ADR 0013 八個唯讀入口仍是 G2 提案；公開 API 凍結與實作待辦。 | 維護者接受明確 API／版本／錯誤範圍；實作 facade 並證明 CLI／SDK 結果等價。Mutation API 仍受 M3 接受門檻管制。 |
| [#108–#110](https://github.com/bynanci/storytelling/issues/108) | TypeScript consumer、reference app 與選配 MCP 依賴公開 facade；mutation 另依賴 M3。 | 使用已接受公開介面，維持唯讀邊界，顯示核准／過期／unknown 狀態並保留單一寫入者。 |
| [#111](https://github.com/bynanci/storytelling/issues/111) | 尚無第二編輯器匯入／播放／儲存／重開證據。 | 選定一項有限宿主設定，取得授權的實際宿主測試及損失、讀回紀錄。 |
| [#112](https://github.com/bynanci/storytelling/issues/112) | 五人前瞻試用規程已備妥，但 #109、#111、M3 接受、招募及凍結素材仍未完成。 | 招募前接受規程；凍結版本、素材、任務及評分；蒐集獨立參與者的實際場次。舊作者案例不能算作 Beta 參與者。 |
| [#113](https://github.com/bynanci/storytelling/issues/113) | 發行與候選版證據依賴已接受的 consumer、宿主及 Beta 結果。 | 整理真實相容性、遷移、回退、依賴及試用收據；G5 發布另由維護者決定。 |
| [#114](https://github.com/bynanci/storytelling/issues/114) | 子議題門檻未完成，父級追蹤仍須維持 Open。 | 子議題取得決定和驗證證據後更新本圖；準備 PR 不等於產品化完成。 |
| [#115](https://github.com/bynanci/storytelling/issues/115) | SK-E06 資料包完整性已測試，#142 增加算術診斷；實際手動／host 執行及 #121／#122 各自證據仍缺。 | 在獲授權環境執行案例，記錄實際 host／skill／runtime 與逐字輸出；否則結果保持未知。不得把參考答案或合成演練當真人證據。 |
| [#121](https://github.com/bynanci/storytelling/issues/121) | 反序重跑 R1 所需原始草稿、完整來源資料、提示／rubric、模型設定及逐字 judge 紀錄不可得。 | 提供已授權的私人原始資料包與合格凍結環境；不得重建缺失的研究輸入。 |
| [#122](https://github.com/bynanci/storytelling/issues/122) | 依賴合格 #121 資料及有理據的指引修訂；目前兩者皆無。 | 先完成前置條件，再凍結前瞻分析計畫，之後才建置或檢視 holdout。 |
| [#129](https://github.com/bynanci/storytelling/issues/129) | A 已實作但缺獨立作者操作；B/C 各自的 G2／隱私與實作門檻未完成；D 仍是清單。 | 記錄獨立作者實際操作；取得 B/C 各自決策後再實作核准範圍；媒體及宿主發現保留在獨立證據欄位。 |

這些門檻不能互相替代。合併及 CI 綠燈只證明提交的程式／文件檢查通過；不證明目前授權、真人身分、參與者結果、實際媒體內容、編輯器行為或發布核准。PR #141 的圓桌選擇仍是技術建議，沒有記為維護者正式 G2 接受。

## 目前交接

目前沒有 Open PR。十六個 Open Issue 為 #103–#115、#121、#122、#129。最近的工程工作須等維護者明確決定是否接受 #103 W01、#107 唯讀 SDK、#129 B 的精確範圍。研究工作缺原始研究材料或真人參與；宿主工作需取得授權的實際觀察；G5 仍是獨立的維護者決定。

本次沒有發布版本、tag、修改 repository policy、操作真實使用者專案，也沒有新增真人結果、Host 接受或 G5 決定。
