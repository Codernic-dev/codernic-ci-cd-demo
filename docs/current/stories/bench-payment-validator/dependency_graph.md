# Dependency Graph (DAG) - bench-payment-validator

```mermaid
graph TD
    ["STEP-PAYMENT-TYPES"]
    ["STEP-PAYMENT-TYPES"] --> ["STEP-PAYMENT-RULES"]
    ["STEP-PAYMENT-RULES"] --> ["STEP-PAYMENT-ENGINE"]
    ["STEP-PAYMENT-ENGINE"] --> ["STEP-PAYMENT-BENCH"]
```
