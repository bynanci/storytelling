<!-- SPDX-License-Identifier: MIT -->
# SK-E04 · Strict input and evidence identity regression

English · [繁體中文](sk-e04-input-boundary.zh-TW.md) · [#121](https://github.com/bynanci/storytelling/issues/121) · [Protocol](../research/evaluations/sk-e04/README.md)

Base: `c562ce0d06b7330813fa42cad739f24f4a49e9da`. This focused fix preserves the accepted v1/v2 record fields and report calculations. It changes no Rust/CLI schema, Skill rule, R1 draft, score, assignment, error flag or research conclusion. It executes synthetic tool inputs only; no model, participant, real media or editor is involved.

## Reproductions and result

| Boundary | Before | After / regression |
| --- | --- | --- |
| Duplicate JSON members, including nested outcomes/preferences | Last member silently replaced an earlier value; contradictory outcome input could produce a successful report. | Reject before analysis, exit 2, no report. The diagnostic does not echo untrusted member/control text. |
| Invalid discriminant types | Arrays/objects in `schema_version` or `record_kind` escaped as `TypeError`, CLI exit 1 with traceback. | API raises `RecordError`; CLI exit 2 without traceback or report. |
| Invalid UTF-8 | Decoder exception escaped as a traceback and exit 1. | Existing input-error path, exit 2, no report. |
| Non-finite constants / decoder numeric failure | Typed validation rejected non-finite fields later; the standard decoder's existing integer-conversion failure escaped. | Explicitly reject non-finite constants and bound the existing decoder failure; no new numeric, size or depth policy. |
| One v2 opaque output reference paired with different SHA-256 values | Contradictory metadata accepted within or across cases. | Reject record-wide; same reference with the same digest remains accepted. |

Artifact checks establish declaration consistency only. They do not retrieve or authenticate evidence bytes. Judge-context scope is unchanged. The existing matched-pair flip count/rate remains pair-weighted; per-draft score changes and order-gap shifts retain within-case averaging followed by across-case averaging. No new field or metric is introduced.

## Observed RED and GREEN

The test-first commit is `c2a0d05b7984a7212e113db86d54293413b6d9cb`. The actual [first RED log](sk-e04-input-boundary/red.log) ran 22 test methods and reported **11 failures and 4 errors**. Subtests overlap methods, so these totals must not be subtracted to invent a pass count. The four errors are the reproduced API `TypeError` defect, not setup errors.

After the first implementation passed those 22 methods, review added two more CLI tests before changing their behavior. The [additional RED log](sk-e04-input-boundary/red-parser-diagnostics.log) records 24 methods and two failures: the decoder's existing integer-conversion error and a duplicate-key diagnostic echoing a newline. That run used an uncommitted intermediate implementation; [receipts](sk-e04-input-boundary/receipts.json) identify the actual source/test hashes rather than inventing a commit. The numeric regression sets `PYTHONINTMAXSTRDIGITS=4300` only in its child process so the decoder failure is reproducible.

The [GREEN log](sk-e04-input-boundary/green-final.log) records all 24 methods passing. The same receipt contains exact log hashes and source/test hashes. Initial sandbox commands could not start because the shared disk was full; those infrastructure failures were not counted as RED. Testing resumed after generated build data was cleaned up.

Raw unittest failure output contains trailing spaces. The first staged diff check detected them; they remain unchanged. `.gitattributes` excludes only these raw `.log` receipts from whitespace linting and pins their LF bytes. Source/document whitespace checks remain enabled.

Both legacy v1 and current v2 synthetic fixture CLI outputs were captured before implementation, then compared byte-for-byte after the fix. Exit codes remain 0 and the stdout SHA-256 values remain:

| Fixture | Before and after stdout SHA-256 |
| --- | --- |
| v1 | `b1de9eb18c1e5d1c8301be9651afdd65365a50bc59ce69cc64ba07d739048c3b` |
| v2 | `ec2aba1d6f4b3f3d54c28fff0f0752f136df3ff74a3b961a43b20fa0ff68d363` |

## Checks and remaining gates

```sh
python3 scripts/test_sk_e04_order_analysis.py -v
python3 -m unittest discover -s scripts -p 'test_*.py' -v
python3 -m unittest discover -s examples/evaluation/n07-pilot -p 'test_*.py'
python3 -m unittest discover -s examples/evaluation/n07-recorder -p 'test_*.py'
python3 -m unittest discover -s examples/evaluation -p 'test_*.py'
python3 tests/oracles/filmcraft-25-v1/test_checks.py
python3 tests/fixtures/execution-contract-v1/check_design.py
python3 scripts/check_repo.py
git diff --check
```

The scripts suite passed 99 methods, pilot helpers 12, recorder helpers 22 with one native replay skipped because this checkout has no built CLI, editorial checks 11, host-evidence rejection checks 6, and execution-design checks 5. These checks neither run a host nor supply human evidence. Repository/diff checks are recorded after these documents are written. Rust workspace checks have not been run for this change locally while another agent owns the shared Cargo slot. Hosted required CI and the expanded Linux/Windows/macOS SK-E04 workflow must pass on the final PR head before merge; no such result is claimed here.

Original #121 rescoring remains blocked on authorized exact drafts, packets, prompt/rubric and judge settings. #122 stays blocked; no holdout is created. The frozen allocation inventory and authorized evidence bytes still need separate checks. G2 decisions, human acceptance and G5 publication retain their existing gates.
