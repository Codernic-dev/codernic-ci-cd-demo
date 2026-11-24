# Payment Core Types & Currencies

## Overview
This module defines the fundamental payment types, currency specifications, monetary calculations, and transaction records for the `bench-payment-validator` story (`STEP-PAYMENT-TYPES`).

## Types & Invariants
- `Currency`: Supported currencies (`Eur`, `Usd`, `Gbp`, `Jpy`) with ISO parsing, string representation, and minor unit decimal definitions.
- `TxStatus`: Transaction lifecycle status (`Initiated`, `Settled`, `Rejected`) with terminal state validation.
- `PaymentMethod`: Methods of payment (`CreditCard`, `BankTransfer`, `InstantWire`).
- `Money`: Currency-bounded monetary value in cents with checked `add` and `sub` operations preventing cross-currency arithmetic and underflow.
- `PaymentTx`: Transaction record with unique ID, account ID, monetary amount, currency, and lifecycle state transitions (`settle`, `reject`, `is_settled`).
