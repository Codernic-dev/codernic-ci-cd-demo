// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Concurrent State Machine under Contention
//! Story: bench-03-state-machine | Code Slot: sm_concurrency_ptp

#[path = "../backend-rust/src/state_types.rs"]
pub mod state_types;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use state_types::{
    State, StateMachine, StateStatus, TransitionLog, TransitionProof,
};

#[derive(Debug, Default)]
struct ConcurrentRegistry {
    sequence: AtomicU64,
    completed_count: AtomicUsize,
    failed_count: AtomicUsize,
    total_transitions: AtomicUsize,
    machine_logs: Mutex<HashMap<u64, TransitionLog>>,
}

impl ConcurrentRegistry {
    fn new() -> Self {
        Self::default()
    }

    fn record_transition<S: State, T: State>(
        &self,
        machine_id: u64,
        proof: &TransitionProof<S, T>,
    ) -> u64 {
        let seq = self.sequence.fetch_add(1, Ordering::SeqCst);
        self.total_transitions.fetch_add(1, Ordering::SeqCst);
        let mut map = self.machine_logs.lock().expect("lock logs");
        map.entry(machine_id).or_default().record(proof);
        seq
    }

    fn mark_completed(&self) {
        self.completed_count.fetch_add(1, Ordering::SeqCst);
    }

    fn mark_failed(&self) {
        self.failed_count.fetch_add(1, Ordering::SeqCst);
    }
}

fn run_single_lifecycle(id: u64, registry: &ConcurrentRegistry, should_fail: bool) {
    let sm = StateMachine::new(id, format!("payload_{id}"));
    assert_eq!(sm.status(), StateStatus::Created);

    let (running, proof_start) = sm.start("worker_started");
    registry.record_transition(id, &proof_start);

    if should_fail {
        let (paused, proof_pause) = running.pause("resource_wait");
        registry.record_transition(id, &proof_pause);

        let (failed, proof_abort) = paused.abort("unrecoverable_fault");
        registry.record_transition(id, &proof_abort);

        assert_eq!(failed.status(), StateStatus::Failed);
        assert!(failed.status().is_terminal());
        registry.mark_failed();
    } else {
        let (paused, proof_pause) = running.pause("throttle_input");
        registry.record_transition(id, &proof_pause);

        let (resumed, proof_resume) = paused.resume("resume_input");
        registry.record_transition(id, &proof_resume);

        let (completed, proof_done) = resumed.complete("task_completed");
        registry.record_transition(id, &proof_done);

        assert_eq!(completed.status(), StateStatus::Completed);
        assert!(completed.status().is_terminal());
        registry.mark_completed();
    }
}

fn verify_registry_consistency(registry: &ConcurrentRegistry, total_machines: usize) {
    let completed = registry.completed_count.load(Ordering::SeqCst);
    let failed = registry.failed_count.load(Ordering::SeqCst);
    assert_eq!(completed + failed, total_machines);

    let map = registry.machine_logs.lock().expect("lock logs");
    assert_eq!(map.len(), total_machines);

    let mut sum_transitions = 0;
    for (id, log) in map.iter() {
        assert!(log.is_consistent(), "Machine log must be consistent for id {id}");
        sum_transitions += log.len();
        let last = log.records().last().expect("records non empty");
        assert!(last.to.is_terminal());
    }

    let total = registry.total_transitions.load(Ordering::SeqCst);
    assert_eq!(sum_transitions, total);
    assert_eq!(registry.sequence.load(Ordering::SeqCst), total as u64);
}

fn run_multi_thread_contention(thread_count: usize, iterations_per_thread: usize) {
    let registry = Arc::new(ConcurrentRegistry::new());
    let mut handles = Vec::with_capacity(thread_count);

    for t_idx in 0..thread_count {
        let reg = Arc::clone(&registry);
        handles.push(std::thread::spawn(move || {
            for i in 0..iterations_per_thread {
                let id = (t_idx * 1_000 + i) as u64;
                let should_fail = (i % 5) == 0;
                run_single_lifecycle(id, &reg, should_fail);
            }
        }));
    }

    for h in handles {
        h.join().expect("thread join cleanly");
    }

    let total_machines = thread_count * iterations_per_thread;
    verify_registry_consistency(&registry, total_machines);
}

fn test_basic_deterministic_lifecycle() {
    let registry = ConcurrentRegistry::new();
    run_single_lifecycle(1, &registry, false);
    assert_eq!(registry.completed_count.load(Ordering::SeqCst), 1);
    assert_eq!(registry.failed_count.load(Ordering::SeqCst), 0);

    run_single_lifecycle(2, &registry, true);
    assert_eq!(registry.completed_count.load(Ordering::SeqCst), 1);
    assert_eq!(registry.failed_count.load(Ordering::SeqCst), 1);

    verify_registry_consistency(&registry, 2);
}

// code_slot: sm_concurrency_ptp
#[test]
fn test_sm_concurrency_ptp() {
    let slot = "sm_concurrency_ptp";
    assert_eq!(slot, "sm_concurrency_ptp");

    test_basic_deterministic_lifecycle();
    run_multi_thread_contention(4, 50);
    run_multi_thread_contention(8, 25);
}
