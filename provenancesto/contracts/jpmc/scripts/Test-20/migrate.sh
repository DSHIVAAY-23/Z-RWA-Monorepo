provenanced tx wasm migrate \
    tp1we9chrq58nj3etdw3yss7ytqeaeeka5725u2m0lg7vg5yzrwa8usp8shgj \
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

