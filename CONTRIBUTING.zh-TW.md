# 貢獻指南

[English](CONTRIBUTING.md) · 繁體中文

1. 閱讀[任務清單](TASKS.zh-TW.md)、[開發流程](docs/SDLC.zh-TW.md)、[需求](docs/REQUIREMENTS.zh-TW.md)及[開發代理規則](AGENTS.zh-TW.md)。
2. 選定一項工作。檢查進行中的 PR。記錄起始提交。
3. 在獨立分支上開發。透過拉取請求（PR）提交功能變更。

<a id="contributing-a-skill-example-or-host-guide"></a>

## 提交 Skill 範例或平台指南

範例可以取自任何專案。請附上作者的需求、提供的素材及預期產物。

平台指南請寫明用戶端名稱與適用範圍。範圍可為單一專案、個人帳號或代管帳號。
註明你實際驗證的項目，例如找到套件或成功叫用功能。
若僅查閱廠商的安裝文件，請註明依據。
連到對應的[平台指南](docs/PLATFORMS.zh-TW.md)。

請編輯 [skills/storytelling](skills/storytelling) 內的原始檔。
儲存庫供 Codex 探索 Skill 的路徑是相對符號連結。
工具與參考文件均使用相對連結。複製整個目錄後，請檢查連結。
新增素材類型時，更新 [package.json](package.json) 的 npm 打包清單。
檢查實際封裝的檔案。

## 撰寫繁體中文文件

預設文件使用英文。繁體中文版使用 `.zh-TW.md` 檔名。兩種語言的文件互相連結。

先寫讀者要完成的工作，再列出輸入與預期結果。操作步驟每句寫一個動作。同一個概念使用相同術語。[中文技術寫作規則](https://github.com/lemonhall/asd-ste100-skill-zh/blob/01345c50d240ab5f5ee34c95702adb6078a70f1b/SKILL.md)

一般範例使用作者提供的筆記、文字或素材。說明特定整合時，列出實測產品、版本與限制。指令與路徑照實列出。授權與來源紀錄保留原名。創作示例可保留作者的語氣。

<a id="checking-the-project"></a>

## 執行檢查

安裝專案指定版本的 Rust 工具鏈。執行下列指令。

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo doc --workspace --no-deps --locked
python3 scripts/check_repo.py
python3 scripts/test_skill_distribution.py
```

儲存庫的中繼資料及連結檢查需要 Python 3.11 以上版本。
獨立使用 SDK 或 CLI 時免裝 Python。
請避免意外更動 `Cargo.lock`。更新相依套件或工具鏈時，另開小型 PR。
說明安全、授權與相容性影響。

## 提交拉取請求

- 每項行為變更須有回歸測試與執行證據。請使用合成測試資料。
- 請勿上傳私人來源檔、本機路徑、GPS 資料、機密或使用者專案。
- 列出尚未執行的測試。分開回報解析器測試與實際宿主驗收。
- 在 PR 範本填入任務及需求 ID。記錄先失敗再通過的測試（RED／GREEN）。附上同一提交的 CI、風險及遷移說明。
- 發布套件或建立標籤前，須取得擁有者明確授權。變更分支規則也需明確授權。程式檢查通過後仍須另行核准發布。
- 遵守現有 MIT 授權與逐檔來源規範。請勿代他人簽署開發者來源聲明（DCO）。目前專案未強制執行 DCO。若採用 DCO，須先建立政策與有效的貢獻者簽署流程。
- 揭露 AI 協作。自行審閱時，請標明為自行審閱。
