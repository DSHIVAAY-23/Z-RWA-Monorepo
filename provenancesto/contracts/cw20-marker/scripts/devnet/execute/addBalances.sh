
provenanced tx wasm execute \
    tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
    '{
    "update_balances": [
        "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct",
        {
            "add": {
                "token_limit": "1000",
                "frozen_bal": "1000"
            }
        }
    ]
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




