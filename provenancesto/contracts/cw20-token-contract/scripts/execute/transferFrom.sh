
provenanced tx wasm execute \
    tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
    '{
    "transfer_from": {
        "owner": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
        "recipient": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct",
        "amount": "100"
    }
}' \
    --from $validator \
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
