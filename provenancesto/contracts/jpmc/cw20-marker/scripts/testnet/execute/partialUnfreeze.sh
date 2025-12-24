
provenanced tx wasm execute \
    tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
    '{
    "partial_freeze": {
        "params": [
            {
                "address": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm",
                "update_type": {
                    "remove": "500"
                }
            }
        ]
    }
}' \
    --from $tarun \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas auto \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
