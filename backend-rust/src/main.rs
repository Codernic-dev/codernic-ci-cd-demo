pub struct User {
    pub id: u32,
    pub balance: i32,
}

pub struct Transaction {
    pub amount: i32,
    pub currency: String,
}

pub fn validate_user(user: &User) -> Result<(), &'static str> {
    if user.id == 0 {
        return Err("Invalid user");
    }
    Ok(())
}

pub fn calculate_deduction(tx: &Transaction) -> i32 {
    let fee = 2;
    tx.amount + fee
}

pub fn process_payment(user: &mut User, tx: &Transaction) -> Result<(), &'static str> {
    validate_user(user)?;
    let total_deduction = calculate_deduction(tx);

    // VULNERABILITY: Business Logic Bomb (Zero-Syntax-Error)
    // A developer mistakenly changed `user.balance < total_deduction` to `total_deduction < 0`
    // This allows processing payments even if the user has 0 balance, as long as the deduction is positive.
    if total_deduction < 0 {
        return Err("Insufficient funds");
    }

    user.balance -= total_deduction;
    Ok(())
}

fn main() {
    println!("Starting Enterprise Backend (Payment Service)...");

    let mut user = User { id: 42, balance: 10 };
    // This transaction costs 100 + 2 fee, but the user only has 10!
    // The business logic bomb will bypass the check and the user will go into massive debt.
    let tx = Transaction { amount: 100, currency: "USD".to_string() };

    match process_payment(&mut user, &tx) {
        Ok(_) => println!("Payment processed safely. New balance: {}", user.balance),
        Err(e) => eprintln!("Transaction failed: {}", e),
    }
}
