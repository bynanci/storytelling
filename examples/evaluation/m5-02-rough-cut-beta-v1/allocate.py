#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Create the constrained M5-02 five-slot allocation in private storage.

Run only after protocol acceptance and immediately before recruitment.
The output path is created exclusively; an existing file is never overwritten.
Each participant receives one pack used in both condition arms.
"""
from __future__ import annotations

import argparse
from itertools import combinations
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
    storytelling_first = set(rng.sample(slots, 3))
    direct_first = set(slots) - storytelling_first

    valid_pack_a_assignments = []
    for pack_a_count in (2, 3):
        for selected in combinations(slots, pack_a_count):
            pack_a_slots = set(selected)
            s_group_has_both = bool(pack_a_slots & storytelling_first) and bool(storytelling_first - pack_a_slots)
            d_group_has_both = bool(pack_a_slots & direct_first) and bool(direct_first - pack_a_slots)
            if s_group_has_both and d_group_has_both:
                valid_pack_a_assignments.append(pack_a_slots)

    pack_a_slots = rng.choice(valid_pack_a_assignments)
    assignments = []
    for slot in slots:
        pack = "A" if slot in pack_a_slots else "B"
        order = ["S", "D"] if slot in storytelling_first else ["D", "S"]
        assignments.append({
            "slot": slot,
            "planned_order": order,
            "assigned_pack": pack,
            "sessions": [{"condition": condition, "pack": pack} for condition in order],
        })

    document = {
        "protocol_id": PROTOCOL_ID,
        "allocation_algorithm": "python-random.Random-v1",
        "python_version": sys.version,
        "seed": seed,
        "constraints": {
            "storytelling_first": len(storytelling_first),
            "direct_first": len(direct_first),
            "pack_a_slots": len(pack_a_slots),
            "storytelling_first_packs": sorted({item["assigned_pack"] for item in assignments if item["planned_order"][0] == "S"}),
            "direct_first_packs": sorted({item["assigned_pack"] for item in assignments if item["planned_order"][0] == "D"}),
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

