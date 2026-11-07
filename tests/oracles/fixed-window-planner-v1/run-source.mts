// SPDX-License-Identifier: Apache-2.0
// Independently authored synthetic source-policy oracle; no upstream test copied.
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { readFile, mkdir, writeFile } from "node:fs/promises";
import { dirname, relative, resolve } from "node:path";
import { promisify } from "node:util";
import { fileURLToPath, pathToFileURL } from "node:url";

const [sourceArg, outputArg, ...extra] = process.argv.slice(2);
assert(sourceArg && outputArg && extra.length === 0,
  "Usage: run-source.mts SOURCE_CHECKOUT OUTPUT_DIR");
assert.equal(process.version, "v24.22.1", "frozen source tool version");
const source = resolve(sourceArg);
const output = resolve(outputArg);
assert(relative(source, output).startsWith(".."), "output must be outside the pinned source checkout");
const runner = fileURLToPath(import.meta.url);
const run = promisify(execFile);
const git = async (...args: string[]) => (await run("git", ["-C", source, ...args])).stdout.trim();
const sha256 = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const sourceHead = "7add4762fe8174d76f43b863375ea39b73121e76";
assert.equal(await git("rev-parse", "HEAD"), sourceHead);
assert.equal(await git("status", "--porcelain"), "");

// The existing M1 source oracle owns the pinned test-only loader and lockfile.
const importIndex = process.execArgv.indexOf("--import");
assert(importIndex >= 0 && process.execArgv[importIndex + 1], "explicit tsx loader required");
const loader = process.execArgv[importIndex + 1]!;
const loaderPath = loader.startsWith("file:") ? fileURLToPath(loader) : resolve(loader);
const tsxPackageBytes = await readFile(resolve(dirname(loaderPath), "../package.json"));
const tsxPackage = JSON.parse(tsxPackageBytes.toString("utf8"));
assert.equal(tsxPackage.name, "tsx");
assert.equal(tsxPackage.version, "4.23.15");
const dependencyLock = await readFile(resolve(dirname(runner), "../openfilm-finite-v1/package-lock.json"));
const loaderLock = await readFile(resolve(dirname(loaderPath), "../../../package-lock.json"));
assert.equal(sha256(loaderLock), sha256(dependencyLock), "loader must use the frozen M1 dependency lock");
assert.equal(sha256(dependencyLock), "6697b505f83230df597922faea37a88535458366ca554a7cb52ec3cdc3b9e535");

const sourceFiles = [
  ["packages/core/src/models.ts", "58dbe93f132577842b7971e0d8aad61aff1d78c6"],
  ["packages/core/src/validation.ts", "ba60475333f60fd9f48c77df2e34f0e7cac8536f"],
  ["packages/solver/src/index.ts", "c701b985495c8c76292880b5acbc9c062b03006c"],
  ["packages/story/src/index.ts", "da4d47ce45772d402db57b11659cc71f1b71e946"],
];
for (const [path, blob] of sourceFiles) {
  assert.equal(await git("rev-parse", `HEAD:${path}`), blob);
  assert.equal(await git("hash-object", resolve(source, path!)), blob);
}
const { compose } = await import(pathToFileURL(`${source}/packages/solver/src/index.ts`).href);

const time = (numerator: number, denominator = 1) => ({ numerator: String(numerator), denominator: String(denominator) });
const candidate = (asset_id: string, n: number, d = 1, score = 0) => ({
  asset_id, content_hash: sha256(`synthetic-declared/${asset_id}`), duration: time(n, d),
  source_range: { start: time(0), end: time(n, d) }, score,
});
const media = (id: string, patch = {}) => ({
  id, name: `Synthetic ${id}`, uri: `synthetic://fixed-window/${id}`,
  mediaType: "image", tags: [], state: {}, metadata: {}, ...patch,
});
const build = (id: string, candidates: any[], assets: any[], target: any, maximum: any,
  selected: string[] = [], constraints: any[] = [], preferred: string[] = []) => {
  const preferences = { preferred_asset_ids: preferred, target_duration: target };
  const input = {
    schema_version: 1,
    brief: { schema_version: 1, brief_id: id, intent: "Synthetic fixed-window proposal",
      assets: assets.map(a => ({ id: a.id })), manual_selected_asset_ids: selected,
      hard_constraints: constraints, preferences },
    candidates, max_duration: maximum, max_shots: 8,
  };
  const seconds = (t: any) => Number(t.numerator) / Number(t.denominator);
  const story = { id, title: "Synthetic story", targetDuration: seconds(target),
    maxDuration: seconds(maximum), beats: [{ id: "beat", title: "Sequence",
      selectedAssetIds: selected, constraints: constraints.map(c => ({ type: c.type, assetIds: c.asset_ids })) }] };
  return { id, input, sourceInput: { story, assets } };
};
const ok = (selectedAssetIds: string[], total: any, targetStatus: string,
  contentClockRate = "1", skippedCandidates: any[] = []) =>
  ({ outcome: "ok", selectedAssetIds, totalDuration: total, targetStatus,
    contentClockRate, skippedCandidates });
const cases = [
  { ...build("one-image", [candidate("a", 4)], [media("a")], time(4), time(4)),
    expectedNative: ok(["a"], time(4), "met"), sourceIds: ["a"], sourceDuration: 4, classification: "preserved-selected-order-and-duration" },
  { ...build("manual-order", [candidate("a", 3), candidate("b", 3)], [media("a"), media("b")], time(6), time(6), ["b", "a"]),
    expectedNative: ok(["b", "a"], time(6), "met"), sourceIds: ["b", "a"], sourceDuration: 6, classification: "preserved-manual-order-and-duration" },
  { ...build("exclude-high-score", [candidate("a", 4, 1, 0), candidate("b", 4, 1, 10)], [media("a", { rating: 0 }), media("b", { rating: 5 })], time(4), time(4), [], [{ type: "must-exclude", asset_ids: ["b"] }]),
    expectedNative: ok(["a"], time(4), "met", "1", [{ assetId: "b", reason: "excluded" }]), sourceIds: ["a"], sourceDuration: 4, classification: "preserved-hard-exclusion" },
  { ...build("required-above-soft-target", [candidate("a", 1), candidate("b", 1)], [media("a"), media("b")], time(1), time(3), ["a", "b"]),
    expectedNative: ok(["a", "b"], time(2), "above"), sourceIds: ["a", "b"], sourceDuration: 2, classification: "preserved-required-membership-over-soft-target" },
  { ...build("required-hard-maximum", [candidate("a", 1), candidate("b", 1)], [media("a"), media("b")], time(1), time(1), ["a", "b"]),
    expectedNative: { outcome: "error", code: "planner.required_duration_exceeded" }, sourceError: "above the story maximum", classification: "preserved-hard-duration-conflict" },
  { ...build("required-short-video", [candidate("v", 1, 2)], [media("v", { mediaType: "video", duration: 0.5 })], time(1, 2), time(1, 2), ["v"]),
    expectedNative: ok(["v"], time(1, 2), "met", "2"), sourceIds: ["v"], sourceDuration: 0.5, classification: "preserved-short-source-bound" },
  { ...build("soft-target-shortfall", [candidate("v", 1, 2)], [media("v", { mediaType: "video", duration: 0.5 })], time(4), time(4)),
    expectedNative: ok(["v"], time(1, 2), "below", "2"), sourceIds: ["v"], sourceDuration: 0.5, classification: "preserved-soft-target-shortfall" },
  { ...build("all-excluded", [candidate("a", 4)], [media("a")], time(4), time(4), [], [{ type: "must-exclude", asset_ids: ["a"] }]),
    expectedNative: { outcome: "error", code: "planner.no_selection" }, sourceIds: [], sourceDuration: 0, classification: "intentional-native-nonempty-plan-refusal" },
  { ...build("declared-score", [candidate("a", 4, 1, 0), candidate("b", 4, 1, 10)], [media("a", { rating: 0 }), media("b", { rating: 5 })], time(4), time(4)),
    expectedNative: ok(["b"], time(4), "met", "1", [{ assetId: "a", reason: "not-needed" }]), sourceIds: ["b"], sourceDuration: 4, classification: "paired-ranking-example-not-score-factor-parity" },
  { ...build("ascii-tie", [candidate("b", 4, 1, 10), candidate("a", 4, 1, 10)], [media("b"), media("a")], time(4), time(4)),
    expectedNative: ok(["a"], time(4), "met", "1", [{ assetId: "b", reason: "not-needed" }]), sourceIds: ["a"], sourceDuration: 4, classification: "lowercase-ascii-example-only-not-locale-parity" },
  { ...build("skip-fixed-window-too-long", [candidate("a", 4, 1, 100), candidate("b", 1, 1, 0)], [media("a", { rating: 5 }), media("b", { mediaType: "video", duration: 1, rating: 0 })], time(1), time(1)),
    expectedNative: ok(["b"], time(1), "met", "1", [{ assetId: "a", reason: "duration-limit" }]), sourceIds: ["a"], sourceDuration: 1, classification: "intentional-fixed-window-no-shortening-selection-delta" },
  { ...build("explicit-preference-over-score", [candidate("a", 4, 1, 0), candidate("b", 4, 1, 100)], [media("a", { rating: 0 }), media("b", { rating: 5 })], time(4), time(4), [], [], ["a"]),
    expectedNative: ok(["a"], time(4), "met", "1", [{ assetId: "b", reason: "not-needed" }]), sourceIds: ["b"], sourceDuration: 4, classification: "intentional-declared-preferred-id-ranking-delta" },
];

// Remove only random generated object IDs. Keep all semantic IDs, facts and order.
function normalize(composition: any) {
  const result = structuredClone(composition);
  delete result.id;
  for (const track of result.tracks) {
    delete track.id;
    for (const clip of track.clips) delete clip.id;
  }
  return result;
}

await mkdir(resolve(output, "inputs"), { recursive: true });
const records = [];
for (const entry of cases) {
  const inputBytes = JSON.stringify(entry.input) + "\n";
  await writeFile(resolve(output, `inputs/${entry.id}.json`), inputBytes);
  const sourceInputBytes = JSON.stringify(entry.sourceInput);
  let observed;
  try {
    const composition = compose(entry.sourceInput.story, entry.sourceInput.assets);
    assert(!entry.sourceError, `${entry.id}: expected a source conflict`);
    const normalized = normalize(composition);
    const assetIds = normalized.tracks.flatMap((t: any) => t.clips.map((c: any) => c.assetId));
    assert.deepEqual(assetIds, entry.sourceIds, `${entry.id}: source selection/order drift`);
    assert.equal(normalized.duration, entry.sourceDuration, `${entry.id}: source timing drift`);
    observed = { outcome: "ok", normalizedComposition: normalized,
      normalizedOutputSha256: sha256(JSON.stringify(normalized)) };
  } catch (error: any) {
    assert(entry.sourceError, `${entry.id}: unexpected source failure ${error}`);
    assert.equal(error.name, "CompositionConstraintError");
    assert(error.message.includes(entry.sourceError));
    observed = { outcome: "error", name: error.name, message: error.message,
      normalizedOutputSha256: sha256(JSON.stringify({ name: error.name, message: error.message })) };
  }
  assert.equal(JSON.stringify(entry.sourceInput), sourceInputBytes, `${entry.id}: source mutated input`);
  records.push({ id: entry.id, inputPath: `inputs/${entry.id}.json`,
    inputSha256: sha256(inputBytes), sourceInputSha256: sha256(sourceInputBytes),
    sourceInput: entry.sourceInput, inputUnchanged: true,
    classification: entry.classification, expectedNative: entry.expectedNative, observedSource: observed });
}
assert.equal(await git("status", "--porcelain"), "");
const evidence = {
  schemaVersion: 1, scope: "fixed-window-selected-source-policy-oracle", sourceHead,
  nodeVersion: process.version, tsxVersion: tsxPackage.version,
  nodeBinarySha256: sha256(await readFile(process.execPath)),
  tsxLoaderSha256: sha256(await readFile(loaderPath)),
  tsxPackageSha256: sha256(tsxPackageBytes),
  dependencyLockPath: "tests/oracles/openfilm-finite-v1/package-lock.json",
  dependencyLockSha256: sha256(dependencyLock),
  runnerSha256: sha256(await readFile(runner)),
  sourceFiles: sourceFiles.map(([path, blob]) => ({ path, blob })),
  normalization: "Delete only generated composition.id, tracks[*].id and tracks[*].clips[*].id; preserve storyId/beatId/assetId, ranges, tracks and all array order. Error output is name/message only.",
  cases: records, allPassed: true,
  intentionalDeltas: [
    "Native fixed windows are never shortened; upstream compose allocates preferred/minimum source lengths.",
    "Native preferred-ID list/declared i32 score/ASCII tie policy is not upstream scoring/capture/locale parity.",
    "Native deterministic shot IDs replace random source object IDs; exact rational planning clock replaces floating seconds.",
    "Native refuses empty selection; upstream can return an empty composition.",
  ],
  notClaimed: ["Rust comparison before execution", "full upstream test suite", "multi-beat/coverage/pins/graph/preset solver parity", "global optimality/feasibility", "media/source/host verification", "release or editorial acceptance"],
};
await writeFile(resolve(output, "source-evidence.json"), JSON.stringify(evidence, null, 2) + "\n");
console.log(JSON.stringify({ ok: true, cases: records.length, sourceHead }));
