
provenanced tx wasm execute \
    tp10m7er24gc7u0fl26qpm4d487d90vug2gw2s4kq9r5zw00nd4hymqgmrpa3 \
    '{
    "manage_roles": {
        "denom": "MCustomMarker",
        "role": {
            "agent": {
                "update_type": {
                    "add": [
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
