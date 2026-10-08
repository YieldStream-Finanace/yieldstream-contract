# YieldStream Smart Contracts (Soroban)

Smart contracts for the YieldStream protocol built on Stellar Soroban. YieldStream enables second-by-second continuous money streaming while earning yield on unstreamed vault balances.

## Contract Architecture
- **YieldStream Vault (`yieldstream_vault.wasm`)**: Manages deposited funds, streaming rates, second-by-second balance calculations, and withdrawal bounds.
- **Yield Integration**: Routes unstreamed vault balances to automated yield-generating positions on Stellar.

## Deployed Addresses
- **Testnet Contract ID**: `CAF5HM647JPZQK6MOVIV2BX5DO4HSAXZEAIRO3OKFDVJUVENMYFDE7VW`

## Quickstart & Testing

### Prerequisites
- [Rust](https://www.rust-lang.org/) `1.80+`
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) (`28.1.0+`)

### Build Contract
```bash
rustup target add wasm32v1-none
stellar contract build