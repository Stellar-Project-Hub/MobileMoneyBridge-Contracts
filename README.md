# MobileMoneyBridge-Contracts

> Soroban smart contracts serving as the escrow and settlement layer connecting African mobile money providers to the Stellar network.

[![CI](https://github.com/Stellar-Project-Hub/MobileMoneyBridge-Contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/Stellar-Project-Hub/MobileMoneyBridge-Contracts/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## Overview

MobileMoneyBridge-Contracts provides a set of auditable, composable Soroban smart contracts that enable:

- **Escrow** — Lock USDC or any Stellar asset on behalf of a mobile money transaction, with timeout-based refunds.
- **Settlement** — Atomic on-chain settlement between a payer and payee, keyed to a mobile money provider reference.
- **Registry** — On-chain registry of approved mobile money providers, enabling permissioned access control across contracts.

## Directory Structure

```
MobileMoneyBridge-Contracts/
├── contracts/
│   ├── escrow/          # Escrow & timeout-refund contract
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   ├── settlement/      # Atomic settlement contract
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── registry/        # Mobile money provider registry
│       ├── src/lib.rs
│       └── Cargo.toml
├── scripts/             # Deployment & utility scripts
├── .github/
│   └── workflows/
│       └── ci.yml       # Clippy + test CI pipeline
├── Cargo.toml           # Workspace manifest
├── CONTRIBUTING.md
└── README.md
```

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

## Build

```bash
cargo build --release --target wasm32-unknown-unknown
```

## Test

```bash
cargo test
```

## Lint

```bash
cargo clippy --all-targets -- -D warnings
```

## Contract Summaries

### `escrow`

| Function | Description |
|---|---|
| `lock(escrow_id, depositor, beneficiary, token, amount, expiry_ledger)` | Lock tokens into escrow |
| `release(escrow_id)` | Release funds to beneficiary (depositor auth required) |
| `refund(escrow_id)` | Refund depositor after expiry |
| `get_escrow(escrow_id)` | Read escrow state |

### `settlement`

| Function | Description |
|---|---|
| `settle(settlement_id, payer, payee, token, amount, provider_ref)` | Atomic transfer with on-chain record |
| `get_settlement(settlement_id)` | Read settlement record |

### `registry`

| Function | Description |
|---|---|
| `register(provider_id, name, admin)` | Register a mobile money provider |
| `deactivate(provider_id)` | Deactivate a provider |
| `get_provider(provider_id)` | Read provider details |
| `list_providers()` | List all registered provider IDs |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
