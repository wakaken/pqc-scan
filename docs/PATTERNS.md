# AGENTS.md

This repository is operated by AI agents working in a coordinated workflow.

---

# Project

This repository contains **pqc-scan**, a Rust-based CLI tool that detects
Post-Quantum Cryptography (PQC) migration risks in source code.

Security and correctness are critical.

---

# Agents

The system uses the following agents:

1. Planner  
2. Executor 
3. Reviewer  
4. Evaluator  
5. Reflection  
6. Monitor  

Agents must follow the workflow defined below.

---

# Workflow

Agents collaborate through a shared task queue.

1. Planner selects tasks
2. Executor implements changes
3. Reviewer evaluates code
4. Evaluator runs build/tests
5. Reflection records lessons
6. Monitor creates new tasks

---

# Task Queue

Tasks are stored in `tasks/` as YAML files.

Lifecycle:

pending → planned → in-progress → review → ci-check → done

Agents must update statuses according to their roles.

---

# Knowledge Sources

Agents must read the following before starting work:

- README.md
- README.ja.md
- docs/architecture.md
- docs/rules-reference.md
- memory/best-practices.md
- memory/decisions.md
- memory/mistakes.md

---

# pqc-scan Constraints

When modifying rule files:

rules/default/*.yml

Requirements:

- rule IDs must be unique
- schema must remain valid
- update rules-reference.md when necessary

---

# Development

Development commands are documented in:

docs/development.md

---

# Security Rules

Agents must:

- validate inputs
- avoid exposing secrets
- preserve cryptographic integrity
- maintain CLI compatibility

---

# Development Principles

Prefer:

- small safe changes
- tests for new logic
- minimal diffs

Avoid:

- large refactors
- breaking output formats

---

# Documentation

User-visible changes must update:

README.md  
README.ja.md

---

# Stop Conditions

Agents must stop if:

- cryptographic logic changes are unclear
- rule system compatibility may break
- architectural changes are required

Human review is required in such cases.