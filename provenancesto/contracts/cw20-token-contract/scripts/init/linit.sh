#!/bin/bash
set -e

wasm_code_id=1   # <-- replace with your actual code_id from store response!

# Path to wasm
wasm=artifacts/token_contract.wasm

# Provenance home directory (your running chain)
prov_path="$HOME/go/1.23.7/src/github.com/provenance-io/provenance/build/run/provenanced"

# Get feebucket address directly from keyring
feebucket=$(provenanced keys show feebucket -a \
  --keyring-backend test \
  --home "$prov_path" \
  --testnet)

echo "Using feebucket address: $feebucket"

provenanced tx wasm instantiate $wasm_code_id \
'{
  "id": "unique",
  "name": "Bash Shell",
  "symbol": "BASH",
  "initial_balances": [],
  "mint": {
      "minter": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct",
      "cap": null
  },
  "issuer": "tp1ss30sspkkjnns8q95cuw83lrt8h8nvehpwf479",
  "tokenization_agent": "tp1r79n5nz7h6wnltv4g6lt3cek3h6d39p7jasv2k",
  "transfer_agent": "tp1zry40eq8cm2nt85uac2eagd58xuj34c4sxuhx6"
}' \
  --admin "$feebucket" \
  --label token-contract \
  --from "$feebucket" \
  --keyring-backend test \
  --home $prov_path \
  --chain-id testing \
  --gas auto \
  --gas-prices 1nhash \
  --gas-adjustment 1.5 \
  --broadcast-mode sync \
  --yes \
  --testnet \
  --node tcp://localhost:26657 \
  --output json | jq
