<!-- SPDX-License-Identifier: MIT -->
# N-R07 · Prospective native round v3

English · [主持人操作版（繁體中文）](n07-native-v3.zh-TW.md)

Protocol ID: **n07-native-notes-v3-proposed**, revision 1. Status: **prepared, not adopted or started; 待真人證據**. Documentation base: `4c78bc128f478265402392eeec7a1cfae1a69db6`. Preparation does not reopen [#54](https://github.com/bynanci/storytelling/issues/54), reverse its external-workflow Stop, or recruit its previous authors again. The [historical result](n07-pilot.md), #48 Skill result and #65 readings remain separate datasets. G5 publication is separate.

## What this changes

The previous submissions could not be bound to the agreed native executable. This prospective protocol adds a clean, pinned build receipt, raw invocation evidence and explicit pair eligibility before analysis. It reuses the matched door-repair task and scoring rubric without relabelling the earlier submissions as native observations. A source hash or JSON shape alone cannot authenticate who performed a session.

Before recruitment, the maintainer records the protocol's Git commit and SHA-256, the frozen material and executable identities, acceptance of the rules below, date/timezone and a new round ID in a **new tracking issue or private agreement**. Copy the existing header-only [forms](../../examples/evaluation/n07-pilot/forms/agreement.csv); use this protocol ID, not v2. Actual agreement, facilitator identity, recruitment and product decision are currently **unrecorded**. A protocol or scoring change after recruitment creates another version and cohort; it cannot repair this round retrospectively.

## Frozen inputs and command preflight

| Identity | Fixed value |
| --- | --- |
| Native source | `ce73836caa86502933f538821fdbf5dac92c1224` |
| Report | ReportV1; native `audit-copy-claims COPY LEDGER`, no report flag |
| Material / helper delivery | `8e2c08094f6bce70a7507e11ed84265bd98e3820` |
| Starting document | `n07-door-repair@1`; S01–S04/I01/C01; C01 only in illustration A |
| Conditions | A = ordinary notes; B = directly invoked native CLI, no model or external transforming wrapper |

Run the following Bash recipe from a repository checkout with those commits available, Rust 1.89.0 and Python 3. Use a fresh private directory. On other systems use equivalent absolute paths and record the actual environment; these shell commands are not a claim of Windows trial execution. The material checkout includes scoring keys: it is facilitator-only.

```sh
set -eu
N07_RUN="$(mktemp -d "${TMPDIR:-/tmp}/storytelling-native-v3.XXXXXX")"
export N07_RUN
git worktree add --detach "$N07_RUN/source" ce73836caa86502933f538821fdbf5dac92c1224
git worktree add --detach "$N07_RUN/materials" 8e2c08094f6bce70a7507e11ed84265bd98e3820
test -z "$(git -C "$N07_RUN/source" status --porcelain)"
test -z "$(git -C "$N07_RUN/materials" status --porcelain)"
rustc --version > "$N07_RUN/rustc.txt"
cargo --version > "$N07_RUN/cargo.txt"
CARGO_TARGET_DIR="$N07_RUN/target" cargo build --release --locked \
  --manifest-path "$N07_RUN/source/Cargo.toml" -p storytelling-cli \
  > "$N07_RUN/build.stdout" 2> "$N07_RUN/build.stderr"
python3 "$N07_RUN/materials/examples/evaluation/n07-pilot/run_cli.py" \
  --cli "$N07_RUN/target/release/storytelling" \
  --manifest "$N07_RUN/materials/examples/evaluation/n07-pilot/cases.json" \
  --out-dir "$N07_RUN/engineering" --runs 5
python3 - <<'PY'
import hashlib, json, os, subprocess
from pathlib import Path
r = Path(os.environ['N07_RUN'])
def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()
def git(folder, *args):
    return subprocess.check_output(['git', '-C', str(r / folder), *args], text=True).strip()
assert git('source', 'rev-parse', 'HEAD') == 'ce73836caa86502933f538821fdbf5dac92c1224'
assert git('materials', 'rev-parse', 'HEAD') == '8e2c08094f6bce70a7507e11ed84265bd98e3820'
assert not git('source', 'status', '--porcelain')
assert not git('materials', 'status', '--porcelain')
report = json.loads((r / 'engineering/results.json').read_text())
assert report['machine_success'] and report['all_inputs_unchanged']
binary = digest(r / 'target/release/storytelling')
assert binary == report['cli']['sha256_before'] == report['cli']['sha256_after']
raw = r / 'engineering/commercial-current-and-missing.stdout'
envelope = json.loads(raw.read_text())
assert set(envelope) == {'ok', 'commercial_copy'} and envelope['ok'] is True
assert envelope['commercial_copy']['schema_version'] == 1
claim = next(c for c in envelope['commercial_copy']['claims'] if c['id'] == 'unsupported-program-claim')
assert claim['reference_status'] == 'missing' and claim['evidence_refs'] == []
receipt = {'protocol': 'n07-native-notes-v3-proposed', 'human_evidence': False,
           'source_commit': git('source', 'rev-parse', 'HEAD'),
           'materials_commit': git('materials', 'rev-parse', 'HEAD'),
           'binary_sha256': binary, 'lock_sha256': digest(r / 'source/Cargo.lock'),
           'raw_report_sha256': digest(raw),
           'engineering_results_sha256': digest(r / 'engineering/results.json'),
           'rustc': (r / 'rustc.txt').read_text().strip(),
           'cargo': (r / 'cargo.txt').read_text().strip()}
with (r / 'build-receipt.json').open('x') as f:
    json.dump(receipt, f, indent=2)
    f.write('\n')
print(r / 'build-receipt.json')
PY
```

Retain the recipe, build logs, source checkout and receipt privately. Inspect toolchain/configuration overrides before building; record any deviations. The helper deliberately reports `source_binding_verified: false`: it hashes an executable but cannot prove its build. The separate build record supplies a facilitator-observed chain, not independent binary attestation. A matching version string alone is insufficient. Failed preflight blocks recruitment; preserve its output and create **N07-V3-PROVENANCE** with synthetic reproduction if reproducible.

## Enrollment and allocation

Recruit five **fresh** authors who have not seen the task, corrections, key, #48/#54 materials or previous answers. Record screening and CLI/JSON experience before assignment. Use anonymous codes only. Keep identity/contact details outside the dataset.

Create a new private allocation once, before the first start. Do not reuse the v2 draw or redraw for a preferred order. For example:

```sh
python3 - <<'PY'
import json, os, random, secrets, sys
from pathlib import Path
seed = secrets.token_hex(16)
orders = ['AB', 'AB', 'AB', 'BA', 'BA']
random.Random(seed).shuffle(orders)
record = {'protocol': 'n07-native-notes-v3-proposed', 'seed_string': seed,
          'method': 'random.Random(seed_string).shuffle',
          'python': sys.version, 'planned_slots': orders}
with (Path(os.environ['N07_RUN']) / 'allocation.json').open('x') as f:
    json.dump(record, f, indent=2)
    f.write('\n')
PY
```

Run this allocation command only once in the fresh directory; retain the original file before assigning authors. Planned slots are not human observations. In a private enrollment log record `slot, anonymous_code, screening_date_timezone, prior_exposure, cli_json_experience, assigned_order, actual_order, started_A, started_B, withdrawn, withdrawal_time_reason, replacement_for, deviation`. Unknown stays blank. Do not expose later allocations to authors. Actual order and deviations must be retained even if they invalidate comparability.

Pre-agreed withdrawal rule: preserve every start and withdrawal in the all-started denominator. A withdrawal with missing primary observations is not a comparable pair and is not zero-imputed. A fresh replacement inherits the unused slot's order; do not reuse the withdrawn author's completed arm. Report replacements separately. Stop recruitment after five comparable pairs or five replacements, whichever comes first; if fewer than five comparable pairs, report 待真人證據 and obtain a new prospective decision before any extension.

## Conduct each pair

1. Give both arms fresh copies of the **same** `participant-kit/shared` starting drafts from the pinned material delivery. Give B `participant-kit/cli` too. Keep corrected fixtures, engineering output and facilitator keys private. Provide the same five-minute introduction and unrelated practice to both arms; log setup separately.
2. A edits source cards, one oral outline, two illustration briefs, commercial draft and answers. B manually edits its working snapshot and draft text, then uses `render-narrative` and `audit-copy-claims` exactly as the [CLI card](../../examples/evaluation/n07-pilot/participant-kit/cli/CLI-instructions.md) describes. Preserve base bytes. C01 stays A-only; updating lineage never makes an imagined umbrella an observation or an assistant suggestion an author approval.
3. Start a continuous author timer after the change card has been read, stop on saved submission or 480 seconds. Include thought, JSON metadata edits, errors, recovery, CLI waiting and saving. At 480 seconds save the partial work, mark incomplete/timeout and use 480 seconds in analysis. Preserve actual elapsed time too. Never substitute process milliseconds for author time.
4. Invoke the exact absolute executable from the receipt. Retain each actual argv, start/end, exit code, raw stdout/stderr, input copies/hashes and executable hash before/after in private call records. Use unique output names for each attempt. The [existing timing example](../../examples/evaluation/n07-pilot/README.md#separate-timing-for-an-actual-participant-cli-invocation) measures the direct child process without transforming stdout. Its engineering helper is not an author-session recorder. Do not pipe through jq or rename another envelope to ReportV1. Verify raw artifacts before any presentation/export transformation; retain derivatives separately.
5. Save actual drafts and all answers before showing the scoring key, including incorrect or unexpected answers. Complete [sessions](../../examples/evaluation/n07-pilot/forms/sessions.csv), [errors](../../examples/evaluation/n07-pilot/forms/errors.csv), [feedback](../../examples/evaluation/n07-pilot/forms/feedback.csv) and [process calls](../../examples/evaluation/n07-pilot/forms/cli-calls.csv) in private copies. Log repeated manual entries and rework as they occur; command retries are not repeated factual entries. Preserve verbatim feedback apart from facilitator interpretation. Do not give scoring feedback between arms.

## Eligibility, scoring and decision

A comparable pair needs fresh-author screening, assigned/actual order, the agreed protocol/material IDs, verified native build/call receipts, both starts, timer and timeout status, submitted or preserved partial artifacts, primary scores, attribution event/major counts and repeated-entry counts. A complete trial record may contain failed tasks or timeouts; **do not select only successful authors**. Unknown measurements, missing arms, changed materials, wrappers or unresolved provenance do not qualify. Keep them in the all-started/deviation report. A shape check is not proof that a participant ran the binary.

Score the unchanged [rubric](../../examples/evaluation/n07-pilot/facilitator/key.md): correct S01/S02 and all three drafts; preserve S03/S04, actions and C01's scope; correctly distinguish C01, I01 and unsupported U01 within 480 seconds. Commercial subcounts are diagnostic: two factual claims and two proposals, not four participants. Preserve offending clauses, source/artifact IDs, error categories and rater. Fabricated C01 or U01 support is major. Count the same underlying judgment once per author/condition when copied to a log and a claim; preserve both occurrences. Seek a second rater for disputed scores; unresolved scores stay unknown.

With five comparable pairs and complete primary measures, apply in this order:

- **Stop:** B completes fewer than 3/5; or at least two B authors have major errors; or no observed improvement in completion/quality, time, repetition or attribution.
- **Continue:** B completes at least 4/5, no B major errors, B's total attribution events and number of authors with major errors are no higher than A, and at least one median improvement: 20% less author time, one fewer repeated entry, or one fewer attribution error per author.
- **Narrow:** neither condition above. A positive time gain below 20% alone is not “no improvement.”

For the no-improvement rule, compare completion counts (higher is better), total attribution events/major-author counts (lower), and arm medians of capped author time, per-author repeated entries and per-author attribution errors (lower). Report all dimensions even if they disagree; no significance or population-effect claim follows from five pairs. If A's time median is zero/unknown, a percentage gain is undefined. Rework, feedback and secondary commercial scores remain separate descriptive measures; do not change the threshold after seeing them.

Report screened/enrolled, starts by arm, withdrawals, replacements, comparable pairs, completed/started and completed/comparable, times including timeouts, attribution events and major-author counts, repeated entries, rework and feedback. Report process timing separately. Also report AB/BA order effects and the same-task learning limitation. Missing required evidence means **待真人證據**, not pass or zero errors. Publish only consented anonymous summaries; keep raw records private.

Record the maintainer's **actual** Continue/Narrow/Stop choice, date, evidence and any deviation from the calculated recommendation in a separate field. All those fields remain blank now. No release candidate is recommended by this preparation.

## Named follow-up routes and verification

| Route | Trigger and next evidence |
| --- | --- |
| N07-V3-PROVENANCE | Wrong executable, transformed/missing raw output or mismatched input; quarantine the pair, retain receipts, make a synthetic reproducer before calling it a native defect. |
| N07-V3-ATTRIBUTION | Native output is read as factual support despite `missing`/reference-only status; preserve the clause and independent rubric scoring, reproduce with synthetic material, decide UI versus contract work at G0/G1. |
| N07-UX01 | Multi-unit revision hits the already accepted pending-review guard; use the existing [design task](../../examples/evaluation/n07-pilot/facilitator/N07-UX01.md), not fabricated approvals. |

These are route names, not newly observed failures or created GitHub issues. Create a named issue with expected/actual behavior and RED/GREEN only when a reproducible failure exists. This documentation changes no CLI/schema behavior; no new RED/GREEN is claimed. Existing packet/helper tests and repository checks verify preparation; build preflight verifies engineering only. PR CI and review record the exact documentation head. N-R07 human acceptance remains unverified until an adopted round supplies the required records.

## Preparation replay evidence

The documented clean-build recipe ran on Linux with Rust 1.89.0: [build receipt](n07-native-v3/build-receipt.json), [all 11 engineering cases](n07-native-v3/engineering-results.json), and [unmodified ReportV1 stdout](n07-native-v3/commercial-current-and-missing.stdout). Each case had one excluded warmup and five measured runs; all expected exits and replay checks passed, inputs and executable hashes stayed unchanged. The unsupported program claim remains `missing` with empty evidence. These process times describe this machine only, not author performance. The helper correctly leaves source binding unverified; the build receipt records this automated local build, not a participant session or independent attestation.

A separate synthetic execution of the allocation recipe produced exactly three AB and two BA slots and rejected an overwrite without changing the first file. That dry-run draw was discarded from recruitment; no authors were assigned. Self-review checked the timer/withdrawal rules, immutable old packet, unchanged reader-sheet digest, and separation of proposed agreement from actual decisions. No independent human review or new observation is claimed.
