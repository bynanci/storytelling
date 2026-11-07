<!-- SPDX-License-Identifier: Apache-2.0 -->

# Fixed-window planner v1 synthetic declarations

These independently authored inputs exercise the frozen downstream
[planner contract](../../../docs/migration/FIXED_WINDOW_PLANNER_V1.md).
The required membership/order/exclusion policy is informed by pinned
`bynanci/open-film@7add4762fe8174d76f43b863375ea39b73121e76`, especially
`packages/solver/src/index.ts` (`compose`, `orderedAssets`; blob
`c701b985495c8c76292880b5acbc9c062b03006c`) and
`packages/core/src/models.ts` (blob `58dbe93f132577842b7971e0d8aad61aff1d78c6`).

The explicit fixed windows, exact rational input, declared ranking and output
schema are downstream design. These are invented IDs and declared hashes,
without media, paths, private facts, credentials or upstream copied fixtures.
JSON cannot contain an embedded comment, so this directory notice supplies the
license marker for both inputs: **SPDX-License-Identifier: Apache-2.0**.
Original repository MIT LICENSE bytes remain unchanged. See the task provenance
inventory for the independently authored component and applicable upstream notices.

- `valid.json`: manual order detail then opening is preserved despite the
  candidate order and scores. Windows are [1/10,3/10) and [0,1/10), total3/10
  seconds at common clock10. Required choices meet the soft target exactly.
  The result contains a real M0-valid declared plan, without source or host proof.
- `conflicting.json`: the required opening asset is also excluded. Return
  planner.brief_conflict with its globally rooted declaration path; no plan.

The finite profile does not establish multi-beat solver parity, general
feasibility, actual source identity, editor execution or editorial acceptance.
