provenanced tx wasm execute \
    tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
    '{
    "transfer": {
        "denom": "WCustomMarker",
        "amount": "2000",
        "to": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy"
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
    