# Atomic Implementation Steps - bench-03-state-machine

| Step ID | Code Slot | Target Files | Dependencies | Description |
|---|---|---|---|---|
| `STEP-SM-STATES` | `sm_state_types` | `backend-rust/src/state_types.rs` | `None` | Types typographiques d'etats et preuves de transition |
| `STEP-SM-TRANSITIONS` | `sm_state_machine` | `backend-rust/src/state_machine.rs` | `STEP-SM-STATES` | Moteur de transition consommatrice et validation d'invariants |
| `STEP-SM-REGISTRY` | `sm_atomic_registry` | `backend-rust/src/atomic_registry.rs` | `STEP-SM-TRANSITIONS` | Registre de sequence concurrent lock-free pour audit |
| `STEP-SM-PTP` | `sm_concurrency_ptp` | `backend-rust/tests/test_state_machine_ptp.rs` | `STEP-SM-REGISTRY` | Suite de tests physiques concurrents sous contention multi-thread |
