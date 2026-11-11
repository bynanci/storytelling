# Software development lifecycle

<a name="sdlc-從需求到維運的交付規則"></a>
<a id="從需求到維運的開發流程"></a>

English · [繁體中文](SDLC.zh-TW.md)

Each change must deliver a result someone can use, review and test independently. The [roadmap](../ROADMAP.md) sets the delivery order. The [task list](../TASKS.md) defines the scope. The [requirements](REQUIREMENTS.md) connect each requirement to its tests and evidence.

<a id="階段與放行門檻"></a>

## Development gates

| Gate | Required evidence | Exit condition | Owner |
| --- | --- | --- | --- |
| G0 · Define the problem | Use case, success measure and exclusions. | State the result the user needs. Check existing features for overlap. | Product owner and maintainer |
| G1 · Define the requirement | Requirement ID, acceptance examples, risks and dependencies. | Separate mandatory conditions from preferences. Identify blocked work and missing data. | Maintainer and engineer |
| G2 · Design the change | Architecture decision, schema or API, data owner and failure cases. | Define testable boundaries. Address migration, approval and security. | Engineer and reviewer |
| G3 · Implement | A focused branch, pull request (PR) and test record. | Observe a failing test before the change. Run it after the change. Preserve existing source data and permissions. | Implementer |
| G4 · Check the result | Unit, contract and integration tests, CI and a review of the changes. | Run all checks on the same commit. Resolve correctness blockers. Record manual acceptance separately. | Reviewer and maintainer |
| G5 · Release | Compatibility matrix, versions, dependencies, software bill of materials (SBOM), migration and rollback instructions. | Test the complete release scope. Obtain explicit maintainer approval for this release. | Maintainer |
| G6 · Maintain | Defects, reproductions, usage results, security advisories and host changes. | Limit the impact of serious defects. Add findings to the next G0 or G1. | Maintainer and users |

One person can hold several roles. Identify self-review and independent review separately. An AI assistant can record human approval only when a person gives it. Only the signer can provide a Developer Certificate of Origin (DCO).

A merge requires explicit user authorization and passing checks for the exact PR head. The existing authorization covers merging completed, verified work and marking its task complete. Human editorial acceptance requires its own record. Releases, tags, repository policies and branch rules each need explicit authorization. See the [delivery and authorization record](../TASKS.md#accepted-lifecycle).

<a id="工作狀態"></a>

## Work states

`proposed → ready → in-progress → review → verified → accepted`

Mark blocked work as `blocked`. Record its cause, owner, evidence and condition for resuming. Track implementation, automated verification, pending manual acceptance and release separately.

<a id="開始工作的條件"></a>

## Definition of ready

Before the PR, record the following information.

1. Select a task ID. Describe the result for the user.
2. Define the scope and exclusions.
3. Record the base commit SHA, dependencies and data contract.
4. Define the acceptance tests. Limit the PR to work that reviewers can assess independently.
5. Describe compatibility, privacy and security effects.

If media, a host or a signer identity is missing, mark the affected gate as blocked. Identify the missing evidence.

<a id="完成工作的條件"></a>

## Definition of done

1. Link each requirement to its implementation and tests.
2. Record the observed failure before implementation (RED). Record the passing result after the change (GREEN).
3. Read all required continuous integration (CI) results for the current PR head. Each required check must finish and pass.
4. Check file, source, plan and revision identities. Exercise the failure paths in the contract.
5. Update the documentation and risk record. Resolve review findings.

Save observations for checks that need a real host, listening session or clean machine. Accept only the tested scope when observations are missing. G5 governs publication.

<a id="提交合併請求"></a>
<a id="pr-規則"></a>

## Submit a pull request

Deliver one coherent result per PR. Freeze schema or time contracts before parallel implementation. Include the following details.

- Task and requirement IDs.
- Base and head commit SHAs.
- Reason for the change, compatibility, limits and rollback steps.
- Test commands, results and evidence for the head commit.

Run new checks after any change to the head commit. A failing test-first commit can serve as RED evidence. Only candidates that meet G5 qualify for publication.

The initial preparation authorization covered branches and PRs. The task history records later merge authority and deliveries. Apply the authorization for the current task. See the [authorization history](../TASKS.md#accepted-lifecycle).

<a id="持續整合與儲存庫政策"></a>
<a id="ci-與真正的治理"></a>

## CI and repository policy

The `Required gates` job combines quality checks, dependency audit and Rust tests on Linux, Windows and macOS. The workflow uses read-only permissions and a pinned checkout commit. Formatting changes cause the check to fail. See the [CI workflow](../.github/workflows/ci.yml).

An administrator must authorize and configure branch protection. Document the review policy that the repository actually uses. A sole maintainer needs a review process they can carry out. Sensitive changes need a human decision. Automatic approval cannot establish human review.

<a id="變更管理與相容性"></a>

## Compatibility and change control

Version the public Rust API, wire formats, planner and compiler rules separately. Maintain versions for adapters and host profiles. For wire format changes, test old input and migration. Check rejection of unsupported newer versions.

Keep private paths, media, location data and credentials out of public files. Obtain explicit maintainer authorization before changing license terms or publishing a release.

<a id="維運"></a>

## Maintenance

Use a minimal synthetic case for public defect reports. Report security issues privately under the [security policy](../SECURITY.md).

When dependencies or hosts change, reassess compatibility and security advisories. Retain the failure evidence and corrective steps when withdrawing a defective release. The project has no formal release. Incident and rollback procedures are release criteria during development. Installation evidence covers only the tested package, platform and version.
