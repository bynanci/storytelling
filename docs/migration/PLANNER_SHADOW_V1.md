# M2-03 selected-domain live planner shadow v1

G2 frozen after independent domain and execution-budget review. User requested continued tasks and completed-PR
organization under standing implementation/verified-merge authority. Engineering
choice: real paired source/native execution in a bounded shared domain, with explicit
negative/difference probes, rather than another replay of the existing12frozen
M2-02 outcomes or an unbounded full solver rewrite. No production API changes.

## Deliverable and authority

Test-only Python stdlib harness runs actual pinned OpenFilm compose through an
independently authored Node/tsx producer and the actual Rust plan-story executable.
It writes a fresh external evidence directory. It neither downloads dependencies
nor changes source checkouts/projects or opens media. Node24.22.1 and tsx4.23.15
reuse the existing hash-frozen test lock; production Rust remains tool-independent.

Invocation: python3 tests/oracles/planner-shadow-v1/run.py --source SOURCE_CHECKOUT
--node ABSOLUTE_NODE --tsx ABSOLUTE_LOADER --rust-cli ABSOLUTE_BINARY
--run-root ABSOLUTE_NEW_DIRECTORY [--seed U32] [--cases-per-domain N].
Default seed20261107, N24; N1..64. Default64cases:24required-fractional,
24optional-ranking,8targeted agreement probes,4explicit intentional deltas and
4unsupported multi-beat/media-balance probes. Maximum144cases. A deterministic
local generator records seed, generator hash, case order and complete input hashes.
Generated inputs are synthetic; no model/network/random clock supplies evidence.

## Shared domains and exact comparison

Required fractional: one beat, zero-origin video windows equal actual declared
source capacities in{1/4,1/2,3/4,1}s, complete distinct-ID manual/explicit chain covering EVERY eligible candidate
(no free optional candidate outside the chain), no source locks/effects/technical scoring metadata. Source min/preferred/max
clip lengths all equal capacity here. Hard maximum and targets use exact dyadics;
source target must not exceed its maximum. Required choices may exceed soft target.

Optional ranking: one beat, videos exactly1s, target=max=positive integer K,
no manual requirements/preferred IDs,
no rejects/locks; native max_shots>=catalog size. Native declared score equals the source rating integer0..5; all other scoring
facts are equal. Synthetic capturedAt timestamps are valid, unique and strictly
ascending in rating-descending/ASCII-ID rank so source ready
ordering matches native admission. Catalog permutations are exercised. This is a
domain precondition, not general score-factor/capture/locale parity.

Compare actual source/native selected asset IDs/order, exact source start/end,
timeline placements/window durations and total. Native rationals and source dyadic
JSON decimals become Fraction with zero tolerance. Normalize only random generated
source composition/track/clip IDs; retain original arrays/asset/story/beat facts.
Native plan IDs/shot IDs and false source/host/execution authority remain checked.
Validate each actual native nested plan through the actual validate command.

Targeted probes cover required-over-target success, explicit exclusions, missing
required IDs, include/exclude contradiction, order cycles, hard duration rejection,
shortfall and required inclusion. Error matching checks expected source
CompositionConstraintError category/message family and native exact code/conflict
kind; arbitrary pairs of errors are never treated as agreement.

## Difference classification

- agreement: both actual outcomes match the case's admitted shared domain, including
  exact success semantics or its explicit typed failure family.
- intentional-delta: four fixed probes for fixed-window non-shortening, explicit
  preferred-ID ranking, empty-plan refusal and target-above-hard-max policy. Each
  has literal expected semantic/error pairs on BOTH sides; a matching label alone
  cannot excuse other differences.
- unsupported: four fixed multi-beat/media-balance probes with source outcomes and
  exact native strict-input refusal. These do not establish native implementation
  of the probed feature.
- unclassified: any other difference, wrong order/window/clock/authority/error,
  missing/duplicate/reordered results or corrupted provenance. Exit1 and preserve
  failed evidence; never silently relabel it agreement or an accepted delta.

The immutable case records/disposition policy must validate their own declared
preconditions and expected pair. Wrong metadata/family cannot convert arbitrary
regressions into intentional/unsupported success. Tests mutate COMPLETE positive
baselines one field at a time and require the intended semantic failure, not KeyError.

A fractional optional target (for example1.5 with maximum2) is outside the shared
optional domain because source budgeting and fixed native windows may diverge.
The precondition validator must reject that relabeling before comparing results.

## Bounds and evidence

Explicit paths only, fresh nonexisting absolute run root outside source/target.
Source HEAD must equal7add4762fe8174d76f43b863375ea39b73121e76 with clean tracked and
untracked state before/after. Verify actual source files/blobs and target binary,
producer/generator/comparator/harness/lock/Node/tsx hashes and actual versions.
Target HEAD/tree/dirty state and all command exit/stdout/stderr hashes are observed,
not inferred from git metadata or a static expectedNative table.

Generated batch<=1MiB; source result<=4MiB; each native response<=4MiB; stderr<=65,536bytes; one tool
process<=30seconds and a180second global deadline across ALL child commands. No shell=True. Bounded capture
must stop an over-budget producer/child rather than buffering arbitrary output.
Terminate and wait for the owned process group on timeout/overflow, including
tsx/esbuild descendants. Source post-check runs on failure too. Runner scope is
Linuxx86_64; Rust remains supported/tested on all three existing CI platforms.
Positive source/native/validator stdout must have the documented exact shape and
empty stderr; infrastructure errors/timeouts/missing tools are failed execution,
not an expected domain rejection. Duplicate JSON properties reject.

Evidence schema_version1, profile storytelling.planner-shadow.v1, scope
selected-domain-live-planner-shadow, allPassed, source/target/tool identities (including resolved esbuild0.28.2 binary hash),
seed/case counts, command receipts, per-case normalized observations/classification,
summary counts, artifact path/hash/size manifest and explicit notClaimed list.
Report/disposition code1 for mismatch/error; successful run0; usage2. No evidence
from an unexecuted or failed stage may be emitted as passed. Raw synthetic inputs,
source output, native/validator stdout/stderr remain in the run root. Bulky artifacts
are not committed; the harness and compact recorded observations reproduce them.

## CI and scope boundaries

Existing167Rust tests and6/8/10/12frozen comparisons remain. Add shadow rejection/
mutation tests and actual live source/native comparison to Linux quality gate.
CI fetches exactly the public pinned source commit into RUNNER_TEMP, verifies it,
uses checksum-pinned official Node archive and copies the existing test package/lock
to a temp prefix. npm ci --ignore-scripts --no-audit --no-fund uses its integrity
lock; no upstream workspace install, global package or production dependency added.
Record and review the actual Node/test-tool closure and unchanged original MIT,
Apache notices and per-file source-informed inventory.

This accepts only selected-domain M2-03 shadow. Full multi-beat solver, coverage,
media balance, locks/pins/echo/graph/presets, all locale/scorer parity, live Catalog
integration/single-writer feature flags, media/source filesystem identity, host
execution/approval, human/editorial acceptance, release/admin/signatures stay open.
PR register separately distinguishes merged implementation/planning/docs from
closed-unmerged superseded and pending dependency changes.

## Frozen producer/report boundary

Producer accepts --source SOURCE_CHECKOUT and one stdin JSON object with exactly
schema_version:1 and cases:[{id,source_input:{story,assets}}]. It emits one JSON
line with exactly schema_version,source_head,batch_sha256,producer_sha256,
node_version,node_sha256,tsx_version,tsx_loader_sha256,tsx_package_sha256,
esbuild_version,esbuild_sha256,results. Actual metadata is independently checked
against preflight files/versions/lock. Child Node overrides NODE_OPTIONS and
ESBUILD_BINARY_PATH are cleared; TSX_TSCONFIG_PATH is explicitly the pinned source
config. Preserve inherited proxies/CA; no network is used by the harness itself.

Success row:{id,input_unchanged:true,ok:true,composition:<normalized full source>}.
Expected domain-error row:{id,input_unchanged:true,ok:false,error:{name,message}}.
Only actual compose CompositionConstraintError is serialized as a domain error;
producer process exit stays0. Any other thrown error/nonzero exit is execution
failure. Missing/duplicate/reordered rows and extra/missing fields reject.

Source-message/native-error table (CompositionConstraintError name required):

| Family | Anchored source message pattern | Native outcome |
| --- | --- | --- |
| missing-reference | Beat "TITLE" references missing asset ID. Relink the asset or remove the constraint. | planner.brief_conflict with unknown-asset |
| required-excluded | Beat "TITLE" both includes and excludes NAME. Remove one of these constraints. | planner.brief_conflict with required-excluded |
| order-cycle | Beat "TITLE" has conflicting asset orders or chronology. Remove or reconcile the ordering constraints. | planner.brief_conflict with order-cycle |
| required-maximum | Required assets and beat minima need MINs, above the story maximum of MAXs. Increase the maximum, reduce beat minima, or remove selections/locks. | planner.required_duration_exceeded |

TITLE/ID/NAME come from the exact validated synthetic case, MIN/MAX from its known
exact dyadic sums rendered with source toFixed(2). No broad substring/both-errors
fallback. Delta/unsupported probes have literal expected normalized outcomes.

Report top keys: schema_version,profile,scope,allPassed,source,target,tools,policy,
summary,commands,cases,artifacts,notClaimed,executionError. executionError is null
on success or a structured failed stage/code/message. Unexecuted identities remain
unknown/empty on failed preflight, never copied from expected values. Summary
records planned/executed cases and each observed classification count. Each case
has id,disposition,family,classification,passed,source,native,differences. Classification
becomes agreement/intentional-delta/unsupported only after complete checks; any
other observation is unclassified/failed. Artifact hashes cover raw actual bytes.
