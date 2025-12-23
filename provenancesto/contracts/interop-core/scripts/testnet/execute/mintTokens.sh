
provenanced tx wasm execute \
    tp1hfcpqqxl0e9g6terx5qw0nvqrfty9thequ6c8czc9k7vytyd98ys9pj40a \
    '{
    "mint_tokens": {
        "order": {
            "order_id": "1",
            "denom": "TJ-Test-6",
            "investor": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
            "amount": "100"
        }
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

