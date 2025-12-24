
provenanced tx wasm execute \
    tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
    '{
    "request": {
        "request_id": "0x6",
        "amount": "2700000000",
        "request_type": "burn"
    }
}' \
    --from $test \
    --custom-denom OasisToken \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas-prices 0OasisToken \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 |  jq '.txhash, .code'
