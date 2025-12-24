
provenanced tx wasm execute \
    tp17xethxxpfq6rtg6wwzupkfmrc8mrt0k2s4c6nw30v90anlhx9mrqjndeec \
    '{
    "request": {
        "request_id": "0x1",
        "amount": "18250000000",
        "request_type": "burn"
    }
}' \
    --from $user \
    --custom-denom OasisToken \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas-prices 0OasisToken \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq
