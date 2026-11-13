# SK-E01 · 離線產生紀錄與盲測資料包

Issue [#118](https://github.com/bynanci/storytelling/issues/118) · W6／ST-R12–ST-R13

本資料包保存每份草稿的實際產生資訊，並另行產生不含條件的評分資料包。工具只用 Python 標準函式庫，不呼叫模型或遠端服務，也不需要 Rust。內附「修理柵門」成對稿是人工編寫的合成測試資料，不是評估結果。

## 紀錄界線

`schemas/skill-evaluation-record-v1.schema.json` 定義私有來源紀錄。每個案例固定共同任務與受眾；每次產生都附精確的 UTF-8 提示、證據資料包及草稿位元組引用與 SHA-256。驗證器要求配對條件使用相同的提示與資料包雜湊。每次產生並記錄：

- case、run、generation 與中性文字 ID；
- 條件來源狀態 `observed`、`declared`、`unknown` 或 `synthetic`，並以獨立欄位記錄 `skill_on`、`skill_off` 或 `unknown`；
- 模型提供者、模型、版本、執行環境與設定；未知值明確填 `null`；
- 產生時間、草稿精確 SHA-256，以及已知的 Skill Git commit 與套件 SHA-256。

`observed` 表示同期紀錄支持該條件標記，不代表已驗證模型提供者或組織授權。`declared` 是尚未獨立查證的填報。缺少證據時，狀態與條件都填 `unknown`，不得由文風或既有分組反推條件。證據引用使用不含私人路徑或參與者姓名的代碼；模型設定不得保存憑證。合成紀錄必須標示為 fixture，不能混入實際觀察結果。

所有引用路徑都相對於資料包根目錄。驗證器直接雜湊檔案位元組，不轉換換行或 Unicode。私人提示、媒體、個人資訊及受著作權保護的原文，應留在獲授權的本機研究資料夾。公開 fixture 只含虛構文字。不要把絕對路徑、憑證或私人佐證放入公開紀錄。

載入 Skill 套件時，請記錄完整 Git commit 及 `hash-skill-tree` 的 `sha256`。V1 套件摘要的算法是：先雜湊領域前綴 `storytelling-skill-tree/v1\0`，再依排序後的相對 POSIX 路徑，依序加入 8 位元組大端序路徑長度、路徑 UTF-8 位元組、8 位元組大端序檔案長度與檔案原始位元組。拒絕符號連結。摘要描述本機載入的檔案；commit 記錄 repo 版本。

## 盲測資料包與分派金鑰

`prepare-blind` 會把兩個檔案寫到**不同資料夾**：

- `skill-evaluation-blind/v1` 包含共同任務、受眾、提示、證據資料包，以及以 `T-` 加六個中性 Crockford 字元表示的草稿 ID；展示順序會打亂。
- `skill-evaluation-allocation/v1` 是私有金鑰，將文字 ID 對應到 case/run/generation ID、條件及來源狀態、Skill 身分與展示位置。隨機種子只寫在金鑰；金鑰不含評分。

評分資料包不得帶出條件、Skill 身分、run/generation ID、雜湊、隨機種子或任何新舊分數。產生器以封閉欄位集檢查資料包，並核對私有紀錄中的 ID 與展示順序。JSON Schema 是版本化契約；標準函式庫工具執行操作檢查，不需額外 JSON Schema 套件。評分完成並凍結前，金鑰由主持人保管。評分者只拿到資料包，不應收到來源紀錄、金鑰、含私有路徑的終端輸出或舊評分。文字本身仍可能讓評分者猜測條件，工具無法消除這種推測。

分數只能存於評分後的獨立檔案，不得放進產生紀錄或評分資料包。`verify-blind` 會拒絕條件／分派和分數／評分欄位；`verify-allocation` 會偵測缺漏、多餘或錯配的 ID，並以種子重算展示順序。

## 手動離線流程

在 repo 根目錄驗證合成範例：

```sh
python scripts/skill_evaluation_records.py verify docs/research/evaluations/sk-e01/fixtures/synthetic-valid
python scripts/skill_evaluation_records.py hash-skill-tree skills/storytelling
```

進行經授權的 model-only 評估時，請先將 fixture 複製到私有資料夾，換成一組凍結任務與配對草稿，並將每份精確提示、資料包與草稿存成 UTF-8 本機檔案。不同條件使用互相隔離的新生成上下文。未知模型／執行欄位填 `null`；只有同期產生紀錄支持時才填 `observed`；skill-on 必須記錄 Skill commit 及套件摘要。

在私有資料包外建立兩個空資料夾並分開保存輸出。省略 `--seed` 時由工具產生隨機種子，只寫入私有金鑰。僅合成重播或刻意重現同一資料包時才指定 `--seed`。

```sh
mkdir -p /tmp/sk-e01-judge /tmp/sk-e01-private
python scripts/skill_evaluation_records.py verify /path/to/private-bundle
python scripts/skill_evaluation_records.py prepare-blind /path/to/private-bundle \
  --judge-out /tmp/sk-e01-judge/packet.json \
  --key-out /tmp/sk-e01-private/allocation.json
python scripts/skill_evaluation_records.py verify-blind /tmp/sk-e01-judge/packet.json
python scripts/skill_evaluation_records.py verify-allocation /path/to/private-bundle \
  /tmp/sk-e01-private/allocation.json
```

請將輸出放在有存取控管的位置。分享前先以評分者視角檢查資料包、凍結規準；評分時不提供金鑰，全部評分凍結後才揭露。實際模型呼叫、真人試用與發布需各自另行決定並保存證據。

## 保留 R1 原貌

[`r1-provenance-status.json`](r1-provenance-status.json) 將缺少的逐稿產生資訊明確記為 `unknown`。它不從文風重建條件，也不把已發布分組升格為觀察到的來源證據。原有 R1 的 40 案分數、旗標、宣告分組、來源稽核與結論仍保留於原檔，不作修改。R1 不可視為完整的 V1 產生紀錄。

## 檢查與限制

```sh
python scripts/test_skill_evaluation_records.py
python scripts/skill_evaluation_records.py verify docs/research/evaluations/sk-e01/fixtures/synthetic-valid
```

測試涵蓋精確位元組雜湊、重複／錯配 ID、不安全路徑、分派順序、條件／分數外洩與 R1 未知欄位。這些測試只證明紀錄工具行為，不證明條件真實、來源品質、實際盲測有效、真人效益或產品效益。本資料包沒有真人或模型觀察。
