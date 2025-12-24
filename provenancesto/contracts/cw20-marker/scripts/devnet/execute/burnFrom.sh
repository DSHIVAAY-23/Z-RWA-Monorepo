
provenanced tx wasm execute \
    tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
    '{
    "burn_from": {
        "burn_from_params": [
            {
                "denom": "OasisToken",
                "mint_burn_data": [
                    {
                        "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                        "amount": "200"
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
    --gas-prices 1905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
