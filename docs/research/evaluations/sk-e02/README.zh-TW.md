# SK-E02 · 凍結證據模式、必要主張與資料包勘誤

繁體中文 · [English](README.md) · [總追蹤 #115](https://github.com/bynanci/storytelling/issues/115) · [Issue #119](https://github.com/bynanci/storytelling/issues/119)

本資料夾將評審可用的證據邊界凍結在草稿產生之前。這是評估文件與離線工具，不新增 Rust、CLI 或產品 Schema，也沒有產生新的模型或真人結果。

## 契約

每次評估只能選一個 `evidence_mode` 字串：

- `packet_only`：只能使用已凍結的資料包位元組。草稿可能超出資料包範圍，即使該細節在現實中為真；不得因此直接裁定為捏造。
- `packet_plus_archived_sources`：可使用凍結資料包及明確列出的封存來源段落。來源無法取得時，狀態為 `unavailable`；取得失敗不能證明主張為假。

在產生任一變體前，先凍結資料包 ID、revision、原始位元組 SHA-256、證據模式、主張 ID 與中性命題摘要、各主張的必要／選用狀態、來源 ID 與 revision，以及資料包和原文的支持位置，並記錄 UTC 凍結時間。已知的首次產生時間必須晚於凍結時間。不得在看過草稿或分數後補造歷史未知值。

主張清單是事前計畫，不是判決。評審時，每項可核查主張沿用 W4 狀態：`supported`、`contradicted`、`insufficient`、`unavailable`。另以兩個獨立陣列記錄 `packet_scope_violations` 與 `packet_source_mismatches`；兩者可同時套用。這些是評估紀錄欄位，不是新的 Rust enum。

## 版本化檔案

| 檔案 | 用途 |
| --- | --- |
| `schemas/evidence-plan-v1.schema.json` | 單一證據模式、資料包／來源識別、凍結主張清單與產生順序。 |
| `schemas/adjudication-v1.schema.json` | W4 主張支持狀態，以及分開的資料包範圍與資料包／原文錯誤標註。 |
| `schemas/packet-errata-v1.schema.json` | 具版本、原文精確度與來源取得脈絡的勘誤片段。 |
| `fixtures/prospective-evidence-plan.json` | 合成計畫：一份來源段落可用，另一份來源無法取得。 |
| `fixtures/synthetic-adjudication.json` | 合成四種 W4 狀態與兩種分離標註軸。 |
| `fixtures/r1-packet-errata.json` | C13／C28 歷史更正，不修改 R1 輸入與結果。 |
| `fixtures/negative/*.json` | 配對非法範例：多重模式、凍結前產生、把來源不可用當矛盾，以及改寫 R1 分數。 |

這些 Schema 只供評估使用，不更動 Rust wire 契約或既有 CLI 命令。

## C13 與 C28 勘誤

C13 引用的運動部頁面於 2026-11-13 擷取。該次 HTTP 回應原始位元組的 SHA-256 為 `de297b15ff73640af20142d8223576c5320d0d08fbc4b1aeb2e61c0a372b99c7`；位元組僅暫存在工作環境，未存入儲存庫或持久封存。頁面支持正確姓名「鄭博隅」。公開紀錄只列更正短片段、引用、回應摘要與擷取中繼資料。R1 當時的來源 revision 與原始資料包仍未知。

C28 的公開 R1 稽核記錄指出應保留「at least 12 new state legislative chambers」的意思。2026-11-13T07:00:36Z 重新連線至引用的轉載來源時，HTTP 回應為 403；因此不宣稱該回應有來源雜湊或封存段落。更正措辭來自已發布的 R1 稽核，但原始 R1 資料包位元組無法取得。直接來源支持狀態保留為 `unavailable`；不得從 403 推論矛盾或捏造。

這兩項勘誤是下一版資料包的提案，不會回寫 R1 資料包、草稿、評分、分組、錯誤旗標或結論。歷史主張清單及產生前凍結時間未記錄，本工作不補造，也不把歷史勘誤包裝成新評估結果。

## 來源快照、雜湊與散布限制

未來採用 `packet_plus_archived_sources` 時：

1. 在解碼或換行轉換前擷取原始回應位元組。記錄引用 URL、UTC 擷取時間、HTTP 狀態、內容型別、來源 revision（出版版本、日期、DOI 或等值識別）及該原始位元組的 SHA-256。
2. 將精確快照保存在私人或其他獲授權的不可變封存中，記錄不含敏感資訊的封存識別與精確段落位置。公開紀錄不得包含私人路徑、簽名 URL 或憑證。
3. 在產生任一變體前凍結資料包位元組與主張清單，並以精確位元組雜湊綁定此次執行。
4. 若擷取失敗，記錄嘗試與狀態；依情況讓段落、雜湊或 revision 保持未知，並回報 `unavailable`。不得把拒絕存取頁面的雜湊冒充引用文章的雜湊。
5. 公開引用、簡短自撰摘要及辨識勘誤所需的最少文字。除非授權條款允許，勿發布完整受著作權保護的文章或掃描檔。雜湊只有在審查者可獲授權檢視相應位元組時才能核驗。

`packet_only` 不得把封存原文放入可用證據集合。資料包中的引用不會自動擴大評審模式。

## 驗證

```sh
python3 scripts/test_sk_e02_contract.py
python3 scripts/sk_e02_contract.py check
python3 scripts/check_repo.py
git diff --check
```

標準函式庫回歸測試檢查封閉欄位、單一模式、先凍結再產生、主張／來源／revision／位置參照、來源不可用處理、兩種標註軸分離、版本化 C13／C28 勘誤、合法與非法 fixture 位元組，以及禁止改寫 R1 結果。此工作環境另以 `jsonschema` 4.26.0 驗證三份 Draft 2020-12 Schema 及合法／非法結構範例；時間先後與跨紀錄 ID 需專用檢查器處理，因 JSON Schema 無法比對時間值或解析跨紀錄 ID。CI 不安裝這個選配套件，而執行獨立的標準函式庫契約測試。

請見[驗證紀錄](../../../validation/sk-e02-evidence-contract.md)，其中列出 base/head、RED/GREEN、測試與來源實際可取得狀態。
