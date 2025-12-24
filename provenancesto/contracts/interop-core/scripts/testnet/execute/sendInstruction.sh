
provenanced tx wasm execute \
    tp1hfcpqqxl0e9g6terx5qw0nvqrfty9thequ6c8czc9k7vytyd98ys9pj40a \
    '{
    "send_instruction": {
        "params": {
            "portfolios": [
                {
                    "dest_chain": "Holesky",
                    "dest_address": "0x0547b3e9253224978ada177570e87707e0ae24bd",
                    "investor": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                    "token": "TJTest-3",
                    "amount": "100",
                    "order_id": "1",
                    "action": "mint"
                },
                {
                    "dest_chain": "Holesky",
                    "dest_address": "0x0547b3e9253224978ada177570e87707e0ae24bd",
                    "investor": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
                    "token": "TJTest-3",
                    "amount": "100",
                    "order_id": "2",
                    "action": "burn"
                }
            ]
        }
    }
}' \
    --from $feebucket \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq '.txhash, .code, .raw_log'

