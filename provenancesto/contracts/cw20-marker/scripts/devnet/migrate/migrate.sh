provenanced tx wasm migrate \
    tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas \
    16 \
    '{}' \
    --from "$dev" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 0vspn \
    --node=http://34.70.126.95:26657 \
	--output json | jq

