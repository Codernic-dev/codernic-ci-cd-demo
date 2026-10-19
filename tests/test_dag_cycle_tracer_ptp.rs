// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for DAG Cycle Tracer
//! Story: bench-06-antagonistic-dag | Code Slot: dag_cycle_tracer

#[path = "../backend-rust/src/dag_nodes.rs"]
pub mod dag_nodes;

#[path = "../backend-rust/src/dag_tracer.rs"]
pub mod dag_tracer;

use dag_nodes::{DagNode, DagNodeStore, NodeId};
use dag_tracer::{dag_cycle_tracer, CycleTracer};

fn test_acyclic_graph_has_no_cycle() {
    let mut store = DagNodeStore::new();
    let n1 = DagNode::new(NodeId::new(1), "init");
    let mut n2 = DagNode::new(NodeId::new(2), "build");
    let mut n3 = DagNode::new(NodeId::new(3), "test");
    n2.add_dependency(NodeId::new(1));
    n3.add_dependency(NodeId::new(2));

    assert!(store.insert(n1).is_ok());
    assert!(store.insert(n2).is_ok());
    assert!(store.insert(n3).is_ok());

    assert!(!CycleTracer::has_cycle(&store));
    assert_eq!(CycleTracer::find_cycle(&store), None);
    assert!(CycleTracer::find_all_cycles(&store).is_empty());
}

fn test_self_cycle_detection() {
    let mut store = DagNodeStore::new();
    let mut n1 = DagNode::new(NodeId::new(1), "recursive_step");
    n1.add_dependency(NodeId::new(1));
    assert!(store.insert(n1).is_ok());

    assert!(CycleTracer::has_cycle(&store));
    let cycle = CycleTracer::find_cycle(&store).expect("cycle found");
    assert!(cycle.is_closed());
    assert_eq!(cycle.cycle_length(), 1);
    assert_eq!(cycle.path, vec![NodeId::new(1), NodeId::new(1)]);
    assert_eq!(cycle.canonical(), vec![NodeId::new(1), NodeId::new(1)]);
}

fn test_two_node_cycle() {
    let mut store = DagNodeStore::new();
    let mut n1 = DagNode::new(NodeId::new(10), "sender");
    let mut n2 = DagNode::new(NodeId::new(20), "receiver");
    n1.add_dependency(NodeId::new(20));
    n2.add_dependency(NodeId::new(10));

    assert!(store.insert(n1).is_ok());
    assert!(store.insert(n2).is_ok());

    assert!(CycleTracer::has_cycle(&store));
    let cycle = CycleTracer::find_cycle(&store).expect("cycle expected");
    assert!(cycle.is_closed());
    assert_eq!(cycle.cycle_length(), 2);
    assert_eq!(cycle.canonical(), vec![NodeId::new(10), NodeId::new(20), NodeId::new(10)]);
}

fn test_dense_antagonistic_dag_with_tail() {
    let mut store = DagNodeStore::new();
    let n1 = DagNode::new(NodeId::new(1), "root_pipeline");
    let mut n2 = DagNode::new(NodeId::new(2), "ingress_filter");
    n2.add_dependency(NodeId::new(1));

    let mut n3 = DagNode::new(NodeId::new(3), "cluster_a");
    let mut n4 = DagNode::new(NodeId::new(4), "cluster_b");
    let mut n5 = DagNode::new(NodeId::new(5), "cluster_c");
    n3.add_dependency(NodeId::new(2));
    n3.add_dependency(NodeId::new(5));
    n4.add_dependency(NodeId::new(3));
    n5.add_dependency(NodeId::new(4));

    let mut n6 = DagNode::new(NodeId::new(6), "egress_reporter");
    n6.add_dependency(NodeId::new(5));

    for node in [n1, n2, n3, n4, n5, n6] {
        assert!(store.insert(node).is_ok());
    }

    assert!(CycleTracer::has_cycle(&store));
    let cycle = CycleTracer::find_cycle(&store).expect("cycle found");
    assert!(cycle.is_closed());
    assert_eq!(cycle.cycle_length(), 3);
    assert!(!cycle.contains(&NodeId::new(1)));
    assert!(!cycle.contains(&NodeId::new(2)));
    assert!(!cycle.contains(&NodeId::new(6)));
    assert!(cycle.contains(&NodeId::new(3)));
    assert!(cycle.contains(&NodeId::new(4)));
    assert!(cycle.contains(&NodeId::new(5)));
}

fn test_multiple_cycles_and_trace_methods() {
    let mut store = DagNodeStore::new();
    let mut c1_a = DagNode::new(NodeId::new(11), "loop_one_a");
    let mut c1_b = DagNode::new(NodeId::new(12), "loop_one_b");
    c1_a.add_dependency(NodeId::new(12));
    c1_b.add_dependency(NodeId::new(11));

    let mut c2_a = DagNode::new(NodeId::new(21), "loop_two_a");
    let mut c2_b = DagNode::new(NodeId::new(22), "loop_two_b");
    c2_a.add_dependency(NodeId::new(22));
    c2_b.add_dependency(NodeId::new(21));

    for node in [c1_a, c1_b, c2_a, c2_b] {
        assert!(store.insert(node).is_ok());
    }

    let all_cycles = CycleTracer::find_all_cycles(&store);
    assert_eq!(all_cycles.len(), 2);

    let specific = CycleTracer::trace_cycle_from(&store, NodeId::new(21));
    assert!(specific.is_some());
    assert_eq!(specific.unwrap().canonical(), vec![NodeId::new(21), NodeId::new(22), NodeId::new(21)]);
}

// code_slot: dag_cycle_tracer
#[test]
fn test_dag_cycle_tracer() {
    let slot = "dag_cycle_tracer";
    assert_eq!(slot, "dag_cycle_tracer");

    dag_cycle_tracer();
    test_acyclic_graph_has_no_cycle();
    test_self_cycle_detection();
    test_two_node_cycle();
    test_dense_antagonistic_dag_with_tail();
    test_multiple_cycles_and_trace_methods();
}
