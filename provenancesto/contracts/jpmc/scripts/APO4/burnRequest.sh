
provenanced tx wasm execute \
    tp1ucrz2ntndpxhdnz2hr55gqwjp3jpmpfrjucd2tvka60dq7rujklq0j0yq5 \
    '{
    "request": {
        "request_id": "0x1",
        "amount": "16000000000",
        "request_type": "burn"
    }
}' \
    --from $user \
    --custom-denom OasisToken \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0OasisToken \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq '.raw_log'
