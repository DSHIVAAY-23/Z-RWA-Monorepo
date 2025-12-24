wasm=artifacts/cw20_marker-aarch64.wasm

provenanced tx wasm store $wasm \
    --instantiate-only-address "$dev" \
    --from $dev \
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
