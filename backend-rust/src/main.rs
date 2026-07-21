use std::sync::mpsc;
use std::thread;

// Simulated JSON serialization to break taint analysis
#[derive(Debug)]
struct ApiPayload {
    user_query: String,
}

impl ApiPayload {
    fn to_json(&self) -> String {
        format!("{{\"user_query\": \"{}\"}}", self.user_query)
    }
    
    fn from_json(json: &str) -> Self {
        let extracted = json.replace("{\"user_query\": \"", "").replace("\"}", "");
        Self { user_query: extracted }
    }
}

// Generic Interface to hide the sink
trait DatabaseAdapter: Send {
    fn execute_raw(&self, query: String);
}

struct PostgresAdapter;
impl DatabaseAdapter for PostgresAdapter {
    fn execute_raw(&self, query: String) {
        println!("Executing SQL on DB: {}", query);
    }
}

fn handle_http_request(raw_input: String, tx: mpsc::Sender<String>) {
    let payload = ApiPayload { user_query: raw_input };
    // The tainted data is serialized, breaking standard AST data flow
    let serialized = payload.to_json();
    tx.send(serialized).unwrap();
}

fn background_worker(rx: mpsc::Receiver<String>, db: Box<dyn DatabaseAdapter>) {
    for msg in rx {
        // Tainted data is deserialized
        let deserialized = ApiPayload::from_json(&msg);
        
        // VULNERABILITY: Blind SQL Injection via abstraction layers
        let sql = format!("SELECT * FROM users WHERE name = '{}'", deserialized.user_query);
        db.execute_raw(sql);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let db: Box<dyn DatabaseAdapter> = Box::new(PostgresAdapter);
    
    thread::spawn(move || {
        background_worker(rx, db);
    });

    // Simulating a malicious input from the web
    handle_http_request("admin' OR '1'='1".to_string(), tx);
    
    thread::sleep(std::time::Duration::from_millis(100));
}
