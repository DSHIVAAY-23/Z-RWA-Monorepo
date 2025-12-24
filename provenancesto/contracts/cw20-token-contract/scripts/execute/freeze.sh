
provenanced tx wasm execute \
    tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
    '{
    "freeze": {
        
        "update_type": [
            {
                "add": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm"
            }
        ]
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
