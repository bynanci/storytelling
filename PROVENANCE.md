# Licenses and source records

<a name="provenance"></a>

## Which license applies?

- The portable Skill and the npm package use the MIT license. The package includes the [project license](LICENSE) and the [Skill license](skills/storytelling/LICENSE).
- Original project code and documentation use MIT, except for files that specify a different license.
- Some Rust components and tests use [Apache-2.0](LICENSES/Apache-2.0.txt). The file inventories below identify those files and their sources.
- The [third-party notice](LICENSES/third-party/NOTICE.txt) contains the original attribution for those components. Names in that notice identify the upstream project and its contributors.

The copyright line in both MIT files reads `Copyright (c) 2026 bynanci`. The inventories record the upstream notice with its original Git object ID.

## Repository origin

The repository started at commit `551ff38920e2085e1d53a7ba777c9680678e9725`.
The original MIT LICENSE has Git object ID `f317799529d13f4d9301daa2dba63dfea559ff69`. The file retains its original bytes.

On 2026-11-07, the OpenAI assistant wrote the initial code, tests and SDLC documents at the maintainer's explicit request.
GitHub API commits may identify the connected account as the author. The commit messages disclose how the implementation originated.
These records claim no maintainer Developer Certificate of Origin (DCO) or cryptographic signature.

The Rust architecture proposal from the user informed the scope. It supplied no implementation code.
The initial work copied no upstream implementation, tests, fonts, private media or model weights.
M0 and M1-01 use synthetic source declarations and hashes. M1-03 generates synthetic files and records the SHA-256 of their actual bytes.

Cargo 1.85.0 generated `Cargo.lock` in hosted run 42314783329, the initial failing test run (RED).
The repository retains this lockfile for locked builds and checks. M0 directly depends on `serde` and `serde_json`.
M1-03 adds the SHA2 dependency described below. The lockfile lists transitive packages.
Each dependency retains its own license. Review those licenses before distributing binaries.

For each future import, record the origin repository, exact source commit, source path, license, notice, modifications and test evidence.
Retain attribution when rewriting code in Rust. Assess license compatibility separately for each future port.

## M3-01/P2 selected proposal / read-only OTIO delivery

The approved Apache component policy covers the bridge, strict planner and audit DTO extensions, synthetic regressions and fixtures, and the official-parser test oracle.
The [per-file inventory](docs/migration/provenance-m3-planner-review.json) records each pinned source commit, path, Git object ID and modification.
The project retains the original MIT and upstream notices. The CLI, CI and module integration code retain MIT.
Independently written design documents and other documentation also use MIT.
The bridge adds no production dependency and copies no upstream code, tests or media.

The test uses the same official OTIO 0.18.1 wheel and checks its pinned hash.
The source-policy review examined the upstream code without running it. The new oracle runs the native CLI and official OTIO.
The selected source files matched the pinned versions on upstream main at commit `08e38b6a63a0b0fbcfd3a073c8bfa5dcc8c08224`.

At the user's request, an AI assistant wrote the implementation and review documentation.
This delivery records no independent review, maintainer signature, approval grant or release.

## M1-01 finite import component

On 2026-11-07, the maintainer approved the policy for source-derived components, SPDX identifiers and notices in the task conversation.
The project retains the original MIT LICENSE bytes. The new importer contracts, Rust mapper, regression tests and synthetic fixtures use Apache-2.0.
The core and engine packages declare `MIT AND Apache-2.0`. Existing integration files retain MIT.
The record claims no maintainer DCO or cryptographic signature.

The [per-file inventory](docs/migration/provenance-m1-01.json) identifies the upstream repository, commit, path and Git object for each source.
It identifies the target files, their relationship to the upstream files and the modifications.
The project retains the original upstream [LICENSE](LICENSES/Apache-2.0.txt) and [NOTICE](LICENSES/third-party/NOTICE.txt) verbatim, including historical and third-party attribution.

The authors wrote the production mapper after inspecting the source contracts. They did not mechanically copy any upstream implementation or test fixture.
The test oracle uses independently written synthetic inputs. Its dependencies retain their own licenses. The lockfile records those dependencies without changing their licenses.

This work imported no private media, fonts, model weights or user project files.
M1-01 fixtures declare synthetic source hashes. Real media and host checks, binary distribution and downstream notice review have separate acceptance gates.

## M1-02 finite interchange component

The approved Apache component policy covers the new OTIO contracts, pure serializer, content-clock helper, synthetic regressions and oracles.
The [per-file inventory](docs/migration/provenance-m1-02.json) identifies the files. The MIT license and the upstream license and notice retain their original bytes.
This work added no production dependency and made no change to `Cargo.lock`.

The tests use official OpenTimelineIO 0.18.1 and its native extensions as an oracle.
The tests pin the SHA of the Linux wheel for CPython 3.12. CI pins the official setup-python action to a full commit.
The interpreter ABI is 3.12. The evidence records the actual versions.
Downloaded dependencies retain their own licenses outside the first-party Apache inventory. The repository bundles no wheel, node_modules directory or host executable.

## M2-01 source-informed brief audit

The approved component policy covers the independently written StoryBrief DTOs, pure audit, regression tests and synthetic source-policy oracle.
The [M2-01 inventory](docs/migration/provenance-m2-01.json) identifies each Apache-2.0 file and the pinned source paths and Git objects.
It lists the original MIT integration code separately.
The authors copied no upstream implementation, tests or media. This project defines the typed brief conflicts and soft preferences.
The [comparison record](docs/migration/records/M2-01.md) covers ten selected upstream policies for hard constraints.

## M1-03 bounded verification and real host test

The pure component that checks supplied buffers and the test-only host harness carry Apache-2.0 SPDX identifiers.
The [per-file inventory](docs/migration/provenance-m1-03.json) records their origins. The original MIT LICENSE and the upstream notices retain their original bytes.
The repository bundles no upstream tests or media. It also excludes the FilmCraft source, SDK and executable.
The tests use the official FilmCraft executable as an external tool. Its license is `MIT OR Apache-2.0`.

The [dependency decision](docs/migration/DEPENDENCY_REVIEW_M1_03.md) covers SHA2 version 0.10.9 and the complete set of locked dependency archives and licenses.
The [proof record](docs/migration/records/M1-03.md) identifies the generated synthetic media and the native, host and output identities.
Earlier fixtures contain declarations only. Native receipts establish the identity of the immutable bytes supplied to the component.
They make no claim about future files, host authority or approval authority.

## M2-02/O2 private exact-time construction optimisation

The 2026-11-08 change retained the accepted component policy and the fixed source baseline.
The [inventory](docs/migration/provenance-m2-clock-opt.json) covers the new exact-clock regression and benchmark module.
The tests use the previous downstream Rust factory as a baseline.
The M2-02 inventory covers the modified planner, clock and integration files.
The change introduced no upstream code, tests or media. It changed no dependency or notice.
The independently written task, requirements and benchmark evidence documents use MIT.
Self-review provides no independent M3 G2 approval or maintainer signature.

## M2-02/O1 private ordering optimisation

The 2026-11-08 change retained the approved Apache component policy and the fixed source baseline.
The [per-file inventory](docs/migration/provenance-m2-order-opt.json) lists the new private ordering and test modules.
The M2-02 inventory covers the existing planner and integration tests.
The independently written task, requirements and benchmark documents retain MIT.
The change introduced no upstream implementation, tests, media or new dependency. It changed no notice.
The record discloses AI implementation and self-review. It claims no maintainer signature or G2 approval.

## M2-02 fixed-window planner

The source-informed core and engine planner modules, synthetic regressions and pinned source oracle use Apache-2.0.
The [per-file inventory](docs/migration/provenance-m2-02.json) identifies them. The original MIT and upstream notices retain their original bytes.
The existing CLI and module exports retain MIT. Independently written design, status and usage documents, including the [CLI guide](docs/CLI_GUIDE.md), also retain MIT.
The authors copied no source implementation or tests. This work added no production package or crate.
The exact source fields, windows and caller scores are declarations. The comparison covers selected policies. It makes no claim of full solver parity.

## M2-03 live planner shadow

The [per-file inventory](docs/migration/provenance-m2-03.json) lists the independently written Apache-2.0 components for the selected domain.
These include the source producer, exact classifier, case generator, bounded process and runner, and synthetic regression baselines.
The original MIT and applicable upstream notices retain their original bytes. Independently written CI, status and delivery documents retain MIT.
This work copied no upstream tests, code or media. It neither copied nor installed a new production package.
The [test-only Node/npm dependency record](docs/migration/dependency-evidence-m2-03.json) lists the external tools and the existing integrity lock.

## M3-01/P1 accepted bounded finite-OTIO review binding

The [per-file inventory](docs/migration/provenance-m3-01-p1.json) covers every new or extended Apache module and test.
It records the source repository commit, source path and Git object, modifications and component license.
The original MIT LICENSE and upstream license and notice retain their original bytes.
The authors independently wrote the review DTOs, binding engine and synthetic regressions under the approved Apache-2.0 component policy.
They copied no upstream implementation, test, fixture or media.
The CLI and module integration code use MIT. The independent usage, contract and status documents also use MIT.

The authors wrote the test-only harness `scripts/oracles/m3-01-openfilm-runtime.mjs` independently under MIT.
It imports the pinned upstream runtime dynamically without copying code or fixtures.
The Node/tsx runner operates outside the Rust workspace. The implementation reuses `sha2` and adds no production dependency.

The component reuses the synthetic finite-v1 composition fixture.
Its scope excludes signer trust, grant issuance, host observation, replay safety, source verification and execution.
PR #32 checks passed on commit `2e7662405a49698be1ebee2136f54ab539ef7c2e` in hosted run [42425697537](https://github.com/bynanci/storytelling/actions/runs/42425697537).
The six jobs covered Linux, Windows, macOS, Required, Quality and Dependency Audit.
The [M3-01 runtime evidence](docs/migration/records/M3-01-RUNTIME-ORACLE-2026-11-08.md) records the paired source-runtime results, fixed digest vectors and error-order vectors.

The [G2 acceptance record](docs/migration/records/M3-01-G2-ACCEPTANCE-2026-11-08.md) records human acceptance of the bounded profile.
The user explicitly requested a textual sign-off. The record contains no GPG or SSH signature and establishes no project-wide DCO policy.
Acceptance grants no authority beyond the scope above.
The original evidence does not establish that a functional test failed before implementation.
The sign-off waived that historical RED result as a completion requirement and retained the original evidence.

PR #39 fixed the evidence. PR #40 added runtime and test coverage. Both merged into main.
For PR #40, both six-job runs (42477209609 and 42477206413) passed on commit `a9b80723e0171497d9b1cc63bc30c6fac88e0ecc`.
All six jobs also passed on main in run 42477392236. The merge tree matches the accepted commit.
The authors independently wrote the follow-up harness under MIT. At runtime it imports only the pinned upstream service.
The paired synthetic results confer no signer authority. They establish no full source, host or digest parity.
The [delivery register](docs/delivery/PR_REGISTER.md) contains the receipts for PR #39 and PR #40.

PR #42 published the signed acceptance at commit `65914146227faac62c4c045f3548786067376925` and merged at `92f7f20b63656ec3e676309310f8a5350d6d0662`.
All six jobs passed in PR run 42484654708 and post-merge main run 42484845782.
The appended delivery receipt records completion of issue #27 for the accepted bounded profile.
