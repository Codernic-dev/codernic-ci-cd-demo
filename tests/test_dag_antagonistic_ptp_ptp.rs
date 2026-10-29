// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Antagonistic DAGs
//! Story: bench-06-antagonistic-dag | Code Slot: dag_antagonistic_ptp

#[path = "../backend-rust/src/dag_nodes.rs"]
pub mod dag_nodes;

#[path = "../backend-rust/src/dag_tracer.rs"]
pub mod dag_tracer;

use dag_nodes::{DagNode, DagNodeStore, NodeId, NodeState};
use dag_tracer::CycleTracer;

fn test_dense_bipartite_dag_acyclic() {
    let mut store = DagNodeStore::new();
    for i in 1..=4 {
        let node = DagNode::new(NodeId::new(i), format!("src_task_{i}"));
        assert!(store.insert(node).is_ok());
    }
    for j in 5..=8 {
        let mut node = DagNode::new(NodeId::new(j), format!("sink_task_{j}"));
        for dep in 1..=4 {
            node.add_dependency(NodeId::new(dep));
        }
        assert!(store.insert(node).is_ok());
    }

    assert_eq!(store.len(), 8);
    assert!(store.validate_dependencies().is_ok());
    assert!(!CycleTracer::has_cycle(&store));
    assert_eq!(CycleTracer::find_cycle(&store), None);
    assert!(CycleTracer::find_all_cycles(&store).is_empty());

    assert_eq!(store.ready_nodes().len(), 4);
    for i in 1..=4 {
        assert!(store.update_state(&NodeId::new(i), NodeState::Completed).is_ok());
    }
    assert_eq!(
        store.ready_nodes(),
        vec![NodeId::new(5), NodeId::new(6), NodeId::new(7), NodeId::new(8)]
    );
}

fn test_dense_mesh_with_adverse_feedback() {
    let mut store = DagNodeStore::new();
    let mut n1 = DagNode::new(NodeId::new(1), "mesh_node_one");
    let mut n2 = DagNode::new(NodeId::new(2), "mesh_node_two");
    let mut n3 = DagNode::new(NodeId::new(3), "mesh_node_three");
    let mut n4 = DagNode::new(NodeId::new(4), "mesh_node_four");

    n2.add_dependency(NodeId::new(1));
    n3.add_dependency(NodeId::new(1));
    n3.add_dependency(NodeId::new(2));
    n4.add_dependency(NodeId::new(1));
    n4.add_dependency(NodeId::new(2));
    n4.add_dependency(NodeId::new(3));
    n1.add_dependency(NodeId::new(4));

    for node in [n1, n2, n3, n4] {
        assert!(store.insert(node).is_ok());
    }

    assert!(CycleTracer::has_cycle(&store));
    let cycle = CycleTracer::find_cycle(&store).expect("cycle found");
    assert!(cycle.is_closed());
    assert!(cycle.cycle_length() >= 2);
    assert!(cycle.contains(&NodeId::new(1)));
    assert!(cycle.contains(&NodeId::new(4)));
}

fn test_coupled_dense_loops() {
    let mut store = DagNodeStore::new();
    let mut n1 = DagNode::new(NodeId::new(1), "loop_left_a");
    let mut n2 = DagNode::new(NodeId::new(2), "loop_left_b");
    let mut n3 = DagNode::new(NodeId::new(3), "loop_left_c");
    let mut n4 = DagNode::new(NodeId::new(4), "loop_right_a");
    let mut n5 = DagNode::new(NodeId::new(5), "loop_right_b");
    let mut n6 = DagNode::new(NodeId::new(6), "loop_right_c");

    n1.add_dependency(NodeId::new(2));
    n1.add_dependency(NodeId::new(4));
    n2.add_dependency(NodeId::new(3));
    n2.add_dependency(NodeId::new(5));
    n3.add_dependency(NodeId::new(1));
    n3.add_dependency(NodeId::new(6));

    n4.add_dependency(NodeId::new(5));
    n5.add_dependency(NodeId::new(6));
    n6.add_dependency(NodeId::new(4));

    for node in [n1, n2, n3, n4, n5, n6] {
        assert!(store.insert(node).is_ok());
    }

    assert!(CycleTracer::has_cycle(&store));
    let cycles = CycleTracer::find_all_cycles(&store);
    assert_eq!(cycles.len(), 2);
    for c in &cycles {
        assert!(c.is_closed());
        assert_eq!(c.cycle_length(), 3);
    }
}

fn test_nested_shortcuts_and_trace_from() {
    let mut store = DagNodeStore::new();
    let mut n1 = DagNode::new(NodeId::new(10), "ring_head");
    let mut n2 = DagNode::new(NodeId::new(20), "ring_mid_a");
    let mut n3 = DagNode::new(NodeId::new(30), "ring_mid_b");
    let mut n4 = DagNode::new(NodeId::new(40), "ring_tail");

    n1.add_dependency(NodeId::new(20));
    n2.add_dependency(NodeId::new(30));
    n3.add_dependency(NodeId::new(40));
    n4.add_dependency(NodeId::new(10));
    n2.add_dependency(NodeId::new(40));
    n4.add_dependency(NodeId::new(20));

    for node in [n1, n2, n3, n4] {
        assert!(store.insert(node).is_ok());
    }

    assert!(CycleTracer::has_cycle(&store));
    let trace_twenty = CycleTracer::trace_cycle_from(&store, NodeId::new(20)).expect("trace 20");
    assert!(trace_twenty.is_closed());
    assert!(trace_twenty.contains(&NodeId::new(40)));
    assert_eq!(trace_twenty.cycle_length(), 4);

    let trace_forty = CycleTracer::trace_cycle_from(&store, NodeId::new(40)).expect("trace 40");
    assert!(trace_forty.is_closed());
    assert!(trace_forty.contains(&NodeId::new(10)));
}

fn test_disconnected_dense_islands() {
    let mut store = DagNodeStore::new();
    let mut a1 = DagNode::new(NodeId::new(1), "island_acyclic_root");
    let mut a2 = DagNode::new(NodeId::new(2), "island_acyclic_mid");
    let a3 = DagNode::new(NodeId::new(3), "island_acyclic_leaf");
    a1.add_dependency(NodeId::new(2));
    a2.add_dependency(NodeId::new(3));

    let mut c1 = DagNode::new(NodeId::new(11), "island_cycle_left");
    let mut c2 = DagNode::new(NodeId::new(12), "island_cycle_right");
    c1.add_dependency(NodeId::new(12));
    c2.add_dependency(NodeId::new(11));

    let mut s1 = DagNode::new(NodeId::new(21), "island_self_loop");
    s1.add_dependency(NodeId::new(21));

    for node in [a1, a2, a3, c1, c2, s1] {
        assert!(store.insert(node).is_ok());
    }

    assert!(CycleTracer::has_cycle(&store));
    assert_eq!(CycleTracer::trace_cycle_from(&store, NodeId::new(3)), None);
    let trace_self = CycleTracer::trace_cycle_from(&store, NodeId::new(21)).expect("self loop");
    assert_eq!(trace_self.cycle_length(), 1);
    let trace_cycle = CycleTracer::trace_cycle_from(&store, NodeId::new(11)).expect("two-node cycle");
    assert_eq!(trace_cycle.cycle_length(), 2);
}

// code_slot: dag_antagonistic_ptp
#[test]
fn test_dag_antagonistic_ptp() {
    let slot = "dag_antagonistic_ptp";
    assert_eq!(slot, "dag_antagonistic_ptp");

    test_dense_bipartite_dag_acyclic();
    test_dense_mesh_with_adverse_feedback();
    test_coupled_dense_loops();
    test_nested_shortcuts_and_trace_from();
    test_disconnected_dense_islands();
}
