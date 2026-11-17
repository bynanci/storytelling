<!-- SPDX-License-Identifier: MIT -->
# SK-E06-F02 · Offline prospective layout preflight

[Issue #115](https://github.com/bynanci/storytelling/issues/115) ·
[original captured run](../research/evaluations/sk-e06-runs-2026-11-17/README.md) ·
[unchanged prospective record](../research/evaluations/sk-e06-runs-2026-11-17/future-run-layout-v2.json)

## 繁體中文操作與證據範圍

這個工具只預檢既有 F02 封存資料，不執行模型。原本的資料包在建立時已記錄
28 份檔案與 79 個連結；本次把相關檢查做成可重跑的入口與三平台回歸，
防止未來再次把 `references/` 展平成同一層。原始五份回答、判準爭議、
ZIP、前瞻記錄及 R1 分數保持原樣。

```sh
python3 scripts/sk_e06_packet.py --layout-preflight
python3 scripts/test_sk_e06_layout.py
python3 scripts/test_sk_e06_packet.py
```

成功只表示封存內容符合宣告的檔名、長度、SHA-256、Git blob 內容雜湊與
支援的行內 Markdown 相對路徑關係。工具明列 `scope:layout_snapshot_integrity_only`、
`can_dispatch:false`、`source_commit_membership_verified:false`；Git blob
雜湊相符不代表已驗證來源 commit 的成員資格或身分可信度。這也不是套件安裝、
完整 Markdown／anchor／HTML、媒體檢視、Skill 語意或真人效果的驗收。

預檢採用有上限的唯讀資料快照，不解壓寫入磁碟、不執行 ZIP 內容、不啟動模型、
Rust 或編輯器。新的研究案例派送、真人、媒體探測與 Host 觀察均為 0；
工程實作與原始碼審查使用了 AI agent，這些不是敘事試用或真人參與者。
F01 仍須實際維護者處理判準適用性並凍結新的規程，才可重新派送。
#121 原始 R1 資料與 #122 的前瞻門檻也不因此完成。

## Baseline and smallest delivery

Actual base: `62541d91c0c52ec6e49b5dc9c8350e92e83b409a`.
Existing `sk_e06_packet.py` validates the authored five-case reference packet;
it did not check the separate prospective full-layout archive. Its default
command/output remains unchanged. The new explicit `--layout-preflight` mode
reads the existing version `SK-E06-LAYOUT-PREFLIGHT-2`, and defaults to its
existing directory. An explicit positional directory permits controlled
offline fixtures, with no remote service or Rust requirement.

The implementation lives in [the research helper](../../scripts/sk_e06_layout.py)
and the small existing CLI routing branch. [Normal-entry/negative tests](../../scripts/test_sk_e06_layout.py)
run on Linux, Windows and macOS in [the existing packet workflow](../../.github/workflows/sk-e06.yml).
This is internal offline research tooling, not a new public Rust/CLI document
schema, TypeScript transport, authority provider or Skill behavior rule.

## Checks and limits

- Strict bounded UTF-8 JSON, duplicate/non-finite/type/identity rejection and
  unchanged empty model/human/decision fields for the prospective record.
- Compressed archive size/hash and exact canonical member set; declared member
  bytes, SHA-256 and Git blob content hashes checked against the same ZIP snapshot.
- Canonical ASCII POSIX paths under `skills/storytelling/`; refuse traversal,
  ambiguous/duplicate/case-colliding names and nonregular/encrypted/unsupported
  members. Nothing is extracted.
- Recompute the local inline-Markdown path multiset in memory, including repeated
  links. Required targets must exist in the same snapshot. This is not an HTML
  or heading-anchor parser.
- Bounded record, compressed/expanded archive, individual member and member count;
  fixed safe errors rather than Python tracebacks/private path details.
- Safe regular-file checks with nonblocking/no-follow flags where supported and
  descriptor checks; these are **not** a qualified race-resistant OS/media read
  view. Ancestor replacement and arbitrary concurrent filesystem mutation do
  not gain a trusted binding claim.

The actual frozen archive has 28 files and 79 local link paths. These counts
identify one pinned fixture, not people, independent runs or efficacy.
The checker does not query Git to attest source commit membership; it checks
the declared blob content fingerprints only. Source authenticity and later
dispatch metadata require their own review.

## Observed test-first development

The required real-entry RED is commit
`db260525c49ae9feafe1d60d6f4b6ef1ec0b7640`: the existing script exits 2 for
`--layout-preflight` and the new success assertion fails. This is an observed
behavioral failure, not a missing import or test-target claim. Subsequent
negative tests are observed before their minimal validation changes; actual
commands, exits and raw streams are retained with the final receipt index.

Implementation GREEN, independent review and final-head/post-merge CI are
recorded when actually completed; the new offline check never becomes evidence
that a model complied or a person benefited. Historical source/answers and
deviations are not retroactively repaired.

## Completed local verification and source review

The final helper source is `db24076ce14cdc9deebadbbba5010115cc4c983d`;
`91f34b3c610595db05897afd417aadbb58e6ebf0` adds stored-member
characterization without changing production behavior. The source history
retains initial RED `db26052`, implementation `867a53e`, linear scanning
`bff3371` and independent decoded-length/CRC correction `9597c4c`.

| Actually executed locally | Result |
| --- | --- |
| New layout suite | 27 passed / 0 failed / 0 skipped on Linux. Both STORED and DEFLATE have positive/CRC coverage. |
| Original reference-packet suite | 14 passed; no default-mode behavior changed. |
| Complete scripts discovery with explicitly bound all-feature native binary | 149 passed / 0 failed / 0 skipped. The recorded earlier discovery without that binding had four documented native-binary skips; they are exercised in the final run. |
| Existing Rust workspace | Default debug 350; all-feature debug 369; all-feature release 369 passed. Each 0 failed / 3 historical ignored. |
| Formatting / Clippy / rustdoc | All passed, Clippy/rustdoc warnings denied. |
| Legacy Python CLI byte comparison | Original base script versus current default mode: exit, stdout and stderr byte-identical. |
| Existing sources/data | Rust, public contracts, examples, Skill, old cases, captured answers, prospective record/archive and R1 artifacts unchanged. |

The [artifact index](sk-e06-layout-preflight/artifact-index.json) binds actual
raw streams, explicit runtime environment, normal-entry results and both
[AI source-review reports](sk-e06-layout-preflight/source-review.txt)
([follow-up](sk-e06-layout-preflight/source-rereview.txt)).
The [TDD inventory](sk-e06-layout-preflight/tdd-inventory.json) and
[data-only receipt archive](sk-e06-layout-preflight/tdd-receipts.zip) retain
79 original command records, including 25 nonzero observations. These include
expected rejection probes, behavioral RED and a corrected test assertion;
they are not 79 independent cases, model runs or participants. Intermediate
receipts do not supply a complete Git snapshot of every edit. Initial RED and
final passing source identities are separately recorded.

The whitespace-heavy incomplete-link case actually exceeded its two-second
watchdog before the forward-only scanner and then completed. This is one
synthetic process check, not a speedup study. The normal five-byte text fixture
also exposed a reader-size trust error; actual decoding is now bounded
independently, with EOF/length/CRC verified. The supported ZIP profile is ordinary
STORED/DEFLATE with versions 10/20, flags 0/UTF-8 and no descriptor/ZIP64/extra
fields. Unsupported framing is rejected; this is not a general ZIP library.

A prior review instance stopped at the service content check and produced no
complete report; [the attempt record](sk-e06-layout-preflight/incomplete-review-attempt.json)
does not count it as completed acceptance. The ordinary small-text regression
confirmed the source-consistency issue, and a separate final source-only reviewer
found no blocker. That reviewer read source/Git/hashes, did not execute tests,
and granted no human/G2 decision. Its optional stored-member coverage observation
was closed by the test-only follow-up.

Research-case dispatches, human/media/Host observations remain **0**. Engineering
AI conversations are technical work, not trial evidence. F01 adjudication,
#121/#122 and all product/Host/G5 gates remain open. Hosted exact-head and
post-merge verification is recorded on the delivering PR after actual completion.

## Reproduce and verify retained receipts

Python 3.12 is used in the three-OS workflow. These checks require no remote
model, Rust installation or media editor:

```sh
python3 scripts/sk_e06_packet.py --layout-preflight
python3 scripts/test_sk_e06_layout.py
python3 scripts/test_sk_e06_packet.py
python3 scripts/check_repo.py
```

To verify the stored evidence without extracting or executing it:

```sh
python3 - <<'PY'
from pathlib import Path
import hashlib, json, zipfile
root = Path('docs/validation/sk-e06-layout-preflight')
for item in json.loads((root / 'artifact-index.json').read_bytes())['files']:
    raw = (root / item['path']).read_bytes()
    assert len(raw) == item['bytes']
    assert hashlib.sha256(raw).hexdigest() == item['sha256']
inventory = json.loads((root / 'tdd-inventory.json').read_bytes())
with zipfile.ZipFile(root / 'tdd-receipts.zip') as archive:
    assert archive.read('inventory.json') == (root / 'tdd-inventory.json').read_bytes()
    for item in inventory['receipts']:
        raw = archive.read(item['file'])
        assert len(raw) == item['bytes']
        assert hashlib.sha256(raw).hexdigest() == item['sha256']
print('Raw evidence bytes match; no research case or Host action executed')
PY
```

The complete-script runner source and explicit native executable fingerprint are
in `final-full-python.json`. This enables the existing four native-file tests
during discovery; no test/assertion is removed. Rust receipt overrides disable
debug symbols only to limit disk use and retain debug-mode assertions. Those
receipts cover unchanged Rust backends, not new model behavior or TypeScript
transport acceptance.
