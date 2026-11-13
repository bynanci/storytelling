<!-- SPDX-License-Identifier: MIT -->
# M3-04 正式執行契約 V1：G2 審查摘要

狀態：**提案，尚未接受 G2**。基礎提交 `48c1ca005c0087a585f2028d59ebb45fcdab5a46`。
[完整欄位／狀態／診斷規格](EXECUTION_CONTRACT_V1.md) ·
[ADR 0011](../adr/0011-production-execution-contract-v1.md) ·
[案例、實作工作包及 G2 清單](../validation/m3-04-execution-contract.md) ·
[封閉 JSON Schema](../../contracts/execution-intent-v1.schema.json)。

## 已有證據與實際缺口

#103 仍為 open／ready；#104 仍受阻於 #103；核對時沒有 open PR。
P2 已從選定提案建立不可修改的 `BoundStoryReviewV1`，持有實際編譯 OTIO。
SQLite Journal 已保存 operation／request／project／review／artifact 識別、
派送前標記、owner epoch、attempt 及晚到回應保護。

但 Journal 的 ownership 是每個 operation 各自持有；兩個 operation 仍可能
同時取得同一專案的寫入權。它也只保存呼叫者提交的 readback enum／摘要，
沒有驗證宿主真相或舊請求已停止。這兩項須由正式契約補足。

FilmCraft v0.18.0 的既有 Linux 證據是獨立合成專案的單次匯入與原生回讀；
回覆是在 child 成功退出後才刻意遺失。它沒有驗證執行中卡住、行程群組停止、
安全重試或當前授權。Review 的 `approval_granted`／`executable` 仍為 false。
原生專案也不被假設能自行驗證 Review 身分。

## 建議凍結的責任

| 保證 | 責任與限制 |
| --- | --- |
| 簽章數學上有效 | PR #28 對呼叫者指定的預期金鑰與 payload 驗章；不證明金鑰擁有者現在有執行權。獨立 G2 仍待接受。 |
| 簽署者／執行者目前有權 | 可信 runtime authority provider 根據已驗證 session、受保護 policy、當前撤銷狀態及明確人工核准決定。JSON 的宣稱不能建立權限。 |
| 審核綁定編譯產物 | engine 只能從 P2 私有結果持有的精確 bytes 建立執行 intent；派送前重驗當前 proposal／snapshot／context。 |
| 宿主確實套用 | 接受過的 profile reader 比對實際原生專案、完整預期投影及保存／重開證據，不能只看 exit 0 或 inspect 摘要。 |

第一版建議僅允許 `import_otio_isolated_project`：新建、隔離、空的專案副本。
現有共用或使用者正式專案須另行接受 profile。Host／profile／binary／adapter
都綁定精確版本與摘要；既有 test-only proof 不等於 profile 已通過生產驗收。
空專案須在 Review／核准前明確建立並登錄 baseline 收據；本匯入動作不得暗中
新建或重建遺失的 target。建立專案的自動化需要另一個已接受的 action 契約。

Intent 新增 operation／request／canonical resource、baseline revision、P2 binding、
proposal／snapshot／artifact／report 摘要、compiler／export profile、實際 host profile
與 binary、adapter build、預期原生投影及 action。各欄位及雜湊 framing 見完整規格。
保持既有 P1／P2／PR #28 wire 與 signing domain 不變。

## 核准、持久化與單一寫入者

V1 建議採本機可信的當前人工核准 provider，且排除離線 signed-grant 輸入。
`approval_ref` 只定位受保護核准紀錄，持有字串不代表有權。核准綁定完整 intent、
operation、principal/session、executor、action/resource、policy epoch、發出與到期時間；
最長五分鐘。provider/session 缺席、到期、撤銷或時鐘不確定都拒絕新派送。

派送前，在同一受保護交易內，序列化當前 policy、一次性核准消耗、專案 guard、
operation fence 及 `dispatching` attempt。核准消耗後 crash 一律先視為 unknown。
重試須重新取得核准，不能重播舊 grant。核准後的撤銷無法取消已發生的外部效果，
必須留下紀錄並回讀。

新增專案層級 guard，將路徑／別名解析為同一真實資源。unknown／partial 必須跨重啟
保留 quarantine，不能改 operation ID 或等 lease 到期就取得寫入權。接手 owner
先取得的是復原觀察責任。宿主若無法提供所需隔離／revision 保護，必須拒絕執行；
advisory file lock 不足以阻止外部編輯器。

正式 store 建議使用新 SQLite schema 2，沿用原交易及 state machine，加上完整 intent、
核准消耗、專案 guard、證據參照。既有 V1 API／資料庫不變、不自動升級或匯入未知操作。
退回舊功能時停用新寫入入口，保留 journal／證據／quarantine，不把舊資料庫備份覆蓋回去後重送。

## 結果與禁止重試的條件

| 結果 | 必要證據 |
| --- | --- |
| `dispatching` | 已提交派送前 attempt；結果尚未確定。 |
| `applied` | 完整且相關聯的原生 readback、舊請求已靜止的正面證據均已驗證並持久化。Host 的 Applied 回應只能是 `AppliedReported`。 |
| `not_applied` | 完整 baseline／觀察範圍未變，且正面證明舊請求、子行程與排隊工作都不會再寫入。仍不能自動重試。 |
| `unknown` | 逾時、crash、遺失回覆、回讀不完整、證據持久化失敗或先前請求可能仍執行。保留隔離狀態。 |
| `partial` | 已驗證部分效果；保留原始證據，人工處理。 |

Readback 必須綁定資源、baseline、intent、attempt、兩層 fence、profile／binary，
並驗證 clip occurrence／順序／數量、來源關係、精確有理數區間與時鐘、位置／長度、
tracks／effects／locks／已接受損失及無額外變更。來源 bytes 的實際驗證與保護由 profile 定義。
雜湊或單次「現在未變」都不能證明宿主已停止，也不能證明觀察範圍以外沒有副作用。
即使目前畫面已符合預期，若舊工作仍可能再寫入，結果仍為 unknown，不能釋放 guard。
新增的[結果／收據 schema](../../contracts/execution-result-v1.schema.json)凍結 operation、
intent、resource、attempt、兩層 fence、實際 host/profile、authority/readback/quiescence
證據摘要與 outcome；缺證據以 null 表示。收據也有獨立版本化 domain／長度 framing。

#104 建議維持單次 attempt。#105 才實作明確的復原重試；必須先確認 not_applied、
quiescence、原 baseline／intent 未變、當前來源／審核／授權、新核准及兩層 guard，
再提交新 attempt。unknown、partial、舊行程仍可完成、授權失效、target 改變、
late conflicting readback 或額度耗盡都禁止重試。不自動 Undo 或補償式編輯。

## 可以交給 Codex 的後續工作

1. 本次停在設計。維護者審閱設計並另行授權後，以獨立、無宿主操作的 #103 契約測試 PR，讓 core 增加 execution DTO／診斷；engine 沿用 P2 做確定性 identity／freshness。測試通過及最終 G2 接受前，#104 仍受阻。
2. #103 最終接受後，#104 的 journal 增加新 schema 的 project guard、quarantine、approval consumption 與同交易 attempt；保留 V1 回歸。
3. #104 在 CLI 組合 runtime authority、storage、單次 FilmCraft adapter 與 readback；缺可信 provider/profile 時入口不派送。
4. #105 驗證執行中 timeout、process-group／host quiescence 與明確安全重試。
5. #106 另做合成隔離專案的實際 host／平台驗收，再接 #109／#111／#112。

案例包含 stale review、錯 host、錯 target、未授權／到期／撤銷、簽章不能升格為權限、
重複 operation／grant、同專案不同 operation／別名競爭、unknown、晚到回應、partial、
DB 失敗與 JSON 邊界。每個預檢拒絕都要在未來測試中斷言 fake mutation calls 為 0。
本 PR 只檢查規格、fixture 與 digest vector 的完整性；正式 runtime contract tests
及其 RED／GREEN 須由另行授權的無宿主契約 PR 提供，才能完成 #103 最終接受；不能提前把那項驗收勾起。

維護者須依[完整 G2 清單](../validation/m3-04-execution-contract.md#g2-acceptance)
接受精確的 commit、欄位、authority provider、storage、readback／retry、診斷與限制。
**目前決定仍為 pending；#103 保持 open、#104 保持 blocked。**
