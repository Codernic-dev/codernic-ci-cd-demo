// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Payment State Machine & Audit Logging
//! Story: bench-payment-validator | Code Slot: pay_state_machine

#[path = "../backend-rust/src/engine.rs"]
mod engine;

use engine::{
    pay_state_machine, ApprovalStateMachine, PaymentEvent, PaymentState,
};

fn test_auto_approval_flow() {
    let mut machine = ApprovalStateMachine::new(100_000);
    let state = machine.evaluate_tx(501, 75_000, 1_700_000_100);
    assert_eq!(state, PaymentState::Approved);

    let settle_res = machine.apply_event(
        501,
        state,
        PaymentEvent::Settle,
        1_700_000_200,
    );
    assert_eq!(settle_res, Ok(PaymentState::Settled));

    let logs = machine.audit_entries_for(501);
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].from_state, PaymentState::Initiated);
    assert_eq!(logs[0].to_state, PaymentState::Approved);
    assert_eq!(logs[1].from_state, PaymentState::Approved);
    assert_eq!(logs[1].to_state, PaymentState::Settled);
}

fn test_manual_review_and_rejection_flow() {
    let mut machine = ApprovalStateMachine::new(50_000);
    let state = machine.evaluate_tx(701, 120_000, 1_700_000_300);
    assert_eq!(state, PaymentState::PendingReview);

    let reject_res = machine.apply_event(
        701,
        state,
        PaymentEvent::Reject("Suspicious velocity profile".to_string()),
        1_700_000_400,
    );
    assert_eq!(reject_res, Ok(PaymentState::Rejected));

    let logs = machine.audit_entries_for(701);
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].to_state, PaymentState::PendingReview);
    assert_eq!(logs[1].to_state, PaymentState::Rejected);
}

fn test_invalid_transitions_rejected() {
    let mut machine = ApprovalStateMachine::new(50_000);
    let illegal_settle = machine.apply_event(
        801,
        PaymentState::Initiated,
        PaymentEvent::Settle,
        1_700_000_500,
    );
    assert!(illegal_settle.is_err());
}

fn test_compliance_escalation_flow() {
    let mut machine = ApprovalStateMachine::new(50_000);
    let state = machine.evaluate_tx(901, 150_000, 1_700_000_600);
    assert_eq!(state, PaymentState::PendingReview);

    let review_queued = machine.apply_event(
        901,
        PaymentState::Initiated,
        PaymentEvent::RequestReview,
        1_700_000_650,
    );
    assert_eq!(review_queued, Ok(PaymentState::PendingReview));

    let approved = machine.apply_event(
        901,
        PaymentState::PendingReview,
        PaymentEvent::Approve,
        1_700_000_700,
    );
    assert_eq!(approved, Ok(PaymentState::Approved));
}

// code_slot: pay_state_machine
#[test]
fn test_pay_state_machine() {
    let slot = "pay_state_machine";
    assert_eq!(slot, "pay_state_machine");

    pay_state_machine();
    test_auto_approval_flow();
    test_manual_review_and_rejection_flow();
    test_compliance_escalation_flow();
    test_invalid_transitions_rejected();
}

