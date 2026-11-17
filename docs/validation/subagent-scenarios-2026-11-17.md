<!-- SPDX-License-Identifier: MIT -->
# Subagent scenario validation · 2026-11-17

[繁體中文](subagent-scenarios-2026-11-17.zh-TW.md) · [Artifact manifest](subagent-scenarios-2026-11-17/artifact-manifest.json) · [Coverage overlay](subagent-scenarios-2026-11-17/coverage-overlay.json)

The requester explicitly asked for subagents representing people and events in simulation. Six separately prompted AI subagents ran author, handoff-editor, commercial-review, journal-audit, capture-audit and gate-review roles from base `91bf8f80139f59c0406340eb3565d296bd562931`. These are actual subagent outputs, not dialogue authored afterward by the root. They remain **synthetic**, with zero new human participants or Host observations. Separate contexts share a filesystem/model environment and are not enforced blind experiments or independent population samples. The session declares `gpt-6.1-sol`; serving revisions and sampling settings were unavailable, with no model override requested.

## Sources and actual role artifacts

| Run | Supplied material / observed scope | Preserved result |
| --- | --- | --- |
| SIM-AUTHOR-01 | Only the existing author sheet and two unsolved starter files; actual feature-enabled native CLI. | [Verbatim model response](subagent-scenarios-2026-11-17/responses/author.txt): four CLI calls, including range rejection/correction, and Q1–Q4 answers. It noticed the V1 path lacks the wrapper's `coverage` segment and the diagnostic omits the constraint explanation. It used Python instead of a text editor and had no continuous task timer; both deviations stay visible. |
| SIM-EDITOR-01 | A source-only packet derived from published synthetic kiln materials; separate R1/R2 cards, no facilitator key. | [Verbatim response](subagent-scenarios-2026-11-17/responses/editor.txt): six full returned drafts. It retained uncertainty about R2's “this version” referring to B and about C's unqualified missing-footage scope. These unexpected interpretations are not edited into a pass. |
| SIM-CLAIMS-01 | Public synthetic product/pilot fixtures; actual `audit-copy-claims --report-v2`, no expected-output key. | [Verbatim response](subagent-scenarios-2026-11-17/responses/claims.txt): ten calls, nine successful reports and one expected duplicate-reference rejection. Current references can still accompany stale or overbroad wording; support remains `human_review_required`. No product-contract discrepancy was found in that bounded probe. |
| AUDIT-JOURNAL-01 | Accepted journal source/tests, not a Host. | [Actual audit](subagent-scenarios-2026-11-17/responses/journal-audit.txt) proposed the old-readback replay defect. Root then reproduced `Ok(2)` in real SQLite tests; [PR #146](https://github.com/bynanci/storytelling/pull/146) fixes it with preserved RED/GREEN and independent fresh AI review. |
| AUDIT-CAPTURE-01 | Accepted rehearsal tool plus controlled native-output mutation. | [Actual audit](subagent-scenarios-2026-11-17/responses/capture-audit.txt) found a consistent wrong range still passed. The [independent oracle correction](n10-06a-replay-oracle-2026-11-17.md) adds exact arithmetic, typed comparisons and fault regressions; the native converter itself was correct. |
| AUDIT-GATES-01 | Accepted task/gate documents and test source anchors. | [Actual coverage review](subagent-scenarios-2026-11-17/responses/gates-audit.txt): eight domains ×six categories, 71 source references, remaining gaps and the exact three technical resume decisions. Its original pending-role flags are retained in the archive; the root overlay records later actual results without rewriting the audit. |

Real-world context is sourced, not invented. [VH-01-S3](https://github.com/bynanci/storytelling/issues/67#issuecomment-6815393001) is an older maintainer-attested self-session motivating JSON friction. [Published kiln material](../../examples/discovery/kiln-coverage.zh-TW.md) is explicitly synthetic, even though previous people read/revised it. The [M3-03 FilmCraft record](m3-03-filmcraft-execution.md) describes a real isolated software execution whose child exited before a reply was withheld; the new journal sequence is a synthetic extension, not a new observed FilmCraft incident. No private missing event, identity, quotation or author decision was reconstructed.

Raw packets, input revisions, model responses and tool traces are retained byte-for-byte in [artifacts.zip](subagent-scenarios-2026-11-17/artifacts.zip). Markdown/scripts in the archive have `.txt` appended for data-only storage; content is unchanged. The manifest maps all 143 originals and their SHA-256 values. Six plain-text previews expose the actual responses without extracting scripts. They are captured data, not commands to execute. Source/build hashes and process times in a trace do not authenticate a person's identity or measure author effort.

## Functional coverage and findings

| Domain | Newly observed or exercised | Explicit limitation |
| --- | --- | --- |
| Exact input A | Author-role file edits, seconds/frames, error correction; exact/mutant replay oracles. | No independent human, physical file, VFR timestamp or execution proof. |
| Narrative revisions | Named accepted Rust tests in full workspace runs; model-only revision hypotheses. | Text reenactment is not a native ledger run or broad semantic dependency proof; selected limits/process interruption remain unestablished. |
| Commercial claims | Ten real CLI probes: current, missing, dangling, revised, historical, case-sensitive, unknown revision and duplicate refs. | Citation identity is distinct from support, author adoption and publication authority. |
| Handoff | Six model draft artifacts plus named accepted Rust tests in workspace runs. | No image/audio readback; exact boundary and interruption gaps in the static matrix remain visible. |
| Planner / OTIO | Accepted named workspace tests and required hosted conformance gates. | No new user rough cut, editor execution or second-host qualification. |
| Review / approval | Accepted binding/signature tests in workspace/CI. | Mathematical signature checks are not current authority, independent PR #28 G2 or trusted execution. |
| SQLite journal | 17 focused tests including eight newer-outcome combinations; historic replay, restart, budget, operation/attempt/fence checks. | Caller evidence remains trusted; historical subtypes and real in-flight Host/quiescence are unproved. |
| Research capture | Independent arithmetic/type oracles, real native calls, failure retention, refusal to overwrite and discovery compatibility. | Fixed synthetic protocol, not a general verifier, human outcome or hostile-executable sandbox. |

The static matrix uses success/error/replay/identity/limit/interrupt categories. The root overlay matches 56 named source anchors to passing entries in source-bound workspace logs. This is traceability, **not a coverage percentage or proof of all possible scenarios**. Four local workspace runs passed 319 default or 338 all-feature Rust tests in debug/release, with three existing manual/benchmark tests ignored per run; source and raw logs are in [the journal record](m3-02-historical-readback/workspace-checks.json). Cases not exercised or lacking source evidence remain unexamined. Required final-head and post-merge CI are recorded on the corresponding PRs.

Independent review also corrected the oracle report's interpretation: of its first 18 failing regression subtests, 16 exposed false success and two reference cases were already rejected later by V1. Additional preview-only bool/float alias probes exposed Python equality coercion; JSON-type-sensitive comparison is required for the fixed references and all JSON parity checks. These are research-tool findings, not native conversion failures. Raw failures, initial outputs and corrected receipts remain separate.

## Tasks that simulation cannot complete

The next exact technical decisions remain #103's bounded W01 design/implementation authorization and eventual tested-contract G2, #107's eight-operation read-only facade scope/API/version/error decision, and #129 B's bridge/order/escaping candidate adoption. Agents do not sign or adopt these on a maintainer's behalf. #104–#106 still need ordered executor/recovery work and actual authorized Host evidence. #108–#113 retain their consumer, Host, real participant and G5 dependencies.

#121 still lacks authorized original drafts/packets/prompts/rubric/model records. These role outputs are not reconstructed R1 inputs. #122 still lacks its eligible pilot/revision prerequisites and was not started. #129's independent-author item, #112's five-user beta and the prior #54 disposition are not replaced by role simulation. No aggregate model “success rate”, human time saving, completion rate or causal benefit is computed. The real-human fields remain null; new human and Host counts remain zero.

## Verify retained bytes

From the repository root, this reads the archive without extraction or execution:

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib, json, zipfile
root = Path('docs/validation/subagent-scenarios-2026-11-17')
m = json.loads((root / 'artifact-manifest.json').read_bytes())
archive = root / m['artifact_archive']['path']
assert hashlib.sha256(archive.read_bytes()).hexdigest() == m['artifact_archive']['sha256']
with zipfile.ZipFile(archive) as z:
    for item in m['files']:
        data = z.read(item['stored_path'])
        assert len(data) == item['bytes']
        assert hashlib.sha256(data).hexdigest() == item['sha256']
for item in m['response_previews']:
    assert hashlib.sha256((root / item['path']).read_bytes()).hexdigest() == item['sha256']
print('143 captured artifacts and six verbatim responses verified; human participants: 0')
PY
```
