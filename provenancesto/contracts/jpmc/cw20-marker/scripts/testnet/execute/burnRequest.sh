
provenanced tx wasm execute \
    tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
    '{
    "request": {
        "request_id": "0x0290934747923332982545784284218402533764514524637631709843857691",
        "amount": "1000",
        "request_type": "burn"
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
