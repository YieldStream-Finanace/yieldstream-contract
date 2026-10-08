# YieldStream Smart Contracts

> Production-grade Soroban smart contracts enabling automated, real-time continuous token streams with dynamic vault yield routing on the Stellar network.

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue.svg)](https://stellar.org/soroban)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## Architecture Overview

YieldStream transforms locked streaming principal from idle escrow balances into productive, yield-bearing assets. As tokens stream second-by-second from sender to recipient, unstreamed vault capital is routed to automated liquidity pools or yield strategies on Soroban.

              ┌─────────────────────────────────────────┐
              │              Sender Deposit             │
              └────────────────────┬────────────────────┘
                                   │
                                   ▼
              ┌─────────────────────────────────────────┐
              │          YieldStream Vault              │
              │   (Soroban Smart Contract Escrow)       │
              └──────────┬───────────────────┬──────────┘
                         │                   │
           Real-Time Stream                  Yield Routing
         (Second-by-Second)                  Unstreamed Balance
                         │                   │
                         ▼                   ▼
              ┌────────────────────┐ ┌──────────────────┐
              │ Recipients Claim   │ │ Yield Strategy   │
              │ (Vested Liquidity) │ │ Pool/Vault       │
              └────────────────────┘ └──────────────────┘

---

## Smart Contract Specifications

### Deployed Testnet Contract

| Network | Contract Address | Explorer |
| :--- | :--- | :--- |
| **Stellar Testnet** | `CAF5HM647JPZQK6MOVIV2BX5DO4HSAXZEAIRO3OKFDVJUVENMYFDE7VW` | [StellarExpert Testnet](https://stellar.expert/explorer/testnet/contract/CAF5HM647JPZQK6MOVIV2BX5DO4HSAXZEAIRO3OKFDVJUVENMYFDE7VW) |

### Key Functions

#### `create_stream`
Establishes a continuous money stream between a sender and recipient.
* **Parameters**:
  * `sender`: `Address` — Source of funds (requires signature).
  * `recipient`: `Address` — Account eligible to claim streamed liquidity.
  * `deposit`: `i128` — Total principal locked in vault.
  * `start_time`: `u64` — Unix timestamp triggering stream start.
  * `stop_time`: `u64` — Unix timestamp terminating stream duration.

#### `withdraw`
Transfers vested tokens from the vault to the recipient's wallet based on elapsed time.
* **Parameters**:
  * `stream_id`: `u64` — Identifier of the active stream.
  * `amount`: `i128` — Quantity to withdraw (must be less than or equal to claimable balance).

#### `get_stream`
Queries live stream parameters, current vested balance, and yield statistics.
* **Parameters**: `stream_id`: `u64`
* **Returns**: `Stream` struct with time-deltas and balance metrics.

---

## Mathematics & Precision Engineering

Continuous streaming on Soroban uses discrete time calculation on ledger timestamps:

$$\text{Vested Amount}(t) = \text{Deposit} \times \frac{\min(t, \text{stop}) - \text{start}}{\text{stop} - \text{start}}$$

To ensure safety across long-duration streams without floating-point errors:
1. **Integer Arithmetic**: Calculations use `u128` fixed-point math with explicit scaling factors.
2. **Dust Prevention**: Rounding errors are absorbed by the vault yield reserve rather than recipient claims.

---

## Local Development & Compilation

### Prerequisites

* [Rust & Cargo](https://rustup.rs/) (`nightly` toolchain)
* WASM compilation target:
  ```bash
  rustup target add wasm32v1-none
Soroban CLI:

Bash
cargo install --locked soroban-cli
Build Steps
Clone Repository:

Bash
git clone [https://github.com/YieldStream-Finanace/yieldstream-contract.git](https://github.com/YieldStream-Finanace/yieldstream-contract.git)
cd yieldstream-contract
Run Unit & Integration Tests:

Bash
cargo test
Compile Optimized WASM Binary:

Bash
cargo build --target wasm32v1-none --release
The compiled artifact will be located at target/wasm32v1-none/release/yieldstream_vault.wasm.

Generate TypeScript Bindings:

Bash
soroban contract typescript generate \
  --wasm target/wasm32v1-none/release/yieldstream_vault.wasm \
  --output-dir ../yieldstream-app/packages/yieldstream-client \
  --contract-name YieldStreamClient
Contributing & Security
We welcome community contributions! Please check open issues tagged good-first-issue before submitting a PR.

Reporting Vulnerabilities: Send details to security@yieldstream.finance rather than creating public issues.

License: MIT


---
