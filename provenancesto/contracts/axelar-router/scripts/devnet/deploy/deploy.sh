wasm=artifacts/axelar_router-aarch64.wasm

provenanced tx wasm store $wasm \
    --instantiate-only-address "$dev" \
    --from $dev \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas-prices 0vspn \
    --gas auto \
    --gas-adjustment 1.5 \
    --node=http://34.70.126.95:26657 \
	--output json | jq
