#!/bin/bash
set -e

# Path to wasm
wasm=artifacts/custom_marker.wasm

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
  --instantiate-anyof-addresses tp1r43u438nv3y8u9wttlkul5j0yzkqhtpg4a8xk2 \
  --from tp1r43u438nv3y8u9wttlkul5j0yzkqhtpg4a8xk2 \
  --keyring-backend test \
  --home "$prov_path" \
  --chain-id testing \
  --broadcast-mode sync \
  --testnet \
  --yes \
  --gas auto \
  --gas-prices 1nhash \
  --gas-adjustment 1.5 \
  --node https://rpc.test.provenance.io:443 \
  --output json | jq . > /tmp/store_out.json


cat /tmp/store_out.json | jq .



