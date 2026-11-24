// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Cycle Detection and Backtracking Tracer for Directed Acyclic Graphs
//! Story: bench-06-antagonistic-dag | Code Slot: dag_cycle_tracer

use std::collections::BTreeSet;
use crate::dag_nodes::{DagNode, DagNodeStore, NodeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleTrace {
    pub path: Vec<NodeId>,
}

impl CycleTrace {
    pub fn new(path: Vec<NodeId>) -> Self {
        Self { path }
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn is_closed(&self) -> bool {
        self.path.len() >= 2 && self.path.first() == self.path.last()
    }

    pub fn cycle_length(&self) -> usize {
        if self.is_closed() {
            self.path.len() - 1
        } else {
            self.path.len()
        }
    }

    pub fn contains(&self, id: &NodeId) -> bool {
        self.path.contains(id)
    }

    pub fn canonical(&self) -> Vec<NodeId> {
        if self.path.len() <= 1 {
            return self.path.clone();
        }
        let nodes = if self.is_closed() {
            &self.path[..self.path.len() - 1]
        } else {
            &self.path[..]
        };
        if nodes.is_empty() {
            return Vec::new();
        }
        let min_pos = nodes
            .iter()
            .enumerate()
            .min_by_key(|&(_, id)| id)
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        let mut canon = Vec::with_capacity(nodes.len() + 1);
        canon.extend_from_slice(&nodes[min_pos..]);
        canon.extend_from_slice(&nodes[..min_pos]);
        canon.push(canon[0]);
        canon
    }
}

#[derive(Debug, Default)]
pub struct CycleTracer;

impl CycleTracer {
    pub fn has_cycle(store: &DagNodeStore) -> bool {
        Self::find_cycle(store).is_some()
    }

    pub fn find_cycle(store: &DagNodeStore) -> Option<CycleTrace> {
        let mut visited = BTreeSet::new();
        let mut on_stack = BTreeSet::new();
        let mut path = Vec::new();

        for id in store.node_ids() {
            if visited.contains(&id) {
                continue;
            }
            if let Some(cycle) = Self::dfs_backtrack(store, id, &mut visited, &mut on_stack, &mut path) {
                return Some(cycle);
            }
        }
        None
    }

    pub fn trace_cycle_from(store: &DagNodeStore, start: NodeId) -> Option<CycleTrace> {
        let mut visited = BTreeSet::new();
        let mut on_stack = BTreeSet::new();
        let mut path = Vec::new();
        Self::dfs_backtrack(store, start, &mut visited, &mut on_stack, &mut path)
    }

    fn dfs_backtrack(
        store: &DagNodeStore,
        current: NodeId,
        visited: &mut BTreeSet<NodeId>,
        on_stack: &mut BTreeSet<NodeId>,
        path: &mut Vec<NodeId>,
    ) -> Option<CycleTrace> {
        visited.insert(current);
        on_stack.insert(current);
        path.push(current);

        let deps = store.dependencies_of(&current).unwrap_or_default();
        for dep in deps {
            if on_stack.contains(&dep) {
                let pos = path.iter().position(|&n| n == dep).unwrap_or(0);
                let mut cycle_path = path[pos..].to_vec();
                cycle_path.push(dep);
                return Some(CycleTrace::new(cycle_path));
            }
            if visited.contains(&dep) {
                continue;
            }
            if let Some(cycle) = Self::dfs_backtrack(store, dep, visited, on_stack, path) {
                return Some(cycle);
            }
        }

        path.pop();
        on_stack.remove(&current);
        None
    }

    pub fn find_all_cycles(store: &DagNodeStore) -> Vec<CycleTrace> {
        let mut results: Vec<CycleTrace> = Vec::new();
        let mut seen_canonical = BTreeSet::new();

        for id in store.node_ids() {
            let mut visited = BTreeSet::new();
            let mut on_stack = BTreeSet::new();
            let mut path = Vec::new();
            if let Some(cycle) = Self::dfs_backtrack(
                store,
                id,
                &mut visited,
                &mut on_stack,
                &mut path,
            ) {
                let canon = cycle.canonical();
                if seen_canonical.insert(canon) {
                    results.push(cycle);
                }
            }
        }
        results
    }
}

// code_slot: dag_cycle_tracer
pub fn dag_cycle_tracer() {
    let slot = "dag_cycle_tracer";
    assert_eq!(slot, "dag_cycle_tracer");

    let mut store = DagNodeStore::new();
    let mut n1 = DagNode::new(NodeId::new(1), "task_alpha");
    let mut n2 = DagNode::new(NodeId::new(2), "task_beta");
    let mut n3 = DagNode::new(NodeId::new(3), "task_gamma");

    n1.add_dependency(NodeId::new(2));
    n2.add_dependency(NodeId::new(3));
    n3.add_dependency(NodeId::new(1));

    assert!(store.insert(n1).is_ok());
    assert!(store.insert(n2).is_ok());
    assert!(store.insert(n3).is_ok());

    let cycle = CycleTracer::find_cycle(&store).expect("cycle expected");
    assert!(cycle.is_closed());
    assert_eq!(cycle.cycle_length(), 3);
    assert_eq!(
        cycle.canonical(),
        vec![
            NodeId::new(1),
            NodeId::new(2),
            NodeId::new(3),
            NodeId::new(1),
        ]
    );
}
