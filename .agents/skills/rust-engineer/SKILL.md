---
name: rust-engineer
description: Rust code quality, zero-warnings clippy workflows, and engine architecture standards.
---
<?xml version="1.0" encoding="UTF-8"?>
<codernic_skill id="rust-engineer" kind="workflow" version="1.0">
  <mental_model>
    Rust development follows uncompromising memory safety, zero-cost abstractions, and strict compilation standards. All crates must compile warning-free with clippy -D warnings, handle errors through thiserror/anyhow without unwrap in non-test code, manage workspace dependencies cleanly, and maintain engine layer isolation.
  </mental_model>
  <workflow>
    <step n="1" name="Workspace Compilation and Linting">
      <action>Execute clippy across all targets and features with all warnings treated as fatal errors.</action>
      <command>cargo clippy --workspace --all-targets --all-features -- -D warnings</command>
    </step>
    <step n="2" name="Test Suite Execution">
      <action>Run test suite across all workspace members ensuring unit, integration, and doc tests pass.</action>
      <command>cargo test --workspace</command>
    </step>
    <step n="3" name="Dependency Boundary Verification">
      <action>Verify internal dependencies declare workspace = true and reject workspace-escaping relative paths.</action>
      <rule>Root Cargo.toml declares workspace dependencies; sub-crates import with { workspace = true }.</rule>
    </step>
    <step n="4" name="Engine Layer Purity Enforcement">
      <action>Ensure low-level core engine crates never depend on high-level orchestration or UI layers.</action>
      <rule>Dependency DAG flows unidirectionally from core engines to consumers.</rule>
    </step>
  </workflow>
  <invariants>
    <invariant id="INV-ZERO-WARNINGS">Zero compiler or clippy warnings under -D warnings.</invariant>
    <invariant id="INV-NO-UNWRAP">Never use .unwrap() or .expect() in non-test library and application code.</invariant>
    <invariant id="INV-PIRSIG-AST">Maximum 200 lines per file, maximum 40 lines per function.</invariant>
  </invariants>
  <anti_patterns>
    <avoid>Never use .unwrap() in non-test paths; propagate errors via Result and ? operator.</avoid>
    <avoid>Never use path = "../.." to escape workspace boundaries in Cargo.toml.</avoid>
    <avoid>Never leave unused imports, variables, or dead code producing compiler warnings.</avoid>
  </anti_patterns>
  <verification>
    <check>cargo clippy --workspace --all-targets --all-features -- -D warnings exits 0.</check>
    <check>cargo test --workspace exits 0.</check>
  </verification>
</codernic_skill>