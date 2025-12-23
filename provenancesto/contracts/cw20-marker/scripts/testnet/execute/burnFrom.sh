
provenanced tx wasm execute \
    tp10m7er24gc7u0fl26qpm4d487d90vug2gw2s4kq9r5zw00nd4hymqgmrpa3 \
    '{
    "burn_from": {
        "burn_from_params": [
            {
                "denom": "TJTest-3",
                "mint_burn_data": [
                    {
                        "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                        "amount": "50"
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
    --gas-prices 50000nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'
