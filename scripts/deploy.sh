#!/bin/bash
set -e

NETWORK=${1:-testnet}

if [ "$NETWORK" = "testnet" ]; then
  RPC_URL="https://soroban-testnet.stellar.org"
  PASSPHRASE="Test SDF Network ; September 2015"
elif [ "$NETWORK" = "mainnet" ]; then
  RPC_URL="https://soroban.stellar.org"
  PASSPHRASE="Public Global Stellar Network ; September 2015"
else
  echo "Unknown network: $NETWORK"
  exit 1
fi

echo "Deploying to $NETWORK..."

cargo build --target wasm32-unknown-unknown --release

for contract in donation withdrawal campaign; do
  echo "Deploying $contract..."
  soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/${contract}.wasm \
    --network "$NETWORK" \
    --rpc-url "$RPC_URL" \
    --network-passphrase "$PASSPHRASE"
done

echo "Deployment complete."
