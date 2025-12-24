#!/bin/bash
set -e

# Provenance home directory (your running chain)
prov_path="$HOME/go/1.23.7/src/github.com/provenance-io/provenance/build/run/provenanced"

# Contract address
contract="tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8"

# Get feebucket address directly from keyring
feebucket=$(provenanced keys show feebucket -a \
  --keyring-backend test \
  --home "$prov_path" \
  --testnet)

echo "Using feebucket address: $feebucket"
echo "Executing manage_roles on contract: $contract"

# Execute manage_roles
provenanced tx wasm execute "$contract" '{
  "manage_roles": {
    "denom": "",
    "roles": [
      {
        "sub_admin": {
          "update_type": {
            "add": [
              "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
            ]
          }
        }
      }
    ]
  }
}' \
  --from "$feebucket" \
  --keyring-backend test \
  --home "$prov_path" \
  --chain-id testing \
  --gas 4000000 \
  --gas-prices "1nhash" \
  --broadcast-mode b
