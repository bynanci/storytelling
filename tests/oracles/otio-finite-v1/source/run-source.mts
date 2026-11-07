// SPDX-License-Identifier: Apache-2.0
// Independently authored synthetic source oracle; never a production dependency.
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { promisify } from "node:util";

const SOURCE_SHA = "7add4762fe8174d76f43b863375ea39b73121e76";
const CASES = [
  "one-still", "video-trim", "repeated-source", "decimal-rate", "explicit-ntsc", "unsupported-scale",
] as const;
if (process.argv.length !== 5) {
  throw new Error("Usage: node --import tsx run-source.mts <source-checkout> <output-dir> <producer-input-dir>");
}
assert.equal(process.version, "v24.22.1", "use frozen test-only Node 24.22.1");
const sourceRoot = resolve(process.argv[2]!);
const outputRoot = resolve(process.argv[3]!);
const inputRoot = resolve(process.argv[4]!);
const runnerRoot = dirname(fileURLToPath(import.meta.url));
const execute = promisify(execFile);
const git = async (...args: string[]) => (await execute("git", ["-C", sourceRoot, ...args], { encoding: "utf8" })).stdout.trim();
assert.equal(await git("rev-parse", "HEAD"), SOURCE_SHA, "source must match pinned baseline");
assert.equal(await git("status", "--porcelain", "--untracked-files=no"), "", "source tracked files must be clean");
const require = createRequire(import.meta.url);
const tsxPackage = require.resolve("tsx/package.json", { paths: [runnerRoot, dirname(inputRoot)] });
const tsxRoot = dirname(tsxPackage);
const loadedDependencyRoot = resolve(tsxRoot, "../..");
const tsxVersion = JSON.parse(await readFile(tsxPackage, "utf8")).version;
assert.equal(tsxVersion, "4.23.15", "use frozen test-only tsx loader 4.23.15");
const sha256 = (value: string | Uint8Array) => createHash("sha256").update(value).digest("hex");
const { exportTimeline } = await import(pathToFileURL(resolve(sourceRoot, "packages/exporters/src/index.ts")).href);
const { validateComposition, validateMediaAsset } = await import(pathToFileURL(resolve(sourceRoot, "packages/core/src/validation.ts")).href);
await mkdir(resolve(outputRoot, "fixtures"), { recursive: true });
await mkdir(resolve(outputRoot, "producer-inputs"), { recursive: true });

const sourcePaths = [
  "packages/exporters/src/index.ts",
  "packages/exporters/src/handoff.ts",
  "packages/core/src/models.ts",
  "packages/core/src/validation.ts",
  "packages/core/src/timebase.ts",
  "packages/application/src/nle-handoff.ts",
];
const sourceFiles = await Promise.all(sourcePaths.map(async path => ({ path, blob: await git("rev-parse", `HEAD:${path}`) })));
const records: Record<string, unknown>[] = [];
const seconds = (time: { value: number; rate: number }) => {
  assert.equal(typeof time.value, "number");
  assert.equal(typeof time.rate, "number");
  assert.ok(Number.isFinite(time.value) && Number.isFinite(time.rate) && time.rate > 0);
  return time.value / time.rate;
};
const close = (actual: number, expected: number, label: string) => {
  assert.ok(Math.abs(actual - expected) <= 1e-9, `${label}: ${actual} != ${expected}`);
};
for (const id of CASES) {
  const inputPath = resolve(inputRoot, `${id}.json`);
  const originalBytes = await readFile(inputPath);
  const input = JSON.parse(originalBytes.toString("utf8"));
  // The original scale fixture is a clone of video-trim with a transform;
  // its producer-input id remains video-trim. Never rewrite frozen input bytes.
  assert.equal(input.id, id === "unsupported-scale" ? "video-trim" : id,
    "producer input identity must match the frozen case declaration");
  const before = JSON.stringify(input);
  validateComposition(input.composition, { assetIds: input.assets.map((asset: any) => asset.id) });
  for (const asset of input.assets) validateMediaAsset(asset);
  const output = exportTimeline("otio", input.composition, input.assets, input.settings, {
    preserveConfiguredFrameRate: true,
  });
  const timeline = JSON.parse(output.content);
  assert.equal(output.extension, "otio");
  assert.equal(timeline.OTIO_SCHEMA, "Timeline.1");
  assert.equal(timeline.tracks.OTIO_SCHEMA, "Stack.1");
  assert.equal(timeline.name, input.composition.id);
  assert.equal(timeline.global_start_time.rate, input.settings.frameRate);
  assert.deepEqual(timeline.metadata.openfilm.composition, input.composition);
  assert.deepEqual(timeline.metadata.openfilm.settings, input.settings);
  assert.equal(timeline.metadata.openfilm.compatibility.realNleVerified, false);
  assert.equal(timeline.metadata.openfilm.compatibility.advancedEdits, "metadata-only");
  assert.deepEqual(timeline.tracks.effects, []);
  assert.equal(timeline.tracks.children.length, input.composition.tracks.length);
  close(seconds(timeline.tracks.source_range.duration), input.composition.duration, "composition duration");
  const observedClips: Record<string, unknown>[] = [];
  for (const [index, track] of timeline.tracks.children.entries()) {
    const originalTrack = input.composition.tracks[index];
    assert.equal(track.OTIO_SCHEMA, "Track.1");
    assert.equal(track.kind, "Video");
    assert.equal(track.name, originalTrack.id);
    assert.deepEqual(track.effects, []);
    assert.equal(track.children.length, originalTrack.clips.length, "finite inputs have no gaps or padding");
    for (const [clipIndex, clip] of track.children.entries()) {
      const originalClip = originalTrack.clips[clipIndex];
      const asset = input.assets.find((value: any) => value.id === originalClip.assetId);
      assert.ok(asset, "synthetic source must exist in input declarations");
      assert.equal(clip.OTIO_SCHEMA, "Clip.1");
      assert.deepEqual(clip.effects, []);
      assert.equal(clip.metadata.openfilm.clipId, originalClip.id);
      assert.equal(clip.metadata.openfilm.assetId, originalClip.assetId);
      assert.deepEqual(clip.metadata.openfilm.clip, originalClip);
      assert.equal(clip.media_reference.OTIO_SCHEMA, "ExternalReference.1");
      assert.equal(clip.media_reference.target_url, asset.uri);
      assert.equal(clip.media_reference.metadata.openfilm.contentHash, asset.contentHash);
      const start = originalClip.sourceIn ?? 0;
      close(seconds(clip.source_range.start_time), start, "source in");
      close(seconds(clip.source_range.duration), originalClip.timelineDuration, "native duration");
      const available = asset.mediaType === "image"
        ? start + originalClip.timelineDuration : asset.duration;
      close(seconds(clip.media_reference.available_range.duration), available, "declared available range");
      observedClips.push({
        clipId: originalClip.id, assetId: originalClip.assetId,
        targetUrl: asset.uri, sourceInSeconds: seconds(clip.source_range.start_time),
        durationSeconds: seconds(clip.source_range.duration),
        availableSeconds: seconds(clip.media_reference.available_range.duration),
        nativeValue: clip.source_range.duration.value, nativeRate: clip.source_range.duration.rate,
      });
    }
  }
  if (id === "unsupported-scale") {
    assert.ok(output.warnings?.some((warning: string) => warning.includes("METADATA ONLY")));
    assert.equal(timeline.tracks.children[0].children[0].metadata.openfilm.clip.transform.scale, 2);
  } else {
    assert.deepEqual(output.warnings, []);
  }
  assert.equal(JSON.stringify(input), before, "source exporter must not mutate supplied declarations");
  assert.deepEqual(await readFile(inputPath), originalBytes, "producer-input bytes must remain unchanged");
  const outputPath = resolve(outputRoot, "fixtures", `${id}.otio`);
  await writeFile(outputPath, output.content);
  await writeFile(resolve(outputRoot, "producer-inputs", `${id}.json`), originalBytes);
  records.push({
    id, producerInputId: input.id,
    inputSha256: sha256(originalBytes),
    exportedSha256: sha256(output.content),
    exportedBytes: Buffer.byteLength(output.content),
    outputFile: `fixtures/${id}.otio`,
    inputBytesUnchanged: true,
    suppliedValuesUnchanged: true,
    globalStartRate: timeline.global_start_time.rate,
    originalSettings: input.settings,
    observedClips,
    compatibility: timeline.metadata.openfilm.compatibility,
    sourceWarnings: output.warnings,
    expectedRustClassification: id === "unsupported-scale"
      ? "unsupported" : "preserved-seconds-identity-uri-with-intentional-content-clock-delta",
  });
}
assert.equal(await git("status", "--porcelain", "--untracked-files=no"), "", "oracle must leave source unchanged");
const evidence = {
  schemaVersion: 1,
  scope: "pinned-source-otio-export-only",
  sourceRepository: "bynanci/open-film",
  sourceHead: SOURCE_SHA,
  sourceFiles,
  sourceTrackedFilesUnchanged: true,
  producerOptions: { preserveConfiguredFrameRate: true },
  nodeVersion: process.version,
  nodeBinarySha256: sha256(await readFile(process.execPath)),
  nodeDistribution: "https://nodejs.org/dist/v24.22.1/node-v24.22.1-linux-x64.tar.xz",
  nodeDistributionSha256: "6c172a9ab6718f7b801fb5a7bd486af2ee5e32758086a04b84fcce2aa77095e6",
  tsxVersion,
  dependencyLockSha256: sha256(await readFile(resolve(loadedDependencyRoot, "package-lock.json"))),
  runnerSha256: sha256(await readFile(import.meta.filename)),
  command: [process.execPath, ...process.execArgv, import.meta.filename, sourceRoot, outputRoot, inputRoot],
  tsconfig: "source-checkout/tsconfig.json via TSX_TSCONFIG_PATH",
  cases: records,
  allPassed: true,
  classification: {
    preserved: "ordered occurrence IDs, source IDs, source/timeline durations and source URIs for five finite plain cases",
    intentionalDelta: "planned Rust writer uses a bounded common integer content tick clock and global_start_time:null; exact project FPS stays owner metadata, not a host FPS claim",
    unsupported: "source OTIO preserves scale as metadata-only with empty native effects; finite Rust profile must reject it",
  },
  notClaimed: [
    "Rust implementation or differential results", "official OTIO parser run",
    "full upstream test suite", "actual source-media verification", "audio preservation",
    "host compatibility", "FilmCraft or Resolve acceptance", "execution approval",
  ],
};
await writeFile(resolve(outputRoot, "source-evidence.json"), JSON.stringify(evidence, null, 2) + "\n");
console.log(JSON.stringify({ ok: true, sourceHead: SOURCE_SHA, cases: records.length, output: resolve(outputRoot, "source-evidence.json") }));
