wasm=artifacts/custom_marker.wasm

$local_prov/provenanced tx wasm store $wasm \
    --instantiate-only-address "$local_validator" \
    --from $local_validator \
    --keyring-backend test \
    --home $local_prov_path \
    --chain-id testing \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 1905nhash \
	--output json | jq

