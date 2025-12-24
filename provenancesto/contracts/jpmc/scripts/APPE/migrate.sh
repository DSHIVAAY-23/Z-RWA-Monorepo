provenanced tx wasm migrate \
    tp1xgzc3t3x3jshwfst83umkxthckh0dyqexa54dsuyyh00jsgs7tlse0a9ty \
    90 \
    '{}' \
    --from "$dev" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas auto \
    --gas-prices 0vspn \
    --node=http://34.70.126.95:26657 \
	--output json | jq

