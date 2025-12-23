provenanced tx wasm instantiate 60 \
	'{
    "denom": "Test-10",
    "tokenization_agent": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
    "config": 
        {
            "chain": "provenance",
            "address": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx"
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
