<!-- SPDX-License-Identifier: MIT -->
# Recheck the proposed packet

This checks schemas, authored example bytes/structural labels, source identities
and planned-case inventory. It does not run a transport or predict diagnostic
precedence, process behavior or domain equivalence. Run from the repository root
with Python 3 and the already installed `jsonschema` package; a missing package
is blocked static verification, not success. No network resolver or product
runtime is used.

```sh
python3 - <<'PY'
from pathlib import Path
from jsonschema import Draft202012Validator
import hashlib, json, struct, subprocess
root = Path("docs/proposals/m4-02-readonly-transport")
validators = {}
for kind, name in [
    ("request", "request-header.schema.json"),
    ("response", "response-header.schema.json"),
    ("narrative", "narrative-projection-view.schema.json"),
]:
    schema = json.loads((root / name).read_bytes())
    Draft202012Validator.check_schema(schema)
    validators[kind] = Draft202012Validator(schema)
manifest = json.loads((root / "examples/manifest.json").read_bytes())
assert manifest["stage"] == "authored_G2_proposed_examples_not_runtime_receipts"
assert len(manifest["files"]) == 21
labels = 0
for item in manifest["files"]:
    raw = (root / "examples" / item["path"]).read_bytes()
    assert len(raw) == item["bytes"], item["path"]
    assert hashlib.sha256(raw).hexdigest() == item["sha256"], item["path"]
    if item["schema_valid"] is None:
        continue  # corrupt/duplicate/truncated frame: indexed authored vector only
    if item["kind"].endswith("_frame"):
        assert raw[:4] == b"STRO"
        size = struct.unpack(">I", raw[4:8])[0]
        assert 1 <= size <= 16384 and 8 + size <= len(raw)
        header = json.loads(raw[8:8 + size])
        assert 8 + size + sum(b["byte_length"] for b in header["buffers"]) == len(raw)
        kind = "request" if item["kind"].startswith("request") else "response"
    else:
        header = json.loads(raw)
        kind = "narrative" if item["kind"] == "narrative_projection" else item["kind"].split("_")[0]
    assert validators[kind].is_valid(header) is item["schema_valid"], item["path"]
    labels += 1
for kind in ("request", "response"):
    schema = validators[kind].schema
    rid = schema["properties"]["request_id"]
    for bad in ("", "x\n", "x\r", "x\u2028", "x\u2029", "x\x00", "x" * 65, "é"):
        assert not Draft202012Validator(rid).is_valid(bad), repr(bad)
    assert Draft202012Validator(rid).is_valid("a" * 64)
plan = json.loads((root / "cases.json").read_bytes())
assert len(plan["cases"]) == 64
assert len({c["id"] for c in plan["cases"]}) == 64
assert {c["group"] for c in plan["cases"]} == {f"TS-G{i:02}" for i in range(1, 13)}
assert all(c["state"] == "planned_not_executed" for c in plan["cases"])
assert plan["runtime_cases_executed"] == 0
for item in plan["sources"]:
    raw = subprocess.check_output(["git", "cat-file", "blob", plan["base_sha"] + ":" + item["path"]])
    assert raw == Path(item["path"]).read_bytes()
    assert len(raw) == item["bytes"] and hashlib.sha256(raw).hexdigest() == item["sha256"]
assert not subprocess.check_output(["git", "diff", plan["base_sha"], "--", "crates", "contracts", "tests", "examples", "scripts", "skills", ".github", "Cargo.toml", "Cargo.lock", "package.json", "package-lock.json"])
print(json.dumps({"schemas": 3, "indexed_authored_examples": 21, "schema_labels_checked": labels,
                  "source_git_identities": len(plan["sources"]), "planned_cases": 64,
                  "transport_runtime_cases_executed": 0, "human_observations": 0, "host_observations": 0}))
PY
```

The unlabelled corrupt/duplicate/truncated binary vectors are hash-checked only;
future G3 tests must assert their runtime diagnoses. A structurally valid frame
with wrong association is intentionally schema-valid but invalid for its call.
The recorded request-ID counterexample is a G2 schema correction, not product
RED/GREEN. Old source fixtures and receipt bytes are read from their actual Git
blobs and remain unchanged.
