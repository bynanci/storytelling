<!-- SPDX-License-Identifier: MIT -->
# N10-06A 重播 oracle 修正 · 2026-11-17

[English](n10-06a-replay-oracle-2026-11-17.md) · [目前演練](../../examples/evaluation/n10-06a-rehearsal/README.zh-TW.md) · [原 PR #145 證據](n10-06a-roundtable-2026-11-17.zh-TW.md) · [收據索引](n10-06a-replay-oracle-2026-11-17/receipt-index.json)

Base：`91bf8f80139f59c0406340eb3565d296bd562931`。本次修正合成評估工具。未修改的原生 CLI 產生正確時間範圍，原有四項測試全部通過；沒有宣稱發現原生轉換錯誤。

## 失敗與修正

舊重播僅比較 file／stdin／repeat 及 preview／emit／既有 V1 的一致性，未以獨立答案核對有效時間範圍。若一致地把合法起點 `5/2` 改成 `2501/1000`，八案／37 次呼叫仍被標為通過。V1 驗證器能接受兩種合法宣告，不能判定 draft 時間是否轉換正確。[保留的誤判通過收據](n10-06a-replay-oracle-2026-11-17/baseline-false-pass.json)刻意包含注入輸出，不是原生算術錯誤的觀察。

現在以 Python 標準函式庫 `Fraction` 獨立計算秒數 2.5–4.5、0–0.1、30/1 fps 的 [75,135)，以及 30000/1001 fps 的 [0,150)。檢查每個有效 mapped 列的兩端點，包含 `declaration-complete` 的第二列，並固定 S02@1／S03@1 的 selected 與 mapping 參照、mapped 狀態／素材參照及原 unresolved 原因。原有原始輸出、emit／V1 一致性及原生首錯診斷均保留。不修改 Rust、產品 schema、fixture 或作者材料。

## 實際 RED 與 GREEN

最終測試 shim 在 Python 子程序中呼叫真正的原生 CLI invocation，再對成功 helper 輸出與其記錄的原始位元組注入單一、一致的 JSON 故障，並原樣執行 `main()`。preview／emit／validation 同步變更；原互相比對漏掉端點／原因故障。兩個參照變動則原已由後續 V1 驗證器拒絕。正式程式沒有注入 hook；這不是不可信執行檔的隔離測試。

- [原始基線](n10-06a-replay-oracle-2026-11-17/baseline-tests.log)：明確傳入原生 binary，原有四項全部通過。
- [RED](n10-06a-replay-oracle-2026-11-17/oracle-red-portable.log)：最終測試對未修改的 base 重播執行七個方法；三個回歸方法內共有 18 個預期的 subtest 失敗，原有四項仍通過。其中 16 個端點／原因故障觀察到錯誤成功退出；兩個參照案例原已在後續 V1 順序檢查以 `unexpected_exit` 失敗，新測試因其診斷不同而失敗，修正後要求更早的 `wrong_canonical_declarations`。後兩者是提早檢查，不是額外兩個原本誤通過案例。
- [GREEN](n10-06a-replay-oracle-2026-11-17/oracle-final-green.log)：七個方法全過且無跳過，涵蓋 12 個端點、兩個參照與四個 unresolved 原因故障。
- [正確原生重播](n10-06a-replay-oracle-2026-11-17/native-green.json)：八案／37 次呼叫通過。[修正後的起點故障](n10-06a-replay-oracle-2026-11-17/mutant-rejected.json)：以 `wrong_canonical_declarations` 失敗，保留一個實際嘗試案例及三次呼叫。

另一次相容性檢查發現既有 test driver 將 unittest 的 `discover` 參數誤當執行檔路徑（[實際 19 個失敗](n10-06a-replay-oracle-2026-11-17/discovery-red.log)）。現在只有直接執行測試檔才讀取 positional binary。[未設定 binary 的 discovery](n10-06a-replay-oracle-2026-11-17/discovery-green.log) 執行七個方法，其中四個原生方法明確跳過；這與必要的明確原生執行分開，後者七項全過且無跳過。

可讀 RED log 僅移除行尾空白；原始位元組保存在同名 `.log.gz`，未壓縮雜湊列於收據索引。完整 discovery RED 重現加入參數 guard 前的 driver。

使用有 feature 的實際原生路徑執行：

```sh
python3 examples/evaluation/n10-06a-rehearsal/test_replay.py /path/to/storytelling
python3 scripts/check_repo.py
```

[故障設定](n10-06a-replay-oracle-2026-11-17/range-fault.json)與[測試 shim](../../examples/evaluation/n10-06a-rehearsal/mutant_cli.py)可透過 `python3 mutant_cli.py CONFIG REPLAY --cli NATIVE --source-commit SHA --output NEW_RECEIPT` 重現單一起點故障。各路徑需完整指定；收據使用新路徑，既有觀察仍禁止覆寫。

## 證據限制

PR #145 的圓桌、初次／主要機器收據及 RED／GREEN logs 位元組不變；本次證據另存新目錄。原有 planned／attempted 計數修正維持不變。重用先前 binary 及原宣告建置 source；重播工具仍明示不驗證 source／build 綁定。未執行 Cargo 建置、網路或 Host 呼叫。新本機證據只涵蓋 Linux；可攜 Python shim 隨既有三平台 CI 的明確原生入口執行，但此紀錄沒有新觀察到的 Windows／macOS 結果。

這是 AI 的合成技術審查與故障注入，不是真人參與、作者時間、倫理審查、G2 接受或新研究結果。真人與 Host 觀察數仍為零。作者交接及 #129、媒體、authority、G5 的獨立門檻不變。

## 後續 JSON 型別回歸

獨立 review 重現 preview-only 的 true／1.0 參照版本在未變動的整數 emit 下仍通過，原因是 Python 相等比較會強制同值。[獨立型別收據](n10-06a-replay-types-2026-11-17/receipt-index.json)保留四個實際誤通過：bool／float 參照、bool catalog revision 與數字 readiness。`same_json` 比較排序後的 JSON，保留陣列順序及 bool／int／float 差異、拒絕非有限值；固定 oracle 與所有 JSON parity 都使用它，原始擷取 bytes 不變。最終八方法／22 故障無跳過通過；未設定 discovery 為三通過／五原生跳過。前文七方法／18 故障保留為當時收據，不主張原生轉換器有錯。
