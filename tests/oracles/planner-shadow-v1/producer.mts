// SPDX-License-Identifier: Apache-2.0
// Independently authored source-informed M2-03 live oracle; no upstream code copied.
import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, isAbsolute, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const SOURCE_SHA = "7add4762fe8174d76f43b863375ea39b73121e76";
const MAX_INPUT = 1_048_576;
const MAX_OUTPUT = 4_194_304;
const sha256 = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");

function object(value: unknown, keys: string[]): asserts value is Record<string, any> {
  assert(value !== null && typeof value === "object" && !Array.isArray(value), "expected object");
  const actual = Object.keys(value);
  assert(actual.length === keys.length && keys.every(key => actual.includes(key)), "unexpected object fields");
}

// JSON.parse alone silently discards duplicate properties. Validate syntax and
// decoded property uniqueness first, then use the platform parser for values.
function strictJson(text: string): any {
  let offset = 0;
  const whitespace = () => { while (offset < text.length && /[\x20\x09\x0a\x0d]/.test(text[offset]!)) offset++; };
  const expect = (character: string) => { assert(text[offset] === character, "invalid JSON separator"); offset++; };
  const string = (): string => {
    const start = offset;
    expect('"');
    while (offset < text.length) {
      const character = text[offset++]!;
      if (character === '"') return JSON.parse(text.slice(start, offset));
      assert(character.charCodeAt(0) >= 32, "invalid JSON string");
      if (character === "\\") offset++;
    }
    throw new Error("unterminated JSON string");
  };
  const value = (depth: number): void => {
    assert(depth <= 128, "JSON depth exceeded");
    whitespace();
    if (text[offset] === "{") {
      offset++; whitespace();
      const keys = new Set<string>();
      if (text[offset] === "}") { offset++; return; }
      for (;;) {
        whitespace(); const key = string();
        assert(!keys.has(key), "duplicate JSON property"); keys.add(key);
        whitespace(); expect(":"); value(depth + 1); whitespace();
        if (text[offset] === "}") { offset++; return; }
        expect(",");
      }
    }
    if (text[offset] === "[") {
      offset++; whitespace();
      if (text[offset] === "]") { offset++; return; }
      for (;;) {
        value(depth + 1); whitespace();
        if (text[offset] === "]") { offset++; return; }
        expect(",");
      }
    }
    if (text[offset] === '"') { string(); return; }
    for (const literal of ["true", "false", "null"]) {
      if (text.startsWith(literal, offset)) { offset += literal.length; return; }
    }
    const number = /^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/.exec(text.slice(offset));
    assert(number, "invalid JSON value"); offset += number[0].length;
  };
  value(0); whitespace(); assert(offset === text.length, "trailing JSON content");
  return JSON.parse(text);
}

function normalize(composition: any): any {
  const result = structuredClone(composition);
  assert(result !== null && typeof result === "object" && Array.isArray(result.tracks), "unexpected composition shape");
  delete result.id;
  for (const track of result.tracks) {
    assert(track !== null && typeof track === "object" && Array.isArray(track.clips), "unexpected track shape");
    delete track.id;
    for (const clip of track.clips) delete clip.id;
  }
  return result;
}

async function main(): Promise<void> {
  const args = process.argv.slice(2);
  if (args.length !== 2 || args[0] !== "--source" || !args[1] || !isAbsolute(args[1])) {
    process.stderr.write("Usage: producer.mts --source ABSOLUTE_SOURCE_CHECKOUT\n");
    process.exitCode = 2;
    return;
  }
  const source = resolve(args[1]);
  assert.equal(process.platform, "linux", "Linux producer required");
  assert.equal(process.arch, "x64", "x86_64 producer required");
  assert.equal(process.version, "v24.22.1", "unexpected Node version");
  assert(!process.env.NODE_OPTIONS && !process.env.ESBUILD_BINARY_PATH, "unreviewed Node/esbuild override");
  assert.equal(process.env.TSX_TSCONFIG_PATH, resolve(source, "tsconfig.json"), "explicit pinned tsconfig required");
  const imports = process.execArgv.flatMap((argument, index) => argument === "--import" ? [process.execArgv[index + 1]] : []);
  assert(imports.length === 1 && imports[0], "one explicit tsx loader required");
  const loaderPath = imports[0]!.startsWith("file:") ? fileURLToPath(imports[0]!) : imports[0]!;
  assert(isAbsolute(loaderPath), "absolute tsx loader required");
  const tsxPackagePath = resolve(dirname(loaderPath), "../package.json");
  const tsxPackageBytes = await readFile(tsxPackagePath);
  const tsxPackage = JSON.parse(tsxPackageBytes.toString("utf8"));
  assert.equal(tsxPackage.name, "tsx"); assert.equal(tsxPackage.version, "4.23.15");
  const requireFromTsx = createRequire(tsxPackagePath);
  const esbuildPackagePath = requireFromTsx.resolve("esbuild/package.json");
  const esbuildPackage = JSON.parse((await readFile(esbuildPackagePath)).toString("utf8"));
  assert.equal(esbuildPackage.name, "esbuild"); assert.equal(esbuildPackage.version, "0.28.2");
  const esbuildPath = createRequire(esbuildPackagePath).resolve("@esbuild/linux-x64/bin/esbuild");
  const sourceHead = (await promisify(execFile)("/usr/bin/git", ["-C", source, "rev-parse", "HEAD"], {encoding: "utf8", maxBuffer: 4096})).stdout.trim();
  assert.equal(sourceHead, SOURCE_SHA, "source checkout revision changed");

  const chunks: Uint8Array[] = [];
  let length = 0;
  for await (const chunk of process.stdin) {
    const bytes = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
    length += bytes.length;
    assert(length <= MAX_INPUT, "source batch exceeds input budget");
    chunks.push(bytes);
  }
  const raw = Buffer.concat(chunks);
  const batch = strictJson(new TextDecoder("utf-8", {fatal: true}).decode(raw));
  object(batch, ["schema_version", "cases"]);
  assert.equal(batch.schema_version, 1, "unsupported batch version");
  assert(Array.isArray(batch.cases) && batch.cases.length >= 1 && batch.cases.length <= 144, "source case count exceeded");
  const ids = new Set<string>();
  for (const entry of batch.cases) {
    object(entry, ["id", "source_input"]);
    assert(typeof entry.id === "string" && /^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$/.test(entry.id), "invalid case identity");
    assert(!ids.has(entry.id), "duplicate case identity"); ids.add(entry.id);
    object(entry.source_input, ["story", "assets"]);
    assert(entry.source_input.story !== null && typeof entry.source_input.story === "object" && !Array.isArray(entry.source_input.story), "story must be an object");
    assert(Array.isArray(entry.source_input.assets), "assets must be an array");
  }
  const { compose, CompositionConstraintError } = await import(pathToFileURL(resolve(source, "packages/solver/src/index.ts")).href);
  const results = [];
  for (const entry of batch.cases) {
    const before = JSON.stringify(entry.source_input);
    let result;
    try {
      const composition = compose(entry.source_input.story, entry.source_input.assets);
      result = {id: entry.id, input_unchanged: true, ok: true, composition: normalize(composition)};
    } catch (failure: unknown) {
      if (!(failure instanceof CompositionConstraintError)) throw failure;
      result = {id: entry.id, input_unchanged: true, ok: false, error: {name: failure.name, message: failure.message}};
    }
    assert.equal(JSON.stringify(entry.source_input), before, "source function mutated supplied input");
    results.push(result);
  }
  const output = {
    schema_version: 1,
    source_head: sourceHead,
    batch_sha256: sha256(raw),
    producer_sha256: sha256(await readFile(fileURLToPath(import.meta.url))),
    node_version: process.version,
    node_sha256: sha256(await readFile(process.execPath)),
    tsx_version: tsxPackage.version,
    tsx_loader_sha256: sha256(await readFile(loaderPath)),
    tsx_package_sha256: sha256(tsxPackageBytes),
    esbuild_version: esbuildPackage.version,
    esbuild_sha256: sha256(await readFile(esbuildPath)),
    results,
  };
  const bytes = JSON.stringify(output) + "\n";
  assert(Buffer.byteLength(bytes) <= MAX_OUTPUT, "source results exceed output budget");
  process.stdout.write(bytes);
}

main().catch((failure: unknown) => {
  process.stderr.write(`planner-shadow producer failed: ${failure instanceof Error ? failure.message : String(failure)}\n`);
  process.exitCode = 1;
});
