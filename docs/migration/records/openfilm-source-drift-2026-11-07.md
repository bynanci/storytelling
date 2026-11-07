# OpenFilm source drift: fixed map snapshot to current main

Checked on 2026-11-07 while reviewing Storytelling PR #2.

## Exact identities

| Role | Commit | `packages` tree | Evidence |
| --- | --- | --- | --- |
| Fixed source snapshot used by the initial 48-entry map | `7add4762fe8174d76f43b863375ea39b73121e76` | `deababde36d7cd38a3290c3329ef14fa3f12a90b` | PR #125 merge; includes PR #120 Tokio update and PR #121 coverage constraints |
| Latest OpenFilm main checked | `5f737a6c8f35dce48e213370ff27900962c9304c` | `e5199b8a364c8171e716d01660789b232f0bfe8a` | PR #126 merge commit |
| PR #126 feature head | `b4aba4c4bb70b8cb03edd54342f1b94cec649245` | — | Merged to main at `5f737a6c8f35dce48e213370ff27900962c9304c` on 2026-11-07 00:24:01 UTC |

The fixed snapshot remains useful for reproducing the initial inventory. It is
not a current-source claim. The ancestry check confirmed PR #120 merge
`d9fbf7801d64f7dc48ef852f63ef5babee6ae101` and PR #121 merge
`11acc54a54d53d9279c24d06e18dc08b29e6402d` are already ancestors of the fixed
snapshot. PR #125 itself is that snapshot. The only product-code change between
the snapshot and latest main is PR #126; other differences are task and
validation documentation.

Storytelling PR #2 was created at 2026-11-07 00:51:12 UTC, after PR #126 had
merged, but its original map called #126 a Draft. That statement was stale at
creation and is corrected in the current map. PR #2 remains a planning document;
the correction does not claim Rust parity or any Storytelling implementation.

## Impact on transfer tasks

- **OF-02:** its fixed frame-time source was already present in the snapshot.
- **OF-48:** visual keyframes are now an implemented OpenFilm host capability,
  not an upstream draft. Storytelling should not port a second evaluator or
  renderer. OF-35/36 must classify opacity/keyframe semantics against each
  target host and report losses explicitly.
- **M1-01:** begin from the latest OpenFilm main at implementation time. Inspect
  exact symbols/tests and source blob IDs for the chosen finite profile; the
  original map snapshot alone is not sufficient provenance.
- **Scope control:** speed ramps, compound editing and multicamera remain
  deferred. This delta adds no new planner or timeline ownership.

## Product-code paths changed by PR #126

The following list comes from a path diff between the fixed snapshot and latest
main; it is a navigation aid, not a substitute for reading the relevant code and
tests during an implementation task.

```text
apps/desktop/src/components/ClipAnimationInspector.vue
apps/desktop/src/components/RecordMonitor.vue
apps/desktop/src/components/TimelineEditor.vue
apps/desktop/src/composables/useTimelineEditor.ts
apps/desktop/src/i18n/modules/editor/en-US.ts
apps/desktop/src/i18n/modules/editor/ja-JP.ts
apps/desktop/src/i18n/modules/editor/zh-TW.ts
packages/application/src/editor.ts
packages/application/tests/editor.test.ts
packages/core/src/clip-animation.ts
packages/core/src/index.ts
packages/core/src/migration.ts
packages/core/src/models.ts
packages/core/src/sequence.ts
packages/core/src/validation.ts
packages/core/tests/audio-policy-schema.test.ts
packages/core/tests/clip-animation-schema.test.ts
packages/core/tests/clip-animation.test.ts
packages/core/tests/project.test.ts
packages/core/tests/sequence.test.ts
packages/render/src/clip-opacity.ts
packages/render/src/filter-graph-cli.ts
packages/render/src/index.ts
packages/render/tests/clip-opacity.test.ts
packages/render/tests/export-profile.test.ts
packages/render/tests/filter-graph-cli.test.ts
packages/render/tests/render.test.ts
packages/solver/src/audio-timing.ts
packages/solver/src/editing.ts
packages/solver/src/user-overrides.ts
packages/solver/tests/clip-animation.test.ts
schemas/openfilm-project-1.4.0.schema.json
tests/e2e/direct-clip-manipulation.spec.ts
tests/integration/export-profiles.test.ts
tests/integration/nle-handoff.test.ts
tests/unit/record-monitor.test.ts
```

No OpenFilm code or test is copied by this record. Preserve Apache-2.0 source
provenance and NOTICE requirements before any future transfer; Storytelling's
MIT license remains unchanged.

## Review boundary

This record verifies Git metadata and changed path identities. It does not
re-audit every OpenFilm function, prove target-host keyframe fidelity, or close
the Apache/MIT provenance decision for future copied or derived code. Each
transfer task must pin its own exact source and target SHAs and add task-specific
comparison evidence.
