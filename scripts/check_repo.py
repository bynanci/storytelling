#!/usr/bin/env python3
"""Check local documentation links, test traceability and M0 boundaries (stdlib only)."""
import hashlib
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
errors = []
manifest = tomllib.loads((ROOT / "Cargo.toml").read_text())
expected = {
    "storytelling-core": {"serde"},
    "storytelling-engine": {"serde", "serde_json", "storytelling-core"},
    "storytelling-cli": {"serde_json", "storytelling-engine"},
}
members = manifest["workspace"]["members"]
if {Path(member).name for member in members} != set(expected):
    errors.append("M0 workspace must contain exactly the three implemented crates")
for member in members:
    data = tomllib.loads((ROOT / member / "Cargo.toml").read_text())
    name = data["package"]["name"]
    if set(data.get("dependencies", {})) != expected.get(name, set()):
        errors.append(f"Unexpected dependency boundary for {name}")
if manifest["workspace"]["package"].get("publish") is not False:
    errors.append("Bootstrap packages must not be publishable")
license_bytes = (ROOT / "LICENSE").read_bytes()
blob = hashlib.sha1(f"blob {len(license_bytes)}\0".encode() + license_bytes).hexdigest()
if blob != "f317799529d13f4d9301daa2dba63dfea559ff69":
    errors.append("Original MIT LICENSE changed without updating the approved provenance contract")
for doc in list(ROOT.glob("*.md")) + list((ROOT / "docs").rglob("*.md")):
    for target in re.findall(r"\[[^\]]+\]\(([^\s)]+)\)", doc.read_text()):
        if "://" in target or target.startswith(("#", "mailto:")):
            continue
        path = target.split("#", 1)[0]
        if path and not (doc.parent / path).resolve().exists():
            errors.append(f"Broken link: {doc.relative_to(ROOT)} -> {target}")
tests = "\n".join(p.read_text() for p in (ROOT / "crates").rglob("*.rs"))
requirements = (ROOT / "docs/REQUIREMENTS.md").read_text()
for name in re.findall(r"`([a-z][a-z0-9_]+)`", requirements):
    if name.startswith(("accepts_", "rejects_", "compares_", "distinguishes_", "reports_")):
        if f"fn {name}(" not in tests:
            errors.append(f"Missing traceable test: {name}")
for sample in [ROOT / "examples/minimal-plan.json", ROOT / "tests/fixtures/invalid-source-range.json"]:
    json.loads(sample.read_text())


def provenance_manifest_covers_every_imported_file():
    """Admit source-informed files only with fixed origin and retained notices."""
    found = []
    record = json.loads((ROOT / "docs/migration/provenance-m1-01.json").read_text())
    if record["source"]["commit"] != "7add4762fe8174d76f43b863375ea39b73121e76":
        found.append("M1 source changed without a profile/evidence refresh")
    origins = {entry["path"] for entry in record["source"]["files"]}
    for entry in record["source"]["files"]:
        if not re.fullmatch(r"[a-f0-9]{40}", entry["blob"]):
            found.append("M1 source blob identity missing")
    admitted = [entry["path"] for entry in record["files"]]
    if len(admitted) != len(set(admitted)):
        found.append("Duplicate M1 provenance target")
    for entry in record["files"]:
        if (not (ROOT / entry["path"]).is_file() or entry["license"] != "Apache-2.0"
                or not entry["source_paths"] or not set(entry["source_paths"]) <= origins):
            found.append(f"Incomplete M1 provenance: {entry['path']}")
    actual = set()
    for directory in [ROOT / "crates", ROOT / "tests/fixtures/openfilm/finite-v1",
                      ROOT / "tests/oracles/openfilm-finite-v1"]:
        for path in directory.rglob("*"):
            if not path.is_file() or {"__pycache__", "node_modules"}.intersection(path.parts):
                continue
            if directory == ROOT / "crates":
                if path.suffix != ".rs" or "SPDX-License-Identifier: Apache-2.0" not in path.read_text():
                    continue
            actual.add(str(path.relative_to(ROOT)))
    if actual != set(admitted):
        found.append(f"M1 provenance coverage differs: {sorted(actual ^ set(admitted))}")
    for notice in record["notices"]:
        value = (ROOT / notice["path"]).read_bytes()
        digest = hashlib.sha1(f"blob {len(value)}\0".encode() + value).hexdigest()
        if digest != notice["blob"]:
            found.append(f"Retained upstream notice changed: {notice['path']}")
    for member in ("storytelling-core", "storytelling-engine"):
        package = tomllib.loads((ROOT / "crates" / member / "Cargo.toml").read_text())
        if package["package"].get("license") != "MIT AND Apache-2.0":
            found.append(f"Mixed component license missing: {member}")
        for path in (ROOT / "crates" / member / "src").rglob("*.rs"):
            if re.search(r"\bstd::(?:fs|net|process|io)\b", path.read_text()):
                found.append(f"I/O introduced into pure crate: {path.relative_to(ROOT)}")
    return found


errors.extend(provenance_manifest_covers_every_imported_file())
work = json.loads((ROOT / "docs/migration/m1-01-work-index.json").read_text())
task_ids = [task["id"] for task in work["tasks"]]
if len(task_ids) != len(set(task_ids)):
    errors.append("Duplicate M1 work ID")
completed = set()
for task in work["tasks"]:
    if not set(task["depends_on"]) <= completed:
        errors.append(f"Missing, cyclic or unordered M1 dependency: {task['id']}")
    completed.add(task["id"])
for requirement in work["requirements"]:
    if requirement["id"] not in requirements:
        errors.append(f"M1 requirement is undocumented: {requirement['id']}")
    for name in requirement["tests"]:
        if f"fn {name}(" not in tests:
            errors.append(f"Missing M1 traceable test: {name}")
if errors:
    print("\n".join(errors), file=sys.stderr)
    raise SystemExit(1)
print("Repository links, provenance, dependencies and requirement test references passed.")
