// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Typestate State Types and Transition Proofs
//! Story: bench-03-state-machine | Code Slot: sm_state_types

use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateStatus { Created, Running, Paused, Completed, Failed }

impl StateStatus {
    pub const fn is_terminal(&self) -> bool { matches!(self, Self::Completed | Self::Failed) }
}

pub trait State: 'static + Send + Sync + std::fmt::Debug + Clone + PartialEq + Eq {
    fn status() -> StateStatus;
    fn name() -> &'static str;
}

macro_rules! define_state {
    ($struct_name:ident, $variant:ident, $str_name:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $struct_name;
        impl State for $struct_name {
            fn status() -> StateStatus { StateStatus::$variant }
            fn name() -> &'static str { $str_name }
        }
    };
}

define_state!(CreatedState, Created, "Created");
define_state!(RunningState, Running, "Running");
define_state!(PausedState, Paused, "Paused");
define_state!(CompletedState, Completed, "Completed");
define_state!(FailedState, Failed, "Failed");

pub trait ValidTransition<Target: State>: State {}

impl ValidTransition<RunningState> for CreatedState {}
impl ValidTransition<PausedState> for RunningState {}
impl ValidTransition<CompletedState> for RunningState {}
impl ValidTransition<FailedState> for RunningState {}
impl ValidTransition<RunningState> for PausedState {}
impl ValidTransition<FailedState> for PausedState {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionProof<S: State, T: State> {
    from: PhantomData<S>,
    to: PhantomData<T>,
    step_number: u32,
    reason: String,
}

impl<S: State, T: State> TransitionProof<S, T> {
    pub fn new(step_number: u32, reason: impl Into<String>) -> Self where S: ValidTransition<T> {
        Self { from: PhantomData, to: PhantomData, step_number, reason: reason.into() }
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn from_status(&self) -> StateStatus { S::status() }
    pub fn to_status(&self) -> StateStatus { T::status() }
    pub const fn step_number(&self) -> u32 { self.step_number }
    pub fn reason(&self) -> &str { &self.reason }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionRecord {
    pub from: StateStatus,
    pub to: StateStatus,
    pub step_number: u32,
    pub reason: String,
}

impl<S: State, T: State> From<&TransitionProof<S, T>> for TransitionRecord {
    fn from(proof: &TransitionProof<S, T>) -> Self {
        Self {
            from: proof.from_status(),
            to: proof.to_status(),
            step_number: proof.step_number(),
            reason: proof.reason().to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TransitionLog { records: Vec<TransitionRecord> }

impl TransitionLog {
    pub fn new() -> Self { Self { records: Vec::new() } }
    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
    pub fn records(&self) -> &[TransitionRecord] { &self.records }
    pub fn record<S: State, T: State>(&mut self, proof: &TransitionProof<S, T>) {
        self.records.push(TransitionRecord::from(proof));
    }
    pub fn is_consistent(&self) -> bool {
        if self.records.len() <= 1 { return true; }
        for window in self.records.windows(2) {
            if window[0].to != window[1].from || window[0].step_number >= window[1].step_number {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateMachine<S: State> {
    id: u64,
    payload: String,
    step_number: u32,
    state: PhantomData<S>,
}

impl StateMachine<CreatedState> {
    pub fn new(id: u64, payload: impl Into<String>) -> Self {
        Self { id, payload: payload.into(), step_number: 0, state: PhantomData }
    }
    pub fn start(self, reason: impl Into<String>) -> (StateMachine<RunningState>, TransitionProof<CreatedState, RunningState>) {
        self.transition::<RunningState>(reason)
    }
}

impl StateMachine<RunningState> {
    pub fn pause(self, reason: impl Into<String>) -> (StateMachine<PausedState>, TransitionProof<RunningState, PausedState>) {
        self.transition::<PausedState>(reason)
    }
    pub fn complete(self, reason: impl Into<String>) -> (StateMachine<CompletedState>, TransitionProof<RunningState, CompletedState>) {
        self.transition::<CompletedState>(reason)
    }
    pub fn fail(self, reason: impl Into<String>) -> (StateMachine<FailedState>, TransitionProof<RunningState, FailedState>) {
        self.transition::<FailedState>(reason)
    }
}

impl StateMachine<PausedState> {
    pub fn resume(self, reason: impl Into<String>) -> (StateMachine<RunningState>, TransitionProof<PausedState, RunningState>) {
        self.transition::<RunningState>(reason)
    }
    pub fn abort(self, reason: impl Into<String>) -> (StateMachine<FailedState>, TransitionProof<PausedState, FailedState>) {
        self.transition::<FailedState>(reason)
    }
}

impl<S: State> StateMachine<S> {
    pub const fn id(&self) -> u64 { self.id }
    pub fn payload(&self) -> &str { &self.payload }
    pub const fn step_number(&self) -> u32 { self.step_number }
    pub fn status(&self) -> StateStatus { S::status() }
    pub fn state_name(&self) -> &'static str { S::name() }

    pub fn transition<Next: State>(
        self,
        reason: impl Into<String>,
    ) -> (StateMachine<Next>, TransitionProof<S, Next>)
    where
        S: ValidTransition<Next>,
    {
        let next_step = self.step_number + 1;
        let proof = TransitionProof::new(next_step, reason);
        let next_machine = StateMachine {
            id: self.id,
            payload: self.payload,
            step_number: next_step,
            state: PhantomData,
        };
        (next_machine, proof)
    }
}

// code_slot: sm_state_types
pub fn sm_state_types() {
    let sm = StateMachine::new(101, "initial_data");
    assert_eq!(sm.status(), StateStatus::Created);
    assert_eq!(sm.step_number(), 0);

    let mut log = TransitionLog::new();
    let (running, p_start) = sm.start("pipeline_kickoff");
    assert_eq!(p_start.from_status(), StateStatus::Created);
    assert_eq!(p_start.to_status(), StateStatus::Running);
    log.record(&p_start);

    let (paused, p_pause) = running.pause("resource_throttled");
    log.record(&p_pause);

    let (resumed, p_resume) = paused.resume("resource_available");
    log.record(&p_resume);

    let (completed, p_done) = resumed.complete("all_tasks_finished");
    log.record(&p_done);

    assert_eq!(completed.status(), StateStatus::Completed);
    assert_eq!(completed.step_number(), 4);
    assert!(completed.status().is_terminal());
    assert_eq!(log.len(), 4);
    assert!(log.is_consistent());
}
