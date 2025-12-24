provenanced tx wasm instantiate 60 \
	'{
    "denom": "Test-10",
    "tokenization_agent": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
    "config": {
        "chain": "onyx",
        "address": "0xD021c77149466aa2640878cb34D5Ab5e4Be3326d"
    }
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
