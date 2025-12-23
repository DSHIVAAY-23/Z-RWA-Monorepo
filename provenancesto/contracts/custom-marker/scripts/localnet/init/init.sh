$local_prov/provenanced tx wasm instantiate 2 \
	'{}' \
    --admin "$local_validator" \
    --label custom-marker \
    --from "$local_validator" \
    --keyring-backend test \
    --home $local_prov_path \
    --chain-id testing \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq
    