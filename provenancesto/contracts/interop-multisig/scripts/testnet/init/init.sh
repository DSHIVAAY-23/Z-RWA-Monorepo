provenanced tx wasm instantiate 692 \
	'{
    "threshold": 1
}' \
    --admin "$feebucket" \
    --label interop-multisig \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 300000nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

