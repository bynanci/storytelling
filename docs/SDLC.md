# SDLC: 從需求到維運的交付規則

本專案以可驗收的 vertical slice 迭代，不採先鋪滿所有模組再補整合。
ROADMAP 決定成果順序，TASKS 決定工作範圍，REQUIREMENTS 連到測試與證據。

## 階段與放行門檻

| Gate | 階段 | 必要產物 | 放行條件 | 負責角色 |
| --- | --- | --- | --- | --- |
| G0 | 問題／範圍 | 使用情境、成功指標、non-goals | 有明確使用者結果，未與既有功能重複 | Product owner / maintainer |
| G1 | 需求分析 | Requirement IDs、驗收例、風險、依賴 | 硬限制與偏好分開；缺資料的項目明示 blocked | Maintainer + engineer |
| G2 | 架構設計 | ADR、schema/API、資料權威、故障模型 | 邊界可測；migration/approval/security 有處理方式 | Engineer + reviewer |
| G3 | 實作 | 小範圍 branch/PR、RED->GREEN 測試 | 單一可用切片；不破壞既有來源與權限 | Implementer |
| G4 | 驗證 | 同一 SHA 的 unit/contract/integration/CI、差異審查 | 無未解 correctness blocker；人工 gate 獨立紀錄 | Reviewer + maintainer |
| G5 | 發布 | 相容矩陣、版本/依賴/SBOM、遷移與回退說明 | 發布範圍真的被驗證；由 maintainer 明確批准 | Maintainer |
| G6 | 維運與迭代 | 缺陷、repro fixture、使用結果、advisory/host drift | 重大缺陷先止損；回饋進入下一輪 G0/G1 | Maintainer + users |

同一個人可以兼多個角色，但 self-review 不能寫成 independent review。
Agent 不冒充人類批准或代簽 DCO。合併依使用者明確授權與同-head 驗證結果執行；
發布、tag、repository policy 與 branch rules 仍須各自的明確授權。
本次延續任務已有「完成且驗證通過後合併並勾選任務」授權，見
[accepted lifecycle](../TASKS.md#accepted-lifecycle)；這不替代人工剪輯驗收。

## 工作狀態

`proposed -> ready -> in-progress -> review -> verified -> accepted`。
被阻擋時附 `blocker / owner / unblock condition / evidence`，不是把待辦直接勾掉。
Implemented、automated-verified、manual-pending、released 是不同維度。

## Definition of Ready

選定 Task ID；寫明使用者結果、範圍／不做項目、base SHA、依賴、資料契約、
驗收測試、相容性／隱私／安全影響，以及可獨立審閱的 PR 範圍。
素材、宿主或簽章身份缺少時，先定義 blocked gate，不捏造證據。

## Definition of Done

需求能追到程式與測試；相關 RED 已觀察，GREEN 已實跑；所有必要同-head CI
完成；檔案、source/plan/revision identity 與故障路徑符合契約；文件和風險更新；
review finding 收斂。真正需要宿主、聽感或乾淨機器的驗收必須有觀察，否則只接受
較小的已驗證範圍，不能宣稱全功能發布。

## PR 規則

一個 PR 解決一個一致的結果。Schema/time changes 先凍結契約，不讓平行工作
各自改格式。PR 包含 Task/Requirement IDs、base/head、變更理由、測試命令、
結果、限制、相容性與回退方式。不使用舊 head 的綠燈替代新 head。
原始 bootstrap 準備階段僅授權建立 branch/PR；之後的明確合併授權與實際交付
保留於 [TASKS](../TASKS.md#accepted-lifecycle)。每次依當前任務授權執行，
不把歷史準備限制當成仍待使用者重複批准。測試基線 commit 的紅燈是明示的
TDD evidence，不是可發布版本。

## CI 與真正的治理

CI 的 `Required gates` 彙總 Quality 和 Linux/Windows/macOS Rust jobs。
workflow 設定最小 read 權限、固定 checkout SHA、拒絕 commit formatter 的自動修補。
新增這個檔案不等於 GitHub branch protection 已啟用；管理者必須另行批准設定。
單一維護者 repo 不應盲目要求無法取得的第二人 approval；記錄可用的 review 政策，
保留敏感變更人工決策，不能用 auto-approve 假裝有人審查。

## 變更管理與相容性

公共 Rust API、wire schema、planner/compiler、adapter/host profile 各自版本化。
改 schema 需舊 fixture、migration 與 newer-version refusal。公開檔案不含私有路徑、
媒體、GPS 或 token。沒有維護者授權不做 license change 或正式發布。

## 維運

公開問題用最小合成 repro；安全問題走私密管道。新依賴或宿主版本改變需重新評估
相容性與 advisory；撤回有問題的 release 必須保留失敗證據與修復說明。
目前尚未發布，事故/rollback 流程先以文件為 gate，不能宣稱已有安裝測試。
