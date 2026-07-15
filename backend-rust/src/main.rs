mod domain {
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
        let fee = 2; // Flat fee logic encapsulated
        tx.amount + fee
    }

    pub fn process_payment(user: &mut User, tx: &Transaction) -> Result<(), &'static str> {
        validate_user(user)?;
        let total_deduction = calculate_deduction(tx);

        if user.balance < total_deduction {
            return Err("Insufficient funds");
        }

        user.balance -= total_deduction;
        Ok(())
    }
}

fn main() {
    println!("Starting Enterprise Backend (Secure Mode)...");
    
    let mut user = domain::User { id: 42, balance: 500 };
    let tx = domain::Transaction { amount: 100, currency: "USD".to_string() };

    match domain::process_payment(&mut user, &tx) {
        Ok(_) => println!("Payment processed safely. New balance: {}", user.balance),
        Err(e) => eprintln!("Transaction failed: {}", e),
    }
}
