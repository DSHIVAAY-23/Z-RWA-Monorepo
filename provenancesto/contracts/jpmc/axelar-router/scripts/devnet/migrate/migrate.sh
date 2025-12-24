provenanced tx wasm migrate \
    tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx \
    84 \
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

