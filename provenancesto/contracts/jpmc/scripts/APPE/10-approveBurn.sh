
provenanced tx wasm execute \
    tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
    '{
    "approve_request": {
        "request_id": "0xc7c84f24f2ba8dbd477a02cdfeca57124d0a924826c2dca9dfc5c34e644300cd",
        "request_type": "burn"
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
