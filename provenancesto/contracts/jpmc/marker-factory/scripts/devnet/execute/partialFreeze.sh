
provenanced tx wasm execute \
    tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
    '{
    "partial_freeze": {
        "denom": "Test-1",
        "params": [
            {
                "address": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm",
                "update_type": {
                    "add": "500"
                }
            }
        ]
    }
}' \
    --from $dev \
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
