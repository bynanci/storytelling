// SPDX-License-Identifier: Apache-2.0
// M2-01 source-informed ordering policy: docs/migration/provenance-m2-01.json;
// NOTICE.openfilm. Iterative cycle-witness traversal is downstream implementation.
//! Find one actual declared-order cycle without choosing or filtering assets.

use std::collections::BTreeMap;
use storytelling_core::brief::{BriefConflictKindV1, BriefConflictV1, PrecedenceEdgeV1};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Visit {
    Unvisited,
    Active,
    Finished,
}

struct Frame {
    node: usize,
    next_neighbor: usize,
}

pub(super) fn first_order_cycle(edges: &[PrecedenceEdgeV1]) -> Option<BriefConflictV1> {
    let mut node_lookup = BTreeMap::new();
    let mut node_ids = Vec::new();
    let mut outgoing = Vec::<Vec<(usize, usize)>>::new();

    for (edge_index, edge) in edges.iter().enumerate() {
        let mut endpoints = [0; 2];
        for (slot, id) in endpoints
            .iter_mut()
            .zip([edge.before.as_str(), edge.after.as_str()])
        {
            *slot = *node_lookup.entry(id).or_insert_with(|| {
                let index = node_ids.len();
                node_ids.push(id);
                outgoing.push(Vec::new());
                index
            });
        }
        outgoing[endpoints[0]].push((endpoints[1], edge_index));
    }

    let mut visits = vec![Visit::Unvisited; node_ids.len()];
    let mut stack = Vec::<Frame>::with_capacity(node_ids.len());

    // The lookup map never controls traversal: indices retain first endpoint
    // appearance, and adjacency retains first edge appearance.
    for root in 0..node_ids.len() {
        if visits[root] != Visit::Unvisited {
            continue;
        }
        visits[root] = Visit::Active;
        stack.push(Frame {
            node: root,
            next_neighbor: 0,
        });

        while let Some(frame) = stack.last_mut() {
            let node = frame.node;
            let Some(&(neighbor, edge_index)) = outgoing[node].get(frame.next_neighbor) else {
                visits[node] = Visit::Finished;
                stack.pop();
                continue;
            };
            frame.next_neighbor += 1;

            match visits[neighbor] {
                Visit::Unvisited => {
                    visits[neighbor] = Visit::Active;
                    stack.push(Frame {
                        node: neighbor,
                        next_neighbor: 0,
                    });
                }
                Visit::Active => {
                    // Only the active segment starting at the gray target is
                    // cyclic; the closing target is not repeated in the report.
                    let asset_ids = stack
                        .iter()
                        .skip_while(|frame| frame.node != neighbor)
                        .map(|frame| node_ids[frame.node].to_owned())
                        .collect();
                    return Some(BriefConflictV1 {
                        kind: BriefConflictKindV1::OrderCycle,
                        path: edges[edge_index]
                            .origins
                            .first()
                            .cloned()
                            .unwrap_or_else(|| "$".to_owned()),
                        asset_ids,
                    });
                }
                Visit::Finished => {}
            }
        }
    }
    None
}
