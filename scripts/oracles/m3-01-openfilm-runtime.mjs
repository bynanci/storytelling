// SPDX-License-Identifier: MIT
// External, test-only source oracle. It imports the pinned OpenFilm service;
// it is not part of production runtime.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const EXPECTED_OPENFILM_SHA = "7add4762fe8174d76f43b863375ea39b73121e76";
const sourceRoot = resolve(process.env.OPENFILM_SOURCE_ROOT ?? process.cwd());
const storytellingRoot = resolve(process.env.STORYTELLING_ROOT ?? process.cwd());
const sourceSha = process.env.OPENFILM_SOURCE_SHA;
if (!sourceSha) throw new Error("OPENFILM_SOURCE_SHA is required");
assert.equal(sourceSha, EXPECTED_OPENFILM_SHA, "unexpected OpenFilm source revision");

const fixturePath = join(
  storytellingRoot,
  "tests/fixtures/openfilm/finite-v1/repeat-source.json",
);
const fixtureBytes = readFileSync(fixturePath);
const fixture = JSON.parse(fixtureBytes.toString("utf8"));
const fixtureSha = createHash("sha256").update(fixtureBytes).digest("hex");
const tempRoot = mkdtempSync(join(tmpdir(), "storytelling-m3-oracle-"));
const stalePublish = async (service, review) => {
  await assert.rejects(
    service.publish({
      projectId: review.projectId,
      compositionId: fixture.composition.id,
      revision: review.revision,
    }),
    /Stale handoff revision/,
  );
};

try {
  const sourceModule = await import(
    pathToFileURL(join(sourceRoot, "packages/application/src/nle-handoff.ts")).href,
  );
  const composition = structuredClone(fixture.composition);
  const project = {
    schemaVersion: "1.0.0",
    id: "project-1",
    title: "Synthetic M3 review oracle",
    createdAt: "2026-01-01T00:00:00Z",
    updatedAt: "2026-01-01T00:00:00Z",
    mediaLibraries: [],
    stories: [{ id: composition.storyId, maxDuration: 20 }],
    timelines: [composition],
    settings: fixture.settings,
  };
  const assets = new Map(fixture.assets.map((asset) => [asset.id, asset]));
  const projectDirectory = join(tempRoot, "project");
  const { mkdirSync } = await import("node:fs");
  mkdirSync(projectDirectory);
  const projectPath = join(projectDirectory, "project.json");
  const persist = () => writeFileSync(projectPath, JSON.stringify(project));
  persist();
  const host = {
    directory: projectDirectory,
    get project() {
      return project;
    },
    getSourceAsset(id) {
      return assets.get(id);
    },
    render() {
      throw new Error("unexpected render in review oracle");
    },
  };
  const upstream = new sourceModule.NleHandoffService(host);

  const sourceInitial = upstream.review(composition.id);
  const sourceUnchanged = upstream.review(composition.id);
  assert.equal(sourceInitial.revision, sourceUnchanged.revision);

  project.updatedAt = "2026-02-01T00:00:00Z";
  persist();
  const sourceTimestampOnly = upstream.review(composition.id);
  assert.equal(sourceInitial.revision, sourceTimestampOnly.revision);

  const sourceBaseReview = sourceTimestampOnly;
  project.id = "project-2";
  persist();
  const sourceChangedProject = upstream.review(composition.id);
  assert.notEqual(sourceBaseReview.projectId, sourceChangedProject.projectId);
  assert.notEqual(sourceBaseReview.revision, sourceChangedProject.revision);
  await stalePublish(upstream, sourceBaseReview);

  project.id = "project-1";
  project.timelines[0].tracks[0].clips[1].id = "occurrence-b";
  persist();
  const sourceChangedContent = upstream.review(composition.id);
  assert.notEqual(sourceInitial.revision, sourceChangedContent.revision);
  await stalePublish(upstream, sourceInitial);

  await assert.rejects(
    upstream.publish({
      projectId: sourceChangedContent.projectId,
      compositionId: composition.id,
      revision: sourceChangedContent.revision,
      approved: true,
    }),
    /Unsupported handoff request/,
  );

  process.stdout.write(
    `${JSON.stringify({
      source: {
        revision: sourceSha,
        service: "NleHandoffService.review/publish",
        unchanged: "same revision",
        updated_at_only: "same revision",
        project_id_change: "changed revision; old publish rejected",
        composition_change: "changed revision; old publish rejected",
        forged_approval: "unsupported request rejected",
      },
      fixture_sha256: fixtureSha,
      limitation:
        "This executes only the pinned upstream NLE review service. Pair it with the Storytelling CLI regression output recorded in the M3-01 runtime comparison; the systems use different input domains and digest encodings.",
    })}\n`,
  );
} finally {
  rmSync(tempRoot, { recursive: true, force: true });
}
