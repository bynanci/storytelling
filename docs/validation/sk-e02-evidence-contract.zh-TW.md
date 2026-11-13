# SK-E02 驗證紀錄 · 證據計畫與 R1 資料包勘誤

繁體中文 · [English](sk-e02-evidence-contract.md) · [Issue #119](https://github.com/bynanci/storytelling/issues/119)

## 範圍與限制

基底：`d7b41310480379c946fae97f43c6560d3aa9077f`（2026-11-13 PR #124 合併至 `main`）。本變更凍結評估專用證據計畫，並記錄兩項 R1 資料包更正。不修改 Rust／CLI Schema、R1 輸入、輸出、分數、分組、錯誤旗標或結論；也沒有產生新的模型或真人評估結果。

R1 原始資料包、完整草稿、主張清單、來源快照與產生前凍結時間未保存。勘誤明確標示為歷史回報，不是修復後的 R1 紀錄。合成契約 fixture 不是評估資料。

## RED / GREEN

- **RED：** 先寫最初 14 項回歸測試，再執行 `python3 scripts/test_sk_e02_contract.py`，因 `ModuleNotFoundError: No module named 'sk_e02_contract'` 失敗（exit 1）。
- **GREEN：** 加入版本化紀錄契約、Schema、合法／非法 fixture 及標準函式庫檢查器後，擴充後的 20 項測試全部通過。`python3 scripts/sk_e02_contract.py check` 通過。

## 來源直接存取觀察

| 案例 | 觀察 | 保留內容 | 限制 |
| --- | --- | --- | --- |
| C13 | 2026-11-13T06:54:33.868047Z，引用的運動部頁面回傳 HTTP 200。檢視的賽果列使用姓名「鄭博隅」。該次暫存 HTTP 回應原始位元組 SHA-256：`de297b15ff73640af20142d8223576c5320d0d08fbc4b1aeb2e61c0a372b99c7`。 | 引用、短更正姓名、擷取時間、內容型別與原始回應位元組雜湊。 | 回應只暫存在工作環境，沒有保留於儲存庫或私人封存。歷史來源 revision 與 R1 原始資料包仍未知。雜湊不能取代可檢視的位元組封存。 |
| C28 | 2026-11-13T07:00:36Z 對所引轉載來源直接請求，回應 HTTP 403。 | 引用、失敗狀態與時間。沒有把錯誤頁雜湊冒充文章雜湊。 | 來源為 `unavailable`，不宣稱直接來源確認或矛盾。更正措辭回報自已發布的 R1 稽核。 |

沒有將完整受著作權保護的文章或私人來源路徑放入儲存庫。未來的原文封存評估必須將精確位元組保存在獲授權的封存中；公開內容以引用、雜湊、位置及最少自撰摘要為限，除非授權允許散布原文。

## #119 驗收對照

| 驗收項 | 工件與檢查 |
| --- | --- |
| 僅一種證據模式 | `evidence-plan-v1.schema.json`；`test_evidence_mode_is_one_frozen_string_not_a_combination`、`test_unsupported_evidence_mode_is_rejected`及已提交的`invalid-multiple-evidence-modes.json`。 |
| 產生變體前凍結主張／來源／revision／位置／必要性 | `prospective-evidence-plan.json`；`test_prospective_fixture_is_frozen_before_variant_generation`、`test_variant_generation_must_follow_claim_freeze`、`test_committed_invalid_freeze_example_is_rejected`、`test_claim_source_ids_and_revisions_must_resolve`。 |
| 沿用 W4 狀態，且範圍違規與資料包／原文差異分開 | `adjudication-v1.schema.json`；`test_w4_support_states_and_scope_annotations_are_separate`、`test_failed_source_access_cannot_be_relabelled_contradicted`。 |
| 版本化 C13／C28 勘誤並保留 R1 | `r1-packet-errata.json`；`test_c13_c28_are_versioned_without_rewriting_r1_outcomes`、`test_errata_cannot_include_replacement_scores_assignments_or_flags`、`test_exact_original_packet_text_cannot_be_invented`。 |
| 來源存取失敗維持 unavailable | C28 fixture、`test_failed_source_access_cannot_be_relabelled_contradicted`及`invalid-unavailable-as-contradicted.json`。 |
| 快照／雜湊來源及公開散布邊界 | SK-E02 README；C13 暫存回應中繼資料；C28 403 紀錄；Git 未納入來源全文。 |

檢查器只用標準函式庫，不依賴產品 runtime。此工作環境另以 `jsonschema` 4.26.0 執行 Draft 2020-12 meta-schema 與 fixture 驗證；hosted CI 執行新增的標準函式庫契約回歸。沒有進行獨立人員的來源封存稽核。

## 命令

```sh
python3 scripts/test_sk_e02_contract.py
python3 scripts/sk_e02_contract.py check
python3 scripts/check_repo.py
git diff --check
```

真人接受與任何新 R2 評估仍由維護者另行決定。SK-E03–SK-E05 保持為獨立任務。
