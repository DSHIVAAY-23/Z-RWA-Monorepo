provenanced tx wasm instantiate 604 \
	'{
    "code_id": 603
}' \
    --admin "$feebucket" \
    --label marker-factory \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas-prices 400000nhash \
    --gas auto \
    --gas-adjustment 1.3 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
