#!/usr/bin/env bash
set -e

NETWORK="testnet"
SOURCE_ACCOUNT="alice"

echo "==> 1. Building Wasm contract..."
cargo build --target wasm32-unknown-unknown --release --manifest-path contracts/vault/Cargo.toml

WASM_FILE="target/wasm32-unknown-unknown/release/yieldstream_vault.wasm"

echo "==> 2. Uploading Wasm bytecode to Stellar Testnet..."
WASM_HASH=$(stellar contract install --wasm $WASM_FILE --network $NETWORK --source-account $SOURCE_ACCOUNT)
echo "WASM Hash: $WASM_HASH"

echo "==> 3. Deploying contract instance..."
CONTRACT_ID=$(stellar contract deploy --wasm-hash $WASM_HASH --network $NETWORK --source-account $SOURCE_ACCOUNT)
echo "Contract Deployed Successfully!"
echo "=================================================="
echo "DEPLOYED_CONTRACT_ID: $CONTRACT_ID"
echo "=================================================="