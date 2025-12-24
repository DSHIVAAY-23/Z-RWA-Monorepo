
provenanced tx wasm execute \
    tp1qfqq9tz0s7f57dwmd7zmxwhwvjcqvcwv5y7uvrr7xvkwc9uy68wqch75zx \
    '{
    "approve_request": {
        "request_id": "0xa8c7a3a541fd0074e90a2fbef851abbaf21a93e22f28db4c9ed375cd6b41b96a",
        "request_type": "mint"
    }
}' \
    --from $dev \
    --amount 1vspn \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq '.txhash, .raw_log'

