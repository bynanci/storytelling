#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Check prospective design artifacts; this is not an execution validator."""
import hashlib
import json
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def read(name):
    return json.loads((HERE / name).read_text(encoding="utf-8"))


def framed(domain, data):
    return hashlib.sha256(domain + b"\0" + len(data).to_bytes(8, "big") + data).hexdigest()


def compact(value):
    return json.dumps(value, ensure_ascii=True, separators=(",", ":")).encode("ascii")


def no_duplicates(pairs):
    value = {}
    for key, item in pairs:
        if key in value:
            raise ValueError("duplicate field")
        value[key] = item
    return value


def container_depth(value):
    if isinstance(value, dict):
        return 1 + max((container_depth(v) for v in value.values()), default=0)
    if isinstance(value, list):
        return 1 + max((container_depth(v) for v in value), default=0)
    return 0


class DesignArtifacts(unittest.TestCase):
    def test_intent_vector_matches_exact_ordered_bytes_and_domain(self):
        request = read("valid-request.json")
        vector = read("identity-vector.json")
        payload = compact({"schema_version": 1, "intent": request["intent"]})
        self.assertEqual(payload.decode(), vector["payload_utf8"])
        self.assertEqual(len(payload), vector["payload_bytes"])
        self.assertEqual(framed(b"storytelling.execution-identity.v1", payload), vector["intent_sha256"])
        self.assertEqual(request["intent_sha256"], vector["intent_sha256"])
        self.assertNotEqual(framed(b"storytelling.execution-identity.v2", payload), vector["intent_sha256"])

    def test_receipt_vector_has_required_synthetic_applied_evidence(self):
        result = read("applied-result.json")
        vector = read("receipt-vector.json")
        payload = compact(result["receipt"]["payload"])
        self.assertEqual(payload.decode(), vector["payload_utf8"])
        self.assertEqual(len(payload), vector["payload_bytes"])
        self.assertEqual(framed(b"storytelling.execution-receipt.v1", payload), vector["receipt_sha256"])
        self.assertEqual(result["receipt"]["sha256"], vector["receipt_sha256"])
        self.assertEqual(result["intent_sha256"], read("valid-request.json")["intent_sha256"])
        for field in ("authority_receipt_sha256", "readback_sha256", "quiescence_sha256", "target_revision_after_sha256"):
            self.assertRegex(result["receipt"]["payload"][field], r"^[0-9a-f]{64}$")

    def test_schema_order_and_closed_shapes_match_examples(self):
        schema = json.loads((ROOT / "contracts/execution-intent-v1.schema.json").read_text())
        self.assertFalse(schema["additionalProperties"])
        self.assertFalse(schema["properties"]["intent"]["additionalProperties"])
        self.assertEqual(schema["required"], list(read("valid-request.json")))
        self.assertEqual(schema["properties"]["intent"]["required"], list(read("valid-request.json")["intent"]))
        result_schema = json.loads((ROOT / "contracts/execution-result-v1.schema.json").read_text())
        self.assertFalse(result_schema["additionalProperties"])
        receipt = result_schema["$defs"]["receipt"]["properties"]["payload"]
        self.assertFalse(receipt["additionalProperties"])
        self.assertEqual(receipt["required"], list(read("applied-result.json")["receipt"]["payload"]))
        self.assertEqual(result_schema["required"], list(read("applied-result.json")))

    def test_case_inventory_is_complete_and_expected_calls_are_explicit(self):
        inventory = read("cases.json")
        self.assertEqual(inventory["status"], "test-design-only")
        cases = inventory["cases"]
        self.assertEqual([c["id"] for c in cases], [f"EX{i:02d}" for i in range(1, 38)])
        schema = json.loads((ROOT / "contracts/execution-result-v1.schema.json").read_text())
        codes = schema["properties"]["error"]["anyOf"][0]["properties"]["code"]["enum"]
        for case in cases:
            expected = case["expected"]
            if expected["code"] is not None:
                self.assertIn(expected["code"], codes)
            self.assertEqual(expected["automatic_retries"], 0)
            self.assertEqual(expected["undo_calls"], 0)
            if expected["outcome"] == "rejected":
                self.assertEqual(expected["fake_mutation_calls"], 0)
            if case["scenario"].endswith(".json"):
                self.assertTrue((HERE / case["scenario"]).is_file())

    def test_raw_boundary_fixtures_keep_their_intended_defects(self):
        with self.assertRaisesRegex(ValueError, "duplicate field"):
            json.loads((HERE / "duplicate-json.json").read_text(), object_pairs_hook=no_duplicates)
        self.assertEqual(read("unknown-version.json")["schema_version"], 2)
        original = compact(read("valid-request.json"))
        for size in (65_536, 65_537):
            wire = original.replace(b':1,', b':' + b' ' * (size - len(original)) + b'1,', 1)
            self.assertEqual(len(wire), size)
            self.assertEqual(json.loads(wire), read("valid-request.json"))
        self.assertEqual(container_depth(read("depth-eight.json")), 8)
        self.assertEqual(container_depth(read("depth-nine.json")), 9)
        self.assertIn("extra", read("unknown-field.json"))
        self.assertIsNone(read("null-approval.json")["approval_ref"])
        self.assertEqual(read("bad-hash.json")["intent"]["artifact_sha256"], "INVALID")
        self.assertIsInstance(read("noninteger-version.json")["schema_version"], float)


if __name__ == "__main__":
    unittest.main()
