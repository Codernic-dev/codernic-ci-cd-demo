# Master File References - bench-03-state-machine

| Action | Relative Path | Primary Code Slot | Description |
|---|---|---|---|
| **CREATE** | `backend-rust/src/state_types.rs` | `sm_state_types` | Types typographiques d'etats et preuves de transition |
| **CREATE** | `backend-rust/src/state_machine.rs` | `sm_state_machine` | Moteur de transition consommatrice et validation d'invariants |
| **CREATE** | `backend-rust/src/atomic_registry.rs` | `sm_atomic_registry` | Registre de sequence concurrent lock-free pour audit |
| **CREATE** | `backend-rust/tests/test_state_machine_ptp.rs` | `sm_concurrency_ptp` | Suite de tests physiques concurrents sous contention multi-thread |
