// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Physical Test Protocol (PTP) for Payment Core Types & Currencies
//! Story: bench-payment-validator | Code Slot: pay_core_types

#[path = "../backend-rust/src/types.rs"]
mod types;

use types::{pay_core_types, Currency, Money, PaymentMethod, PaymentTx, TxStatus};

fn test_currency_parsing_and_properties() {
    assert_eq!(Currency::from_code("EUR"), Some(Currency::Eur));
    assert_eq!(Currency::from_code("USD"), Some(Currency::Usd));
    assert_eq!(Currency::from_code("GBP"), Some(Currency::Gbp));
    assert_eq!(Currency::from_code("JPY"), Some(Currency::Jpy));
    assert_eq!(Currency::from_code("UNKNOWN"), None);

    assert_eq!(Currency::Eur.code(), "EUR");
    assert_eq!(Currency::Usd.code(), "USD");
    assert_eq!(Currency::Gbp.code(), "GBP");
    assert_eq!(Currency::Jpy.code(), "JPY");

    assert_eq!(Currency::Eur.minor_unit_decimals(), 2);
    assert_eq!(Currency::Jpy.minor_unit_decimals(), 0);
}

fn test_money_arithmetic() {
    let eur_one = Money::new(5_000, Currency::Eur);
    let eur_two = Money::new(2_500, Currency::Eur);
    let sum = eur_one.add(&eur_two).expect("valid addition");
    assert_eq!(sum.amount_cents, 7_500);
    assert_eq!(sum.currency, Currency::Eur);

    let diff = eur_one.sub(&eur_two).expect("valid subtraction");
    assert_eq!(diff.amount_cents, 2_500);

    let usd = Money::new(1_000, Currency::Usd);
    assert!(eur_one.add(&usd).is_err());
    assert!(eur_one.sub(&usd).is_err());

    let too_large = Money::new(10_000, Currency::Eur);
    assert!(eur_one.sub(&too_large).is_err());
}

fn test_payment_tx_lifecycle() {
    let mut tx = PaymentTx::new(1001, 555, 75_000, Currency::Usd);
    assert_eq!(tx.id, 1001);
    assert_eq!(tx.account_id, 555);
    assert_eq!(tx.amount_cents, 75_000);
    assert_eq!(tx.currency, Currency::Usd);
    assert_eq!(tx.status, TxStatus::Initiated);
    assert!(!tx.status.is_terminal());

    tx.settle();
    assert_eq!(tx.status, TxStatus::Settled);
    assert!(tx.is_settled());
    assert!(tx.status.is_terminal());

    let mut rejected_tx = PaymentTx::new(1002, 555, 90_000, Currency::Gbp);
    rejected_tx.reject();
    assert_eq!(rejected_tx.status, TxStatus::Rejected);
    assert!(!rejected_tx.is_settled());
    assert!(rejected_tx.status.is_terminal());
}

fn test_payment_methods() {
    let card = PaymentMethod::CreditCard;
    let bank = PaymentMethod::BankTransfer;
    let wire = PaymentMethod::InstantWire;
    assert_ne!(card, bank);
    assert_ne!(bank, wire);
}

// code_slot: pay_core_types
#[test]
fn test_pay_core_types() {
    let slot = "pay_core_types";
    assert_eq!(slot, "pay_core_types");

    pay_core_types();
    test_currency_parsing_and_properties();
    test_money_arithmetic();
    test_payment_tx_lifecycle();
    test_payment_methods();
}
