# Rust 原生化邊界與補強任務

[English](NATIVE_BOUNDARIES_AND_GAPS.md)

這份文件補充 [48項技術對照](OPENFILM_RUST_PORT_MAP.zh-TW.md)。GAP 是完整能力的規劃範圍，
不是把某個有限切片通過直接當成整項完成。下列回歸名稱原為規劃入口；
實際存在的程式、測試與已接受範圍以 [TASKS](../../TASKS.zh-TW.md) 和逐任務 records 為準。

已接受的局部證據包括：GAP-01 的 `provenance_manifest_covers_every_imported_file`
repository guard；GAP-02/03 的固定來源／精確時間比對；GAP-04 的明確 supplied-buffer
驗證與不可反序列化信任型別；GAP-05 的有限 OTIO loss 契約；GAP-09 的合成
Linux／25 fps CPU 宿主證據。參見 [M1-02](records/M1-02.md)、[M1-03](records/M1-03.md)
、[M2-01](records/M2-01.md) 與 [M2-02 固定視窗規劃](records/M2-02.md)。一般化 evidence、檔案 race closure、更多宿主／媒體、
人工驗收和其餘 GAP 的完整驗收仍保留。

## 先統一「原生」的意思

| 層 | 本專案承諾 | 不是什麼 |
| --- | --- | --- |
| 純Rust core/engine | 同一輸入可在沒有Node/Python/SQLite/FFmpeg/網路的程序內驗證與規劃 | 不代表所有媒體與模型都由Rust實作 |
| Rust native runtime | Rust編譯的檔案/交易/owner/程序控制 | 不代表沒有OS API、SQLite C、系統library或平台差異 |
| Rust adapter | 受控呼叫FFmpeg、模型、宿主；提供typed contract | 不把包裝外部程序當成純Rust codec/ASR |
| 由剪輯平台提供 | 使用者介面、混音、調色、效果、剪輯與既有算圖工具 | 整合後保留原平台的功能。 |

Rust可重寫pure算法與I/O控制，但「全部不依賴Node/Python」是production核心的目標，不是移植比對/官方OTIO oracle的測試環境限制。每個feature列production/test-only/external依賴，不能用同一個標籤掩蓋。

## 真正需要補的16項

下方的具體驗收與依賴也存於 [machine ledger](openfilm-port-index.json)。

### GAP-01｜授權、provenance與原生化邊界

**P0／M1-01；前置：** 無；第一道移植門檻。

逐項記錄source SHA/path/license、來源與修改；維持MIT LICENSE不變，Apache衍生內容不能因Rust重寫就當MIT。先由維護者批准component/SPDX/NOTICE政策；無權利證據則blocked。Rust-native與純Rust、外部工具、模型、作業系統依賴分開標示。

**新增驗收測試：** `provenance_manifest_covers_every_imported_file`。

### GAP-02｜跨語言conformance與fixture凍結

**P0／M1-01；前置：** GAP-01。

以來源實際tests挑選合成fixture，保存input/expected/error與工具版本。TS只做測試oracle、不進Rust production；差異分為port bug/upstream bug/unsupported/intentional delta。測試數不當完成百分比。

**新增驗收測試：** `differential_fixture_is_pinned_and_classified`。

### GAP-03｜公共wire與精確時間/數字契約

**P0／M1-01；前置：** GAP-01。

保留M0 v1或明確升版；DTO/domain分離、decimal-string大整數、rational normalization、Unicode/canonical digest規則。外部host wire另適配，不能假設接受字串整數；boundary quantization產生可審閱損失。

**新增驗收測試：** `cross_language_large_integer_and_unicode_vectors`。

### GAP-04｜declared／verified evidence信任升級

**P0／M1-01；前置：** OF-29。

新增宣告與已驗證資料不同型別/狀態；來源實際hash、producer/model版本、時間、scope可驗證，不能只檢查64字hex。資料與演算法更新不得沿用舊evidence身分；人工推測/LLM推測不得當事實。

**新增驗收測試：** `declared_hash_never_becomes_verified_by_deserialization`。

### GAP-05｜宿主capability與編譯損失契約

**P0／M1-02；前置：** GAP-03。

既有匯出工具只支援特定剪輯器。每個轉接器須列出平台、版本、支援規格與必要能力。分別標明原生支援、預先算圖、僅留中繼資料與未支援的項目。記錄識別碼與時間的對應方式。M1 初期規劃涵蓋有限的硬切、靜態圖片與基本音訊。實際接受範圍以[畫面軌交換規格](FINITE_OTIO_V1.md)為準。

**新增驗收測試：** `unsupported_effect_blocks_without_explicit_loss_choice`。

### GAP-06｜人類批准與Agent權限分離

**P0／M3-01；前置：** GAP-03, GAP-05。

批准綁plan/compiled digest、host snapshot、source/capability版本與loss選擇；可信UI/CLI產生、可撤回/過期，模型不能自行confirm:true。readOnlyHint不當授權；token不寫portable project。

**新增驗收測試：** `agent_cannot_self_approve_or_reuse_stale_approval`。

### GAP-07｜宿主資料權威、ID mapping與drift

**P0／M3-01；前置：** GAP-05, GAP-06。

優先建立獨立的剪輯專案。編輯者在平台精修後，以該平台的時間軸為準。保存來源素材、鏡頭及每次使用位置與平台識別碼的對照。先讀取指紋再寫入，仍可能遇到並行修改。原地更新與無鎖雙向同步都需要可靠的原子前置條件。

**新增驗收測試：** `host_manual_edit_invalidates_incremental_plan`。

### GAP-08｜外部副作用的unknown-result恢復

**P0／M3-02；前置：** GAP-06, GAP-07。

SQLite commit不包含外部宿主操作；用operation journal、intent fingerprint、attempt/receipt做reconcile。逾時先讀回，不盲重送、不Undo N次；無法證明結果時保留unknown/partial，禁止宣稱exactly-once。

目前只有 M3-02 的 SQLite journal storage slice；它記錄 caller-supplied readback
而不呼叫宿主或證明該觀察為真。實際已套用後逾時、宿主請求停止與安全重試仍由 M3-03 驗收。

**新增驗收測試：** `timeout_after_host_commit_does_not_duplicate_import`。

### GAP-09｜結構與媒體結果的獨立驗證

**P0／M1-03；前置：** GAP-05。

檢查來源可用性、warnings/offlineMedia、host讀回順序/ranges、影片PTS/duration與產物hash；成功旗標不等於正確影片。結構、renderer、encoded output、人耳/視覺分開報告。

**新增驗收測試：** `successful_export_with_missing_media_fails_verification`。

### GAP-10｜路徑/程序/provider信任邊界與預算

**P0／M1-01；前置：** GAP-01。

限制根目錄、symlink/reparse、大小/節點/時間/輸出、shell與網路。模型/字幕/檔名皆不可信；不允許其擴權或導向任意檔案。實際OS process isolation與權限明示，非假sandbox。

**新增驗收測試：** `untrusted_source_text_cannot_expand_capabilities`。

### GAP-11｜跨平台故障與資料庫可回復性

**P1／M3-02；前置：** GAP-02。

覆蓋Windows路徑/大小寫/檔案鎖、Linux/macOS fsync/rename、SQLite busy/WAL/crash、PID reuse/unknown。正常unknown不當dead；故障後queue不能永久poison；資料量上限可測。

有限的 WAL/FULL、SQLite busy、模擬程序重啟、owner fencing 與操作資料上限已在
[M3-02 journal contract](M3-02-OPERATION-JOURNAL.md)實作/測試；三平台 hosted CI
與各 OS 真實 crash/power-loss/network-filesystem 行為仍是獨立證據，整項 GAP-11 未完成。

**新增驗收測試：** `crash_lock_and_write_failure_recover_without_loss`。

### GAP-12｜比對並切換至 Rust 實作

**P1／M4-01；前置：** GAP-02。

每個服務只指定一個可寫入的實作。用唯讀快照比對既有 TypeScript 與新 Rust 實作。通過比對後，以功能開關切換實作。最後停用舊實作。

測試新舊格式的雙向相容性與回退路徑。每個專案只允許一個規劃器寫入。保留使用者在線上使用的素材目錄。

**新增驗收測試：** `feature_flag_has_one_writer_and_safe_rollback`。

### GAP-13｜原生CLI供應鏈與平台分發

**P1／M5-02；前置：** GAP-01, GAP-11。

鎖toolchain/MSRV與依賴，逐feature列Rust/C/native/system/tool/model閉包及SBOM；三OS與乾淨環境驗證。story-core/engine無Node/Python/GUI/runtime副作用；FFmpeg/host需要時明示。禁止先發布未審候選。

**新增驗收測試：** `native_core_runs_without_node_python_or_media_tools`。

### GAP-14｜公開API與consumer/transport一致性

**P1／M4-01；前置：** GAP-03, GAP-06。

外部Rust樣例、CLI JSON與TS client共用一套errors/contracts。SDK/JSON/planner/adapter各自版本；有界responses、stable IDs、progress/cancel。MCP為薄層；N-API/Python/WASM不一次啟動。

**新增驗收測試：** `external_consumer_and_cli_share_errors_and_results`。

### GAP-15｜實際故事價值與人工工作站驗收

**P1／M5-03；前置：** GAP-09, GAP-12。

使用同一組獲授權素材比較兩種工作方式。一組由來源工具經 Rust 規劃後交給剪輯平台。另一組由 AI 助理直接操作剪輯平台。記錄操作時間、限制條件的保留情形、重做與再次使用的結果。

缺少素材、設備或試聽人員時，標記人工驗收受阻。公開 CI 只使用合成或獲授權資料。主觀評價由實際使用者提供。

**新增驗收測試：** `manual_gate_cannot_be_closed_by_synthetic_metrics`。

### GAP-16｜來源漂移、任務證據與差距稽核

**P0／M1-01；前置：** GAP-01。

每個移植PR重新fetch兩repo，記錄固定SHA、source blob/測試、target paths、差異與依賴。PR狀態以GitHub metadata與程式為準，不照過時body。新增任務需Task→Requirement→test→evidence鏈；自查不冒充獨立審查。

**新增驗收測試：** `transfer_manifest_has_no_missing_dependencies_or_stale_claims`。

## 原有需求與補強的差別

來源工具已有雜湊比對、寫入者隔離、檔案保護與使用者同意機制。此處列出獨立 Rust SDK、跨語言介面與外部平台操作的驗證工作。逐項對照既有行為，確認保護機制在新的執行環境仍然有效。剪輯操作由既有平台負責。

## 授權與來源決策

Storytelling 的原創程式碼採用 MIT 授權。此移植計畫參考的上游第一方程式採用 Apache-2.0。逐檔記錄移植程式、測試、文字與衍生內容的授權。保留適用的上游聲明。

Apache 元件須標示 SPDX 授權識別碼。整體授權政策的變更須由維護者核准。改寫成 Rust 仍須遵守原有義務。[逐檔來源與聲明](../../PROVENANCE.md)

建立本規劃時，團隊未匯入該上游的實作或測試。後續移植須記錄儲存庫、完整提交識別碼、來源路徑與 Git 物件。另列原始授權、上游聲明、修改內容與目標檔案。第三方媒體、模型、字型與編解碼器各自查核。缺少使用權利的證據時，標記 `blocked`。

官方依據：[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.html) 的再分發條件、[LICENSE/NOTICE指引](https://infra.apache.org/licensing-howto.html)。這是工程合規門檻，不是未審來源的整體法律認證。

## 放行與停止條件

- M1 basic interchange若無法保留順序/source range/時間，先縮profile，不擴張多宿主。
- 來源有bug時保留最小repro與intentional correction，不為diff全綠照搬bug。
- SHA/宿主/素材改變需重驗受影響項目；不沿用舊版本的人工通過。
- 無Rust工具鏈、宿主、真實素材或設備時，完成獨立工作並標blocked，不偽造RED或人工驗收。
- 小範圍PR可接受已驗證切片；不因本清單涵蓋全部方向就一次重寫48項。
