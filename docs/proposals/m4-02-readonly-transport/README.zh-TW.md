<!-- SPDX-License-Identifier: MIT -->
# #108 · 唯讀 TypeScript transport G2 候選

[English](README.md) · [ADR 0014](../../adr/0014-readonly-process-transport-v1.md) · [決定與證據](../../validation/m4-02-transport-g2.md)

**尚未接受，也沒有可執行的 transport／TS client。** 工程建議是 Node.js
本機 client，每次唯讀呼叫啟動一個受控程序，以明確長度傳送原始 bytes。
使用者已選定這個方向；完整欄位、framing、診斷及生命週期仍待 G2 審查，
這次方向選擇不代表核准新契約或開始 G3。

## 審查範圍

- [Protocol](protocol.md)：八個 facade 操作加 describe、STRO 框架、版本、上限、
  EOF、錯誤優先順序、取消／逾時／程序失敗。
- [TypeScript 型別](types.md)：client 自有結果與錯誤；Narrative projection
  新增的是 binding view，core 不加 serializer、不改既有 V1。
- [驗收計畫](test-plan.md)、[案例索引](cases.json)及[範例收據](examples/manifest.json)：
  正反例是提案資料，不是跑過產品後擷取的 GREEN。
- 三份候選 schema 僅放在本目錄，不能當成已接受 `contracts/` 的格式。
- [靜態重播命令](verification.md)核對三份 schema、21 份範例、13 個原始 Git
  身分與 64 項待執行案例；不能把靜態結果當成 transport 已通過。

## 精確候選與需審查的取捨

框架是 `STRO + u32 big-endian header 長度 + UTF-8 JSON header + 原始 buffers + EOF`，
不加 LF。Header 上限 16 KiB，request frame 16 MiB、response frame 64 MiB；
這些限制是 wire budget，不是程序記憶體上限。每個 client 同時只處理一個呼叫，
沒有自動排隊／重試。預設 wall deadline 30 秒，可明確設為 1–120,000 ms，
包括複製、啟動、等待、解碼與結束；取消後另有最多一秒的清理預算。

先檢查總長度再複製 bytes，超量回 `client.request_limit_exceeded`，不啟動程序。
若子程序提早回合法拒絕，stdin 的 EPIPE 先保留，等 response／EOF／exit 0
再分類；不能因為寫入先斷掉就遺失正確錯誤。未確認程序結束則鎖住該 client，
不啟動替代程序，也不推論 Host 已停止。

G3 候選是新增預設關閉的 `readonly-transport` feature 與
`storytelling sdk-readonly-v1`。為實作嚴格 header visitor，提議 CLI 直接使用
既有 workspace `serde =1.0.229`，並在該實作 PR 明確調整 dependency whitelist；
沒有新增 graph 版本，目前 manifest／whitelist 完全未改。此項亦屬待採納 G2。

Wire／facade／view／domain／profile 各自版本化。Package／compiler 字串原樣保留，
不拿 package patch 當成相容性白名單；compiler freshness 仍由原 backend 判定。
三平台 Node 與實際 client 測試必須等 G3，現有 CI 不能代替它們。

同一份 proposal-response、bundle 與 OTIO 必須原樣傳遞，不 trim、不轉換換行、
不 parse/re-encode，也不把 rational strings 轉成浮點數。Planner 的 typed
result 與 review 所需的完整 response wrapper 分開命名；只建立一次 wrapper，
選定後一路沿用該 bytes。既有 CLI 加的一個 LF 與本 transport 的無 LF framing
各自有契約，不能互相當成相同 bytes。

`prepare_story_review` 僅把同一個 Rust compiler-owned 值的 record／artifact／bundle
交出去。TS view 是不受信任的可攜資料，不能反向建構可信 handle、取得核准或執行權。
Unresolved 與 stale 仍是成功的 domain 結果，不能因可解析就宣稱素材／Host 已驗證。

## 現在可執行的既有參考

```sh
cargo run --manifest-path examples/consumer-sdk-v1/Cargo.toml --locked -- \
  tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json
```

這是已接受的 Rust consumer，預期包含 `video_coverage.unresolved_required` 及
`ready_for_planning:false`。候選的 `storytelling sdk-readonly-v1` 及 TS client
尚未實作，不能把文件中的 proposed 命令當成目前成功證據。

## 接下來的門檻

本 PR 只交付 G0–G2 設計。維護者針對 exact head 接受範圍、framing、view、
診斷與生命週期後，才展開獨立 G3 RED／GREEN。#108 的實作驗收框維持未勾選。
之後須重跑三平台 regression、TS 與 Rust 的同 fixture 等價，以及 fresh-checkout
範例。現有 npm root 是 Skill 套件；SDK client 要獨立、未發布且有自己的 lockfile。

取消或逾時只處理本機唯讀子程序，不證明編輯器停止。沒有自動重送、Undo、
作者自我核准、媒體探測、Host 寫入或發布。#103–#106、#109 及 G5 各自保留。
新真人、真實媒體與 Host 觀察均為 0。
