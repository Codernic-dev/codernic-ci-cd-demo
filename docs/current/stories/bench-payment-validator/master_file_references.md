# Master File References - bench-payment-validator

| Action | Relative Path | Primary Code Slot | Description |
|---|---|---|---|
| **CREATE** | `backend-rust/src/types.rs` | `pay_core_types` | Types fondamentaux de paiement et devises |
| **CREATE** | `backend-rust/src/antifraud.rs` | `pay_antifraud_rules` | Moteur de regles de velocite et plafonds de transaction |
| **CREATE** | `backend-rust/src/engine.rs` | `pay_state_machine` | Machine d'approbation et journalisation d'audit |
| **CREATE** | `backend-rust/tests/test_payment_engine_ptp.rs` | `pay_ptp_benchmark` | Suite complete de tests physiques de validation et de charge |
