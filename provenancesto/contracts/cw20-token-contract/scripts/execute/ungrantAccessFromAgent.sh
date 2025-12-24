
provenanced tx wasm execute \
    tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz \
    '{
    "manage_roles": {
        
        "role": {
            "agent": {
                "update_type": {
                    "remove": [
                        "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
                    ]
                },
                "marker_access": [
                    "admin",
                    "burn",
                    "deposit",
                    "delete",
                    "mint",
                    "transfer",
                    "unspecified",
                    "withdraw",
                    "freeze",
                    "unfreeze",
                    "force_transfer"
                ]
            }
        }
    }
}
' \
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

