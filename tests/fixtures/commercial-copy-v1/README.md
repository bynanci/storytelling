<!-- SPDX-License-Identifier: MIT -->
# CommercialCopyV1 synthetic fixtures

These independently authored fixtures support Issue #52 / N-R05. They reuse the fictional two-neighbors gate-repair observations from the N-01/N-03 examples and the accepted N-04 revision ledger. No product performance, repair duration, cost reduction or durability result is asserted.

| File | Expected reference status | Purpose |
| --- | --- | --- |
| `n05-copy-v1.json` | `current`, `missing`, `not_applicable` | A factual test result with a current source reference, an unsupported U01-like claim, a proposed slogan and a proposed call to action. |
| `n05-copy-reviewed-v1.json` | `current`, `not_applicable` | Copy revision 2 applies the requester's review: delete the unsupported program claim and explicitly prefix the slogan and CTA as proposals. |
| `n05-copy-revised-ref-v1.json` | `revised` | A factual statement still cites S02@1 after the ledger has revised S02 to revision 2. |
| `n05-copy-dangling-ref-v1.json` | `dangling` | A factual statement cites an absent source ID. |

The CLI uses `../narrative-v1/n04-revision-history-v1.json` as its evidence ledger. Statuses check reference identity and revision only; they do not establish that a source supports the wording or that copy is fit to publish.
