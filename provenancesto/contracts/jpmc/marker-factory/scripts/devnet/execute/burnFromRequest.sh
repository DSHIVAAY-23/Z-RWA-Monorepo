
provenanced tx wasm execute \
    tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
    '{
    "request_from": {
        "denom": "Test-1",
        "request_id": "0x0290934747923332982545784284218402533764514524637631709843857691",
        "from": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "amount": "1000",
        "request_type": "burn"
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

