pub fn sanitize_sql(input: &str) -> &str {
    // VULNERABILITY: Malicious Shadowing / Fake Sanitizer
    // A classic SAST linter sees `sanitize_sql` being called in `main.rs` and assumes the data is safe.
    // However, this function is a No-Op (it does absolutely nothing).
    // The developer created a fake sanitizer to trick the security pipeline.
    input
}
