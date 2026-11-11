# Contribute to Storytelling

<a name="contributing"></a>
<a name="貢獻指南"></a>

English · [繁體中文](CONTRIBUTING.zh-TW.md)

Start with the [task list](TASKS.md). Find the requirement and its acceptance criteria in the [requirements register](docs/REQUIREMENTS.md). Check open pull requests for work on the same task. Record the base commit. Use a separate branch. Submit a pull request (PR) for review.

<a name="提交-skill-範例或平台指南"></a>

## Contributing a skill example or host guide

Start with a real authoring question. Include the request, the supplied material and the output you expect. Use synthetic material or material you can share publicly.

For a host guide, identify the application and installation scope. Specify whether the installation applies to one project, a user account or a hosted account. Distinguish installation instructions from a test in the application. Report what you observed. Link to the [platform guide](docs/PLATFORMS.md).

Edit the source in [skills/storytelling](skills/storytelling). Codex discovers this directory through a relative symbolic link. Use relative links inside the skill. If you introduce an asset type, update the file list in [package.json](package.json). Check the contents of the npm package.

<a name="撰寫繁體中文文件"></a>

## Write documentation

English is the default for public documentation, package instructions and contribution forms. Use the unsuffixed filename for English. Add a link to the full Traditional Chinese edition when one exists. Keep translated pages beside the English page with the suffix `.zh-TW.md`.

1. Start with the reader's task.
2. Give the exact command or input.
3. Show the expected result.
4. Identify conditions that limit the result.

Use one term for each technical concept. Keep instructions short. Preserve commands, version numbers, permissions and source credits. Give an editor name only when the name identifies an integration or test. Preserve the author's voice in creative examples. See the [English writing guide](https://github.com/danyuchn/asd-ste100-skill/blob/a4905d922500dd7530ca8a0eb9e4940494255709/SKILL.md) and the [Traditional Chinese writing guide](https://github.com/lemonhall/asd-ste100-skill-zh/blob/01345c50d240ab5f5ee34c95702adb6078a70f1b/SKILL.md).

<a name="執行檢查"></a>

## Checking the project

Install the Rust toolchain that the repository specifies. Run these commands from the repository root.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --release --locked
cargo doc --workspace --no-deps --locked
python3 scripts/check_repo.py
python3 scripts/test_skill_distribution.py
```

The repository checks require Python 3.11 or later. The Rust CLI can run without Python. Submit dependency and toolchain updates separately. Explain any change to security, licenses or compatibility. Check unexpected changes to `Cargo.lock`.

<a name="提交拉取請求"></a>

## Submit a pull request

- Link the task and requirement IDs. Include a failing test and the result after the fix for behavior changes.
- Record the CI results for the exact commit. List any checks that you could not run. Separate parser tests from checks in a real host.
- State the risks, migration steps and rollback method. Keep private projects, paths, credentials, GPS data and private media out of the repository.
- Preserve [licenses and attribution](PROVENANCE.md). Sign only for yourself. The project does not currently require a Developer Certificate of Origin.
- Disclose AI assistance. Identify a review that you performed yourself.
- Obtain explicit owner approval for publication, tags or branch rule changes. Follow the [development process](docs/SDLC.md) and [agent instructions](AGENTS.md).
