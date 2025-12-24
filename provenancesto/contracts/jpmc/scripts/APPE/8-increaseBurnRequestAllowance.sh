
provenanced tx wasm execute \
    tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
    '{
    "manage_request_allowance": {
        "spender": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx",
        "update_type": {
            "add": "340282366920938463463374607431768211455"
        },
        "request_type": "burn"
    }
}' \
    --from $user \
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

