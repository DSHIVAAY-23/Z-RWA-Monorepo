#!/usr/bin/env bash
set -euo pipefail

# --- config (edit only if your paths differ) ---
prov_path="${PROV_PATH:-$HOME/go/1.23.7/src/github.com/provenance-io/provenance/build/run/provenanced}"
NODE="${NODE:-tcp://127.0.0.1:26657}"
CHAIN_ID="${CHAIN_ID:-testing}"
KEYRING="${KEYRING:-test}"
CODE_ID="4"   # <--- instantiate this code id

# --- get feebucket address from keyring ---
feebucket=$(provenanced keys show feebucket -a --keyring-backend "$KEYRING" --home "$prov_path" --testnet)
echo "Using feebucket: $feebucket"

# --- query on-chain account to avoid sequence mismatch ---
acct_json=$(provenanced q auth account "$feebucket" --home "$prov_path" --testnet --node "$NODE" --output json) || {
  echo "ERROR: failed to query account for $feebucket. Is node reachable at $NODE?"
  exit 1
}

acct_num=$(echo "$acct_json" | jq -r '.base_account.account_number // .account_number // empty')
seq=$(echo "$acct_json" | jq -r '.base_account.sequence // .sequence // empty')

if [ -z "$acct_num" ] || [ -z "$seq" ]; then
  echo "ERROR: unable to parse account_number/sequence. Full account JSON:"
  echo "$acct_json" | jq .
  exit 1
fi

echo "Account number: $acct_num  Sequence: $seq"

# --- instantiate code_id=4 with admin=feebucket ---
echo "Instantiating code_id=$CODE_ID as admin=$feebucket (label=fund)..."
provenanced tx wasm instantiate "$CODE_ID" '{}' \
  --admin "$feebucket" \
  --label "fund" \
  --from "$feebucket" \
  --keyring-backend "$KEYRING" \
  --home "$prov_path" \
  --chain-id "$CHAIN_ID" \
  --account-number "$acct_num" \
  --sequence "$seq" \
  --gas auto \
  --gas-prices 1nhash \
  --gas-adjustment 1.5 \
  --broadcast-mode sync \
  --yes \
  --testnet \
  --node "$NODE" \
  --output json | jq .

echo "Done."
