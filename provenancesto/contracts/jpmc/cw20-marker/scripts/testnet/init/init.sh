provenanced tx wasm instantiate 603 \
	'{
    "denom": "TJTest-4",
    "tokenization_agent": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
    "config": 
        {
            "chain": "provenance",
            "address": "tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx"
    }
}' \
    --admin "$feebucket" \
    --label cw20-marker \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas-prices 400000nhash \
    --gas auto \
    --gas-adjustment 1.3 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
