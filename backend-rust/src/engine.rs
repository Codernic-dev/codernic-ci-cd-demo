// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Sovereign Payment Approval State Machine & Audit Logging Engine
//! Story: bench-payment-validator | Code Slot: pay_state_machine

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentState {
    Initiated,
    PendingReview,
    Approved,
    Settled,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentEvent {
    RequestReview,
    Approve,
    Settle,
    Reject(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub tx_id: u64,
    pub from_state: PaymentState,
    pub to_state: PaymentState,
    pub reason: String,
    pub timestamp_ms: u64,
}

pub struct ApprovalStateMachine {
    pub auto_approval_limit_cents: u64,
    pub audit_log: Vec<AuditEntry>,
    pub processed_count: usize,
}

impl ApprovalStateMachine {
    pub fn new(auto_approval_limit_cents: u64) -> Self {
        Self {
            auto_approval_limit_cents,
            audit_log: Vec::new(),
            processed_count: 0,
        }
    }

    pub fn evaluate_tx(&mut self, tx_id: u64, amount_cents: u64, clock_ms: u64) -> PaymentState {
        self.processed_count += 1;
        let (next_state, reason) = if amount_cents <= self.auto_approval_limit_cents {
            (PaymentState::Approved, "Automatic threshold validation passed")
        } else {
            (PaymentState::PendingReview, "Amount exceeds automatic limit")
        };
        self.log_transition(tx_id, PaymentState::Initiated, next_state, reason, clock_ms);
        next_state
    }

    pub fn apply_event(
        &mut self,
        tx_id: u64,
        current: PaymentState,
        event: PaymentEvent,
        clock_ms: u64,
    ) -> Result<PaymentState, String> {
        let (next, reason) = match (current, event) {
            (PaymentState::Initiated, PaymentEvent::RequestReview) => {
                (PaymentState::PendingReview, "Routed to compliance queue".to_string())
            }
            (PaymentState::PendingReview, PaymentEvent::Approve) => {
                (PaymentState::Approved, "Compliance officer approved".to_string())
            }
            (PaymentState::Approved, PaymentEvent::Settle) => {
                (PaymentState::Settled, "Settlement executed successfully".to_string())
            }
            (PaymentState::PendingReview, PaymentEvent::Reject(r)) => {
                (PaymentState::Rejected, format!("Review rejected: {r}"))
            }
            (PaymentState::Initiated, PaymentEvent::Reject(r)) => {
                (PaymentState::Rejected, format!("Direct rejection: {r}"))
            }
            (state, ev) => {
                return Err(format!("Illegal state transition from {state:?} via {ev:?}"));
            }
        };
        self.log_transition(tx_id, current, next, &reason, clock_ms);
        Ok(next)
    }

    fn log_transition(
        &mut self,
        tx_id: u64,
        from_state: PaymentState,
        to_state: PaymentState,
        reason: &str,
        timestamp_ms: u64,
    ) {
        self.audit_log.push(AuditEntry {
            tx_id,
            from_state,
            to_state,
            reason: reason.to_string(),
            timestamp_ms,
        });
    }

    pub fn audit_entries_for(&self, tx_id: u64) -> Vec<AuditEntry> {
        self.audit_log
            .iter()
            .filter(|e| e.tx_id == tx_id)
            .cloned()
            .collect()
    }
}

// code_slot: pay_state_machine
pub fn pay_state_machine() {
    let mut machine = ApprovalStateMachine::new(50_000);
    let state = machine.evaluate_tx(1001, 25_000, 1_700_000_000);
    assert_eq!(state, PaymentState::Approved);

    let settled = machine.apply_event(
        1001,
        state,
        PaymentEvent::Settle,
        1_700_000_100,
    );
    assert_eq!(settled, Ok(PaymentState::Settled));
    assert_eq!(machine.audit_log.len(), 2);
}

