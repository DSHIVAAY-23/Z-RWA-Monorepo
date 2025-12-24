provenanced tx wasm execute \
    tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
    '{
    "transfer": {
        "denom": "APO1",
        "amount": "2000000",
        "to": "tp1e65ee4mfkj4amdjzw0fx37kp50gpf6pfs8wqey"
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
    