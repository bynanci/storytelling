#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Verify R1 raw bytes; refresh the manifest only from staged Git blobs."""
import argparse
import hashlib
import re
import subprocess
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = "docs/research/evaluations/2026-11-13-gemini-r1"
MANIFEST = "MANIFEST.sha256"


def git(root, *args):
    return subprocess.run(
        ["git", "-C", str(root), *args], check=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    ).stdout


def package_blobs(root, ref=None):
    """Read exact object bytes, with no checkout/clean/newline conversion."""
    objects = {}
    if ref is None:
        records = git(root, "ls-files", "--stage", "-z", "--", PACKAGE)
        for entry in records.split(b"\0"):
            if not entry:
                continue
            metadata, path = entry.split(b"\t", 1)
            mode, oid, stage = metadata.decode("ascii").split()
            if stage != "0":
                raise ValueError("Resolve index conflicts before refreshing the manifest")
            objects[path.decode("utf-8")] = (mode, oid)
    else:
        if ref.startswith("-"):
            raise ValueError("Invalid Git revision")
        commit = git(root, "rev-parse", "--verify", ref + "^{commit}").decode().strip()
        records = git(root, "ls-tree", "-r", "-z", commit, "--", PACKAGE)
        for entry in records.split(b"\0"):
            if not entry:
                continue
            metadata, path = entry.split(b"\t", 1)
            mode, kind, oid = metadata.decode("ascii").split()
            if kind != "blob":
                raise ValueError("Package entries must be regular Git blobs")
            objects[path.decode("utf-8")] = (mode, oid)
    blobs = {}
    for path, (mode, oid) in objects.items():
        if mode not in ("100644", "100755"):
            raise ValueError("Package entries must be regular files")
        name = str(PurePosixPath(path).relative_to(PACKAGE))
        blobs[name] = git(root, "cat-file", "blob", oid)
    if MANIFEST not in blobs:
        raise ValueError("Tracked R1 manifest is missing")
    return blobs


def parse_manifest(raw):
    if b"\r" in raw or not raw.endswith(b"\n"):
        raise ValueError("Manifest must use LF and a final newline")
    entries = {}
    for line in raw.decode("utf-8").splitlines():
        match = re.fullmatch(r"([0-9a-f]{64})  (.+)", line)
        if not match:
            raise ValueError("Malformed SHA-256 manifest row")
        digest, name = match.groups()
        path = PurePosixPath(name)
        if path.is_absolute() or ".." in path.parts or str(path) != name or "\\" in name:
            raise ValueError("Manifest paths must be canonical package-relative paths")
        if name in entries or name == MANIFEST:
            raise ValueError("Duplicate or self-referential manifest entry")
        entries[name] = digest
    if list(entries) != sorted(entries):
        raise ValueError("Manifest entries must be sorted")
    return entries


def render_manifest(blobs):
    rows = []
    for name, raw in sorted(blobs.items()):
        if name == MANIFEST:
            continue
        if b"\r" in raw:
            raise ValueError(f"Staged package file is not LF: {name}")
        rows.append(f"{hashlib.sha256(raw).hexdigest()}  {name}\n")
    return "".join(rows).encode("utf-8")


def verify(blobs):
    entries = parse_manifest(blobs[MANIFEST])
    paths = set(blobs) - {MANIFEST}
    if set(entries) != paths:
        raise ValueError("Manifest coverage differs from tracked package files")
    failures = [name for name in entries if hashlib.sha256(blobs[name]).hexdigest() != entries[name]]
    if failures:
        raise ValueError("SHA-256 mismatch: " + ", ".join(failures))
    for name, raw in blobs.items():
        if b"\r" in raw:
            raise ValueError(f"Package file is not LF: {name}")
        if name.endswith(".csv") and (not raw.startswith(b"\xef\xbb\xbf") or not raw.endswith(b"\n")):
            raise ValueError(f"CSV must retain UTF-8 BOM and final LF: {name}")
    return len(entries)


def verify_worktree(root, committed):
    working = {name: (root / PACKAGE / name).read_bytes() for name in committed}
    for name in committed:
        if working[name] != committed[name]:
            raise ValueError(f"Checkout bytes differ from Git blob: {name}")
    return verify(working)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    check = commands.add_parser("verify")
    check.add_argument("--ref", default="HEAD")
    check.add_argument("--worktree", action="store_true")
    refresh = commands.add_parser("refresh")
    refresh.add_argument("--staged", action="store_true", required=True)
    args = parser.parse_args()
    try:
        if args.command == "refresh":
            blobs = package_blobs(ROOT)
            manifest = render_manifest(blobs)
            verify({**blobs, MANIFEST: manifest})
            (ROOT / PACKAGE / MANIFEST).write_bytes(manifest)
            print("Refreshed from staged Git blobs; stage MANIFEST.sha256, commit, then verify the commit/checkout.")
        else:
            blobs = package_blobs(ROOT, args.ref)
            count = verify(blobs)
            if args.worktree:
                verify_worktree(ROOT, blobs)
            print(f"Verified {count} manifest entries against Git blobs" + (" and checkout bytes." if args.worktree else "."))
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"Checksum verification failed: {error}\n")


if __name__ == "__main__":
    main()
