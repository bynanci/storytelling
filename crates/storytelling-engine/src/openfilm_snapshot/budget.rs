// SPDX-License-Identifier: Apache-2.0
// Source semantics and attribution: docs/migration/provenance-m1-01.json, LICENSES/third-party/NOTICE.txt.
//! Bounded JSON traversal without converting source number lexemes.

use std::collections::BTreeSet;
use std::fmt;

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde_json::value::RawValue;
use storytelling_core::compatibility::ImportError;

const MAX_DEPTH: usize = 32;
const MAX_ENTRIES: usize = 65_536;
const MAX_STRING_BYTES: usize = 4096;
const MAX_KEY_BYTES: usize = 128;

pub(super) fn preflight(bytes: &[u8]) -> Result<(), ImportError> {
    let raw: &RawValue = serde_json::from_slice(bytes).map_err(|_| error(false))?;
    let mut budget = Budget::default();
    budget.scan(raw, 0).map_err(|_| error(budget.exceeded))
}

fn error(exceeded: bool) -> ImportError {
    ImportError {
        code: if exceeded {
            "import.budget_exceeded"
        } else {
            "input.invalid_json"
        }
        .to_owned(),
        path: "$".to_owned(),
        losses: Vec::new(),
    }
}

#[derive(Default)]
struct Budget {
    entries: usize,
    exceeded: bool,
}

impl Budget {
    fn check<E: de::Error>(&mut self, allowed: bool) -> Result<(), E> {
        if allowed {
            Ok(())
        } else {
            self.exceeded = true;
            Err(E::custom("snapshot budget exceeded"))
        }
    }

    fn entry<E: de::Error>(&mut self) -> Result<(), E> {
        self.check(self.entries < MAX_ENTRIES)?;
        self.entries += 1;
        Ok(())
    }

    fn scan(&mut self, raw: &RawValue, depth: usize) -> Result<(), serde_json::Error> {
        self.entry()?;
        match raw.get().as_bytes()[0] {
            b'{' => {
                self.check(depth < MAX_DEPTH)?;
                let mut deserializer = serde_json::Deserializer::from_str(raw.get());
                de::Deserializer::deserialize_map(
                    &mut deserializer,
                    ObjectVisitor {
                        budget: self,
                        depth: depth + 1,
                    },
                )?;
                deserializer.end()
            }
            b'[' => {
                self.check(depth < MAX_DEPTH)?;
                let mut deserializer = serde_json::Deserializer::from_str(raw.get());
                de::Deserializer::deserialize_seq(
                    &mut deserializer,
                    ArrayVisitor {
                        budget: self,
                        depth: depth + 1,
                    },
                )?;
                deserializer.end()
            }
            b'"' => {
                let decoded: String = serde_json::from_str(raw.get())?;
                self.check(decoded.len() <= MAX_STRING_BYTES)
            }
            // RawValue has already validated primitive JSON syntax. In particular,
            // numbers must never pass through serde's floating-point visitor.
            _ => Ok(()),
        }
    }
}

struct ObjectVisitor<'a> {
    budget: &'a mut Budget,
    depth: usize,
}

impl<'de> Visitor<'de> for ObjectVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<(), A::Error> {
        let mut keys = BTreeSet::new();
        while let Some(key) = object.next_key::<String>()? {
            self.budget.entry()?;
            self.budget.check(key.len() <= MAX_KEY_BYTES)?;
            if !keys.insert(key) {
                return Err(de::Error::custom("duplicate JSON object key"));
            }
            let value: &RawValue = object.next_value()?;
            self.budget
                .scan(value, self.depth)
                .map_err(|_| <A::Error as de::Error>::custom("snapshot preflight rejected"))?;
        }
        Ok(())
    }
}

struct ArrayVisitor<'a> {
    budget: &'a mut Budget,
    depth: usize,
}

impl<'de> Visitor<'de> for ArrayVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON array")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut array: A) -> Result<(), A::Error> {
        while let Some(value) = array.next_element::<&RawValue>()? {
            self.budget
                .scan(value, self.depth)
                .map_err(|_| <A::Error as de::Error>::custom("snapshot preflight rejected"))?;
        }
        Ok(())
    }
}

/// Remove only JSON whitespace outside strings, preserving source lexemes.
pub(super) fn compact(bytes: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes.len());
    let mut in_string = false;
    let mut escaped = false;
    for &byte in bytes {
        if in_string {
            result.push(byte);
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else if byte == b'"' {
            in_string = true;
            result.push(byte);
        } else if !matches!(byte, b' ' | b'\t' | b'\r' | b'\n') {
            result.push(byte);
        }
    }
    result
}
