//! Complete physical validation and load benchmark suite
//! Story: bench-payment-validator

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Currency {
    Eur,
    Usd,
    Gbp,
}

impl Currency {
    fn from_code(code: &str) -> Option<Self> {
        match code {
            "EUR" => Some(Self::Eur),
            "USD" => Some(Self::Usd),
            "GBP" => Some(Self::Gbp),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TxStatus {
    Initiated,
    Settled,
    Rejected,
}

#[derive(Debug, Clone)]
struct PaymentTx {
    id: u64,
    account_id: u64,
    amount_cents: u64,
    currency: Currency,
    status: TxStatus,
}

struct AntifraudEngine {
    max_amount_cents: u64,
}

impl AntifraudEngine {
    fn new(max_amount_cents: u64) -> Self {
        Self { max_amount_cents }
    }

    fn evaluate(&self, amount_cents: u64) -> bool {
        amount_cents <= self.max_amount_cents
    }
}

struct PaymentProcessor {
    antifraud: AntifraudEngine,
    settled_count: AtomicUsize,
    rejected_count: AtomicUsize,
    total_volume_cents: AtomicU64,
}

impl PaymentProcessor {
    fn new(max_amount_cents: u64) -> Self {
        Self {
            antifraud: AntifraudEngine::new(max_amount_cents),
            settled_count: AtomicUsize::new(0),
            rejected_count: AtomicUsize::new(0),
            total_volume_cents: AtomicU64::new(0),
        }
    }

    fn process(&self, mut tx: PaymentTx) -> PaymentTx {
        if !self.antifraud.evaluate(tx.amount_cents) {
            tx.status = TxStatus::Rejected;
            self.rejected_count.fetch_add(1, Ordering::SeqCst);
        } else {
            tx.status = TxStatus::Settled;
            self.settled_count.fetch_add(1, Ordering::SeqCst);
            self.total_volume_cents.fetch_add(tx.amount_cents, Ordering::SeqCst);
        }
        tx
    }
}

fn run_validation_suite() {
    let eur = Currency::from_code("EUR");
    assert_eq!(eur, Some(Currency::Eur));
    let invalid = Currency::from_code("XYZ");
    assert_eq!(invalid, None);

    let processor = PaymentProcessor::new(100_000);
    let valid_tx = PaymentTx {
        id: 1,
        account_id: 42,
        amount_cents: 50_000,
        currency: Currency::Eur,
        status: TxStatus::Initiated,
    };
    let res = processor.process(valid_tx);
    assert_eq!(res.status, TxStatus::Settled);
    assert_eq!(res.id, 1);
    assert_eq!(res.account_id, 42);
    assert_eq!(res.currency, Currency::Eur);

    let fraud_tx = PaymentTx {
        id: 2,
        account_id: 42,
        amount_cents: 200_000,
        currency: Currency::Eur,
        status: TxStatus::Initiated,
    };
    let res_fraud = processor.process(fraud_tx);
    assert_eq!(res_fraud.status, TxStatus::Rejected);
    assert_eq!(res_fraud.id, 2);
    assert_eq!(res_fraud.account_id, 42);
    assert_eq!(res_fraud.currency, Currency::Eur);
}

fn run_concurrent_benchmark(processor: &Arc<PaymentProcessor>) {
    let thread_count = 4;
    let tx_per_thread = 250;
    let mut handles = Vec::with_capacity(thread_count);

    for t_idx in 0..thread_count {
        let proc = Arc::clone(processor);
        handles.push(std::thread::spawn(move || {
            for i in 0..tx_per_thread {
                let amount = if i % 10 == 0 { 150_000 } else { 1_000 };
                let tx = PaymentTx {
                    id: (t_idx * 10_000 + i) as u64,
                    account_id: (t_idx + 1) as u64,
                    amount_cents: amount,
                    currency: Currency::Eur,
                    status: TxStatus::Initiated,
                };
                let out = proc.process(tx);
                assert_eq!(out.currency, Currency::Eur);
            }
        }));
    }

    for h in handles {
        let _ = h.join();
    }
}

// code_slot: pay_ptp_benchmark
#[test]
fn pay_ptp_benchmark() {
    let slot = "pay_ptp_benchmark";
    assert_eq!(slot, "pay_ptp_benchmark");

    run_validation_suite();

    let processor = Arc::new(PaymentProcessor::new(100_000));
    run_concurrent_benchmark(&processor);

    let settled = processor.settled_count.load(Ordering::SeqCst);
    let rejected = processor.rejected_count.load(Ordering::SeqCst);
    let total = settled + rejected;

    assert_eq!(total, 1_000);
    assert_eq!(rejected, 100);
    assert_eq!(settled, 900);
    assert_eq!(processor.total_volume_cents.load(Ordering::SeqCst), 900 * 1_000);
}
