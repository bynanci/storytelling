# M1-01 工作與任務

[English](M1-01-WORK-QUEUE.md)

本切片將上游既有的 composition JSON 匯出資料 唯讀轉成有限、可由 M0 驗證的
visual plan。實際來源、差異與驗證見 [comparison record](records/M1-01.md)，
精確輸入／輸出／錯誤契約見 [finite-v1](FINITE_V1.md)，任務連結見
[工作索引](m1-01-work-index.json)。

## 實作階段核實基線（歷史，2026-11-07 UTC）

| 項目 | 完整 SHA / 狀態 |
| --- | --- |
| Storytelling main | `551ff38920e2085e1d53a7ba777c9680678e9725`；尚無 workspace |
| M0 / PR #1 | `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`；open Draft、未合併 |
| 本 PR 的 base / 規劃 PR #2 | `95291d31c47601181070bf89a28b1a182ddf6ed8`；open Draft、stack 在 #1 |
| 固定的上游版本 | `7add4762fe8174d76f43b863375ea39b73121e76` |
| 查詢時的上游 main | `5f737a6c8f35dce48e213370ff27900962c9304c`；PR #126 已合併 |

附件與原 ledger 的 Draft/watch-only 是固定來源快照時的歷史狀態。本次未默默改用新
main；新增 animation 以 unsupported regression 防止無聲遺失。PR #1/#2 各自 head
查詢到的 Quality、三 OS、Required gates 均成功，但不代替本 PR 的同 head CI。

## 已接受與合併的生命週期

上述表格保留當時 source/base/head 與 Draft 狀態，不能當作目前 PR 狀態。
使用者後續明確授權合併已完成且驗證通過的 PR，並勾選已完成任務：

| PR | main merge commit | 合併時間（UTC） |
| --- | --- | --- |
| #1 M0 | `22279814ff6b147c386186569d7f77f3a803cf8b` | 2026-11-07 02:17:16 |
| #2 planning | `9fb403be811643baf948acdb6d72232d4a6207bb` | 2026-11-07 02:17:48 |
| #7 M1-01 | `610ed49673d6f554e539008b34f5fc5479b9dbc1` | 2026-11-07 02:18:09 |

M1-01/P1、P2、P3 皆 accepted，issues #3–#6 已勾選並關閉。這只接受已驗證的 finite
visual mapping；不代表 OTIO、實際來源／audio／FilmCraft、發布或人類剪輯驗收完成。

## 工作分解

| ID | 狀態 | 交付 | 依賴／驗收 |
| --- | --- | --- | --- |
| M1-01/P1 | accepted | 固定來源、權利、原 producer 與 finite-v1 契約 | GAP-01/02/03/16；獨立 G2 review，source oracle，逐檔 provenance |
| M1-01/P2 | accepted | 純 Rust mapper、精確 clocks、immutable snapshot、loss | P1；實際 RED→GREEN；每個來源 occurrence 保留，回傳 M0 plan 可驗證 |
| M1-01/P3 | accepted | 可用 file/stdin CLI 與 JSON/exit 契約 | P2；CLI process regressions、乾淨 PATH smoke、debug/release/quality/CI |
| M0-G01 / PORT-G01 | accepted | 維護者接受 foundation、規劃與 provenance policy | 使用者明確授權後合併 #1/#2；並非以實作或 CI 自行代替批准 |

2026-11-07 使用者已批准來源衍生 component 政策：原 MIT LICENSE 不變，來源衍生
modules/tests/fixtures 採 Apache-2.0、逐檔 provenance、適用 LICENSE/NOTICE；其餘整合
逐檔分類。見 [清單](provenance-m1-01.json) 與 [PROVENANCE](../../PROVENANCE.md)。
政策批准與 PR 接受是不同 gate；本次後續接受／合併的實際紀錄列於上方，
不能進一步推定發布或未驗證宿主的能力。

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
