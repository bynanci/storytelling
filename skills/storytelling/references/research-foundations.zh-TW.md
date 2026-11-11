# 研究基礎

[English](research-foundations.md) · 繁體中文

查核日期：2026-11-11。這是支援設計判斷的精選書目，並非系統性回顧。選文聚焦理解、作者控制、修訂、跨媒體溝通與評估。「應用」是 **Storytelling 的設計推論**，並非論文已為本專案證實的結果。沒有任何一篇證明本技能能改善所有語言或媒介的寫作。

搭配[規則](editorial-rules.zh-TW.md)與[工作流程](research-workflows.zh-TW.md)使用。只讀取當前任務需要的來源。

## 證據登錄

| ID 與出版資訊 | 實際查閱範圍與原始研究範圍 | 應用與界線 |
| --- | --- | --- |
| P01 · Zwaan、Langston、Graesser（1995），*The Construction of Situation Models in Narrative Comprehension: An Event-Indexing Model*，Psychological Science。[DOI](https://doi.org/10.1111/j.1467-9280.1995.tb00513.x) · [出版者摘要](https://www.psychologicalscience.org/journals/psychological-science/j.1467-9280.1995.tb00513.x/) | 摘要。以簡單文字敘事檢驗事件索引模型，透過時間、空間、主角、因果與意圖連結事件。 | 可用這些維度提問連貫性。不要求每個段落填滿五欄，也不替有意保留的模糊補造原因。 |
| P02 · Riedl、Young（2010），*Narrative Planning: Balancing Plot and Character*，JAIR 39，217–268。[論文](https://arxiv.org/abs/1401.3841) · [DOI](https://doi.org/10.1613/jair.2989) | 作者摘要與期刊紀錄。IPOCL 同時處理因果推進與角色目標，評估涉及角色意圖的理解。arXiv 於 2014 年存入；期刊論文出版於 2010 年。 | 分開角色目標、作者意圖與觀眾已知資訊。目標導向設定只在合適時使用。符號計畫無法證明情緒效果。 |
| P03 · Segel、Heer（2010），*Narrative Visualization: Telling Stories with Data*，IEEE TVCG。[作者頁面](https://idl.uw.edu/papers/narrative) · [DOI](https://doi.org/10.1109/TVCG.2010.179) | 作者摘要與敘事視覺化的設計空間說明，描述作者引導與讀者探索。 | 寫明第一注意焦點與可探索空間。移用至海報、聲音或影片須另做驗證。 |
| P04 · Lee、Liang、Yang（2022），*CoAuthor: Designing a Human-AI Collaborative Writing Dataset for Exploring Language Model Capabilities*，CHI。[論文](https://arxiv.org/abs/2201.06796) · [專案](https://coauthor.stanford.edu/) | 專案說明與摘要。資料集記錄 63 位作者、1,445 次英文寫作，含要求、採用、駁回與修改建議。 | 分開 AI 提議與作者決定。採用建議不等於滿意、身分驗證或品質提升。 |
| P05 · Masson、Zhao、Chevalier（2025），*Visual Story-Writing: Writing by Manipulating Visual Representations of Stories*，UIST。[作者論文](https://damienmasson.com/pdfs/visualstorywriting.pdf) · [DOI](https://doi.org/10.1145/3746059.3747758) | 摘要與第 5–7 節。兩次原型研究分別檢視規劃／審閱（12 人）與創作（8 人）；視覺表示協助定位與追蹤故事元素。研究一未確立顯著的認知負荷降低。 | 提供修訂影響圖，同時檢查實際改稿。不宣稱圖一定省力，或本專案交接已由該論文驗證。 |
| P06 · Yang、Klein、Peng、Tian（2023），*DOC: Improving Long Story Coherence With Detailed Outline Control*，ACL。[論文](https://aclanthology.org/2023.acl-long.190/) | 出版者摘要。針對自動產生的長篇故事，以詳細階層大綱與控制器對照特定基準。 | 長篇先用小型大綱，再核對段落與選定情節。不強迫短軼事承擔同樣成本，也不沿用論文效果量作為本技能成效。 |
| P07 · Min 等（2023），*FActScore: Fine-grained Atomic Evaluation of Factual Precision in Long Form Text Generation*，EMNLP。[論文](https://aclanthology.org/2023.emnlp-main.741/) | 出版者摘要。以參考知識來源，評估模型生成傳記中的原子事實支持。 | 將複合商業主張拆成可個別審閱的斷言。來源存在與支持主張分開。本專案人工主張表不是 FActScore 實作，也不是它的基準分數。 |
| P08 · Amershi 等（2019），*Guidelines for Human-AI Interaction*，CHI。[作者頁面](https://www.microsoft.com/en-us/research/publication/guidelines-for-human-ai-interaction/) · [DOI](https://doi.org/10.1145/3290605.3300233) | 作者摘要。18 項準則經多輪評估，其中包含 49 位實務工作者檢視 20 項 AI 產品。 | 顯示能力限制，支援修正與拒絕。這是互動設計依據，不是文學品質證據。 |
| P09 · Mayer、Moreno（2003），*Nine Ways to Reduce Cognitive Load in Multimedia Learning*，Educational Psychologist 38(1)，43–52。[出版者](https://www.tandfonline.com/doi/abs/10.1207/S15326985EP3801_6) · [DOI](https://doi.org/10.1207/S15326985EP3801_6) | 透過搜尋查閱出版者摘要；直接頁面回傳 403。討論多媒體教學中的有限處理容量與超載。 | 教學作品可檢查旁白、視覺與聲音是否爭奪注意力。論文不提供通用鏡長、配樂密度或禁用字幕規則。無障礙需求仍是明確限制。 |
| P10 · Zheng 等（2023），*Judging LLM-as-a-Judge with MT-Bench and Chatbot Arena*，NeurIPS Datasets and Benchmarks。[論文 v4](https://arxiv.org/abs/2306.05685v4) | 作者摘要。聊天助理評估發現位置、冗長與自我偏好，以及推理限制。 | 模型評論用來找候選缺陷；比較時交換 A/B 順序。模型回應不能算受眾參與者，也不能認證情緒效果。 |
| P11 · Mokhov、Mitchell、Peyton Jones（2018），*Build Systems à la Carte*，ICFP。[作者頁面](https://www.microsoft.com/en-us/research/publication/build-systems-la-carte/) · [DOI](https://doi.org/10.1145/3236774) | 作者與會議摘要、早期手稿導論。框架分開排程與是否重建的判斷。 | 分開依賴遍歷與重新驗證。宣告引用只能支援涵蓋範圍內的局部審閱；語意依賴仍需人工發現。這是工程類比，不是敘事研究。 |
| S01 · Moreau、Missier 編（2013），*PROV-DM: The PROV Data Model*。[W3C Recommendation](https://www.w3.org/TR/prov-dm/) | 摘要，以及實體／活動／代理者、衍生與修訂定義。**這是標準，不是研究論文。** | 分開來源、轉換與負責者。來源宣告不證明真實、權限或身分。本專案未宣稱 PROV 序列化或符合性。 |

## 這些來源不支持的推論

- 不要求每件作品都套三幕劇、英雄旅程、衝突、快樂結局或固定情緒曲線。這些可成為具名、可選的創作設定。
- 不從情緒分數推斷受眾感受，也不從欄位齊全推斷理解。
- 不混同事件順序、揭露順序與執行前置條件。只有最後一種在現有流程規劃器中必須是 DAG。
- 不因為缺少某媒介的 renderer 就推斷 Rust 無法支援。分開可表達契約、已實作命令與外部製作能力。
- 引用公開不代表可匯入論文全文、圖片、資料集、模型權重或參考程式。本套件僅含原創摘要與合成範例；未來匯入前須核對個別授權。

## 新增或修訂證據

記錄標題、作者、出版年、會議／期刊、原始網址或 DOI、查閱段落、存取限制、原始任務／族群、適用規則、反例與本地評估。區分理論、設計空間研究、實證結果、標準與工程類比。保留矛盾結果；較新不等於較適合。

研究能支援可測試的問題。只有專案實際觀察，才能確立此流程是否幫助目標使用者。
