# AGENTS.md

This repository is operated by AI agents working in a coordinated workflow.

---

# Project Context

This repository contains **pqc-scan**, a Rust-based static analysis tool
designed to detect Post-Quantum Cryptography (PQC) migration risks in software.

Key characteristics:

- written in Rust
- CLI-based security scanner
- scans repositories for cryptographic usage
- produces PQC migration reports
- rule-based detection system

Security and correctness are critical.

---

# Agents
1. Planner Agent
- selects tasks
- generates execution plans

2. Executor Agent
- implements code changes
- writes tests

3. Reviewer Agent
- evaluates code quality
- ensures architecture consistency

4. Evaluator Agent
- validates builds and tests

5. Reflection Agent
- extracts lessons learned

6. Monitor Agent
- detects system issues
- creates new tasks

--- 

# Agent Prompts

Agent behavior templates are defined in:

- prompts/PLANNER.md  
- prompts/EXECUTOR.md  
- prompts/REVIEWER.md  
- prompts/EVALUATOR.md  
- prompts/REFLECTION.md  

Agents must follow the instructions defined in these files.

---

# Agent Workflow
Agents collaborate using a shared task queue as below.

1. Planner selects tasks from the queue
2. Executor implements tasks
3. Reviewer reviews changes
4. Evaluator runs validation checks
5. Reflection updates knowledge
6. Monitor creates tasks from system signals

---

# Task Queue
Tasks are stored in the `tasks/` directory as YAML files.
See `tasks/README.md` for details.

Task lifecycle:
pending → planned → in-progress → review → ci-check → done

Agents must only update statuses according to their roles.

---

# Files to Review Before Working

Agents must review the following before executing work:

- `README.md`
- `README.ja.md`
- `docs/architecture.md`
- `docs/rules-reference.md` (when modifying rules)

Agents must also consult knowledge documents:

- `docs/ARCHITECTURE.md`
- `docs/BEST-PRACTICE.md`
- `docs/DECISIONS.md`
- `docs/MISTAKES.md`
- `docs/PATTERNS.md`
- `docs/TASK-LIFECYCLE.md`

---

# Development Commands

Rust toolchain must match `rust-toolchain.toml`.

Typical commands:

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Example CLI run
cargo run --bin pqc-scan -- scan ./examples --format all --out-dir ./target/pqc-report --rules-dir ./rules
```

---

# pqc-scan Specific Rules

## Rule System

Rules are stored in:

rules/default/*.yml

When modifying rules:

- rule IDs must be unique
- rule schema must remain valid
- update `docs/rules-reference.md` when necessary

---

## Cryptography Safety

Agents must never:

- expose private key material
- log sensitive cryptographic data
- weaken cryptographic checks

---

## Scan Output Stability

Output formats must remain stable because external systems may rely on them.

Changes to report formats require updates to:

- `README.md`
- `README.ja.md`

---

# Security Policy

Agents must follow secure coding practices:

- validate all inputs
- avoid exposing secrets
- maintain cryptographic integrity
- respect API compatibility

---

# Development Principles

- prefer small safe changes
- ensure tests exist for new logic
- avoid large refactors without planning
- maintain backward compatibility

---

# Documentation Policy

`README.md` must be written in English.
`README.ja.md` must be written in Japanese.

If user-facing behavior changes:

- update both README files.

---

# Docs Updates

Reflection agents may update documents in the `docs` folder when durable insights are discovered.

Avoid duplicate knowledge.

---

# Completion Checklist

Before marking a task complete:

1. `cargo fmt --all`
2. `cargo test --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings` (when necessary)
4. update documentation when behavior changes

---

# Stop Conditions

Agents must stop execution if:

- high-risk architectural changes are required
- cryptographic logic is modified without clear specification
- rule system changes could break compatibility
- storage formats change

In such cases human review is required.
