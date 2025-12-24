
provenanced tx wasm execute \
    tp1h7scjsemhcy9pgs9ges488n5cxdghz4sj2qk0qwgrjw7jrpgvets8j4tzq \
    '{
    "manage_roles": {
        "roles": [
            {
                "sub_admin": {
                    "update_type": {
                        "remove": [
                            "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
                        ]
                    }
                }
            }
        ]
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
