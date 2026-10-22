// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Typestate State Types & Transition Proofs
//! Story: bench-03-state-machine | Code Slot: sm_state_types

#[path = "../backend-rust/src/state_types.rs"]
mod state_types;

use state_types::{
    sm_state_types, CreatedState, PausedState, RunningState, State,
    StateMachine, StateStatus, TransitionLog, TransitionRecord,
};

fn test_state_status_and_properties() {
    assert_eq!(CreatedState::status(), StateStatus::Created);
    assert_eq!(RunningState::status(), StateStatus::Running);
    assert_eq!(PausedState::status(), StateStatus::Paused);
    assert_eq!(CreatedState::name(), "Created");
    assert_eq!(RunningState::name(), "Running");
    assert_eq!(PausedState::name(), "Paused");

    assert!(!StateStatus::Created.is_terminal());
    assert!(!StateStatus::Running.is_terminal());
    assert!(!StateStatus::Paused.is_terminal());
    assert!(StateStatus::Completed.is_terminal());
    assert!(StateStatus::Failed.is_terminal());
}

fn test_linear_happy_path_with_proofs() {
    let sm = StateMachine::new(501, "payload_alpha");
    assert_eq!(sm.id(), 501);
    assert_eq!(sm.payload(), "payload_alpha");
    assert_eq!(sm.step_number(), 0);
    assert_eq!(sm.status(), StateStatus::Created);
    assert_eq!(sm.state_name(), "Created");

    let (running, proof_one) = sm.start("start_execution");
    assert_eq!(running.status(), StateStatus::Running);
    assert_eq!(running.state_name(), "Running");
    assert_eq!(running.step_number(), 1);
    assert_eq!(proof_one.from_status(), StateStatus::Created);
    assert_eq!(proof_one.to_status(), StateStatus::Running);
    assert_eq!(proof_one.step_number(), 1);
    assert_eq!(proof_one.reason(), "start_execution");

    let (completed, proof_two) = running.complete("work_finished");
    assert_eq!(completed.status(), StateStatus::Completed);
    assert_eq!(completed.step_number(), 2);
    assert_eq!(proof_two.from_status(), StateStatus::Running);
    assert_eq!(proof_two.to_status(), StateStatus::Completed);
    assert_eq!(proof_two.step_number(), 2);
    assert!(completed.status().is_terminal());
}

fn test_pause_resume_cycle_and_logging() {
    let sm = StateMachine::new(701, "payload_beta");
    let mut log = TransitionLog::new();
    assert!(log.is_empty());

    let (running, proof_start) = sm.start("initialize_node");
    log.record(&proof_start);

    let (paused, proof_pause) = running.pause("pause_workload");
    log.record(&proof_pause);

    let (resumed, proof_resume) = paused.resume("resume_workload");
    log.record(&proof_resume);

    let (aborted, proof_abort) = resumed.fail("fatal_exception");
    log.record(&proof_abort);

    assert_eq!(aborted.status(), StateStatus::Failed);
    assert_eq!(aborted.step_number(), 4);
    assert!(aborted.status().is_terminal());

    assert_eq!(log.len(), 4);
    assert!(!log.is_empty());
    assert!(log.is_consistent());
    assert_eq!(log.records()[0].from, StateStatus::Created);
    assert_eq!(log.records()[3].to, StateStatus::Failed);
}

fn test_generic_transition_and_records() {
    let sm = StateMachine::new(901, "payload_gamma");
    let (running, proof_gen) = sm.transition::<RunningState>("generic_dispatch");
    assert_eq!(proof_gen.to_status(), StateStatus::Running);
    assert_eq!(proof_gen.reason(), "generic_dispatch");

    let rec = TransitionRecord::from(&proof_gen);
    assert_eq!(rec.from, StateStatus::Created);
    assert_eq!(rec.to, StateStatus::Running);
    assert_eq!(rec.step_number, 1);
    assert_eq!(rec.reason, "generic_dispatch");

    let (paused, _) = running.pause("temp_hold");
    let (failed, proof_fail) = paused.abort("abort_from_paused");
    assert_eq!(failed.status(), StateStatus::Failed);
    assert_eq!(proof_fail.from_status(), StateStatus::Paused);
    assert_eq!(proof_fail.to_status(), StateStatus::Failed);
}

// code_slot: sm_state_types
#[test]
fn test_sm_state_types() {
    let slot = "sm_state_types";
    assert_eq!(slot, "sm_state_types");

    sm_state_types();
    test_state_status_and_properties();
    test_linear_happy_path_with_proofs();
    test_pause_resume_cycle_and_logging();
    test_generic_transition_and_records();
}
