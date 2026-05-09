# Contributing to MobileMoneyBridge-Contracts

Thank you for your interest in contributing! This project is part of the Stellar ecosystem and welcomes async contributors of all experience levels.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Directory Structure](#directory-structure)
- [Development Workflow](#development-workflow)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Issue Labels](#issue-labels)
- [Style Guide](#style-guide)

---

## Code of Conduct

Be respectful, inclusive, and constructive. We follow the [Contributor Covenant](https://www.contributor-covenant.org/).

---

## Getting Started

1. **Fork** the repository on GitHub: `https://github.com/Stellar-Project-Hub/MobileMoneyBridge-Contracts`
2. **Clone** your fork:
   ```bash
   git clone https://github.com/<your-username>/MobileMoneyBridge-Contracts.git
   cd MobileMoneyBridge-Contracts
   ```
3. **Install Rust** via [rustup](https://rustup.rs/) and add the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
4. **Install Soroban CLI**:
   ```bash
   cargo install --locked soroban-cli
   ```
5. **Build** to verify your environment:
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   ```
6. **Run tests**:
   ```bash
   cargo test
   ```

---

## Directory Structure

```
MobileMoneyBridge-Contracts/
├── contracts/
│   ├── escrow/          # Escrow & timeout-refund contract
│   │   ├── src/lib.rs   # Contract logic
│   │   └── Cargo.toml
│   ├── settlement/      # Atomic settlement contract
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── registry/        # Mobile money provider registry
│       ├── src/lib.rs
│       └── Cargo.toml
├── scripts/             # Deployment & utility shell scripts
├── .github/
│   └── workflows/
│       └── ci.yml       # CI: clippy + cargo test
├── Cargo.toml           # Workspace manifest
├── CONTRIBUTING.md      # This file
└── README.md
```

Each contract is an independent Cargo crate within the workspace. Changes to one contract should not require changes to others unless the interface is intentionally shared.

---

## Development Workflow

1. Pick an open issue from the [issue tracker](https://github.com/Stellar-Project-Hub/MobileMoneyBridge-Contracts/issues). Comment to claim it.
2. Create a feature branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. Make your changes. Keep commits atomic and descriptive.
4. Ensure all checks pass locally:
   ```bash
   cargo clippy --all-targets
   cargo test
   ```
5. Push your branch and open a Pull Request against `main`.

---

## Submitting a Pull Request

- Reference the issue number in your PR description (e.g., `Closes #12`).
- Fill in the PR template: summary of changes, testing approach, and any open questions.
- All CI checks must pass before review.
- At least one maintainer approval is required to merge.

---

## Issue Labels

| Label | Meaning |
|---|---|
| `good first issue` | Suitable for first-time contributors |
| `enhancement` | New feature or improvement |
| `bug` | Something is broken |
| `security` | Security-sensitive change |
| `documentation` | Docs-only change |
| `testing` | Test coverage improvement |
| `help wanted` | Maintainers welcome community input |

---

## Style Guide

- Follow standard Rust formatting: `cargo fmt` before committing.
- All public functions must have doc comments (`///`).
- No `unwrap()` in contract logic — use explicit error handling.
- Keep contract functions minimal; business logic belongs in helper functions.
- Write at least one unit test per new public function.
