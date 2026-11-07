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
    "storytelling-core": {"serde", "serde_json", "sha2", "ed25519-dalek"},
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
    if name == "storytelling-core" and data.get("dependencies", {}).get("sha2") != {"workspace": True}:
        errors.append("Core SHA-256 must inherit only the reviewed workspace configuration")
if manifest["workspace"]["package"].get("publish") is not False:
    errors.append("Bootstrap packages must not be publishable")
if manifest["workspace"]["dependencies"].get("sha2") != {
        "version": "=0.10.9", "default-features": False, "features": ["force-soft"]}:
    errors.append("M1-03 SHA-256 dependency differs from reviewed pure-buffer contract")
if manifest["workspace"]["dependencies"].get("ed25519-dalek") != {
        "version": "=3.0.0", "default-features": False, "features": ["fast"]}:
    errors.append("PR #28 approval-candidate Ed25519 dependency differs from the reviewed verification-only pin")
core_dependencies = tomllib.loads((ROOT / "crates/storytelling-core/Cargo.toml").read_text()).get("dependencies", {})
for dependency in ("serde_json", "sha2", "ed25519-dalek"):
    if core_dependencies.get(dependency) != {"workspace": True}:
        errors.append(f"Core {dependency} must inherit its exact reviewed workspace configuration")
approval_source = ROOT / "crates/storytelling-core/src/approval.rs"
if approval_source.is_file() and re.search(r"\b(?:SigningKey|SecretKey|Signer|OsRng)\b", approval_source.read_text()):
    errors.append("PR #28 approval-candidate core must not expose signing, secret-key, or RNG APIs")
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
    admitted = set()
    records = sorted((ROOT / "docs/migration").glob("provenance-*.json"))
    if not records:
        found.append("Transfer provenance manifests missing")
    for manifest_path in records:
        record = json.loads(manifest_path.read_text())
        if record["source"]["commit"] != "7add4762fe8174d76f43b863375ea39b73121e76":
            found.append(f"Source changed without profile/evidence refresh: {manifest_path.name}")
        origins = {entry["path"] for entry in record["source"]["files"]}
        for entry in record["source"]["files"]:
            if not re.fullmatch(r"[a-f0-9]{40}", entry["blob"]):
                found.append(f"Source blob identity missing: {manifest_path.name}")
        targets = [entry["path"] for entry in record["files"]]
        if len(targets) != len(set(targets)):
            found.append(f"Duplicate provenance target: {manifest_path.name}")
        admitted.update(targets)
        for entry in record["files"]:
            if (not (ROOT / entry["path"]).is_file() or entry["license"] != "Apache-2.0"
                    or not entry["source_paths"] or not set(entry["source_paths"]) <= origins):
                found.append(f"Incomplete provenance: {entry['path']}")
        for notice in record["notices"]:
            value = (ROOT / notice["path"]).read_bytes()
            digest = hashlib.sha1(f"blob {len(value)}\0".encode() + value).hexdigest()
            if digest != notice["blob"]:
                found.append(f"Retained upstream notice changed: {notice['path']}")
    actual = set()
    ignored_parts = {"__pycache__", "node_modules", ".venv"}
    for path in (ROOT / "crates").rglob("*.rs"):
        if "SPDX-License-Identifier: Apache-2.0" in path.read_text():
            actual.add(str(path.relative_to(ROOT)))
    # JSON has no comments. A directory README declares the component license
    # without changing exact fixture bytes; original M0 MIT fixtures stay separate.
    directories = []
    for parent in [ROOT / "tests/fixtures", ROOT / "tests/oracles"]:
        for readme in parent.rglob("README.md"):
            if ignored_parts.intersection(readme.parts):
                continue
            if "SPDX-License-Identifier: Apache-2.0" in readme.read_text():
                directories.append(readme.parent)
    for directory in directories:
        for path in directory.rglob("*"):
            if not path.is_file() or ignored_parts.intersection(path.parts):
                continue
            actual.add(str(path.relative_to(ROOT)))
    if actual != admitted:
        found.append(f"Transfer provenance coverage differs: {sorted(actual ^ admitted)}")
    for member in ("storytelling-core", "storytelling-engine"):
        package = tomllib.loads((ROOT / "crates" / member / "Cargo.toml").read_text())
        if package["package"].get("license") != "MIT AND Apache-2.0":
            found.append(f"Mixed component license missing: {member}")
        for path in (ROOT / "crates" / member / "src").rglob("*.rs"):
            if re.search(r"\bstd::(?:fs|net|process|io)\b", path.read_text()):
                found.append(f"I/O introduced into pure crate: {path.relative_to(ROOT)}")
    return found


errors.extend(provenance_manifest_covers_every_imported_file())
for work_path in sorted((ROOT / "docs/migration").glob("*-work-index.json")):
    work = json.loads(work_path.read_text())
    task_ids = [task["id"] for task in work["tasks"]]
    if len(task_ids) != len(set(task_ids)):
        errors.append(f"Duplicate work ID: {work_path.name}")
    completed = set()
    for task in work["tasks"]:
        if not set(task["depends_on"]) <= completed:
            errors.append(f"Missing, cyclic or unordered task dependency: {task['id']}")
        completed.add(task["id"])
    for requirement in work["requirements"]:
        if requirement["id"] not in requirements:
            errors.append(f"Requirement is undocumented: {requirement['id']}")
        for name in requirement.get("tests", []):
            if f"fn {name}(" not in tests:
                errors.append(f"Missing traceable test: {name}")
        for path in requirement.get("verification_paths", []):
            if not (ROOT / path).is_file():
                errors.append(f"Missing verification artifact: {path}")
if errors:
    print("\n".join(errors), file=sys.stderr)
    raise SystemExit(1)
print("Repository links, provenance, dependencies and requirement test references passed.")
