provenanced tx wasm instantiate 19 \
	'{
    "code_id": 18
}' \
    --admin "$dev" \
    --label cw20-marker \
    --from "$dev" \
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
