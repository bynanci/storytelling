// SPDX-License-Identifier: Apache-2.0
// Independently authored synthetic policy oracle; no upstream test or media copied.
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { promisify } from "node:util";
import { pathToFileURL, fileURLToPath } from "node:url";

const [sourceArg, outputArg] = process.argv.slice(2);
assert(sourceArg && outputArg, "Usage: run-source.mts SOURCE_CHECKOUT OUTPUT_DIR");
const source = resolve(sourceArg);
const output = resolve(outputArg);
const run = promisify(execFile);
const git = async (...args: string[]) => (await run("git", ["-C", source, ...args])).stdout.trim();
const expectedHead = "7add4762fe8174d76f43b863375ea39b73121e76";
assert.equal(await git("rev-parse", "HEAD"), expectedHead);
assert.equal(await git("status", "--porcelain", "--untracked-files=no"), "");
const { compose } = await import(pathToFileURL(`${source}/packages/solver/src/index.ts`).href);
const sha256 = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const assets = ["a", "b", "c"].map(id => ({
  id, uri: `synthetic://brief/${id}.jpg`, mediaType: "image", name: `Synthetic ${id}`,
  tags: [], state: {}, metadata: {}, dimensions: { width: 1920, height: 1080 },
}));
const brief = (id: string, selected: string[], constraints: any[], preferred: string[] = []) => ({
  schema_version: 1, brief_id: id, intent: "A synthetic declared brief",
  assets: assets.map(a => ({ id: a.id })), manual_selected_asset_ids: selected,
  hard_constraints: constraints,
  preferences: { preferred_asset_ids: preferred, target_duration: { numerator: "4", denominator: "1" } },
});
const cases = [
  { id: "manual-outside-candidates", brief: brief("outside", ["b"], [{ type: "must-include", asset_ids: ["a"] }]), candidates: ["a"], required: ["b", "a"] },
  { id: "manual-order", brief: brief("manual-order", ["b", "a"], []), required: ["b", "a"], order: ["b", "a"] },
  { id: "order-implies-membership", brief: brief("order-members", [], [{ type: "asset-order", asset_ids: ["c", "a"] }]), required: ["c", "a"], order: ["c", "a"] },
  { id: "manual-excluded", brief: brief("manual-excluded", ["a"], [{ type: "must-exclude", asset_ids: ["a"] }]), conflict: "required-excluded", message: "both includes and excludes" },
  { id: "include-excluded", brief: brief("include-excluded", [], [{ type: "must-include", asset_ids: ["a"] }, { type: "must-exclude", asset_ids: ["a"] }]), conflict: "required-excluded", message: "both includes and excludes" },
  { id: "missing-manual", brief: brief("missing-manual", ["missing"], []), conflict: "unknown-asset", message: "references missing asset" },
  { id: "missing-excluded", brief: brief("missing-excluded", [], [{ type: "must-exclude", asset_ids: ["missing"] }]), conflict: "unknown-asset", message: "references missing asset" },
  { id: "order-cycle", brief: brief("order-cycle", [], [{ type: "asset-order", asset_ids: ["a", "b"] }, { type: "asset-order", asset_ids: ["b", "a"] }]), conflict: "order-cycle", message: "conflicting asset orders" },
  { id: "manual-order-cycle", brief: brief("manual-cycle", ["b", "a"], [{ type: "asset-order", asset_ids: ["a", "b"] }]), conflict: "order-cycle", message: "conflicting asset orders" },
  { id: "excluded-preference", brief: brief("excluded-pref", [], [{ type: "must-exclude", asset_ids: ["a"] }], ["a"]), excluded: ["a"], warning: "excluded-preference" },
];
await mkdir(`${output}/inputs`, { recursive: true });
const records = [];
for (const entry of cases) {
  const beat = {
    id: "beat", title: "Synthetic beat", minDuration: 0, targetDuration: 4, maxDuration: 12,
    selectedAssetIds: entry.brief.manual_selected_asset_ids,
    candidateAssetIds: entry.candidates,
    constraints: entry.brief.hard_constraints.map(c => ({ type: c.type, assetIds: c.asset_ids })),
  };
  const story = { id: entry.brief.brief_id, title: "Synthetic story", targetDuration: 4, maxDuration: 12, beats: [beat] };
  const sourceInput = { story, assets };
  const before = JSON.stringify(sourceInput);
  const briefBytes = JSON.stringify(entry.brief);
  await writeFile(`${output}/inputs/${entry.id}.json`, briefBytes + "\n");
  let result;
  try {
    const composition = compose(story, assets);
    const ids = composition.tracks.flatMap((track: any) => track.clips.map((clip: any) => clip.assetId));
    assert(!entry.conflict, `${entry.id} should reject in upstream`);
    for (const id of entry.required ?? []) assert(ids.includes(id), `source dropped ${id}`);
    if (entry.order) assert(ids.indexOf(entry.order[0]) < ids.indexOf(entry.order[1]), "source order changed");
    for (const id of entry.excluded ?? []) assert(!ids.includes(id), "source relaxed an exclusion");
    result = { outcome: "consistent-selected-policy", sourceAssetIds: ids };
  } catch (error: any) {
    assert(entry.conflict, `${entry.id}: unexpected upstream failure ${error}`);
    assert.equal(error.name, "CompositionConstraintError");
    assert(error.message.includes(entry.message));
    result = { outcome: "hard-conflict", conflictKind: entry.conflict, sourceError: error.name, sourceMessage: error.message };
  }
  assert.equal(JSON.stringify(sourceInput), before, "source mutated input declarations");
  records.push({
    id: entry.id, briefSha256: sha256(briefBytes + "\n"), sourceInputSha256: sha256(before),
    expectedRequired: entry.required ?? [], expectedOrder: entry.order ?? [],
    expectedExcluded: entry.excluded ?? [], expectedWarning: entry.warning,
    inputUnchanged: true, ...result,
  });
}
assert.equal(await git("status", "--porcelain", "--untracked-files=no"), "");
const evidence = {
  schemaVersion: 1, scope: "selected-source-constraint-policy-only", sourceHead: expectedHead,
  nodeVersion: process.version, runnerSha256: sha256(await readFile(fileURLToPath(import.meta.url))),
  sourceFiles: [
    { path: "packages/core/src/models.ts", blob: "58dbe93f132577842b7971e0d8aad61aff1d78c6" },
    { path: "packages/core/src/validation.ts", blob: "ba60475333f60fd9f48c77df2e34f0e7cac8536f" },
    { path: "packages/solver/src/index.ts", blob: "c701b985495c8c76292880b5acbc9c062b03006c" },
  ],
  cases: records, allPassed: true,
  intentionalDelta: "New StoryBrief schema, typed diagnostics and soft preferences are downstream design; no full compose/feasibility parity claim.",
  notClaimed: ["Rust comparison before its run", "full upstream suite", "duration feasibility", "selection planning", "source/media/host verification"],
};
await writeFile(`${output}/source-evidence.json`, JSON.stringify(evidence, null, 2) + "\n");
console.log(JSON.stringify({ ok: true, cases: records.length, sourceHead: expectedHead }));
