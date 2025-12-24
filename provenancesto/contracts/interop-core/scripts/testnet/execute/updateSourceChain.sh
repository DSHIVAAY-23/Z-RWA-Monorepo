
provenanced tx wasm execute \
    tp1hfcpqqxl0e9g6terx5qw0nvqrfty9thequ6c8czc9k7vytyd98ys9pj40a \
    '{
    "update_source_chain": {
        "chain": "Provenance"
    }
}' \
    --from $feebucket \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq 

