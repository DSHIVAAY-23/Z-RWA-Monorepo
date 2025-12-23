provenanced tx wasm execute \
    tp190s9ns0jvgr423jznaxcca332kze964dqpf64rr47vxrx2lag04sney6sx \
    '{
    "share_stable_coin": {
        "denom": "MCustomMarker",
        "coin_type": "usdt",
        "share_params": [
            {
                "to": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                "payment": "100"
            },
            {
                "to": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                "payment": "100"
            },
            {
                "to": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                "payment": "100"
            }
        ]
    }
}' \
    --from $feebucket \
    --amount 200USDT-Test \
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