// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Dependency Node Structure and Storage by Identifier
//! Story: bench-06-antagonistic-dag | Code Slot: dag_core_nodes

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u64);

impl NodeId {
    pub const fn new(id: u64) -> Self { Self(id) }
    pub const fn as_u64(&self) -> u64 { self.0 }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "node_{}", self.0)
    }
}

impl From<u64> for NodeId {
    fn from(val: u64) -> Self { Self(val) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeState {
    Pending,
    Ready,
    Running,
    Completed,
    Failed,
    Skipped,
}

impl NodeState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Skipped)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagNode {
    pub id: NodeId,
    pub name: String,
    pub dependencies: Vec<NodeId>,
    pub state: NodeState,
    pub priority: u32,
}

impl DagNode {
    pub fn new(id: NodeId, name: impl Into<String>) -> Self {
        Self { id, name: name.into(), dependencies: Vec::new(), state: NodeState::Pending, priority: 0 }
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn add_dependency(&mut self, dep: NodeId) {
        if !self.dependencies.contains(&dep) { self.dependencies.push(dep); }
    }

    pub fn has_dependency(&self, dep: &NodeId) -> bool { self.dependencies.contains(dep) }
    pub fn set_state(&mut self, state: NodeState) { self.state = state; }
}

#[derive(Debug, Clone, Default)]
pub struct DagNodeStore {
    nodes: BTreeMap<NodeId, DagNode>,
}

impl DagNodeStore {
    pub fn new() -> Self { Self { nodes: BTreeMap::new() } }
    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }

    pub fn insert(&mut self, node: DagNode) -> Result<(), String> {
        if self.nodes.contains_key(&node.id) {
            return Err(format!("Duplicate node id: {}", node.id.as_u64()));
        }
        self.nodes.insert(node.id, node);
        Ok(())
    }

    pub fn get(&self, id: &NodeId) -> Option<&DagNode> { self.nodes.get(id) }
    pub fn get_mut(&mut self, id: &NodeId) -> Option<&mut DagNode> { self.nodes.get_mut(id) }
    pub fn contains(&self, id: &NodeId) -> bool { self.nodes.contains_key(id) }
    pub fn remove(&mut self, id: &NodeId) -> Option<DagNode> { self.nodes.remove(id) }
    pub fn node_ids(&self) -> Vec<NodeId> { self.nodes.keys().copied().collect() }

    pub fn dependencies_of(&self, id: &NodeId) -> Result<Vec<NodeId>, String> {
        let node = self.get(id).ok_or_else(|| format!("Node not found: {}", id.as_u64()))?;
        Ok(node.dependencies.clone())
    }

    pub fn dependents_of(&self, id: &NodeId) -> Vec<NodeId> {
        let mut deps: Vec<NodeId> = self.nodes.values()
            .filter(|n| n.has_dependency(id)).map(|n| n.id).collect();
        deps.sort();
        deps
    }

    pub fn in_degree(&self, id: &NodeId) -> Result<usize, String> {
        let node = self.get(id).ok_or_else(|| format!("Node not found: {}", id.as_u64()))?;
        Ok(node.dependencies.len())
    }

    pub fn root_nodes(&self) -> Vec<NodeId> {
        let mut roots: Vec<NodeId> = self.nodes.values()
            .filter(|n| n.dependencies.is_empty()).map(|n| n.id).collect();
        roots.sort();
        roots
    }

    pub fn leaf_nodes(&self) -> Vec<NodeId> {
        let mut leaves: Vec<NodeId> = self.nodes.keys()
            .filter(|id| self.dependents_of(id).is_empty()).copied().collect();
        leaves.sort();
        leaves
    }

    pub fn validate_dependencies(&self) -> Result<(), String> {
        for node in self.nodes.values() {
            for dep in &node.dependencies {
                if !self.nodes.contains_key(dep) {
                    return Err(format!("Node {} missing dependency {}", node.id.as_u64(), dep.as_u64()));
                }
            }
        }
        Ok(())
    }

    pub fn ready_nodes(&self) -> Vec<NodeId> {
        let mut ready: Vec<NodeId> = self.nodes.values()
            .filter(|n| n.state == NodeState::Pending)
            .filter(|n| n.dependencies.iter().all(|d| {
                self.nodes.get(d).is_some_and(|dep| dep.state == NodeState::Completed)
            }))
            .map(|n| n.id).collect();
        ready.sort();
        ready
    }

    pub fn update_state(&mut self, id: &NodeId, state: NodeState) -> Result<(), String> {
        let node = self.get_mut(id).ok_or_else(|| format!("Node not found: {}", id.as_u64()))?;
        node.set_state(state);
        Ok(())
    }
}

// code_slot: dag_core_nodes
pub fn dag_core_nodes() {
    let mut store = DagNodeStore::new();
    let n1 = DagNode::new(NodeId::new(101), "fetch_data");
    let mut n2 = DagNode::new(NodeId::new(102), "transform_data");
    n2.add_dependency(NodeId::new(101));

    assert!(store.insert(n1).is_ok());
    assert!(store.insert(n2).is_ok());
    assert_eq!(store.len(), 2);
    assert!(store.validate_dependencies().is_ok());

    assert_eq!(store.root_nodes(), vec![NodeId::new(101)]);
    assert_eq!(store.ready_nodes(), vec![NodeId::new(101)]);

    assert!(store.update_state(&NodeId::new(101), NodeState::Completed).is_ok());
    assert_eq!(store.ready_nodes(), vec![NodeId::new(102)]);
}

