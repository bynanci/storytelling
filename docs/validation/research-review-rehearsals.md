# Review follow-up: synthetic workflow rehearsals

English · [繁體中文](research-review-rehearsals.zh-TW.md) · [Delivery record](research-rules-skills.md)

Date: 2026-11-11. These are **synthetic agent outputs**, not reader/editor participant observations, measured improvement or maintainer acceptance. Three fresh GPT-6.1 Sol agents received task material without expected answers or the review discussion. This record preserves inputs and observed extracts; it is not a full tool transcript. Original output excerpts remain in the requested Traditional Chinese.

## Input and instruction scope

The agents used `skills/storytelling/SKILL.md`, `references/research-workflows.md` and `references/editorial-rules.md` after the review changes. Source-case files came from incorporated main `c1f164354c3e88d90acb32075828954091c149d8` ([PR #84](https://github.com/bynanci/storytelling/pull/84), source head `ec4a879ccbf2ea1b04288d2f0fbec09b94f8dcce`). They could read only the named source README and A/B drafts, not keys, calibration, boundary tables, previous validation or the parent conversation. They did not edit files, contact anyone or use media production tools. W3/W6 used only the supplied material below. Review against the keys happened afterward in the facilitating agent's pass.

- Agent A: produce an opening and audit N-01 claims (RV1); deliver a three-beat branching-game R2 scene, player/Ivo knowledge table and staging handoff (RV2).
- Agent B: independently apply R1 to each named case, starting from the originals; return complete A/B/C copies including unchanged drafts and a short source handoff (RV3–RV6).
- Agent C: produce an audio-description cue sheet/status (RV7); give an evaluation plan, classify a constructed response and handle a legacy record (RV8).

The follow-up makes W5's negative-inversion guard explicit. After Agent B's run, its wording was clarified to distinguish a claim's `contradicted` judgment from a source reporting a negative result; no source facts or expected outputs changed. W4 already contained that definition. This final wording received direct editorial review, not a new independent rehearsal.

## Coverage and observed results

| Case | Inputs / task | Observed behavior | Scope |
| --- | --- | --- | --- |
| RV1 · W1/W4 | N-01 cards and four claims, detailed below | All four review labels used with distinct evidence reasons; newsletter omits unsupported program claims and illustration-only umbrella. | Text/source audit, no empirical factuality score. |
| RV2 · W2/W5 | [branching-game](../../examples/studies/branching-game/README.zh-TW.md), original A/B and R2 | Full-frame card remains before the first bell; proposes showing dock context first within the same beat. Ivo learns the note at the second bell. | Written staging proposal, no implemented game or player result. |
| RV3 · W5/W4 | [ux-interview](../../examples/studies/ux-interview/README.zh-TW.md), R1 | All three quotations corrected; adjacent button interpretation narrows; operation sequence and review field retained. | Returned draft comparison, not measured error reduction. |
| RV4 · W5/W4 | [account-review](../../examples/studies/account-review/README.zh-TW.md), R1 | A/B use 9,200 and 8%; C remains unchanged; no migration-causality claim. | Arithmetic and scope check. |
| RV5 · W5/W4 | [open-source-release](../../examples/studies/open-source-release/README.zh-TW.md), R1 | A/B distinguish Linux import from Windows build; do not claim Windows import failure. C retains two seconds. | Evidence-scope check. |
| RV6 · W5 | [nonprofit-report](../../examples/studies/nonprofit-report/README.zh-TW.md), R1 | Personal quotation and experiential paraphrases removed from A/B; independent attendance data, B invitation and C retained. | Current-consent boundary, not a legal determination. |
| RV7 · W3 | Supplied stage notes/gaps/pause, below | Cue sheet preserves dialogue gap and audience-left orientation; timing and production remain unverified. | Script only, no audio file. |
| RV8 · W6 | No human observations; constructed account response and legacy row, below | Reports separate reading axes, three handoff labels in two distinct comparisons, unsupported causal assertion and legacy `unassessed`. | Protocol and synthetic calibration only. |

The facilitator inspected the returned copies against original sources and cards, and checked RV2/RV3–RV6 against the relevant key/boundary rows afterward. No participant counts, completion times, damaged-rate differences or statistical effect sizes were generated. Eight selected scenarios cannot establish generalized reliability. No pre-change agent baseline was rerun; these observations are not a before/after error-rate comparison.

## RV1 · N-01 input and observed extracts

Task material: S01, a toolbox by the shared wooden gate on Saturday with Mei and An present; S02, Mei holds the gate and An tightens an upper-hinge screw; S03, they close the gate once and it clears the ground; S04, tools return to the box. No timing, cost, sample or comparison records. I01 is the author's mutual-help interpretation. C01 is an imaginary red umbrella for illustration A only.

Request: write a short factual newsletter opening, then audit these four propositions using workflow labels and evidence: Mei held the gate; Mei tightened the screw; one closing proves durable repair; the program cuts costs by 30%.

Observed opening:

> 週六上午，共用木門旁擺著一只工具箱。Mei 扶著門，An 鎖緊上方鉸鏈的一顆螺絲。兩人關了一次門，這回門底沒有碰到地面。接著，他們把工具收回工具箱。

| Proposition | Returned label | Returned evidence reason |
| --- | --- | --- |
| Mei held the gate | `supported` | S02 explicitly records it. |
| Mei tightened the screw | `contradicted` | S02 attributes tightening to An. |
| One successful closing establishes durability | `insufficient` | S03 contains only one test; no durability observation. |
| Program cuts costs by 30% | `unavailable` | No cost, comparison or program records supplied; the percentage/denominator cannot be checked. |

The agent kept I01 as interpretation and C01 outside factual news. A missing cost record was not invented.

## RV2 · Branching scene input and observed extracts

Request: read only the case README and original A/B; apply R2 (replace the inset with a full-frame black card; retain note text and timing); return three beats, player/Ivo knowledge and staging handoff.

Observed scene:

> 1.〔先呈現碼頭環境，隨即切至全幅黑底字卡；第一聲鐘響前顯示。〕「我借了牌子，好送爸爸過河。我會還的。」——瑪拉
>
> 2. 第一聲鐘響。一點銅光沒入瑪拉袖中。伊沃伸手，只摸到空鉤。他喊渡口管理員。
>
> 3. 第二聲鐘響。伊沃攤開紙條，拇指停在「爸爸」兩個字上。

Observed knowledge/handoff: the player sees the reason first; the source does not establish that Ivo saw Mara take the token. Ivo reads the reason and promise only at the second bell; a promise does not establish return. Showing the dock and then the card in beat 1 is identified as a staging proposal, not an added world event. Timing feasibility and the produced scene remain untested.

## RV3–RV6 · Returned W5 copies

Each request was “apply this case's R1 independently to the original A/B/C; return actual revised and unchanged copies plus a concise source handoff.” The following are the returned draft texts, with formatting normalized; they are generated rehearsal artifacts, not replacements for the study's original reader packets.

### RV3 · UX interview

A:

> 「我以為我已經傳出去了。」這是 U4 對當時傳送狀態的理解。這場操作中，U4 開啟預覽、按下儲存，然後關閉分頁。紀錄裡沒有分享操作。

B:

> 開啟預覽，按下儲存，關閉分頁。U4 這場操作的紀錄裡沒有分享動作。談起當時的理解，U4 說：「我以為我已經傳出去了。」

C:

> 給案例編輯：確認過的原句是「我以為我已經傳出去了。」請只歸給 U4。若要在研究團隊以外使用，請先核對該受眾的引述授權。
>
> 想追問的問題：＿＿＿＿＿＿

Handoff excerpt: A's button interpretation narrows to the then-perceived transmission state; S2 and P1 remain. One person's account cannot establish that the button caused the misunderstanding.

### RV4 · Account review

A:

> A 服務後四週用了 9,200 分鐘，比前四週的 10,000 分鐘少了 8%。上次您問，兩處搬遷的據點是否都在帳單裡。我們能先核對帳單涵蓋哪些據點，再一起解讀這段變化嗎？

B:

> 兩處據點搬遷之後，是不是都還列在這張帳單上？這是您上次留下的問題。兩個四週的用量從 10,000 變為 9,200 分鐘，降幅是 8%。我們可以把搬遷日期標在用量圖上嗎？

C (unchanged):

> 給帳務分析人員：請找出客戶提到的兩處搬遷據點各對應哪筆計費紀錄。若有據點未列入，記下是哪一處。解讀紀錄前，再向客戶確認搬遷日期。

Handoff excerpt: `(10,000−9,200)÷10,000＝8%`; C, S2 and B's date-marking proposal retained. Dates, billing coverage and cause remain unresolved.

### RV5 · Release demo

A:

> 按下 M，從範例資料夾的對照圖點選一個項目，旁邊就會顯示原始路徑。想跟著操作的話，請留意維護者更正的驗證資訊：Linux 已實際操作並通過範例匯入測試；Windows 的 CI 只完成編譯。

B:

> 維護者更正驗證資訊：Linux 已實際操作並通過範例匯入測試；Windows 的 CI 只完成編譯。匯入以後可以試這一步：按 M 開啟範例資料夾的對照圖，點一個項目，看它來自哪條路徑。

C (unchanged):

> 講師提示卡：按 M 打開範例資料夾的對照圖，點選一個項目。游標在原始路徑停兩秒，再繼續往下講。

Handoff excerpt: Windows import has no supplied result, so failure and platform incompatibility cannot be asserted. Other platforms have no verification material. No video produced.

### RV6 · Nonprofit report

A:

> 兩次活動共留下 18 筆簽到；一個人來兩次，就會有兩筆紀錄。

B:

> 兩次活動共有 18 人次簽到，每來一次算一次。你想問工作小組什麼？歡迎寄一個問題給我們。

C (unchanged):

> 給志工帶隊人：出席摘要請寫「兩次活動共 18 人次簽到」。日後若要回報實際參加人數或回訪比率，需另外記錄不重複人數，讓同一人的多次出席可以辨認。

Handoff excerpt: S2 consent is withdrawn, so the quotation and identifying experiential paraphrase leave A/B. Independent attendance, B's invitation and C remain. Eighteen attendances do not establish unique people, return rate or confidence change. R2 was not applied.

## RV7 · Audio description input and observed extracts

Supplied input: S1, chair on audience-left, glove on seat, empty palm turns toward audience at 00:08; no performer intention. S2, gaps 00:00–00:06 and 00:08–00:14 with dialogue between. P1 proposes a half-second first-opening pause. No timed reading or recording exists. Request: a cue sheet and delivery/verification status.

| Returned cue | Text/action | Status |
| --- | --- | --- |
| 00:00–00:06 | 「觀眾左側，一把椅子。」〔擬停頓 0.5 秒〕「椅面上，一隻手套。」 | Proposed script/pause; fit unverified. |
| 00:06–00:08 | No added description; preserve dialogue. | Declared gap respected. |
| 00:08–00:14 | After the action, 「一隻空著的手掌，轉向觀眾。」 | No invented motivation or early reveal. |

Observed status: no recorded file, audio generation, listening or synchronized performance inspection. The agent required timed reading before claiming that the script plus pause fits six seconds.

## RV8 · Evaluation input and observed extracts

Task: no human observations; request a first-focus/inference and handoff-damage plan with arms, materials, fields and denominators. Constructed training response: “I first noticed the drop: 8,800 and 12%. Moving the two sites definitely left one off the bill, causing the entire decline.” Sources: 10,000 then 8,800 minutes; customer mentioned two site moves and asked whether both remain billed; coverage, dates and causes unverified. Also handle a legacy `rating=match` row with no inference field.

Observed classification: `inference_status=unsupported` for asserted omitted billing and complete causality. The numerical decline is supported. `rating=match` is conditional on a frozen key naming the decline; because no key was supplied, the agent did not claim an unconditional focus result. A legacy row keeps `rating=match` and gains `inference_status=unassessed`, not `grounded` or `none`.

Observed plan: counterbalanced neutral reader packets; notes–mapped and a separate mapped–mapped-bounds comparison; only the latter adds source-limit rows. No moderator answers for recipients. Primary H2 measure is damaged/received drafts. Report focus matches/first exposures, inference distribution and unassessed counts, plus each handoff error's own denominator and preparation time. Constructed calibration is excluded from empirical numerators/denominators. Human result fields stay blank; no audience or editor effect is claimed.
