
provenanced tx wasm execute \
    tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3 \
    '{
    "partial_freeze": {
        "denom": "MCustomMarker",
        "params": [
            {
                "address": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm",
                "update_type": {
                    "add": "500"
                }
            }
        ]
    }
}' \
    --from $tarun \
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
