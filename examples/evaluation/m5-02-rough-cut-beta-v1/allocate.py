#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Create the constrained M5-02 five-slot allocation in private storage.

Run only after protocol acceptance and immediately before recruitment.
The output path is created exclusively; an existing file is never overwritten.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import random
import secrets
import sys
from pathlib import Path

PROTOCOL_ID = "m5-02-rough-cut-beta-v1"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", required=True, help="private output JSON path; must not exist")
    parser.add_argument("--seed", help="optional pre-generated seed; keep the allocation private")
    args = parser.parse_args()

    seed = args.seed if args.seed is not None else secrets.token_hex(16)
    rng = random.Random(seed)
    slots = [f"P{i:02d}" for i in range(1, 6)]
    story_first = set(rng.sample(slots, 3))
    story_pack_a_count = rng.choice((2, 3))
    story_pack_a = set(rng.sample(slots, story_pack_a_count))

    assignments = []
    for slot in slots:
        s_pack = "A" if slot in story_pack_a else "B"
        d_pack = "B" if s_pack == "A" else "A"
        order = ["S", "D"] if slot in story_first else ["D", "S"]
        pack_by_condition = {"S": s_pack, "D": d_pack}
        assignments.append({
            "slot": slot,
            "planned_order": order,
            "storytelling_pack": s_pack,
            "direct_pack": d_pack,
            "sessions": [
                {"condition": condition, "pack": pack_by_condition[condition]}
                for condition in order
            ],
        })

    document = {
        "protocol_id": PROTOCOL_ID,
        "allocation_algorithm": "python-random.Random-v1",
        "python_version": sys.version,
        "seed": seed,
        "constraints": {
            "storytelling_first": 3,
            "direct_first": 2,
            "storytelling_pack_a": story_pack_a_count,
            "direct_pack_a": 5 - story_pack_a_count,
        },
        "assignments": assignments,
    }
    payload = (json.dumps(document, ensure_ascii=False, sort_keys=True, indent=2) + "\n").encode()
    output = Path(args.output)
    with output.open("xb") as stream:
        stream.write(payload)
    print(json.dumps({
        "protocol_id": PROTOCOL_ID,
        "allocation_sha256": hashlib.sha256(payload).hexdigest(),
        "slots": len(assignments),
        "created": True,
    }, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

