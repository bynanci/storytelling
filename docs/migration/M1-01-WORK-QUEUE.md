# M1-01 snapshot import

<a name="m1-01-工作與任務"></a>

[繁體中文](M1-01-WORK-QUEUE.zh-TW.md) · [Input contract](FINITE_V1.md) · [Comparison record](records/M1-01.md) · [Task index](m1-01-work-index.json)

M1-01 reads a composition export and produces a visual plan that M0 can check. It does not write to the source catalog, the user project or an editor.

<a name="實作階段核實基線歷史2026-11-07-utc"></a>

## Historical implementation baseline

The following observations refer to November 7, 2026 UTC.

| Item | Observed commit and state |
| --- | --- |
| Storytelling main | `551ff38920e2085e1d53a7ba777c9680678e9725`. The workspace did not yet exist. |
| M0, PR #1 | `4c5682c221b8d602ad7bbebe7a4c400c3e1ff95b`. Open draft. |
| Planning PR #2 and work base | `95291d31c47601181070bf89a28b1a182ddf6ed8`. Open draft based on PR #1. |
| Pinned upstream | `7add4762fe8174d76f43b863375ea39b73121e76`. |
| Observed upstream main | `5f737a6c8f35dce48e213370ff27900962c9304c`. Includes PR #126. |

The importer keeps the pinned comparison baseline. A regression rejects unsupported animation from the later main. Quality checks, three-platform CI and the required gate passed for the observed heads of #1 and #2. Their results cover those heads only.

<a name="已接受與合併的生命週期"></a>
<a name="工作分解"></a>

## Acceptance and merge history

The maintainer later authorized merging completed, verified PRs and marking their tasks complete.

| PR | Merge commit | UTC time on 2026-11-07 |
| --- | --- | --- |
| #1 M0 | `22279814ff6b147c386186569d7f77f3a803cf8b` | 02:17:16 |
| #2 planning | `9fb403be811643baf948acdb6d72232d4a6207bb` | 02:17:48 |
| #7 M1-01 | `610ed49673d6f554e539008b34f5fc5479b9dbc1` | 02:18:09 |

Issues #3 to #6 closed after acceptance of P1, P2 and P3. The acceptance covers the specified visual mapping only. OTIO export, source bytes, audio, editor execution, release and human review have separate evidence.

| Task | Accepted scope | Prerequisite or evidence |
| --- | --- | --- |
| P1 | Source revision, rights, producer and finite-v1 contract | GAP-01, GAP-02, GAP-03 and GAP-16. Independent design review, upstream comparison and per-file provenance. |
| P2 | Pure Rust mapper, exact clocks, immutable input and loss report | P1. Observed failure and success. Preserve each source occurrence and produce a valid M0 plan. |
| P3 | File or standard input and a JSON response with exit status | P2. CLI process regressions, empty tool path, debug and release tests, quality and CI. |
| M0-G01 and PORT-G01 | Foundation, planning and source policy | Maintainer authorization for #1 and #2. |

On November 7 the maintainer accepted the component policy. Original code uses MIT. Specified source-derived modules, tests and fixtures use Apache License 2.0. Record each component and retain applicable notices. Approval of this policy is separate from product acceptance. [Component manifest](provenance-m1-01.json) · [Sources and licenses](../../PROVENANCE.md)

<a name="本次邊界與後續"></a>

## Input, output and limits

Core and engine have no file, runtime, media or host dependencies. Core owns data and loss types. The engine maps the snapshot. The CLI reads explicit JSON and writes the result. The proposed OF-04 adapter path does not require a separate crate.

Successful results use `declared-visual-mapping-only` and `executable:false`. A video record carries `audio.not_mapped` and `blocks-host-handoff`. The importer rejects explicit audio, effects and animation without producing a plan. A still receives a synthetic hold window. Its original declarations remain available. The mapper treats 29.97 as an exact decimal. This differs deliberately from upstream NTSC inference. The command does not authenticate source bytes or a declared hash.

OF-03 covers immutable input, order and numeric identity in this slice. Cross-language canonical digests require further work. The six host-owned capabilities remain outside the Rust port. Later M1-02 and M1-03 deliveries have separate [OTIO](records/M1-02.md) and [editor test](records/M1-03.md) records.

To roll back this slice, disable its CLI command or revert its implementation. The source catalog and editor project remain unchanged.
