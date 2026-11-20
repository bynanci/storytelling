<!-- SPDX-License-Identifier: MIT -->
# M4-03 · Read-only reference application

[繁體中文](README.zh-TW.md) · [Acceptance scenarios](acceptance.md) · [Issue #109](https://github.com/bynanci/storytelling/issues/109)

**G0–G1 proposal.** Source baseline `04d2ac6f5cdb43d16cc7e4c94a2ad14fad49974d`. Issue #107 is accepted. The live inventory at the baseline had one open Draft, #151 for the #108 TypeScript transport, and no #109 PR. This packet supplies the first author task and acceptance cases for the read-only slice of M4-R03. The client choice and handling of local inputs move to G2.

## First author task

An author preparing a rough cut receives a coverage handoff with one missing shot and an older review of a separate synthetic proposal. They need to report the missing unit and its reason, identify the input changed since the review and tell an editor what they can safely use. In the current workflow those answers span coverage JSON, revision impact output and review results. A reference surface should make each answer findable without requiring the author to read raw JSON or infer the relationship between two IDs.

Use two labeled fixture sets. The coverage fixture is an example of a blocked handoff. The plan and review fixture set shows a successful review check followed by a target revision change. Coverage and narrative identifiers have no accepted automatic binding to the separate planner request in these fixtures. The interface must identify their inputs separately. A missing coverage row stays visible when a planner example is loaded, and a planner result never fills that gap.

The author passes this task by naming the unresolved unit and reason, pointing to the changed field and retaining the same proposal, and correctly explaining why the current and stale review views both offer inspection only. Record completion for each answer, where the author looked, mistakes, assistance and verbatim unexpected interpretations. Success in an observed session would establish that one author navigated this scenario. Additional independent sessions would be needed for a broader usability claim.

## Candidate client boundary

| Route | What it can prove | Cost or dependency |
| --- | --- | --- |
| Rust local reference surface using `storytelling_engine::consumer_v1` | Direct use of the accepted SDK and a review experience in one process. An accessible text presentation is sufficient for the first fixture. | Owns presentation, explicit input selection and bounded reads. Requires an app-specific G2 review. |
| TypeScript application | Can demonstrate a web-style interaction and reuse portable views if adopted. | Requires the #108 transport decision and accepted implementation. |
| Existing SDK example or CLI | Provides a fixture baseline and a way to confirm expected states. | The example covers one operation. Separate CLI output does not demonstrate the reference task or screen state transitions. |

Advance the Rust local reference surface to G2 review. At G2, choose the actual UI and executable boundary, freeze how files are selected and read, define review invalidation and handling of late results, and identify the test entry point. A facility to upload, persist or export input bytes would need its own product case. The existing planner response, OTIO and review bundle are sufficient artifacts for this slice. No new public schema is required by G1.

## Requirements for the proposed slice

| ID | Observable requirement |
| --- | --- |
| M4-R03.1 | Display declared narrative and revision identities and direct impact results from the public SDK, with the role of each selected input visible. Coverage distinguishes valid unresolved items from rejected input and identifies the first applicable diagnostic. |
| M4-R03.2 | Display plan and review identity for the selected inputs. Prepare from the exact retained proposal response bytes and inspect the opaque compiler result. Changing any review input immediately invalidates the old display. Only a check against the current inputs may show `current` or `stale`. A failed check displays its diagnostic and leaves no current marker. |
| M4-R03.3 | Show scope beside each status. Declared coverage, computed review freshness, authority, Host observation and recovery each have their own labels. With the read-only client the execution action is unavailable throughout. The reason remains visible and usable with a keyboard and screen reader. |
| M4-R03.4 | Preserve input bytes and use only explicitly selected local inputs. Use the existing SDK limits and diagnostic order. Display bounded identifiers and stable diagnostic codes and paths, treating them as text. Keep local file locations, raw buffers and diagnostics containing private values out of default logs and public receipts. |
| M4-R03.5 | Reproduce the public fixture walkthrough from the app’s future normal entry. Keep the automated output and anonymous, consented author observations in distinct records. |

For review bytes, follow the SDK rule to serialize a freshly generated planner response once and keep it. Reuse a previously selected proposal response byte for byte. The target revision experiment checks the original bundle against a changed context. Preparing a replacement first would answer a different question. Use the SDK to determine freshness. No UI digest comparison can substitute for that result.

If the author makes another selection while a check is pending, bind results to the selected input set. A late result for the earlier selection may not set freshness for the current selection. Proposals and artifacts in a history view need labels for their originating inputs. There is no durable approval or writer state in this slice.

## Acceptance and dependencies

The [acceptance scenarios](acceptance.md) define proposed future app checks and pre-existing SDK evidence. Review them at G1 before choosing an interface. A scripted read-only walkthrough and an author reading can then be evaluated on the chosen build. #109 stays open for the full project flow. Its one-writer, execution, persisted receipt, recovery and lost-reply criteria depend on #103–#106 and the accepted host workflow. #111 and #112 retain their app and host acceptance dependencies.

A maintainer can adopt this slice for G2, narrow it to coverage or review if both panes confuse authors, or defer it if there is no author task beyond the existing CLI. Removing this packet before implementation requires no data migration. The packet records prospective acceptance only.
