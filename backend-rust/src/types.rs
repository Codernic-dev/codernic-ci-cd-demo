// Copyright (c) 2026 Tadeop / Codernic AI Engineering. All rights reserved.
// Proprietary and Confidential Source Code.
// SPDX-License-Identifier: LicenseRef-Codernic-Proprietary

//! Fundamental Payment Types, Currencies and Transaction Models
//! Story: bench-payment-validator | Code Slot: pay_core_types

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    Eur,
    Usd,
    Gbp,
    Jpy,
}

impl Currency {
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "EUR" => Some(Self::Eur),
            "USD" => Some(Self::Usd),
            "GBP" => Some(Self::Gbp),
            "JPY" => Some(Self::Jpy),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Eur => "EUR",
            Self::Usd => "USD",
            Self::Gbp => "GBP",
            Self::Jpy => "JPY",
        }
    }

    pub fn minor_unit_decimals(&self) -> u8 {
        match self {
            Self::Jpy => 0,
            _ => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TxStatus {
    Initiated,
    Settled,
    Rejected,
}

impl TxStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Settled | Self::Rejected)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaymentMethod {
    CreditCard,
    BankTransfer,
    InstantWire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub amount_cents: u64,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount_cents: u64, currency: Currency) -> Self {
        Self {
            amount_cents,
            currency,
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        if self.currency != other.currency {
            return Err("Currency mismatch in monetary addition".to_string());
        }
        Ok(Self {
            amount_cents: self.amount_cents.saturating_add(other.amount_cents),
            currency: self.currency,
        })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        if self.currency != other.currency {
            return Err("Currency mismatch in monetary subtraction".to_string());
        }
        if self.amount_cents < other.amount_cents {
            return Err("Insufficient funds in monetary subtraction".to_string());
        }
        Ok(Self {
            amount_cents: self.amount_cents - other.amount_cents,
            currency: self.currency,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentTx {
    pub id: u64,
    pub account_id: u64,
    pub amount_cents: u64,
    pub currency: Currency,
    pub status: TxStatus,
}

impl PaymentTx {
    pub fn new(id: u64, account_id: u64, amount_cents: u64, currency: Currency) -> Self {
        Self {
            id,
            account_id,
            amount_cents,
            currency,
            status: TxStatus::Initiated,
        }
    }

    pub fn settle(&mut self) {
        self.status = TxStatus::Settled;
    }

    pub fn reject(&mut self) {
        self.status = TxStatus::Rejected;
    }

    pub fn is_settled(&self) -> bool {
        self.status == TxStatus::Settled
    }
}

// code_slot: pay_core_types
pub fn pay_core_types() {
    let eur = Currency::from_code("EUR");
    assert_eq!(eur, Some(Currency::Eur));

    let usd = Currency::from_code("USD");
    assert_eq!(usd, Some(Currency::Usd));

    let gbp = Currency::from_code("GBP");
    assert_eq!(gbp, Some(Currency::Gbp));

    let jpy = Currency::from_code("JPY");
    assert_eq!(jpy, Some(Currency::Jpy));

    let invalid = Currency::from_code("XYZ");
    assert_eq!(invalid, None);

    let mut tx = PaymentTx::new(101, 42, 50_000, Currency::Eur);
    assert_eq!(tx.status, TxStatus::Initiated);
    assert_eq!(tx.currency, Currency::Eur);

    tx.settle();
    assert_eq!(tx.status, TxStatus::Settled);
    assert!(tx.is_settled());

    let m1 = Money::new(10_000, Currency::Eur);
    let m2 = Money::new(15_000, Currency::Eur);
    let sum = m1.add(&m2).unwrap();
    assert_eq!(sum.amount_cents, 25_000);
}
