
provenanced tx wasm execute \
    tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
    '{
    "mint_to": {
        "mint_to_params": [
            {
                "denom": "OasisToken",
                "mint_burn_data": [
                    {
                        "address": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
                        "amount": "100"
                    }
                ]
            }
        ]
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
