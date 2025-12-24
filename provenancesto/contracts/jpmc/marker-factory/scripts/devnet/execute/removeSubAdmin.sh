
provenanced tx wasm execute \
    tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla \
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
    --from $dev \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-axl-devnet-1 \
    --gas auto \
    --gas-prices 0vspn \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=http://34.70.126.95:26657 | jq
