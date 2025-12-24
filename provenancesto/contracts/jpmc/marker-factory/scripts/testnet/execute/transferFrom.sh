provenanced tx wasm execute \
    tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
    '{
    "transfer_from": {
        "denom": "WCustomMarker",
        "amount": "500",
        "to": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct",
        "from": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
    }
}' \
    --from $feebucket \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas auto \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
    