
provenanced tx wasm execute \
    tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3 \
    '{
    "burn_from": {
        "burn_from_params": [
            {
                "denom": "MCustomMarker",
                "mint_burn_data": [
                    {
                        "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                        "amount": "200"
                    }
                ]
            }
        ]
    }
}' \
    --from $minter \
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
