<!-- SPDX-License-Identifier: Apache-2.0 -->
# Bounded StoryBrief policy oracle

Test-only, independently authored synthetic cases informed by pinned OpenFilm
`7add4762fe8174d76f43b863375ea39b73121e76`. No upstream test, media, provider or
host is copied/invoked. All new files here, including generated JSON inputs and
evidence, are part of the source-informed Apache-2.0 component; dependency licenses
remain their own. See root LICENSES/Apache-2.0.txt and NOTICE.openfilm.

This executes actual source `compose` only for selected hard membership/order,
missing references and contradictions. New StoryBrief wire/preferences/typed
diagnostics are downstream design; a clean audit is not planner feasibility.

Use official Node24.22.1 with the existing M1 oracle's pinned tsx4.23.15 loader:

```sh
TSX_TSCONFIG_PATH=/path/to/open-film/tsconfig.json node \
  --import /path/to/storytelling/tests/oracles/openfilm-finite-v1/node_modules/tsx/dist/loader.mjs \
  tests/oracles/story-brief-v1/run-source.mts /path/to/open-film \
  tests/oracles/story-brief-v1
```

`source-evidence.json` freezes runner/input/source identities and actual outcomes;
`inputs/*.json` are the exact independent downstream audit inputs. Rust comparison
will validate only these policy invariants, not equate the complete solver with
the new brief audit. No production Node/Python dependency is introduced.
