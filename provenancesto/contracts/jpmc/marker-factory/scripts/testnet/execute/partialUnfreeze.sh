
provenanced tx wasm execute \
    tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
    '{
    "partial_freeze": {
        "denom": "WCustomMarker",
        "params": [
            {
                "address": "tp1gpw2r2ga427d6trrsxq8l8axjgmmh8vwxda4gm",
                "update_type": {
                    "remove": "500"
                }
            }
        ]
    }
}' \
    --from $tarun \
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
