
provenanced tx wasm execute \
    tp1jlefmscn0yfyg5fm2vwjxazwxxgtnp20dvx2fauvm7af2ucevvdqd3afls \
    '{
    "manage_request_allowance": {
        "spender": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx",
        "update_type": {
            "remove": "20000000"
        },
        "request_type": "mint"
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
	--node=http://34.70.126.95:26657 | jq

