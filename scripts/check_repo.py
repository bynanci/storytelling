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
if errors:
    print("\n".join(errors), file=sys.stderr)
    raise SystemExit(1)
print("Repository links, provenance, dependencies and requirement test references passed.")
