#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Actual planner -> bound artifact -> official OTIO read/write/read, no host."""
import copy
from fractions import Fraction
import hashlib
import importlib.metadata
import json
from pathlib import Path
import subprocess
import sys
import tempfile

import opentimelineio as otio

ROOT = Path(__file__).resolve().parents[3]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def wire(value):
    return Fraction(int(value["numerator"]), int(value["denominator"]))


def native(value):
    require(value.value.is_integer() and value.rate.is_integer(), "noninteger native clock")
    return Fraction(int(value.value), int(value.rate))


def main():
    binary = Path(sys.argv[1]).resolve(strict=True)
    require(importlib.metadata.version("opentimelineio") == "0.18.1", "wrong OTIO oracle")
    request = json.loads((ROOT / "tests/fixtures/story-review-v1/planner-request.json").read_text())
    ctx = (ROOT / "tests/fixtures/story-review-v1/context.json").read_bytes()
    for case in ("video-trim", "repeat-source"):
        snapshot = (ROOT / f"tests/fixtures/openfilm/finite-v1/{case}.json").read_bytes()
        current_request = copy.deepcopy(request)
        if case == "repeat-source":
            current_request["brief"]["assets"] = [{"id": "reused-photo"}]
            current_request["brief"]["manual_selected_asset_ids"] = ["reused-photo"]
            c = current_request["candidates"][0]
            c.update(asset_id="reused-photo", content_hash="c" * 64,
                     duration={"numerator": "3", "denominator": "1"},
                     source_range={"start": {"numerator": "0", "denominator": "1"},
                                   "end": {"numerator": "2", "denominator": "1"}})
        with tempfile.TemporaryDirectory(prefix="story-review-oracle-") as directory:
            root = Path(directory)

            def run(*args, data=None, exit_code=0):
                result = subprocess.run([str(binary), *args], input=data, capture_output=True,
                                        cwd=root, timeout=30, env={"PATH": directory})
                require(result.returncode == exit_code and not result.stderr, "unexpected CLI outcome")
                return result.stdout

            (root / "snapshot.json").write_bytes(snapshot)
            (root / "context.json").write_bytes(ctx)
            (root / "OpenFilm-Catalog.sqlite").write_bytes(b"synthetic Catalog sentinel")
            (root / "FilmCraft-project.json").write_bytes(b"synthetic project sentinel")
            proposal = run("plan-story", "-", data=json.dumps(current_request).encode())
            (root / "proposal.json").write_bytes(proposal)
            before = {p.name: p.read_bytes() for p in root.iterdir()}
            bundle_bytes = run("prepare-story-review", "proposal.json", "snapshot.json", "context.json")
            require(bundle_bytes == run("prepare-story-review", "-", "snapshot.json", "context.json", data=proposal), "nondeterministic bundle")
            bundle = json.loads(bundle_bytes)
            artifact = run("export-story-otio", "proposal.json", "snapshot.json", "context.json").removesuffix(b"\n")
            require(json.loads(artifact) == bundle["otio"], "different exported/reviewed artifact")
            identity = bundle["review"]["identity"]
            require(hashlib.sha256(artifact).hexdigest() == identity["artifact_sha256"], "artifact hash")
            require(hashlib.sha256(snapshot).hexdigest() == identity["composition_sha256"], "snapshot hash")
            owner = bundle["otio"]["metadata"]["storytelling"]
            require(hashlib.sha256(proposal).hexdigest() == owner["proposal_sha256"], "proposal hash")
            require(owner["selected_proposal_json"].encode() == proposal, "proposal text")
            timeline = otio.adapters.read_from_string(artifact.decode(), adapter_name="otio_json")
            roundtrip = otio.adapters.read_from_string(
                otio.adapters.write_to_string(timeline, adapter_name="otio_json"), adapter_name="otio_json")
            selected = json.loads(proposal)["planning"]["plan"]
            assets = {a["id"]: a for a in json.loads(snapshot)["assets"]}
            for parsed in (timeline, roundtrip):
                require(parsed.name == selected["plan_id"], "selected plan identity")
                clips = list(parsed.find_clips())
                require(len(clips) == len(selected["shots"]), "cut count")
                for clip, shot in zip(clips, selected["shots"]):
                    require(clip.metadata["storytelling"]["occurrence_id"] == shot["id"], "shot order")
                    require(clip.media_reference.target_url == assets[shot["asset_id"]]["uri"], "source reference")
                    require(native(clip.source_range.start_time) == wire(shot["source_range"]["start"]), "cut in")
                    require(native(clip.source_range.duration) == wire(shot["source_range"]["end"]) - wire(shot["source_range"]["start"]), "cut duration")
                require(parsed.metadata["storytelling"]["selected_proposal_json"] == owner["selected_proposal_json"], "roundtrip proposal")
                require(parsed.metadata["storytelling"]["source_snapshot_json"] == owner["source_snapshot_json"], "roundtrip source facts")
            require(before == {p.name: p.read_bytes() for p in root.iterdir()}, "source/host write or extra output")
            (root / "review.json").write_bytes(bundle_bytes)
            checked = json.loads(run("check-story-review", "review.json", "proposal.json", "snapshot.json", "context.json"))
            require(checked["freshness"]["current"], "fresh review rejected")
            changed = json.loads(ctx); changed["loss_choices"] = ["metadata_only_locks", "picture_only"]
            (root / "context.json").write_text(json.dumps(changed))
            stale = json.loads(run("check-story-review", "review.json", "proposal.json", "snapshot.json", "context.json", exit_code=1))
            require(not stale["freshness"]["current"], "changed loss choice did not invalidate review")
    print("2 actual planner/bridge/hashlib/official-OTIO read-write-read cases passed; no host invocation.")


if __name__ == "__main__":
    main()
