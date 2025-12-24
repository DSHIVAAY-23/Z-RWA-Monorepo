
provenanced tx wasm execute \
    tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
    '{
    "update_frozen_list": {
        "discard": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
    }
}' \
    --from $minter \
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
