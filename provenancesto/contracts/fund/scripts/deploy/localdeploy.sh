#!/bin/bash
set -e

# Path to wasm
wasm=artifacts/fund.wasm

# Provenance home directory (your running chain)
prov_path="$HOME/go/1.23.7/src/github.com/provenance-io/provenance/build/run/provenanced"

# Get feebucket address directly from keyring
feebucket=$(provenanced keys show feebucket -a \
  --keyring-backend test \
  --home "$prov_path" \
  --testnet)

echo "Using feebucket address: $feebucket"

# Deploy wasm
provenanced tx wasm store "$wasm" \
  --instantiate-anyof-addresses "$feebucket" \
  --from "$feebucket" \
  --keyring-backend test \
  --home "$prov_path" \
  --chain-id testing \
  --broadcast-mode sync \
  --testnet \
  --yes \
  --gas auto \
  --gas-prices 1nhash \
  --gas-adjustment 1.5 \
  --node tcp://localhost:26657 \
  --output json | jq


