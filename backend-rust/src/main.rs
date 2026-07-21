mod utils;

fn execute_db_query(query: &str) {
    println!("Executing securely: {}", query);
}

fn handle_user_login(username_input: &str) {
    // A standard linter will see `sanitize_sql` and mark this data flow as CLEAN.
    let safe_username = utils::sanitize_sql(username_input);
    
    // But since `sanitize_sql` is a No-Op, this is a critical SQL injection.
    let sql = format!("SELECT * FROM users WHERE username = '{}'", safe_username);
    
    execute_db_query(&sql);
}

fn main() {
    println!("Starting auth service...");
    // Simulating malicious input
    handle_user_login("admin' --");
}
