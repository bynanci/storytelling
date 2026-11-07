#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Exercise the actual pinned source producer under bounded process ownership.

These live tests require the three explicit PLANNER_SHADOW_* tool/source paths;
missing configuration fails instead of silently skipping source evidence.
"""
from __future__ import annotations

import copy
import hashlib
import json
import os
from pathlib import Path
import re
import time
import unittest

from process import MAX_INPUT, ProcessFailure, run_bounded


PRODUCER = Path(__file__).with_name("producer.mts").resolve()
SOURCE_SHA = "7add4762fe8174d76f43b863375ea39b73121e76"
TOP_KEYS = {
    "schema_version", "source_head", "batch_sha256", "producer_sha256",
    "node_version", "node_sha256", "tsx_version", "tsx_loader_sha256",
    "tsx_package_sha256", "esbuild_version", "esbuild_sha256", "results",
}


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate producer output property")
        result[key] = value
    return result


def batch():
    return {
        "schema_version": 1,
        "cases": [{
            "id": "valid",
            "source_input": {
                "story": {
                    "id": "research", "title": "Synthetic",
                    "targetDuration": 0.5, "maxDuration": 0.5,
                    "beats": [{"id": "beat", "title": "Sequence",
                               "selectedAssetIds": ["a"]}],
                },
                "assets": [{
                    "id": "a", "name": "Synthetic a",
                    "uri": "synthetic://shadow/a", "mediaType": "video",
                    "duration": 0.5, "rating": 3, "tags": [],
                    "state": {}, "metadata": {},
                }],
            },
        }],
    }


class ProducerChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.paths = {}
        for name in ("SOURCE", "NODE", "TSX"):
            key = "PLANNER_SHADOW_" + name
            supplied = os.environ.get(key)
            if not supplied or not Path(supplied).is_absolute():
                raise RuntimeError(f"{key} must explicitly name an existing absolute path")
            path = Path(supplied).resolve(strict=True)
            if not (path.is_dir() if name == "SOURCE" else path.is_file()):
                raise RuntimeError(f"{key} has the wrong path type")
            cls.paths[name] = path
        cls.environment = dict(os.environ)
        cls.environment.pop("NODE_OPTIONS", None)
        cls.environment.pop("ESBUILD_BINARY_PATH", None)
        cls.environment["TSX_TSCONFIG_PATH"] = str(cls.paths["SOURCE"] / "tsconfig.json")
        loader = cls.paths["TSX"]
        cls.expected_hashes = {
            "node_sha256": digest(cls.paths["NODE"]),
            "tsx_loader_sha256": digest(loader),
            "tsx_package_sha256": digest(loader.parent.parent / "package.json"),
            "esbuild_sha256": digest(loader.parents[2] / "@esbuild/linux-x64/bin/esbuild"),
            "producer_sha256": digest(PRODUCER),
        }

    def invoke(self, document):
        raw = document if isinstance(document, bytes) else json.dumps(document).encode()
        receipt = run_bounded(
            [str(self.paths["NODE"]), "--import", str(self.paths["TSX"]),
             str(PRODUCER), "--source", str(self.paths["SOURCE"])],
            raw, time.monotonic() + 10, timeout=10,
            env=self.environment,
        )
        self.assertTrue(receipt["capture_complete"])
        self.assertFalse(receipt["stdout_truncated"])
        self.assertFalse(receipt["stderr_truncated"])
        return raw, receipt

    def success(self, document):
        raw, receipt = self.invoke(document)
        self.assertEqual(receipt["exit"], 0, receipt["stderr"])
        self.assertEqual(receipt["stderr"], b"")
        self.assertTrue(receipt["stdout"].endswith(b"\n"))
        self.assertEqual(receipt["stdout"].count(b"\n"), 1)
        response = json.loads(receipt["stdout"], object_pairs_hook=unique)
        self.assertEqual(set(response), TOP_KEYS)
        self.assertEqual(response["schema_version"], 1)
        self.assertEqual(response["source_head"], SOURCE_SHA)
        self.assertEqual(response["batch_sha256"], hashlib.sha256(raw).hexdigest())
        self.assertEqual(response["node_version"], "v24.22.1")
        self.assertEqual(response["tsx_version"], "4.23.15")
        self.assertEqual(response["esbuild_version"], "0.28.2")
        for name, expected in self.expected_hashes.items():
            self.assertEqual(response[name], expected, name)
            self.assertIsNotNone(re.fullmatch(r"[a-f0-9]{64}", response[name]))
        return response

    def rejection(self, document, family):
        _, receipt = self.invoke(document)
        self.assertEqual(receipt["exit"], 1)
        self.assertEqual(receipt["stdout"], b"")
        self.assertIn(family, receipt["stderr"])

    def test_actual_compose_metadata_and_exact_short_source_semantics(self):
        document = batch()
        before = copy.deepcopy(document)
        response = self.success(document)
        self.assertEqual(document, before)
        self.assertEqual(response["results"], [{
            "id": "valid", "input_unchanged": True, "ok": True,
            "composition": {
                "storyId": "research", "duration": 0.5,
                "tracks": [{"type": "video", "clips": [{
                    "assetId": "a", "beatId": "beat", "sourceIn": 0,
                    "sourceOut": 0.5, "timelineStart": 0,
                    "timelineDuration": 0.5,
                }]}],
            },
        }])

    def test_multiple_source_rows_keep_the_supplied_semantic_order(self):
        document = batch()
        first = document["cases"][0]
        first["id"] = "z-first"
        second = copy.deepcopy(first)
        second["id"] = "a-second"
        second["source_input"]["story"]["id"] = "second-story"
        document["cases"].append(second)
        response = self.success(document)
        self.assertEqual([row["id"] for row in response["results"]], ["z-first", "a-second"])
        self.assertEqual([row["composition"]["storyId"] for row in response["results"]],
                         ["research", "second-story"])

    def test_only_actual_composition_constraint_error_is_serialized_as_data(self):
        document = batch()
        document["cases"][0]["source_input"]["story"]["beats"][0]["selectedAssetIds"] = ["missing"]
        response = self.success(document)
        self.assertEqual(response["results"], [{
            "id": "valid", "input_unchanged": True, "ok": False,
            "error": {
                "name": "CompositionConstraintError",
                "message": 'Beat "Sequence" references missing asset missing. Relink the asset or remove the constraint.',
            },
        }])

    def test_unexpected_source_exception_is_an_execution_failure(self):
        document = batch()
        document["cases"][0]["source_input"]["story"]["beats"] = None
        self.rejection(document, b"Cannot read properties of null (reading 'map')")

    def test_duplicate_properties_including_decoded_keys_are_rejected(self):
        raw = json.dumps(batch()).encode()
        duplicates = [
            raw.replace(b'"schema_version": 1', b'"schema_version": 1, "schema_version": 1', 1),
            raw.replace(b'"schema_version": 1', b'"schema_version": 1, "schema_versi\\u006fn": 1', 1),
            raw.replace(b'"id": "valid"', b'"id": "valid", "id": "valid"', 1),
        ]
        for document in duplicates:
            with self.subTest(document=document):
                self.rejection(document, b"duplicate JSON property")

    def test_extra_missing_null_version_and_duplicate_case_shapes_reject(self):
        mutations = []
        document = batch(); document["extra"] = True
        mutations.append((document, b"unexpected object fields"))
        document = batch(); del document["schema_version"]
        mutations.append((document, b"unexpected object fields"))
        document = batch(); document["cases"] = None
        mutations.append((document, b"source case count exceeded"))
        document = batch(); document["schema_version"] = 2
        mutations.append((document, b"unsupported batch version"))
        document = batch(); document["cases"][0]["source_input"]["extra"] = True
        mutations.append((document, b"unexpected object fields"))
        document = batch(); document["cases"].append(copy.deepcopy(document["cases"][0]))
        mutations.append((document, b"duplicate case identity"))
        for document, family in mutations:
            with self.subTest(family=family):
                self.rejection(document, family)

    def test_invalid_utf8_cannot_be_replaced_into_valid_source_json(self):
        raw = json.dumps(batch()).encode().replace(b'Synthetic a', b'Synthetic \xff')
        self.rejection(raw, b"encoded data")

    def test_exact_input_limit_accepts_and_transport_refuses_the_next_byte(self):
        raw = json.dumps(batch()).encode()
        exact = raw + b" " * (MAX_INPUT - len(raw))
        self.success(exact)
        with self.assertRaises(ProcessFailure) as rejected:
            self.invoke(exact + b" ")
        self.assertEqual(rejected.exception.code, "input-overflow")
        self.assertFalse(rejected.exception.receipt["started"])
        self.assertEqual(rejected.exception.receipt["stdout"], b"")


if __name__ == "__main__":
    unittest.main()
