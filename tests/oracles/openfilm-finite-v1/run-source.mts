// SPDX-License-Identifier: Apache-2.0
// Independently authored synthetic oracle runner; test-only, never production.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const sourceHeadExpected = "7add4762fe8174d76f43b863375ea39b73121e76";
if (process.argv.length !== 4) throw new Error("Usage: node --import tsx run-source.mts <source-checkout> <output-dir>");
assert.equal(process.version, "v24.22.1", "use the frozen test-only Node version");
const sourceRoot = resolve(process.argv[2]!);
const root = resolve(process.argv[3]!);
const packageRoot = fileURLToPath(new URL(".", import.meta.url));
const git = (...args: string[]) => execFileSync("git", ["-C", sourceRoot, ...args], { encoding: "utf8" }).trim();
const sourceHead = git("rev-parse", "HEAD");
assert.equal(sourceHead, sourceHeadExpected, "source checkout must match frozen oracle");
assert.equal(git("status", "--porcelain", "--untracked-files=no"), "", "source tracked files must be unchanged");
const tsxVersion = JSON.parse(await readFile(`${packageRoot}/node_modules/tsx/package.json`, "utf8")).version;
assert.equal(tsxVersion, "4.23.15", "use the frozen test-only loader version");
const { exportTimeline } = await import(pathToFileURL(`${sourceRoot}/packages/exporters/src/index.ts`).href);
const { validateComposition, validateMediaAsset } = await import(pathToFileURL(`${sourceRoot}/packages/core/src/validation.ts`).href);
const { resolveProjectTimebase } = await import(pathToFileURL(`${sourceRoot}/packages/core/src/timebase.ts`).href);
const fixtureRoot = `${root}/fixtures`;
await mkdir(fixtureRoot, { recursive: true });
await mkdir(`${root}/producer-inputs`, { recursive: true });
const sha256 = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const picture = (id: string, kind: "image" | "video", duration?: number) => ({
  id,
  uri: `file:///synthetic/oracle/${id}.${kind === "image" ? "jpg" : "mp4"}`,
  mediaType: kind,
  name: `Synthetic ${id}`,
  ...(duration === undefined ? {} : { duration }),
  contentHash: (kind === "image" ? "a" : "b").repeat(64),
  tags: [],
  state: {},
  metadata: {},
});
const clip = (id: string, assetId: string, timelineStart: number, sourceIn: number, duration: number) => ({
  id, assetId, beatId: "opening", sourceIn, sourceOut: sourceIn + duration,
  timelineStart, timelineDuration: duration,
});
const composition = (id: string, clips: ReturnType<typeof clip>[]) => ({
  id, storyId: "synthetic-story",
  duration: clips.reduce((end, c) => Math.max(end, c.timelineStart + c.timelineDuration), 0),
  tracks: [{ id: `${id}-pictures`, type: "video" as const, clips }],
});
const still = picture("still-asset", "image");
const video = picture("video-asset", "video", 12);
const fixtures = [
  { id: "one-still", composition: composition("still-story", [clip("still-occurrence", still.id, 0, 0, 5)]), assets: [still], settings: { width: 1920, height: 1080, frameRate: 30 } },
  { id: "video-trim", composition: composition("trim-story", [clip("trim-occurrence", video.id, 0, 2, 3)]), assets: [video], settings: { width: 1920, height: 1080, frameRate: 30 } },
  { id: "repeated-source", composition: composition("repeat-story", [clip("first-occurrence", video.id, 0, 1, 2), clip("second-occurrence", video.id, 2, 6, 3)]), assets: [video], settings: { width: 1920, height: 1080, frameRate: 30 } },
  { id: "decimal-rate", composition: composition("decimal-story", [clip("decimal-occurrence", still.id, 0, 0, 5)]), assets: [still], settings: { width: 1920, height: 1080, frameRate: 29.97 } },
  { id: "explicit-ntsc", composition: composition("ntsc-story", [clip("ntsc-occurrence", still.id, 0, 0, 5)]), assets: [still], settings: { width: 1920, height: 1080, frameRate: 30000 / 1001, timebase: { version: 1 as const, rateNumerator: 30000, rateDenominator: 1001, dropFrame: false } } },
];
const evidence: Record<string, unknown>[] = [];
for (const fixture of fixtures) {
  const before = JSON.stringify(fixture);
  await writeFile(`${root}/producer-inputs/${fixture.id}.json`, before);
  validateComposition(fixture.composition, { assetIds: fixture.assets.map(a => a.id) });
  for (const asset of fixture.assets) validateMediaAsset(asset);
  const output = exportTimeline("json", fixture.composition, fixture.assets, fixture.settings);
  const document = JSON.parse(output.content);
  assert.equal(document.schemaVersion, "1.0.0");
  assert.deepEqual(document.composition, fixture.composition);
  assert.deepEqual(document.assets, fixture.assets);
  assert.deepEqual(document.settings, fixture.settings);
  assert.equal(JSON.stringify(fixture), before, "producer must leave supplied values unchanged");
  await writeFile(`${fixtureRoot}/${fixture.id}.json`, output.content);
  const occurrenceIds = document.composition.tracks.flatMap((t: any) => t.clips.map((c: any) => c.id));
  assert.equal(new Set(occurrenceIds).size, occurrenceIds.length);
  const derived = resolveProjectTimebase(fixture.settings);
  evidence.push({
    id: fixture.id, inputSha256: sha256(before), exportedSha256: sha256(output.content),
    exportedBytes: Buffer.byteLength(output.content), occurrenceIds,
    assetIds: document.assets.map((a: any) => a.id), settings: document.settings,
    sourceDerivedTimebase: derived, inputUnchanged: true,
  });
}
const decimal = evidence.find(e => e.id === "decimal-rate")!;
assert.deepEqual(decimal.sourceDerivedTimebase, { version: 1, rateNumerator: 30000, rateDenominator: 1001, dropFrame: false });
assert.equal((decimal.settings as any).frameRate, 29.97);
const edited = structuredClone(fixtures[1]);
(edited.composition.tracks[0]!.clips[0] as any).transform = { scale: 2 };
const editedBefore = JSON.stringify(edited);
await writeFile(`${root}/producer-inputs/unsupported-scale.json`, editedBefore);
const editedJson = exportTimeline("json", edited.composition, edited.assets, edited.settings);
assert.equal(JSON.parse(editedJson.content).composition.tracks[0].clips[0].transform.scale, 2);
assert.equal(JSON.stringify(edited), editedBefore);
await writeFile(`${fixtureRoot}/unsupported-scale.json`, editedJson.content);
evidence.push({ id: "unsupported-scale", inputSha256: sha256(editedBefore), exportedSha256: sha256(editedJson.content), exportedBytes: Buffer.byteLength(editedJson.content), sourceBehavior: "preserved-in-json", expectedFiniteRustClassification: "unsupported", inputUnchanged: true });
assert.equal(git("status", "--porcelain", "--untracked-files=no"), "", "oracle must leave source unchanged");
const manifest = {
  schemaVersion: 1,
  scope: "source-export-and-clock-oracle-only",
  sourceRepository: "https://github.com/bynanci/open-film",
  sourceHead,
  nodeVersion: process.version,
  nodeDistribution: "https://nodejs.org/dist/v24.22.1/node-v24.22.1-linux-x64.tar.xz",
  nodeDistributionSha256: "6c172a9ab6718f7b801fb5a7bd486af2ee5e32758086a04b84fcce2aa77095e6",
  nodeBinarySha256: sha256(await readFile(process.execPath)),
  tsxVersion,
  runnerSha256: sha256(await readFile(import.meta.filename)),
  dependencyLockSha256: sha256(await readFile(`${packageRoot}/package-lock.json`)),
  command: [process.execPath, "--import", "tsx", import.meta.filename, sourceRoot, root],
  tsconfig: "source-checkout/tsconfig.json (TSX_TSCONFIG_PATH)",
  sourceTrackedFilesUnchanged: true,
  fixtures: evidence,
  classification: {
    unchanged: "source composition/asset/settings JSON and occurrence identity",
    intentionalDelta: "finite importer retains decimal 29.97 as 2997/100 rather than silently adopting sourceDerivedTimebase 30000/1001",
    unsupported: "non-identity scale is preserved by source JSON but must be rejected by finite-v1",
    notClaimed: ["Rust differential results", "full source test suite", "actual source SHA256 verification", "audio preservation", "OTIO or host compatibility"],
  },
};
await writeFile(`${root}/source-evidence.json`, JSON.stringify(manifest, null, 2) + "\n");
console.log(JSON.stringify({ ok: true, sourceHead, nodeVersion: process.version, fixtures: evidence.length, output: `${root}/source-evidence.json` }));
