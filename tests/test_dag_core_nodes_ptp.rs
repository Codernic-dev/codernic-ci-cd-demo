// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Dependency Node Structure
//! Story: bench-06-antagonistic-dag | Code Slot: dag_core_nodes

#[path = "../backend-rust/src/dag_nodes.rs"]
mod dag_nodes;

use dag_nodes::{dag_core_nodes, DagNode, DagNodeStore, NodeId, NodeState};

fn test_node_id_and_state() {
    let id_a = NodeId::new(10);
    let id_b = NodeId::from(20);
    assert_eq!(id_a.as_u64(), 10);
    assert_eq!(id_b.as_u64(), 20);
    assert_eq!(format!("{id_a}"), "node_10");
    assert!(id_a < id_b);

    assert!(!NodeState::Pending.is_terminal());
    assert!(!NodeState::Ready.is_terminal());
    assert!(!NodeState::Running.is_terminal());
    assert!(NodeState::Completed.is_terminal());
    assert!(NodeState::Failed.is_terminal());
    assert!(NodeState::Skipped.is_terminal());
}

fn test_node_construction_and_dependencies() {
    let mut node = DagNode::new(NodeId::new(1), "compile_task").with_priority(5);
    assert_eq!(node.id, NodeId::new(1));
    assert_eq!(node.name, "compile_task");
    assert_eq!(node.priority, 5);
    assert_eq!(node.state, NodeState::Pending);

    node.add_dependency(NodeId::new(2));
    node.add_dependency(NodeId::new(2));
    assert_eq!(node.dependencies.len(), 1);
    assert!(node.has_dependency(&NodeId::new(2)));
    assert!(!node.has_dependency(&NodeId::new(3)));

    node.set_state(NodeState::Running);
    assert_eq!(node.state, NodeState::Running);
}

fn test_store_crud_and_validation() {
    let mut store = DagNodeStore::new();
    assert!(store.is_empty());

    let n1 = DagNode::new(NodeId::new(1), "init");
    let mut n2 = DagNode::new(NodeId::new(2), "build");
    n2.add_dependency(NodeId::new(1));

    assert!(store.insert(n1.clone()).is_ok());
    assert!(store.insert(n1).is_err());
    assert!(store.insert(n2).is_ok());
    assert_eq!(store.len(), 2);
    assert!(store.contains(&NodeId::new(1)));

    assert!(store.validate_dependencies().is_ok());

    let mut n3 = DagNode::new(NodeId::new(3), "deploy");
    n3.add_dependency(NodeId::new(999));
    assert!(store.insert(n3).is_ok());
    assert!(store.validate_dependencies().is_err());
    assert!(store.remove(&NodeId::new(3)).is_some());
    assert!(store.validate_dependencies().is_ok());
}

fn test_store_topology_and_readiness() {
    let mut store = DagNodeStore::new();
    let root = DagNode::new(NodeId::new(10), "root_task");
    let mut leaf = DagNode::new(NodeId::new(20), "leaf_task");
    leaf.add_dependency(NodeId::new(10));

    assert!(store.insert(root).is_ok());
    assert!(store.insert(leaf).is_ok());

    assert_eq!(store.node_ids(), vec![NodeId::new(10), NodeId::new(20)]);
    assert_eq!(store.get(&NodeId::new(10)).unwrap().name, "root_task");
    assert!(store.get_mut(&NodeId::new(10)).is_some());

    assert_eq!(store.root_nodes(), vec![NodeId::new(10)]);
    assert_eq!(store.leaf_nodes(), vec![NodeId::new(20)]);
    assert_eq!(store.dependents_of(&NodeId::new(10)), vec![NodeId::new(20)]);
    assert_eq!(store.dependencies_of(&NodeId::new(20)).unwrap(), vec![NodeId::new(10)]);
    assert_eq!(store.in_degree(&NodeId::new(20)).unwrap(), 1);

    assert_eq!(store.ready_nodes(), vec![NodeId::new(10)]);
    assert!(store.update_state(&NodeId::new(10), NodeState::Completed).is_ok());
    assert_eq!(store.ready_nodes(), vec![NodeId::new(20)]);
}

// code_slot: dag_core_nodes
#[test]
fn test_dag_core_nodes() {
    let slot = "dag_core_nodes";
    assert_eq!(slot, "dag_core_nodes");

    dag_core_nodes();
    test_node_id_and_state();
    test_node_construction_and_dependencies();
    test_store_crud_and_validation();
    test_store_topology_and_readiness();
}
