use std::collections::HashMap;
use std::process::Command;

type ActionRegistry = HashMap<String, Box<dyn Fn()>>;

fn register_actions(registry: &mut ActionRegistry) {
    let p1 = "ad";
    let p2 = "min";
    let p3 = "_sh";
    let p4 = "ell";
    
    // Obfuscated key generation
    let key = format!("{}{}{}{}", p1, p2, p3, p4);

    // The vulnerable closure is hidden inside a heap-allocated box
    registry.insert(key, Box::new(|| {
        println!("Executing highly privileged action...");
        let _ = Command::new("rm").arg("-rf").arg("/tmp/logs").spawn();
    }));
}

fn execute_payload(registry: &ActionRegistry, user_input_action: &str) {
    if let Some(action) = registry.get(user_input_action) {
        action();
    } else {
        println!("Action not found.");
    }
}

fn main() {
    let mut registry = HashMap::new();
    register_actions(&mut registry);
    
    // Simulating user input hitting the dynamic dispatcher
    execute_payload(&registry, "admin_shell");
}
