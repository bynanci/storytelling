# Codex：OpenFilm → Rust 單項移植 Runbook

本文件是可複製執行契約。來源對照在 [port map](OPENFILM_RUST_PORT_MAP.md)，機器索引在 [JSON](openfilm-port-index.json)，SDLC仍由 [SDLC](../SDLC.md) 管理。

## 1. 開始前必須核實

- Source baseline：bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76。
- Historical first target inspection：main `551ff38920e2085e1d53a7ba777c9680678e9725`；M0 PR #1 head `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`，當次仍 Draft。保留這些調查身份，不當作目前完成狀態。
- M1-02 階段的歷史 target main 快照：`9a5232e97435f543238b90a2006460f319f4e651`；當時 M0 PR #1、map PR #2、有限 M1-01 PR #7 與有限 M1-02 OTIO PR #16 均已合併。
- 本輪延續工作的 accepted base：`c03d4c8ede33563451b8ad590a6df982d0d8b82a`。另有 M2-01 bounded audit PR #17 與 M1-03 PR #18 已接受；後者只證明 supplied-buffer SHA-256 與一個合成 Linux／25 fps／CPU 預覽往返案例，不涵蓋檔案 TOCTOU、一般宿主 executor 或人工剪輯驗收。使用方式見 [CLI guide](../CLI_GUIDE.md)，最新狀態見 [TASKS](../../TASKS.md)。
- 重新 fetch 兩個 repo 與 open PR，核對 [TASKS](../../TASKS.md) 及相依交付；上述 SHA 是查核快照，不取代每次移植前的即時查核，也不要求重做已合併的有限切片。
- 固定來源已含 coverage PR #121、frame time PR #125；PR #126 在最初規劃快照是 Draft，後續已確認 feature head `b4aba4c4bb70b8cb03edd54342f1b94cec649245` 合併於 observed main `5f737a6c8f35dce48e213370ff27900962c9304c`，而不在其 parent、固定 baseline `7add4762fe8174d76f43b863375ea39b73121e76`。M1-01/M1-02 保留 baseline 與 animation 拒絕政策，見 [drift record](records/M1-01.md) 與 [finite OTIO contract](FINITE_OTIO_V1.md)；新任務仍須按實際查詢更新來源狀態。
- 對每個選定source path執行git存在檢查，讀取實作與實際呼叫者/測試。表列位置是定位入口，不是對整個檔案的正確性保證。

## 2. 一個任務的comparison record

每支移植PR新增自己的 `docs/migration/records/<task-id>.md`。此路徑為未來實作產物，不要求現在建立空檔。

```text
Task ID / parent milestone / requirements:
Source repository + full commit + paths + blob hashes:
Actual source symbols and test names inspected:
Target base/head + actual existing behavior:
Classification: native-core / native-runtime / external-adapter / retain-host
Current target status: absent / partial / implemented / verified
Licensing/provenance decision and notices:
Input/output DTO + validated domain + clock/unit:
Side effects and authoritative owner:
Behavior preserved / intentional change / unsupported:
Production dependencies / oracle-only tools / external runtimes:
Tests: requirement ID -> new test -> original fixture/provenance -> result
Manual/host gates:
PR dependency / rollback / next task:
```

不得把source implemented和target implemented混成一個勾選；M0只做declared-metadata驗證。匯入fixture也需來源權利，不可包含私人片段/路徑/GPS/憑證。

## 3. 標準實作流程

1. G0/G1：選一個OF/GAP ID與原M task；列scope/non-goals、依賴、來源行為與無法驗證處。
2. G2：凍結本PR的schema/signature/error codes、資料權威、time conversion與失敗模型。跨檔案同一public contract只由一支PR主導。
3. 來源tests用 `rg -n '<實際symbol或error>' packages tests` 定位；固定合法/非法fixture與oracle版本。不存在的測試不要編造。
4. 寫Rust regression並觀察RED；缺工具鏈或依賴下載失敗不是功能RED。
5. 原生重寫最小可用切片；不以呼叫整個OpenFilm Node server宣稱完成native port。外部adapter則公開其真實依賴。
6. 跑Rust/TS differential，對每個差異分類，保留預期與原始證據。不把所有輸出排個序就消除語意差異。
7. Fault paths與邊界：超限、來源換檔、stale revision、busy、取消、crash、unknown host result；沒有副作用的pure任務不被迫引入runtime。
8. 同-head CI、文件/traceability、review、PR。未經批准不merge、release、簽維護者DCO或更改管理設定。

## 4. 驗證指令（依實際新增範圍）

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo doc --workspace --no-deps --locked
python3 scripts/check_repo.py
git diff --check
```

`check_repo.py` 保留三個實作 crate、明列 direct dependencies、逐任務 provenance 與 test traceability 檢查；M1-03 的 SHA2 例外已有獨立依賴審查。新增真實 adapter 的同一 PR 須更新需求與 guard，保留「core/engine 無 I/O 依賴」檢查；不得簡單刪 guard。不要先建沒有 consumer 的 workflow/adapters 空 crate。

對pure native slice另跑沒有Node/Python/FFmpeg的production CLI smoke。TS shadow與官方OTIO Python oracle屬驗證工具，不因此被包入Rust production。宿主測試要綁host binary/version/hash/profile與新隔離專案。

## 5. 可直接複製的總控 Prompt

```text
Repository: bynanci/storytelling
Read AGENTS.md, TASKS.md, ROADMAP.md, docs/SDLC.md,
docs/migration/OPENFILM_RUST_PORT_MAP.md,
docs/migration/NATIVE_BOUNDARIES_AND_GAPS.md,
docs/migration/openfilm-port-index.json.

本次只完成指定 TASK_ID 尚未接受的一個可驗收切片；先讀 accepted lifecycle，
不重做已合併的 M0、有限 M1-01/M1-02、bounded M2-01 或 M1-03 proof。
1. 重新核實 storytelling main、相依 open PR、已接受交付及 OpenFilm main；記錄完整base/source SHA與保留的重現baseline。
2. 在來源快照讀實際source/tests與既有Rust行為，先列already/partial/missing/blocked。
3. 完成GAP-01/02/03/16相關門檻。MIT LICENSE不改；Apache來源內容保留義務。
4. 用task table提供的Rust落點作設計起點，不把planned file當成已存在。
   在G2凍結本次的exact signatures、DTO、error、clock與side-effect邊界。
5. 先寫失敗測試再以idiomatic Rust實作；pure engine不得呼叫Node/Python/SQLite/host。
   需外部工具的adapter明示依賴，不冒充純Rust；不重寫codec/GUI/完整NLE。
6. 留下source-to-Rust comparison record與RED/GREEN/differential結果；保留guard。
7. 完成同head測試與CI；缺宿主/工具/權利/素材就標blocked，不自行推定通過。
8. 建立單一範圍 Draft PR 並讀同-head CI；若本次已有明確合併授權且所有必要門檻通過，
   合併、勾選並關閉完成任務。缺乏合併授權時保留 PR；release/tag/repo policy 另須授權。

完成回報：task+requirements、source/target SHA、程式/測試映射、實際native邊界、
使用方式、差異分類、驗證證據、人工gate與下一個可做task。
```

## 6. PR策略與生命週期

歷史規劃 PR #2 只修改 docs/TASKS/ROADMAP/AGENTS 與 JSON ledger，當時 stack 在 M0 PR #1 之上；兩者現已合併。後續 PR 依實際 main 與相依交付選 base，不把原始 stacked 狀態當成仍未合併，也不改寫歷史 inspection 身份。

實作PR以結果命名：M1-01 snapshot mapping、M1-02 finite OTIO、M1-03 host proof；每支只完成自己需要的OF/GAP子集。研究native替代與真正替換分開；沒有證明品質/相容性不要刪既有FFmpeg adapter。

每支PR的rollback是停用新consumer/保留舊路徑或隔離產物。不能用刪除使用者專案、強制push主線、或對宿主盲目Undo宣稱回退成功。
