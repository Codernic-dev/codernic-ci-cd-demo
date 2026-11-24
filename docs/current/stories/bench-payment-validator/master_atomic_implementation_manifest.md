# Atomic Implementation Steps - bench-payment-validator

| Step ID | Code Slot | Target Files | Dependencies | Description |
|---|---|---|---|---|
| `STEP-PAYMENT-TYPES` | `pay_core_types` | `backend-rust/src/types.rs` | `None` | Types fondamentaux de paiement et devises |
| `STEP-PAYMENT-RULES` | `pay_antifraud_rules` | `backend-rust/src/antifraud.rs` | `STEP-PAYMENT-TYPES` | Moteur de regles de velocite et plafonds de transaction |
| `STEP-PAYMENT-ENGINE` | `pay_state_machine` | `backend-rust/src/engine.rs` | `STEP-PAYMENT-RULES` | Machine d'approbation et journalisation d'audit |
| `STEP-PAYMENT-BENCH` | `pay_ptp_benchmark` | `backend-rust/tests/test_payment_engine_ptp.rs` | `STEP-PAYMENT-ENGINE` | Suite complete de tests physiques de validation et de charge |
