// SPDX-License-Identifier: Apache-2.0
// Source-informed fixed-window planner; see provenance-m2-02.json.
use storytelling_core::planner::{PlannedStoryV1, PlanningErrorV1};

pub fn plan_story(_bytes: &[u8]) -> Result<PlannedStoryV1, PlanningErrorV1> {
    Err(PlanningErrorV1 {
        code: "planner.not_implemented".into(),
        path: "$".into(),
        conflicts: vec![],
    })
}
