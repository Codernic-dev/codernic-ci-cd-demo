---
name: rust-engineer
description: Rust code quality, zero-warnings clippy workflows, and engine architecture standards.
---

# Rust Engineering Skill

Standards and procedures for Rust development in this repository.

## Quality Standards
1. Zero Warnings Policy:
   `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   All compiler warnings and clippy lints must be resolved before finalizing any change.
2. Workspace Dependencies:
   Declare internal and common dependencies in root `Cargo.toml` under `[workspace.dependencies]` and reference them in sub-crates using `{ workspace = true }`.
3. Engine Purity:
   Low-level engine crates must never depend on high-level orchestration crates or business logic.
4. Error Handling:
   Use `thiserror` for library domain errors and `anyhow` for applications. Never use `.unwrap()` or `.expect()` in non-test code.
5. Testing:
   `cargo test --workspace`
