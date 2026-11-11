# 架構

<a id="architecture"></a>

[English](ARCHITECTURE.md) · 繁體中文

<a id="how-projects-use-storytelling"></a>

## 選擇入口

宿主是載入 Skill 或執行整合操作的應用程式。寫作時可以是 AI 助理，影片交接時可以是剪輯器。

| 使用情境 | 輸入與輸出 | 執行環境 |
| --- | --- | --- |
| 作者和 AI 共寫 | 作者提供筆記。代理依 Skill 起草口述稿、場景、構圖說明、文案或影片大綱。 | 宿主產生文字並提供已啟用的工具。Skill 內含範例與工作流程。[可攜式 Skill](../skills/storytelling/SKILL.zh-TW.md) · [平台安裝指南](PLATFORMS.zh-TW.md) |
| 應用程式需要可重跑的結果 | 應用程式傳入符合契約的 JSON 或來源位元組。Rust 命令列介面（CLI）回傳標準輸出與結束碼。 | 本機可獨立執行。CLI 可驗證計畫、檢查創作需求、規劃固定時間窗。它也能輸出口述大綱與插畫構圖，或查詢修訂後的影響。[命令列指南](CLI_GUIDE.zh-TW.md) · [契約](CONTRACTS.zh-TW.md) |

影片快照匯入、OTIO 匯出與對應的審閱命令各有輸入限制。
任何能提供必要事實的專案，都可建立符合規格的快照。
實際宿主相容性依版本、平台與測試案例判定。

<a id="video-implementation-status"></a>

## 影片功能驗收範圍

| 項目 | 已驗收的範圍 |
| --- | --- |
| M0、M1-01、M1-02 | 宣告資料驗證、有限剪輯快照與純畫面時間軸。[M0 契約](CONTRACTS.zh-TW.md) · [快照契約](migration/FINITE_V1.md) |
| M1-03 | 指定位元組驗證與隔離宿主測試。[測試紀錄](migration/records/M1-03.md) |
| M2-01、M2-02 | 指定範圍的需求檢查與固定時間窗規劃。 |
| M3-01/P2、M3-02 | 唯讀 OTIO 審閱橋接與 PR #56 的 SQLite 操作日誌。日誌沒有宿主執行路徑。 |

PR #28 已合併，但仍待獨立 G2 審查。其驗收範圍與 P1 分開。

<a id="implemented-dependency-direction"></a>

## 模組與依賴方向

| 模組 | 職責 | 邊界 |
| --- | --- | --- |
| CLI | 讀寫本機資料。輸出結果與結束碼。 | CLI 呼叫引擎。 |
| 引擎 | 驗證輸入、映射快照、產生敘事輸出、序列化 OTIO。 | 引擎依賴核心與 `serde_json`。不使用檔案系統、時鐘、隨機狀態、資料庫、網路或剪輯器。 |
| 核心 | 定義嚴格的傳輸型別、有理數時間與錯誤型別。 | 使用 Serde 與已審閱的 SHA-2 軟體實作。雜湊的輸入限於呼叫端提供的位元組。 |
| `storytelling-journal` | 首個原生執行環境轉接器。以內附的 SQLite 儲存操作狀態並限制寫入者。 | 資料庫 I/O 與核心、引擎分離。CLI 尚未組裝或呼叫此日誌。 |

`scripts/check_repo.py` 檢查直接依賴。貢獻者另須審查間接依賴與實際行為。
專案禁止在自有 Rust 程式碼中使用 `unsafe`。

<a id="future-boundaries-not-empty-crates"></a>

<a id="cross-media-authoring-and-extensions"></a>

## 跨媒介敘事與後續擴充

<a id="proposed-cross-media-authoring-model"></a>

<a id="source-material-variants-and-revisions"></a>

### 素材、呈現方式與修訂

N 系列將作者提供的素材與呈現方式分開。口述稿用順序組織內容。
單幅插畫用焦點、前景與背景組織內容。兩者都可參照同一筆觀察。

| 問題 | 契約與責任 |
| --- | --- |
| 作者提供了什麼？ | 創作需求記錄目的與受眾。每個內容單元記錄 ID、來源與修訂版號。觀察、詮釋與創作提案各自標記。 |
| 單元如何連結？ | 設計上區分依據、聯想與強制順序。雙向聯想可以成環。系統須檢查指定變體內的強制先後順序。 |
| 作者如何更換表達方式？ | 命名變體參照單元及修訂。口述節拍有順序。插畫元素有空間角色。未來的構思模式可保留替代分支。 |
| 軟體能驗證什麼？ | 引擎可檢查參照、限制與修訂是否吻合。引用只記錄作者提供的依據。作者評估內容準確性與發布適切性。AI 提案也須通過輸入檢查。 |

維護者於 2026-11-11 在 #50 採納獨立版本的敘事契約。
N-03 的核心定義敘事型別。引擎驗證及轉換資料。CLI 負責本機 I/O。
`render-narrative` 將宣告內容輸出口述大綱與插畫構圖。N-01 試用包已備妥。
真人試用及繼續、縮小或停止的決策仍待記錄。[ADR 0005](adr/0005-independent-narrative-document-v1.md) · [N-03 驗證](validation/n03-narrative-cli.md)

N-04 以 `NarrativeDocumentV1` 為初始快照。另以 `NarrativeRevisionLedgerV1` 保存只新增的事件。
`track-narrative-revisions` 重播來源修訂與變體歷史。它保留作者宣告的選擇與待審閱狀態。
`query-narrative-impact` 接受來源修訂事件 ID。引擎只列出直接參照該修訂的變體。
`based_on` 記錄衍生關係。間接影響仍須由作者檢查。
[修訂紀錄契約](narrative/NARRATIVE_REVISION_LEDGER_V1.md) · [N-04 驗證](validation/n04-narrative-revisions.md)

敘事輸入獨立於媒體雜湊、來源區間及幀率。
後續影片轉接器可將所選單元連到明確素材，再經公開契約呼叫規劃器。
保留影片 V1 輸入、CLI 行為、摘要向量與 OTIO 測試資料作回歸檢查。

N-02 的來源與衍生關係可參考 [W3C PROV-DM](https://www.w3.org/TR/prov-dm/)。
空間分組可參考 [JSON Canvas 1.0](https://jsoncanvas.org/spec/1.0/)。
用合成案例評估實作成本。將決定記在[敘事任務](../TASKS.zh-TW.md#cross-media-narrative-backlog)指定的架構決策紀錄。

N-05 的 `audit-copy-claims` 沿用相同邊界：core 保存版本化文案與報告型別，engine 對照呼叫端提供的敘事修訂簿驗證有上限的引用，CLI 讀取兩份本機輸入並輸出一行 JSON。報告附上所引用 revision 的原始文字供人比對。引用目前有效不代表語意上獲得佐證、來源真實或文案可發布。

<a id="workflow-and-editor-adapters"></a>

### 工作流程與宿主轉接器

規劃中的依賴方向如下。

- 工作流程依賴引擎與核心，定義應用情境及介面。
- 轉接器依賴工作流程介面與核心。用途包括資料庫、快照讀取、OTIO、剪輯器及選用服務。
- CLI 組裝工作流程與轉接器。應用層與引擎保有規劃及核准政策。

目前 SQLite 日誌可獨立運作。完整工作流程、正式宿主執行器與宿主應用程式轉接仍在規劃中。
新增 crate 須有可執行的功能。TypeScript 專案可先透過子程序使用 CLI 的 JSON 協定。
封裝 SDK 與額外傳輸方式列入後續里程碑。

<a id="authority"></a>

<a id="data-ownership"></a>

## 資料責任

| 資料 | 責任方 |
| --- | --- |
| 故事意圖、限制、作者選取的內容、提案與決策 | Storytelling |
| 來源位元組、檔案存取與媒體分析 | 獲授權的來源工具或轉接器 |
| 來源專案的素材、剪輯組合、目錄與歷史 | 來源專案 |
| 交接後的宿主時間軸與後製 | 目標剪輯器 |
| 映射、核准、執行與驗證紀錄 | Storytelling 工作流程 |

計畫文件是不可變的提案輸入。M0 拒絕超出範圍的欄位。
M0 未涵蓋圖片、軌道、速度、音量自動化或宿主特效。
M1-01 將符合規格的剪輯快照映射到 M0。它保留完整來源事實與結構化的限制紀錄。
該相容範圍僅含文件列出的快照。

M1-02 將指定畫面內容序列化為原生 OTIO。內容使用一個有範圍限制的整數時鐘。
呼叫端須明確選擇僅輸出畫面或僅以中繼資料保留鎖定狀態。轉換時保留來源事實。
專案幀率另行處理。官方 Python 解析器僅供測試。

M1-03 另行驗證指定位元組，並完成 FilmCraft 在 Linux 上以 25 fps 執行的合成 CPU 測試。
正式宿主執行器仍待實作。[M1-03 證據](migration/records/M1-03.md)

M2-01 檢查必要內容及順序衝突。M2-02 選取固定時間窗。通用求解器仍在規劃中。

<a id="time-and-identity"></a>

## 時間與身分

M0 以約分後的非負分數表示時間。分子與分母以 `u64` 字串編碼。
有效的 `RationalTime` 以 `u128` 交叉相乘比較，可避免浮點捨入。
區間採左閉右開 `[start,end)`。系統不猜分母，也不自動對齊影格或取樣點。

來源、時間軸、影格及取樣時間的型別須分開。
宿主轉換量化精度時，須提供明確報告。勿自行將 29.97 等同於 30000/1001。

素材 ID 與鏡頭實例 ID 各有用途。同一份素材可出現兩次。
SHA 字串只代表宣告的值。M1-03 可驗證呼叫端提供的位元組。
核心持有私有、不可變的緩衝區。此結果無法證明日後讀取的檔案內容。
檔案系統身分仍待轉接器處理。M0 的可攜文件拒絕路徑、網址、命令、模型憑證與任意擴充欄位。

<a id="future-execution-safety"></a>

<a id="execution-safety-and-recovery"></a>

## 執行安全與復原

編譯計畫須綁定提案、實際來源、編譯器、宿主能力及明確的損失選擇。
真人核准須來自可信介面。語言模型的 `confirmed:true` 無法授權執行。
首次執行應建立隔離的宿主專案。

外部請求逾時後，結果為未知。先讀回實際狀態再決定如何復原。
禁止盲目重試或反覆復原。SQLite 交易無法涵蓋宿主的副作用。
執行憑據區分成功、失敗、部分完成與未知。規劃、工作生命週期與資料新舊程度各自記錄。

日誌保留審閱綁定、產物的精確摘要、嘗試憑據、執行者 ID 與單調遞增的寫入世代。
`AppliedReported` 表示執行者回報套用。`AppliedObserved` 表示已觀察套用結果。
執行中的嘗試若過期，復原為未知。寫入隔離只保護日誌更新。

M3-03 須證明舊請求已停止，才能將宿主讀回用作重試依據。
固定版本的 Linux 測試重現一次回應遺失。該次 FilmCraft 已成功結束。
測試工具讀回隔離專案，核對實際結果。宿主卡住時的停止判定與正式執行器仍待驗證。
[M3-03 測試紀錄](validation/m3-03-filmcraft-execution.md)

<a id="decisions"></a>

<a id="architecture-decisions"></a>

## 架構決策紀錄

- [ADR 0001](adr/0001-scope-and-boundaries.md) 記錄 Rust 移植策略。
- [ADR 0002](adr/0002-exact-time-and-wire.md) 記錄時間與傳輸限制。
- [ADR 0003](adr/0003-license-and-provenance.md) 記錄儲存庫授權與來源。
- [ADR 0008](adr/0008-local-png-host-feasibility.md) 提議 N-12 本機 PNG 交付邊界，保留獨立宿主與讀回證據。

<a id="accepted-fixed-window-planner"></a>

## 固定時間窗規劃器

M2-02 重用嚴格的創作需求檢查。規劃器先納入所有必要候選。
它依固定規則排列可選候選，再依拓撲順序排列固定時間窗。
核心定義宣告型別。引擎的規劃與時鐘模組輸出符合 M0 的計畫及理由路徑。
本功能未新增正式環境的相依套件或執行權限。完整的多節拍求解仍超出驗收範圍。

<a id="merged-pr-28-approval-identity-candidate-not-accepted"></a>

<a id="pr-28-approval-identity-candidate"></a>

## PR #28 的核准身分候選方案

PR #28 已合併，獨立 G2 審查仍待完成。已驗收的 M3-01/P1 限於有限 OTIO 的審閱綁定。
兩者使用不同契約。PR #28 無法作為該項 P1 的驗收證據。[P1 範圍](migration/M3-01-NEXT.md)

`storytelling-core::approval` 提供純函式契約與驗證器。
它只從既有的私有 `VerifiedBufferV1` 取得來源身分。
它分別計算提案與編譯產物確切位元組序列的摘要。
它記錄宣告的宿主規格、版本、能力與已接受的損失。
解析傳輸資料後，須先檢查身分結構，再與本機建立的身分憑證完整比對。

外部簽署者、可信主體及金鑰登錄由核心以外的元件管理。
呼叫端提供公鑰綁定。Ed25519 嚴格驗證只證明簽署者控制該金鑰，且簽署了指定用途的資料。
人員身分與核准須另行確認。`HostBindingV1.observation` 綁定呼叫端的聲明。
它沒有提供獨立的宿主觀察。`VerifiedApprovalV1` 是不透明型別，不可序列化為持有者權杖。

此候選方案未涵蓋金鑰儲存、防重播日誌、到期或撤銷服務、核准資料庫、CLI 簽署與宿主執行。
後續執行須支援持久化的防重播與寫入者隔離。執行前須取得宿主的實際證據。
欄位順序、正規表示方式及資料上限見[核准身分契約](migration/APPROVAL_IDENTITY_V1.md)。
