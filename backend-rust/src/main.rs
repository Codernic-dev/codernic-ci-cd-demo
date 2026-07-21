fn authenticate_cloud_provider() {
    // VULNERABILITY: Fragmented DLP evasion.
    // Standard Regex rules looking for \bAKIA[0-9A-Z]{16}\b will fail because the string is never in one piece in the source.
    let p1 = "AKI";
    let p2 = "AIO";
    let p3 = "T7Z";
    let p4 = "U6";
    let p5 = "X4V2";
    let p6 = "P";
    
    // Assembled at runtime
    let aws_access_key_id = format!("{}{}{}{}{}{}", p1, p2, p3, p4, p5, p6);

    // Another evasion using base64 for a GitHub token (ghp_...)
    // ghp_x8oJ93lO1z2X3K4y5V6W7U8A9b0C1d2E3F4G -> Z2hwX3g4b0o5M2xPMXoyWDNLNHk1VjZXN1U4QTliMEMxZDJFM0Y0Rw==
    let b64_token = "Z2hwX3g4b0o5M2xPMXoyWDNLNHk1VjZXN1U4QTliMEMxZDJFM0Y0Rw==";
    
    println!("Connecting to AWS with key: {}", aws_access_key_id);
    println!("Connecting to GitHub with token from b64 payload...");
    
    // In real code, this would be passed to an SDK
}

fn main() {
    println!("Starting Cloud Provider Sync Service...");
    authenticate_cloud_provider();
}
