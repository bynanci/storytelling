<!-- SPDX-License-Identifier: MIT -->
# CommercialCopyV1 synthetic fixtures

These independently authored MIT fixtures support Issue #52 / N-R05. All people, products and measurements are fictional. No real product result is asserted.

## Product pilot

The fictional product 理序盒 has a trial record for three self-selected teams over one week. `PILOT_LOG@1` says average organizing time changed from 10 to 8 minutes for the same task. It does not describe all teams or measure overall efficiency. The scoped and overbroad sentences deliberately cite this same record. Both references are current; factual support remains a human decision.

| Input pair | Expected reference attention | Purpose |
| --- | --- | --- |
| `n05-pilot-copy-v1.json` + `n05-pilot-ledger-v1.json` | None | Both pilot claims have current citations; retain audience, feature, proposed slogan and CTA. |
| `n05-pilot-copy-no-reference-v1.json` + baseline ledger | `pilot-average-time` / missing | A factual sentence has no reference. |
| `n05-pilot-copy-dangling-ref-v1.json` + baseline ledger | `pilot-average-time` / dangling | A factual sentence names a source absent from the ledger. |
| Unchanged `n05-pilot-copy-v1.json` + `n05-pilot-ledger-revised-v1.json` | `pilot-average-time`, `all-teams-efficiency` / revised | The record corrects 10→8 to 10→9 at revision 2. Preserve the old copy and cited text; show the current record. |

The baseline is a valid N-04 ledger. Its sole event records an unaccepted assistant suggestion for creative `TAGLINE@1`; it is not an author acceptance or an editorial result. The revised ledger appends one source revision. `PRODUCT@1` is an unchanged control. The proposed slogan cites `TAGLINE@1`, whose origin stays `creative_proposal`; the CTA has no evidence reference.

The opt-in ReportV2 (`--report-v2`) keeps default ReportV1 unchanged. The four [expected reports](expected/) were specified before adding the new report fields. Tests invoke the actual CLI and compare the complete JSON result. Actual stdout/exit codes and a blank author-review table are in the [validation record](../../../docs/validation/n05-commercial-copy.md).

## Earlier gate-repair regressions

The existing `n05-copy-v1.json`, `n05-copy-revised-ref-v1.json` and `n05-copy-dangling-ref-v1.json` continue using `../narrative-v1/n04-revision-history-v1.json`. They cover current/missing/proposed copy, S02@1 after revision, and absent S99@1. They remain compatibility regressions for the original N-05 slice.

`n05-copy-reviewed-v1.json` preserves the requester's reviewed gate draft from concurrent commit `67e01469afd6bbe5b34daf47e00c01d1a6549c2d`. It removes the unsupported program claim and explicitly frames the slogan/CTA as proposals. Its [recorded assessment](../../../docs/validation/n05-gate-requester-review.md) applies to that gate sample, not the new product pilot.
