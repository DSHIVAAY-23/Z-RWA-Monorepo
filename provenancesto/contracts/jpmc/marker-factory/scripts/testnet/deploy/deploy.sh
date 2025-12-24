wasm=artifacts/marker_factory-aarch64.wasm

provenanced tx wasm store $wasm \
    --instantiate-only-address "$feebucket" \
    --from validator \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --node=https://rpc.test.provenance.io:443 \
	--output json | jq
