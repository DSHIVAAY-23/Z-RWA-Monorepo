
provenanced tx wasm execute \
    tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
    '{
    "approve": {
        "order_id": [
            0,
            0
        ],
        "request_type": "burn"
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

