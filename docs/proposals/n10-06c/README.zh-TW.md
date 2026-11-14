<!-- SPDX-License-Identifier: MIT -->
# N10-06C · 本機素材身分預檢候選

[English](README.md) · 繁體中文 · [Issue #129](https://github.com/bynanci/storytelling/issues/129)

**狀態：獨立 G2 契約／威脅模型提案；adapter 實作、OS／工具資格及真實檔案存取均受阻，等待另外的維護者／隱私範圍核准。** 本份只處理 C 設計。準備時 base 為 `52dc6c315f7b3d20693f3c51e052509997ab791b`。[候選 sidecar schema](sidecar-v0.schema.json)、[合成 unknown 範例](synthetic-unknown.example.json)及[待執行案例](cases-v0.json)是審查材料，尚非已接受的公開格式或觀察結果。

## 作者結果與既有邊界

作者要知道宣告的 `asset_id@revision` 是否仍對應自己選取的精確本機檔案，以及所選來源區間是否符合可觀察的影片時間域。檔案可讀、雜湊相符、metadata、作者確認內容、宿主能力與執行核准各自回答不同問題，必須分開呈現。

已接受的 [coverage 契約](../../adr/0010-narrative-video-coverage-v1.md)只驗證宣告。[N10-05](../../validation/n10-05-execution-record.zh-TW.md)曾讀取提供的合成 MP4 metadata，沒有把該檔案可信地綁定 JSON asset ID。[PR #130](https://github.com/bynanci/storytelling/pull/130)是 A 的精確輸入輔助；B 的 advisory UX 另行處理。C 不更動 V1 欄位、診斷或命令。core／engine 不新增媒體存取。

既有 `verify_supplied_sha256` 只驗證呼叫者已持有的 bytes，限制 16 MiB；它不開啟路徑、不識別素材、不授權讀取，也不保證可變檔案在 probe 期間保持不變。`RationalTime` 可驗證 reduced nonnegative u64 字串。[FilmCraft oracle](../../../tests/oracles/filmcraft-25-v1/README.md)提供明確鎖定工具及隔離執行的參考，但它不是此 C adapter 或正式檔案信任邊界。本提案未複製第三方實作或測試。

## 候選契約與資料所有權

版本為 `n10-06c.local-sidecar.0-candidate`。sidecar 與其證據保存在私人本機儲存區，不放入 portable V1 或公開 repository，也不是存取憑證。未來 parser 須在查詢任何 capability 前，拒絕未知版本、未知欄位、重複 JSON key、錯誤型別及上限。schema 只檢查 shape；以下關聯條件仍需 G2 後的程式與測試。候選 sidecar 輸入限制 262,144 UTF-8 bytes、16 層容器、32 個診斷、128-byte 不透明 ID、canonical u64 有理數字串及 i128 範圍 signed ticks，先做 duplicate-key preflight。未來 adapter 回傳私人記憶體報告；持久化另需呼叫者明確指定的 evidence destination，沒有隱含媒體寫入。私下 evidence refs 不可取得時不得重用。

| 欄位組 | 候選意義 |
| --- | --- |
| `coverage_ref` | 精確 coverage bytes SHA-256、catalog ID/revision、asset ID/revision、宣告片長與半開來源區間。任一改變就不得沿用舊 binding；不自動更新 Catalog 或 revision。 |
| `selection_ref` | 可信任本機 chooser 針對使用者明確選取的一個 regular file 發出的 `grant_id`、`root_scope_ref`、`selected_file_ref`。JSON 不接受路徑、URL 或命令。這些是不透明報告參照，不能反序列化成可重用 handle／權限。 |
| `file_access` | 該次是否成功唯讀開啟並讀取所選檔案物件。存在／可讀不等於指紋相符。 |
| `stability` | 前後檔案身分及已具資格的 read-view／lease profile。相同 stat 或相同前後雜湊不能單獨證明 probe 期間不可變。 |
| `byte_identity` | 完整檔案前後 SHA-256、byte length，以及**另行提供**的該 asset revision 預期指紋及來源。第一次觀察若無已授權 expected digest，維持 `not_verified`；不能把 observed 填成 expected 後宣稱相符。 |
| `metadata` | 工具／probe 證據參照、明確選取的影片 stream、精確 time-base／ticks、timing 與 boundary basis。不能因此證明解碼、播放、同步或敘事內容。 |
| `duration_check`／`range_check` | 分別根據合格的精確觀察判斷相容性。不自動夾限或改寫片長／區間。 |
| `content_review` | 獨立的人工作業參照；雜湊與 metadata 不能自動設為 confirmed。載入 sidecar 也不驗證填寫者身分。 |
| `binding_state` | `not_verified`、`bound_for_read_view`、`unknown`、`rejected`。沒有整體 `media_verified` 或 export-ready 狀態。 |

未來的 `local_observation` 只有在以下條件同時成立，才能填 `bound_for_read_view`：目前選取 grant 相符、已接受且測試的穩定 read profile、整個操作維持同一檔案物件、完整前後雜湊等於獨立 expected digest、合格片長／區間觀察相容，以及不可變證據參照。保證只涵蓋該次 read view。內容、宿主及 authority 維持獨立；序列化的 bound 值重新載入後仍是不可信資料，不能恢復 grant。

未來 validator 的關聯規則：完成的檢查都須有非 null 私人 evidence ref；`matches` 須有獨立 expected/ref、完整前後 hash 及 byte length，三個 digest 相等。stable view 須有已接受 profile／read-view／identity 證據。local observation 填真實 UTC 時間，synthetic 範例保留 null；unknown／rejected 保留固定診斷。匯入的這些狀態，在 protected evidence 檢查前仍不可信；JSON Schema 不能驗證相等或 OS 資格。scope／process 不安全失敗立即停止讀取，未執行檢查保持 not_verified；已取得的觀察保留，不升級 binding。

相同 asset ID／revision 若 bytes 改變，須保留 mismatch、拒絕舊 binding 重用及保留舊證據。選取檔案、來源 revision、區間、工具／profile 或 expected 證據 revision 改變時建立新 attempt；舊回應不得覆蓋新狀態或刪除失敗紀錄。

## 可信任選取與穩定讀取流程

以下全為**未實作／未具資格的計畫**。

1. 由可信任本機 UI／CLI chooser 取得使用者明確選取及 narrow root scope。adapter 接收持有的唯讀 file capability，不根據任意 JSON 路徑、名稱、副檔名或 asset label 自動選檔。
2. 只開啟該本機 regular-file 物件，以 OS handles 驗證所有路徑元件。拒絕 symlink／reparse point、`..`、網路／UNC share、device／FIFO／socket、跨 mount／目錄逃逸與未支援 alias。字串 canonicalize 後再開路徑仍有競態。初始 profile 拒絕多個 hardlink，除非另有 alias 資格測試。
3. 建立已具資格的 OS 強制 immutable snapshot 或有效 deny-write/delete lease；此 adapter 不寫入或複製媒體檔案。advisory flock、唯讀 descriptor、唯讀屬性與前後 stat 都不足以證明穩定。已有 writable handle／mapping、rename／delete 競態須由 OS profile 測試覆蓋。無法證明時，僅能保留觀察並將 stability／binding 設為 unknown，或停止。
4. 保留安全的本機 file-identity refs 及大小，對**同一完整 read view** 做有界串流 SHA-256。不能為大檔案直接放寬既有 16 MiB buffer API；未來 streaming adapter 仍是獨立本機 I/O，不暴露可偽造 `VerifiedBufferV1` 的 constructor。
5. controlled probe 使用持有的 view／descriptor，不重新開啟原 pathname。若工具／OS 不能在 sandbox 內消費該物件，就拒絕該 profile。改用 pathname fallback 會失去 binding，只能 unknown。
6. probe 後完整再 hash 並核對身分／大小。替換、截斷、寫入、lease 遺失、短讀或差異均使結果失效。相同前後 SHA 是額外觀察，缺少穩定 view 時無法防止改寫後還原的 ABA。
7. 以精確整數／有理數比較 expected 指紋、片長及區間，分別留存結果後釋放 capability。失敗／unknown 不刪除；再試須新的明確選取／操作決定與 attempt ID，不能把 unknown 改成成功。

本提案沒有替任何 OS 賦予 stable-read 資格。Linux／Windows／macOS 上的一般可變檔案不能只因兩次 hash 相符就被標為 bound。

## 受控 `ffprobe` 邊界

未來 adapter 必須鎖定絕對的可信執行檔身分／版本／hash，以 process API 逐項 argv 呼叫（`shell=false`／Rust `Command::arg`），不得 shell interpolation、`sh -c`、PATH 搜尋或使用者自訂旗標。清理 loader／plugin／proxy 環境，固定 locale 與最小 env；不改 HOME 或 repository／系統設定。

候選 transport 使用唯一所選 read view 經 stdin／`pipe:0` 傳入。只 allowlist 必要的 pipe 協定及 demuxer，另以 OS sandbox 禁網路，並禁止子行程開啟其他媒體。protocol／format flags 本身不是 sandbox；相對參照、playlist、URL、parser 觸發的旁路讀取都必須失敗。工具／library 讀取另有明確 allowlist。無法提供可靠 seek／access 的 container 回報 unsupported／unknown，不改走檔案路徑或下載。

英文文件列出具體候選 argv：metadata 只要求固定的 format、stream、stream disposition 欄位；另一次有界 timestamp pass 針對已驗證的 stream index 使用 `-show_packets`，只回傳 `stream_index,pts,duration`。兩次 process 都從同一 qualified view 重設起點並傳入，不建立臨時媒體副本。argv 是分開的引數，沒有命令字串。工具 build、欄位、demuxer、descriptor 行為及 OS 支援要在 C 的 G2 驗證並凍結；目前沒有 supported-host 主張。

metadata allowlist 排除檔名、tags、GPS、chapters 及 attachments。raw stdout／stderr 只留在私人 evidence store。公開診斷使用固定碼與 logical JSONPath，不回顯路徑、不可信 key 或 parser 文字。

候選每次 budget：一個檔案、268,435,456 bytes（256 MiB）、兩次完整 SHA pass、最多兩個 probe process、合計 30 秒、stdout 16 MiB、stderr 64 KiB、16 streams、100,000 packets／timestamp records；兩次 hash 與兩次 probe 的 adapter 媒體讀取／pipe delivery 合計不超過 1 GiB，每 pass 不超過檔案 byte length。在讀取時即 enforce caps，不能無界 capture 後才檢查。超時要終止受監督的 process tree 並確認 quiescence；不能確認就維持 unknown。禁止 partial-output success 或自動重試。這些上限均待 G2；尚無效能或真人可用性實測。

## 精確片長、stream 與來源區間

- 多個可用影片 stream 時，作者須明確選 index；不能默認第一個 stream 或把 attached picture 當所需 footage。缺少／越界／歧義選擇維持 unknown 或 rejected。
- `time_base`、i128 範圍 signed `start_pts` 及 end ticks 以字串保留。先以 checked signed arithmetic 減去 profile 定義的 presentation origin，再以 checked u128、GCD 及最終 u64 範圍檢查產生非負秒數。負起點無法解釋、overflow、discontinuity／gap 未支援時回 unknown。不得用浮點解析報告的 decimal duration 或 average fps。
- header 的 duration、duration_ts、nb_frames、r_frame_rate／avg_frame_rate 是宣告／估計觀察，不證明完整可觀察區間。compatible 需要有資格的完整邊界基礎，例如在上限內完整檢查所選 stream 的 packets／timestamps 且有 interpretation tests。部分掃描、缺 PTS／duration、截斷、解析器分歧或未具資格 codec 都回 unknown。packet PTS 仍不證明完整解碼、播放或影音同步。
- 候選嚴格比較：只有合格完整 interval 從 0 起算、exclusive end 與宣告片長完全相等，duration 才是 compatible；有合格矛盾證據為 incompatible，基礎不完整／未具資格則 unknown，不存在隱含容差或四捨五入。range 另判定 `[start,end)` 是否落在合格連續 interval 且通過原 V1；profile 未解釋的 gap／retiming 維持 unknown。較短的宣告片長可能含安全區間，同時不通過獨立的片長相等檢查。保留 origin／precision／basis 及各自矛盾，不改寫 Catalog。
- VFR 在另行接受 per-frame PTS 設計前不支援 frame index。秒數也須有合格 observed interval；nominal／average fps 不證明固定 cadence。多 stream、edit list、retiming 需 profile 明定與測試。
- 作者仍須確認畫面對應敘事單元；完全相符的 hash 也可能識別了錯的素材。

## 威脅、驗收與停止條件

[待執行案例](cases-v0.json)涵蓋 qualified immutable-view 正例，以及偽造／重播參照、scope escape、檔案不存在、bytes 改變、rename 替換、ABA、lease 遺失、工具旁路讀取、budget／截斷、stream 歧義、VFR、header-only 片長、實際 OUT 超界、無 expected digest、內容不符及未知格式。每列的 observed_result／receipt_ref 都留 null；沒有把規格當成果。

候選固定碼屬獨立 `media_identity.*` namespace，不加入 V1：input_too_large、unsupported_version、invalid_sidecar、selection_required、scope_rejected、unreadable、unsupported_profile、stability_unproven、file_changed、fingerprint_mismatch、probe_failed、budget_exceeded、stream_ambiguous、timing_unknown、duration_incompatible、range_incompatible。候選 parse error 為 `{"error":{"code":"media_identity.invalid_sidecar","path":null}}`，不回顯 parser 文字。原始 byte／depth／duplicate／syntax 檢查先於版本與 closed shape，再查 capability；缺 selection／scope 被拒時在 probe 前停止，之後未執行檢查維持 not_verified。完整 sidecar 只記錄取得的觀察，不授權寫入。最終路徑與診斷順序須由 C G2 接受。

實作前維護者須接受 sidecar 欄位／狀態、選取／隱私範圍、OS 穩定 view 與 sandbox profile、工具 identity／argv／上限、精確時間詮釋及診斷。之後先保存 handle 替換、ABA／lease loss、budget、unknown 傳播的真正 RED，再實作 GREEN；每個受支援的 OS／工具組合都需實際證據。schema／範例可離線做 shape check，但不因此取得檔案或 adapter 資格。

本設計沒有執行 probe、實際檔案 hash、作者操作、Host readback、G2 核准或發布。#117 已以提案文件合併為 `a23a2909cf3734d207c66b372f29a85c5e541c86`，#103 technical G2／W01 及 #104–#106 的執行／宿主門檻仍待決；#107–#109 管理日後公開 consumer 整合。

若無法建立可靠 read-view binding／containment、需要未支援 VFR／stream／codec、budget 使觀察不完整，或宣告輔助已足夠，就縮小或停止 C，保留 unknown／not_verified。回退移除 optional adapter／sidecar reader；既有 V1／A 仍可用，沒有媒體／Catalog／專案修改需要 Undo。
