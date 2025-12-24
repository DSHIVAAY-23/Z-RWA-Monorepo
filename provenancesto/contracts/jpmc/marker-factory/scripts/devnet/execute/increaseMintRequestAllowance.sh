
provenanced tx wasm execute \
    tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
    '{
    "manage_request_allowance": {
        "denom": "Test-1",
        "spender": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "update_type": {
            "add": "1000"
        },
        "request_type": "mint"
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

