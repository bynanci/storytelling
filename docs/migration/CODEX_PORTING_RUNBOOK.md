# Codex：OpenFilm → Rust 單項移植 Runbook

本文件是可複製執行契約。來源對照在 [port map](OPENFILM_RUST_PORT_MAP.md)，機器索引在 [JSON](openfilm-port-index.json)，SDLC仍由 [SDLC](../SDLC.md) 管理。

## 1. 開始前必須核實

- Source baseline：bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76。
- Target main at inspection：551ff38920e2085e1d53a7ba777c9680678e9725；M0 PR #1 head：4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b，當次仍Draft。
- 重新fetch兩個repo與open PR。若M0未合併，只有明確相依branch能使用它；不能對只有README的main假設三crate存在。
- 固定來源已含coverage PR #121、frame time PR #125；PR #126 在最初規劃快照是 Draft，2026-11-07 重查已 merged，main 為 `5f737a6c8f35dce48e213370ff27900962c9304c`。M1-01 保留原 pinned source，見 [drift record](records/M1-01.md)；新的狀態仍由實際查詢更新。
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

既有M0 `check_repo.py` 目前只允許三crate與精簡dependencies。新增真實adapter的同一PR須更新需求與guard，保留「core/engine無I/O依賴」檢查；不得簡單刪guard。不要先建沒有consumer的workflow/adapters空crate。

對pure native slice另跑沒有Node/Python/FFmpeg的production CLI smoke。TS shadow與官方OTIO Python oracle屬驗證工具，不因此被包入Rust production。宿主測試要綁host binary/version/hash/profile與新隔離專案。

## 5. 可直接複製的總控 Prompt

```text
Repository: bynanci/storytelling
Read AGENTS.md, TASKS.md, ROADMAP.md, docs/SDLC.md,
docs/migration/OPENFILM_RUST_PORT_MAP.md,
docs/migration/NATIVE_BOUNDARIES_AND_GAPS.md,
docs/migration/openfilm-port-index.json.

本次只完成指定 TASK_ID 的一個可驗收切片，預設由 M1-01 開始。
1. 重新核實 storytelling main、PR #1 及 OpenFilm main；記錄完整base/source SHA。
2. 在來源快照讀實際source/tests與既有Rust行為，先列already/partial/missing/blocked。
3. 完成GAP-01/02/03/16相關門檻。MIT LICENSE不改；Apache來源內容保留義務。
4. 用task table提供的Rust落點作設計起點，不把planned file當成已存在。
   在G2凍結本次的exact signatures、DTO、error、clock與side-effect邊界。
5. 先寫失敗測試再以idiomatic Rust實作；pure engine不得呼叫Node/Python/SQLite/host。
   需外部工具的adapter明示依賴，不冒充純Rust；不重寫codec/GUI/完整NLE。
6. 留下source-to-Rust comparison record與RED/GREEN/differential結果；保留guard。
7. 完成同head測試與CI；缺宿主/工具/權利/素材就標blocked，不自行推定通過。
8. 建立單一範圍Draft PR；不自動merge、release或變更repo policy。

完成回報：task+requirements、source/target SHA、程式/測試映射、實際native邊界、
使用方式、差異分類、驗證證據、人工gate與下一個可做task。
```

## 6. PR策略與生命週期

本規劃PR只修改docs/TASKS/ROADMAP/AGENTS與JSON ledger，stack在M0 PR #1之上。M0合併後由維護者決定rebase/retarget；不要默默把PR #1合進main。

實作PR以結果命名：M1-01 snapshot mapping、M1-02 finite OTIO、M1-03 host proof；每支只完成自己需要的OF/GAP子集。研究native替代與真正替換分開；沒有證明品質/相容性不要刪既有FFmpeg adapter。

每支PR的rollback是停用新consumer/保留舊路徑或隔離產物。不能用刪除使用者專案、強制push主線、或對宿主盲目Undo宣稱回退成功。
