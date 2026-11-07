# M1-01 工作與任務

本切片將既有 OpenFilm composition JSON export 唯讀轉成有限、可由 M0 驗證的
visual plan。實際來源、差異與驗證見 [comparison record](records/M1-01.md)，
精確輸入／輸出／錯誤契約見 [finite-v1](FINITE_V1.md)，任務連結見
[工作索引](m1-01-work-index.json)。

## 核實基線與目前狀態（2026-11-07 UTC）

| 項目 | 完整 SHA / 狀態 |
| --- | --- |
| Storytelling main | `610ed49673d6f554e539008b34f5fc5479b9dbc1`；M0、移轉索引與 M1-01 已在主線 |
| M0 / PR #1 | head `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`；merge `22279814ff6b147c386186569d7f77f3a803cf8b` |
| 移轉索引 / PR #2 | head `95291d31c47601181070bf89a28b1a182ddf6ed8`；merge `9fb403be811643baf948acdb6d72232d4a6207bb` |
| M1-01 / PR #7 | head `c2fb478c278550b682efba5f898082e8bcd20e24`；merge `610ed49673d6f554e539008b34f5fc5479b9dbc1` |
| 固定 OpenFilm 來源 | `7add4762fe8174d76f43b863375ea39b73121e76` |
| M1-01 實際比對的 OpenFilm main | `5f737a6c8f35dce48e213370ff27900962c9304c`；PR #126 已合併 |

PR #7 的 exact-head push 與 PR-integration 均通過 Quality/contracts、Linux、Windows、
macOS Rust jobs 與 Required gates：[push 42327081582](https://github.com/bynanci/storytelling/actions/runs/42327081582)、[PR 42327143353](https://github.com/bynanci/storytelling/actions/runs/42327143353)。
Draft/watch-only 描述只代表初始快照的歷史狀態；M1-01 僅接受 finite-v1 visual profile，
明確拒絕新增 animation。它不涵蓋 OTIO、media source-byte 驗證或 FilmCraft host proof.
## 工作分解

| ID | 交付 | 依賴／驗收 | 狀態 |
| --- | --- | --- | --- |
| M1-01/P1 | 固定來源、權利、原 producer 與 finite-v1 契約 | GAP-01/02/03/16；獨立 G2 review，source oracle，逐檔 provenance | 完成；PR #7 合併 |
| M1-01/P2 | 純 Rust mapper、精確 clocks、immutable snapshot、loss | P1；實際 RED→GREEN；每個來源 occurrence 保留，回傳 M0 plan 可驗證 | 完成；PR #7 合併 |
| M1-01/P3 | 可用 file/stdin CLI 與 JSON/exit 契約 | P2；CLI process regressions、乾淨 PATH smoke、debug/release/quality/CI | 完成；PR #7 合併 |
| M0-G01 / PORT-G01 | 維護者接受 foundation 與移轉邊界 | PR #1/#2；source-derived Apache component policy in PR #7 | 接受；PR #1/#2/#7 合併 |

2026-11-07 使用者已批准來源衍生 component 政策：原 MIT LICENSE 不變，來源衍生
modules/tests/fixtures 採 Apache-2.0、逐檔 provenance、適用 LICENSE/NOTICE；其餘整合
逐檔分類。見 [清單](provenance-m1-01.json) 與 [PROVENANCE](../../PROVENANCE.md)。
政策批准與 PR 合併分別記錄；目前 PR #1/#2/#7 已合併，具體 SHA 列於上表。

## 本次邊界與後續

core/engine 仍無 I/O/runtime/media/host dependencies。DTO／loss 契約放現有 core，
mapper 放 engine，CLI 僅負責顯式 JSON input 與 stdout。OF-04 ledger 的 adapter 路徑
是未來設計入口，本次不建空 adapter crate。

所有成功結果仍為 `declared-visual-mapping-only`、`executable:false`。video 帶
`audio.not_mapped / blocks-host-handoff`；明確 audio/effects/animation 拒絕且沒有 plan。
stills 的 mapped duration 是合成 hold window，原始宣告完整保留。29.97 的精確 decimal
解讀相對 upstream NTSC 推導列為 intentional delta。來源 bytes／SHA 不升級 verified。

OF-03 只完成不可變輸入、順序與 number identity 子集；canonical digest 留待後續，
沒有使用一般 JSON serialization 冒充 canonicalization。48 OF /16 GAP 仍是覆蓋索引；
六個 retain-host 項目不變成重寫任務，M2–M5 沿用原 TASKS。

下一步是 M1-02 有限 OTIO＋loss profile／官方 oracle，接著 M1-03 真實授權來源與
隔離 FilmCraft import/render/readback/save/reopen。這兩項不由合成 CI 關閉。
回退可停用新 CLI command 或 revert 本切片；不寫入來源 Catalog、使用者專案或宿主。
