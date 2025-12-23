
provenanced tx wasm execute \
    tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 \
    '{
    "request_from": {
        "denom": "WCustomMarker",
        "request_id": "0x0290934747923332982545784284218402533764514524637631709843857691",
        "from": "tp1dftv3wslxwzl99n7g4nqge47n07p9lczgeearp",
        "amount": "1000",
        "request_type": "mint"
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
