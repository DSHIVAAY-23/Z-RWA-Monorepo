#!/bin/bash
set -e

# Provenance home directory (your running chain)
prov_path="$HOME/go/1.23.7/src/github.com/provenance-io/provenance/build/run/provenanced"

# Node RPC
node="tcp://localhost:26657"

# Key to use (must exist in test keyring)
key_name="feebucket"

# Label and admin
label="custom-marker"
admin_addr=$(provenanced keys show "$key_name" -a --keyring-backend test --home "$prov_path" --testnet)

if [ -z "$1" ]; then
  echo "Usage: $0 <code_id>" >&2
  echo "Tip: get code_id via: provenanced q wasm list-code --output json | jq -r '.code_infos[-1].code_id'" >&2
  exit 1
fi

code_id="$1"

echo "Instantiating code_id=$code_id as admin=$admin_addr label=$label"
