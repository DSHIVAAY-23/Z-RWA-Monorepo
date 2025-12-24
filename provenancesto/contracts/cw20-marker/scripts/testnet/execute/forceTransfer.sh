provenanced tx wasm execute \
    tp10m7er24gc7u0fl26qpm4d487d90vug2gw2s4kq9r5zw00nd4hymqgmrpa3 \
    '{
    "force_transfer": {
        "denom": "MCustomMarker",
        "params": [
            {
                "amount": "500",
                "to": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                "from": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct"
            }
        ]
    }
}' \
    --from $tarun \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 1905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
    