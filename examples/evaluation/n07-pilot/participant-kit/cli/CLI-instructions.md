<!-- SPDX-License-Identifier: MIT -->
# CLI 條件操作卡

主持人提供固定版本的 storytelling 可執行檔。將 `<CLI>` 換成其絕對路徑，工作目錄為你的私人工作複本。不要看 facilitator/ 或評分參考。

1. base-document.json 是凍結原稿，勿改。working-document.json 是相同的工作副本；你要建立新的 NarrativeDocumentV1 文件快照，不是替作者填入審核或批次日誌。
2. 依修改卡更正 S01、S02 文字，將這兩個 unit revision 與 document_revision 改為 2。涉及的每個精確引用也要改為相應新 revision，包括 relations、based_on、beats.unit_ref 與 unit_refs。
3. I01 的 based_on 指向 S02，C01 的 based_on 指向 S01；更新這些 lineage 後，I01、C01 本身的 revision 也改為 2，引用它們之處一併更新。其文字、origin 與使用範圍不因 metadata 改動而變成新事實或已確認提案。S03、S04 留在 revision 1。這些版本編輯耗時要計入作者時間，不能當作工具自動完成。
4. 你仍須自己更正三份變體內的實際姓名／動作文字；只改來源不會替你改稿。把 render 的實際輸出保存為 narrative-final.txt，不必再把同稿抄回 shared/narrative-drafts.md。

```sh
<CLI> render-narrative cli/working-document.json > narrative-final.txt
```

5. 用同詞的 copy-template.json 查核商業文案。evidence-ledger.json 是原始來源的有效 N-04 日誌，只有未接受的助手提議；此處引用的 S03 沒有改變。它不冒充新的合併改名歷史。閱讀引用／歷史文字並與來源卡比對，修改自己的文案後保存實際報告。

```sh
<CLI> audit-copy-claims cli/copy-template.json cli/evidence-ledger.json > commercial-final.json
```

`current` 只表示引用 ID／revision 相符；missing、revised、dangling 是審查資訊。exit 0 不代表作者完成或可刊出。填 shared/answers.md，交付 working-document.json、narrative-final.txt、copy-template.json、commercial-final.json 與回答。

目前 N-04 日誌會在前一筆變更仍待 review 時拒絕第二筆 source_revision。這輪不要求你偽造 review 來繞過，採用明確版本化的前後文件快照並保留原稿。中間 metadata、錯誤恢復與重複的手動改字都是需要量測的成本。
