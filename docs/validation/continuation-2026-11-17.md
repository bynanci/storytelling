<!-- SPDX-License-Identifier: MIT -->
# Verified continuation and remaining gates · 2026-11-17

**Historical snapshot:** this record predates #149 acceptance and captured SK-E06 exercises. [Subsequent acceptance/issue convergence](sdk-acceptance-2026-11-17.md) is the current status source. Original evidence/pending text is retained rather than rewritten with hindsight.


This is the current status receipt for the productization map in [Issue #114](https://github.com/bynanci/storytelling/issues/114). It is based on main `f77ccad6dc40affdd6f7b2541ae93ba88095665d`. The repository showed zero open PRs and sixteen open issues when checked. This update records delivered engineering scope and the evidence still needed to resume blocked tasks; it does not accept a contract, attest a maintainer decision, close a product milestone or authorize publication.

## New verified deliveries

| PR | Submitted head → merge | Result |
| --- | --- | --- |
| [#142](https://github.com/bynanci/storytelling/pull/142) | `eac63b8a5adaac5208436ff1622d3d66e1f4bb6d` → `a219201f46bc78595225deea79795bd64f5ef671` | Added an append-only exact-arithmetic sensitivity report over the existing R1 scores. All five derived artifacts reproduced byte-for-byte; original values, declared groups, flags and conclusions stayed unchanged. Ten new regressions and 24 existing SK-E04 tests passed. The report treats 40 purposive cases as units and enumerated group assignments as sensitivity scenarios, not new samples. It cannot establish causal Skill effectiveness. |
| [#143](https://github.com/bynanci/storytelling/pull/143) | `9d238b8c2cea51205f333546753582b2c81b248a` → `f77ccad6dc40affdd6f7b2541ae93ba88095665d` | Indexed bound source references in read-only handoff rendering. Three CLI characterization tests preserve order, revision binding, variant scope and diagnostics. Frozen synthetic release workloads with 32/63 artifacts measured median ratios 0.5474/0.5480 against the predeclared ≤0.75 target; all 176 process runs returned identical output bytes. This is one-machine process evidence, not author-time or host acceptance. |

The final-head CI for each PR passed six ordinary jobs plus three SK-E06 platform jobs. Post-merge CI passed the same nine jobs for both merges. See the linked PRs and their exact run receipts: [#142 final](https://github.com/bynanci/storytelling/actions/runs/43692841559), [#142 post-merge](https://github.com/bynanci/storytelling/actions/runs/43693164744), [#143 final](https://github.com/bynanci/storytelling/actions/runs/43693354711), [#143 post-merge](https://github.com/bynanci/storytelling/actions/runs/43693659339). Their dedicated matrix runs are linked from each PR. The merged tree matches the verified #143 head and the original R1 package's 15 checksums still match Git blobs and checkout bytes.

## Remaining issues and resume conditions

| Open issue(s) | Current blocker | Resume condition |
| --- | --- | --- |
| [#103](https://github.com/bynanci/storytelling/issues/103) | The execution contract and roundtable recommendation are proposals; final technical G2 acceptance and W01 authorization/implementation are not recorded. | Maintainer accepts the exact contract scope; implement deterministic no-host tests; maintainer records a decision on the exact tested commit. |
| [#104–#106](https://github.com/bynanci/storytelling/issues/104) | Executor depends on #103; recovery depends on the executor; host acceptance needs actual supported-profile execution and readback. | Complete in dependency order with durable operation identity, single-writer fencing, unknown-outcome recovery, actual isolated FilmCraft readback and the maintainer's accept/narrow/stop decision. No production project mutation is implied by this status document. |
| [#107](https://github.com/bynanci/storytelling/issues/107) | ADR 0013's eight-operation read-only facade remains a G2 proposal; public API freeze and implementation are pending. | Maintainer accepts the exact API/version/error scope; implement the facade and demonstrate CLI/SDK parity. Mutation APIs remain gated by M3 acceptance. |
| [#108–#110](https://github.com/bynanci/storytelling/issues/108) | TypeScript consumer, reference app and optional MCP depend on the public facade; mutation also depends on M3. | Use the accepted public surface, keep read-only behavior isolated, expose approval/stale/unknown states, and preserve the single-writer boundary. |
| [#111](https://github.com/bynanci/storytelling/issues/111) | No second-editor import/playback/save/reopen evidence. | Select one bounded host/profile and obtain an authorized real-host test with exact losses and readback. |
| [#112](https://github.com/bynanci/storytelling/issues/112) | The prospective five-user protocol exists, but #109, #111, accepted M3, recruitment and frozen trial materials are incomplete. | Accept the protocol before recruiting; freeze build, assets, tasks and scoring; collect actual independent sessions. Earlier author cases do not count as beta participants. |
| [#113](https://github.com/bynanci/storytelling/issues/113) | Distribution and release-candidate evidence depends on accepted consumer, host and beta results. | Assemble actual compatibility, migration, rollback, dependency and study receipts; the maintainer separately decides G5 publication. |
| [#114](https://github.com/bynanci/storytelling/issues/114) | Parent map remains open while its child gates are open. | Update this map after child decisions and verified evidence; do not mark productization complete from preparation PRs. |
| [#115](https://github.com/bynanci/storytelling/issues/115) | SK-E06 packet integrity is tested and #142 adds arithmetic diagnostics; observed manual/host execution and the separate #121/#122 evidence gates remain absent. | Run authorized cases with exact host/skill/runtime and verbatim output, or keep result fields unknown. Do not turn reference answers or synthetic rehearsals into human evidence. |
| [#121](https://github.com/bynanci/storytelling/issues/121) | The exact original drafts, source packets, prompts/rubric, model settings and verbatim judge records needed for an order-only R1 rerun are unavailable. | Supply an authorized private source bundle and eligible frozen environment. Do not reconstruct missing study inputs. |
| [#122](https://github.com/bynanci/storytelling/issues/122) | It depends on eligible #121 data and a justified instruction revision; neither is available. | Complete those prerequisites, then freeze a prospective analysis plan before constructing or inspecting a holdout. |
| [#129](https://github.com/bynanci/storytelling/issues/129) | A is implemented but independent author use is missing; B/C retain separate G2/privacy and implementation gates; D is a checklist. | Record an independent author's actual use; obtain the separate B/C decisions and implement only accepted scope; keep media and host findings in their own evidence lanes. |

These are not interchangeable gates. A merge or green CI proves the submitted code/document checks only. It does not provide current authorization, human identity, participant outcomes, actual media content, editor behavior or publication approval. The roundtable choices in PR #141 remain technical recommendations; they are not recorded as formal maintainer G2 acceptance.

## Current handoff

There are no open PRs. The sixteen open issues are #103–#115, #121, #122 and #129. The most direct next engineering work is blocked until the maintainer records whether to accept the exact #103 W01, #107 read-only SDK and #129 B scopes. Research work is blocked by unavailable original study material or actual participants. Host-dependent work needs authorized real-host observations. G5 remains a separate maintainer decision.

No release, tag, repository-policy change, real user-project mutation, new human result or Host acceptance was performed in this continuation.
