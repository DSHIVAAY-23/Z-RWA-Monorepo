provenanced tx wasm instantiate 477 \
	'{
    "client_id": "testing",
    "oracle_script_id": "360",
    "ask_count": "4",
    "min_count": "3",
    "fee_limit": [
        {
            "denom": "uband",
            "amount": "25000"
        }
    ],
    "prepare_gas": "100000",
    "execute_gas": "500000",
    "minimum_sources": 1
}' \
    --admin "$feebucket" \
    --label custom-marker \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
    
    


