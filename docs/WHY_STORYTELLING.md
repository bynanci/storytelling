# Why Storytelling?

<a name="why-storytelling-為什麼需要這個專案"></a>
<a name="為什麼需要-storytelling"></a>

[繁體中文](WHY_STORYTELLING.zh-TW.md) · [Project guide](../README.md)

Two neighbors repair a gate. A community report can open with who held the gate and who adjusted the hinge. An illustration can show only their hands. A spoken account can open with the result: “This time the gate cleared the ground.”

Each version directs attention to a different detail. Suppose the notes name the wrong neighbor. The speaker must revise the account, and the illustrator must change any captions that name the neighbors. A drawing of the hinge alone can stay. Storytelling helps the author keep these choices and trace them to the source.

**Product hypothesis.** When authors revise several versions or share work, a record of their sources and choices may reduce missed or unnecessary changes. A single invitation with a new date may be easier to edit directly. The project compares the two approaches in an [author study](narrative/N-01-AUTHOR-STUDY.md).

<a name="作者應該拿到什麼"></a>

## What an author should receive

| Author's question | Useful output | How to check it |
| --- | --- | --- |
| Who needs to understand what? | Identify the audience, its background, and what it should understand afterward. The story can leave an open question or invite action. | Ask a reader to summarize the point. The author judges whether the summary reflects the intent. |
| Which events matter? | Identify the people, intentions, and consequences that the work needs. Distinguish event order from the order in which the audience learns what happened. Leave unknown causes open. | Move or remove a beat. Does the reader still find the necessary clue? A collage can preserve an association. A notice can start with the result. |
| What can I change while keeping the same material? | Compare two openings or two compositions with different focal points. Identify where each works. | Hide the style labels. Ask the author what they saw or heard first. The author can accept, combine, or decline the options. |
| Where did a detail come from? | Distinguish source material, the author's interpretation, and a creative proposal. Attribute quotations. Keep an invented prop in the version that calls for it. | Trace one fact and one creative detail. Check that the reference exists. Then decide whether the source supports the claim. |
| What needs review after a change? | List the affected drafts and the parts that can stay. Identify the result as a script, composition brief, or actual file. | Ask the recipient which draft to use. Open an image file, listen to an audio file, or read the result from the editor. |

<a name="六個使用情境"></a>

## Six situations to try

These examples describe outcomes a team can observe. Authors and readers must try them before the team can assess their value. [Eight authoring templates](../skills/storytelling/references/multimodal-templates.md)

| Situation | Materials and output | Observation |
| --- | --- | --- |
| Tell a family reunion story | Use memories and a photo for a 45-second spoken account. Start with an action. Paraphrase if you lack the original words. | A relative can distinguish the action in the photo from the teller's feelings. A date correction identifies a specific sentence to revise. |
| Explain a change to students | Use a process diagram and two observations from different times. Write an explanation and two cards, each with the same question. | A student can explain the change with the diagram and identify an inference. Each card supplies its own context. |
| Edit a documentary or podcast | Use an interview with source times. Compare an opening with the original voice and one with background information. Supply quotations, narration, and recording cues. | The editor can locate each quotation. If location sound is missing, label proposed effects. Use material within the permissions available. |
| Explain a product change | Draft an announcement and a spoken account from trial records and feedback. Distinguish what participants observed from promotional claims. | Trace “The trial went from ten minutes to eight.” Flag “Everyone works 20% faster” when the sources do not support it. |
| Develop two screenplay openings | Use two events, a character goal, and a dilemma. Follow event order in one opening. Delay the reveal in the other. Keep the remaining branches and their consequences. | A reader can identify when the clue appears. The author can revisit the other branches and distinguish fiction from the source. |
| Adapt one event for a poster and radio | Use a notice, a diagram, and two synthetic taps. The poster directs attention to the date. A pause separates the sound from the invitation. | Change Saturday to Sunday. Check each draft that names the date. The source drawing and sound should remain usable. [Workshop](../skills/storytelling/references/workshop-demo.md) |

<a name="研究如何影響設計"></a>

## How research informs the design

The [research design map](research/README.md) connects a broader evidence register to stable editorial rules, six skill workflows and existing requirements. It records access limits, counterexamples and evaluation priorities.

| Research and scope | Design inference |
| --- | --- |
| Zwaan, Langston, and Graesser (1995) studied short written narratives. They examined how readers connect events through time, space, characters, causes, and intentions. [Paper](https://doi.org/10.1111/j.1467-9280.1995.tb00513.x) | Ask readers to describe who acted and what changed. Authors can retain gaps, ambiguity, or an open ending. |
| Segel and Heer (2010) studied narrative visualizations. They describe trade-offs between a reading path the author sets and exploration the reader controls. [Paper and examples](https://idl.uw.edu/papers/narrative) | Choose an observable result for each medium, such as the first object a reader sees or the first clue a listener hears. Its use across media is a design hypothesis. |
| Lee, Liang, and Yang (2022) created CoAuthor. Their data cover 63 writers and 1,445 English writing sessions. Writers could request, accept, ignore, or edit model suggestions. [Study and data](https://coauthor.stanford.edu/) | Keep the author's intent, each suggestion, and the adopted wording separate. Check whether a rejected detail returns after a revision. Acceptance counts alone cannot establish satisfaction. |
| Masson, Zhao, and Chevalier (2025) ran two creative writing studies with prototypes. They examined how visual representations of story elements help authors revise text and explore variants. [Paper](https://damienmasson.com/pdfs/visualstorywriting.pdf) | Use a small case with known answers to identify missed or unnecessary edits. Then observe how authors actually plan revisions. |

These studies cover different people, tasks, and media. They help define tests. The effects of Storytelling on quality, effort, and writing in different languages still need direct evidence.

<a name="與現有工具怎麼搭配"></a>

## How it works with other tools

| Task | Tool | Storytelling's role |
| --- | --- | --- |
| Organize an interview or draft text | A chat tool or shared document. [Notion writing guide](https://www.notion.com/help/guides/notion-ai-for-docs) | Keep the source of each quotation and the wording the author selected across drafts. |
| Explore designs or create a layout | An illustration or layout tool. [Canva Visual Suite](https://www.canva.com/visual-suite/) | Write a composition brief that identifies the first focal point. Separate source details from choices the designer can change. |
| Edit, mix, grade, or master media | An editor or audio production tool. [DaVinci Resolve](https://www.blackmagicdesign.com/products/davinciresolve) | Organize material by story beat. Export a proposal when the input meets the documented timeline format. [Editor handoff test](migration/records/M1-03.md) |

<a name="哪些能力可以用哪些需要開發"></a>

## Current capabilities and proposed work

| Capability | Current scope | Evidence to collect |
| --- | --- | --- |
| Draft and compare treatments | The skill can draft from supplied material. Authors can follow eight templates manually. The video tools support specific planning, inspection, and timeline exchange tasks. | Use the same material in two distinct treatments. Trace each source detail and creative choice. |
| Use one source for speech and illustration | N-02 defines the narrative document. N-03 prints declared content with `render-narrative`. N-04 replays revisions and lists variants with direct source references. Author observations remain pending. | Compare output across repeated runs with the same input. [Contract decision](adr/0005-independent-narrative-document-v1.md) · [N-03 test record](validation/n03-narrative-cli.md) · [N-04 test record](validation/n04-narrative-revisions.md) |
| Support claims and other writing tasks | N-05 covers references for claims. N-06 compares screenplay, brainstorming, and video handoffs. N-07 uses author trials to decide whether to continue. | Identify missing or stale references. An editor judges whether the source supports the wording. Each proposed use needs a case that a template cannot handle. [Tasks](../TASKS.md#cross-media-narrative-backlog) |
| Add style rules, audio cues, and media handoffs | N-08 through N-12 cover style rules, cue sheets, a selected mode, a source manifest, and a host trial. The task list gives current status. | Check each task against its prerequisites. To deliver a file, use the required materials and host capability. Open the resulting file. [Multimodal tasks](../TASKS.md#multimodal-rust-backlog) |

A host is the application that loads the skill or performs a media operation. The author decides what to say and whether to publish it. The skill helps draft the material. The core defines data types. The engine checks declared references, revisions, and order. The CLI handles local input and output.

Source links in the V1 format record the author's declarations. Revision history, factual support, permissions, and emotional impact each need their own evidence. [Architecture](ARCHITECTURE.md) · [Narrative document format](narrative/NARRATIVE_DOCUMENT_V1.md)

<a name="怎麼知道值得繼續做"></a>

## How to decide whether to continue

The N-01 study calls for five authors. Each author completes the same timed revision task once with ordinary notes and once with the skill. Record completions, total attempts, revision time in seconds, errors in attribution, and repeated input. Evaluate the Rust tool in a separate trial. The study has no recorded human observations yet. [Study protocol and decision thresholds](narrative/N-01-AUTHOR-STUDY.md)

- **N-04 revisions.** List the expected drafts first. Compare the result with that list. Record missed and unnecessary changes by ID. Keep a record of suggestions the author declines.
- **N-05 claims.** Remove a source. Check that statements using it need review and other supported statements remain. The editor checks the claim against the source.
- **N-06 and N-07 handoffs.** Ask readers what each version communicates first. Ask the author to explain the choice. Ask the recipient to identify missing material. Record human observations separately from software results. Narrow the scope if ordinary notes suffice or the extra classification causes errors and rework.

[Project guide](../README.md) · [Roadmap](../ROADMAP.md) · [Requirements and tests](REQUIREMENTS.md)
