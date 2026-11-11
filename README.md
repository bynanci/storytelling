# Storytelling

[繁體中文](README.zh-TW.md) · [Try the workshop](#try-one-source-set-in-two-media) · [Install](#start-with-the-skill) · [Templates](skills/storytelling/references/multimodal-templates.md) · [Contributing](CONTRIBUTING.md) · [CI](https://github.com/bynanci/storytelling/actions/workflows/ci.yml)

Storytelling is an open-source [Agent Skill](skills/storytelling/SKILL.md) for turning source material into a story for a particular audience. Bring notes, images or recordings your agent can read, or work from a description or transcript. Ask for a spoken account, scene, illustration brief, audio script or another form you can use.

## Why Storytelling?

Two neighbors repair a gate. Open with “One held the gate while the other tightened the hinge,” and a newsletter reader sees the cooperation. Open with “This time the gate cleared the ground,” and a listener has a reason to stay for the next line. The event is the same. Choosing what the audience learns first changes how they understand it. Storytelling helps you draft both openings and decide which one serves your purpose.

For a quick anecdote, a paragraph may be all you need. When you adapt it into a spoken piece and two illustrations, more decisions carry over. If your notes change who tightened the screw, ask which lines and drawings need reviewing. If you approved an imaginary red umbrella for one drawing, it belongs to that version alone. You choose the interpretation and what goes into each draft. [Author exercise](https://github.com/bynanci/storytelling/blob/main/docs/narrative/N-01-AUTHOR-STUDY.zh-TW.md) · [Research and product decisions](https://github.com/bynanci/storytelling/blob/main/docs/WHY_STORYTELLING.md)

## Try one source set in two media

Try the fictional workshop with its [event notice and transcript](skills/storytelling/references/workshop-demo.md), [workbench diagram (SVG)](skills/storytelling/assets/umbrella-workshop/bench.svg) and [two-tap sound cue (WAV)](skills/storytelling/assets/umbrella-workshop/taps.wav). Open the files in a host that reads them, or paste the descriptions below.

<img src="skills/storytelling/assets/umbrella-workshop/bench.svg" width="360" alt="Workshop source diagram showing a folded umbrella on the left and a spool of thread on the right.">

```text
Make an invitation for residents visiting our umbrella workshop for the first time.
Notice: the session is on Saturday. Bring one umbrella.
Diagram: a folded umbrella on the left, a spool of thread on the right.
Audio: two synthesized taps, no speech.
Write a radio cue and two poster briefs, one Swiss-style grid and one cut-paper
collage. Show which details come from the sources and which are design choices.
```

| Deliverable | Example of a usable draft |
| --- | --- |
| Radio cue | **SOUND** Play the two taps, then leave a beat. **VOICE** “One umbrella. Bring yours to Saturday’s repair workshop.” The invitation comes from the notice. The taps are a production cue. |
| Poster A · Swiss-style grid | Keep the diagram’s umbrella and spool in their original order. Set “Saturday” and “Bring one umbrella” on the same left edge below them. Alignment directs the eye to the invitation. |
| Poster B · cut-paper collage | Cut a yellow paper arc and pull a loose thread across it toward the word “Saturday.” Label the open shape and yellow paper as choices for this illustration. |

A narrator can record the cue and an illustrator can sketch either brief. The Skill supplies scripts and directions. Finished images and audio require a capable host or a separate production tool. [Full mixed-media template](skills/storytelling/references/multimodal-templates.md#mixed)

## Choose a template and an approach

Each template includes a brief you can copy, two ways to treat the material and a check for the result.

| Situation | Two approaches to try | What you take away |
| --- | --- | --- |
| [Screenplay](skills/storytelling/references/multimodal-templates.md#screenplay) · A character returns a borrowed object | Social realism follows the exchange in real time. A flashback reveals why returning it matters. | A scene with playable action, dialogue and a reveal. |
| [Illustration](skills/storytelling/references/multimodal-templates.md#illustration) · Turn an event notice into a poster | Swiss-style typography gives the facts a reading order. Cut-paper collage uses shape to draw the eye. | A composition brief with subject, focal point, layout and text placement. |
| [Business copy](skills/storytelling/references/multimodal-templates.md#copy) · Invite someone to try a service | AIDA leads from attention to action. Problem–solution copy opens with the reader’s difficulty. | A headline, body, call to action and claims to verify. |
| [Brainstorm](skills/storytelling/references/multimodal-templates.md#brainstorm) · A team needs three event concepts | SCAMPER varies an existing idea. Reverse brainstorming asks how to make the experience worse, then reverses the answer. | Distinct options, a selection criterion and a small test. |
| [Everyday story](skills/storytelling/references/multimodal-templates.md#everyday) · Tell a friend what happened | Open inside the scene to build curiosity, or give the outcome first when the listener needs context. | A tellable account that keeps observed events separate from interpretation. |
| [Audio](skills/storytelling/references/multimodal-templates.md#audio) · Open a podcast with a field recording | A sound diary lets the listener hear the space first. A narrated essay opens with a point of view. | A script with voice, sound cues, pauses and source notes. |
| [Video](skills/storytelling/references/multimodal-templates.md#video) · Cut a short sequence from supplied clips | Continuity editing follows the action. Associative montage connects images by shape or idea. | A shot plan that maps beats to clips and identifies missing coverage. |

## Cross-media Rust support

Use the templates now with an agent or as Markdown. The Rust CLI validates structured video plans and proposes an order for supplied shot windows. For cross-media work, the project has accepted a [narrative document contract](https://github.com/bynanci/storytelling/blob/main/docs/narrative/NARRATIVE_DOCUMENT_V1.md). A local workflow for one spoken outline and two illustration briefs is the next implementation task. Native revision tracking follows it. [Oral and visual workflow](https://github.com/bynanci/storytelling/issues/50) · [Revision tracking](https://github.com/bynanci/storytelling/issues/51) · [Rust backlog](TASKS.md#multimodal-rust-backlog) · [Roadmap](ROADMAP.md)

## Start with the skill

### Install for an agent

From the project where you want to use Storytelling, run the following with Node.js 22.20 or later. Keep just the agent name you use, or omit `-a codex claude-code` to choose interactively.

```sh
npx skills add bynanci/storytelling --skill storytelling -a codex claude-code
npx skills list
```

### Install as an npm dependency

Use this route when you want your project’s npm lockfile to record the skill revision. In a project with a `package.json`, install the package from GitHub, then copy the bundled skill into the agent directories.

```sh
npm install -D github:bynanci/storytelling
npx skills add ./node_modules/@bynanci/storytelling/skills/storytelling --skill storytelling -a codex claude-code --copy -y
```

Commit `package.json` and `package-lock.json`. Rerun the second command after changing the dependency revision. The npm package contains the skill instructions, references and Python helpers. Build the optional Rust CLI separately using the source quick start below.

### Use the installed skill

The table gives the project locations and invocation syntax. For a manual install, copy the complete [skill folder](skills/storytelling), including its license, to the listed destination.

| Where you work | How to start |
| --- | --- |
| Codex in a project | Place the folder at `.agents/skills/storytelling/` in the project. Invoke `$storytelling`. |
| Claude Code in a local project | Place the same folder at `.claude/skills/storytelling/` in the project. Invoke `/storytelling`. |
| Another Agent Skills client | Use the client's documented skill loader. Discovery paths, invocation syntax and available tools belong to the client. |
| Plain Markdown | Open [SKILL.md](skills/storytelling/SKILL.md) and the references it links. A writer or team can follow the workflow by hand. |
| A script, CI job or application | Use the [standalone CLI](#quick-start) with structured input, JSON output and exit codes. |

For personal installs, other clients and verification status, see the [platform guide](https://github.com/bynanci/storytelling/blob/main/docs/PLATFORMS.md).

## Working with sources and revisions

- **Review a draft.** Compare the author’s notes with a draft, mark what was inferred or invented, and ask for revisions to the affected versions. [Worked examples](skills/storytelling/references/examples.md)
- **Coordinate complex work.** Use the graph-loop playbook for dependencies, independent branches, review and bounded retries. The skill includes optional Python helpers for planning a dependency graph and checking a recorded execution order. [Playbook](skills/storytelling/references/graph-loops.md)
- **Find supporting material.** When a question spans documents and a suitable Graphify index is available, locate candidate passages and check them against the originals. [Graphify guide](skills/storytelling/references/graphify-tools.md)

## Quick start

### Optional Rust CLI

The development version is `0.1.0-dev.0`, available from source with Rust 1.89.0. No AI subscription, model API key or video editor is required for these examples. With [rustup](https://rustup.rs/) installed, run these commands from your terminal.

```sh
git clone https://github.com/bynanci/storytelling.git
cd storytelling
cargo run --locked -p storytelling-cli -- validate examples/minimal-plan.json
cargo run --locked -p storytelling-cli -- plan-story tests/fixtures/planner-v1/valid.json
```

`validate` reports `ok: true` for one asset and two shots. `plan-story` prints JSON with `detail` before `opening`, preserving the author's explicit order even though `opening` has a higher score. To integrate your own application, start with these input shapes and the [CLI contracts](docs/CONTRACTS.md). [Planner input](tests/fixtures/planner-v1/valid.json) · [CLI guide](docs/CLI_GUIDE.md)

### Video integrations

<details>
<summary>Video commands, input limits and editor compatibility</summary>

| Input | Commands | Result |
| --- | --- | --- |
| A plan, brief or fixed-window planning request | `validate`, `audit-brief`, `plan-story` | Check declared references and constraints, or propose a shot order. |
| Local bytes and an expected SHA-256 digest | `verify-source` | Verify up to 16 MiB from an explicit file or stdin. |
| A supported OpenFilm composition snapshot | `import-openfilm`, `export-otio` | Convert a single contiguous picture track into a plan or OpenTimelineIO (OTIO). |
| A snapshot and its review context | `prepare-review`, `check-review` | Save a review record, then compare that record with current inputs. |
| A saved proposal, matching snapshot and review context | `prepare-story-review`, `export-story-otio`, `check-story-review` | Export selected cuts as OTIO. Compare a saved review bundle with the current inputs to detect changes. |

The snapshot adapter reads OpenFilm composition exports. It accepts stills and video cuts on the supported picture track. Video export requires an explicit picture-only choice. A FilmCraft v0.18.0 test on Linux at 25 fps covered save/reopen, OTIO reimport and a 45-frame CPU preview. A separate isolated test recovered from a lost import reply by reading the resulting project state. Production editor writes are planned. Audio handoff and full-resolution output await verification. [FilmCraft test record](docs/migration/records/M1-03.md) · [Recovery test](docs/validation/m3-03-filmcraft-execution.md) · [Review contract](docs/migration/PLANNER_OTIO_REVIEW_V1.md)

</details>

## Architecture and contributions

The portable skill lives in `skills/storytelling/`. The Rust workspace contains four crates. `storytelling-core` owns wire types and exact time, `storytelling-engine` owns in-memory planning and validation, `storytelling-cli` handles process I/O, and `storytelling-journal` provides a durable SQLite operation log. [Architecture](docs/ARCHITECTURE.md)

Contributions from other domains and agent clients are welcome. For a new workflow, include a small synthetic request, the expected output and how a contributor can check it. For a compatibility report, include the client, version, installation location and what you observed. Consult the [contribution guide](CONTRIBUTING.md), [open tasks](TASKS.md), [requirements](docs/REQUIREMENTS.md) and [development process](docs/SDLC.md). [Report an issue](https://github.com/bynanci/storytelling/issues/new/choose) · [Security policy](SECURITY.md)

## License

The portable skill includes its [MIT license](skills/storytelling/LICENSE). Original project code and documentation use the [MIT License](LICENSE). Identified OpenFilm-informed components use [Apache-2.0](LICENSES/Apache-2.0.txt) with a retained [upstream notice](NOTICE.openfilm). [Per-file provenance](PROVENANCE.md)
