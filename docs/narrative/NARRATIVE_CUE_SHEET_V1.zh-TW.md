<!-- SPDX-License-Identifier: MIT -->
# NarrativeCueSheetV1 提示表規格

**狀態：** N-09 有界實作 profile。

**JSON Schema：** [JSON Schema Draft 2020-12](../../contracts/narrative-cue-sheet-v1.schema.json)。

**範例及 CLI 證據：** [N-09 驗證紀錄](../validation/n09-cue-sheet.zh-TW.md)。

## 範圍

`NarrativeCueSheetV1` 是獨立、封閉且有版本的文件，綁定一份確切的 `NarrativeDocumentV1` revision 與一個 `spoken_outline` 變體。它不向已採納的 `NarrativeDocumentV1` 契約加入音訊、時鐘、片長或素材欄位。旁白提示引用 beat 與來源單元 revision；CLI 從該文件輸出 beat 已宣告的台詞。音訊素材是含儲存庫相對路徑、revision、摘要、來源與片長的宣告。引擎不讀取路徑、不驗證摘要、不播放音訊、不錄製或合成語音，也不混音。

驗證成功表示兩份文件識別資料相符、引用有效、提示順序及聲軌區間符合規格，且提供的 metadata 在上限內。這不代表音訊位元組符合宣告，或錄音聽起來正確。

## 時間與聲軌語意

V1 時間單位是毫秒。提示陣列順序與 `order` 欄位都從一開始，並逐一遞增。絕對區間採半開形式 `[start_ms, end_ms)`；因此前一提示結束時間等於下一提示開始時間時，屬於首尾相接而非重疊。狀態可為 `known` 或 `estimated`。

`after` 提示錨定先前 cue ID，並宣告已知、估時或未知的片長。只有錨點結束時間與提示片長都已解析，區間才會解析。`unknown` 時間保留原因並列在提示表中。未解析時間會計數，不會當成 0，也無法證明互斥聲軌沒有衝突。CLI 會揭露這項限制。

聲軌 ID 代表獨立的旁白、音效或標記軌。不同聲軌允許重疊，即使兩者都標為 `exclusive`。在同一 `exclusive` 軌上，只有完全解析且半開區間相交時才衝突。`mixable` 軌允許重疊；`marker` 軌承載非音訊製作指示，不參與混音。聲軌 channel 必須與 cue 類型相符：voice 軌承載旁白、sound-effect 軌承載音效；voice 或 marker 軌可承載靜音提示。

## 來源與修訂

提示表的文件識別資料與每個旁白 beat 的變體、beat ID、單元 ID 及單元 revision，必須與提供的 `NarrativeDocumentV1` 相符。過期的文件或 beat revision 會被拒絕。音效 cue 透過 ID 和 revision 引用已宣告音訊素材。素材 ID 與 cue ID 各自在各自集合中唯一。停頓長度等編輯選擇會標示為製作選擇，不當作來源事實。

引擎檢查宣告的 SHA-256 格式與安全相對路徑，但不計算摘要。需要核對實際位元組時，使用既有 `verify-source` 命令。N-09 範例會先以宣告摘要核對 `taps.wav`，才把其片長標為已知。

## 上限與診斷

輸入沿用 1 MiB JSON 上限，並使用敘事 reader 的重複鍵／深度前置檢查。此 profile 最多接受 32 條軌、128 個音訊素材及 512 個 cue；ID 最長 128 個 ASCII 位元組、revision 不可為零、宣告片長最多一小時。物件欄位封閉。未知欄位、未知版本、重複 ID、不支援的來源類型、過期引用、無效區間及互斥聲軌衝突都會以單行 JSON 診斷拒絕。

診斷包含 `narrative.cue.unsupported_version`、`narrative.cue.stale_document`、`narrative.cue.stale_revision`、`narrative.cue.invalid_range`、`narrative.cue.duplicate_id` 和 `narrative.cue.track_conflict`。衝突會列出兩個 cue ID。錯誤不會回傳不受信任輸入中的來源文字或路徑。

## 本機入口

在儲存庫根目錄執行：

```sh
cargo run --locked -p storytelling-cli -- render-cue-sheet \
  tests/fixtures/narrative-v1/n09-umbrella-source-v1.json \
  tests/fixtures/narrative-v1/n09-cue-sheet-v1.json
```

命令讀取兩個明確檔案，或一個檔案搭配 stdin 的 `-`，然後輸出穩定、可讀的提示表。成功退出碼為 0；無效宣告輸出一行 JSON 並以 1 退出；用法錯誤退出 2；輸入 I/O 錯誤退出 3。CLI 只讀取及轉換資料，不是音訊宿主。

可讀輸出的角色、來源及時間欄位使用 JSON 字串跳脫控制字元，讓它們留在原欄位內，不會變成額外 cue 行。
