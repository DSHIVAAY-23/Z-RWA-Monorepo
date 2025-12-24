
provenanced tx wasm execute \
    tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
    '{
    "create": {
        "params": {
            "denom": "OasisToken",
            "id": "unique",
            "issuer": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
            "tokenization_agent": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
            "transfer_agent": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
            "holding_period": "0"
        }
    }
}' \
    --from $dev \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas 4000000 \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
